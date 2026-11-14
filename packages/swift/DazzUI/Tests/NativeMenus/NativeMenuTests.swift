import Foundation
import NativeAPI
import SwiftUI
import XCTest

@testable import DazzUI
@testable import SwiftUIRuntime

#if os(Linux)
  import CGTK3
#elseif os(Windows)
  import WinSDK
#endif

@MainActor
final class NativeMenuTests: XCTestCase {
  override func setUp() {
    #if os(Windows)
      WindowsDrawingBackend.prepareApplication()
    #endif
    _ = NativeAPI.Application.shared.isRunning()
  }

  private func pump(until done: () -> Bool, timeout: TimeInterval = 2) {
    let deadline = Date().addingTimeInterval(timeout)
    while !done() && Date() < deadline {
      #if os(Linux)
        while g_main_context_iteration(nil, 0) != 0 {}
      #elseif os(Windows)
        var message = MSG()
        while PeekMessageW(&message, nil, 0, 0, UINT(PM_REMOVE)) {
          TranslateMessage(&message)
          DispatchMessageW(&message)
        }
      #endif
      Thread.sleep(forTimeInterval: 0.002)
    }
  }

  func testNativeRowsAndNestedChoices() async throws {
    XCTAssertTrue(MenuPresentation.native.usesNativeMenus)
    XCTAssertFalse(MenuPresentation.custom.usesNativeMenus)
    var calls = 0
    let session = NativeMenuSession(
      entries: [
        .heading("People"),
        .item(DazzUI.MenuItem("Ada", shortcut: "Ctrl+A", checked: true)),
        .item(DazzUI.MenuItem("Disabled", checked: false, disabled: true)),
        .item(
          DazzUI.MenuItem(
            "Move",
            items: [
              DazzUI.MenuItem("Nested", items: [DazzUI.MenuItem("Choose") { calls += 1 }])
            ])),
      ], theme: .default)
    let menu = try XCTUnwrap(session.menu)
    let items = menu.allItems
    XCTAssertEqual(items.map(\.label), ["People", "Ada\tCtrl+A", "Disabled", "Move"])
    XCTAssertFalse(items[0].isEnabled)
    XCTAssertFalse(items[2].isEnabled)
    XCTAssertEqual(items[1].state, .checked)
    XCTAssertEqual(items[2].state, .unchecked)
    let nested = try XCTUnwrap(items[3].submenu?.allItems.first?.submenu?.allItems.first)
    XCTAssertEqual(nested.label, "Choose")
    #if os(Linux)
      gtk_menu_item_activate(
        try XCTUnwrap(nested.nativeObject).assumingMemoryBound(to: GtkMenuItem.self))
      gtk_menu_item_activate(
        try XCTUnwrap(nested.nativeObject).assumingMemoryBound(to: GtkMenuItem.self))
      XCTAssertEqual(calls, 1)
    #endif
    session.cancel()
  }

  func testComponentsRouteToNativePopup() async throws {
    func nodes(_ node: Element) -> [Element] { [node] + node.children.flatMap(nodes) }
    for kind in 0..<4 {
      StateRegistry.shared.reset()
      var calls = 0
      let items = [DazzUI.MenuItem("Choose") { calls += 1 }]
      let content: AnyView
      switch kind {
      case 0:
        content = AnyView(
          DazzUI.Menu(items: items) { state in
            DazzUI.Button("Open", action: state.toggle)
          })
      case 1:
        content = AnyView(
          DazzUI.Select(
            options: [
              SelectOption(value: "a", label: "Ada"), SelectOption(value: "b", label: "Ben"),
            ], selection: Binding(get: { "a" }, set: { _ in calls += 1 })))
      case 2:
        content = AnyView(DazzUI.ContextMenu(items: items) { Text("Context").padding(20) })
      default:
        content = AnyView(DazzUI.Menubar(menus: [MenubarMenu("File", items: items)]))
      }
      let window = try XCTUnwrap(NativeAPI.Window.create())
      window.setContentSize(NativeAPI.Size(width: 480, height: 320))
      #if os(Windows)
        let backend = try XCTUnwrap(WindowsDrawingBackend(window: window))
        let measure = WindowsDrawingBackend.measureText
      #else
        let backend = try XCTUnwrap(LinuxDrawingBackend(window: window))
        let measure = backend.measureText
      #endif
      let renderer = Renderer(
        backend: backend,
        root: AnyView(content.menuPresentation(.native)),
        size: SwiftUIRuntime.Size(width: 480, height: 320),
        layoutEngine: LayoutEngine(textMeasurer: measure))
      renderer.mount()
      window.show()
      window.focus()
      defer { window.hide() }
      #if os(Windows)
        let hwnd = try XCTUnwrap(window.nativeObject).assumingMemoryBound(to: HWND__.self)
        guard GetForegroundWindow() == hwnd else {
          throw XCTSkip("Native popup requires an interactive foreground window")
        }
        var opened = false
        let closer = NativeRunLoopTimer(milliseconds: 20) {
          var info = GUITHREADINFO()
          info.cbSize = DWORD(MemoryLayout<GUITHREADINFO>.size)
          if GetGUIThreadInfo(GetCurrentThreadId(), &info), info.hwndMenuOwner == hwnd {
            opened = true
            EndMenu()
          }
        }
        defer { closer.invalidate() }
        let timeout = NativeRunLoopTimer(milliseconds: 1800) { EndMenu() }
        defer { timeout.invalidate() }
      #endif
      if kind == 2 {
        let region = try XCTUnwrap(nodes(renderer.tree!).first { $0.secondaryAction != nil })
        _ = renderer.input(
          id: nil, event: 9,
          point: CGPoint(
            x: region.frame.minX + region.frame.width / 2,
            y: region.frame.minY + region.frame.height / 2))
      } else {
        let trigger = try XCTUnwrap(nodes(renderer.tree!).first { $0.action != nil })
        renderer.performTap(trigger.id)
      }
      #if os(Windows)
        pump(until: { opened && !NativeMenuSession.isTracking })
        XCTAssertTrue(opened, "Component \(kind) opens a native popup")
        XCTAssertEqual(calls, 0, "Cancellation does not select an item")
      #else
        pump(until: { gtk_grab_get_current() != nil })
        let popup = try XCTUnwrap(
          gtk_grab_get_current(), "Component \(kind) opens GTK's native popup")
        XCTAssertNotEqual(
          g_type_check_instance_is_a(
            UnsafeMutableRawPointer(popup).assumingMemoryBound(to: GTypeInstance.self),
            gtk_menu_get_type()), 0)
        let list = gtk_container_get_children(
          UnsafeMutableRawPointer(popup).assumingMemoryBound(to: GtkContainer.self))
        defer { g_list_free(list) }
        let item = try XCTUnwrap(g_list_nth_data(list, kind == 1 ? 1 : 0))
          .assumingMemoryBound(to: GtkWidget.self)
        gtk_menu_shell_activate_item(
          UnsafeMutableRawPointer(popup).assumingMemoryBound(to: GtkMenuShell.self), item, 1)
        pump(until: { !NativeMenuSession.isTracking })
        XCTAssertEqual(calls, 1, "Component \(kind) dispatches one selection")
      #endif
      XCTAssertFalse(NativeMenuSession.isTracking)
    }
  }

