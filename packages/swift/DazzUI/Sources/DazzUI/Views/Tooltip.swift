import SwiftUI

#if os(macOS)
import AppKit
#endif

/// Which edge of its control a ``Tooltip`` hangs from.
public enum TooltipSide: String, CaseIterable, Sendable {
  case top
  case bottom
  case left
  case right
}

/// How a ``Tooltip`` aligns along the edge it hangs from.
public enum TooltipAlignment: String, CaseIterable, Sendable {
  case start
  case center
  case end
}

/// Shares the opening delay across a group of tooltips.
///
/// Once one label has appeared, moving through the controls in this subtree
/// opens the next one immediately. This is the toolbar behaviour React gets
/// from its tooltip provider.
public struct TooltipProvider<Content: View>: View {
  @ViewBuilder public var content: () -> Content

  @StateObject private var delayContext = TooltipDelayContext()

  public init(@ViewBuilder content: @escaping () -> Content) {
    self.content = content
  }

  public var body: some View {
    content()
      .environment(\.tooltipDelayContext, delayContext)
  }
}

/// A tooltip's register: what the tooltips inside it call to say they exist.
///
/// A control that owns a hint already — ``IconButton`` — does not want a
/// second hint opened around it, so an outer tooltip with a tooltip inside
/// stands down, the way React's and ArkUI's do.
private typealias TooltipScope = (_ id: UUID, _ mounted: Bool) -> Void

private struct TooltipScopeKey: EnvironmentKey {
  static let defaultValue: TooltipScope? = nil
}

private extension EnvironmentValues {
  var tooltipScope: TooltipScope? {
    get { self[TooltipScopeKey.self] }
    set { self[TooltipScopeKey.self] = newValue }
  }
}

/// The short label a control surfaces on hover or keyboard focus.
///
/// It is only a reminder of what the control means. The control must still
/// carry its own accessible name; the tooltip is hidden from accessibility so
/// that name is not announced twice.
public struct Tooltip<Label: View, Content: View>: View {
  public var side: TooltipSide
  public var align: TooltipAlignment
  /// Seconds before the label opens on first hover.
  public var delay: TimeInterval
  /// Seconds the label remains visible after hover or focus leaves.
  public var closeDelay: TimeInterval
  public var disabled: Bool
  /// Lets a control surface the label from keyboard focus as well as hover.
  public var presented: Bool
  @ViewBuilder public var label: () -> Label
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline
  @Environment(\.tooltipDelayContext) private var delayContext
  @Environment(\.tooltipScope) private var parentScope
  @State private var hovered = false
  @State private var open = false
  /// The tooltips mounted inside this one, which take precedence over it.
  @State private var nested: Set<UUID> = []
  @State private var id = UUID()

  public init(
    side: TooltipSide = .top,
    align: TooltipAlignment = .center,
    delay: TimeInterval = 0.6,
    closeDelay: TimeInterval = 0,
    disabled: Bool = false,
    presented: Bool = false,
    @ViewBuilder label: @escaping () -> Label,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.side = side
    self.align = align
    self.delay = delay
    self.closeDelay = closeDelay
    self.disabled = disabled
    self.presented = presented
    self.label = label
    self.content = content
  }

  public var body: some View {
    scheduled(
      presentation(
        content()
          .onHover { hovered = $0 }
          // Only the control is scoped: a label is a label, and a tooltip in
          // one is about something else.
          .environment(\.tooltipScope, { id, mounted in
            if mounted {
              nested.insert(id)
            } else {
              nested.remove(id)
            }
          })
      )
    )
    .onAppear { parentScope?(id, true) }
    .onDisappear { parentScope?(id, false) }
  }

