import SwiftUI

#if os(macOS)
import AppKit
#endif

/// How far the divider may travel. Below the floor the labels stop fitting
/// their rows; above the ceiling the sidebar starts competing with the pane it
/// is meant to serve.
///
/// These are behaviour rather than design: the React component hard-codes them
/// in the module too, because the token graph has no role for the reach of a
/// drag.
public let sidebarMinimumWidth: CGFloat = 150
public let sidebarMaximumWidth: CGFloat = 320

/// Drag this far past the floor and the sidebar collapses instead of
/// shrinking. A desktop divider does this, and it is the only way to close a
/// sidebar without going back to the toolbar button.
private let collapseSlop: CGFloat = 32

/// Arrow keys walk the divider; shift makes the step a coarse one.
private let keyStep: CGFloat = 8
private let coarseKeyStep: CGFloat = 32

/// The separator lights up under the pointer and goes solid while dragging.
private let resizerHintOpacity: Double = 0.6

/// How a ``Sidebar`` collapses when ``Sidebar/collapsed`` is true.
public enum SidebarCollapsible: String, CaseIterable, Sendable {
  /// Slides completely out of view (0pt width, non-interactive).
  case offcanvas
  /// Collapses to the compact icon rail (`frameSidebarIconWidth`, 48pt),
  /// keeping navigation icons interactive with tooltips.
  case icon
}

private struct SidebarCollapsedKey: EnvironmentKey {
  static let defaultValue = false
}

private struct SidebarCollapsibleKey: EnvironmentKey {
  static let defaultValue: SidebarCollapsible = .offcanvas
}

public extension EnvironmentValues {
  /// Whether the enclosing ``Sidebar`` is currently collapsed.
  var sidebarCollapsed: Bool {
    get { self[SidebarCollapsedKey.self] }
    set { self[SidebarCollapsedKey.self] = newValue }
  }

  /// Collapse mode of the enclosing ``Sidebar``.
  var sidebarCollapsible: SidebarCollapsible {
    get { self[SidebarCollapsibleKey.self] }
    set { self[SidebarCollapsibleKey.self] = newValue }
  }
}

/// The left workspace column.
///
/// Its width is a token rather than a layout choice because the columns of a
/// desktop shell have to agree with each other: the header strip is exactly
/// the titlebar's height, so it lines up with the toolbar in the pane beside
/// it.
///
/// A column put away with ``collapsed`` is not unmounted: in `.offcanvas` mode
/// it keeps its width behind a clip anchored to the leading edge; in `.icon`
/// mode it shrinks to `frameSidebarIconWidth` (48pt) so icon destinations stay
/// reachable while preserving the expanded width across round trips.
public struct Sidebar<Header: View, Content: View, Footer: View>: View {
  /// Content for the strip above the row list, held at exactly the titlebar's
  /// height.
  public var header: (() -> Header)?
  /// Optional content pinned below the scrolling row list.
  public var footer: (() -> Footer)?
  /// Let the separator on the trailing edge be dragged. Off by default: a
  /// sidebar standing on its own has no pane to trade width with, and a handle
  /// that leads nowhere is worse than no handle.
  public var resizable: Bool
  /// Controlled width. Leave it out and the sidebar owns its own.
  public var width: Binding<CGFloat>?
  /// Starting width for the uncontrolled case; defaults to the token.
  public var defaultWidth: CGFloat?
  public var minWidth: CGFloat
  public var maxWidth: CGFloat
  /// Whether the column is put away.
  public var collapsed: Bool
  /// Whether collapsing slides the column off-canvas (`0pt`) or compacts it to
  /// its icon rail (`frameSidebarIconWidth`).
  public var collapsible: SidebarCollapsible
  /// Called when the divider is dragged past the floor. Left out, the drag
  /// simply stops at `minWidth` — pass it only where collapsing is a state the
  /// window can actually be in.
  public var onCollapse: (() -> Void)?
  /// Accessible name for the divider.
  public var resizeLabel: String
  public var tint: Tint
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme

