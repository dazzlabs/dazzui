import FluentIcons
import SwiftUI

/// `medium` in a sidebar, `small` in the narrower rail.
public enum NavItemSize: String, CaseIterable, Sendable {
  case small
  case medium
}

/// Whether the row selects or adds. Internal: a caller reaches the second one
/// through ``NavAction`` rather than by setting a flag.
enum NavItemRole {
  case item
  case action
}

/// One row of a navigation column, in a sidebar or a rail.
///
/// The idle row rests in the secondary ink at the quiet weight — a column of
/// full-ink rows would leave the selected one nothing to win by — and its
/// hover is the accent at 8%, a preview of the filled selection sitting far
/// below it. A menu item or a table row hovers neutral instead; the difference
/// is what the selected state is.
public struct NavItem<Leading: View, Label: View>: View {
  /// Marks this row as the page or pane currently shown.
  public var active: Bool
  public var size: NavItemSize
  public var tint: Tint
  public var enabled: Bool
  public var action: () -> Void
  /// Leading glyph. It follows the row's colour, so it tints with selection.
  public var icon: (() -> Leading)?
  @ViewBuilder public var label: () -> Label

  var role: NavItemRole = .item

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool

  public init(
    active: Bool = false,
    size: NavItemSize = .medium,
    tint: Tint = .primary,
    enabled: Bool = true,
    action: @escaping () -> Void,
    @ViewBuilder icon: @escaping () -> Leading,
    @ViewBuilder label: @escaping () -> Label
  ) {
    self.init(
      active: active,
      size: size,
      tint: tint,
      enabled: enabled,
      action: action,
      iconBuilder: icon,
      label: label
    )
  }

  /// The one initialiser that fills the box. An absent icon has to stay
  /// absent rather than becoming an empty one, because the glyph slot is a
  /// fixed width the label would otherwise be indented by.
  init(
    active: Bool,
    size: NavItemSize,
    tint: Tint,
    enabled: Bool,
    action: @escaping () -> Void,
    iconBuilder: (() -> Leading)?,
    label: @escaping () -> Label
  ) {
    self.active = active
    self.size = size
    self.tint = tint
    self.enabled = enabled
    self.action = action
    self.icon = iconBuilder
    self.label = label
  }

  public var body: some View {
    let vars = theme.vars

    Pressable(action: action, enabled: enabled, cornerRadius: vars.radiusSmall) { state in
      HStack(spacing: vars.spacing15) {
        if let icon {
          // The glyph is taller than the label, so it is boxed to the row's
          // line height and allowed to overflow rather than stretching the row.
          icon()
            .frame(width: vars.spacing4, height: vars.labelQuiet.size)
        }
        label()
        Spacer(minLength: 0)
      }
      .font(vars.labelQuiet.font)
      .fontWeight(role == .action ? vars.labelMedium.weight : vars.labelQuiet.weight)
      .lineLimit(1)
      .foregroundStyle(foreground(state: state))
      .padding(.vertical, paddingBlock)
      .padding(.horizontal, vars.spacing25)
      .frame(maxWidth: .infinity, alignment: .leading)
      .background(background(state: state))
      .clipShape(RoundedRectangle(cornerRadius: vars.radiusSmall, style: .continuous))
      .animation(vars.motion, value: state.controlState)
    }
    .focused($focused)
    .focusRing(
      visible: focused && enabled,
      color: vars.focusRing,
      cornerRadius: vars.radiusSmall,
      width: vars.focusWidth
    )
    .accessibilityAddTraits(active ? [.isSelected] : [])
  }

  /// The sidebar row: 12px on 8px pads — 28px, the regular control height. The
  /// rail row is a step tighter on both axes.
  private var paddingBlock: CGFloat {
    size == .medium ? theme.vars.spacing2 : theme.vars.menuItemPadding
  }

