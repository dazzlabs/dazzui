import DazzUI
import SwiftUI

/// The arrangements a bare specimen needs — the counterpart of the Storybook's
/// `components/stage.tsx`.
///
/// Only a component's `Overview` is a documented page; every other story
/// draws the component and nothing else, so all it ever needs is a row or a
/// column with a gap in it, and a width where the component has none of its
/// own. Nothing here draws.

/// A row that wraps rather than clipping, which is what a shelf of variants
/// wants at any window width.
struct Row<Content: View>: View {
  var spacing: CGFloat?
  var alignment: VerticalAlignment = .center
  @ViewBuilder let content: () -> Content

  @Environment(\.theme) private var theme

  init(
    spacing: CGFloat? = nil,
    alignment: VerticalAlignment = .center,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.spacing = spacing
    self.alignment = alignment
    self.content = content
  }

  var body: some View {
    Flow(spacing: spacing ?? theme.vars.spacing3, alignment: alignment) {
      content()
    }
  }
}

/// A row that wraps, which `HStack` cannot do.
///
/// Six tints beside each other are wider than most canvases, and a clipped
/// shelf hides exactly the variant a reader came to see. So the row is laid
/// out directly: fill a line until the next child would not fit, then start
/// another.
struct Flow: Layout {
  var spacing: CGFloat
  var lineSpacing: CGFloat?
  var alignment: VerticalAlignment = .center

  func sizeThatFits(
    proposal: ProposedViewSize,
    subviews: Subviews,
    cache: inout Void
  ) -> CGSize {
    let lines = lines(of: subviews, within: proposal.width ?? .infinity)
    let width = lines.map(\.width).max() ?? 0
    let height =
      lines.map(\.height).reduce(0, +)
      + gapBetweenLines * CGFloat(max(0, lines.count - 1))
    return CGSize(width: width, height: height)
  }

  func placeSubviews(
    in bounds: CGRect,
    proposal: ProposedViewSize,
    subviews: Subviews,
    cache: inout Void
  ) {
    var y = bounds.minY

    for line in lines(of: subviews, within: bounds.width) {
      var x = bounds.minX
      for item in line.items {
        let size = subviews[item].sizeThatFits(.unspecified)
        subviews[item].place(
          at: CGPoint(x: x, y: y + centreOffset(of: size.height, in: line.height)),
          anchor: .topLeading,
          proposal: ProposedViewSize(size)
        )
        x += size.width + spacing
      }
      y += line.height + gapBetweenLines
    }
  }

  private var gapBetweenLines: CGFloat { lineSpacing ?? spacing }

  private func centreOffset(of height: CGFloat, in line: CGFloat) -> CGFloat {
    alignment == .center ? (line - height) / 2 : 0
  }

  private struct Line {
    var items: [LayoutSubviews.Index] = []
    var width: CGFloat = 0
    var height: CGFloat = 0
  }

  private func lines(of subviews: Subviews, within maxWidth: CGFloat) -> [Line] {
    var lines: [Line] = []
    var line = Line()

    for index in subviews.indices {
      let size = subviews[index].sizeThatFits(.unspecified)
      let advance = line.items.isEmpty ? size.width : size.width + spacing

      if !line.items.isEmpty, line.width + advance > maxWidth {
        lines.append(line)
        line = Line()
      }

      line.items.append(index)
      line.width += line.items.count == 1 ? size.width : advance
      line.height = max(line.height, size.height)
    }

    if !line.items.isEmpty { lines.append(line) }
    return lines
  }
}

/// A column, for the components that stack.
struct Col<Content: View>: View {
  var spacing: CGFloat?
  var alignment: HorizontalAlignment = .leading
  var width: CGFloat?
  @ViewBuilder let content: () -> Content

  @Environment(\.theme) private var theme

  init(
    spacing: CGFloat? = nil,
    alignment: HorizontalAlignment = .leading,
    width: CGFloat? = nil,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.spacing = spacing
    self.alignment = alignment
    self.width = width
    self.content = content
  }

  var body: some View {
    VStack(alignment: alignment, spacing: spacing ?? theme.vars.spacing3) {
      content()
    }
    .frame(width: width)
  }
}

/// A window, for the columns that only make sense with a pane beside them.
///
/// It is a frame rather than a shell: the corner and the border are the
/// design's own window metrics, but nothing here draws a titlebar, because
/// the real one belongs to the app.
struct Frame<Content: View>: View {
  var width: CGFloat = 620
  var height: CGFloat = 340
  @ViewBuilder let content: () -> Content

  @Environment(\.theme) private var theme

  init(
    width: CGFloat = 620,
    height: CGFloat = 340,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.width = width
    self.height = height
    self.content = content
  }

  var body: some View {
    let vars = theme.vars

    HStack(spacing: 0) {
      content()
    }
    .frame(width: width, height: height)
    .background(vars.colorSurface)
    .clipShape(
      RoundedRectangle(cornerRadius: vars.frameWindowRadius, style: .continuous)
    )
    .border(
      vars.colorBorder,
      width: vars.spacingPx,
      cornerRadius: vars.frameWindowRadius
    )
  }
}

/// A vertical rule between two runs of controls.
///
/// CSS gives the React one its height from the row it stretches in. A
/// `Layout` proposes each child its ideal size instead, so a rule in a
/// toolbar has to be told how tall the row is — the height of the controls
/// beside it.
struct ToolbarSeparator: View {
  var size: DazzUI.ControlSize = .tiny

  @Environment(\.theme) private var theme

  var body: some View {
    DazzUI.Divider(orientation: .vertical)
      .frame(height: theme.vars.metrics(size).size)
  }
}

/// The empty pane a navigation column is a navigation column to.
struct Pane<Content: View>: View {
  @ViewBuilder let content: () -> Content

  @Environment(\.theme) private var theme

  init(@ViewBuilder content: @escaping () -> Content = { EmptyView() }) {
    self.content = content
  }

  var body: some View {
    VStack(alignment: .leading, spacing: theme.vars.spacing3) {
      content()
    }
    .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
    .padding(theme.vars.spacing5)
  }
}