  func testQueuedOpenCanBeCancelledWithoutShowing() async throws {
    let session = NativeMenuSession(entries: [.item(DazzUI.MenuItem("Choose"))], theme: .default)
    var opened = false
    let menu = try XCTUnwrap(session.menu)
    let listener = menu.addListener { if case .opened = $0 { opened = true } }
    defer { _ = menu.removeListener(listener) }
    session.present(strategy: .cursorPosition(), placement: .bottomStart)
    session.cancel()
    pump(until: { !session.tracking })
    XCTAssertFalse(opened)
    XCTAssertFalse(session.tracking)
  }

  func testRealPopupClosesAndUsesContentCoordinates() async throws {
    let window = try XCTUnwrap(NativeAPI.Window.create())
    window.setTitle("DazzUI native menu test")
    window.setPosition(NativeAPI.Point(x: 180, y: 140))
    window.setContentSize(NativeAPI.Size(width: 480, height: 320))
    window.show()
    window.focus()
    defer { window.hide() }
    #if os(Windows)
      let hwnd = try XCTUnwrap(window.nativeObject).assumingMemoryBound(to: HWND__.self)
      guard GetForegroundWindow() == hwnd else {
        throw XCTSkip("Native popup requires an interactive foreground window")
      }
    #endif
    let controller = PortNativeMenuController()
    controller.frame = CGRect(x: 80, y: 40, width: 100, height: 25)
    var dismissed = 0
    controller.update(
      window: window, shown: true, presentation: .native,
      entries: [.item(DazzUI.MenuItem("Choose"))], theme: .default, align: .start,
      point: nil, bar: nil, dismiss: { dismissed += 1 })
    let session = try XCTUnwrap(controller.session)
    let menu = try XCTUnwrap(session.menu)
    var opened = false
    let listener = menu.addListener { if case .opened = $0 { opened = true } }
    defer { _ = menu.removeListener(listener) }
    session.whileTracking = {
      guard opened else { return }
      #if os(Linux)
        let widget = menu.nativeObject!.assumingMemoryBound(to: GtkWidget.self)
        let nativeWindow = gtk_widget_get_window(widget)
        var x: Int32 = 0
        var y: Int32 = 0
        gdk_window_get_origin(nativeWindow, &x, &y)
        let content = window.contentBounds
        XCTAssertEqual(Double(x), content.x + 80, accuracy: 8)
        XCTAssertEqual(Double(y), content.y + 65, accuracy: 8)
      #endif
      session.cancel()
    }
    // A native timer also bounds the nested Win32 popup loop on regressions.
    let timeout = NativeRunLoopTimer(milliseconds: 1800) { session.cancel() }
    defer { timeout.invalidate() }
    pump(until: { controller.session == nil })
    XCTAssertTrue(opened)
    XCTAssertFalse(session.tracking)
    XCTAssertNil(controller.session)
    XCTAssertEqual(dismissed, 1)
  }
}
