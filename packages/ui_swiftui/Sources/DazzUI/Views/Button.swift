import SwiftUI

/// A button.
///
/// Colour comes entirely from the shared control state machine: a variant
/// picks the recipe, a tint picks the ramp it resolves against, and hover,
/// press and disabled follow on their own. So this view is only the box.
///
/// Named `Button` rather than prefixed, and reached as `DazzUI.Button`
/// where `SwiftUI.Button` is also in scope.
public struct Button<Label: View>: View {
  public var size: ControlSize
  public var variant: ControlVariant
  public var tint: Tint
  public var fullWidth: Bool
  /// Trailing hint — the key that also triggers this action.
  public var shortcut: String?
  public var enabled: Bool
  public var action: () -> Void
  @ViewBuilder public var label: () -> Label

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool

  public init(
    size: ControlSize = .small,
    variant: ControlVariant = .recessed,
    tint: Tint = .primary,
    fullWidth: Bool = false,
    shortcut: String? = nil,
    enabled: Bool = true,
    action: @escaping () -> Void,
    @ViewBuilder label: @escaping () -> Label
  ) {
    self.size = size
    self.variant = variant
    self.tint = tint
    self.fullWidth = fullWidth
    self.shortcut = shortcut
    self.enabled = enabled
    self.action = action
    self.label = label
  }

  public var body: some View {
    let vars = theme.vars
    let metrics = vars.metrics(size)

    Pressable(action: action, enabled: enabled, cornerRadius: metrics.radius) { state in
      let colors = vars.controlColors(
        variant: variant,
        tint: tint,
        state: state.controlState
      )

      HStack(spacing: metrics.gap) {
        label()
        if let shortcut {
          // A trailing shortcut is a hint, not part of the label — it takes
          // the display cut and steps back.
          Text(shortcut).opacity(0.7)
        }
      }
      .font(font(metrics: metrics, state: state))
      .lineLimit(1)
      .frame(height: metrics.content.lineHeight)
      .foregroundStyle(colors.content)
      .padding(.vertical, metrics.paddingBlock)
      // CSS's border participates in the intrinsic width; our stroke is an
      // overlay, so reserve its width in the content inset explicitly.
      .padding(.horizontal, metrics.paddingInline + vars.spacingPx)
      .frame(maxWidth: fullWidth ? .infinity : nil)
      .frame(height: metrics.size)
      .background(colors.surface)
      .clipShape(RoundedRectangle(cornerRadius: metrics.radius, style: .circular))
      .border(colors.border, width: vars.spacingPx, cornerRadius: metrics.radius)
      // The one action a view points at carries the hairline lift — flat like
      // every AppKit button, but separable from a tinted surface by that one
      // pixel of shadow. It goes when the button is disabled and flattens.
      .tokenShadow(liftedShadow(state: state))
      .animation(vars.motion, value: state.controlState)
    }
    .focused($focused)
    .focusRing(
      visible: focused && enabled,
      color: vars.focusRing,
      cornerRadius: metrics.radius,
      width: vars.focusWidth
    )
  }

  /// The one action a view points at also carries the extra weight, and gives
  /// it up when it flattens.
  private func font(metrics: ControlMetrics, state: PressableState) -> Font {
    guard variant == .filled, state.enabled else { return metrics.content.font }
    return .system(
      size: metrics.content.size,
      weight: theme.vars.labelStrong.weight
    )
  }

  private func liftedShadow(state: PressableState) -> [TokenShadow] {
    variant == .filled && state.enabled ? theme.vars.shadow2xs : []
  }
}

public extension Button where Label == Text {
  /// A button whose label is a plain string.
  init(
    _ title: String,
    size: ControlSize = .small,
    variant: ControlVariant = .recessed,
    tint: Tint = .primary,
    fullWidth: Bool = false,
    shortcut: String? = nil,
    enabled: Bool = true,
    action: @escaping () -> Void
  ) {
    self.init(
      size: size,
      variant: variant,
      tint: tint,
      fullWidth: fullWidth,
      shortcut: shortcut,
      enabled: enabled,
      action: action,
      label: { Text(title) }
    )
  }
}
