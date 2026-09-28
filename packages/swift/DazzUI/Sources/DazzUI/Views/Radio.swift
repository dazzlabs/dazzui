import SwiftUI

/// The ring and its dot keep per-size dimensions of their own — that is the
/// point of `radio.<size>.box` and `radio.<size>.dot`.
public enum RadioSize: String, CaseIterable, Sendable {
  case small
  case medium
  case large
}

/// One choice. A set of them belongs in a ``RadioGroup``.
///
/// The row, the label and the disabled treatment are the checkbox's — see
/// ``Checkbox``. What differs is the shape: a radio is a ring that keeps its
/// outline when selected and grows a dot inside it, where a checkbox fills
/// solid.
public struct Radio<Label: View>: View {
  public var checked: Bool
  public var size: RadioSize
  /// Which colour ramp the selected ring and its dot are drawn from.
  public var tint: Tint
  public var enabled: Bool
  public var action: (() -> Void)?
  @ViewBuilder public var label: () -> Label

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool

  public init(
    checked: Bool,
    size: RadioSize = .medium,
    tint: Tint = .primary,
    enabled: Bool = true,
    action: (() -> Void)? = nil,
    @ViewBuilder label: @escaping () -> Label
  ) {
    self.checked = checked
    self.size = size
    self.tint = tint
    self.enabled = enabled
    self.action = action
    self.label = label
  }

  public var body: some View {
    let vars = theme.vars

    Pressable(action: action, enabled: enabled, cornerRadius: vars.radiusNone) { state in
      HStack(alignment: .center, spacing: vars.spacing25) {
        box
        // The line box the stylesheet states. Left to itself a `Text` is as
        // tall as its font's own leading, and a row whose box is smaller
        // than that would be set by the face rather than by the design.
        label().frame(height: vars.labelQuiet.lineHeight)
      }
      .font(.system(size: vars.labelQuiet.size, weight: labelWeight))
      .foregroundStyle(checked ? vars.colorContent : vars.colorContentMuted)
      .multilineTextAlignment(.leading)
      .opacity(state.enabled ? 1 : Self.disabledOpacity)
      .animation(vars.motion, value: checked)
    }
    .focused($focused)
    .focusRing(
      visible: focused && enabled,
      color: vars.focusRing,
      cornerRadius: vars.radiusNone,
      width: vars.focusWidth
    )
    .accessibilityAddTraits(checked ? [.isSelected] : [])
  }

  /// The ring, and the dot it grows. Both are the resting `filled` surface —
  /// a radio has no hover or press colour of its own, so it stays out of the
  /// shared state machine.
  private var box: some View {
    let vars = theme.vars
    let selected = vars.resolve(
      vars.descriptor(.filled, .surface),
      tint: tint,
      state: .normal
    )

    return ZStack {
      if checked {
        RoundedRectangle(cornerRadius: vars.radiusFull, style: .circular)
          .fill(selected)
          .frame(width: dotSize, height: dotSize)
      }
    }
    .frame(width: boxSize, height: boxSize)
    .border(
      checked ? selected : vars.colorBorderMuted,
      width: vars.radioThickness,
      cornerRadius: vars.radiusFull
    )
  }

  private var boxSize: CGFloat {
    switch size {
    case .small: return theme.vars.radioSmallBox
    case .medium: return theme.vars.radioMediumBox
    case .large: return theme.vars.radioLargeBox
    }
  }

  private var dotSize: CGFloat {
    switch size {
    case .small: return theme.vars.radioSmallDot
    case .medium: return theme.vars.radioMediumDot
    case .large: return theme.vars.radioLargeDot
    }
  }

  private var labelWeight: Font.Weight {
    checked ? theme.vars.labelMedium.weight : theme.vars.labelQuiet.weight
  }

  /// `:disabled { opacity: 0.6 }` — the stylesheet states the number outright,
  /// so there is no token to read it from.
  private static var disabledOpacity: CGFloat { 0.6 }
}

public extension Radio where Label == Text {
  /// A radio whose label is a plain string.
  init(
    _ title: String,
    checked: Bool,
    size: RadioSize = .medium,
    tint: Tint = .primary,
    enabled: Bool = true,
    action: (() -> Void)? = nil
  ) {
    self.init(
      checked: checked,
      size: size,
      tint: tint,
      enabled: enabled,
      action: action,
      label: { Text(title) }
    )
  }
}

/// One entry in a ``RadioGroup``.
public struct RadioGroupOption<Value: Hashable>: Identifiable {
  public var value: Value
  public var label: String
  public var enabled: Bool

  public init(value: Value, label: String, enabled: Bool = true) {
    self.value = value
    self.label = label
    self.enabled = enabled
  }

  public var id: Value { value }
}

/// A column of ``Radio`` rows over one value.
public struct RadioGroup<Value: Hashable>: View {
  public var options: [RadioGroupOption<Value>]
  public var value: Value
  public var onValueChange: ((Value) -> Void)?
  public var size: RadioSize
  public var tint: Tint
  /// A short set of answers can sit on one line, the way a scope choice does.
  public var axis: Axis
  /// The run between options; the column's own step when unset.
  public var spacing: CGFloat?

  @Environment(\.theme) private var theme

  public init(
    options: [RadioGroupOption<Value>],
    value: Value,
    onValueChange: ((Value) -> Void)? = nil,
    size: RadioSize = .medium,
    tint: Tint = .primary,
    axis: Axis = .vertical,
    spacing: CGFloat? = nil
  ) {
    self.options = options
    self.value = value
    self.onValueChange = onValueChange
    self.size = size
    self.tint = tint
    self.axis = axis
    self.spacing = spacing
  }

  public var body: some View {
    let vars = theme.vars

    let layout =
      axis == .vertical
      ? AnyLayout(VStackLayout(alignment: .leading, spacing: spacing ?? vars.spacing25))
      : AnyLayout(HStackLayout(spacing: spacing ?? vars.spacing25))

    layout {
      ForEach(options) { option in
        Radio(
          option.label,
          checked: option.value == value,
          size: size,
          tint: tint,
          enabled: option.enabled,
          action: { onValueChange?(option.value) }
        )
      }
    }
    .accessibilityElement(children: .contain)
  }
}
