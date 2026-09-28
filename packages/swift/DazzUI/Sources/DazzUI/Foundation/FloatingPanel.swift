#if canImport(AppKit)
  import AppKit
#endif
import SwiftUI

public extension View {
  /// Hangs a floating sheet off this view, the counterpart of the portal and
  /// positioner a Base UI popup renders into.
  ///
  /// On macOS the sheet is drawn in a borderless child window rather than in
  /// an overlay. An overlay is only above its own siblings: a view later in an
  /// enclosing stack — the next row of a settings section, the field under a
  /// form's select — still draws over it and still takes the clicks meant for
  /// it. A window is above the whole of its parent, and it is not clipped by
  /// a scroll view either.
  ///
  /// `modal` is Base UI's axis of the same name. A modal sheet — a select's
  /// list, a menu — makes the window behind inert while it is open: a press
  /// outside closes it and goes no further, and scrolling and typing are held
  /// back. A non-modal one — a popover, a combobox's list — closes on a press
  /// outside and lets the press land, and a press on this view itself is left
  /// to it, which is what lets a combobox's field keep taking the typing and
  /// a trigger toggle its own panel. Either way Escape closes it, and so does
  /// the app or the window losing the focus.
  ///
  /// The sheet opens on `side` with `gap` between the two, and flips to the
  /// opposite side where the screen has no room. Environment does not cross a
  /// window, so the theme is carried over explicitly. `panel` is handed this
  /// view's size, for a list that has to be at least as wide as its trigger.
  func floatingPanel<Panel: View>(
    isPresented: Binding<Bool>,
    side: PopoverSide = .bottom,
    align: PopoverAlign = .start,
    gap: CGFloat,
    modal: Bool,
    @ViewBuilder panel: @escaping (_ anchor: CGSize) -> Panel
  ) -> some View {
    modifier(
      FloatingPanelModifier(
        isPresented: isPresented,
        side: side,
        align: align,
        gap: gap,
        modal: modal,
        panel: panel
      )
    )
  }

  /// A floating sheet whose content does not depend on the anchor's size.
  func floatingPanel<Panel: View>(
    isPresented: Binding<Bool>,
    side: PopoverSide = .bottom,
    align: PopoverAlign = .start,
    gap: CGFloat,
    modal: Bool,
    @ViewBuilder panel: @escaping () -> Panel
  ) -> some View {
    floatingPanel(
      isPresented: isPresented,
      side: side,
      align: align,
      gap: gap,
      modal: modal,
      panel: { _ in panel() }
    )
  }
}

private struct FloatingPanelModifier<Panel: View>: ViewModifier {
  @Binding var isPresented: Bool
  let side: PopoverSide
  let align: PopoverAlign
  let gap: CGFloat
  let modal: Bool
  let panel: (CGSize) -> Panel

  @Environment(\.theme) private var theme
  @State private var anchor: CGSize = .zero

  func body(content: Content) -> some View {
    let measured = content.background {
      GeometryReader { proxy in
        Color.clear
          .onAppear { anchor = proxy.size }
          .onChange(of: proxy.size) { anchor = $0 }
      }
    }

    #if canImport(AppKit)
      let margin = Self.margin(theme.vars.shadowLg)
      measured.background(
        FloatingPanelAnchor(
          isPresented: $isPresented,
          placement: Placement(side: side, align: align, gap: gap, margin: margin),
          modal: modal,
          panel: AnyView(
            panel(anchor)
              .padding(margin)
              .theme(theme.name)
          )
        )
      )
    #else
      // No second window to draw in: the overlay is as far as the platform
      // goes, raised above its own siblings at least.
      measured
        .anchored(side, align, gap: gap) {
          if isPresented { panel(anchor) }
        }
        .zIndex(isPresented ? 1 : 0)
        .background {
          if isPresented {
            SwiftUI.Button("") { isPresented = false }
              .keyboardShortcut(.cancelAction)
              .opacity(0)
              .frame(width: 0, height: 0)
          }
        }
    #endif
  }

  /// The transparent room around the sheet its shadow is drawn in; a window
  /// clips whatever falls outside it.
  static func margin(_ shadows: [TokenShadow]) -> CGFloat {
    shadows.map { $0.blur + $0.spread + max(abs($0.x), abs($0.y)) }.max() ?? 0
  }
}

