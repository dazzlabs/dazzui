import AppKit
import SwiftUI
import XCTest

@testable import DazzUI

@MainActor
final class MenuPresentationTests: XCTestCase {
  private var windows: [NSWindow] = []

  override func tearDown() {
    for window in windows {
      window.orderOut(nil)
      window.contentView = nil
    }
    windows = []
    super.tearDown()
  }

  private func host<V: View>(_ content: V) -> NSHostingView<V> {
    let host = NSHostingView(rootView: content)
    let window = NSWindow(
      contentRect: NSRect(x: 200, y: 200, width: 500, height: 300),
      styleMask: [.titled, .closable], backing: .buffered, defer: false)
    window.isReleasedWhenClosed = false
    window.contentView = host
    windows.append(window)
    NSApp.activate(ignoringOtherApps: true)
    window.makeKeyAndOrderFront(nil)
    host.layoutSubtreeIfNeeded()
    pump()
    return host
  }

  private func pump() { RunLoop.main.run(until: Date().addingTimeInterval(0.08)) }

  private func trackingTimer(
    after interval: TimeInterval = 0.02, repeats: Bool = true,
    action: @escaping @MainActor () -> Void
  ) -> Timer {
    let timer = Timer(timeInterval: interval, repeats: repeats) { _ in
      MainActor.assumeIsolated { action() }
    }
    RunLoop.main.add(timer, forMode: .eventTracking)
    return timer
  }
  private func descendants(_ view: NSView) -> [NSView] {
    [view] + view.subviews.flatMap(descendants)
  }

  private struct Probe: View {
    @Environment(\.menuPresentation) private var presentation
    @Environment(\.theme) private var theme
    let read: (MenuPresentation, Theme) -> Void
    var body: some View {
      ProbeReader(presentation: presentation, theme: theme, read: read)
        .frame(width: 10, height: 10)
    }
  }

  private struct ProbeReader: NSViewRepresentable {
    let presentation: MenuPresentation
    let theme: Theme
    let read: (MenuPresentation, Theme) -> Void
    func makeNSView(context: Context) -> NSView { NSView() }
    func updateNSView(_ view: NSView, context: Context) { read(presentation, theme) }
  }

  func testDefaultAndNestedScope() {
    XCTAssertEqual(EnvironmentValues().menuPresentation, .custom)
    var outer: MenuPresentation?
    var inner: MenuPresentation?
    _ = host(
      VStack {
        Probe { value, _ in outer = value }
        Probe { value, _ in inner = value }.menuPresentation(.custom)
      }.menuPresentation(.native))
    XCTAssertEqual(outer, .native)
    XCTAssertEqual(inner, .custom)
  }

  @MainActor final class Model: ObservableObject {
    @Published var presentation: MenuPresentation = .native
    @Published var shown = true
    @Published var mounted = true
    @Published var theme = Theme(.studioDark, sizing: .system)
  }

  private struct FloatingFixture: View {
    @ObservedObject var model: Model
    let read: (MenuPresentation, Theme) -> Void
    var body: some View {
      Text("Anchor")
        .floatingPanel(isPresented: $model.shown, gap: 4, modal: false) {
          Probe(read: read).frame(width: 100, height: 60)
        }
        .environment(\.theme, model.theme)
        .menuPresentation(model.presentation)
    }
  }

  func testFloatingHostingInheritsAndUpdatesResolvedScope() {
    let model = Model()
    var read: (MenuPresentation, Theme)?
    _ = host(FloatingFixture(model: model) { read = ($0, $1) })
    XCTAssertEqual(read?.0, .native)
    XCTAssertEqual(read?.1.sizing, .system)
    XCTAssertEqual(read?.1.vars.bodyMedium.size, model.theme.vars.bodyMedium.size)
    XCTAssertEqual(windows[0].childWindows?.count, 1)
    model.presentation = .custom
    pump()
    XCTAssertEqual(read?.0, .custom)
    model.shown = false
    pump()
    XCTAssertTrue(windows[0].childWindows?.isEmpty ?? true)
  }

  private struct OverlayFixture: View {
    @ObservedObject var model: Model
    let read: (MenuPresentation, Theme) -> Void
    var body: some View {
      Text("Dialog")
        .dialog(isPresented: $model.shown) { Probe(read: read) }
        .environment(\.theme, model.theme)
        .menuPresentation(model.presentation)
        .overlayHost()
    }
  }

