import DazzUI
import SwiftUI

/// How wide a column may be, and how wide it starts.
struct ColumnWidth {
  let initial: CGFloat
  let min: CGFloat
  let max: CGFloat

  func clamp(_ width: CGFloat) -> CGFloat {
    Swift.min(Swift.max(width, min), max).rounded()
  }
}

/// The edge of a column, to be dragged. It draws nothing until the pointer
/// is on it — the rule is already there, drawn by the column — and then the
/// accent line; a double-click resets it.
struct Resizer: View {
  /// `1` when dragging right widens the column, `-1` when dragging left does.
  let outward: CGFloat
  let column: ColumnWidth
  @Binding var width: CGFloat
  let label: String
  /// Draws the line on the handle's leading edge rather than its middle.
  var lineAtStart = false

  @Environment(\.theme) private var theme
  @State private var hovered = false
  @State private var from: CGFloat?

  var body: some View {
    let vars = theme.vars
    ZStack(alignment: lineAtStart ? .leading : .center) {
      Color.clear.contentShape(Rectangle())
      Rectangle()
        .fill(vars.colorPrimary[500])
        .frame(width: vars.spacing05)
        .opacity(hovered || from != nil ? 1 : 0)
        .animation(vars.motion, value: hovered || from != nil)
    }
    .frame(width: vars.spacing2)
    .onHover { inside in
      hovered = inside
      if inside { NSCursor.resizeLeftRight.push() } else { NSCursor.pop() }
    }
    .gesture(
      DragGesture(minimumDistance: 0, coordinateSpace: .global)
        .onChanged { drag in
          let start = from ?? width
          from = start
          width = column.clamp(start + drag.translation.width * outward)
        }
        .onEnded { _ in from = nil }
    )
    .onTapGesture(count: 2) { width = column.initial }
    .accessibilityElement()
    .accessibilityLabel(label)
    .accessibilityValue("\(Int(width))")
    .accessibilityAdjustableAction { direction in
      switch direction {
      case .increment: width = column.clamp(width + 8)
      case .decrement: width = column.clamp(width - 8)
      @unknown default: break
      }
    }
  }
}