  /// Where the label is drawn.
  ///
  /// On macOS it is not drawn in this layout at all: a tooltip is a remark
  /// about a control rather than part of what the control lays out, and in
  /// the hierarchy it is clipped by whatever ancestor clips (a sidebar's
  /// edge, a scrolling pane) and painted under whatever the next sibling
  /// draws. The label is presented in a panel of its own instead, beside the
  /// control and above the window — the shape of React's portal: always
  /// whole, always on top.
  ///
  /// Elsewhere it still hangs off its anchor: placed by a layout of its own
  /// rather than by the overlay's alignment, because the overlay offers the
  /// label the anchor's size and the label is to take its own.
  @ViewBuilder
  private func presentation<V: View>(_ anchor: V) -> some View {
    #if os(macOS)
    anchor.background {
      TooltipPortal(
        open: open,
        side: side,
        align: align,
        gap: theme.vars.spacing1,
        maxWidth: theme.vars.toastMaxWidth,
        shadowInset: theme.vars.spacing3,
        theme: theme,
        duration: theme.vars.motionDuration,
        easing: theme.vars.motionEasing
      ) {
        panel
      }
    }
    #else
    anchor
      .overlay {
        TooltipLayout(
          side: side,
          align: align,
          gap: theme.vars.spacing1,
          maxWidth: theme.vars.toastMaxWidth
        ) {
          if open {
            panel
              .transition(.opacity.combined(with: .scale(scale: 0.97)))
          }
        }
      }
      .zIndex(open ? 1 : 0)
    #endif
  }

  private func scheduled<V: View>(_ view: V) -> some View {
    view.task(id: shouldPresent) {
      if shouldPresent {
        let openingDelay = delayContext?.isPrimed == true ? 0 : max(0, delay)
        guard await wait(openingDelay) else { return }
        guard shouldPresent else { return }
        delayContext?.markOpened()
        withAnimation(theme.vars.motion) { open = true }
      } else {
        guard await wait(max(0, closeDelay)) else { return }
        guard !shouldPresent else { return }
        withAnimation(theme.vars.motion) { open = false }
      }
    }
  }

  private var shouldPresent: Bool {
    !disabled && nested.isEmpty && (hovered || presented)
  }

  private var panel: some View {
    let vars = theme.vars

    return label()
      .font(vars.labelSmall.font)
      .foregroundStyle(vars.colorContent)
      .multilineTextAlignment(.leading)
      .padding(.vertical, vars.spacing1 + vars.spacing05)
      .padding(.horizontal, vars.spacing2)
      .background(vars.colorSurfaceRaised)
      .clipShape(
        RoundedRectangle(cornerRadius: vars.radiusSmall, style: .circular)
      )
      .border(vars.colorBorder, width: hairline, cornerRadius: vars.radiusSmall)
      .tokenShadow(vars.shadowSm)
      .allowsHitTesting(false)
      .accessibilityHidden(true)
  }

  private func wait(_ seconds: TimeInterval) async -> Bool {
    guard seconds > 0 else { return !Task.isCancelled }
    do {
      try await Task.sleep(nanoseconds: UInt64(seconds * 1_000_000_000))
      return !Task.isCancelled
    } catch {
      return false
    }
  }
}

public extension Tooltip where Label == Text {
  /// A tooltip whose label is a plain string.
  init(
    _ label: String,
    side: TooltipSide = .top,
    align: TooltipAlignment = .center,
    delay: TimeInterval = 0.6,
    closeDelay: TimeInterval = 0,
    disabled: Bool = false,
    presented: Bool = false,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.init(
      side: side,
      align: align,
      delay: delay,
      closeDelay: closeDelay,
      disabled: disabled,
      presented: presented,
      label: { Text(label) },
      content: content
    )
  }
}

#if os(macOS)
/// Presents the label in a window of its own, beside the control.
///
/// A tooltip is a remark about a control rather than part of what the control
/// lays out, and in the hierarchy it is clipped by whatever ancestor clips (a
/// sidebar's edge, a scrolling pane) and painted under whatever the next
/// sibling draws. A panel has neither problem: it belongs to no view, so no
/// view can clip it, and it is above the window's content by construction —
/// which is how AppKit has always drawn its own tooltips, and what the issue
/// asks for: always whole, always on top.
///
/// The view is the anchor: it draws nothing and takes the control's bounds,
/// which is all the portal needs to know where the control is.
private struct TooltipPortal<Label: View>: NSViewRepresentable {
  var open: Bool
  var side: TooltipSide
  var align: TooltipAlignment
  var gap: CGFloat
  var maxWidth: CGFloat
  /// Room for the label's shadow, which is drawn outside the label's bounds.
  var shadowInset: CGFloat
  var theme: Theme
  var duration: Double
  var easing: BezierCurve
  @ViewBuilder var label: () -> Label

