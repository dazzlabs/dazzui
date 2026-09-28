import SwiftUI

/// Which way the panels are laid out, not which way a handle runs.
public enum SplitterOrientation: String, CaseIterable, Sendable {
  case horizontal
  case vertical
}

/// One region of a ``Splitter``.
///
/// A description as much as a view: the splitter counts these to know how many
/// handles to draw and reads their constraints to know what each handle may do.
public struct SplitterPanel: Identifiable {
  public let id = UUID()
  /// Share of the run at rest. The shares are normalised, so three panels
  /// written `50`, `25`, `25` and three written `2`, `1`, `1` lay out
  /// identically.
  public var defaultSize: CGFloat
  /// Floor and ceiling, as percentages of the whole run.
  public var minSize: CGFloat
  public var maxSize: CGFloat
  public var content: AnyView

  public init<Content: View>(
    defaultSize: CGFloat = 1,
    minSize: CGFloat = 0,
    maxSize: CGFloat = 100,
    @ViewBuilder content: () -> Content
  ) {
    self.defaultSize = defaultSize
    self.minSize = minSize
    self.maxSize = maxSize
    self.content = AnyView(content())
  }
}

/// Two or more regions the reader can re-proportion.
///
/// A desktop window is mostly this — a column beside a document beside an
/// inspector — and which of the three deserves the room is the reader's
/// decision rather than the designer's. Where a column has one right width and
/// the reader only opens or shuts it, that is ``Sidebar`` instead: it carries
/// its own width token and its own rail, and drawing it with this would offer a
/// choice the design has already made.
///
/// The handle is drawn *from* the space between two panels rather than added to
/// it: the hit target is a whole step of the scale, so grabbing a seam does not
/// demand pixel accuracy, while the rule itself stays the hairline every other
/// separator in the system is.
///
/// The splitter owns the shares; ``onSizesChange`` is for persisting them. A
/// drag that had to round-trip through a host's state would lag the pointer,
/// and a seam that lags the pointer feels broken.
public struct Splitter: View {
  public var panels: [SplitterPanel]
  public var orientation: SplitterOrientation
  public var onSizesChange: (([CGFloat]) -> Void)?

  @Environment(\.theme) private var theme
  @State private var shares: [CGFloat] = []

  public init(
    orientation: SplitterOrientation = .horizontal,
    panels: [SplitterPanel],
    onSizesChange: (([CGFloat]) -> Void)? = nil
  ) {
    self.orientation = orientation
    self.panels = panels
    self.onSizesChange = onSizesChange
  }

  private var horizontal: Bool { orientation == .horizontal }

  public var body: some View {
    let vars = theme.vars
    let target = vars.spacing2
    let seams = max(panels.count - 1, 0)

    GeometryReader { proxy in
      // The handles take their width off the top and the panels divide what is
      // left, so a share is a share of the panels' run rather than of the box.
      let whole = horizontal ? proxy.size.width : proxy.size.height
      let run = max(whole - CGFloat(seams) * target, 1)
      let current = shares.isEmpty ? normalised() : shares

      layout {
        ForEach(Array(panels.enumerated()), id: \.element.id) { index, panel in
          if index > 0 {
            Seam(
              orientation: orientation,
              target: target,
              onDrag: { delta in resize(index - 1, delta: delta / run * 100) },
              onReset: {
                let pair = current[index - 1] + current[index]
                set(index - 1, leading: pair / 2)
              }
            )
          }
          panel.content
            .frame(
              width: horizontal ? run * current[index] / 100 : nil,
              height: horizontal ? nil : run * current[index] / 100
            )
            .frame(maxWidth: horizontal ? nil : .infinity,
                   maxHeight: horizontal ? .infinity : nil)
            .clipped()
        }
      }
    }
    .onAppear { if shares.isEmpty { shares = normalised() } }
  }

