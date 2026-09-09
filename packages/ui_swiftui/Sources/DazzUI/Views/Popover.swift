import SwiftUI

/// Which side of the trigger the panel opens on.
public enum PopoverSide: String, CaseIterable, Sendable {
  case top
  case bottom
  case leading
  case trailing
}

/// Which edge of the trigger the panel lines up with.
public enum PopoverAlign: String, CaseIterable, Sendable {
  case start
  case center
  case end
}

/// What a ``Popover`` hands its trigger builder.
public struct PopoverTriggerState {
  public var open: Bool
  public var toggle: () -> Void

  public init(open: Bool, toggle: @escaping () -> Void) {
    self.open = open
    self.toggle = toggle
  }
}

/// The floating sheet a popover and a preview card both wear.
///
/// It is the menu panel's materials — the overlay surface, the strong
/// hairline, the deep shadow — padded for prose rather than for rows: a sheet
/// built for short rows pads by a single step, and a paragraph in that would
/// sit on its own edge.
///
/// The width is deliberately not set. A menu has a minimum because a menu
/// narrower than its trigger reads as broken; a popover is as wide as what is
/// in it, and a caller who knows better passes one.
struct PopoverPanel<Content: View>: View {
  var width: CGFloat?
  var maxWidth: CGFloat?
  @ViewBuilder var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  var body: some View {
    let vars = theme.vars

    VStack(alignment: .leading, spacing: 0) {
      content()
    }
    .font(vars.bodySmall.font)
    .foregroundStyle(vars.colorContent)
    .padding(vars.spacing4 + vars.spacing05)
    .frame(width: width, alignment: .leading)
    .frame(maxWidth: maxWidth, alignment: .leading)
    .background(vars.colorSurfaceOverlay)
    .clipShape(RoundedRectangle(cornerRadius: vars.radiusLarge, style: .circular))
    .border(vars.colorBorderStrong, width: hairline, cornerRadius: vars.radiusLarge)
    .tokenShadow(vars.shadowLg)
    .fixedSize(horizontal: width == nil && maxWidth == nil, vertical: true)
    .accessibilityElement(children: .contain)
  }
}

extension View {
  /// Hangs a panel off one side of this view, at one alignment, with the gap
  /// the tokens state — the counterpart of the padding a positioner carries in
  /// the browser.
  @ViewBuilder
  func anchored<Panel: View>(
    _ side: PopoverSide,
    _ align: PopoverAlign,
    gap: CGFloat,
    @ViewBuilder panel: () -> Panel
  ) -> some View {
    switch side {
    case .bottom:
      overlay(alignment: verticalAlignment(align)) {
        panel().alignmentGuide(.bottom) { _ in 0 }.offset(y: gap)
      }
    case .top:
      overlay(alignment: verticalAlignment(align, top: true)) {
        panel().alignmentGuide(.top) { dimension in dimension.height }.offset(y: -gap)
      }
    case .leading:
      overlay(alignment: .leading) {
        panel().alignmentGuide(.leading) { dimension in dimension.width }.offset(x: -gap)
      }
    case .trailing:
      overlay(alignment: .trailing) {
        panel().alignmentGuide(.trailing) { _ in 0 }.offset(x: gap)
      }
    }
  }

  private func verticalAlignment(_ align: PopoverAlign, top: Bool = false) -> Alignment {
    switch align {
    case .start: return top ? .topLeading : .bottomLeading
    case .center: return top ? .top : .bottom
    case .end: return top ? .topTrailing : .bottomTrailing
    }
  }
}

/// A panel of anything, anchored to what opened it.
///
/// It is the sibling of ``Menu`` rather than of ``Dialog``: it wears the same
/// kind of floating sheet and is positioned against its trigger, but it holds
/// content rather than rows. Reach for a dialog instead when the reader has to
/// finish before doing anything else — a popover closes on Escape, which makes
/// it the wrong home for something half-typed.
public struct Popover<Trigger: View, Content: View>: View {
  /// Names the panel as well as heading it.
  public var title: String?
  /// A line under the title.
  public var description: String?
  public var side: PopoverSide
  public var align: PopoverAlign
  /// Overrides the panel's own width, which is otherwise its content's.
  public var width: CGFloat?
  @ViewBuilder public var trigger: (PopoverTriggerState) -> Trigger
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @State private var open = false

