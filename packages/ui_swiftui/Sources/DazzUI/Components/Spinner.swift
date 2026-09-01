import SwiftUI

/// Three diameters of the same ring. The stroke does not change with them —
/// a thicker ring at a larger size would read as a different component.
public enum SpinnerSize: String, CaseIterable, Sendable {
  case small
  case medium
  case large
}

/// A ring with one segment picked out.
///
/// The track is the tint at quarter strength, the head is the tint itself, and
/// the whole thing turns once a second — fast enough to read as work, slow
/// enough not to read as alarm.
public struct Spinner: View {
  public var size: SpinnerSize
  /// Which colour ramp the ring is drawn from.
  public var tint: Tint
  /// White ring for a spinner sitting on a filled control's own tint.
  public var onAccent: Bool
  /// Announced to assistive tech; pass your own copy to localise it.
  public var label: String

  @Environment(\.theme) private var theme
  @Environment(\.accessibilityReduceMotion) private var reduceMotion
  @State private var turning = false

  public init(
    size: SpinnerSize = .medium,
    tint: Tint = .primary,
    onAccent: Bool = false,
    label: String = "Loading"
  ) {
    self.size = size
    self.tint = tint
    self.onAccent = onAccent
    self.label = label
  }

  public var body: some View {
    let vars = theme.vars
    let thickness = vars.spacing05

    ZStack {
      Circle()
        .strokeBorder(track, lineWidth: thickness)
      Circle()
        .inset(by: thickness / 2)
        .trim(from: 0, to: headArc)
        .stroke(head, lineWidth: thickness)
        // A trimmed circle starts at three o'clock; the CSS picks out the
        // *top* border, so the head is swung back to straddle twelve.
        .rotationEffect(.degrees(headStart))
        .rotationEffect(.degrees(turning ? fullTurn : 0))
    }
    .frame(width: diameter, height: diameter)
    .onAppear {
      // Slowed rather than stopped: a still spinner reads as a frozen app,
      // which is the opposite of what it is there to say.
      withAnimation(
        .linear(duration: reduceMotion ? slowTurn : turn)
          .repeatForever(autoreverses: false)
      ) {
        turning = true
      }
    }
    .accessibilityElement()
    .accessibilityLabel(label)
  }

  /// A wash rather than a ramp step, so the track sits as correctly over a
  /// tinted chip or a dark canvas as over the paper. On a filled control the
  /// ring goes white instead — a tinted spinner on its own tint's fill is
  /// invisible.
  private var track: Color {
    onAccent
      ? theme.vars.colorOnAccent.opacity(onAccentTrackAlpha)
      : theme.vars.ramp(tint)[600].opacity(trackAlpha)
  }

  private var head: Color {
    onAccent ? theme.vars.colorOnAccent : theme.vars.ramp(tint)[600]
  }

  private var diameter: CGFloat {
    switch size {
    case .small: return theme.vars.spacing35
    case .medium: return theme.vars.spacing4
    case .large: return theme.vars.spacing5
    }
  }

  /// The CSS states these as inline alphas and as one border edge of four, so
  /// they stay ratios here rather than becoming lengths or tokens.
  private var trackAlpha: CGFloat { 0.25 }
  private var onAccentTrackAlpha: CGFloat { 0.35 }
  private var headArc: CGFloat { 0.25 }
  private var headStart: Double { -135 }
  private var fullTurn: Double { 360 }
  private var turn: Double { 1 }
  private var slowTurn: Double { 2.4 }
}
