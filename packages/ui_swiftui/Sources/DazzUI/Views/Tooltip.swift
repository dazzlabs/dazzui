import SwiftUI

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
  @State private var hovered = false
  @State private var open = false

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
    content()
      .onHover { hovered = $0 }
      .overlay(alignment: overlayAlignment) {
        if open {
          panel
            .modifier(TooltipPlacement(side: side, gap: theme.vars.spacing1))
            .transition(.opacity.combined(with: .scale(scale: 0.97)))
        }
      }
      .zIndex(open ? 1 : 0)
      .task(id: shouldPresent) {
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
    !disabled && (hovered || presented)
  }

  private var panel: some View {
    let vars = theme.vars

    return label()
      .font(vars.labelSmall.font)
      .foregroundStyle(vars.colorContent)
      .multilineTextAlignment(.leading)
      .padding(.vertical, vars.spacing1 + vars.spacing05)
      .padding(.horizontal, vars.spacing2)
      .frame(maxWidth: vars.toastMaxWidth)
      .fixedSize(horizontal: false, vertical: true)
      .background(vars.colorSurfaceRaised)
      .clipShape(
        RoundedRectangle(cornerRadius: vars.radiusSmall, style: .circular)
      )
      .border(vars.colorBorder, width: hairline, cornerRadius: vars.radiusSmall)
      .tokenShadow(vars.shadowSm)
      .allowsHitTesting(false)
      .accessibilityHidden(true)
  }

  private var overlayAlignment: Alignment {
    switch (side, align) {
    case (.top, .start): return .topLeading
    case (.top, .center): return .top
    case (.top, .end): return .topTrailing
    case (.bottom, .start): return .bottomLeading
    case (.bottom, .center): return .bottom
    case (.bottom, .end): return .bottomTrailing
    case (.left, .start): return .topLeading
    case (.left, .center): return .leading
    case (.left, .end): return .bottomLeading
    case (.right, .start): return .topTrailing
    case (.right, .center): return .trailing
    case (.right, .end): return .bottomTrailing
    }
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

private struct TooltipPlacement: ViewModifier {
  let side: TooltipSide
  let gap: CGFloat

  func body(content: Content) -> some View {
    switch side {
    case .top:
      content.alignmentGuide(.top) { dimensions in
        dimensions[.bottom] + gap
      }
    case .bottom:
      content.alignmentGuide(.bottom) { dimensions in
        dimensions[.top] - gap
      }
    case .left:
      content.alignmentGuide(.leading) { dimensions in
        dimensions[.trailing] + gap
      }
    case .right:
      content.alignmentGuide(.trailing) { dimensions in
        dimensions[.leading] - gap
      }
    }
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
