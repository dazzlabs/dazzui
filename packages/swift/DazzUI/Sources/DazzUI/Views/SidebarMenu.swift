import FluentIcons
import SwiftUI

/// The six `control.color.*` recipes as a sidebar row spends them.
///
/// A row idles transparent in every variant but `normal`, so what the variant
/// decides is how the row marks itself as the **current page**, and its hover
/// follows from that: a row that selects with a fill can afford an accent
/// hover, since there is a long way between an 8% wash and a solid one; a row
/// that selects with a wash cannot, and hovers in grey instead.
///
/// `filled` is the destination row a window navigates by. `tinted` marks the
/// page more quietly, for a column whose selection is one of many. `outlined`
/// rings it instead of washing it, where the rows carry colour of their own.
/// `recessed` selects in grey, for a list whose selection is a cursor rather
/// than a destination. `normal` is the raised row a workspace switcher takes,
/// and is never a page. `plain` is printed in the ramp rather than filled with
/// it, for the row that adds rather than selects.
public enum SidebarMenuButtonVariant: String, CaseIterable, Sendable {
  case filled
  case tinted
  case outlined
  case recessed
  case normal
  case plain
}

/// `medium` in a sidebar, `small` in a narrower second column, `large` for a
/// two-line row.
public enum SidebarMenuButtonSize: String, CaseIterable, Sendable {
  case small
  case medium
  case large
}

/// A trailing inline action button inside a ``SidebarMenuButton``.
///
/// When `showOnHover` is true, the action stays hidden at rest and swaps in
/// over any ``SidebarMenuBadge`` while the row is hovered or active.
public struct SidebarMenuAction<Content: View>: View {
  public var showOnHover: Bool
  /// Keep the action visible even after pointer hover leaves (for example while
  /// its popup menu is open).
  public var active: Bool
  @ViewBuilder public var content: () -> Content

  @Environment(\.sidebarRowHovered) private var rowHovered
  @Environment(\.theme) private var theme

  public init(
    showOnHover: Bool = false,
    active: Bool = false,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.showOnHover = showOnHover
    self.active = active
    self.content = content
  }

  private var visible: Bool {
    !showOnHover || rowHovered || active
  }

  public var body: some View {
    content()
      .opacity(visible ? 1 : 0)
      .allowsHitTesting(visible)
      .animation(theme.vars.motion, value: visible)
  }
}

/// One row of a sidebar — `SidebarMenuButton` in `sidebar.tsx`, with its
/// `SidebarMenuBadge` and optional `SidebarMenuAction` on the trailing edge.
///
/// Idle, it rests in the nav ink at the quiet weight — a column of full-ink
/// rows would leave the selected one nothing to win by.
public struct SidebarMenuButton<Leading: View, Label: View, Badge: View, RowAction: View>: View {
  /// Marks this row as the page or pane currently shown.
  public var active: Bool
  public var size: SidebarMenuButtonSize
  public var variant: SidebarMenuButtonVariant
  public var tint: Tint
  public var enabled: Bool
  /// Tooltip label surfaced when the sidebar is collapsed to `.icon` mode.
  public var tooltip: String?
  /// When true, hovering the row (or setting `keepActionVisible`) reveals
  /// `rowAction` and hides `badge` in its place.
  public var showActionOnHover: Bool
  /// Keeps a hover-revealed `rowAction` visible (e.g. while a menu is open).
  public var keepActionVisible: Bool
  public var action: () -> Void
  /// Leading glyph. It follows the row's colour, so it tints with selection.
  public var icon: (() -> Leading)?
  @ViewBuilder public var label: () -> Label
  /// A count on the row's trailing edge, in the row's own colour.
  public var badge: (() -> Badge)?
  /// An interactive trailing control (such as a `...` menu button).
  public var rowAction: (() -> RowAction)?

  @Environment(\.theme) private var theme
  @Environment(\.sidebarCollapsed) private var sidebarCollapsed
  @Environment(\.sidebarCollapsible) private var sidebarCollapsible
  @FocusState private var focused: Bool
  @State private var rowHovered = false