  func makeNSView(context: Context) -> NSView {
    let anchor = TooltipAnchor()
    // The anchor can be asked to present before it is in a window — the
    // representable is updated as the view is built — so it presents again
    // once it lands in one.
    anchor.onWindowChange = {
      [weak coordinator = context.coordinator, weak anchor] in
      guard let anchor else { return }
      coordinator?.reapply(anchor: anchor)
    }
    return anchor
  }

  func updateNSView(_ anchor: NSView, context: Context) {
    context.coordinator.remember(
      open: open,
      side: side,
      align: align,
      gap: gap,
      maxWidth: maxWidth,
      shadowInset: shadowInset,
      duration: duration,
      easing: easing,
      content: AnyView(label().environment(\.theme, theme))
    )
    context.coordinator.reapply(anchor: anchor)
  }

  static func dismantleNSView(_ anchor: NSView, coordinator: Coordinator) {
    coordinator.dismiss(animated: false)
  }

  func makeCoordinator() -> Coordinator { Coordinator() }

  @MainActor
  final class Coordinator {
    private var panel: TooltipPanel?
    private var host: TooltipHost?
    private var visible = false
    /// The motion the label travels over, kept so that a view torn down
    /// without a last update still closes the way the theme asks.
    private var duration: Double = 0.12
    private var easing = BezierCurve(0.4, 0, 1, 1)

    private var open = false
    private var side: TooltipSide = .top
    private var align: TooltipAlignment = .center
    private var gap: CGFloat = 0
    private var maxWidth: CGFloat = 0
    private var shadowInset: CGFloat = 0
    private var content = AnyView(EmptyView())

    func remember(
      open: Bool,
      side: TooltipSide,
      align: TooltipAlignment,
      gap: CGFloat,
      maxWidth: CGFloat,
      shadowInset: CGFloat,
      duration: Double,
      easing: BezierCurve,
      content: AnyView
    ) {
      self.open = open
      self.side = side
      self.align = align
      self.gap = gap
      self.maxWidth = maxWidth
      self.shadowInset = shadowInset
      self.duration = duration
      self.easing = easing
      self.content = content
    }

    func reapply(anchor: NSView) {
      guard open, let window = anchor.window, window.contentView != nil else {
        dismiss(animated: true)
        return
      }

      let panel = self.panel ?? makePanel()
      if panel.parent !== window {
        panel.parent?.removeChildWindow(panel)
        window.addChildWindow(panel, ordered: .above)
      }

      let size = measure(content, in: host, maxWidth: maxWidth)
      // The window's layout rect, not its frame: a label over the titlebar
      // reads as chrome drawn wrong, so the titlebar is outside where a
      // label may go — a control at the top of the window gets its label
      // below it.
      let placed = place(
        size,
        side: side,
        align: align,
        gap: gap,
        anchor: anchor,
        window: window,
        room: window.convertToScreen(window.contentLayoutRect)
      )
      host?.rootView = AnyView(content.padding(shadowInset))
      panel.setFrame(
        placed.insetBy(dx: -shadowInset, dy: -shadowInset), display: true)

      if !visible {
        visible = true
        panel.alphaValue = 0
        panel.orderFront(nil)
        animate(panel, to: 1)
      } else {
        panel.alphaValue = 1
      }
    }

    func dismiss(animated: Bool) {
      guard let panel else { return }
      guard visible else {
        panel.orderOut(nil)
        return
      }
      visible = false
      guard animated else {
        panel.orderOut(nil)
        panel.alphaValue = 1
        return
      }
      animate(panel, to: 0) { [weak self] in
        guard let self, !self.visible else { return }
        self.panel?.orderOut(nil)
        self.panel?.alphaValue = 1
      }
    }

