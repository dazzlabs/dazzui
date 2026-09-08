import SwiftUI

#if canImport(AppKit)
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

/// The left workspace column.
///
/// Its width is a token rather than a layout choice because the columns of a
/// desktop shell have to agree with each other: the header strip is exactly
/// the titlebar's height, so it lines up with the toolbar in the pane beside
/// it.
public struct Sidebar<Header: View, Content: View>: View {
  /// Content for the strip above the row list, held at exactly the titlebar's
  /// height.
  public var header: (() -> Header)?
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
  /// Called when the divider is dragged past the floor. Left out, the drag
  /// simply stops at `minWidth` — pass it only where collapsing is a state the
  /// window can actually be in.
  public var onCollapse: (() -> Void)?
  /// Accessible name for the divider.
  public var resizeLabel: String
  public var tint: Tint
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  @State private var ownWidth: CGFloat?
  @State private var dragOrigin: CGFloat?
  @State private var dragging = false
  @State private var collapsedByThisDrag = false
  @State private var hoveringResizer = false
  @FocusState private var resizerFocused: Bool

  public init(
    resizable: Bool = false,
    width: Binding<CGFloat>? = nil,
    defaultWidth: CGFloat? = nil,
    minWidth: CGFloat = sidebarMinimumWidth,
    maxWidth: CGFloat = sidebarMaximumWidth,
    onCollapse: (() -> Void)? = nil,
    resizeLabel: String = "Resize sidebar",
    tint: Tint = .primary,
    @ViewBuilder header: @escaping () -> Header,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.init(
      headerBuilder: header,
      resizable: resizable,
      width: width,
      defaultWidth: defaultWidth,
      minWidth: minWidth,
      maxWidth: maxWidth,
      onCollapse: onCollapse,
      resizeLabel: resizeLabel,
      tint: tint,
      content: content
    )
  }

  init(
    headerBuilder: (() -> Header)?,
    resizable: Bool,
    width: Binding<CGFloat>?,
    defaultWidth: CGFloat?,
    minWidth: CGFloat,
    maxWidth: CGFloat,
    onCollapse: (() -> Void)?,
    resizeLabel: String,
    tint: Tint,
    content: @escaping () -> Content
  ) {
    self.header = headerBuilder
    self.resizable = resizable
    self.width = width
    self.defaultWidth = defaultWidth
    self.minWidth = minWidth
    self.maxWidth = maxWidth
    self.onCollapse = onCollapse
    self.resizeLabel = resizeLabel
    self.tint = tint
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    VStack(spacing: 0) {
      if let header {
        HStack(spacing: 0) {
          header()
          Spacer(minLength: 0)
        }
        .padding(.horizontal, vars.spacing4)
        .frame(height: vars.frameTitlebarSize)
      }
      rowList(vars: vars)
    }
    .frame(width: resolvedWidth)
    .background(vars.colorSurfaceColumn)
    .rule(vars.colorBorder, edge: .trailing, width: hairline)
    .overlay(alignment: .trailing) {
      if resizable { resizer(vars: vars) }
    }
  }