  @State private var ownWidth: CGFloat?

  public init(
    resizable: Bool = false,
    width: Binding<CGFloat>? = nil,
    defaultWidth: CGFloat? = nil,
    minWidth: CGFloat = sidebarMinimumWidth,
    maxWidth: CGFloat = sidebarMaximumWidth,
    collapsed: Bool = false,
    collapsible: SidebarCollapsible = .offcanvas,
    onCollapse: (() -> Void)? = nil,
    resizeLabel: String = "Resize sidebar",
    tint: Tint = .primary,
    @ViewBuilder header: @escaping () -> Header,
    @ViewBuilder content: @escaping () -> Content,
    @ViewBuilder footer: @escaping () -> Footer
  ) {
    self.init(
      headerBuilder: header,
      footerBuilder: footer,
      resizable: resizable,
      width: width,
      defaultWidth: defaultWidth,
      minWidth: minWidth,
      maxWidth: maxWidth,
      collapsed: collapsed,
      collapsible: collapsible,
      onCollapse: onCollapse,
      resizeLabel: resizeLabel,
      tint: tint,
      content: content
    )
  }

  init(
    headerBuilder: (() -> Header)?,
    footerBuilder: (() -> Footer)?,
    resizable: Bool,
    width: Binding<CGFloat>?,
    defaultWidth: CGFloat?,
    minWidth: CGFloat,
    maxWidth: CGFloat,
    collapsed: Bool,
    collapsible: SidebarCollapsible,
    onCollapse: (() -> Void)?,
    resizeLabel: String,
    tint: Tint,
    content: @escaping () -> Content
  ) {
    self.header = headerBuilder
    self.footer = footerBuilder
    self.resizable = resizable
    self.width = width
    self.defaultWidth = defaultWidth
    self.minWidth = minWidth
    self.maxWidth = maxWidth
    self.collapsed = collapsed
    self.collapsible = collapsible
    self.onCollapse = onCollapse
    self.resizeLabel = resizeLabel
    self.tint = tint
    self.content = content
  }

  private var isOffcanvasHidden: Bool {
    collapsed && collapsible == .offcanvas
  }

  private var isIconCompact: Bool {
    collapsed && collapsible == .icon
  }

  public var body: some View {
    let vars = theme.vars
    let targetWidth: CGFloat = collapsed
      ? (collapsible == .icon ? vars.frameSidebarIconWidth : 0)
      : resolvedWidth

    column(vars: vars, innerWidth: isIconCompact ? vars.frameSidebarIconWidth : resolvedWidth)
      .environment(\.sidebarCollapsed, collapsed)
      .environment(\.sidebarCollapsible, collapsible)
      // Put away offcanvas, the column keeps the width it has when open and is
      // cut by its own leading edge as it shuts, so a width the divider was
      // dragged to survives the round trip. In icon mode it compacts to
      // `frameSidebarIconWidth` while staying interactive.
      .frame(width: targetWidth, alignment: .leading)
      .clipped()
      .overlay(alignment: .trailing) {
        if resizable && !collapsed {
          SidebarResizer(
            vars: vars,
            tint: tint,
            minWidth: minWidth,
            maxWidth: maxWidth,
            naturalWidth: naturalWidth,
            resolvedWidth: resolvedWidth,
            resizeLabel: resizeLabel,
            onCollapse: onCollapse,
            commit: commit
          )
        }
      }
      .allowsHitTesting(!isOffcanvasHidden)
      .accessibilityHidden(isOffcanvasHidden)
  }

