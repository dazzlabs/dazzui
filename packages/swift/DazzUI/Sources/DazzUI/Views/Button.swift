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
      // The box, drawn twice: once filled behind the label and once stroked
      // over it, so each colour has a layer of its own to crossfade on.
      let shape = RoundedRectangle(cornerRadius: metrics.radius, style: .circular)

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
      // The ink crossfades with the surface, as `button.css` transitions
      // `color` beside `background-color`. The rest of what a state touches
      // is geometry — a filled label gives up its weight when the control
      // flattens, and the box narrows with it. SwiftUI animates a subtree's
      // layout along with its colours, so that transition is a resize as well
      // as a crossfade; a box that landed while its fill was still crossing
      // over would read as a clipping fault.
      .animation(vars.motion, value: colors.content)
      .padding(.vertical, metrics.paddingBlock)
      // CSS's border participates in the intrinsic width; our stroke is an
      // overlay, so reserve its width in the content inset explicitly.
      .padding(.leading, leadingPadding)
      .padding(.trailing, trailingPadding)
      .frame(maxWidth: fullWidth ? .infinity : nil)
      .frame(height: metrics.size)
      // The surface and the stroke are shapes rather than a clipped colour
      // fill: a clip has nothing to do with a button — CSS draws the label
      // over the box and lets it overflow — and pairing one with an animated
      // background is what macOS 15 draws as the fill being wiped away from
      // one edge. Each shape is keyed on its own colour, so a state change
      // crossfades the two layers instead of replacing the box.
      .background {
        shape.fill(colors.surface)
          .animation(vars.motion, value: colors.surface)
      }
      .overlay {
        shape.strokeBorder(colors.border, lineWidth: vars.spacingPx)
          .animation(vars.motion, value: colors.border)
      }
      // The one action a view points at carries the hairline lift — flat like
      // every AppKit button, but separable from a tinted surface by that one
      // pixel of shadow. It goes when the button is disabled and flattens,
      // and it is not a transitioned property in CSS either.
      .tokenShadow(liftedShadow(state: state))
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

  /// The hairline lift the one action a view points at carries, and gives up
  /// when it flattens.
  ///
  /// A flat button keeps the layer and draws nothing in it rather than
  /// dropping it: see ``SwiftUI/View/tokenShadow(_:)``. Handing this an empty
  /// list would make the whole button a new view the moment it was disabled,
  /// and the fill would be swapped instead of crossfading.
  private func liftedShadow(state: PressableState) -> [TokenShadow] {
    variant == .filled && state.enabled ? theme.vars.shadow2xs : [.none]
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
