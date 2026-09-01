import FluentIcons
import SwiftUI

/// Which edge a ``ToastViewport``'s stack hangs from.
public enum ToastPlacement: String, CaseIterable, Sendable {
  case bottom
  case top
}

/// A transient receipt for something that happened out of view.
///
/// One line, no wrapping — anything needing a second sentence belongs in an
/// ``AlertBanner`` in the flow instead. Lifetime belongs to the host, so the
/// system stays stateless, but the contract worth keeping is: a few seconds
/// for a plain notice, longer when it carries an action, and a failure stays
/// until it is dismissed.
public struct Toast<Leading: View, Message: View, Trailing: View>: View {
  /// Which colour ramp the leading glyph is drawn from.
  public var tint: Tint
  /// Leading slot — a glyph, or a ``Spinner`` for work still in flight.
  @ViewBuilder public var leading: () -> Leading
  @ViewBuilder public var message: () -> Message
  /// Trailing affordance — undo, retry.
  @ViewBuilder public var trailing: () -> Trailing
  /// Renders the dismiss button; wire it to remove the toast.
  public var onDismiss: (() -> Void)?
  /// Accessible name of the dismiss button.
  public var dismissLabel: String

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline
  @Environment(\.accessibilityReduceMotion) private var reduceMotion
  @State private var landed = false

  public init(
    tint: Tint = .neutral,
    onDismiss: (() -> Void)? = nil,
    dismissLabel: String = "Dismiss",
    @ViewBuilder leading: @escaping () -> Leading,
    @ViewBuilder message: @escaping () -> Message,
    @ViewBuilder trailing: @escaping () -> Trailing
  ) {
    self.tint = tint
    self.onDismiss = onDismiss
    self.dismissLabel = dismissLabel
    self.leading = leading
    self.message = message
    self.trailing = trailing
  }

  public var body: some View {
    let vars = theme.vars

    HStack(spacing: vars.spacing25) {
      leading()
        .foregroundStyle(vars.ramp(tint)[600])
      message()
        .font(vars.labelMedium.font)
        .lineLimit(1)
        .truncationMode(.tail)
        .foregroundStyle(vars.colorContent)
        .padding(.vertical, vars.spacing05)
      trailing()
      if let onDismiss {
        ToastDismiss(label: dismissLabel, action: onDismiss)
      }
    }
    .frame(minHeight: vars.spacing9)
    // Sized to its content but never past the viewport, so the stack needs no
    // magic width and a long message truncates instead of pushing wide.
    .frame(maxWidth: vars.toastMaxWidth)
    .padding(.vertical, vars.spacing15)
    // Asymmetric on purpose: the text gets breathing room, the dismiss button
    // sits tight in the corner it lives in.
    .padding(.leading, vars.spacing3)
    .padding(.trailing, vars.spacing15)
    .background(vars.colorSurface)
    .clipShape(RoundedRectangle(cornerRadius: vars.radiusBig, style: .continuous))
    .border(vars.colorBorderStrong, width: hairline, cornerRadius: vars.radiusBig)
    .tokenShadow(vars.shadowMd)
    .opacity(landed ? 1 : 0)
    .scaleEffect(landed ? 1 : entryScale)
    .offset(y: landed ? 0 : entryRise)
    .animation(entry, value: landed)
    .onAppear { landed = true }
    .accessibilityElement(children: .combine)
  }

  /// The arrival: a short overshooting curve, skipped outright when the reader
  /// has asked for less motion.
  private var entry: Animation? {
    reduceMotion ? nil : .timingCurve(0.21, 0.9, 0.35, 1.05, duration: 0.26)
  }

  private var entryScale: CGFloat { 0.98 }

  private var entryRise: CGFloat { theme.vars.spacing25 }
}

public extension Toast where Leading == EmptyView, Trailing == EmptyView {
  /// A toast that is only its message.
  init(
    tint: Tint = .neutral,
    onDismiss: (() -> Void)? = nil,
    dismissLabel: String = "Dismiss",
    @ViewBuilder message: @escaping () -> Message
  ) {
    self.init(
      tint: tint,
      onDismiss: onDismiss,
      dismissLabel: dismissLabel,
      leading: { EmptyView() },
      message: message,
      trailing: { EmptyView() }
    )
  }
}

public extension Toast
where Leading == EmptyView, Message == Text, Trailing == EmptyView {
  /// A toast whose message is a plain string.
  init(
    _ text: String,
    tint: Tint = .neutral,
    onDismiss: (() -> Void)? = nil,
    dismissLabel: String = "Dismiss"
  ) {
    self.init(
      tint: tint,
      onDismiss: onDismiss,
      dismissLabel: dismissLabel,
      message: { Text(text) }
    )
  }
}

/// The dismiss glyph: quiet ink that takes the neutral row wash under the
/// pointer, rather than a control chip that would compete with the action
/// beside it.
private struct ToastDismiss: View {
  let label: String
  let action: () -> Void

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool

  var body: some View {
    let vars = theme.vars

    Pressable(action: action, cornerRadius: vars.radiusSmall) { state in
      Icon(.dismiss12Regular, size: vars.labelMedium.size)
        .foregroundStyle(
          state.hovered || state.pressed ? vars.colorContent : vars.colorContentSubtle
        )
        .frame(width: vars.controlTinySize, height: vars.controlTinySize)
        .background(
          state.hovered || state.pressed
            ? vars.resolve(
              vars.descriptor(.normal, .surface),
              tint: .neutral,
              state: .hovered
            )
            : .clear
        )
        .clipShape(RoundedRectangle(cornerRadius: vars.radiusSmall, style: .continuous))
        .animation(vars.motion, value: state.controlState)
    }
    .accessibilityLabel(label)
    .focused($focused)
    .focusRing(
      visible: focused,
      color: vars.focusRing,
      cornerRadius: vars.radiusSmall,
      width: vars.focusWidth
    )
  }
}

/// Where a window's toasts land.
///
/// The stack hangs from one edge and is transparent to the pointer between its
/// toasts, so the page underneath stays usable while one is on screen. Order
/// is the host's: SwiftUI has no `column-reverse` for arbitrary builder
/// content, so a top-placed viewport shows what it is given in the order it is
/// given.
public struct ToastViewport<Content: View>: View {
  public var placement: ToastPlacement
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme

  public init(
    placement: ToastPlacement = .bottom,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.placement = placement
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    VStack(spacing: vars.spacing2) {
      content()
    }
    .padding(vars.spacing4)
    .frame(
      maxWidth: .infinity,
      maxHeight: .infinity,
      alignment: placement == .bottom ? .bottom : .top
    )
  }
}
