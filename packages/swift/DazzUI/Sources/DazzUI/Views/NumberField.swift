import FluentIcons
import SwiftUI

/// A number, with the two things a plain text control never gets right: a
/// stepper you can hold down, and a value that stays a number.
///
/// The box is the ``TextField``'s — `FieldStyle` draws it for both — so a form
/// mixing the two does not step. What is this view's own is the stepper at the
/// trailing edge, and the clamp that keeps the value inside its bounds.
public struct NumberField: View {
  /// The value, or `nil` while the field is empty.
  @Binding public var value: Double?
  public var minimum: Double?
  public var maximum: Double?
  /// What one press of the stepper, and one arrow key, moves.
  public var step: Double
  public var placeholder: String
  /// A trailing unit inside the box — `px`, `%`, `ms`. It is a hint rather
  /// than part of the value, so it never enters what the field holds.
  public var unit: String?
  public var size: TextFieldSize
  public var state: TextFieldState
  public var tint: Tint
  public var enabled: Bool

  @Environment(\.theme) private var theme
  @FocusState private var focused: Bool
  @State private var text: String

  public init(
    value: Binding<Double?>,
    minimum: Double? = nil,
    maximum: Double? = nil,
    step: Double = 1,
    placeholder: String = "",
    unit: String? = nil,
    size: TextFieldSize = .medium,
    state: TextFieldState = .default,
    tint: Tint = .primary,
    enabled: Bool = true
  ) {
    self._value = value
    self.minimum = minimum
    self.maximum = maximum
    self.step = step
    self.placeholder = placeholder
    self.unit = unit
    self.size = size
    self.state = state
    self.tint = tint
    self.enabled = enabled
    self._text = State(initialValue: Self.format(value.wrappedValue))
  }

  public var body: some View {
    let vars = theme.vars
    let metrics = vars.metrics(size.controlSize)
    let ramp = tintFor(state, tint)

    HStack(spacing: vars.spacing1) {
      ZStack(alignment: .leading) {
        if text.isEmpty {
          Text(placeholder).foregroundStyle(vars.colorContentFaint)
        }
        SwiftUI.TextField("", text: $text)
          .textFieldStyle(.plain)
          .focused($focused)
          .disabled(!enabled)
          // The single-argument `onChange` rather than the pair: the package
          // deploys to macOS 13, where the two-argument form does not exist.
          .onChange(of: text) { next in
            value = Double(next.trimmingCharacters(in: .whitespaces)).map(clamp)
          }
          .onChange(of: value) { next in
            // Only when the caller moved it: rewriting on every change would
            // fight the caret while the reader is typing.
            if Double(text) != next { text = Self.format(next) }
          }
      }
      .lineLimit(1)
      .font(FieldStyle.font(size: metrics.content.size, mono: false, vars: vars))
      if let unit {
        Text(unit)
          .font(.system(size: vars.labelSmall.size, weight: vars.labelQuiet.weight))
          .foregroundStyle(vars.colorContentFaint)
      }
      // Stacked rather than a minus and a plus on either side: the field is
      // read left to right, and a value with a control on both sides of it
      // reads as three things. The pair is one AppKit stepper.
      Stepper(
        height: metrics.size - vars.spacing1 * 2,
        increase: canIncrease ? { nudge(step) } : nil,
        decrease: canDecrease ? { nudge(-step) } : nil
      )
    }
    // The trailing step is the stepper's own inset: it is a control rather
    // than text, so it sits closer to the edge than a value.
    .padding(.leading, vars.spacing3)
    .padding(.trailing, vars.spacing1)
    .frame(height: metrics.size)
    .fieldChrome(
      FieldStyle(
        vars: vars,
        tint: ramp,
        invalid: state == .error,
        enabled: enabled,
        focused: focused
      ),
      cornerRadius: vars.controlFieldRadius,
      vars: vars
    )
  }

  private func nudge(_ delta: Double) {
    let next = clamp((value ?? minimum ?? 0) + delta)
    value = next
    text = Self.format(next)
  }

  private func clamp(_ candidate: Double) -> Double {
    var result = candidate
    if let minimum { result = max(minimum, result) }
    if let maximum { result = min(maximum, result) }
    return result
  }

  private var canIncrease: Bool {
    enabled && maximum.map { (value ?? minimum ?? 0) < $0 } ?? true
  }

  private var canDecrease: Bool {
    enabled && minimum.map { (value ?? maximum ?? 0) > $0 } ?? true
  }

  /// A whole number prints without its zeros; anything else prints as typed.
  private static func format(_ value: Double?) -> String {
    guard let value else { return "" }
    return value == value.rounded() ? String(Int(value)) : String(value)
  }
}

/// Two half-height buttons in one column, hairlined off the value.
private struct Stepper: View {
  let height: CGFloat
  let increase: (() -> Void)?
  let decrease: (() -> Void)?

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  var body: some View {
    let vars = theme.vars

    VStack(spacing: 0) {
      StepButton(icon: .chevronUp12Regular, action: increase, help: "Increase")
      StepButton(icon: .chevronDown12Regular, action: decrease, help: "Decrease")
    }
    // Split rather than given a height each: a fixed one overflows the small
    // profile, whose box is barely two glyphs tall.
    .frame(height: max(0, height))
    .padding(.leading, vars.spacing1)
    .rule(vars.colorBorder, edge: .leading, width: hairline)
  }
}

private struct StepButton: View {
  let icon: FluentIcon
  let action: (() -> Void)?
  let help: String

  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars

    Pressable(
      action: action,
      enabled: action != nil,
      cornerRadius: vars.radiusTiny,
      help: help
    ) { state in
      Icon(icon, size: min(vars.iconSmall, vars.spacing3))
        .foregroundStyle(ink(state: state))
        .frame(width: vars.spacing4)
        .frame(maxHeight: .infinity)
        .background(wash(state: state))
        .clipShape(RoundedRectangle(cornerRadius: vars.radiusTiny, style: .circular))
        .animation(vars.motion, value: state.controlState)
    }
  }

  private func wash(state: PressableState) -> Color {
    guard state.enabled, state.hovered || state.pressed else { return .clear }
    return theme.vars.resolve(
      theme.vars.descriptor(.normal, .surface),
      tint: .neutral,
      state: .hovered
    )
  }

  private func ink(state: PressableState) -> Color {
    guard state.enabled else {
      return theme.vars.resolve(
        theme.vars.descriptor(.normal, .content),
        tint: .neutral,
        state: .disabled
      )
    }
    return state.hovered ? theme.vars.colorContent : theme.vars.colorContentSubtle
  }
}