  public init(
    active: Bool = false,
    size: SidebarMenuButtonSize = .medium,
    variant: SidebarMenuButtonVariant = .filled,
    tint: Tint = .primary,
    enabled: Bool = true,
    tooltip: String? = nil,
    showActionOnHover: Bool = false,
    keepActionVisible: Bool = false,
    action: @escaping () -> Void,
    @ViewBuilder icon: @escaping () -> Leading,
    @ViewBuilder label: @escaping () -> Label,
    @ViewBuilder badge: @escaping () -> Badge,
    @ViewBuilder rowAction: @escaping () -> RowAction
  ) {
    self.init(
      active: active, size: size, variant: variant, tint: tint, enabled: enabled,
      tooltip: tooltip, showActionOnHover: showActionOnHover, keepActionVisible: keepActionVisible,
      action: action, iconBuilder: icon, label: label, badgeBuilder: badge,
      rowActionBuilder: rowAction)
  }

  /// The one initialiser that fills the box. An absent icon stays absent
  /// rather than becoming an empty one, because the glyph slot is a fixed
  /// width the label would otherwise be indented by.
  init(
    active: Bool,
    size: SidebarMenuButtonSize,
    variant: SidebarMenuButtonVariant,
    tint: Tint,
    enabled: Bool,
    tooltip: String? = nil,
    showActionOnHover: Bool = false,
    keepActionVisible: Bool = false,
    action: @escaping () -> Void,
    iconBuilder: (() -> Leading)?,
    label: @escaping () -> Label,
    badgeBuilder: (() -> Badge)?,
    rowActionBuilder: (() -> RowAction)? = nil
  ) {
    self.active = active
    self.size = size
    self.variant = variant
    self.tint = tint
    self.enabled = enabled
    self.tooltip = tooltip
    self.showActionOnHover = showActionOnHover
    self.keepActionVisible = keepActionVisible
    self.action = action
    self.icon = iconBuilder
    self.label = label
    self.badge = badgeBuilder
    self.rowAction = rowActionBuilder
  }

  private var isIconCompact: Bool {
    sidebarCollapsed && sidebarCollapsible == .icon
  }

  private var isActionVisible: Bool {
    guard rowAction != nil else { return false }
    return !showActionOnHover || rowHovered || keepActionVisible
  }

  public var body: some View {
    if isIconCompact {
      compactBody
    } else {
      expandedBody
    }
  }

  @ViewBuilder
  private var compactBody: some View {
    let vars = theme.vars
    let radius = vars.radiusSmall
    let boxSize = vars.metrics(.medium).size

    let button = Pressable(action: action, enabled: enabled, cornerRadius: radius) { state in
      Group {
        if let icon {
          icon().environment(\.dazzIconSize, vars.iconMedium)
        } else {
          label().lineLimit(1)
        }
      }
      .foregroundStyle(foreground(state))
      .frame(width: boxSize, height: boxSize)
      .background(background(state))
      .clipShape(RoundedRectangle(cornerRadius: radius, style: .circular))
      .overlay {
        if let ring = ring(state) {
          RoundedRectangle(cornerRadius: radius, style: .circular)
            .strokeBorder(ring, lineWidth: vars.strokeHairline)
        }
      }
      .animation(vars.motion, value: state.controlState)
      .animation(vars.motion, value: active)
    }
    .focused($focused)
    .focusRing(
      visible: focused && enabled,
      color: vars.focusRing,
      cornerRadius: radius,
      width: vars.focusWidth
    )
    .accessibilityAddTraits(active ? [.isSelected] : [])

    if let tooltip, !tooltip.isEmpty {
      Tooltip(tooltip, side: .right, disabled: !enabled, presented: focused) {
        button
      }
    } else {
      button
    }
  }

