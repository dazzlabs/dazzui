import SwiftUI

/// A button.
///
/// Colour comes entirely from the shared control state machine: a variant
/// picks the recipe, a tint picks the ramp it resolves against, and hover,
/// press and disabled follow on their own. So this view is only the box.
///
/// A glyph beside the label is passed as `leading` or `trailing` rather than
/// drawn inside it, because the button sizes and places it: a step under the
/// profile's icon, which is sized to fill a square button alone, and the end
/// it sits at pads less, since the glyph's frame carries whitespace of its
/// own. A glyph-only button is an ``IconButton``.
///
/// Named `Button` rather than prefixed, and reached as `DazzUI.Button`
/// where `SwiftUI.Button` is also in scope.
public struct Button<Label: View>: View {
  public var size: ControlSize
  public var variant: ControlVariant
  public var tint: Tint
  public var fullWidth: Bool
  /// A glyph before the label — usually an ``Icon``, which the button sizes.
  public var leading: AnyView?
  /// A glyph after the label — where the action goes next.
  public var trailing: AnyView?
  /// Trailing hint — the key that also triggers this action.
  public var shortcut: String?
  /// The same hint as a view, usually a small ``KeyCap`` — React's
  /// `shortcut` node. Drawn in place of the string when both are given.
  public var shortcutView: AnyView?
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
    leading: (any View)? = nil,
    trailing: (any View)? = nil,
    shortcut: String? = nil,
    enabled: Bool = true,
    action: @escaping () -> Void,
    @ViewBuilder label: @escaping () -> Label
  ) {
    self.size = size
    self.variant = variant
    self.tint = tint
    self.fullWidth = fullWidth
    self.leading = leading.map { AnyView($0) }
    self.trailing = trailing.map { AnyView($0) }
    self.shortcut = shortcut
    self.enabled = enabled
    self.action = action
    self.label = label
  }

  /// A button whose shortcut hint is a view — usually
  /// `KeyCap(size: .small) { Text("⌘↩") }`.
  public init<Shortcut: View>(
    size: ControlSize = .small,
    variant: ControlVariant = .recessed,
    tint: Tint = .primary,
    fullWidth: Bool = false,
    leading: (any View)? = nil,
    trailing: (any View)? = nil,
    enabled: Bool = true,
    action: @escaping () -> Void,
    @ViewBuilder label: @escaping () -> Label,
    @ViewBuilder shortcut: () -> Shortcut
  ) {
    self.init(
      size: size,
      variant: variant,
      tint: tint,
      fullWidth: fullWidth,
      leading: leading,
      trailing: trailing,
      enabled: enabled,
      action: action,
      label: label
    )
    self.shortcutView = AnyView(shortcut())
  }

  public var body: some View {
    let vars = theme.vars
    let metrics = vars.metrics(size)
    // A glyph beside a label is drawn `control.label-glyph.shrink` under the
    // profile's icon; the end it sits at pads `control.label-glyph.inset`
    // less. A stroke's width is reserved on both ends either way.
    let glyphSize = metrics.iconSize - vars.controlLabelGlyphShrink
    let leadingPadding =
      metrics.paddingInline + vars.spacingPx
      - (leading == nil ? 0 : vars.controlLabelGlyphInset)
    let trailingPadding =
      metrics.paddingInline + vars.spacingPx
      - (trailing == nil ? 0 : vars.controlLabelGlyphInset)

    Pressable(action: action, enabled: enabled, cornerRadius: metrics.radius) { state in
      let colors = vars.controlColors(
        variant: variant,
        tint: tint,
        state: state.controlState
      )

      HStack(spacing: metrics.gap) {
        Group {
          leading
          label()
          trailing
        }
        .environment(\.dazzIconSize, glyphSize)
        // A trailing shortcut is a hint, not part of the label — it takes
        // the display cut and steps back.
        if let shortcutView {
          shortcutView.opacity(0.7)
        } else if let shortcut {
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
      .padding(.leading, leadingPadding)
      .padding(.trailing, trailingPadding)
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
    leading: (any View)? = nil,
    trailing: (any View)? = nil,
    shortcut: String? = nil,
    enabled: Bool = true,
    action: @escaping () -> Void
  ) {
    self.init(
      size: size,
      variant: variant,
      tint: tint,
      fullWidth: fullWidth,
      leading: leading,
      trailing: trailing,
      shortcut: shortcut,
      enabled: enabled,
      action: action,
      label: { Text(title) }
    )
  }
}

public extension Button where Label == Text {
  /// A button whose label is a plain string and whose shortcut hint is a view.
  init<Shortcut: View>(
    _ title: String,
    size: ControlSize = .small,
    variant: ControlVariant = .recessed,
    tint: Tint = .primary,
    fullWidth: Bool = false,
    leading: (any View)? = nil,
    trailing: (any View)? = nil,
    enabled: Bool = true,
    action: @escaping () -> Void,
    @ViewBuilder shortcut: () -> Shortcut
  ) {
    self.init(
      size: size,
      variant: variant,
      tint: tint,
      fullWidth: fullWidth,
      leading: leading,
      trailing: trailing,
      enabled: enabled,
      action: action,
      label: { Text(title) },
      shortcut: shortcut
    )
  }
}