  /// The column itself, at the open width (or icon rail width when icon-collapsed).
  private func column(vars: ThemeVariables, innerWidth: CGFloat) -> some View {
    let hairline = vars.strokeHairline
    return VStack(spacing: 0) {
      if let header {
        HStack(spacing: 0) {
          header()
          if !isIconCompact {
            Spacer(minLength: 0)
          }
        }
        .padding(.horizontal, isIconCompact ? vars.spacing15 : vars.spacing4)
        .frame(maxWidth: .infinity, alignment: isIconCompact ? .center : .leading)
        .frame(height: vars.frameTitlebarSize)
      }
      rowList(vars: vars)
      if let footer {
        footer()
          .frame(maxWidth: .infinity, alignment: isIconCompact ? .center : .leading)
          .fixedSize(horizontal: false, vertical: true)
          .padding(.horizontal, isIconCompact ? vars.spacing15 : vars.spacing25)
          .padding(.vertical, vars.spacing2)
          .rule(isIconCompact ? vars.colorBorder : .clear, edge: .top, width: hairline)
      }
    }
    // The edge's hairline is drawn inside the column, as CSS's border is.
    .padding(.trailing, hairline)
    .frame(width: innerWidth)
    .background(vars.colorSurfaceColumn)
    .rule(vars.colorBorder, edge: .trailing, width: hairline)
  }

  /// Only the row list scrolls, so a pinned card stays put. Groups carry
  /// their own inset (`.dz-sidebar__group`); the list adds only the content's
  /// block padding (`.dz-sidebar__content`).
  private func rowList(vars: ThemeVariables) -> some View {
    GeometryReader { proxy in
      ScrollView(.vertical) {
        VStack(alignment: .leading, spacing: 0) {
          content()
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding(.vertical, vars.spacing2)
        // A short list still has to let a pinned card — or a trailing
        // `NavAction` — sink to the foot, which needs a height to sink through.
        .frame(minHeight: proxy.size.height, alignment: .top)
      }
    }
  }

  private var resolvedWidth: CGFloat {
    width?.wrappedValue ?? ownWidth ?? naturalWidth
  }

  /// Where a double-click sends the divider home to.
  ///
  /// React measures this off the rendered element, because there CSS owns the
  /// width and reading `--frame-sidebar-width` would mean agreeing with
  /// whatever unit the token carries forever. Here the token is already a
  /// number in points, so it can simply be read.
  private var naturalWidth: CGFloat {
    defaultWidth ?? theme.vars.frameSidebarWidth
  }

  private func commit(_ next: CGFloat) {
    guard next != resolvedWidth else { return }
    if let width {
      width.wrappedValue = next
    } else {
      ownWidth = next
    }
  }
}

private final class SidebarDragState {
  var origin: CGFloat?
  var collapsedByThisDrag = false
}

/// Isolated resizer handle so hover, focus, and drag state updates never
/// re-evaluate the sidebar's `header`, `content`, or `footer` subtrees.
private struct SidebarResizer: View {
  let vars: ThemeVariables
  let tint: Tint
  let minWidth: CGFloat
  let maxWidth: CGFloat
  let naturalWidth: CGFloat
  let resolvedWidth: CGFloat
  let resizeLabel: String
  let onCollapse: (() -> Void)?
  let commit: (CGFloat) -> Void

  @State private var dragState = SidebarDragState()
  @State private var dragging = false
  @State private var hovering = false
  @FocusState private var focused: Bool

  private var lineOpacity: Double {
    if dragging { return 1 }
    return hovering || focused ? resizerHintOpacity : 0
  }

  private func clamp(_ value: CGFloat) -> CGFloat {
    min(maxWidth, max(minWidth, value)).rounded()
  }

  /// Discrete jumps — a double-click home, an arrow-key step, an accessibility
  /// nudge — settle over the motion token, reflowing the column the way a short
  /// smooth drag would. A drag keeps applying its width on the spot so the
  /// divider never trails the pointer.
  private func settle(_ change: () -> Void) {
    withAnimation(vars.motion, change)
  }

