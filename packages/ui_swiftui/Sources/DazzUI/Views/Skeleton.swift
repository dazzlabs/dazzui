import SwiftUI

/// What a bar is standing in for.
///
/// `text` is a bar on the line box of the type it replaces; `circle` is an
/// avatar or a glyph, squared from its width alone; `rect` is everything with
/// a box of its own — a thumbnail, a chart, a card.
public enum SkeletonShape: String, CaseIterable, Sendable {
  case text
  case circle
  case rect
}

/// The shape of something that has not arrived.
///
/// A ``Spinner`` says the app is busy; a skeleton says *what* is coming, and
/// where it will be — which is why it belongs on the first load of a view whose
/// layout is already known, and a spinner belongs on an action whose result is
/// not. Drawing one for content that then lands somewhere else is worse than
/// drawing nothing.
///
/// The fill is `surface-sunken`, the same dead step a disabled control flattens
/// to, which is exactly what this is: a place where something will be, drawn as
/// though it were not there yet. The highlight sweeping over it is
/// `surface-subtle`, the ink wash the whole system already hovers with, so it
/// moves paper, card and column by the same amount without a per-theme value.
public struct Skeleton: View {
  public var shape: SkeletonShape
  public var width: CGFloat?
  public var height: CGFloat?

  @Environment(\.theme) private var theme
  @Environment(\.accessibilityReduceMotion) private var reduceMotion
  @State private var sweeping = false

  /// How long one pass of the highlight takes. Slower than the spinner's turn:
  /// this is scenery behind a layout, and at the spinner's pace a page of bars
  /// reads as an alarm.
  private static let sweep: Double = 1.6

  public init(
    shape: SkeletonShape = .text,
    width: CGFloat? = nil,
    height: CGFloat? = nil
  ) {
    self.shape = shape
    self.width = width
    self.height = height
  }

  public var body: some View {
    let vars = theme.vars

    RoundedRectangle(cornerRadius: corner, style: .circular)
      .fill(vars.colorSurfaceSunken)
      .overlay {
        // Stopped rather than slowed, where the spinner is slowed. A still
        // spinner reads as a hung app because motion is a spinner's only
        // message; a still skeleton still says everything it has to say.
        if !reduceMotion {
          GeometryReader { proxy in
            LinearGradient(
              colors: [.clear, vars.colorSurfaceSubtle, .clear],
              startPoint: .leading,
              endPoint: .trailing
            )
            // Twice the box, so the band starts off one edge and finishes off
            // the other rather than fading in and out in place.
            .frame(width: proxy.size.width * 2)
            .offset(x: sweeping ? proxy.size.width : -proxy.size.width * 2)
          }
        }
      }
      .clipShape(RoundedRectangle(cornerRadius: corner, style: .circular))
      .frame(width: resolvedWidth, height: resolvedHeight)
      .onAppear {
        guard !reduceMotion else { return }
        withAnimation(.linear(duration: Self.sweep).repeatForever(autoreverses: false)) {
          sweeping = true
        }
      }
      .accessibilityHidden(true)
  }

  private var corner: CGFloat {
    switch shape {
    case .text: return theme.vars.radiusTiny
    case .circle: return theme.vars.radiusFull
    case .rect: return theme.vars.radiusSmall
    }
  }

  private var resolvedWidth: CGFloat? { width }

  private var resolvedHeight: CGFloat? {
    switch shape {
    // The line box of body copy, so a bar occupies the run the sentence it
    // replaces would have.
    case .text: return theme.vars.bodySmall.lineHeight
    // Square from the width alone, so one measurement sizes it.
    case .circle: return width ?? height
    case .rect: return height
    }
  }
}

/// A paragraph's worth of bars.
///
/// The last one stops short because a full-width final line reads as a table
/// rather than as prose, and that ragged edge is most of what makes a block of
/// bars legible as text at all.
public struct SkeletonText: View {
  public var lines: Int

  @Environment(\.theme) private var theme

  /// Three fifths: short enough to read as the end of a paragraph at any
  /// measure, long enough not to read as a heading. A proportion rather than a
  /// length, so there is no step of the scale to compose it out of.
  private static let lastLine: CGFloat = 0.6

  public init(lines: Int = 3) {
    self.lines = lines
  }

  public var body: some View {
    // The leading between two lines of body copy rather than a step of the
    // spacing scale, so a stack of bars occupies the run the paragraph would
    // have.
    VStack(alignment: .leading, spacing: theme.vars.spacing15) {
      ForEach(0..<max(lines, 0), id: \.self) { line in
        if line == lines - 1 {
          GeometryReader { proxy in
            Skeleton(width: proxy.size.width * Self.lastLine)
          }
          .frame(height: theme.vars.bodySmall.lineHeight)
        } else {
          Skeleton()
        }
      }
    }
    .frame(maxWidth: .infinity, alignment: .leading)
  }
}
