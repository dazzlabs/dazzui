import SwiftUI

/// Where the free space in the row goes.
public enum ActionBarAlign: String, CaseIterable, Sendable {
  case start
  /// Pushes the row to the trailing edge — a dialog footer's usual shape.
  case end
  case between
}

/// A row of controls kept on one rhythm.
///
/// It owns no colour and no box — only the rhythm — so it takes the shared
/// control gap rather than a token of its own: the small profile's, whatever
/// the children's size, since the design draws every action row on that one
/// step.
public struct ActionBar<Content: View>: View {
  public var align: ActionBarAlign
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme

  public init(
    align: ActionBarAlign = .start,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.align = align
    self.content = content
  }

  public var body: some View {
    ActionBarLayout(gap: theme.vars.controlSmallGap, align: align) {
      content()
    }
  }
}

/// `justify-content` is a distribution rather than an alignment, and `between`
/// is the case an `HStack` cannot express: it hands the slack to the gaps
/// *between* the children, which no single alignment or interposed spacer
/// reproduces for an arbitrary number of them. So the row is laid out
/// directly.
private struct ActionBarLayout: Layout {
  let gap: CGFloat
  let align: ActionBarAlign

  func sizeThatFits(
    proposal: ProposedViewSize,
    subviews: Subviews,
    cache: inout Void
  ) -> CGSize {
    let sizes = subviews.map { $0.sizeThatFits(.unspecified) }
    return CGSize(
      // A block-level flex row fills its parent; only its height is its own.
      width: proposal.width ?? intrinsicWidth(of: sizes),
      height: sizes.map(\.height).max() ?? 0
    )
  }

  func placeSubviews(
    in bounds: CGRect,
    proposal: ProposedViewSize,
    subviews: Subviews,
    cache: inout Void
  ) {
    let sizes = subviews.map { $0.sizeThatFits(.unspecified) }
    let free = max(0, bounds.width - intrinsicWidth(of: sizes))

    var x = bounds.minX
    var spread: CGFloat = 0
    switch align {
    case .start:
      break
    case .end:
      x += free
    case .between:
      // A lone child under `space-between` sits at the start, not the end.
      if subviews.count > 1 { spread = free / CGFloat(subviews.count - 1) }
    }

    for (index, subview) in subviews.enumerated() {
      subview.place(
        at: CGPoint(x: x, y: bounds.midY),
        anchor: .leading,
        proposal: ProposedViewSize(sizes[index])
      )
      x += sizes[index].width + gap + spread
    }
  }

  private func intrinsicWidth(of sizes: [CGSize]) -> CGFloat {
    sizes.reduce(0) { $0 + $1.width } + gap * CGFloat(max(0, sizes.count - 1))
  }
}
