import SwiftUI

/// The edge a ``Drawer`` comes in from.
public enum DrawerSide: String, CaseIterable, Sendable {
  case leading
  case trailing
  case top
  case bottom
}

/// A sheet pinned to one edge of the view.
///
/// It is the ``Dialog``'s sibling, and the choice between them is about where
/// the reader's attention has to go rather than about the screen it is on: a
/// dialog takes the middle of the view and asks a question; a drawer takes a
/// side of it and holds a panel the reader dips into — filters, a detail
/// pane — while what they were reading stays where it was.
///
/// The header and the footer are the dialog's own parts rather than a second
/// set: a drawer and a sheet in the same app are set the same way.
public struct Drawer<Content: View, Footer: View>: View {
  public var side: DrawerSide
  /// Overrides the token width — or height, on a top or bottom drawer.
  public var size: CGFloat?
  public var title: String?
  public var description: String?
  /// Whether the body scrolls. Off, it takes the height the sheet leaves it
  /// — a list or an editor inside can fill that and scroll itself.
  public var scrolls: Bool
  @ViewBuilder public var footer: () -> Footer
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  public init(
    side: DrawerSide = .trailing,
    size: CGFloat? = nil,
    title: String? = nil,
    description: String? = nil,
    scrolls: Bool = false,
    @ViewBuilder footer: @escaping () -> Footer,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.side = side
    self.size = size
    self.title = title
    self.description = description
    self.scrolls = scrolls
    self.footer = footer
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars
    let extent = size ?? vars.drawerSize

    VStack(spacing: 0) {
      // The grabber, on the two edges a thumb actually pushes. It is the one
      // mark that says the sheet can be pushed away.
      if !horizontal {
        Capsule(style: .circular)
          .fill(vars.colorBorderStrong)
          .frame(width: vars.spacing9, height: vars.spacing1)
          .padding(.vertical, vars.spacing2)
      }
      if let title {
        DialogHeader(title, subtitle: description)
      }
      // Only the body scrolls, so a long panel keeps its heading and its
      // actions in view — and only when asked, since a scroll view offers
      // its content no height to fill.
      if scrolls {
        ScrollView { bodyStack(vars) }
      } else {
        bodyStack(vars)
          .frame(maxHeight: .infinity, alignment: .top)
      }
      // No band unless there is something to put in it.
      if Footer.self != EmptyView.self {
        DialogFooter { footer() }
      }
    }
    .frame(
      maxWidth: horizontal ? extent : .infinity,
      maxHeight: horizontal ? .infinity : extent
    )
    .background(vars.colorSurfaceOverlay)
    // It keeps a corner only on the two sides still inside the view. A sheet
    // flush to an edge with all four corners rounded reads as floating just
    // off the screen rather than as attached to it.
    .clipShape(DrawerShape(side: side, corner: vars.frameWindowRadius))
    .rule(vars.colorBorderStrong, edge: flushEdge, width: hairline)
    .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: alignment)
  }

  private func bodyStack(_ vars: ThemeVariables) -> some View {
    VStack(alignment: .leading, spacing: vars.spacing4) {
      content()
    }
    .frame(maxWidth: .infinity, alignment: .leading)
    .padding(.vertical, vars.spacing4 + vars.spacing05)
    .padding(.horizontal, vars.spacing5)
  }

  private var horizontal: Bool { side == .leading || side == .trailing }

  private var alignment: Alignment {
    switch side {
    case .leading: return .leading
    case .trailing: return .trailing
    case .top: return .top
    case .bottom: return .bottom
    }
  }

  /// The edge the sheet is flush to, which is the one that gets the rule.
  private var flushEdge: Edge {
    switch side {
    case .leading: return .trailing
    case .trailing: return .leading
    case .top: return .bottom
    case .bottom: return .top
    }
  }
}

public extension Drawer where Footer == EmptyView {
  /// A drawer with no action band.
  init(
    side: DrawerSide = .trailing,
    size: CGFloat? = nil,
    title: String? = nil,
    description: String? = nil,
    scrolls: Bool = false,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.init(
      side: side,
      size: size,
      title: title,
      description: description,
      scrolls: scrolls,
      footer: { EmptyView() },
      content: content
    )
  }
}

/// The two corners a drawer keeps: the ones still inside the view.
private struct DrawerShape: Shape {
  let side: DrawerSide
  let corner: CGFloat

  func path(in rect: CGRect) -> Path {
    let radii: RectangleCornerRadii = {
      switch side {
      case .trailing: return .init(topLeading: corner, bottomLeading: corner)
      case .leading: return .init(bottomTrailing: corner, topTrailing: corner)
      case .top: return .init(bottomLeading: corner, bottomTrailing: corner)
      case .bottom: return .init(topLeading: corner, topTrailing: corner)
      }
    }()
    return Path(
      UnevenRoundedRectangle(cornerRadii: radii, style: .circular).path(in: rect).cgPath
    )
  }
}