  func testDialogInheritsAndUpdatesResolvedScope() {
    let model = Model()
    var read: (MenuPresentation, Theme)?
    _ = host(OverlayFixture(model: model) { read = ($0, $1) })
    XCTAssertEqual(read?.0, .native)
    XCTAssertEqual(read?.1.sizing, .system)
    model.presentation = .custom
    pump()
    XCTAssertEqual(read?.0, .custom)
    model.theme = Theme(.brightLight)
    pump()
    XCTAssertEqual(read?.1.name, .brightLight)
    XCTAssertEqual(read?.1.sizing, .design)
    XCTAssertEqual(read?.1.vars.bodyMedium.size, model.theme.vars.bodyMedium.size)
  }

  func testNativeConversionRetainsSlotsChecksGroupsAndHintWithoutAccelerator() throws {
    let session = NativeMenuSession(
      entries: [
        .heading("People"),
        .item(
          MenuItem(
            label: "Ada", icon: AnyView(Text("A")), shortcut: AnyView(Text("⌘A")), checked: true)),
        .item(MenuItem("Unavailable", checked: false, disabled: true)),
      ], theme: Theme(.studioLight))
    XCTAssertEqual(session.menu.items[0].title, "People")
    XCTAssertTrue(session.menu.items[1].title.hasPrefix("Ada"))
    XCTAssertEqual(session.menu.items[1].accessibilityLabel(), "Ada")
    XCTAssertEqual(session.menu.items[2].title, "Unavailable")
    XCTAssertFalse(session.menu.items[0].isEnabled)
    let item = session.menu.items[1]
    XCTAssertNotNil(item.image)
    XCTAssertNotNil(item.attributedTitle)
    XCTAssertEqual(item.state, .on)
    XCTAssertFalse(session.menu.items[2].isEnabled)
    XCTAssertTrue(session.menu.items.allSatisfy { $0.view == nil && $0.keyEquivalent.isEmpty })
  }

  func testNativeSubmenuBuildsNestedRowsAndDispatchesTheirChoice() throws {
    var chosen: String?
    let session = NativeMenuSession(
      entries: [
        .item(MenuItem("Rename")),
        .item(
          MenuItem(
            label: "Move to", disabled: false,
            items: [
              MenuItem(
                label: "Projects", disabled: false,
                items: [
                  MenuItem("DazzUI") { chosen = "DazzUI" },
                  MenuItem("Shell"),
                ]),
              MenuItem("Archive"),
            ])),
      ], theme: .default)
    let moveTo = try XCTUnwrap(session.menu.items[1].submenu)
    XCTAssertEqual(moveTo.items.map(\.title), ["Projects", "Archive"])
    let projects = try XCTUnwrap(moveTo.items[0].submenu)
    XCTAssertEqual(projects.items.map(\.title), ["DazzUI", "Shell"])
    // The nested row reports through the session like a top-level one, and
    // the one-choice-per-session rule holds across levels.
    projects.performActionForItem(at: 0)
    XCTAssertEqual(chosen, "DazzUI")
    projects.performActionForItem(at: 1)
    XCTAssertEqual(chosen, "DazzUI")
  }

  func testNativeSelectConversionUpdatesBindingAndKeepsMixedGroupOrder() {
    var selection = "a"
    let select = DazzUI.Select(
      options: [
        SelectEntry.option(value: "none", label: "Unassigned"),
        .group(
          "People",
          options: [
            SelectOption(value: "a", label: "Ada"),
            SelectOption(value: "b", label: "Ben", enabled: false),
          ]),
        .option(value: "c", label: "Cleo"),
      ], selection: Binding(get: { selection }, set: { selection = $0 }))
    let session = NativeMenuSession(entries: select.nativeEntries, theme: .default)
    XCTAssertEqual(session.menu.items.map(\.title), ["Unassigned", "People", "Ada", "Ben", "Cleo"])
    XCTAssertEqual(session.menu.items[2].state, .on)
    session.menu.performActionForItem(at: 3)
    XCTAssertEqual(selection, "a")
    session.menu.performActionForItem(at: 4)
    XCTAssertEqual(selection, "c")
    session.menu.performActionForItem(at: 0)
    XCTAssertEqual(selection, "c", "One tracking session may dispatch only once")
  }