#if canImport(AppKit)
  /// Where the sheet goes against the anchor, in screen coordinates — AppKit's,
  /// whose origin is the bottom-left.
  private struct Placement {
    let side: PopoverSide
    let align: PopoverAlign
    let gap: CGFloat
    /// The shadow's room on every edge of the window, outside the sheet.
    let margin: CGFloat

    /// The window's frame for a sheet of `size` (margin included) against
    /// `anchor`, flipped where `screen` has no room on the side asked for.
    func frame(size: CGSize, anchor: NSRect, screen: NSRect?) -> NSRect {
      let sheet = CGSize(width: size.width - margin * 2, height: size.height - margin * 2)
      var origin = sheetOrigin(side, sheet: sheet, anchor: anchor)

      if let screen {
        let flipped: PopoverSide?
        switch side {
        case .bottom: flipped = origin.y < screen.minY ? .top : nil
        case .top: flipped = origin.y + sheet.height > screen.maxY ? .bottom : nil
        case .leading: flipped = origin.x < screen.minX ? .trailing : nil
        case .trailing: flipped = origin.x + sheet.width > screen.maxX ? .leading : nil
        }
        if let flipped {
          origin = sheetOrigin(flipped, sheet: sheet, anchor: anchor)
        }
        // Slid back along the edge it hangs from rather than cut off.
        origin.x = min(max(origin.x, screen.minX), max(screen.minX, screen.maxX - sheet.width))
        origin.y = min(max(origin.y, screen.minY), max(screen.minY, screen.maxY - sheet.height))
      }

      return NSRect(
        x: origin.x - margin,
        y: origin.y - margin,
        width: size.width,
        height: size.height
      )
    }

    private func sheetOrigin(_ side: PopoverSide, sheet: CGSize, anchor: NSRect) -> NSPoint {
      switch side {
      case .bottom, .top:
        let x: CGFloat
        switch align {
        case .start: x = anchor.minX
        case .center: x = anchor.midX - sheet.width / 2
        case .end: x = anchor.maxX - sheet.width
        }
        let y = side == .bottom ? anchor.minY - gap - sheet.height : anchor.maxY + gap
        return NSPoint(x: x, y: y)
      case .leading, .trailing:
        // Centred on the cross axis, the way `anchored` places a side panel.
        let x = side == .leading ? anchor.minX - gap - sheet.width : anchor.maxX + gap
        return NSPoint(x: x, y: anchor.midY - sheet.height / 2)
      }
    }
  }

  private struct FloatingPanelAnchor: NSViewRepresentable {
    @Binding var isPresented: Bool
    let placement: Placement
    let modal: Bool
    let panel: AnyView

    func makeNSView(context: Context) -> AnchorView {
      AnchorView()
    }

    func updateNSView(_ view: AnchorView, context: Context) {
      view.placement = placement
      view.modal = modal
      view.dismiss = { isPresented = false }
      view.update(presented: isPresented, content: panel)
    }

    static func dismantleNSView(_ view: AnchorView, coordinator: ()) {
      view.close()
    }
  }

  /// Sits behind the anchor at the anchor's own size, so its frame in the
  /// window is what the sheet hangs from.
  private final class AnchorView: NSView {
    var placement = Placement(side: .bottom, align: .start, gap: 0, margin: 0)
    var modal = true
    var dismiss: () -> Void = {}

    private var panel: FloatingWindow?
    private var host: FirstMouseHostingView?
    private var monitor: Any?
    private var observers: [NSObjectProtocol] = []

    override func hitTest(_ point: NSPoint) -> NSView? { nil }

    override func viewDidMoveToWindow() {
      super.viewDidMoveToWindow()
      if window == nil, panel != nil {
        close()
        dismiss()
      }
    }

    func update(presented: Bool, content: AnyView) {
      if presented {
        if let host {
          host.rootView = content
          place()
        } else {
          open(content)
        }
      } else {
        close()
      }
    }

    private func open(_ content: AnyView) {
      guard let parent = window else {
        // Not in a window yet — the first layout pass has not attached it.
        DispatchQueue.main.async { [weak self] in
          guard let self, self.window != nil, self.panel == nil else { return }
          self.open(content)
        }
        return
      }

      let host = FirstMouseHostingView(rootView: content)
      let panel = FloatingWindow(
        contentRect: .zero,
        styleMask: [.borderless, .nonactivatingPanel],
        backing: .buffered,
        defer: true
      )
      panel.isOpaque = false
      panel.backgroundColor = .clear
      // The sheet draws the token shadow itself, inside the margin.
      panel.hasShadow = false
      panel.isReleasedWhenClosed = false
      panel.hidesOnDeactivate = true
      // Key only for what needs it — a text field in a popover. A row pressed
      // in a list leaves the focus where it was, in a combobox's field.
      panel.becomesKeyOnlyIfNeeded = true
      panel.contentView = host
      self.host = host
      self.panel = panel

      place()
      parent.addChildWindow(panel, ordered: .above)
      watch(parent)
    }

    private func place() {
      guard let panel, let host, let parent = window else { return }
      let anchor = parent.convertToScreen(convert(bounds, to: nil))
      let screen = (parent.screen ?? NSScreen.main)?.visibleFrame
      panel.setFrame(
        placement.frame(size: host.fittingSize, anchor: anchor, screen: screen),
        display: true
      )
    }

    /// Whether a press in window coordinates of `window` fell on the sheet
    /// itself rather than on the shadow's margin around it.
    private func onSheet(_ event: NSEvent) -> Bool {
      guard let panel, event.window === panel, let bounds = panel.contentView?.bounds else {
        return false
      }
      return bounds.insetBy(dx: placement.margin, dy: placement.margin)
        .contains(event.locationInWindow)
    }

    /// Whether a press landed on the anchor — the trigger or the field.
    private func onAnchor(_ event: NSEvent) -> Bool {
      guard let window, event.window === window else { return false }
      return bounds.contains(convert(event.locationInWindow, from: nil))
    }

    private func watch(_ parent: NSWindow) {
      monitor = NSEvent.addLocalMonitorForEvents(
        matching: [
          .leftMouseDown, .rightMouseDown, .otherMouseDown, .scrollWheel, .keyDown,
        ]
      ) { [weak self] event in
        guard let self, let panel = self.panel else { return event }

        switch event.type {
        case .keyDown:
          if event.keyCode == 53 {  // Escape
            self.dismiss()
            return nil
          }
          // A shortcut still reaches the app's menu, and the sheet's own
          // field still takes its typing; the window behind a modal sheet
          // takes neither.
          if !self.modal || event.window === panel || event.modifierFlags.contains(.command) {
            return event
          }
          return nil
        case .scrollWheel:
          if event.window === panel { return event }
          guard !self.modal else { return nil }
          // What the anchor sits in may have moved under it.
          DispatchQueue.main.async { [weak self] in self?.place() }
          return event
        default:
          if self.onSheet(event) { return event }
          if !self.modal && self.onAnchor(event) { return event }
          self.dismiss()
          return self.modal ? nil : event
        }
      }

      let center = NotificationCenter.default
      observers.append(
        center.addObserver(
          forName: NSWindow.didResignKeyNotification, object: nil, queue: .main
        ) { [weak self] note in
          guard let self, let panel = self.panel,
            let window = note.object as? NSWindow, window === parent || window === panel
          else { return }
          // The key window moving between the two — a popover's field taking
          // the focus, or handing it back — is not the focus leaving.
          DispatchQueue.main.async { [weak self] in
            guard let self, self.panel != nil else { return }
            let key = NSApp.keyWindow
            if key !== parent && key !== self.panel { self.dismiss() }
          }
        })
      observers.append(
        center.addObserver(
          forName: NSWindow.willCloseNotification, object: parent, queue: .main
        ) { [weak self] _ in
          self?.dismiss()
        })
      observers.append(
        center.addObserver(
          forName: NSWindow.didResizeNotification, object: parent, queue: .main
        ) { [weak self] _ in
          self?.place()
        })
    }

    func close() {
      if let monitor { NSEvent.removeMonitor(monitor) }
      monitor = nil
      observers.forEach(NotificationCenter.default.removeObserver)
      observers = []
      if let panel {
        let parent = panel.parent
        let wasKey = panel.isKeyWindow
        parent?.removeChildWindow(panel)
        panel.orderOut(nil)
        if wasKey { parent?.makeKey() }
      }
      panel = nil
      host = nil
    }

    deinit {
      if let monitor { NSEvent.removeMonitor(monitor) }
      observers.forEach(NotificationCenter.default.removeObserver)
    }
  }

  /// A borderless panel may still take the key focus, for a field inside it.
  private final class FloatingWindow: NSPanel {
    override var canBecomeKey: Bool { true }
    override var canBecomeMain: Bool { false }
  }

  /// The panel is rarely key, so its first click has to act rather than only
  /// focus it.
  private final class FirstMouseHostingView: NSHostingView<AnyView> {
    override func acceptsFirstMouse(for event: NSEvent?) -> Bool { true }
  }
#endif
