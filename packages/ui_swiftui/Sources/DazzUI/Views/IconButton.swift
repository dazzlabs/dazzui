import FluentIcons
import SwiftUI

/// A square button.
///
/// Colour comes from the shared control state machine, but the default
/// posture — plain variant, neutral tint — carries its own quiet chrome: a
/// subtle-ink glyph that washes grey and darkens to full ink under the
/// pointer. A toolbar row of these has to read as chrome until touched, and
/// the recipes' neutral ramp is too flat a grey for that.
public struct IconButton<Content: View>: View {
  /// Accessible name, also surfaced as the tooltip.
  public var label: String
  public var size: ControlSize
  public var variant: ControlVariant
  public var tint: Tint
  public var enabled: Bool
  public var action: () -> Void
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool

  public init(
    label: String,
    size: ControlSize = .tiny,
    variant: ControlVariant = .plain,
    // Neutral by default: a toolbar of these should read as chrome, not as a
    // row of accented actions. Pass a tint where one button genuinely leads.
    tint: Tint = .neutral,
    enabled: Bool = true,
    action: @escaping () -> Void,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.label = label
    self.size = size
    self.variant = variant
    self.tint = tint
    self.enabled = enabled
    self.action = action
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars
    let metrics = vars.metrics(size)

    Tooltip(label, disabled: !enabled, presented: focused) {
      Pressable(
        action: action,
        enabled: enabled,
        cornerRadius: metrics.radius
      ) { state in
        let colors = vars.controlColors(
          variant: variant,
          tint: tint,
          state: state.controlState
        )

        content()
          .foregroundStyle(foreground(colors: colors, state: state))
          .frame(width: metrics.size, height: metrics.size)
          .background(background(colors: colors, state: state))
          .clipShape(
            RoundedRectangle(cornerRadius: metrics.radius, style: .circular)
          )
          .border(
            colors.border,
            width: vars.spacingPx,
            cornerRadius: metrics.radius
          )
          .animation(vars.motion, value: state.controlState)
      }
      .accessibilityLabel(label)
      .focused($focused)
      .focusRing(
        visible: focused && enabled,
        color: vars.focusRing,
        cornerRadius: metrics.radius,
        width: vars.focusWidth
      )
    }
  }

  /// Is this the quiet chrome posture the toolbar wants?
  private var chrome: Bool {
    variant == .plain && tint == .neutral
  }

  private func foreground(colors: ControlColors, state: PressableState) -> Color {
    guard chrome, state.enabled else { return colors.content }
    return state.hovered || state.pressed
      ? theme.vars.colorContent
      : theme.vars.colorContentSubtle
  }

  private func background(colors: ControlColors, state: PressableState) -> Color {
    guard chrome, state.enabled else { return colors.surface }
    return state.hovered || state.pressed ? theme.vars.colorSurfaceSubtle : .clear
  }
}

public extension IconButton where Content == Icon {
  /// An icon button drawn from an SF Symbol.
  init(
    label: String,
    icon: FluentIcon,
    size: ControlSize = .tiny,
    variant: ControlVariant = .plain,
    tint: Tint = .neutral,
    enabled: Bool = true,
    action: @escaping () -> Void
  ) {
    self.init(
      label: label,
      size: size,
      variant: variant,
      tint: tint,
      enabled: enabled,
      action: action,
      content: { Icon(icon, size: 16) }
    )
  }
}