  /// These enter NSMenu's actual nested event-tracking loop in a visible window.
  /// Action dispatch uses AppKit's API; cancellation is AppKit's real cancellation.
  func testRealMenuTrackingSelectCancelAndDisabledDispatch() {
    let view = host(Text("Menu anchor"))
    var calls = 0
    for index in [0, 1, -1] {
      let session = NativeMenuSession(
        entries: [
          .item(MenuItem("Choose") { calls += 1 }),
          .item(MenuItem("Disabled", disabled: true) { calls += 100 }),
        ], theme: .default)
      var entered = false
      var closed = 0
      session.didClose = { closed += 1 }
      session.whileTracking = {
        guard !entered else { return }
        entered = true
        XCTAssertTrue(session.tracking)
        if index >= 0 { session.menu.performActionForItem(at: index) }
        session.cancel()
      }
      session.present(at: NSPoint(x: 10, y: 10), in: view)
      XCTAssertTrue(entered)
      XCTAssertFalse(session.tracking)
      XCTAssertEqual(closed, 1)
    }
    XCTAssertEqual(calls, 1)
  }

  private struct NestedMenuFixture: View {
    @ObservedObject var model: Model
    let chosen: () -> Void
    var body: some View {
      Text("Popover anchor").floatingPanel(isPresented: $model.shown, gap: 4, modal: false) {
        DazzUI.Menu(items: [MenuItem("Choose", onSelect: chosen)]) { state in
          DazzUI.Button("Open inside", action: state.toggle)
        }.padding(20)
      }.menuPresentation(model.presentation)
    }
  }

  func testNativeKeyboardSelectionInsideFloatingPanelPreservesParent() throws {
    let model = Model()
    var calls = 0
    let view = host(NestedMenuFixture(model: model) { calls += 1 })
    let panel = try XCTUnwrap(view.window?.childWindows?.first)
    let root = try XCTUnwrap(panel.contentView)
    let anchor = try XCTUnwrap(descendants(root).compactMap { $0 as? NativeMenuAnchorView }.first)
    var sent = false
    let timer = trackingTimer {
      guard anchor.session?.tracking == true, !sent else { return }
      sent = true
      for (characters, code) in [("\u{f701}", UInt16(125)), ("\r", UInt16(36))] {
        let event = NSEvent.keyEvent(
          with: .keyDown, location: .zero, modifierFlags: [],
          timestamp: ProcessInfo.processInfo.systemUptime, windowNumber: panel.windowNumber,
          context: nil, characters: characters, charactersIgnoringModifiers: characters,
          isARepeat: false, keyCode: code)!
        NSApp.postEvent(event, atStart: false)
      }
    }
    let timeout = trackingTimer(after: 2, repeats: false) { anchor.cancel() }
    click(anchor.convert(NSPoint(x: anchor.bounds.midX, y: anchor.bounds.midY), to: nil), in: panel)
    pump()
    timer.invalidate()
    timeout.invalidate()
    XCTAssertTrue(sent)
    XCTAssertEqual(calls, 1)
    XCTAssertTrue(model.shown)
    XCTAssertEqual(view.window?.childWindows?.count, 1)
  }

  func testRealKeyboardSelectionDispatchesBeforeSessionCleanup() {
    let view = host(Text("Keyboard anchor"))
    var calls = 0
    let session = NativeMenuSession(
      entries: [.item(MenuItem("Choose") { calls += 1 })], theme: .default)
    var sent = false
    session.whileTracking = {
      guard !sent else { return }
      sent = true
      for (characters, code) in [("\u{f701}", UInt16(125)), ("\r", UInt16(36))] {
        let event = NSEvent.keyEvent(
          with: .keyDown, location: .zero, modifierFlags: [],
          timestamp: ProcessInfo.processInfo.systemUptime, windowNumber: view.window!.windowNumber,
          context: nil, characters: characters, charactersIgnoringModifiers: characters,
          isARepeat: false, keyCode: code)!
        NSApp.postEvent(event, atStart: false)
      }
    }
    let timeout = trackingTimer(after: 2, repeats: false) { session.cancel() }
    session.present(at: NSPoint(x: 10, y: 10), in: view)
    timeout.invalidate()
    XCTAssertTrue(sent)
    XCTAssertEqual(calls, 1)
    XCTAssertTrue(session.selected)
    XCTAssertFalse(session.cancelled)
  }

  private struct AnchorFixture: View {
    @ObservedObject var model: Model
    let chosen: () -> Void
    var body: some View {
      Group {
        if model.mounted {
          Text("Trigger").nativeMenu(
            isPresented: $model.shown,
            entries: [.item(MenuItem("Choose", onSelect: chosen))])
        }
      }.menuPresentation(model.presentation)
    }
  }