    /// `width: max-content` under `max-width`: the label's own width, capped,
    /// and wrapped at the cap — never the anchor's width, which in an icon
    /// button's 24 points would set it one letter to a line.
    private func measure(
      _ content: AnyView, in host: TooltipHost?, maxWidth: CGFloat
    ) -> CGSize {
      guard let host else { return .zero }
      host.rootView = content
      let ideal = host.fittingSize
      guard ideal.width > maxWidth else {
        return CGSize(width: ceil(ideal.width), height: ceil(ideal.height))
      }
      host.rootView = AnyView(content.frame(width: maxWidth))
      let wrapped = host.fittingSize
      return CGSize(width: ceil(maxWidth), height: ceil(wrapped.height))
    }

    /// The label's frame in screen coordinates, flipped or nudged when the
    /// side it was asked for would leave the window.
    private func place(
      _ size: CGSize,
      side: TooltipSide,
      align: TooltipAlignment,
      gap: CGFloat,
      anchor: NSView,
      window: NSWindow,
      room: CGRect
    ) -> CGRect {
      let anchorOnScreen = window.convertToScreen(
        anchor.convert(anchor.bounds, to: nil))
      var room = room.insetBy(dx: 4, dy: 4)
      if let visibleFrame = window.screen?.visibleFrame {
        room = room.intersection(visibleFrame)
      }
      if room.isNull { room = window.frame.insetBy(dx: 4, dy: 4) }

      func origin(_ side: TooltipSide) -> CGPoint {
        let alongX = aligned(
          from: anchorOnScreen.minX, to: anchorOnScreen.maxX,
          extent: size.width, align: align)
        let alongY = aligned(
          from: anchorOnScreen.minY, to: anchorOnScreen.maxY,
          extent: size.height, align: align)
        switch side {
        case .top:
          return CGPoint(x: alongX, y: anchorOnScreen.maxY + gap)
        case .bottom:
          return CGPoint(x: alongX, y: anchorOnScreen.minY - gap - size.height)
        case .left:
          return CGPoint(x: anchorOnScreen.minX - gap - size.width, y: alongY)
        case .right:
          return CGPoint(x: anchorOnScreen.maxX + gap, y: alongY)
        }
      }

      func rect(_ side: TooltipSide) -> CGRect {
        CGRect(origin: origin(side), size: size)
      }

      var chosen = side
      if !room.contains(rect(side)), room.contains(rect(opposite(side))) {
        chosen = opposite(side)
      }
      var placed = rect(chosen)
      placed.origin.x = min(
        max(placed.minX, room.minX), max(room.minX, room.maxX - placed.width))
      placed.origin.y = min(
        max(placed.minY, room.minY), max(room.minY, room.maxY - placed.height))
      return placed
    }

    private func aligned(
      from start: CGFloat, to end: CGFloat, extent: CGFloat,
      align: TooltipAlignment
    ) -> CGFloat {
      switch align {
      case .start: return start
      case .center: return (start + end - extent) / 2
      case .end: return end - extent
      }
    }

    private func opposite(_ side: TooltipSide) -> TooltipSide {
      switch side {
      case .top: return .bottom
      case .bottom: return .top
      case .left: return .right
      case .right: return .left
      }
    }

    private func animate(
      _ panel: NSPanel, to alpha: CGFloat, then finish: (() -> Void)? = nil
    ) {
      guard duration > 0, panel.isVisible else {
        panel.alphaValue = alpha
        finish?()
        return
      }
      NSAnimationContext.runAnimationGroup { context in
        context.duration = duration
        context.timingFunction = CAMediaTimingFunction(
          controlPoints: Float(easing.x1), Float(easing.y1),
          Float(easing.x2), Float(easing.y2))
        panel.animator().alphaValue = alpha
      } completionHandler: {
        finish?()
      }
    }

    private func makePanel() -> TooltipPanel {
      let panel = TooltipPanel(
        contentRect: .zero,
        styleMask: [.borderless, .nonactivatingPanel],
        backing: .buffered,
        defer: false
      )
      panel.isOpaque = false
      panel.backgroundColor = .clear
      panel.hasShadow = false
      panel.ignoresMouseEvents = true
      panel.hidesOnDeactivate = true
      panel.isReleasedWhenClosed = false
      panel.animationBehavior = .none
      panel.collectionBehavior = [.ignoresCycle, .transient]
      let host = TooltipHost()
      host.autoresizingMask = [.width, .height]
      panel.contentView = host
      self.panel = panel
      self.host = host
      return panel
    }
  }
}