  private var expandedBody: some View {
    let vars = theme.vars
    let radius = size == .large ? vars.radiusLarge : vars.radiusSmall
    let hasTrailing = badge != nil || rowAction != nil

    return Pressable(action: action, enabled: enabled, cornerRadius: radius) { state in
      HStack(spacing: vars.spacing15) {
        if let icon {
          // The glyph is taller than the label, so it is boxed to the row's
          // line height and allowed to overflow rather than stretching it.
          icon()
            .environment(\.dazzIconSize, vars.iconMedium)
            .frame(width: vars.iconMedium, height: vars.labelQuiet.size)
        }
        label()
          .truncationMode(.tail)
        Spacer(minLength: 0)
      }
      .font(vars.labelQuiet.font)
      .fontWeight(variant == .plain ? vars.labelMedium.weight : vars.labelQuiet.weight)
      .lineLimit(1)
      // `line-height: 1`: the content box is one em, and the padding is what
      // makes the row a control height. Left to itself a `Text` is as tall
      // as its face's leading, and the row would stand three points proud.
      .frame(height: vars.labelQuiet.size)
      .foregroundStyle(foreground(state))
      .padding(.vertical, paddingBlock)
      .padding(.leading, vars.spacing25)
      // A row with a count or action keeps its label clear of it.
      .padding(.trailing, hasTrailing ? vars.spacing8 : vars.spacing25)
      .frame(maxWidth: .infinity, minHeight: size == .large ? vars.spacing12 : nil, alignment: .leading)
      .background(background(state))
      .clipShape(RoundedRectangle(cornerRadius: radius, style: .circular))
      .overlay {
        if let ring = ring(state) {
          RoundedRectangle(cornerRadius: radius, style: .circular)
            .strokeBorder(ring, lineWidth: vars.strokeHairline)
        }
      }
      .overlay(alignment: .trailing) {
        if let badge, !(showActionOnHover && isActionVisible) {
          badge()
            .environment(\.sidebarBadgeInk, badgeColor(state))
            .padding(.trailing, rowAction != nil && !showActionOnHover ? vars.spacing7 : vars.spacing1)
        }
      }
      // The wash under the pointer and the fill that marks the current page
      // travel on the same token: a selection that landed while hover
      // crossfaded read as the one state the system had no opinion about.
      .animation(vars.motion, value: state.controlState)
      .animation(vars.motion, value: active)
    }
    .modifier(
      SidebarRowActionModifier(
        rowAction: rowAction,
        rowHovered: $rowHovered,
        keepActionVisible: keepActionVisible,
        isActionVisible: isActionVisible,
        trailingPadding: vars.spacing1,
        motion: vars.motion
      )
    )
    .focused($focused)
    .focusRing(
      visible: focused && enabled,
      color: vars.focusRing,
      cornerRadius: radius,
      width: vars.focusWidth
    )
    .accessibilityAddTraits(active ? [.isSelected] : [])
  }

  /// 12 on 8 pads — 28, the regular control height; the small row is a step
  /// tighter, the large one the two-line row.
  private var paddingBlock: CGFloat {
    size == .small ? theme.vars.menuItemPadding : theme.vars.spacing2
  }

  private func hovering(_ state: PressableState) -> Bool {
    state.enabled && (state.hovered || state.pressed)
  }

  private func color(_ variant: ControlVariant, _ part: ControlPart, _ state: ControlState) -> Color {
    theme.vars.controlColor(variant, part, tint: tint, state: state)
  }

  private func background(_ state: PressableState) -> Color {
    guard state.enabled else { return .clear }
    let hover = hovering(state) && !active
    switch variant {
    case .filled:
      if active { return color(.filled, .surface, .normal) }
      return hover ? color(.plain, .surface, .hovered) : .clear
    case .tinted:
      if active { return color(.tinted, .surface, .normal) }
      return hover ? color(.recessed, .surface, .normal) : .clear
    case .outlined:
      return hover ? color(.recessed, .surface, .normal) : .clear
    case .recessed:
      if active { return color(.recessed, .surface, .hovered) }
      return hover ? color(.recessed, .surface, .normal) : .clear
    case .normal:
      return active ? color(.normal, .surface, .hovered) : color(.normal, .surface, .normal)
    case .plain:
      return hovering(state) ? color(.plain, .surface, .hovered) : .clear
    }
  }