  public init(
    title: String? = nil,
    description: String? = nil,
    side: PopoverSide = .bottom,
    align: PopoverAlign = .center,
    width: CGFloat? = nil,
    @ViewBuilder trigger: @escaping (PopoverTriggerState) -> Trigger,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.title = title
    self.description = description
    self.side = side
    self.align = align
    self.width = width
    self.trigger = trigger
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    trigger(PopoverTriggerState(open: open, toggle: { open.toggle() }))
      .anchored(side, align, gap: vars.spacing1) {
        if open {
          PopoverPanel(width: width) {
            if let title {
              Text(title)
                .font(.system(size: vars.labelLarge.size, weight: vars.labelLarge.weight))
                .foregroundStyle(vars.colorContent)
            }
            if let description {
              Text(description)
                .font(vars.bodySmall.font)
                .lineSpacing(vars.bodySmall.lineSpacing)
                .foregroundStyle(vars.colorContentSecondary)
                .padding(.top, vars.spacing1)
                .fixedSize(horizontal: false, vertical: true)
            }
            // The run between the head and whatever the panel is actually for.
            // It is a pad above the body rather than a gap on the panel, so a
            // popover that is only a form has no leading space to trim.
            content()
              .padding(.top, title == nil && description == nil ? 0 : vars.spacing3)
          }
        }
      }
      .background {
        if open {
          SwiftUI.Button("") { open = false }
            .keyboardShortcut(.cancelAction)
            .opacity(0)
            .frame(width: 0, height: 0)
        }
      }
  }
}

public extension Popover where Content == EmptyView {
  /// A popover that is only its heading — a short explanation with nothing to
  /// press.
  init(
    title: String? = nil,
    description: String? = nil,
    side: PopoverSide = .bottom,
    align: PopoverAlign = .center,
    width: CGFloat? = nil,
    @ViewBuilder trigger: @escaping (PopoverTriggerState) -> Trigger
  ) {
    self.init(
      title: title,
      description: description,
      side: side,
      align: align,
      width: width,
      trigger: trigger,
      content: { EmptyView() }
    )
  }
}

/// What is behind a link, shown without following it — a repository, a person,
/// an issue.
///
/// It is a ``Tooltip`` that holds a card rather than a label, and the
/// difference is what may go in it: a tooltip must carry nothing a reader
/// needs, because a pointer never reaches it; a preview card stays open while
/// the pointer travels, so it can hold a link of its own. It is never the only
/// way to reach what it shows.
///
/// The sheet is the ``Popover``'s — a preview card is a popover the pointer
/// opens, so a second drawing of it could only drift. What is its own is a
/// ceiling on the width: a popover is as wide as what a caller put in it; this
/// is filled from whatever it previews.
public struct PreviewCard<Trigger: View, Content: View>: View {
  public var side: PopoverSide
  public var align: PopoverAlign
  public var width: CGFloat?
  @ViewBuilder public var trigger: () -> Trigger
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @State private var open = false

  public init(
    side: PopoverSide = .bottom,
    align: PopoverAlign = .center,
    width: CGFloat? = nil,
    @ViewBuilder trigger: @escaping () -> Trigger,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.side = side
    self.align = align
    self.width = width
    self.trigger = trigger
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    trigger()
      // The pointer opens it and the pointer closes it. The card hangs off the
      // trigger's own box, so the hover that keeps it up is the same one that
      // raised it — which is what lets a reader travel into the card without
      // it going out from under them.
      .onHover { open = $0 }
      .anchored(side, align, gap: vars.spacing1) {
        if open {
          PopoverPanel(maxWidth: width ?? vars.previewCardWidth, content: content)
        }
      }
  }
}