/// The panel a label is drawn in: it belongs to no view, so nothing can clip
/// it, and it never takes the keyboard from the window it hangs over.
private final class TooltipPanel: NSPanel {
  override var canBecomeKey: Bool { false }
  override var canBecomeMain: Bool { false }
}

/// The control's corner of the hierarchy. It draws nothing; it exists to say
/// where the control is and when it lands in a window, so a label asked for
/// before the view was mounted can still be placed.
private final class TooltipAnchor: NSView {
  var onWindowChange: (() -> Void)?

  override func viewDidMoveToWindow() {
    super.viewDidMoveToWindow()
    onWindowChange?()
  }
}

/// The label's own hosting view. It is read, never pressed: nothing under the
/// pointer may be taken from the control the label is about.
private final class TooltipHost: NSHostingView<AnyView> {
  init() {
    super.init(rootView: AnyView(EmptyView()))
  }

  @available(*, unavailable)
  required init?(coder: NSCoder) {
    fatalError("TooltipHost is created in code")
  }

  @available(*, unavailable)
  required init(rootView: AnyView) {
    fatalError("TooltipHost is created in code")
  }

  override func hitTest(_ point: NSPoint) -> NSView? { nil }
}
#endif

/// Sizes the label and hangs it off its anchor.
///
/// The label is `width: max-content` under `max-width`: its ideal width,
/// capped, and wrapped at the cap — never the width the anchor happened to
/// be, which in an icon button's 24 points would set it one letter to a line.
/// The layout itself is the anchor's size; the label is placed outside it.
private struct TooltipLayout: Layout {
  let side: TooltipSide
  let align: TooltipAlignment
  let gap: CGFloat
  let maxWidth: CGFloat

  func sizeThatFits(proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) -> CGSize {
    proposal.replacingUnspecifiedDimensions(by: .zero)
  }

  func placeSubviews(
    in bounds: CGRect,
    proposal: ProposedViewSize,
    subviews: Subviews,
    cache: inout ()
  ) {
    for subview in subviews {
      let size = measure(subview)
      let along: (CGFloat, CGFloat, CGFloat) -> CGFloat = { start, end, extent in
        switch align {
        case .start: return start
        case .center: return (start + end - extent) / 2
        case .end: return end - extent
        }
      }
      let origin: CGPoint
      switch side {
      case .top:
        origin = CGPoint(x: along(bounds.minX, bounds.maxX, size.width), y: bounds.minY - gap - size.height)
      case .bottom:
        origin = CGPoint(x: along(bounds.minX, bounds.maxX, size.width), y: bounds.maxY + gap)
      case .left:
        origin = CGPoint(x: bounds.minX - gap - size.width, y: along(bounds.minY, bounds.maxY, size.height))
      case .right:
        origin = CGPoint(x: bounds.maxX + gap, y: along(bounds.minY, bounds.maxY, size.height))
      }
      subview.place(
        at: origin,
        anchor: .topLeading,
        proposal: ProposedViewSize(width: size.width, height: size.height)
      )
    }
  }

  private func measure(_ subview: LayoutSubview) -> CGSize {
    let ideal = subview.sizeThatFits(.unspecified)
    guard ideal.width > maxWidth else { return ideal }
    return subview.sizeThatFits(ProposedViewSize(width: maxWidth, height: nil))
  }
}

private final class TooltipDelayContext: ObservableObject {
  private var lastOpenedAt: Date?

  // A brief grace period is what makes moving along one toolbar immediate
  // without making an unrelated tooltip later in the session skip its delay.
  private let gracePeriod: TimeInterval = 0.8

  var isPrimed: Bool {
    guard let lastOpenedAt else { return false }
    return Date().timeIntervalSince(lastOpenedAt) <= gracePeriod
  }

  func markOpened() {
    lastOpenedAt = Date()
  }
}

private struct TooltipDelayContextKey: EnvironmentKey {
  static let defaultValue: TooltipDelayContext? = nil
}

private extension EnvironmentValues {
  var tooltipDelayContext: TooltipDelayContext? {
    get { self[TooltipDelayContextKey.self] }
    set { self[TooltipDelayContextKey.self] = newValue }
  }
}
