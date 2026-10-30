import FluentIcons
import SwiftUI

/// Which edge of the trigger the panel hangs from.
public enum MenuAlign: String, CaseIterable, Sendable {
  case start
  case end
}

/// What a ``Menu`` hands its trigger builder: whether the panel is open, and
/// the handler that flips it.
public struct MenuTriggerState {
  public var open: Bool
  public var toggle: () -> Void

  public init(open: Bool, toggle: @escaping () -> Void) {
    self.open = open
    self.toggle = toggle
  }
}

/// One row of a ``Menu``.
///
/// The leading slots are data rather than a builder because a menu is a list
/// of choices rather than a layout — the row draws itself, and the item only
/// says what it is.
public struct MenuItem: Identifiable {
  public var id: String { label }
  public var label: String
  public var icon: AnyView?
  /// Trailing hint; displaying it does not register a keyboard shortcut.
  public var shortcut: AnyView?
  /// Radio-style check state. Set it on every item of a group, including the
  /// unchecked ones, so the leading column stays aligned.
  public var checked: Bool?
  public var disabled: Bool
  public var onSelect: (() -> Void)?

  public init(
    label: String,
    icon: AnyView? = nil,
    shortcut: AnyView? = nil,
    checked: Bool? = nil,
    disabled: Bool = false,
    onSelect: (() -> Void)? = nil
  ) {
    self.label = label
    self.icon = icon
    self.shortcut = shortcut
    self.checked = checked
    self.disabled = disabled
    self.onSelect = onSelect
  }

  /// The common row: a label, optionally an SF Symbol and a shortcut set as
  /// glyphs rather than boxed as key caps.
  public init(
    _ label: String,
    icon: FluentIcon? = nil,
    shortcut: String? = nil,
    checked: Bool? = nil,
    disabled: Bool = false,
    onSelect: (() -> Void)? = nil
  ) {
    self.init(
      label: label,
      icon: icon.map { AnyView(Icon($0)) },
      shortcut: shortcut.map { AnyView(Text($0)) },
      checked: checked,
      disabled: disabled,
      onSelect: onSelect
    )
  }
}

/// A popover menu.
///
/// It closes on Escape, on selecting a row, and on a press outside it, so it
/// stays operable from the keyboard as well as the mouse. The panel is a
/// ``View/floatingPanel(isPresented:side:align:gap:modal:panel:)``, so it is
/// drawn above the whole window rather than only above the trigger's siblings,
/// and like Base UI's menu it is modal: the window behind waits while it is
/// open.
///
/// Named `Menu` rather than prefixed, and reached as `DazzUI.Menu` where
/// `SwiftUI.Menu` is also in scope.
public struct Menu<Trigger: View>: View {
  @ViewBuilder public var trigger: (MenuTriggerState) -> Trigger
  public var items: [MenuItem]
  public var align: MenuAlign
  /// Which colour ramp the row wash resolves against.
  public var tint: Tint

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline
  @Environment(\.menuPresentation) private var presentation
  @State private var open = false

  public init(
    items: [MenuItem],
    align: MenuAlign = .end,
    tint: Tint = .neutral,
    @ViewBuilder trigger: @escaping (MenuTriggerState) -> Trigger
  ) {
    self.items = items
    self.align = align
    self.tint = tint
    self.trigger = trigger
  }

  public var body: some View {
    trigger(MenuTriggerState(open: open, toggle: { open.toggle() }))
      // Modal, as Base UI's menu is by default: a press outside only closes
      // it, so pressing the trigger again closes rather than reopens.
      .floatingPanel(
        isPresented: Binding(get: { open && !presentation.usesNativeMenus }, set: { open = $0 }),
        side: .bottom,
        align: align == .start ? .start : .end,
        gap: theme.vars.spacing1,
        modal: true
      ) {
        panel
      }
      .nativeMenu(isPresented: $open, entries: items.map(NativeMenuEntry.item), align: align)
  }

  private var panel: some View {
    MenuSheet {
      ForEach(items) { item in
        MenuRow(item: item, tint: tint) { open = false }
      }
    }
  }
}

/// The floating panel a menu's rows sit in.
///
/// `popup.css` gives it a `min-width` and no width at all, so the panel is as
/// wide as its widest row and never narrower than `menu.min-width` — a menu
/// narrower than its trigger reads as broken, whatever its labels.
///
/// A ``ContextMenu``, a ``Menubar`` and a ``Combobox``'s list wear the same
/// sheet, which is what keeps the four from drifting.
struct MenuSheet<Content: View>: View {
  var minWidth: CGFloat?
  @ViewBuilder var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  init(minWidth: CGFloat? = nil, @ViewBuilder content: @escaping () -> Content) {
    self.minWidth = minWidth
    self.content = content
  }

  var body: some View {
    let vars = theme.vars

    return VStack(spacing: 0) {
      content()
    }
    .padding(vars.spacing1)
    .frame(minWidth: minWidth ?? vars.menuMinWidth, alignment: .leading)
    .background(vars.colorSurfaceOverlay)
    .clipShape(RoundedRectangle(cornerRadius: vars.radiusLarge, style: .circular))
    .border(vars.colorBorderStrong, width: hairline, cornerRadius: vars.radiusLarge)
    .tokenShadow(vars.shadowLg)
    .fixedSize(horizontal: minWidth == nil, vertical: true)
    .accessibilityElement(children: .contain)
  }
}