  private func ring(_ state: PressableState) -> Color? {
    guard state.enabled else { return nil }
    switch variant {
    case .outlined where active:
      return color(.outlined, .border, .normal)
    case .normal:
      if active { return color(.normal, .border, .normal) }
      return hovering(state) ? color(.outlined, .border, .hovered) : theme.vars.colorBorder
    default:
      return nil
    }
  }

  private func foreground(_ state: PressableState) -> Color {
    let vars = theme.vars
    if !state.enabled { return color(.normal, .content, .disabled) }
    if active {
      switch variant {
      case .filled: return color(.filled, .content, .normal)
      case .tinted: return color(.tinted, .content, .normal)
      case .outlined: return color(.outlined, .content, .normal)
      case .recessed: return color(.recessed, .content, .normal)
      case .normal: return color(.normal, .content, .normal)
      case .plain: break
      }
    }
    if variant == .plain {
      return hovering(state) ? color(.plain, .content, .hovered) : color(.plain, .content, .normal)
    }
    return hovering(state) ? vars.colorContent : vars.colorContentNav
  }

  /// The count follows the row's colour, so a selected row's count turns
  /// with it rather than staying grey on accent.
  private func badgeColor(_ state: PressableState) -> Color {
    let vars = theme.vars
    if active {
      switch variant {
      case .filled: return color(.filled, .content, .normal)
      case .tinted: return color(.tinted, .content, .normal)
      case .outlined: return color(.outlined, .content, .normal)
      case .recessed, .normal: return vars.colorContent
      case .plain: break
      }
    }
    return hovering(state) ? vars.colorContent : vars.colorContentNav
  }
}

public extension SidebarMenuButton where RowAction == EmptyView {
  /// A row with a glyph, label, and count.
  init(
    active: Bool = false,
    size: SidebarMenuButtonSize = .medium,
    variant: SidebarMenuButtonVariant = .filled,
    tint: Tint = .primary,
    enabled: Bool = true,
    tooltip: String? = nil,
    action: @escaping () -> Void,
    @ViewBuilder icon: @escaping () -> Leading,
    @ViewBuilder label: @escaping () -> Label,
    @ViewBuilder badge: @escaping () -> Badge
  ) {
    self.init(
      active: active, size: size, variant: variant, tint: tint, enabled: enabled,
      tooltip: tooltip, action: action, iconBuilder: icon, label: label,
      badgeBuilder: badge, rowActionBuilder: nil)
  }
}

public extension SidebarMenuButton where Badge == EmptyView, RowAction == EmptyView {
  /// A row with no count.
  init(
    active: Bool = false,
    size: SidebarMenuButtonSize = .medium,
    variant: SidebarMenuButtonVariant = .filled,
    tint: Tint = .primary,
    enabled: Bool = true,
    tooltip: String? = nil,
    action: @escaping () -> Void,
    @ViewBuilder icon: @escaping () -> Leading,
    @ViewBuilder label: @escaping () -> Label
  ) {
    self.init(
      active: active, size: size, variant: variant, tint: tint, enabled: enabled,
      tooltip: tooltip, action: action, iconBuilder: icon, label: label,
      badgeBuilder: nil, rowActionBuilder: nil)
  }
}

public extension SidebarMenuButton where Leading == EmptyView, Badge == EmptyView, RowAction == EmptyView {
  /// A row with neither a glyph nor a count.
  init(
    active: Bool = false,
    size: SidebarMenuButtonSize = .medium,
    variant: SidebarMenuButtonVariant = .filled,
    tint: Tint = .primary,
    enabled: Bool = true,
    tooltip: String? = nil,
    action: @escaping () -> Void,
    @ViewBuilder label: @escaping () -> Label
  ) {
    self.init(
      active: active, size: size, variant: variant, tint: tint, enabled: enabled,
      tooltip: tooltip, action: action, iconBuilder: nil, label: label,
      badgeBuilder: nil, rowActionBuilder: nil)
  }
}

