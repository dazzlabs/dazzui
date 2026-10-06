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
///
/// A panel is known by its place in the row unless it is given a `key`, so the
/// view it holds keeps its identity — and its state, and whatever AppKit view
/// sits inside it — across every redraw of the splitter's host. A key is for a
/// row whose panels come and go, where a place would hand one panel's state to
/// the next.
public struct SplitterPanel {
  /// What the panel is known by, when its place in the row is not enough.
  public var key: AnyHashable?
  /// Share of the run at rest. The shares are normalised, so three panels
  /// written `50`, `25`, `25` and three written `2`, `1`, `1` lay out
  /// identically.
  public var defaultSize: CGFloat
  /// Floor and ceiling, as percentages of the whole run.
  public var minSize: CGFloat
  public var maxSize: CGFloat
  public var content: AnyView

  public init<Content: View>(
    key: AnyHashable? = nil,
    defaultSize: CGFloat = 1,
    minSize: CGFloat = 0,
    maxSize: CGFloat = 100,
    @ViewBuilder content: () -> Content
  ) {
    self.key = key
    self.defaultSize = defaultSize
    self.minSize = minSize
    self.maxSize = maxSize
    self.content = AnyView(content())
  }
}

/// How much of the run a handle takes.
public enum SplitterSeam: String, CaseIterable, Sendable {
  /// A whole step of the scale, with the rule centred in it — React's handle.
  case gutter
  /// The rule's own width, with a target reaching a step into the panels
  /// either side: for panes whose surfaces should meet at the rule.
  case hairline
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
  public var seam: SplitterSeam
  public var onSizesChange: (([CGFloat]) -> Void)?

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline
  @State private var shares: [CGFloat] = []

  public init(
    orientation: SplitterOrientation = .horizontal,
    seam: SplitterSeam = .gutter,
    panels: [SplitterPanel],
    onSizesChange: (([CGFloat]) -> Void)? = nil
  ) {
    self.orientation = orientation
    self.seam = seam
    self.panels = panels
    self.onSizesChange = onSizesChange
  }

  private var horizontal: Bool { orientation == .horizontal }

  public var body: some View {
    let vars = theme.vars
    // What a handle takes of the run, and how far its target reaches.
    let (extent, target): (CGFloat, CGFloat) = switch seam {
    case .gutter: (vars.spacing2, vars.spacing2)
    case .hairline: (hairline, hairline + 2 * vars.spacing1)
    }
    let seams = max(panels.count - 1, 0)

    GeometryReader { proxy in
      // The handles take their width off the top and the panels divide what is
      // left, so a share is a share of the panels' run rather than of the box.
      let whole = horizontal ? proxy.size.width : proxy.size.height
      let run = max(whole - CGFloat(seams) * extent, 1)
      let current = shares.count == panels.count ? shares : normalised()

      layout {
        // Known by place, or by key: never by a value made afresh on each
        // redraw, which would rebuild every panel's content whenever the host
        // changed anything at all.
        ForEach(Array(panels.enumerated()), id: \.offset) { index, panel in
          if index > 0 {
            Seam(
              orientation: orientation,
              extent: extent,
              target: target,
              onDrag: { delta in resize(index - 1, delta: delta / run * 100) },
              onReset: {
                let pair = current[index - 1] + current[index]
                set(index - 1, leading: pair / 2)
              }
            )
            // Over the panels beside it, where its target reaches into them.
            .zIndex(1)
          }
          panel.content
            .id(panel.key ?? AnyHashable(index))
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
    .onAppear { if shares.count != panels.count { shares = normalised() } }
    .onChange(of: panels.count) { _ in shares = normalised() }
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
    let current = shares.count == panels.count ? shares : normalised()
    set(index, leading: current[index] + delta)
  }

  /// Given the leading panel's new share, the trailing one takes what the pair
  /// has left over. Nothing else in the row moves, which is what stops a drag
  /// at one seam from rippling through the whole window.
  private func set(_ index: Int, leading wanted: CGFloat) {
    var next = shares.count == panels.count ? shares : normalised()
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
  /// What the handle takes of the run.
  let extent: CGFloat
  /// How far its hit target reaches, centred on the rule.
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
        width: horizontal ? extent : nil,
        height: horizontal ? nil : extent
      )
      .frame(maxWidth: horizontal ? nil : .infinity,
             maxHeight: horizontal ? .infinity : nil)
      .overlay {
        // The target, which may be wider than what the handle takes.
        Color.clear
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
              // Pushed and popped in the same handler, so the stack stays
              // balanced even when the pointer leaves during a drag.
              if inside {
                (horizontal ? NSCursor.resizeLeftRight : NSCursor.resizeUpDown).push()
              } else {
                NSCursor.pop()
              }
            #endif
          }
          .gesture(
            DragGesture(minimumDistance: 0, coordinateSpace: .global)
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
      }
      .animation(vars.motion, value: lit)
  }
}
