import FluentIcons
import SwiftUI

/// A button that stays down.
///
/// The distinction it draws against its neighbours is worth stating, because
/// three views here are one press away from each other. A ``Switch`` commits
/// immediately and reads as a setting. A ``SegmentedControl`` is a choice
/// *between* its segments and always has one. This is neither: a single
/// property that is on or off — bold, wrap, show whitespace — in a row where
/// none of them may be on.
public struct Toggle<Label: View>: View {
  public var pressed: Bool
  public var size: ControlSize
  public var variant: ControlVariant
  public var tint: Tint
  public var enabled: Bool
  /// Set by ``ToggleGroup`` on every member of a run: a grouped toggle really
  /// is drawn differently — square corners inside the run, and no edge of its
  /// own where it meets its neighbour.
  public var grouped: Bool
  /// A toggle whose whole label is one glyph is square at the control height,
  /// the way an ``IconButton`` is — the padding goes, because a glyph has no
  /// side bearings to pay for. That is what lets a latching glyph sit in a
  /// toolbar beside the buttons that do not latch without being the one cell
  /// of a different width. The glyph initialiser sets it.
  public var glyphOnly: Bool
  /// What a screen reader says, for a toggle whose label is a glyph.
  public var accessibilityName: String?
  public var action: (() -> Void)?
  @ViewBuilder public var label: () -> Label

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool

  public init(
    pressed: Bool = false,
    size: ControlSize = .tiny,
    variant: ControlVariant = .plain,
    // Neutral for the reason `IconButton` is: a toolbar of these is chrome,
    // and a row of accented glyphs reads as a row of actions.
    tint: Tint = .neutral,
    enabled: Bool = true,
    grouped: Bool = false,
    glyphOnly: Bool = false,
    accessibilityName: String? = nil,
    action: (() -> Void)? = nil,
    @ViewBuilder label: @escaping () -> Label
  ) {
    self.pressed = pressed
    self.size = size
    self.variant = variant
    self.tint = tint
    self.enabled = enabled
    self.grouped = grouped
    self.glyphOnly = glyphOnly
    self.accessibilityName = accessibilityName
    self.action = action
    self.label = label
  }

  public var body: some View {
    let vars = theme.vars
    let metrics = vars.metrics(size)
    let corner = grouped ? 0 : metrics.radius

    Pressable(action: action ?? {}, enabled: enabled, cornerRadius: corner) { state in
      // Down: the system's own selection rule, applied. The tinted wash for the
      // colour, and the label a weight heavier — the weight is the half that
      // survives on a monochrome toolbar and for a reader who cannot tell the
      // two grades apart, which is why it is not decoration.
      //
      // The recipe stays tinted when the toggle is disabled and only the weight
      // drops. Falling back to the variant would leave a disabled `plain`
      // toggle with no surface at all, where the whole system flattens a dead
      // control to the groove.
      let drawn = pressed ? ControlVariant.tinted : variant
      let colors = vars.controlColors(
        variant: drawn,
        tint: tint,
        state: state.controlState
      )

      label()
        .environment(\.dazzIconSize, metrics.iconSize)
        .font(
          .system(
            size: metrics.content.size,
            weight: pressed && state.enabled ? vars.labelStrong.weight : metrics.content.weight
          )
        )
        .lineLimit(1)
        .frame(height: metrics.content.lineHeight)
        .foregroundStyle(colors.content)
        .padding(.vertical, metrics.paddingBlock)
        // CSS's border participates in the intrinsic width; our stroke is an
        // overlay, so reserve its width in the content inset explicitly.
        .padding(.horizontal, glyphOnly ? 0 : metrics.paddingInline + vars.spacingPx)
        .frame(width: glyphOnly ? metrics.size : nil, height: metrics.size)
        .background(colors.surface)
        .clipShape(RoundedRectangle(cornerRadius: corner, style: .circular))
        // A member of a run draws no edge of its own: the group paints one ring
        // around the whole thing and one rule between each pair, which is what
        // keeps two hairlines from stacking into a two-pixel seam.
        .border(
          grouped ? .clear : colors.border,
          width: vars.spacingPx,
          cornerRadius: corner
        )
        .animation(vars.motion, value: state.controlState)
    }
    .focused($focused)
    .focusRing(
      visible: focused && enabled,
      color: vars.focusRing,
      cornerRadius: corner,
      width: vars.focusWidth
    )
    .accessibilityAddTraits(pressed ? [.isButton, .isSelected] : .isButton)
    .accessibilityLabel(accessibilityName ?? "")
  }
}