  /// Only the row list scrolls, so a pinned card stays put.
  private func rowList(vars: ThemeVariables) -> some View {
    GeometryReader { proxy in
      ScrollView(.vertical) {
        VStack(alignment: .leading, spacing: vars.frameNavGap) {
          content()
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding(.horizontal, vars.spacing25)
        // A group carries its own leading step and CSS zeroes it on the first
        // one with `:first-child`. SwiftUI has no such selector, so the body
        // hands that much of its top padding over instead: a first group puts
        // back exactly what was given up, and every later group adds the step
        // again on top of the gap.
        .padding(.top, vars.spacing35 - sidebarGroupStep(vars))
        .padding(.bottom, vars.spacing35)
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

  private func clamp(_ value: CGFloat) -> CGFloat {
    min(maxWidth, max(minWidth, value)).rounded()
  }

  private func commit(_ next: CGFloat) {
    if let width {
      width.wrappedValue = next
    } else {
      ownWidth = next
    }
  }

  /// The grab area straddles the separator and reaches a few points into the
  /// pane beside it, because a one-pixel target is not a target. Nothing is
  /// drawn there — the separator itself is what lights up.
  private func resizer(vars: ThemeVariables) -> some View {
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
          commit(clamp(naturalWidth))
        }
        .gesture(drag)
        .onHover { inside in
          hoveringResizer = inside
          #if canImport(AppKit)
          // The cursor belongs to whatever the pointer is over, so it is
          // pinned for as long as the pointer is over the handle.
          if inside {
            NSCursor.resizeLeftRight.push()
          } else {
            NSCursor.pop()
          }
          #endif
        }
    )
    .accessibilityElement()
    .accessibilityLabel(resizeLabel)
    .accessibilityValue(Text("\(Int(resolvedWidth))"))
    .accessibilityAdjustableAction { direction in
      switch direction {
      case .increment: commit(clamp(resolvedWidth + keyStep))
      case .decrement: commit(clamp(resolvedWidth - keyStep))
      @unknown default: break
      }
    }
  }

  private var lineOpacity: Double {
    if dragging { return 1 }
    return hoveringResizer || resizerFocused ? resizerHintOpacity : 0
  }

  private var drag: some Gesture {
    DragGesture(minimumDistance: 1)
      .onChanged { value in
        guard !collapsedByThisDrag else { return }
        let from = dragOrigin ?? resolvedWidth
        dragOrigin = from

        let raw = from + value.translation.width
        if let onCollapse, raw < minWidth - collapseSlop {
          // Collapsed is not a width. Hand back the one the drag started
          // from, so re-opening the sidebar does not inherit a half-dragged
          // number.
          commit(clamp(from))
          collapsedByThisDrag = true
          dragging = false
          onCollapse()
          return
        }
        dragging = true
        commit(clamp(raw))
      }
      .onEnded { _ in
        dragOrigin = nil
        dragging = false
        collapsedByThisDrag = false
      }
  }

  /// Arrow keys walk the divider. Keyboard focus and move commands are a
  /// desktop affordance and only exist there, so the handle is otherwise
  /// reached through its adjustable accessibility action.
  private func withResizerFocus<V: View>(_ view: V) -> some View {
    #if os(macOS)
    return view
      .focusable(resizable)
      .focused($resizerFocused)
      .onMoveCommand { direction in
        let step = NSEvent.modifierFlags.contains(.shift) ? coarseKeyStep : keyStep
        switch direction {
        case .left: commit(clamp(resolvedWidth - step))
        case .right: commit(clamp(resolvedWidth + step))
        default: break
        }
      }
    #else
    return view
    #endif
  }
}

public extension Sidebar where Header == EmptyView {
  /// A sidebar with no header strip.
  init(
    resizable: Bool = false,
    width: Binding<CGFloat>? = nil,
    defaultWidth: CGFloat? = nil,
    minWidth: CGFloat = sidebarMinimumWidth,
    maxWidth: CGFloat = sidebarMaximumWidth,
    onCollapse: (() -> Void)? = nil,
    resizeLabel: String = "Resize sidebar",
    tint: Tint = .primary,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.init(
      headerBuilder: nil,
      resizable: resizable,
      width: width,
      defaultWidth: defaultWidth,
      minWidth: minWidth,
      maxWidth: maxWidth,
      onCollapse: onCollapse,
      resizeLabel: resizeLabel,
      tint: tint,
      content: content
    )
  }
}

/// A labelled run of rows.
///
/// A sidebar breaks its rows into groups rather than stacking them; the gap
/// between groups does most of the work and the label only names it.
public struct SidebarGroup<Label: View, Content: View>: View {
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

    VStack(alignment: .leading, spacing: vars.frameNavGap) {
      if let label {
        SectionLabel { label() }
          .padding(.vertical, vars.spacing1)
          .padding(.horizontal, vars.spacing25)
      }
      content()
    }
    .frame(maxWidth: .infinity, alignment: .leading)
    .padding(.top, sidebarGroupStep(vars))
  }
}

public extension SidebarGroup where Label == Text {
  /// A group whose label is a plain string.
  init(_ title: String, @ViewBuilder content: @escaping () -> Content) {
    self.init(labelBuilder: { Text(title) }, content: content)
  }
}

public extension SidebarGroup where Label == EmptyView {
  /// An unnamed run of rows.
  init(@ViewBuilder content: @escaping () -> Content) {
    self.init(labelBuilder: nil, content: content)
  }
}

/// The card pinned to the foot of a sidebar — a count, a version, a queue.
public struct SidebarCard<Label: View, Content: View>: View {
  public var label: (() -> Label)?
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

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
      .border(vars.colorBorder, width: hairline, cornerRadius: vars.radiusLarge)
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

/// The step a sidebar group sets itself off by, over and above the body's gap.
private func sidebarGroupStep(_ vars: ThemeVariables) -> CGFloat {
  vars.spacing35
}