  private func background(state: PressableState) -> Color {
    let vars = theme.vars
    if active {
      return vars.resolve(vars.descriptor(.filled, .surface), tint: tint, state: .normal)
    }
    guard state.enabled, state.hovered || state.pressed else { return .clear }
    return vars.resolve(vars.descriptor(.plain, .surface), tint: tint, state: .hovered)
  }

  private func foreground(state: PressableState) -> Color {
    let vars = theme.vars
    if active {
      return vars.resolve(vars.descriptor(.filled, .content), tint: tint, state: .normal)
    }
    if !state.enabled {
      return vars.resolve(vars.descriptor(.normal, .content), tint: tint, state: .disabled)
    }
    // The hover rule outranks the action's own colour in the cascade, so both
    // kinds of row come to full ink under the pointer.
    if state.hovered || state.pressed { return vars.colorContent }
    return role == .action
      ? vars.resolve(vars.descriptor(.plain, .content), tint: tint, state: .normal)
      : vars.colorContentNav
  }
}

public extension NavItem where Leading == EmptyView {
  /// A row with no leading glyph.
  init(
    active: Bool = false,
    size: NavItemSize = .medium,
    tint: Tint = .primary,
    enabled: Bool = true,
    action: @escaping () -> Void,
    @ViewBuilder label: @escaping () -> Label
  ) {
    self.init(
      active: active,
      size: size,
      tint: tint,
      enabled: enabled,
      action: action,
      iconBuilder: nil,
      label: label
    )
  }
}

public extension NavItem where Leading == EmptyView, Label == Text {
  /// A row whose label is a plain string.
  init(
    _ title: String,
    active: Bool = false,
    size: NavItemSize = .medium,
    tint: Tint = .primary,
    enabled: Bool = true,
    action: @escaping () -> Void
  ) {
    self.init(
      active: active,
      size: size,
      tint: tint,
      enabled: enabled,
      action: action,
      label: { Text(title) }
    )
  }
}

public extension NavItem where Leading == Icon, Label == Text {
  /// A row drawn from an SF Symbol and a plain string.
  init(
    _ title: String,
    icon: FluentIcon,
    active: Bool = false,
    size: NavItemSize = .medium,
    tint: Tint = .primary,
    enabled: Bool = true,
    action: @escaping () -> Void
  ) {
    self.init(
      active: active,
      size: size,
      tint: tint,
      enabled: enabled,
      action: action,
      icon: { Icon(icon, size: 16) },
      label: { Text(title) }
    )
  }
}

/// The trailing action pinned to the foot of a navigation column — "New", "Add
/// a provider".
///
/// Shaped like a ``NavItem`` so the column reads as one list, but printed in
/// the ramp rather than filled with it, because it adds rather than selects —
/// and it never takes the current-page state or a leading glyph.
public struct NavAction<Label: View>: View {
  public var size: NavItemSize
  public var tint: Tint
  public var enabled: Bool
  public var action: () -> Void
  @ViewBuilder public var label: () -> Label

  public init(
    size: NavItemSize = .medium,
    tint: Tint = .primary,
    enabled: Bool = true,
    action: @escaping () -> Void,
    @ViewBuilder label: @escaping () -> Label
  ) {
    self.size = size
    self.tint = tint
    self.enabled = enabled
    self.action = action
    self.label = label
  }

  public var body: some View {
    var item = NavItem<EmptyView, Label>(
      size: size,
      tint: tint,
      enabled: enabled,
      action: action,
      label: label
    )
    item.role = .action

    // `margin-top: auto`. A flexible leading spacer is how a stack hands its
    // slack to one child, so the action sinks to the foot of the column when
    // there is room and stays in the flow when there is not.
    return VStack(spacing: 0) {
      Spacer(minLength: 0)
      item
    }
  }
}

public extension NavAction where Label == Text {
  /// An action whose label is a plain string.
  init(
    _ title: String,
    size: NavItemSize = .medium,
    tint: Tint = .primary,
    enabled: Bool = true,
    action: @escaping () -> Void
  ) {
    self.init(
      size: size,
      tint: tint,
      enabled: enabled,
      action: action,
      label: { Text(title) }
    )
  }
}
