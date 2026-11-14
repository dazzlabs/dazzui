import SwiftUI
#if (os(Windows) || os(Linux)) && !SWIFTUI_PORT_HARMONY
import NativeAPI
#endif

#if canImport(AppKit)
  import AppKit
#endif

/// Headings and choices retain their original order, including mixed Select groups.
internal enum NativeMenuEntry {
  case item(MenuItem)
  case heading(String)
}

extension SwiftUI.View {
  func nativeMenu(
    isPresented: Binding<Bool>, entries: [NativeMenuEntry],
    align: MenuAlign = .start, point: CGPoint? = nil,
    bar: NativeMenuBar? = nil, menuID: String? = nil,
    switchMenu: @escaping () -> Void = {}
  ) -> some SwiftUI.View {
    modifier(
      NativeMenuModifier(
        isPresented: isPresented, entries: entries,
        align: align, point: point, bar: bar, menuID: menuID, switchMenu: switchMenu))
  }
}

private struct NativeMenuModifier: ViewModifier {
  @Binding var isPresented: Bool
  let entries: [NativeMenuEntry]
  let align: MenuAlign
  let point: CGPoint?
  let bar: NativeMenuBar?
  let menuID: String?
  let switchMenu: () -> Void
  @Environment(\.menuPresentation) private var presentation
  @Environment(\.theme) private var theme

  func body(content: Content) -> some SwiftUI.View {
    content
      .background {
        #if canImport(AppKit)
          NativeMenuAnchor(
            isPresented: $isPresented, presentation: presentation,
            entries: entries, theme: theme, align: align, point: point,
            bar: bar, menuID: menuID, switchMenu: switchMenu)
        #elseif (os(Windows) || os(Linux)) && !SWIFTUI_PORT_HARMONY
          PortNativeMenuAnchor(
            isPresented: $isPresented, presentation: presentation,
            entries: entries, theme: theme, align: align, point: point,
            bar: bar, menuID: menuID, switchMenu: switchMenu)
        #endif
      }
      .onChange(of: presentation) { _ in isPresented = false }
      .onDisappear { isPresented = false }
  }
}

/// One coordinator per in-window menubar, never the application's main menu.
internal final class NativeMenuBar: ObservableObject {
  #if canImport(AppKit)
    private let anchors = NSHashTable<NativeMenuAnchorView>.weakObjects()
    func add(_ anchor: NativeMenuAnchorView) { anchors.add(anchor) }
    @MainActor func switchAtPointer(from current: NativeMenuAnchorView, point: NSPoint? = nil) {
      let point = point ?? NSEvent.mouseLocation
      guard
        let next = anchors.allObjects.first(where: { anchor in
          guard anchor !== current, anchor.canSwitch, let window = anchor.window else {
            return false
          }
          return window.convertToScreen(anchor.convert(anchor.bounds, to: nil)).contains(point)
        })
      else { return }
      current.cancel()
      next.switchMenu()
    }
  #elseif (os(Windows) || os(Linux)) && !SWIFTUI_PORT_HARMONY
    private struct WeakAnchor { weak var value: PortNativeMenuController? }
    private var anchors: [WeakAnchor] = []
    @MainActor func add(_ anchor: PortNativeMenuController) {
      anchors.removeAll { $0.value == nil }
      if !anchors.contains(where: { $0.value === anchor }) {
        anchors.append(WeakAnchor(value: anchor))
      }
    }
    @MainActor func switchAtPointer(from current: PortNativeMenuController) {
      guard let window = current.window else { return }
      let cursor = NativeAPI.DisplayManager.shared.getCursorPosition()
      let bounds = window.contentBounds
      let point = CGPoint(x: cursor.x - bounds.x, y: cursor.y - bounds.y)
      guard let next = anchors.compactMap(\.value).first(where: {
        $0 !== current && $0.canSwitch && $0.window?.id == window.id && $0.frame.contains(point)
      }) else { return }
      current.cancel()
      next.switchMenu()
    }
  #endif
}

#if canImport(AppKit)
  private struct NativeMenuAnchor: NSViewRepresentable {
    @Binding var isPresented: Bool
    let presentation: MenuPresentation
    let entries: [NativeMenuEntry]
    let theme: Theme
    let align: MenuAlign
    let point: CGPoint?
    let bar: NativeMenuBar?
    let menuID: String?
    let switchMenu: () -> Void

    func makeNSView(context: Context) -> NativeMenuAnchorView { NativeMenuAnchorView() }
    func updateNSView(_ view: NativeMenuAnchorView, context: Context) {
      view.canSwitch = !entries.isEmpty && menuID != nil
      view.switchMenu = switchMenu
      bar?.add(view)
      view.update(
        shown: isPresented, presentation: presentation, entries: entries,
        theme: theme, align: align, point: point, bar: bar,
        dismiss: { isPresented = false })
    }
    static func dismantleNSView(_ view: NativeMenuAnchorView, coordinator: ()) { view.cancel() }
  }

  @MainActor
  internal final class NativeMenuAnchorView: NSView {
    var canSwitch = false
    var switchMenu: () -> Void = {}
    private(set) var session: NativeMenuSession?
    private var generation = 0
    private var presentation: MenuPresentation?
    private var closeObserver: NSObjectProtocol?

    override func hitTest(_ point: NSPoint) -> NSView? { nil }
    override func viewDidMoveToWindow() {
      super.viewDidMoveToWindow()
      if window == nil { cancel() }
    }

    func update(
      shown: Bool, presentation: MenuPresentation, entries: [NativeMenuEntry],
      theme: Theme, align: MenuAlign, point: CGPoint?, bar: NativeMenuBar?,
      dismiss: @escaping () -> Void
    ) {
      let changed = self.presentation != nil && self.presentation != presentation
      self.presentation = presentation
      guard !changed, shown, presentation.usesNativeMenus else {
        cancel()
        if changed && shown { DispatchQueue.main.async { dismiss() } }
        return
      }
      guard session == nil else { return }
      generation += 1
      let token = generation
      let session = NativeMenuSession(
        entries: entries, theme: theme, scale: window?.backingScaleFactor ?? 2)
      self.session = session
      session.didClose = { [weak self] in
        guard let self, self.generation == token else { return }
        self.cleanUp()
        dismiss()
      }
      session.whileTracking = { [weak self, weak bar] in
        guard let self else { return }
        bar?.switchAtPointer(from: self)
      }
      DispatchQueue.main.async { [weak self] in
        guard let self, self.generation == token else { return }
        guard let window = self.window else {
          self.cancel()
          dismiss()
          return
        }
        self.closeObserver = NotificationCenter.default.addObserver(
          forName: NSWindow.willCloseNotification, object: window, queue: .main
        ) { [weak self] _ in
          MainActor.assumeIsolated {
            self?.cancel()
            dismiss()
          }
        }
        var location = NSPoint(x: 0, y: 0)
        if let point {
          location = NSPoint(x: point.x, y: self.bounds.height - point.y)
        } else if align == .end {
          location.x = self.bounds.width
        }
        session.present(at: location, in: self, align: point == nil ? align : .start)
      }
    }

    private func cleanUp() {
      if let closeObserver { NotificationCenter.default.removeObserver(closeObserver) }
      closeObserver = nil
      session = nil
    }

    func cancel() {
      generation += 1
      session?.cancel()
      cleanUp()
    }

    deinit {
      if let closeObserver { NotificationCenter.default.removeObserver(closeObserver) }
    }
  }
#endif