  var body: some View {
    withResizerFocus(
      Color.clear
        .frame(width: vars.spacing2)
        .contentShape(Rectangle())
        .overlay(alignment: .trailing) {
          Rectangle()
            .fill(vars.ramp(tint)[600])
            .frame(width: vars.spacingPx)
            .padding(.trailing, vars.spacing2 / 2)
            .opacity(lineOpacity)
            .animation(vars.motion, value: lineOpacity)
        }
        .offset(x: vars.spacing2 / 2)
        .onTapGesture(count: 2) {
          // Double-clicking a divider puts it back where it started, the same
          // thing every split view does.
          settle { commit(clamp(naturalWidth)) }
        }
        .gesture(drag)
        .onHover { inside in
          guard hovering != inside else { return }
          hovering = inside
          #if os(macOS)
            // The cursor belongs to whatever the pointer is over, so it is
            // pinned for as long as the pointer is over the handle.
            if inside {
              NSCursor.resizeLeftRight.push()
            } else {
              NSCursor.pop()
            }
          #endif
        }
        .onDisappear {
          #if os(macOS)
            if hovering {
              NSCursor.pop()
              hovering = false
            }
          #endif
          dragState.origin = nil
          dragState.collapsedByThisDrag = false
          dragging = false
        }
    )
    .accessibilityElement()
    .accessibilityLabel(resizeLabel)
    .accessibilityValue(Text("\(Int(resolvedWidth))"))
    .accessibilityAdjustableAction { direction in
      settle {
        switch direction {
        case .increment: commit(clamp(resolvedWidth + keyStep))
        case .decrement: commit(clamp(resolvedWidth - keyStep))
        @unknown default: break
        }
      }
    }
  }

  private var drag: some Gesture {
    // Measured in the window, not in the handle: the handle rides the edge
    // it moves, so a translation taken in its own space would lose every
    // point the previous frame moved it and the sidebar would shudder
    // between two widths under the pointer.
    DragGesture(minimumDistance: 1, coordinateSpace: .global)
      .onChanged { value in
        guard !dragState.collapsedByThisDrag else { return }
        let from = dragState.origin ?? resolvedWidth
        dragState.origin = from

        let raw = from + value.translation.width
        if let onCollapse, raw < minWidth - collapseSlop {
          // Collapsed is not a width. Hand back the one the drag started
          // from, so re-opening the sidebar does not inherit a half-dragged
          // number.
          commit(clamp(from))
          dragState.collapsedByThisDrag = true
          if dragging { dragging = false }
          onCollapse()
          return
        }
        if !dragging { dragging = true }
        commit(clamp(raw))
      }
      .onEnded { _ in
        dragState.origin = nil
        dragState.collapsedByThisDrag = false
        if dragging { dragging = false }
      }
  }

  /// Arrow keys walk the divider. Keyboard focus and move commands are a
  /// desktop affordance and only exist there, so the handle is otherwise
  /// reached through its adjustable accessibility action.
  @ViewBuilder
  private func withResizerFocus<V: View>(_ view: V) -> some View {
    #if os(macOS)
      if #available(macOS 14.0, *) {
        // A control, not a field: it takes focus only when the reader tabs to
        // it with keyboard navigation on, never as the window's first
        // responder — a divider that lit up on every window the moment it
        // opened read as already being dragged. The separator already marks
        // keyboard focus, so the system ring, which would outline the whole
        // grab area with a blue line on each side, stays off.
        withResizerMoveCommands(view.focusable(true, interactions: .activate))
          .focusEffectDisabled()
      } else {
        withResizerMoveCommands(view.focusable(true))
      }
    #else
      view
    #endif
  }

  #if os(macOS)
    private func withResizerMoveCommands<V: View>(_ view: V) -> some View {
      view
        .focused($focused)
        .onMoveCommand { direction in
          let step = NSEvent.modifierFlags.contains(.shift) ? coarseKeyStep : keyStep
          switch direction {
          case .left: settle { commit(clamp(resolvedWidth - step)) }
          case .right: settle { commit(clamp(resolvedWidth + step)) }
          default: break
          }
        }
    }
  #endif
}

