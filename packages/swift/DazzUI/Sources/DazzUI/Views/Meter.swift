import SwiftUI

/// Three weights of the same rule, the way a progress bar has three.
public enum MeterSize: String, CaseIterable, Sendable {
  case small
  case medium
  case large
}

/// A reading inside a known range — disk used of disk there, spend against a
/// budget, an API quota.
///
/// It is not a ``Progress``, and the difference is not how they look. Progress
/// reports on something running: it moves on its own, it can be indeterminate,
/// and it ends. A meter measures something that already has a value, so it
/// never animates and never has none — which is why it prints its number and
/// progress does not.
public struct Meter: View {
  /// The reading, between `minimum` and `maximum`.
  public var value: Double
  public var minimum: Double
  public var maximum: Double
  public var size: MeterSize
  public var tint: Tint
  /// Names the reading. It is the meter's accessible label as well.
  public var label: String?
  /// What is printed beside the label. The number is the meter's whole point,
  /// so most readings want one.
  public var valueLabel: String?
  public var showValue: Bool

  @Environment(\.theme) private var theme

  public init(
    value: Double,
    minimum: Double = 0,
    maximum: Double = 100,
    size: MeterSize = .medium,
    tint: Tint = .primary,
    label: String? = nil,
    valueLabel: String? = nil,
    showValue: Bool = false
  ) {
    self.value = value
    self.minimum = minimum
    self.maximum = maximum
    self.size = size
    self.tint = tint
    self.label = label
    self.valueLabel = valueLabel
    self.showValue = showValue
  }

  public var body: some View {
    let vars = theme.vars
    let reading = valueLabel ?? (showValue ? "\(Int((fraction * 100).rounded()))%" : nil)

    VStack(alignment: .leading, spacing: vars.spacing2) {
      if label != nil || reading != nil {
        HStack(alignment: .firstTextBaseline, spacing: vars.spacing3) {
          if let label {
            Text(label)
              .font(.system(size: vars.labelSmall.size, weight: vars.labelMedium.weight))
              .lineBox(vars.labelSmall, lineHeight: vars.labelSmall.size)
              .foregroundStyle(vars.colorContent)
              .frame(maxWidth: .infinity, alignment: .leading)
          }
          if let reading {
            // A reading rather than a label: the mono face, so a column of
            // meters keeps its numbers on one edge.
            Text(reading)
              .font(.system(size: vars.labelSmall.size, design: .monospaced))
              .lineBox(vars.labelSmall, lineHeight: vars.labelSmall.size)
              .foregroundStyle(vars.colorContentMuted)
          }
        }
      }
      // The same cut a Progress bar is — the sunken groove, the soft square
      // corner, the hard-clipped fill. The fill has no transition on purpose:
      // a progress bar interpolates because the thing behind it is running; a
      // meter that eased into place would be animating a fact.
      GeometryReader { geometry in
        vars.ramp(tint)[600]
          .frame(width: geometry.size.width * fraction)
          .frame(maxWidth: .infinity, alignment: .leading)
      }
      .frame(height: thickness)
      .background(vars.colorSurfaceSunken)
      .clipShape(RoundedRectangle(cornerRadius: thickness / 2, style: .circular))
    }
    .accessibilityElement(children: .ignore)
    .accessibilityLabel(label ?? "")
    .accessibilityValue(reading ?? "")
  }

  private var fraction: Double {
    let span = maximum - minimum
    guard span > 0 else { return 0 }
    return min(1, max(0, (value - minimum) / span))
  }

  private var thickness: CGFloat {
    switch size {
    case .small: return theme.vars.spacing1
    case .medium: return theme.vars.spacing15
    case .large: return theme.vars.spacing2
    }
  }
}