public extension Toggle where Label == Icon {
  /// A toggle whose whole label is one glyph, drawn square at the control
  /// height so it sits in a toolbar beside the ``IconButton``s that do not
  /// latch.
  init(
    _ icon: FluentIcon,
    label: String,
    pressed: Bool = false,
    size: ControlSize = .tiny,
    variant: ControlVariant = .plain,
    tint: Tint = .neutral,
    enabled: Bool = true,
    action: (() -> Void)? = nil
  ) {
    self.init(
      pressed: pressed,
      size: size,
      variant: variant,
      tint: tint,
      enabled: enabled,
      glyphOnly: true,
      accessibilityName: label,
      action: action,
      // The same glyph step ``IconButton`` draws, so the two line up.
      label: { Icon(icon) }
    )
  }
}

public extension Toggle where Label == Text {
  /// A toggle whose label is a plain string.
  init(
    _ title: String,
    pressed: Bool = false,
    size: ControlSize = .tiny,
    variant: ControlVariant = .plain,
    tint: Tint = .neutral,
    enabled: Bool = true,
    grouped: Bool = false,
    action: (() -> Void)? = nil
  ) {
    self.init(
      pressed: pressed,
      size: size,
      variant: variant,
      tint: tint,
      enabled: enabled,
      grouped: grouped,
      action: action,
      label: { Text(title) }
    )
  }
}

/// One member of a ``ToggleGroup``.
public struct ToggleItem: Identifiable, Sendable {
  public var id: String { value }
  public var value: String
  public var label: String
  public var enabled: Bool
  /// Draws the member as a lone glyph, with ``label`` as its accessible name.
  public var icon: FluentIcon?

  public init(value: String, label: String, enabled: Bool = true, icon: FluentIcon? = nil) {
    self.value = value
    self.label = label
    self.enabled = enabled
    self.icon = icon
  }
}

/// Several toggles that belong together, joined into one control.
///
/// Joined rather than merely adjacent: the corners are rounded at the ends of
/// the run only and the shared edges collapse to one, which is what says the
/// group is one thing with several states rather than three buttons that happen
/// to touch. The ring belongs to the group and its members draw none of their
/// own, so a pair's shared rule is a single hairline.
///
/// ``multiple`` is where it parts from ``SegmentedControl``: several on at once,
/// or none. A group that must always have exactly one is that view, and drawing
/// it with this one would let the reader clear a choice that cannot be cleared.
public struct ToggleGroup: View {
  public var items: [ToggleItem]
  public var value: [String]
  public var multiple: Bool
  public var size: ControlSize
  public var variant: ControlVariant
  public var tint: Tint
  public var enabled: Bool
  public var onChange: (([String]) -> Void)?

  @Environment(\.theme) private var theme

  public init(
    items: [ToggleItem],
    value: [String],
    multiple: Bool = true,
    size: ControlSize = .small,
    variant: ControlVariant = .recessed,
    tint: Tint = .primary,
    enabled: Bool = true,
    onChange: (([String]) -> Void)? = nil
  ) {
    self.items = items
    self.value = value
    self.multiple = multiple
    self.size = size
    self.variant = variant
    self.tint = tint
    self.enabled = enabled
    self.onChange = onChange
  }

  public var body: some View {
    let vars = theme.vars
    let metrics = vars.metrics(size)
    // The run's own edge, read at rest: one ring for the whole group rather
    // than one per member, so the corners land only at the two ends.
    let edge = vars.resolve(
      vars.descriptor(variant, .border),
      tint: tint,
      state: enabled ? .normal : .disabled
    )

    HStack(spacing: 0) {
      ForEach(Array(items.enumerated()), id: \.element.id) { index, item in
        if index > 0 {
          // The rule between a pair, drawn once by the group.
          Rectangle().fill(edge).frame(width: vars.spacingPx)
        }
        Toggle(
          pressed: value.contains(item.value),
          size: size,
          variant: variant,
          tint: tint,
          enabled: enabled && item.enabled,
          grouped: true,
          glyphOnly: item.icon != nil,
          accessibilityName: item.icon == nil ? nil : item.label,
          action: { press(item.value) }
        ) {
          if let icon = item.icon {
            Icon(icon, size: metrics.iconSize)
          } else {
            Text(item.label)
          }
        }
      }
    }
    .fixedSize()
    .clipShape(RoundedRectangle(cornerRadius: metrics.radius, style: .circular))
    .border(edge, width: vars.spacingPx, cornerRadius: metrics.radius)
  }

  private func press(_ pressed: String) {
    guard let onChange else { return }
    let on = value.contains(pressed)
    guard multiple else {
      onChange(on ? [] : [pressed])
      return
    }
    onChange(on ? value.filter { $0 != pressed } : value + [pressed])
  }
}
