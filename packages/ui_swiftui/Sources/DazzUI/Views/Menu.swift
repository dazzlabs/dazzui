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
  /// Trailing hint — the keys that also trigger this item.
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
      icon: icon.map { AnyView(Icon($0, size: 14)) },
      shortcut: shortcut.map { AnyView(Text($0)) },
      checked: checked,
      disabled: disabled,
      onSelect: onSelect
    )
  }
}

/// A popover menu.
///
/// It closes on Escape and on selecting a row, so it stays operable from the
/// keyboard as well as the mouse. Dismissal on an outside press is the host
/// window's job here rather than the menu's: SwiftUI has no equivalent of the
/// document-level pointer listener the React menu installs, and a full-screen
/// catcher would swallow the very click the user meant for something else.
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
      .overlay(alignment: align == .start ? .bottomLeading : .bottomTrailing) {
        if open {
          panel
            // Pins the panel's *top* edge to the trigger's bottom edge, which
            // is what `top: 100%` does to an absolutely placed panel.
            .alignmentGuide(.bottom) { _ in 0 }
            .offset(y: theme.vars.spacing1)
        }
      }
      .background {
        if open {
          SwiftUI.Button("") { open = false }
            .keyboardShortcut(.cancelAction)
            .opacity(0)
            .frame(width: 0, height: 0)
        }
      }
  }

  private var panel: some View {
    let vars = theme.vars

    return VStack(spacing: 0) {
      ForEach(items) { item in
        MenuRow(item: item, tint: tint) { open = false }
      }
    }
    .padding(vars.spacing1)
    .frame(minWidth: vars.menuMinWidth, alignment: .leading)
    .background(vars.colorSurface)
    .clipShape(RoundedRectangle(cornerRadius: vars.radiusLarge, style: .circular))
    .border(vars.colorBorderStrong, width: hairline, cornerRadius: vars.radiusLarge)
    .tokenShadow(vars.shadowLg)
    .fixedSize()
    .accessibilityElement(children: .contain)
  }
}

/// A menu row: a quiet-weight label on a short row, where `menu.item-padding`
/// carries the odd value that squares the two.
///
/// The row's corner is the panel's minus its padding, so the hover wash stays
/// concentric with the panel edge.
private struct MenuRow: View {
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
          Icon(.checkmark12Filled, size: vars.labelSmall.size)
            .frame(width: vars.spacing3)
            .opacity(checked ? 1 : 0)
        }
        if let icon = item.icon {
          icon.foregroundStyle(vars.colorContentMuted)
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