  @ViewBuilder
  private func layout<Content: View>(@ViewBuilder content: () -> Content) -> some View {
    if horizontal {
      HStack(spacing: 0) { content() }
    } else {
      VStack(spacing: 0) { content() }
    }
  }

  /// Normalised to a hundred once, here, so everything downstream — the clamps,
  /// what a seam reports, what a host is handed to persist — speaks one unit
  /// rather than whatever units the panels were written in.
  private func normalised() -> [CGFloat] {
    let sum = panels.reduce(0) { $0 + $1.defaultSize }
    guard sum > 0 else { return panels.map { _ in 100 / CGFloat(max(panels.count, 1)) } }
    return panels.map { $0.defaultSize / sum * 100 }
  }

  private func resize(_ index: Int, delta: CGFloat) {
    let current = shares.isEmpty ? normalised() : shares
    set(index, leading: current[index] + delta)
  }

  /// Given the leading panel's new share, the trailing one takes what the pair
  /// has left over. Nothing else in the row moves, which is what stops a drag
  /// at one seam from rippling through the whole window.
  private func set(_ index: Int, leading wanted: CGFloat) {
    var next = shares.isEmpty ? normalised() : shares
    guard index + 1 < next.count else { return }
    let pair = next[index] + next[index + 1]
    let lead = panels[index]
    let trail = panels[index + 1]
    let leading = min(
      max(min(max(wanted, lead.minSize), lead.maxSize), pair - trail.maxSize),
      pair - trail.minSize
    )
    next[index] = leading
    next[index + 1] = pair - leading
    shares = next
    onSizesChange?(next)
  }
}

/// The seam: a hit target with a rule inside it.
///
/// Under the pointer the rule takes the accent and thickens, which is the whole
/// affordance: a hairline that never answers is indistinguishable from a
/// border, and a reader has no way to learn that this one can be dragged. It
/// swells rather than widens, so the panels beside it never move on the way to
/// a drag.
private struct Seam: View {
  let orientation: SplitterOrientation
  let target: CGFloat
  let onDrag: (CGFloat) -> Void
  let onReset: () -> Void

  @Environment(\.theme) private var theme
  @State private var hovered = false
  @State private var dragging = false
  @State private var last: CGFloat = 0

  /// How much wider the rule is drawn while the pointer is on it.
  private static let swell: CGFloat = 2

  private var horizontal: Bool { orientation == .horizontal }

  var body: some View {
    let vars = theme.vars
    let lit = hovered || dragging
    let ink = lit
      ? vars.resolve(vars.descriptor(.filled, .surface), tint: .primary, state: .normal)
      : vars.colorBorder
    let weight = vars.strokeHairline * (lit ? Self.swell : 1)

    Rectangle()
      .fill(ink)
      .frame(
        width: horizontal ? weight : nil,
        height: horizontal ? nil : weight
      )
      .frame(
        width: horizontal ? target : nil,
        height: horizontal ? nil : target
      )
      .frame(maxWidth: horizontal ? nil : .infinity,
             maxHeight: horizontal ? .infinity : nil)
      .contentShape(Rectangle())
      .onHover { inside in
        hovered = inside
        #if os(macOS)
          // Pushed and popped in the same handler, so the stack stays balanced
          // even when the pointer leaves during a drag.
          if inside {
            (horizontal ? NSCursor.resizeLeftRight : NSCursor.resizeUpDown).push()
          } else {
            NSCursor.pop()
          }
        #endif
      }
      .gesture(
        DragGesture(minimumDistance: 0)
          .onChanged { value in
            let at = horizontal ? value.translation.width : value.translation.height
            if !dragging {
              dragging = true
              last = 0
            }
            onDrag(at - last)
            last = at
          }
          .onEnded { _ in
            dragging = false
            last = 0
          }
      )
      // An even split of this pair alone, which is what a reader means by
      // double-clicking one seam.
      .onTapGesture(count: 2, perform: onReset)
      .animation(vars.motion, value: lit)
  }
}
