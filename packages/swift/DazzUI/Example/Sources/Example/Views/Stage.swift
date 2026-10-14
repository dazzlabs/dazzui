import DazzUI
import SwiftUI

/*
 * The four stage components a story arranges with — Row, Col, Frame, Text —
 * read the way `example/src/components/stage.tsx` draws them on the web.
 */

/// A spacing step as the web writes it (`"3"`, `"0-5"`), in this theme.
func spacing(_ vars: ThemeVariables, _ step: String?) -> CGFloat {
  switch step {
  case nil, "3": return vars.spacing3
  case "0": return 0
  case "px": return 1
  case "0-5": return vars.spacing05
  case "1": return vars.spacing1
  case "1-5": return vars.spacing15
  case "2": return vars.spacing2
  case "2-5": return vars.spacing25
  case "3-5": return vars.spacing35
  case "4": return vars.spacing4
  case "5": return vars.spacing5
  case "6": return vars.spacing6
  case "7": return vars.spacing7
  case "8": return vars.spacing8
  case "9": return vars.spacing9
  case "10": return vars.spacing10
  case "12": return vars.spacing12
  case "14": return vars.spacing14
  case "16": return vars.spacing16
  case "20": return vars.spacing20
  default: fatalError("No spacing step \(step ?? "")")
  }
}

/// A length as the web writes it: points, `rem` against the 16pt root every
/// port shares, or a share of the room there is.
enum Length {
  case points(CGFloat)
  case fraction(CGFloat)

  init?(_ value: Value) {
    switch value {
    case .number(let number):
      self = .points(number)
    case .string(let text) where text.hasSuffix("rem"):
      self = .points((Double(text.dropLast(3)) ?? 0) * 16)
    case .string(let text) where text.hasSuffix("px"):
      self = .points(Double(text.dropLast(2)) ?? 0)
    case .string(let text) where text.hasSuffix("%"):
      self = .fraction((Double(text.dropLast()) ?? 0) / 100)
    case .string(let text):
      guard let number = Double(text) else { return nil }
      self = .points(number)
    default:
      return nil
    }
  }

  var points: CGFloat? {
    if case .points(let points) = self { return points }
    return nil
  }
}

/// A child at a width — the web's `width` with `max-width: 100%`: the width
/// asked for, never more than the room it is offered.
struct Sized: Layout {
  var width: Length?
  var height: Length?

  private func resolve(_ length: Length?, _ offered: CGFloat?) -> CGFloat? {
    switch length {
    case nil: return nil
    case .points(let points): return offered.map { min(points, $0) } ?? points
    case .fraction(let share): return offered.map { $0 * share }
    }
  }

  func sizeThatFits(proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) -> CGSize {
    let width = resolve(self.width, proposal.width)
    let height = resolve(self.height, proposal.height)
    let child = subviews.first?.sizeThatFits(ProposedViewSize(width: width ?? proposal.width, height: height ?? proposal.height)) ?? .zero
    return CGSize(width: width ?? child.width, height: height ?? child.height)
  }

  func placeSubviews(in bounds: CGRect, proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) {
    subviews.first?.place(
      at: bounds.origin,
      proposal: ProposedViewSize(width: bounds.width, height: height == nil ? nil : bounds.height)
    )
  }
}

/// Children stacked, all as wide as the widest of them, which is as wide as
/// its content wants and never wider than the room: CSS's shrink-to-fit
/// width, `min(max-content, available)`, with `align-items: stretch`.
struct ShrinkToFit: Layout {
  var spacing: CGFloat

  private func width(_ proposal: ProposedViewSize, _ subviews: Subviews) -> CGFloat {
    let wanted = subviews.map { $0.sizeThatFits(.unspecified).width }.max() ?? 0
    return proposal.width.map { min(wanted, $0) } ?? wanted
  }

  func sizeThatFits(proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) -> CGSize {
    let width = width(proposal, subviews)
    var height: CGFloat = 0
    for subview in subviews {
      height += subview.sizeThatFits(ProposedViewSize(width: width, height: nil)).height
    }
    height += spacing * CGFloat(max(0, subviews.count - 1))
    return CGSize(width: width, height: height)
  }

  func placeSubviews(in bounds: CGRect, proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) {
    var y = bounds.minY
    for subview in subviews {
      let offer = ProposedViewSize(width: bounds.width, height: nil)
      let height = subview.sizeThatFits(offer).height
      subview.place(at: CGPoint(x: bounds.minX, y: y), proposal: ProposedViewSize(width: bounds.width, height: height))
      y += height + spacing
    }
  }
}

/// Children side by side, wrapping onto the next line when the row runs
/// out — the web's `flex-wrap: wrap`.
struct Flow: Layout {
  var spacing: CGFloat
  var alignment: VerticalAlignment

  private func lines(_ width: CGFloat?, _ subviews: Subviews) -> [[(Int, CGSize)]] {
    var lines: [[(Int, CGSize)]] = [[]]
    var x: CGFloat = 0
    for (index, subview) in subviews.enumerated() {
      let size = subview.sizeThatFits(.unspecified)
      if let width, x > 0, x + size.width > width {
        lines.append([])
        x = 0
      }
      lines[lines.count - 1].append((index, size))
      x += size.width + spacing
    }
    return lines
  }