  func testLiveModeChangeUnmountAndWindowCloseCancelRealTracking() throws {
    for operation in 0..<3 {
      let model = Model()
      model.shown = false
      var calls = 0
      let view = host(AnchorFixture(model: model) { calls += 1 })
      let anchor = try XCTUnwrap(descendants(view).compactMap { $0 as? NativeMenuAnchorView }.first)
      var entered = false
      let timer = trackingTimer {
        guard !entered, let session = anchor.session, session.tracking else { return }
        entered = true
        switch operation {
        case 0: model.presentation = .custom
        case 1: model.mounted = false
        default: view.window?.close()
        }
      }
      // Safety timeout makes a lifecycle regression fail instead of hanging the suite.
      let timeout = trackingTimer(after: 2, repeats: false) { anchor.cancel() }
      model.shown = true
      let deadline = Date().addingTimeInterval(2)
      repeat { pump() } while !entered && Date() < deadline
      timer.invalidate()
      timeout.invalidate()
      pump()
      XCTAssertTrue(entered, "Lifecycle operation \(operation) must run during tracking")
      XCTAssertNil(anchor.session)
      XCTAssertEqual(calls, 0)
      XCTAssertFalse(model.shown)
    }
  }

  private func click(_ point: NSPoint, in window: NSWindow, secondary: Bool = false) {
    for type in secondary
      ? [NSEvent.EventType.rightMouseDown, .rightMouseUp] : [.leftMouseDown, .leftMouseUp]
    {
      let event = NSEvent.mouseEvent(
        with: type, location: point, modifierFlags: [],
        timestamp: ProcessInfo.processInfo.systemUptime, windowNumber: window.windowNumber,
        context: nil, eventNumber: 0, clickCount: 1, pressure: 1)!
      if secondary {
        // Synthetic sendEvent does not replace NSApp.currentEvent, which the
        // catcher's hitTest reads. Deliver directly to the actual native
        // catcher, as AppKit does after hit testing a real secondary click.
        if type == .rightMouseDown, let root = window.contentView,
          let catcher = descendants(root).first(where: {
            String(describing: Swift.type(of: $0)).contains("CatcherView")
          })
        {
          catcher.rightMouseDown(with: event)
        }
      } else {
        NSApp.sendEvent(event)
      }
    }
  }

