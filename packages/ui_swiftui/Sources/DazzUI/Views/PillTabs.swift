import SwiftUI

/// One pill in a ``PillTabs`` row.
public struct PillTabItem<Value: Hashable>: Identifiable {
  public var value: Value
  public var label: String
  /// Trailing count, set inside the pill.
  public var count: Int?
  public var disabled: Bool

  public var id: Value { value }

  public init(
    value: Value,
    label: String,
    count: Int? = nil,
    disabled: Bool = false
  ) {
    self.value = value
    self.label = label
    self.count = count
    self.disabled = disabled
  }
}

/// Free-standing filter pills.
///
/// Unlike a segmented control there is no track: each pill stands on the page,
/// so an unselected one needs its own fill to be a target at all — the inset
/// step, one deeper than the cards around it, with the groove as its hover.
///
/// One size, drawn tight. The active pill jumps two weight steps (600 → 700)
/// as well as filling, because at this size the fill alone can vanish under a
/// thumb.
public struct PillTabs<Value: Hashable>: View {
  public var items: [PillTabItem<Value>]
  public var value: Value
  public var onValueChange: ((Value) -> Void)?
  public var tint: Tint

  @Environment(\.theme) private var theme

  public init(
    items: [PillTabItem<Value>],
    value: Value,
    onValueChange: ((Value) -> Void)? = nil,
    tint: Tint = .primary
  ) {
    self.items = items
    self.value = value
    self.onValueChange = onValueChange
    self.tint = tint
  }

  public var body: some View {
    HStack(spacing: theme.vars.spacing2) {
      ForEach(items) { item in
        PillTab(
          item: item,
          selected: item.value == value,
          tint: tint,
          action: { onValueChange?(item.value) }
        )
      }
    }
    .accessibilityElement(children: .contain)
  }
}

private struct PillTab<Value: Hashable>: View {
  let item: PillTabItem<Value>
  let selected: Bool
  let tint: Tint
  let action: () -> Void

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool

  var body: some View {
    let vars = theme.vars

    Pressable(
      action: action,
      enabled: !item.disabled,
      cornerRadius: vars.radiusFull
    ) { state in
      HStack(spacing: vars.controlTinyGap) {
        Text(item.label)
        if let count = item.count {
          // The count runs on in the label's own ink — "收藏 64" reads as one
          // label, not a label wearing a badge.
          Text(String(count)).monospacedDigit()
        }
      }
      .font(.system(size: vars.labelSmall.size, weight: weight))
      .lineLimit(1)
      .foregroundStyle(content(state: state))
      .padding(.horizontal, vars.spacing3)
      .frame(height: vars.controlTinySize)
      .background(surface(state: state))
      .clipShape(RoundedRectangle(cornerRadius: vars.radiusFull, style: .continuous))
      .opacity(item.disabled ? Self.disabledOpacity : 1)
      .animation(vars.motion, value: state.controlState)
    }
    .focused($focused)
    .focusRing(
      visible: focused && !item.disabled,
      color: vars.focusRing,
      cornerRadius: vars.radiusFull,
      width: vars.focusWidth
    )
  }

  /// The half-strength the stylesheet dims a disabled pill to — a fade rather
  /// than the groove, because a pill has no chrome left to flatten.
  private static var disabledOpacity: CGFloat { 0.5 }

  private var weight: Font.Weight {
    selected ? theme.vars.labelStrong.weight : theme.vars.labelSmall.weight
  }

  private func surface(state: PressableState) -> Color {
    let vars = theme.vars
    if selected {
      return vars.resolve(vars.descriptor(.filled, .surface), tint: tint, state: .normal)
    }
    if state.enabled, state.hovered || state.pressed { return vars.colorSurfaceSunken }
    return vars.colorSurfaceInset
  }

  private func content(state: PressableState) -> Color {
    let vars = theme.vars
    if selected {
      return vars.resolve(vars.descriptor(.filled, .content), tint: tint, state: .normal)
    }
    if state.enabled, state.hovered || state.pressed { return vars.colorContent }
    return vars.colorContentNav
  }
}
