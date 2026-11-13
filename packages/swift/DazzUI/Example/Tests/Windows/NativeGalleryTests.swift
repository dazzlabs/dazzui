import DazzUI
import NativeAPI
import SwiftUI
import WinSDK
import XCTest

@testable import Example

final class NativeGalleryTests: XCTestCase {
  func testNativeThemePreviewInkIsStableAcrossCardHover() throws {
    try withWindow(
      AppView(page: "button", sizing: .system).frame(minWidth: 960, minHeight: 600),
      schedulesUpdates: true
    ) { renderer, backend in
      func pump(for seconds: Double = 0.05) {
        let until = Date().addingTimeInterval(seconds)
        repeat {
          var message = MSG()
          while PeekMessageW(&message, nil, 0, 0, UINT(PM_REMOVE)) {
            TranslateMessage(&message)
            DispatchMessageW(&message)
          }
          Thread.sleep(forTimeInterval: 0.001)
        } while Date() < until
      }
      backend.advanceFocus(from: nil)
      pump()
      let theme = try XCTUnwrap(button(renderer, "Tokyo Night"))
      renderer.performTap(theme.id)
      pump()
      let scroll = try XCTUnwrap(
        nodes(renderer.tree!).first { $0.kind == .scroll && $0.frame.minX > 1000 })
      renderer.input(
        event: 5, point: CGPoint(x: scroll.frame.minX + 70, y: scroll.frame.minY + 150),
        value: -1200)
      pump()
      let card = try XCTUnwrap(button(renderer, "Tokyo Night"))
      let handle = try XCTUnwrap(backend.nativeButton(card.id)?.nativeObject)
      let hwnd = handle.assumingMemoryBound(to: HWND__.self)
      let preview = try XCTUnwrap(nodes(card).first { $0.kind == .geometry })
      let ink = nodes(preview).filter {
        $0.kind == .shape && $0.frame.height > 0 && $0.frame.height <= 3 && $0.frame.width > 3
      }
      XCTAssertEqual(ink.count, 2)
      func pixels() throws -> [COLORREF] {
        var rect = RECT()
        GetClientRect(hwnd, &rect)
        let dc = try XCTUnwrap(GetDC(hwnd))
        defer { ReleaseDC(hwnd, dc) }
        let memory = try XCTUnwrap(CreateCompatibleDC(dc))
        defer { DeleteDC(memory) }
        let bitmap = try XCTUnwrap(CreateCompatibleBitmap(dc, rect.right, rect.bottom))
        let old = SelectObject(memory, bitmap)
        defer { SelectObject(memory, old); DeleteObject(bitmap) }
        SendMessageW(
          hwnd, UINT(WM_PRINTCLIENT), WPARAM(UInt(bitPattern: memory)), LPARAM(PRF_CLIENT))
        let scale = Double(GetDpiForWindow(hwnd)) / 96
        return ink.map { line in
          GetPixel(
            memory,
            Int32(((line.frame.minX + line.frame.width / 2 - card.frame.minX) * scale).rounded()),
            Int32(((line.frame.minY + line.frame.height / 2 - card.frame.minY) * scale).rounded()))
        }
      }
      let settled = Date().addingTimeInterval(0.4)
      while Date() < settled { pump(); Thread.sleep(forTimeInterval: 0.008) }
      let initial = try pixels()
      XCTAssertFalse(initial.contains(COLORREF(CLR_INVALID)))
      XCTAssertNotEqual(initial.first, initial.last, "Sample both opaque and translucent ink")
      let other = try XCTUnwrap(button(renderer, "filled"))
      for target in [card, other] {
        for hovering in [true, false, true, false] {
          renderer.input(
            event: hovering ? 1 : 10,
            point: CGPoint(
              x: target.frame.minX + target.frame.width / 2,
              y: target.frame.minY + target.frame.height / 2))
          let until = Date().addingTimeInterval(0.3)
          repeat {
            pump(for: 0.001)
            let all = nodes(renderer.tree!)
            XCTAssertFalse(all.contains { $0.id.rawValue == 0 })
            XCTAssertEqual(Set(all.map(\.id)).count, all.count)
            let current = try XCTUnwrap(all.first { $0.id == ink[1].id })
            XCTAssertEqual(current.props.opacity, 0.4)
            XCTAssertEqual(
              try pixels(), initial,
              "Preview ink must remain stable during native hover and animation frames")
            Thread.sleep(forTimeInterval: 0.008)
          } while Date() < until
        }
      }
    }
  }

