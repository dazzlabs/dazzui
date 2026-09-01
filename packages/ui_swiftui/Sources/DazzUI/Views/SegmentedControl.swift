import SwiftUI

/// The three track sizes. Each reads the matching control profile, so a
/// segmented control lines up with the buttons beside it.
public enum SegmentedControlSize: String, CaseIterable, Sendable {
  case small
  case medium
  case large

  /// The control profile this track is drawn to.
  var control: ControlSize {
    switch self {
    case .small: return .small
    case .medium: return .medium
    case .large: return .large
    }
  }
}

/// How the chosen segment is picked out.
///
/// `raised` lifts it off the track, the way a desktop segmented control draws
/// it. `filled` paints it with the tint — louder, and worth reserving for a
/// choice that has to shout.
public enum SegmentedActiveStyle: String, CaseIterable, Sendable {
  case raised
  case filled
}

/// One segment in a ``SegmentedControl``.
public struct SegmentedItem<Value: Hashable>: Identifiable {
  public var value: Value
  public var label: String
  public var disabled: Bool

  public var id: Value { value }

  public init(value: Value, label: String, disabled: Bool = false) {
    self.value = value
    self.label = label
    self.disabled = disabled
  }
}

/// A recessed track with one segment picked out.
///
/// The nesting is derived, not restated: the track takes the control corner,
/// the capsule sits `segmentedControlInset` inside it, and its corner is the
/// track's minus the inset — the only relationship that keeps the two curves
/// concentric. The segment's height is likewise the control height minus the
/// inset on both sides.
public struct SegmentedControl<Value: Hashable>: View {
  public var items: [SegmentedItem<Value>]
  public var value: Value
  public var onValueChange: ((Value) -> Void)?
  public var size: SegmentedControlSize
  public var tint: Tint
  public var activeStyle: SegmentedActiveStyle
  /// Fill the available width, splitting it evenly between the segments.
  public var stretch: Bool

  @Environment(\.theme) private var theme

  public init(
    items: [SegmentedItem<Value>],
    value: Value,
    onValueChange: ((Value) -> Void)? = nil,
    size: SegmentedControlSize = .medium,
    tint: Tint = .primary,
    activeStyle: SegmentedActiveStyle = .raised,
    stretch: Bool = false
  ) {
    self.items = items
    self.value = value
    self.onValueChange = onValueChange
    self.size = size
    self.tint = tint
    self.activeStyle = activeStyle
    self.stretch = stretch
  }

  public var body: some View {
    let vars = theme.vars

    HStack(spacing: 0) {
      ForEach(items) { item in
        Segment(
          item: item,
          checked: item.value == value,
          size: size,
          tint: tint,
          activeStyle: activeStyle,
          stretch: stretch,
          trackRadius: trackRadius,
          action: { onValueChange?(item.value) }
        )
      }
    }
    .padding(vars.segmentedControlInset)
    .frame(maxWidth: stretch ? .infinity : nil)
    .background(vars.colorSurfaceInset)
    .clipShape(RoundedRectangle(cornerRadius: trackRadius, style: .continuous))
    .accessibilityElement(children: .contain)
  }

  /// The medium track takes the regular control corner — one step above the
  /// button beside it, because the track is a container for capsules rather
  /// than a control face itself.
  private var trackRadius: CGFloat {
    size == .medium ? theme.vars.radiusMedium : theme.vars.metrics(size.control).radius
  }
}

private struct Segment<Value: Hashable>: View {
  let item: SegmentedItem<Value>
  let checked: Bool
  let size: SegmentedControlSize
  let tint: Tint
  let activeStyle: SegmentedActiveStyle
  let stretch: Bool
  let trackRadius: CGFloat
  let action: () -> Void

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool

  var body: some View {
    let vars = theme.vars

    Pressable(action: action, enabled: !item.disabled, cornerRadius: radius) { state in
      Text(item.label)
        .font(.system(size: vars.metrics(size.control).content.size, weight: weight))
        .lineLimit(1)
        .multilineTextAlignment(.center)
        .foregroundStyle(content(state: state))
        .padding(.horizontal, vars.spacing3)
        .frame(maxWidth: stretch ? .infinity : nil)
        .frame(height: height)
        .background(surface)
        .clipShape(RoundedRectangle(cornerRadius: radius, style: .continuous))
        // Both active styles lift with the hairline shadow, and the label
        // takes the full label weight — selection is a weight shift as much
        // as a fill.
        .tokenShadow(checked ? vars.shadow2xs : [])
        .opacity(item.disabled ? Self.disabledOpacity : 1)
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

  /// The half-strength the stylesheet dims a disabled segment to.
  private static var disabledOpacity: CGFloat { 0.5 }

  private var height: CGFloat {
    let vars = theme.vars
    return vars.metrics(size.control).size - 2 * vars.segmentedControlInset
  }

  private var radius: CGFloat {
    trackRadius - theme.vars.segmentedControlInset
  }

  private var weight: Font.Weight {
    checked ? theme.vars.labelMedium.weight : theme.vars.labelQuiet.weight
  }

  private var surface: Color {
    let vars = theme.vars
    guard checked else { return .clear }
    switch activeStyle {
    case .raised: return vars.colorSurfaceRaised
    case .filled:
      return vars.resolve(vars.descriptor(.filled, .surface), tint: tint, state: .normal)
    }
  }

  private func content(state: PressableState) -> Color {
    let vars = theme.vars
    if checked {
      switch activeStyle {
      case .raised: return vars.colorContent
      case .filled:
        return vars.resolve(vars.descriptor(.filled, .content), tint: tint, state: .normal)
      }
    }
    if state.enabled, state.hovered || state.pressed { return vars.colorContent }
    return vars.colorContentSecondary
  }
}
