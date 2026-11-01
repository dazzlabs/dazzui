import SwiftUI

/// Three weights of the same groove, each with its own thumb.
public enum SliderSize: String, CaseIterable, Sendable {
  case small
  case medium
  case large

  /// The shared size profile the row's height comes off, so a slider in a form
  /// sits on the same rhythm as the fields around it.
  var controlSize: ControlSize {
    switch self {
    case .small: return .small
    case .medium: return .medium
    case .large: return .large
    }
  }
}

/// A value picked along a range.
///
/// A range is the same view: pass more than one value and it draws a thumb per
/// value. There is no second view, because everything but the number of thumbs
/// is identical — and which thumb the pointer grabbed when the two sit on the
/// same value is the part a hand-rolled range never finishes.
public struct Slider: View {
  /// One entry per thumb, in order.
  @Binding public var values: [Double]
  public var minimum: Double
  public var maximum: Double
  /// What one notch of the pointer, and one arrow key, moves.
  public var step: Double
  public var size: SliderSize
  public var tint: Tint
  /// Printed at the trailing edge. A slider whose value the reader has to be
  /// able to say — a font size, a threshold — needs it; one they only have to
  /// feel — a volume — does not.
  public var valueLabel: String?
  public var enabled: Bool

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline
  /// Which thumb the pointer took hold of. A range whose thumbs sit on the
  /// same value is the case this exists for: without it every drag would move
  /// whichever one the array happens to start with.
  @State private var active = 0

  public init(
    values: Binding<[Double]>,
    minimum: Double = 0,
    maximum: Double = 100,
    step: Double = 1,
    size: SliderSize = .medium,
    tint: Tint = .primary,
    valueLabel: String? = nil,
    enabled: Bool = true
  ) {
    self._values = values
    self.minimum = minimum
    self.maximum = maximum
    self.step = step
    self.size = size
    self.tint = tint
    self.valueLabel = valueLabel
    self.enabled = enabled
  }

  public init(
    value: Binding<Double>,
    minimum: Double = 0,
    maximum: Double = 100,
    step: Double = 1,
    size: SliderSize = .medium,
    tint: Tint = .primary,
    valueLabel: String? = nil,
    enabled: Bool = true
  ) {
    self.init(
      values: Binding(
        get: { [value.wrappedValue] },
        set: { value.wrappedValue = $0.first ?? minimum }
      ),
      minimum: minimum,
      maximum: maximum,
      step: step,
      size: size,
      tint: tint,
      valueLabel: valueLabel,
      enabled: enabled
    )
  }

  public var body: some View {
    let vars = theme.vars

    HStack(spacing: vars.spacing3) {
      GeometryReader { geometry in
        let width = geometry.size.width

        ZStack(alignment: .leading) {
          // The groove, and the filled run between the thumbs. Flat tint
          // rather than the progress gradient: a gradient reads as travel, and
          // a slider is not going anywhere on its own.
          Capsule(style: .circular)
            .fill(vars.colorSurfaceSunken)
            .frame(height: track)
            .overlay(alignment: .leading) {
              Capsule(style: .circular)
                .fill(fill)
                .frame(width: max(0, width * (end - start)))
                .offset(x: width * start)
            }
          ForEach(values.indices, id: \.self) { index in
            thumbView
              // The thumb's centre sits on the control's edge at either end,
              // so half of it hangs outside the track's box.
              .offset(x: width * fraction(values[index]) - thumb / 2)
          }
        }
        .frame(height: geometry.size.height)
        .contentShape(Rectangle())
        .gesture(drag(width: width))
      }
      .frame(height: rowHeight)
      if let valueLabel {
        // A reading rather than a label: the mono face, so the row does not
        // reflow as the digits change under the pointer.
        Text(valueLabel)
          .font(.system(size: vars.labelSmall.size, design: .monospaced))
          .foregroundStyle(vars.colorContentMuted)
          .frame(minWidth: vars.spacing10, alignment: .trailing)
      }
    }
    .accessibilityElement()
    .accessibilityValue(valueLabel ?? "")
  }

  /// The raised control that sits on the groove: paper, a hairline edge and
  /// the small lift a `filled` button carries, so it reads as sitting on the
  /// track rather than in it.
  private var thumbView: some View {
    let vars = theme.vars

    return Circle()
      .fill(
        enabled
          ? vars.colorSurface
          : vars.resolve(vars.descriptor(.normal, .surface), tint: tint, state: .disabled)
      )
      .frame(width: thumb, height: thumb)
      .overlay(Circle().strokeBorder(edge, lineWidth: hairline))
      .tokenShadow(enabled ? vars.shadowXs : [.none])
  }

  private func drag(width: CGFloat) -> some Gesture {
    DragGesture(minimumDistance: 0)
      .onChanged { gesture in
        guard enabled, width > 0 else { return }
        let ratio = min(1, max(0, gesture.location.x / width))
        let raw = minimum + Double(ratio) * span
        let snapped = step > 0 ? minimum + (raw - minimum).rounded(toStep: step) : raw

        // The nearest thumb is chosen once, at the moment the press lands, so
        // a drag that crosses another thumb does not hand it over.
        if gesture.translation == .zero {
          active = nearest(to: snapped)
        }
        commit(snapped, at: active)
      }
  }

  private func commit(_ value: Double, at index: Int) {
    guard values.indices.contains(index) else { return }
    // A range's thumbs cannot pass each other, which is the least surprising
    // of the three behaviours a range can have.
    let low = index == 0 ? minimum : values[index - 1]
    let high = index == values.count - 1 ? maximum : values[index + 1]
    var next = values
    next[index] = min(high, max(low, value))
    if next != values { values = next }
  }

  private func nearest(to value: Double) -> Int {
    values.enumerated()
      .min { abs($0.element - value) < abs($1.element - value) }
      .map(\.offset) ?? 0
  }

  private var span: Double { maximum - minimum }

  private func fraction(_ value: Double) -> CGFloat {
    guard span > 0 else { return 0 }
    return CGFloat(min(1, max(0, (value - minimum) / span)))
  }

  private var start: CGFloat { values.count > 1 ? fraction(values.first ?? minimum) : 0 }
  private var end: CGFloat { fraction(values.last ?? minimum) }

  /// Disabled flattens to the groove, like every other control: no lift, no
  /// accent, one dead look.
  private var fill: Color {
    enabled
      ? theme.vars.ramp(tint)[600]
      : theme.vars.resolve(
        theme.vars.descriptor(.normal, .content),
        tint: tint,
        state: .disabled
      )
  }

  private var edge: Color {
    enabled ? theme.vars.colorBorderStrong : theme.vars.colorBorder
  }

  private var track: CGFloat {
    switch size {
    case .small: return theme.vars.sliderSmallTrack
    case .medium: return theme.vars.sliderMediumTrack
    case .large: return theme.vars.sliderLargeTrack
    }
  }

  private var thumb: CGFloat {
    switch size {
    case .small: return theme.vars.sliderSmallThumb
    case .medium: return theme.vars.sliderMediumThumb
    case .large: return theme.vars.sliderLargeThumb
    }
  }

  private var rowHeight: CGFloat { theme.vars.metrics(size.controlSize).size }
}

private extension Double {
  /// Snapped to a multiple of the step, which is what makes a discrete slider
  /// land on its own values rather than between them.
  func rounded(toStep step: Double) -> Double {
    (self / step).rounded() * step
  }
}
