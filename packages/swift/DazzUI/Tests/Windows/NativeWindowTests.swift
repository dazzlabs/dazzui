import NativeAPI
import SwiftUI
import WinSDK
import XCTest

@testable import DazzUI

/// Runs real nativeapi HWNDs. Activation is sent to the retained native BUTTON,
/// so the suite exercises the native event bridge without moving the pointer.
final class NativeWindowTests: XCTestCase {
  private struct Probe: SwiftUI.View {
    @State var count = 0
    @State var dark = false
    var body: some SwiftUI.View {
      VStack {
        DazzUI.Button("Count \(count)", variant: .filled) { count += 1 }
        DazzUI.Button("Disabled", enabled: false) { count = -1 }
        DazzUI.Button("Theme") { dark.toggle() }
        Card { Badge(dark ? "Dark" : "Light") }
      }.theme(dark ? .studioDark : .brightLight)
    }
  }
  private func nodes(_ node: Element) -> [Element] { [node] + node.children.flatMap(nodes) }
  private func label(_ node: Element) -> String {
    nodes(node).filter { $0.kind == .text }.map(\.props.text).joined(separator: " ")
  }
  func testNativeActivationThemeResizeAndDisabledState() throws {
    StateRegistry.shared.reset()
    WindowsDrawingBackend.prepareApplication()
    _ = NativeAPI.Application.shared.isRunning()
    let window = try XCTUnwrap(NativeAPI.Window.create())
    window.setTitle("DazzUI Windows native integration test")
    window.setContentSize(NativeAPI.Size(width: 640, height: 480))
    let backend = try XCTUnwrap(WindowsDrawingBackend(window: window))
    let renderer = Renderer(
      backend: backend, root: AnyView(Probe()),
      size: SwiftUIRuntime.Size(width: 640, height: 480),
      layoutEngine: LayoutEngine(textMeasurer: WindowsDrawingBackend.measureText))
    backend.interactionHandler = { [weak renderer] id, event in renderer?.interact(id, event) }
    backend.onResize = { [weak renderer] in renderer?.resize(to: $0) }
    renderer.mount()
    window.show()
    defer { window.hide() }

    let buttons = nodes(renderer.tree!).filter { $0.kind == .styledButton }
    XCTAssertEqual(buttons.count, 3)
    let counter = buttons[0]
    let disabled = buttons[1]
    let theme = buttons[2]
    let native = try XCTUnwrap(backend.nativeButton(counter.id))
    let originalHandle = native.nativeObject
    let hwnd = try XCTUnwrap(originalHandle).assumingMemoryBound(to: HWND__.self)
    func hover(_ inside: Bool) {
      // Scope input and deferred callbacks to this test's own native control.
      SendMessageW(hwnd, UINT(inside ? WM_MOUSEMOVE : WM_MOUSELEAVE), 0, 0)
      var message = MSG()
      while PeekMessageW(
        &message, GetParent(hwnd), UINT(WM_APP), UINT(WM_APP + 1024), UINT(PM_REMOVE))
      {
        DispatchMessageW(&message)
      }
    }
    func colors() -> [SwiftUIRuntime.Color?] {
      nodes(renderer.tree!).filter { $0.kind == .shape }.map(\.props.background)
    }
    let idleColors = colors()
    for _ in 0..<10 {
      hover(true)
      XCTAssertNotEqual(colors(), idleColors)
      let hoverColors = colors()
      hover(true)
      XCTAssertEqual(colors(), hoverColors, "Moving inside a button must keep its hover state")
      XCTAssertEqual(backend.nativeButton(counter.id)?.nativeObject, originalHandle)
      hover(false)
      XCTAssertEqual(colors(), idleColors)
    }
    backend.activate(counter.id)
    backend.activate(counter.id)
    XCTAssertTrue(nodes(renderer.tree!).contains { $0.props.text == "Count 2" })
    backend.activate(disabled.id)
    XCTAssertTrue(nodes(renderer.tree!).contains { $0.props.text == "Count 2" })
    XCTAssertFalse(try XCTUnwrap(backend.nativeButton(disabled.id)).isEnabled)
    native.focus()
    XCTAssertTrue(native.isFocused)
    backend.advanceFocus(from: counter.id)
    XCTAssertTrue(
      try XCTUnwrap(backend.nativeButton(theme.id)).isFocused, "Tab skips disabled controls")
    backend.advanceFocus(from: theme.id, backward: true)
    XCTAssertTrue(native.isFocused)

    let initialShapes = nodes(renderer.tree!).filter { $0.kind == .shape }.map(\.props.background)
    backend.activate(theme.id)
    XCTAssertTrue(nodes(renderer.tree!).contains { $0.props.text == "Dark" })
    XCTAssertTrue(nodes(renderer.tree!).contains { $0.props.text == "Count 2" })
    XCTAssertNotEqual(
      initialShapes, nodes(renderer.tree!).filter { $0.kind == .shape }.map(\.props.background))
    XCTAssertEqual(backend.nativeButton(counter.id)?.nativeObject, originalHandle)

    window.setContentSize(NativeAPI.Size(width: 820, height: 600))
    renderer.resize(
      to: SwiftUIRuntime.Size(width: window.contentSize.width, height: window.contentSize.height))
    XCTAssertEqual(renderer.tree!.frame.width, 820, accuracy: 1)
    XCTAssertEqual(backend.nativeButton(counter.id)?.nativeObject, originalHandle)
    let current = try XCTUnwrap(nodes(renderer.tree!).first { $0.id == counter.id })
    XCTAssertEqual(native.frame.width, current.frame.width, accuracy: 1)
    XCTAssertEqual(native.frame.height, current.frame.height, accuracy: 1)
    XCTAssertEqual(label(current), "Count 2")
  }
}