  func testDisplayScrollBenchmark() throws {
    guard let path = ProcessInfo.processInfo.environment["SWIFTUI_PORT_DISPLAY_PATH"] else {
      throw XCTSkip("Set SWIFTUI_PORT_DISPLAY_PATH and run the desktop frame probe")
    }
    try withWindow(AppView(page: "button")) { renderer, backend in
      let control = try XCTUnwrap(button(renderer, "filled"))
      let handle = try XCTUnwrap(backend.nativeButton(control.id)?.nativeObject)
      let host = GetAncestor(handle.assumingMemoryBound(to: HWND__.self), UINT(GA_ROOT))
      // Test-only sequence painted after the real WM_PAINT, decoded independently
      // from composed desktop pixels. No production drawing or input is altered.
      let marker = UnsafeMutablePointer<UInt32>.allocate(capacity: 1)
      marker.initialize(to: 0)
      let markerProc: SUBCLASSPROC = { hwnd, message, wparam, lparam, _, data in
        let result = DefSubclassProc(hwnd, message, wparam, lparam)
        if message == UINT(WM_PAINT), let counter = UnsafePointer<UInt32>(bitPattern: UInt(data)),
          counter.pointee > 0
        {
          var client = RECT()
          GetClientRect(hwnd, &client)
          let left = client.right * 30 / 100 + 8
          let top = client.bottom * 15 / 100 + 8
          let value =
            UInt64(0xA55A) | (UInt64(counter.pointee & 0xffff) << 16)
            | (UInt64((~counter.pointee) & 0xffff) << 32)
          if let dc = GetDC(hwnd) {
            let white = CreateSolidBrush(COLORREF(0xffffff))
            let black = CreateSolidBrush(0)
            for bit in 0..<48 {
              var rect = RECT(
                left: left + LONG(bit * 3), top: top,
                right: left + LONG(bit * 3 + 3), bottom: top + 8)
              FillRect(dc, &rect, value & (UInt64(1) << bit) != 0 ? white : black)
            }
            GdiFlush()
            DeleteObject(white)
            DeleteObject(black)
            ReleaseDC(hwnd, dc)
          }
        }
        return result
      }
      XCTAssertTrue(
        SetWindowSubclass(host, markerProc, 0x46524d, DWORD_PTR(UInt(bitPattern: marker))))
      defer {
        RemoveWindowSubclass(host, markerProc, 0x46524d)
        marker.deinitialize(count: 1)
        marker.deallocate()
      }
      SetWindowPos(
        host, HWND(bitPattern: -1), 0, 0, 0, 0,
        UINT(SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE))
      let center = try XCTUnwrap(
        nodes(renderer.tree!).first {
          $0.kind == .scroll && $0.frame.minX > 270 && $0.frame.minX < 500
        })
      // Keep the physical pointer untouched. Position this test window so a
      // gallery buttons scroll through its column, ensuring native hover overlap
      // is exercised regardless of Windows' cascading default window positions.
      var cursor = POINT(), outer = RECT(), origin = POINT()
      GetCursorPos(&cursor)
      GetWindowRect(host, &outer)
      ClientToScreen(host, &origin)
      var monitor = MONITORINFO()
      monitor.cbSize = DWORD(MemoryLayout<MONITORINFO>.size)
      GetMonitorInfoW(MonitorFromPoint(cursor, DWORD(MONITOR_DEFAULTTONEAREST)), &monitor)
      let scale = Double(GetDpiForWindow(host)) / 96
      var position = POINT(x: outer.left, y: outer.top)
      var closest = Double.infinity
      for candidate in nodes(renderer.tree!) where backend.nativeButton(candidate.id) != nil {
        let x = candidate.frame.minX + candidate.frame.width / 2
        let y = candidate.frame.minY + candidate.frame.height / 2
        guard x > center.frame.minX, x < center.frame.maxX,
          y > center.frame.minY, y < center.frame.maxY else { continue }
        let wantedX = Double(cursor.x) - x * scale - Double(origin.x - outer.left)
        let wantedY = Double(cursor.y) - y * scale - Double(origin.y - outer.top)
        let actualX = max(Double(monitor.rcWork.left), min(wantedX,
          Double(monitor.rcWork.right - (outer.right - outer.left))))
        let actualY = max(Double(monitor.rcWork.top), min(wantedY,
          Double(monitor.rcWork.bottom - (outer.bottom - outer.top))))
        let distance = abs(wantedX - actualX) + abs(wantedY - actualY)
        if distance < closest {
          closest = distance
          position = POINT(x: LONG(actualX.rounded()), y: LONG(actualY.rounded()))
        }
      }
      SetWindowPos(host, HWND(bitPattern: -1), position.x, position.y, 0, 0,
        UINT(SWP_NOSIZE | SWP_NOACTIVATE))
      // The observer only binds after positioning is complete.
      "DazzUI display frame test".withCString(encodedAs: UTF16.self) { SetWindowTextW(host, $0) }
      var frequency = LARGE_INTEGER()
      QueryPerformanceFrequency(&frequency)
      let hz = Double(ProcessInfo.processInfo.environment["SWIFTUI_PORT_DISPLAY_HZ"] ?? "60") ?? 60
      let count =
        Int(ProcessInfo.processInfo.environment["SWIFTUI_PORT_DISPLAY_SAMPLES"] ?? "1200") ?? 1200
      // SYNCHRONIZE | TIMER_MODIFY_STATE; compound SDK macros are not imported.
      let timer = try XCTUnwrap(CreateWaitableTimerExW(nil, nil, DWORD(2), DWORD(0x0010_0002)))
      defer { CloseHandle(timer) }
      func now() -> Int64 {
        var time = LARGE_INTEGER()
        QueryPerformanceCounter(&time)
        return time.QuadPart
      }
      func pump() {
        var message = MSG()
        while PeekMessageW(&message, host, 0, 0, UINT(PM_REMOVE)) { DispatchMessageW(&message) }
      }
      func wait(until deadline: Int64) {
        let remaining = Double(deadline - now()) / Double(frequency.QuadPart)
        if remaining > 0 {
          var due = LARGE_INTEGER()
          due.QuadPart = -max(1, Int64(remaining * 10_000_000))
          SetWaitableTimer(timer, &due, 0, nil, nil, false)
          WaitForSingleObject(timer, DWORD(INFINITE))
        }
      }
      // Let the independent DXGI observer bind before producing frames.
      pump()
      wait(until: now() + frequency.QuadPart)
      var samples: [[String: Any]] = []
      var deadline = now()
      for index in 0..<(count + 30) {
        wait(until: deadline)
        let started = now()
        let paintsBefore = backend.totalPaintTimings
        let publicationsBefore = backend.publicationCount
        let fullBefore = backend.fullPublicationCount
        let hoverBefore = backend.nativeHoverEventCount
        let enterBefore = backend.nativeHoverEnterCount
        let pointerBefore = backend.totalPointerQueryMilliseconds
        marker.pointee = UInt32(index + 1)
        backend.pointer(
          event: 5, at: CGPoint(x: center.frame.minX + 30, y: center.frame.minY + 100),
          value: index % 20 < 10 ? -120 : 120)
        // Use the same queued WM_PAINT path as the application's message loop.
        // Do not force a paint or wait on DWM inside the measured update.
        pump()
        let finished = now()
        var sample: [String: Any] = renderer.lastTimings
        for (key, value) in backend.lastTimings { sample["backend." + key] = value }
        sample["phase"] = index < 30 ? "warmup" : "steady"
        sample["start_qpc"] = started
        sample["finish_qpc"] = finished
        sample["deadline_qpc"] = deadline
        sample["cpu_ms"] = Double(finished - started) * 1000 / Double(frequency.QuadPart)
        sample["paint_count"] = backend.lastPaintTimings["count"] ?? 0
        sample["frame_id"] = index + 1
        sample["publications"] = backend.publicationCount - publicationsBefore
        sample["full_publications"] = backend.fullPublicationCount - fullBefore
        sample["native_hover_events"] = backend.nativeHoverEventCount - hoverBefore
        sample["native_hover_enter"] = backend.nativeHoverEnterCount - enterBefore
        sample["pointer_query_ms"] = backend.totalPointerQueryMilliseconds - pointerBefore
        for (key, value) in backend.totalPaintTimings {
          sample["all_paint." + key] = value - (paintsBefore[key] ?? 0)
        }
        XCTAssertGreaterThan(backend.lastPaintTimings["count"] ?? 0, 0)
        samples.append(sample)
        deadline += Int64(Double(frequency.QuadPart) / hz)
        // A missed deadline must be reported, not hidden by catch-up bursts.
        if deadline < finished { deadline = finished }
      }
      wait(until: now() + frequency.QuadPart)
      var finalCursor = POINT()
      GetCursorPos(&finalCursor)
      XCTAssertEqual(finalCursor.x, cursor.x, "Pointer moved during display benchmark")
      XCTAssertEqual(finalCursor.y, cursor.y, "Pointer moved during display benchmark")
      let report: [String: Any] = [
        "frequency": frequency.QuadPart, "target_hz": hz, "samples": samples,
        "pointer_before": [cursor.x, cursor.y],
        "pointer_after": [finalCursor.x, finalCursor.y],
        "window_origin": [position.x, position.y],
      ]
      XCTAssertGreaterThan(samples.reduce(0) { $0 + ($1["native_hover_events"] as? Int ?? 0) },
        count / 50, "Continuous scrolling must exercise native hover transitions")
      try JSONSerialization.data(withJSONObject: report, options: [.prettyPrinted, .sortedKeys])
        .write(to: URL(fileURLWithPath: path))
    }
  }
  func testRenderingBenchmark() throws {
    guard let path = ProcessInfo.processInfo.environment["SWIFTUI_PORT_BENCHMARK_PATH"] else {
      throw XCTSkip("Set SWIFTUI_PORT_BENCHMARK_PATH to record rendering timings")
    }
    try withWindow(AppView(page: "button")) { renderer, backend in
      let control = try XCTUnwrap(button(renderer, "filled"))
      let handle = try XCTUnwrap(backend.nativeButton(control.id)?.nativeObject)
      let hwnd = handle.assumingMemoryBound(to: HWND__.self)
      // The renderer may nest native controls under scrolling content HWNDs.
      let host = GetAncestor(hwnd, UINT(GA_ROOT))
      // Occluded/minimized windows may suppress WM_PAINT. Keep this short
      // benchmark visible without taking keyboard focus, and reject no-paint samples.
      SetWindowPos(
        host, HWND(bitPattern: -1), 0, 0, 0, 0,
        UINT(SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE))
      var samples: [[String: Any]] = []
      let sustained = Int(
        ProcessInfo.processInfo.environment["SWIFTUI_PORT_BENCHMARK_SAMPLES"] ?? "")
      var phase = sustained == nil ? "steady" : "warmup"
      func sample(_ name: String, _ action: () -> Void) {
        let start = Date.timeIntervalSinceReferenceDate
        action()
        var message = MSG()
        while PeekMessageW(&message, host, UINT(WM_APP), UINT(WM_APP + 1024), UINT(PM_REMOVE)) {
          DispatchMessageW(&message)
        }
        XCTAssertTrue(IsWindow(host), "The benchmark must keep its native surface")
        var damage = RECT()
        GetUpdateRect(host, &damage, false)
        let painted = Date.timeIntervalSinceReferenceDate
        RedrawWindow(host, nil, nil, UINT(RDW_ALLCHILDREN | RDW_UPDATENOW))
        var record: [String: Any] = renderer.lastTimings
        for (key, value) in backend.lastTimings { record["backend." + key] = value }
        record["operation"] = name
        record["phase"] = phase
        record["damagePixels"] = (damage.right - damage.left) * (damage.bottom - damage.top)
        record["paint"] = (Date.timeIntervalSinceReferenceDate - painted) * 1000
        record["reusedPixels"] = backend.lastReusedPixels
        for (key, value) in backend.lastPaintTimings { record["paint." + key] = value }
        XCTAssertTrue(
          IsWindowVisible(host) && !IsIconic(host), "The benchmark window must remain visible")
        XCTAssertGreaterThan(
          backend.lastPaintTimings["count"] ?? 0, 0, "Every sample must paint its updated frame")
        record["total"] = (Date.timeIntervalSinceReferenceDate - start) * 1000
        samples.append(record)
      }
      func colors() -> [SwiftUIRuntime.Color?] {
        guard let current = node(control.id, in: renderer.tree!) else { return [] }
        return nodes(current).filter { $0.kind == .shape }.map(\.props.background)
      }
      let idleColors = colors()
      func hover(_ inside: Bool) {
        sample("hover") { SendMessageW(hwnd, UINT(inside ? WM_MOUSEMOVE : WM_MOUSELEAVE), 0, 0) }
        if inside {
          XCTAssertNotEqual(colors(), idleColors)
        } else {
          XCTAssertEqual(colors(), idleColors)
        }
      }
      if sustained != nil {
        for _ in 0..<2 {
          hover(true)
          hover(false)
        }
        phase = "steady"
      }
      for _ in 0..<max(1, (sustained ?? 12) / 2) {
        hover(true)
        hover(false)
      }
      let center = try XCTUnwrap(
        nodes(renderer.tree!).first {
          $0.kind == .scroll && $0.frame.minX > 270 && $0.frame.minX < 500
        })
      let deltas =
        sustained.map { count in
          (0..<max(1, count)).map { $0 % 20 < 10 ? -120 : 120 }
        } ?? [-120, -120, -120, 120, 120, 120]
      for delta in deltas {
        sample("scroll") {
          backend.pointer(
            event: 5, at: CGPoint(x: center.frame.minX + 30, y: center.frame.minY + 100),
            value: delta)
        }
      }
      for label in ["Switch to dark", "Switch to light"] {
        let theme = try XCTUnwrap(
          nodes(renderer.tree!).first { $0.props.accessibilityLabel == label })
        let button = try XCTUnwrap(nodes(theme).first { $0.kind == .styledButton })
        sample("theme") { backend.activate(button.id) }
      }
      let data = try JSONSerialization.data(
        withJSONObject: samples, options: [.prettyPrinted, .sortedKeys])
      try data.write(to: URL(fileURLWithPath: path))
      print("Rendering benchmark: \(samples.count) samples written to \(path)")
    }
  }
  private func nodes(_ tree: Element) -> [Element] { [tree] + tree.children.flatMap(nodes) }
  private func node(_ id: NodeID, in tree: Element) -> Element? {
    if tree.id == id { return tree }
    for child in tree.children { if let found = node(id, in: child) { return found } }
    return nil
  }
  private func button(_ renderer: Renderer, _ title: String) -> Element? {
    nodes(renderer.tree!).first {
      $0.kind == .styledButton && nodes($0).contains { $0.props.text == title }
    }
  }
  private func withWindow<V: SwiftUI.View>(
    _ view: V, schedulesUpdates: Bool = false, _ test: (Renderer, WindowsDrawingBackend) throws -> Void
  ) throws {
    StateRegistry.shared.reset()
    WindowsDrawingBackend.prepareApplication()
    _ = NativeAPI.Application.shared.isRunning()
    let window = try XCTUnwrap(NativeAPI.Window.create())
    window.setTitle("DazzUI full gallery integration test")
    window.setContentSize(NativeAPI.Size(width: 1360, height: 860))
    let backend = try XCTUnwrap(WindowsDrawingBackend(window: window, schedulesUpdates: schedulesUpdates))
    let renderer = Renderer(
      backend: backend, root: AnyView(view), size: SwiftUIRuntime.Size(width: 1360, height: 860),
      layoutEngine: LayoutEngine(textMeasurer: WindowsDrawingBackend.measureText))
    backend.interactionHandler = { [weak renderer] id, event in renderer?.interact(id, event) }
    backend.inputHandler = { [weak renderer] id, event, point, value, modifiers, text in
      renderer?.input(
        id: id, event: event, point: point, value: value, modifiers: modifiers, text: text) ?? false
    }
    renderer.mount()
    window.show()
    defer { window.hide() }
    try test(renderer, backend)
  }
  private final class AsyncModel: ObservableObject { @Published var text = "waiting" }
  private struct AsyncProbe: SwiftUI.View {
    @ObservedObject var model: AsyncModel
    var body: some SwiftUI.View {
      Text(model.text).task {
        try? await Task.sleep(nanoseconds: 50_000_000)
        model.text = "ready"
      }
    }
  }
  func testNativeMainQueueRunsTasksAndTimers() throws {
    StateRegistry.shared.reset()
    WindowsDrawingBackend.prepareApplication()
    _ = NativeAPI.Application.shared.isRunning()
    let window = try XCTUnwrap(NativeAPI.Window.create())
    window.setTitle("SwiftUI asynchronous lifecycle test")
    let backend = try XCTUnwrap(WindowsDrawingBackend(window: window))
    let model = AsyncModel()
    let renderer = Renderer(
      backend: backend, root: AnyView(AsyncProbe(model: model)),
      size: SwiftUIRuntime.Size(width: 300, height: 200))
    renderer.mount()
    window.show()
    let finished = DispatchSemaphore(value: 0)
    Thread.detachNewThread {
      if finished.wait(timeout: .now() + 3) == .timedOut { NativeAPI.Application.shared.quit(1) }
    }
    DispatchQueue.main.asyncAfter(deadline: .now() + 0.2) { NativeAPI.Application.shared.quit(0) }
    let started = Date()
    let result = NativeAPI.Application.shared.runWithWindow(window)
    finished.signal()
    window.hide()
    XCTAssertEqual(result, 0)
    XCTAssertLessThan(Date().timeIntervalSince(started), 2)
    XCTAssertEqual(model.text, "ready")
    XCTAssertTrue(nodes(renderer.tree!).contains { $0.props.text == "ready" })
  }
  func testNativeSearchNavigationThemeAndScroll() throws {
    try withWindow(AppView()) { renderer, backend in
      let search = try XCTUnwrap(nodes(renderer.tree!).first { $0.kind == .textField })
      backend.replaceText(search.id, with: "calendar")
      XCTAssertFalse(nodes(renderer.tree!).contains { $0.props.text == "Checkbox" })
      backend.activate(try XCTUnwrap(button(renderer, "Calendar")).id)
      XCTAssertTrue(nodes(renderer.tree!).contains { $0.props.text == Stories.calendar.summary })
      let themeControl = try XCTUnwrap(
        nodes(renderer.tree!).first {
          $0.props.accessibilityLabel == "Switch to dark"
        })
      let dark = try XCTUnwrap(nodes(themeControl).first { $0.kind == .styledButton })
      backend.activate(dark.id)
      XCTAssertTrue(
        nodes(renderer.tree!).contains { $0.props.accessibilityLabel == "Switch to light" })
      let center = try XCTUnwrap(
        nodes(renderer.tree!).first {
          $0.kind == .scroll && $0.frame.minX > 270 && $0.frame.minX < 500
        })
      let firstControl = try XCTUnwrap(
        nodes(center).first {
          $0.kind == .styledButton && backend.nativeButton($0.id)?.isVisible == true
        })
      backend.pointer(
        event: 5, at: CGPoint(x: center.frame.minX + 30, y: center.frame.minY + 100), value: -360)
      XCTAssertGreaterThan(
        nodes(renderer.tree!).first { $0.id == center.id }?.props.scrollOffset.y ?? 0, 0)
      backend.pointer(
        event: 5, at: CGPoint(x: center.frame.minX + 30, y: center.frame.minY + 100), value: -12000)
      let retained = try XCTUnwrap(backend.nativeButton(firstControl.id)?.nativeObject)
      XCTAssertFalse(
        inNativeViewport(retained), "Offscreen controls must be clipped by their native viewport")
      backend.pointer(
        event: 5, at: CGPoint(x: center.frame.minX + 30, y: center.frame.minY + 100), value: 12000)
      XCTAssertEqual(backend.nativeButton(firstControl.id)?.nativeObject, retained)
      XCTAssertTrue(inNativeViewport(retained))
    }
  }
  private func inNativeViewport(_ handle: UnsafeMutableRawPointer) -> Bool {
    let hwnd = handle.assumingMemoryBound(to: HWND__.self)
    guard IsWindowVisible(hwnd) else { return false }
    var visible = RECT()
    GetWindowRect(hwnd, &visible)
    var parent = GetParent(hwnd)
    while let ancestor = parent {
      var clip = RECT()
      GetClientRect(ancestor, &clip)
      withUnsafeMutablePointer(to: &clip) { pointer in
        pointer.withMemoryRebound(to: POINT.self, capacity: 2) {
          _ = MapWindowPoints(ancestor, nil, $0, 2)
        }
      }
      var intersection = RECT()
      guard IntersectRect(&intersection, &visible, &clip) else { return false }
      visible = intersection
      parent = GetParent(ancestor)
    }
    return true
  }
  func testNativeSelectAndDialogActions() throws {
    try withWindow(SceneView(scene: Stories.select.preview).theme(.brightLight).overlayHost()) {
      renderer, backend in
      backend.activate(try XCTUnwrap(button(renderer, "English")).id)
      XCTAssertNotNil(button(renderer, "Japanese"))
      backend.activate(try XCTUnwrap(button(renderer, "Japanese")).id)
      XCTAssertNotNil(button(renderer, "Japanese"))
      XCTAssertNil(button(renderer, "German"), "Selecting a value dismisses the popup")
    }
    try withWindow(SceneView(scene: Stories.dialog.preview).theme(.brightLight).overlayHost()) {
      renderer, backend in
      backend.activate(try XCTUnwrap(button(renderer, "Rename…")).id)
      XCTAssertTrue(nodes(renderer.tree!).contains { $0.props.text == "Rename project" })
      let field = try XCTUnwrap(nodes(renderer.tree!).first { $0.kind == .textField })
      backend.replaceText(field.id, with: "Windows project")
      XCTAssertTrue(nodes(renderer.tree!).contains { $0.props.text == "Windows project" })
      backend.activate(try XCTUnwrap(button(renderer, "Cancel")).id)
      XCTAssertFalse(nodes(renderer.tree!).contains { $0.props.text == "Rename project" })
    }
  }
  func testNativeComboboxTypingAndOtpPaste() throws {
    try withWindow(SceneView(scene: Stories.combobox.preview).theme(.brightLight)) {
      renderer, backend in
      func field() throws -> Element {
        try XCTUnwrap(nodes(renderer.tree!).first { $0.kind == .textField })
      }
      backend.replaceText(try field().id, with: "G")
      XCTAssertNotNil(button(renderer, "Germany"))
      XCTAssertTrue(try field().props.hitTesting, "The non-modal popup keeps its editor enabled")
      backend.replaceText(try field().id, with: "Ger")
      XCTAssertNotNil(button(renderer, "Germany"))
      backend.activate(try XCTUnwrap(button(renderer, "Germany")).id)
      XCTAssertEqual(try field().props.text, "Germany")
      XCTAssertNil(button(renderer, "Germany"))
    }
    try withWindow(SceneView(scene: Stories.otpField.preview).theme(.brightLight)) {
      renderer, backend in
      let field = try XCTUnwrap(nodes(renderer.tree!).first { $0.kind == .textField })
      XCTAssertEqual(field.props.opacity, 0)
      backend.replaceText(field.id, with: "12a34567")
      XCTAssertEqual(nodes(renderer.tree!).first { $0.kind == .textField }?.props.text, "123456")
      XCTAssertEqual(
        nodes(renderer.tree!).filter {
          $0.kind == .text && ["1", "2", "3", "4", "5", "6"].contains($0.props.text)
        }.count, 6)
    }
  }
  func testNativeSliderDrag() throws {
    try withWindow(SceneView(scene: Stories.slider.examples[1].demo).theme(.brightLight)) {
      renderer, backend in
      let slider = try XCTUnwrap(
        nodes(renderer.tree!).first { $0.props.accessibilityLabel == "Quality" })
      let before = slider.props.accessibilityValue
      let y = slider.frame.minY + slider.frame.height / 2
      backend.pointer(event: 2, at: CGPoint(x: slider.frame.minX + 10, y: y))
      backend.pointer(event: 1, at: CGPoint(x: slider.frame.maxX - 10, y: y))
      backend.pointer(event: 3, at: CGPoint(x: slider.frame.maxX - 10, y: y))
      let after = nodes(renderer.tree!).first { $0.props.accessibilityLabel == "Quality" }?.props
        .accessibilityValue
      XCTAssertNotEqual(before, after)
    }
  }
  func testNativeAutocompleteSelectionStaysDismissedAfterBatchedUpdate() throws {
    let example = try XCTUnwrap(Stories.combobox.examples.first { $0.title == "Autocomplete" })
    try withWindow(SceneView(scene: example.demo).theme(.brightLight)) {
      renderer, backend in
      func field() throws -> Element {
        try XCTUnwrap(nodes(renderer.tree!).first { $0.kind == .textField })
      }
      backend.replaceText(try field().id, with: "I")
      backend.activate(try XCTUnwrap(button(renderer, "Inter")).id)
      XCTAssertEqual(nodes(renderer.tree!).first { $0.kind == .textField }?.props.text, "Inter")
      XCTAssertNil(button(renderer, "Inter"))
      backend.replaceText(try field().id, with: "I")
      XCTAssertNotNil(button(renderer, "Inter"), "Typing again should reopen suggestions")
      backend.activate(try XCTUnwrap(button(renderer, "IBM Plex Sans")).id)
      XCTAssertNil(button(renderer, "IBM Plex Sans"))
    }
  }
  func testNativeGalleryHoverKeepsStateAndHandlesStable() throws {
    try withWindow(AppView(page: "button")) { renderer, backend in
      let control = try XCTUnwrap(button(renderer, "filled"))
      let native = try XCTUnwrap(backend.nativeButton(control.id))
      let handle = try XCTUnwrap(native.nativeObject)
      let hwnd = handle.assumingMemoryBound(to: HWND__.self)
      func hover(_ inside: Bool) {
        SendMessageW(hwnd, UINT(inside ? WM_MOUSEMOVE : WM_MOUSELEAVE), 0, 0)
        var message = MSG()
        while PeekMessageW(
          &message, GetAncestor(hwnd, UINT(GA_ROOT)), UINT(WM_APP), UINT(WM_APP + 1024),
          UINT(PM_REMOVE))
        {
          DispatchMessageW(&message)
        }
      }
      func colors() -> [SwiftUIRuntime.Color?] {
        guard let current = nodes(renderer.tree!).first(where: { $0.id == control.id }) else {
          return []
        }
        return nodes(current).filter { $0.kind == .shape }.map(\.props.background)
      }
      let idleColors = colors()
      let start = Date()
      for _ in 0..<3 {
        hover(true)
        XCTAssertNotEqual(colors(), idleColors)
        let hoveredColors = colors()
        hover(true)
        XCTAssertEqual(colors(), hoveredColors)
        XCTAssertEqual(native.nativeObject, handle)
        XCTAssertEqual(nodes(renderer.tree!).first { $0.id == control.id }?.frame, control.frame)
        hover(false)
        XCTAssertEqual(colors(), idleColors)
      }
      print(
        "Full gallery: six native hover transitions in \(Date().timeIntervalSince(start)) seconds")
    }
  }
}