/// The sheet a list dropped from a field or a select wears: at least as wide
/// as what it dropped from — `min-width: var(--anchor-width)` — and wider
/// where a row needs it. The sheet sizes to its content, so the floor goes on
/// the content, less the sheet's own inset.
struct AnchoredMenuSheet<Rows: View>: View {
  let width: CGFloat
  @ViewBuilder let rows: () -> Rows

  @Environment(\.theme) private var theme

  var body: some View {
    let inset = theme.vars.spacing1 * 2

    MenuSheet {
      VStack(spacing: 0) { rows() }
        .frame(minWidth: max(0, width - inset))
    }
  }
}

/// A group heading inside the sheet: the section label's voice, on the row's
/// own inset. A ``Combobox``'s grouped list is what reaches for it.
struct MenuGroupLabel: View {
  let title: String

  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars

    Text(title)
      .font(.system(size: vars.captionSmall.size, weight: vars.labelSmall.weight))
      .foregroundStyle(vars.colorContentSubtle)
      .lineLimit(1)
      // `line-height: 1`: the line box is the face's size, and the face's own
      // taller line overhangs it evenly, as CSS centres it.
      .frame(height: vars.captionSmall.size)
      .frame(maxWidth: .infinity, alignment: .leading)
      .padding(.top, vars.spacing15)
      .padding(.bottom, vars.spacing1)
      .padding(.horizontal, vars.spacing2)
  }
}

/// What the sheet says when a query matches nothing.
///
/// A line of prose in a panel of rows, so it takes the row's inset and the
/// faint ink rather than looking like a row that cannot be picked.
struct MenuEmpty: View {
  let label: String

  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars

    Text(label)
      .font(vars.bodySmall.font)
      .foregroundStyle(vars.colorContentFaint)
      .frame(maxWidth: .infinity, alignment: .leading)
      .padding(.vertical, vars.menuItemPadding)
      .padding(.horizontal, vars.spacing2)
  }
}

/// A menu row: a quiet-weight label on a short row, where `menu.item-padding`
/// carries the odd value that squares the two.
///
/// The row's corner is the panel's minus its padding, so the hover wash stays
/// concentric with the panel edge.
struct MenuRow: View {
  let item: MenuItem
  let tint: Tint
  let dismiss: () -> Void

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool

  var body: some View {
    let vars = theme.vars
    let radius = vars.radiusLarge - vars.spacing1

    Pressable(
      action: {
        item.onSelect?()
        dismiss()
      },
      enabled: !item.disabled,
      cornerRadius: radius
    ) { state in
      HStack(spacing: vars.spacing2) {
        if let checked = item.checked {
          // `Checkmark12Filled`, the glyph `menu.tsx` names. The column holds
          // its width whether or not the item is checked, so the labels of a
          // radio group stay on one left edge.
          Icon(.checkmark12Filled, size: vars.iconSmall)
            .frame(width: vars.iconSmall)
            .opacity(checked ? 1 : 0)
        }
        if let icon = item.icon {
          icon.environment(\.dazzIconSize, vars.iconMedium).foregroundStyle(vars.colorContentMuted)
        }
        Text(item.label)
          .frame(maxWidth: .infinity, alignment: .leading)
        if let shortcut = item.shortcut {
          // A faint hint in the display cut — never a boxed key cap; a cap in
          // a row this short would be furniture.
          shortcut
            .font(.system(size: vars.labelSmall.size, weight: vars.labelSmall.weight))
            .foregroundStyle(vars.colorContentFaint)
        }
      }
      .font(vars.labelQuiet.font)
      .lineLimit(1)
      .foregroundStyle(content(state: state))
      .padding(.vertical, vars.menuItemPadding)
      .padding(.horizontal, vars.spacing2)
      .frame(maxWidth: .infinity, alignment: .leading)
      .background(surface(state: state))
      .clipShape(RoundedRectangle(cornerRadius: radius, style: .circular))
      .animation(vars.motion, value: state.controlState)
    }
    .focused($focused)
    .focusRing(
      visible: focused && !item.disabled,
      color: vars.focusRing,
      cornerRadius: radius,
      width: vars.focusWidth
    )
  }

  /// The `normal` recipe, not `plain`. Both have no resting fill, but `plain`
  /// is the quiet *accent* control — its wash comes off the tinted ramp, which
  /// on a row would land heavier than a tinted selection sitting one step
  /// lower. A row's wash is neutral, and it holds while pressed: a menu item
  /// has no pressed look of its own.
  private func surface(state: PressableState) -> Color {
    guard state.enabled, state.hovered || state.pressed else { return .clear }
    return theme.vars.resolve(
      theme.vars.descriptor(.normal, .surface),
      tint: tint,
      state: .hovered
    )
  }

  private func content(state: PressableState) -> Color {
    guard state.enabled else {
      return theme.vars.resolve(
        theme.vars.descriptor(.normal, .content),
        tint: tint,
        state: .disabled
      )
    }
    return theme.vars.colorContent
  }
}