public extension SidebarMenuButton where Leading == EmptyView, RowAction == EmptyView {
  /// A row with a count and no glyph.
  init(
    active: Bool = false,
    size: SidebarMenuButtonSize = .medium,
    variant: SidebarMenuButtonVariant = .filled,
    tint: Tint = .primary,
    enabled: Bool = true,
    tooltip: String? = nil,
    action: @escaping () -> Void,
    @ViewBuilder label: @escaping () -> Label,
    @ViewBuilder badge: @escaping () -> Badge
  ) {
    self.init(
      active: active, size: size, variant: variant, tint: tint, enabled: enabled,
      tooltip: tooltip, action: action, iconBuilder: nil, label: label,
      badgeBuilder: badge, rowActionBuilder: nil)
  }
}

public extension SidebarMenuButton where Badge == EmptyView {
  /// A row with a glyph, label, and trailing action (no badge).
  init(
    active: Bool = false,
    size: SidebarMenuButtonSize = .medium,
    variant: SidebarMenuButtonVariant = .filled,
    tint: Tint = .primary,
    enabled: Bool = true,
    tooltip: String? = nil,
    showActionOnHover: Bool = false,
    keepActionVisible: Bool = false,
    action: @escaping () -> Void,
    @ViewBuilder icon: @escaping () -> Leading,
    @ViewBuilder label: @escaping () -> Label,
    @ViewBuilder rowAction: @escaping () -> RowAction
  ) {
    self.init(
      active: active, size: size, variant: variant, tint: tint, enabled: enabled,
      tooltip: tooltip, showActionOnHover: showActionOnHover, keepActionVisible: keepActionVisible,
      action: action, iconBuilder: icon, label: label, badgeBuilder: nil,
      rowActionBuilder: rowAction)
  }
}

/// A count on a sidebar row's trailing edge — `SidebarMenuBadge`. Passed as
/// a row's `badge`, it takes the row's colour for the row's state.
public struct SidebarMenuBadge<Content: View>: View {
  @ViewBuilder var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.sidebarBadgeInk) private var color

  public init(@ViewBuilder content: @escaping () -> Content) {
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars
    content()
      .font(vars.captionSmall.font)
      .fontWeight(vars.labelMedium.weight)
      .monospacedDigit()
      .lineLimit(1)
      .foregroundStyle(color ?? vars.colorContentNav)
      .padding(.horizontal, vars.spacing1)
      .frame(minWidth: vars.spacing5, minHeight: vars.spacing5)
      .allowsHitTesting(false)
  }
}

public extension SidebarMenuBadge where Content == Text {
  /// A count from a string.
  init(_ text: String) {
    self.init { Text(text) }
  }
}

private struct SidebarRowActionModifier<RowAction: View>: ViewModifier {
  let rowAction: (() -> RowAction)?
  @Binding var rowHovered: Bool
  let keepActionVisible: Bool
  let isActionVisible: Bool
  let trailingPadding: CGFloat
  let motion: Animation?

  func body(content: Content) -> some View {
    if let rowAction {
      content
        .overlay(alignment: .trailing) {
          rowAction()
            .environment(\.sidebarRowHovered, rowHovered || keepActionVisible)
            .padding(.trailing, trailingPadding)
            .opacity(isActionVisible ? 1 : 0)
            .allowsHitTesting(isActionVisible)
            .animation(motion, value: isActionVisible)
        }
        .onHover { inside in
          if rowHovered != inside { rowHovered = inside }
        }
    } else {
      content
    }
  }
}

private struct SidebarBadgeInkKey: EnvironmentKey {
  static let defaultValue: Color? = nil
}

private struct SidebarRowHoveredKey: EnvironmentKey {
  static let defaultValue = false
}

extension EnvironmentValues {
  /// The ink a row hands the count beside it: the count is the row's, so it
  /// turns with the row's state.
  var sidebarBadgeInk: Color? {
    get { self[SidebarBadgeInkKey.self] }
    set { self[SidebarBadgeInkKey.self] = newValue }
  }

  var sidebarRowHovered: Bool {
    get { self[SidebarRowHoveredKey.self] }
    set { self[SidebarRowHoveredKey.self] = newValue }
  }
}