  func sizeThatFits(proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) -> CGSize {
    let lines = lines(proposal.width, subviews)
    var width: CGFloat = 0
    var height: CGFloat = 0
    for line in lines {
      var lineWidth: CGFloat = 0
      var lineHeight: CGFloat = 0
      for (_, size) in line {
        lineWidth += size.width
        lineHeight = max(lineHeight, size.height)
      }
      lineWidth += spacing * CGFloat(max(0, line.count - 1))
      width = max(width, lineWidth)
      height += lineHeight
    }
    height += spacing * CGFloat(max(0, lines.count - 1))
    return CGSize(width: width, height: height)
  }

  func placeSubviews(in bounds: CGRect, proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) {
    var y = bounds.minY
    for line in lines(bounds.width, subviews) {
      let height = line.map(\.1.height).max() ?? 0
      var x = bounds.minX
      for (index, size) in line {
        let offset: CGFloat = switch alignment {
        case .top: 0
        case .bottom: height - size.height
        default: (height - size.height) / 2
        }
        subviews[index].place(at: CGPoint(x: x, y: y + offset), proposal: ProposedViewSize(size))
        x += size.width + spacing
      }
      y += height + spacing
    }
  }
}

private func vertical(_ align: String?) -> VerticalAlignment {
  switch align {
  case "start": return .top
  case "end": return .bottom
  default: return .center
  }
}

private func horizontal(_ align: String?) -> HorizontalAlignment {
  switch align {
  case "center": return .center
  case "end": return .trailing
  default: return .leading
  }
}

@MainActor
func stageRow(_ node: Node) -> AnyView {
  AnyView(StageRow(node: node))
}

private struct StageRow: View {
  let node: Node
  @Environment(\.theme) private var theme

  var body: some View {
    let gap = spacing(theme.vars, node.string("gap"))
    let align = node.string("align")
    Group {
      if node.string("justify") == "between" {
        HStack(alignment: vertical(align), spacing: gap) {
          ForEach(Array(node.children.enumerated()), id: \.offset) { index, child in
            if index > 0 { Spacer(minLength: 0) }
            NodeView(child)
          }
        }
      } else if align == "stretch" {
        // Every child as tall as the tallest, as a flex row stretches them.
        HStack(alignment: .top, spacing: gap) {
          ForEach(Array(node.children.enumerated()), id: \.offset) { _, child in
            NodeView(child).frame(maxHeight: .infinity, alignment: .top)
          }
        }
        .fixedSize(horizontal: false, vertical: true)
      } else {
        Flow(spacing: gap, alignment: vertical(align)) { node.kids }
      }
    }
    .layout(width: node["width"])
  }
}

@MainActor
func stageCol(_ node: Node) -> AnyView {
  AnyView(StageCol(node: node))
}

private struct StageCol: View {
  let node: Node
  @Environment(\.theme) private var theme

  var body: some View {
    let gap = spacing(theme.vars, node.string("gap"))
    let align = node.string("align")
    let stretch = align == nil || align == "stretch"
    let column = VStack(alignment: horizontal(align), spacing: gap) {
      ForEach(Array(node.children.enumerated()), id: \.offset) { _, child in
        if stretch {
          NodeView(child).frame(maxWidth: .infinity, alignment: .leading)
        } else {
          NodeView(child)
        }
      }
    }
    if node.has("width") {
      column.layout(width: node["width"])
    } else if stretch {
      // Without a width a stretched column is as wide as its widest child
      // — but never wider than the room — the way a shrink-to-fit flex box
      // is on the web.
      ShrinkToFit(spacing: gap) {
        ForEach(Array(node.children.enumerated()), id: \.offset) { _, child in
          NodeView(child)
        }
      }
    } else {
      column
    }
  }
}

@MainActor
func stageFrame(_ node: Node) -> AnyView {
  AnyView(StageFrame(node: node))
}

private struct StageFrame: View {
  let node: Node
  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  var body: some View {
    let vars = theme.vars
    let shape = RoundedRectangle(cornerRadius: vars.radiusLarge, style: .circular)
    node.kids
      .frame(maxWidth: .infinity, maxHeight: .infinity)
      // The border is inside the box, as CSS draws it, and the content
      // inside the border.
      .padding(hairline)
      .background(vars.colorSurface)
      .clipShape(shape)
      .overlay(shape.strokeBorder(vars.colorBorder, lineWidth: hairline))
      .layout(
        width: node.has("width") ? node["width"] : .string("36rem"),
        height: node.has("height") ? node["height"] : .string("16rem")
      )
  }
}

/// A run of text that may hold components inline, as a paragraph would.
@MainActor
func stageText(_ node: Node) -> AnyView {
  AnyView(StageText(node: node))
}

private struct StageText: View {
  let node: Node
  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars
    let color: Color? =
      if let tint = node.string("tint").flatMap(Tint.init(rawValue:)) {
        vars.ramp(tint)[500]
      } else if node.string("tone") == "secondary" {
        vars.colorContentSecondary
      } else {
        nil
      }
    let run = HStack(alignment: .firstTextBaseline, spacing: 0) {
      ForEach(Array(node.children.enumerated()), id: \.offset) { _, child in
        if let text = child.string {
          Text(text)
        } else if isLive(child) {
          Text(node.context.resolve(child).text)
        } else {
          NodeView(child)
        }
      }
    }
    .fontWeight(node.flag("strong") ? vars.labelStrong.weight : nil)
    .lineBox(vars.bodyMedium)
    if let color {
      run.foregroundStyle(color)
    } else {
      run
    }
  }
}

extension View {
  /// This view at a width and height written as the web writes them.
  func layout(width: Value = .null, height: Value = .null) -> some View {
    Sized(width: Length(width), height: Length(height)) { self }
  }
}
