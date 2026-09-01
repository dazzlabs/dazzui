import SwiftUI

/// Three weights of the same rule. A progress bar has no type, so a size is
/// only its thickness — and its corner, which is half of that.
public enum ProgressSize: String, CaseIterable, Sendable {
  case small
  case medium
  case large
}

/// A rule cut into the surface, not a capsule.
///
/// The corner is a soft square and the fill's leading edge is a hard clip
/// against it — a pill's rounded head would read as a thumb.
public struct Progress: View {
  /// 0–100. Omit for an indeterminate bar.
  public var value: Double?
  public var size: ProgressSize
  /// Which colour ramp the flat fill is drawn from.
  public var tint: Tint
  /// Paint the fill with the theme's two-stop accent ramp instead of a flat
  /// tint.
  public var gradient: Bool

  @Environment(\.theme) private var theme
  @Environment(\.accessibilityReduceMotion) private var reduceMotion

  public init(
    value: Double? = nil,
    size: ProgressSize = .small,
    tint: Tint = .primary,
    gradient: Bool = false
  ) {
    self.value = value
    self.size = size
    self.tint = tint
    self.gradient = gradient
  }

  public var body: some View {
    let vars = theme.vars

    GeometryReader { geometry in
      if let clamped {
        fill
          .frame(width: geometry.size.width * clamped / 100)
          .frame(maxWidth: .infinity, alignment: .leading)
          .animation(travel, value: clamped)
      } else {
        Sweep(reduceMotion: reduceMotion, track: geometry.size.width) { fill }
      }
    }
    .frame(height: thickness)
    .background(vars.colorSurfaceSunken)
    .clipShape(RoundedRectangle(cornerRadius: thickness / 2, style: .continuous))
    .accessibilityElement()
    .accessibilityValue(clamped.map { Text("\(Int($0))%") } ?? Text(""))
  }

  @ViewBuilder private var fill: some View {
    if gradient {
      // The document-progress fill: the theme's two-stop accent ramp.
      LinearGradient(
        colors: [theme.vars.progressGradientFrom, theme.vars.progressGradientTo],
        startPoint: .leading,
        endPoint: .trailing
      )
    } else {
      theme.vars.ramp(tint)[600]
    }
  }

  private var clamped: Double? {
    value.map { min(100, max(0, $0)) }
  }

  /// The width transition the CSS states inline — its own curve rather than
  /// the system's colour crossfade, because a bar that eased like a hover
  /// would look like it was correcting itself.
  private var travel: Animation? {
    reduceMotion ? nil : .timingCurve(0.25, 0.1, 0.25, 1, duration: 0.3)
  }

  private var thickness: CGFloat {
    switch size {
    case .small: return theme.vars.spacing1
    case .medium: return theme.vars.spacing15
    case .large: return theme.vars.spacing2
    }
  }
}

/// An indeterminate bar has no width to animate, so it sweeps instead — a
/// short head crossing the whole track and starting over.
private struct Sweep<Content: View>: View {
  let reduceMotion: Bool
  let track: CGFloat
  @ViewBuilder let content: () -> Content

  @State private var sweeping = false

  private var headWidth: CGFloat { track * headFraction }

  var body: some View {
    content()
      .frame(width: headWidth)
      .frame(maxWidth: .infinity, alignment: .leading)
      .offset(x: sweeping ? headWidth * farEdge : -headWidth)
      .animation(
        // Slowed rather than stopped: a still bar reads as a frozen app, which
        // is the opposite of what it is there to say.
        .easeInOut(duration: reduceMotion ? slowTurn : turn)
          .repeatForever(autoreverses: false),
        value: sweeping
      )
      .onAppear { sweeping = true }
  }

  /// The CSS states the head as a fraction of the track and its travel as
  /// multiples of its own width, so both stay ratios here.
  private var headFraction: CGFloat { 0.4 }
  private var farEdge: CGFloat { 2.5 }
  private var turn: Double { 1.2 }
  private var slowTurn: Double { 3 }
}