public extension Sidebar where Footer == EmptyView {
  /// A sidebar with a header strip and no pinned footer.
  init(
    resizable: Bool = false,
    width: Binding<CGFloat>? = nil,
    defaultWidth: CGFloat? = nil,
    minWidth: CGFloat = sidebarMinimumWidth,
    maxWidth: CGFloat = sidebarMaximumWidth,
    collapsed: Bool = false,
    collapsible: SidebarCollapsible = .offcanvas,
    onCollapse: (() -> Void)? = nil,
    resizeLabel: String = "Resize sidebar",
    tint: Tint = .primary,
    @ViewBuilder header: @escaping () -> Header,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.init(
      headerBuilder: header,
      footerBuilder: nil,
      resizable: resizable,
      width: width,
      defaultWidth: defaultWidth,
      minWidth: minWidth,
      maxWidth: maxWidth,
      collapsed: collapsed,
      collapsible: collapsible,
      onCollapse: onCollapse,
      resizeLabel: resizeLabel,
      tint: tint,
      content: content
    )
  }
}

public extension Sidebar where Header == EmptyView {
  /// A sidebar with no header strip and a pinned footer.
  init(
    resizable: Bool = false,
    width: Binding<CGFloat>? = nil,
    defaultWidth: CGFloat? = nil,
    minWidth: CGFloat = sidebarMinimumWidth,
    maxWidth: CGFloat = sidebarMaximumWidth,
    collapsed: Bool = false,
    collapsible: SidebarCollapsible = .offcanvas,
    onCollapse: (() -> Void)? = nil,
    resizeLabel: String = "Resize sidebar",
    tint: Tint = .primary,
    @ViewBuilder content: @escaping () -> Content,
    @ViewBuilder footer: @escaping () -> Footer
  ) {
    self.init(
      headerBuilder: nil,
      footerBuilder: footer,
      resizable: resizable,
      width: width,
      defaultWidth: defaultWidth,
      minWidth: minWidth,
      maxWidth: maxWidth,
      collapsed: collapsed,
      collapsible: collapsible,
      onCollapse: onCollapse,
      resizeLabel: resizeLabel,
      tint: tint,
      content: content
    )
  }
}

public extension Sidebar where Header == EmptyView, Footer == EmptyView {
  /// A sidebar with neither a header strip nor a pinned footer.
  init(
    resizable: Bool = false,
    width: Binding<CGFloat>? = nil,
    defaultWidth: CGFloat? = nil,
    minWidth: CGFloat = sidebarMinimumWidth,
    maxWidth: CGFloat = sidebarMaximumWidth,
    collapsed: Bool = false,
    collapsible: SidebarCollapsible = .offcanvas,
    onCollapse: (() -> Void)? = nil,
    resizeLabel: String = "Resize sidebar",
    tint: Tint = .primary,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.init(
      headerBuilder: nil,
      footerBuilder: nil,
      resizable: resizable,
      width: width,
      defaultWidth: defaultWidth,
      minWidth: minWidth,
      maxWidth: maxWidth,
      collapsed: collapsed,
      collapsible: collapsible,
      onCollapse: onCollapse,
      resizeLabel: resizeLabel,
      tint: tint,
      content: content
    )
  }
}

