import SwiftUI

#if os(macOS)
  import AppKit
#endif

/// Default drag bounds for a resizable ``Rail``.
public let railMinimumWidth: CGFloat = 120
public let railMaximumWidth: CGFloat = 280

private let railKeyStep: CGFloat = 8
private let railCoarseKeyStep: CGFloat = 32
private let railResizerHintOpacity: Double = 0.6

/// Background fill option for ``Rail``.
public enum RailSurface: String, CaseIterable, Sendable {
  /// Standard paper surface (`colorSurface`).
  case surface
  /// Midpoint between `colorSurfaceColumn` and `colorSurface` (`railSurface`),
  /// setting a secondary navigation rail softly apart from the main content pane.
  case rail
}

/// The second navigation column, between the sidebar and the content: the
/// groups, books or pages inside whatever the sidebar selected. Lighter than
/// the sidebar, because it sits closer to the content.
public struct Rail<Content: View>: View {
  public var resizable: Bool
  /// Controlled width. Leave nil to let the rail manage its own width.
  public var width: Binding<CGFloat>?
  /// Starting width when uncontrolled; defaults to `vars.frameRailWidth`.
  public var defaultWidth: CGFloat?
  public var minWidth: CGFloat
  public var maxWidth: CGFloat
  public var resizeLabel: String
  public var tint: Tint
  public var surface: RailSurface
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  @State private var ownWidth: CGFloat?
  @State private var dragOrigin: CGFloat?
  @State private var dragging = false
  @State private var hoveringResizer = false
  @FocusState private var resizerFocused: Bool

  public init(
    resizable: Bool = false,
    width: Binding<CGFloat>? = nil,
    defaultWidth: CGFloat? = nil,
    minWidth: CGFloat = railMinimumWidth,
    maxWidth: CGFloat = railMaximumWidth,
    resizeLabel: String = "Resize rail",
    tint: Tint = .primary,
    surface: RailSurface = .surface,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.resizable = resizable
    self.width = width
    self.defaultWidth = defaultWidth
    self.minWidth = minWidth
    self.maxWidth = maxWidth
    self.resizeLabel = resizeLabel
    self.tint = tint
    self.surface = surface
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    GeometryReader { proxy in
      ScrollView(.vertical) {
        VStack(alignment: .leading, spacing: vars.frameNavGap) {
          content()
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        // The column's inset is one value on every side. A group carries its
        // own leading step and CSS zeroes it on the first one with
        // `:first-child`. SwiftUI has no such selector, so the column hands
        // that much of its top padding over instead: a first group puts back
        // exactly what was given up — landing on the same 10 as the sides —
        // and every later group adds the step again on top of the gap.
        .padding(.horizontal, vars.spacing25)
        .padding(.top, vars.spacing25 - railGroupStep(vars))
        .padding(.bottom, vars.spacing25)
        // A run that does not fill the column still has to let a trailing
        // `NavAction` sink to the foot, which needs a height to sink through.
        .frame(minHeight: proxy.size.height, alignment: .top)
      }
    }
    .frame(width: resolvedWidth)
    .background(surface == .rail ? vars.railSurface : vars.colorSurface)
    .rule(vars.colorBorder, edge: .trailing, width: hairline)
    .overlay(alignment: .trailing) {
      if resizable { resizer(vars: vars) }
    }
  }

  private var resolvedWidth: CGFloat {
    width?.wrappedValue ?? ownWidth ?? naturalWidth
  }

  private var naturalWidth: CGFloat {
    defaultWidth ?? theme.vars.frameRailWidth
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

  private func settle(_ change: () -> Void) {
    withAnimation(theme.vars.motion, change)
  }

  private var lineOpacity: Double {
    if dragging { return 1 }
    return hoveringResizer || resizerFocused ? railResizerHintOpacity : 0
  }

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
          settle { commit(clamp(naturalWidth)) }
        }
        .gesture(
          DragGesture(minimumDistance: 1, coordinateSpace: .global)
            .onChanged { value in
              let from = dragOrigin ?? resolvedWidth
              dragOrigin = from
              dragging = true
              commit(clamp(from + value.translation.width))
            }
            .onEnded { _ in
              dragOrigin = nil
              dragging = false
            }
        )
        .onHover { inside in
          hoveringResizer = inside
          #if os(macOS)
            if inside {
              NSCursor.resizeLeftRight.push()
            } else {
              NSCursor.pop()
            }
          #endif
        }
        .onDisappear {
          #if os(macOS)
            if hoveringResizer {
              NSCursor.pop()
              hoveringResizer = false
            }
          #endif
        }
    )
    .accessibilityElement()
    .accessibilityLabel(resizeLabel)
    .accessibilityValue(Text("\(Int(resolvedWidth))"))
    .accessibilityAdjustableAction { direction in
      settle {
        switch direction {
        case .increment: commit(clamp(resolvedWidth + railKeyStep))
        case .decrement: commit(clamp(resolvedWidth - railKeyStep))
        @unknown default: break
        }
      }
    }
  }

  @ViewBuilder
  private func withResizerFocus<V: View>(_ view: V) -> some View {
    #if os(macOS)
      if #available(macOS 14.0, *) {
        withResizerMoveCommands(view.focusable(resizable, interactions: .activate))
          .focusEffectDisabled()
      } else {
        withResizerMoveCommands(view.focusable(resizable))
      }
    #else
      view
    #endif
  }

  #if os(macOS)
    private func withResizerMoveCommands<V: View>(_ view: V) -> some View {
      view
        .focused($resizerFocused)
        .onMoveCommand { direction in
          let step = NSEvent.modifierFlags.contains(.shift) ? railCoarseKeyStep : railKeyStep
          switch direction {
          case .left: settle { commit(clamp(resolvedWidth - step)) }
          case .right: settle { commit(clamp(resolvedWidth + step)) }
          default: break
          }
        }
    }
  #endif
}

/// A run of rail rows, optionally named.
///
/// The first run usually goes unlabelled: it is what the pane is already
/// called, and naming it twice says nothing. A second run is the one that
/// needs the label, because a gap alone leaves the reader to guess what the
/// rows below it have in common.
public struct RailGroup<Label: View, Content: View>: View {
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
    .padding(.top, railGroupStep(vars))
  }
}

public extension RailGroup where Label == Text {
  /// A group whose label is a plain string.
  init(_ title: String, @ViewBuilder content: @escaping () -> Content) {
    self.init(labelBuilder: { Text(title) }, content: content)
  }
}

public extension RailGroup where Label == EmptyView {
  /// An unnamed run of rows.
  init(@ViewBuilder content: @escaping () -> Content) {
    self.init(labelBuilder: nil, content: content)
  }
}

/// The step a rail group sets itself off by, over and above the column's gap.
private func railGroupStep(_ vars: ThemeVariables) -> CGFloat {
  vars.spacing15
}
