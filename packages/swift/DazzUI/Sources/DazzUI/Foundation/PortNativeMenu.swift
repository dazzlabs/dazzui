#if (os(Windows) || os(Linux)) && !CROSS_SWIFTUI_HARMONY
  import Foundation
  import NativeAPI
  import SwiftUI

  /// GeometryReader's global frame is relative to this backend's content area.
  private struct PortMenuGeometry: Equatable {
    let frame: CGRect
    let shown: Bool
    let presentation: MenuPresentation
    let canSwitch: Bool
  }

  internal struct PortNativeMenuAnchor: SwiftUI.View {
    @Binding var isPresented: Bool
    let presentation: MenuPresentation
    let entries: [NativeMenuEntry]
    let theme: Theme
    let align: MenuAlign
    let point: CGPoint?
    let bar: NativeMenuBar?
    let menuID: String?
    let switchMenu: () -> Void
    @Environment(\.nativeWindow) private var window
    @StateObject private var anchor = PortNativeMenuController()

    var body: some SwiftUI.View {
      // Register the binding read during evaluation, before the layout closure.
      let shown = isPresented
      return GeometryReader { geometry in
        let snapshot = PortMenuGeometry(
          frame: geometry.frame(in: .global), shown: shown, presentation: presentation,
          canSwitch: menuID != nil && !entries.isEmpty)
        Color.clear
          .onAppear { update(snapshot) }
          .onChange(of: snapshot) { update($0) }
          .onDisappear { anchor.cancel() }
      }
    }

    private func update(_ geometry: PortMenuGeometry) {
      anchor.frame = geometry.frame
      anchor.switchMenu = switchMenu
      anchor.canSwitch = geometry.canSwitch
      bar?.add(anchor)
      anchor.update(
        window: window, shown: geometry.shown, presentation: presentation,
        entries: entries, theme: theme, align: align, point: point, bar: bar,
        dismiss: { isPresented = false })
    }
  }

  @MainActor
  internal final class PortNativeMenuController: ObservableObject {
    var frame = CGRect.zero
    var canSwitch = false
    var switchMenu: () -> Void = {}
    private(set) var window: NativeAPI.Window?
    private(set) var session: NativeMenuSession?
    private var listener: NativeAPI.ListenerId?
    private var generation = 0
    private var presentation: MenuPresentation?

    func update(
      window: NativeAPI.Window?, shown: Bool, presentation: MenuPresentation,
      entries: [NativeMenuEntry], theme: Theme, align: MenuAlign, point: CGPoint?,
      bar: NativeMenuBar?, dismiss: @escaping () -> Void
    ) {
      let changed = self.presentation != nil && self.presentation != presentation
      self.presentation = presentation
      if self.window?.id != window?.id {
        cancel()
        if let listener { _ = self.window?.removeListener(listener) }
        self.window = window
        listener = window?.addListener { [weak self] event in
          switch event {
          case .closed, .resized:
            guard self?.session != nil else { return }
            self?.cancel()
            dismiss()
          default: break
          }
        }
      }
      guard shown, presentation.usesNativeMenus, !changed else {
        cancel()
        if changed && shown { dismiss() }
        return
      }
      guard session == nil else { return }
      guard let window else {
        dismiss()
        return
      }
      generation += 1
      let token = generation
      let session = NativeMenuSession(entries: entries, theme: theme)
      self.session = session
      session.didClose = { [weak self] in
        guard let self, self.generation == token else { return }
        self.session = nil
        dismiss()
      }
      session.whileTracking = { [weak self, weak bar] in
        if let self { bar?.switchAtPointer(from: self) }
      }
      let alignOffset: CGFloat
      switch align {
      case .start: alignOffset = 0
      case .center: alignOffset = frame.width / 2
      case .end: alignOffset = frame.width
      }
      let x = frame.minX + (point?.x ?? alignOffset)
      let y = frame.minY + (point?.y ?? frame.height)
      let placement: NativeAPI.Placement
      if point != nil {
        placement = .bottomStart
      } else {
        switch align {
        case .start: placement = .bottomStart
        case .center: placement = .bottom
        case .end: placement = .bottomEnd
        }
      }
      session.present(
        strategy: .relativeWithWindowAndOffset(window, offset: NativeAPI.Point(x: x, y: y)),
        placement: placement)
    }

    func cancel() {
      generation += 1
      session?.cancel()
      session = nil
    }

    deinit {
      if let listener { _ = window?.removeListener(listener) }
    }
  }
#endif