  func testComponentsRouteNativeAndCustomWithoutChangingTriggerGeometry() throws {
    for kind in 0..<4 {
      let model = Model()
      model.shown = false
      var calls = 0
      let items = [MenuItem("Choose") { calls += 1 }]
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
              SelectOption(value: "a", label: "Ada"),
              SelectOption(value: "b", label: "Ben"),
            ],
            selection: Binding(get: { "a" }, set: { _ in calls += 1 }), width: .value))
      case 2:
        content = AnyView(DazzUI.ContextMenu(items: items) { Text("Right click").padding(20) })
      default:
        content = AnyView(DazzUI.Menubar(menus: [MenubarMenu("File", items: items)]))
      }
      let view = host(ModeFixture(model: model, content: content))
      let anchor = try XCTUnwrap(descendants(view).compactMap { $0 as? NativeMenuAnchorView }.first)
      let geometry = anchor.frame
      var entered = false
      let timer = trackingTimer {
        guard let session = anchor.session, session.tracking, !entered else { return }
        entered = true
        session.menu.performActionForItem(at: kind == 1 ? 1 : 0)
        session.cancel()
      }
      let timeout = trackingTimer(after: 2, repeats: false) { anchor.cancel() }
      let window = try XCTUnwrap(view.window)
      let center = anchor.convert(NSPoint(x: anchor.bounds.midX, y: anchor.bounds.midY), to: nil)
      click(center, in: window, secondary: kind == 2)
      pump()
      timer.invalidate()
      timeout.invalidate()
      XCTAssertTrue(entered, "Component \(kind) must enter native tracking")
      XCTAssertEqual(calls, 1, "Component \(kind) dispatches exactly once")
      XCTAssertNil(anchor.session)
      model.presentation = .custom
      pump()
      XCTAssertEqual(anchor.frame, geometry, "Mode must preserve trigger layout")
      click(center, in: window, secondary: kind == 2)
      pump()
      XCTAssertNil(anchor.session, "Custom mode must not open NSMenu")
      if kind < 2 { XCTAssertEqual(window.childWindows?.count, 1) }
      model.mounted = false
      pump()
    }
  }

  private struct ModeFixture: View {
    @ObservedObject var model: Model
    let content: AnyView
    var body: some View {
      Group { if model.mounted { content } }.menuPresentation(model.presentation)
    }
  }

  func testMenubarSwitchesDuringRealNativeTrackingWithoutStaleDismissal() throws {
    let items = [MenuItem("Choose")]
    let view = host(
      DazzUI.Menubar(menus: [
        MenubarMenu("File", items: items),
        MenubarMenu("Edit", items: items),
      ]).menuPresentation(.native))
    let anchors = descendants(view).compactMap { $0 as? NativeMenuAnchorView }
      .sorted { $0.convert(.zero, to: nil).x < $1.convert(.zero, to: nil).x }
    XCTAssertEqual(anchors.count, 2)
    let first = anchors[0]
    let second = anchors[1]
    let window = try XCTUnwrap(view.window)
    let oldPointer = NSEvent.mouseLocation
    defer {
      CGWarpMouseCursorPosition(
        CGPoint(x: oldPointer.x, y: NSScreen.screens[0].frame.height - oldPointer.y))
    }
    var switched = false
    var secondTracked = false
    let timer = trackingTimer {
      if first.session?.tracking == true && !switched {
        switched = true
        let target = window.convertToScreen(second.convert(second.bounds, to: nil))
        CGWarpMouseCursorPosition(
          CGPoint(x: target.midX, y: NSScreen.screens[0].frame.height - target.midY))
      }
      if let session = second.session, session.tracking {
        secondTracked = true
        session.cancel()
      }
    }
    let timeout = trackingTimer(after: 2, repeats: false) {
      first.cancel()
      second.cancel()
    }
    click(first.convert(NSPoint(x: first.bounds.midX, y: first.bounds.midY), to: nil), in: window)
    let deadline = Date().addingTimeInterval(2)
    repeat { pump() } while !secondTracked && Date() < deadline
    timer.invalidate()
    timeout.invalidate()
    XCTAssertTrue(switched)
    XCTAssertTrue(secondTracked)
    XCTAssertNil(first.session)
    XCTAssertNil(second.session)
  }

  private struct SearchFixture: View {
    @ObservedObject var model: SearchModel
    var body: some View {
      DazzUI.Combobox(
        options: [ComboboxOption("a", label: "Ada"), ComboboxOption("b", label: "Ben")],
        value: $model.value, query: $model.query
      )
      .frame(width: 240).menuPresentation(.native)
    }
  }

  @MainActor private final class SearchModel: ObservableObject {
    @Published var query = ""
    @Published var value: String?
  }

  func testSearchSuggestionsRemainCustomAndEditableInNativeMode() throws {
    let model = SearchModel()
    let view = host(SearchFixture(model: model))
    let field = try XCTUnwrap(descendants(view).compactMap { $0 as? NSTextField }.first)
    view.window?.makeFirstResponder(field)
    model.query = "Ad"
    pump()
    XCTAssertEqual(field.stringValue, "Ad")
    XCTAssertEqual(view.window?.childWindows?.count, 1)
    XCTAssertTrue(descendants(view).compactMap { $0 as? NativeMenuAnchorView }.isEmpty)
    model.query = "Nobody"
    pump()
    XCTAssertEqual(field.stringValue, "Nobody")
    XCTAssertEqual(view.window?.childWindows?.count, 1)
  }

  private struct AutocompleteFixture: View {
    @ObservedObject var model: SearchModel
    var body: some View {
      DazzUI.Autocomplete(items: ["Ada", "Ben"], text: $model.query)
        .frame(width: 240).menuPresentation(.native)
    }
  }

  func testAutocompleteSuggestionsRemainCustomInNativeMode() throws {
    let model = SearchModel()
    let view = host(AutocompleteFixture(model: model))
    let field = try XCTUnwrap(descendants(view).compactMap { $0 as? NSTextField }.first)
    view.window?.makeFirstResponder(field)
    model.query = "Ad"
    pump()
    XCTAssertEqual(field.stringValue, "Ad")
    XCTAssertEqual(view.window?.childWindows?.count, 1)
    XCTAssertTrue(descendants(view).compactMap { $0 as? NativeMenuAnchorView }.isEmpty)
  }

  func testPendingOpenCannotReappearAfterCancellation() {
    let anchor = NativeMenuAnchorView()
    _ = host(Text("Host"))
    let container = NSView(frame: windows[0].contentView!.frame)
    windows[0].contentView = container
    container.addSubview(anchor)
    anchor.frame = NSRect(x: 0, y: 0, width: 80, height: 30)
    var dismissed = 0
    anchor.update(
      shown: true, presentation: .native, entries: [.item(MenuItem("A"))],
      theme: .default, align: .start, point: nil, bar: nil, dismiss: { dismissed += 1 })
    let session = anchor.session
    anchor.cancel()
    pump()
    XCTAssertTrue(session?.cancelled == true)
    XCTAssertFalse(session?.tracking ?? true)
    XCTAssertNil(anchor.session)
    XCTAssertEqual(dismissed, 0)
  }
}