/// A labelled run of rows — `SidebarGroup` with its `SidebarGroupLabel` and
/// `SidebarGroupAction`.
///
/// The first run usually goes unlabelled — it is what the column is already
/// about — and the gap between groups does most of the work. The label sits on
/// the rows' own inset, in the faint tone, so it reads as the heading of the
/// labels under it rather than of the column.
public struct SidebarGroup<Label: View, Action: View, Content: View>: View {
  public var label: (() -> Label)?
  public var action: (() -> Action)?
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.sidebarCollapsed) private var sidebarCollapsed
  @Environment(\.sidebarCollapsible) private var sidebarCollapsible

  public init(
    @ViewBuilder label: @escaping () -> Label,
    @ViewBuilder action: @escaping () -> Action,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.init(labelBuilder: label, actionBuilder: action, content: content)
  }

  init(labelBuilder: (() -> Label)?, actionBuilder: (() -> Action)?, content: @escaping () -> Content) {
    self.label = labelBuilder
    self.action = actionBuilder
    self.content = content
  }

  private var isIconCompact: Bool {
    sidebarCollapsed && sidebarCollapsible == .icon
  }

  public var body: some View {
    let vars = theme.vars

    VStack(alignment: isIconCompact ? .center : .leading, spacing: 0) {
      if let label, !isIconCompact {
        // A heading carrying a button is a row of two columns — the label
        // taking what is left, the button a column of its own — both on the
        // row's centre. The label's line reserves `sidebar.group-label-height`,
        // the button's height, so a heading is as tall with one as without.
        HStack(spacing: 0) {
          HStack(spacing: 0) {
            SectionLabel(faint: true) { label() }
            Spacer(minLength: 0)
          }
          .padding(.vertical, vars.spacing1)
          .padding(.horizontal, vars.spacing25)
          .frame(minHeight: vars.sidebarGroupLabelHeight)
          if let action {
            action()
              .padding(.leading, vars.spacing1)
          }
        }
      }
      VStack(alignment: isIconCompact ? .center : .leading, spacing: vars.frameNavGap) {
        content()
      }
      .frame(maxWidth: .infinity, alignment: isIconCompact ? .center : .leading)
    }
    .frame(maxWidth: .infinity, alignment: isIconCompact ? .center : .leading)
    .padding(.vertical, vars.spacing15)
    .padding(.horizontal, isIconCompact ? vars.spacing15 : vars.spacing25)
  }
}

public extension SidebarGroup where Label == Text, Action == EmptyView {
  /// A group whose label is a plain string.
  init(_ title: String, @ViewBuilder content: @escaping () -> Content) {
    self.init(labelBuilder: { Text(title) }, actionBuilder: nil, content: content)
  }
}

public extension SidebarGroup where Label == Text {
  /// A group whose label is a plain string, with a button on it — "add",
  /// "collapse".
  init(
    _ title: String,
    @ViewBuilder action: @escaping () -> Action,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.init(labelBuilder: { Text(title) }, actionBuilder: action, content: content)
  }
}

public extension SidebarGroup where Label == EmptyView, Action == EmptyView {
  /// An unnamed run of rows.
  init(@ViewBuilder content: @escaping () -> Content) {
    self.init(labelBuilder: nil, actionBuilder: nil, content: content)
  }
}

public extension SidebarGroup where Action == EmptyView {
  /// A group with a label of its own making.
  init(@ViewBuilder label: @escaping () -> Label, @ViewBuilder content: @escaping () -> Content) {
    self.init(labelBuilder: label, actionBuilder: nil, content: content)
  }
}

/// The card pinned to the foot of a sidebar — a count, a version, a queue.
public struct SidebarCard<Label: View, Content: View>: View {
  public var label: (() -> Label)?
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme

  public init(
    @ViewBuilder label: @escaping () -> Label,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.init(labelBuilder: label, content: content)
  }

  init(labelBuilder: (() -> Label)?, content: @escaping () -> Content) {
    self.label = labelBuilder
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    // `margin-top: auto` — a flexible leading spacer is how a stack hands its
    // slack to one child, so the card sinks to the foot of the column
    // whatever the list above it does.
    VStack(spacing: 0) {
      Spacer(minLength: 0)
      VStack(alignment: .leading, spacing: vars.spacing2) {
        if let label {
          SectionLabel { label() }
        }
        content()
      }
      .frame(maxWidth: .infinity, alignment: .leading)
      .padding(vars.spacing3)
      .background(vars.colorSurface)
      .clipShape(RoundedRectangle(cornerRadius: vars.radiusLarge, style: .circular))
      .border(vars.colorBorder, width: vars.strokeHairline, cornerRadius: vars.radiusLarge)
    }
  }
}

public extension SidebarCard where Label == Text {
  /// A card whose label is a plain string.
  init(_ title: String, @ViewBuilder content: @escaping () -> Content) {
    self.init(labelBuilder: { Text(title) }, content: content)
  }
}

public extension SidebarCard where Label == EmptyView {
  /// An unlabelled card.
  init(@ViewBuilder content: @escaping () -> Content) {
    self.init(labelBuilder: nil, content: content)
  }
}

