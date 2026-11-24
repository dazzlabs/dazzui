import SwiftUI

/// The trailing information column.
///
/// It carries what the current selection *is*, where the sidebar and the rail
/// carry what you can navigate to — which is why it hangs off the opposite
/// edge and takes no rows. It shares the sidebar's surface rather than the
/// rail's, so a window reads as content between two quieter edges rather than
/// as three different greys.
///
/// What it holds is the app's: where the sidebar is a column of rows and owns
/// their rhythm, this is a panel — a title, a list of its own, whatever the
/// selection is about. And it collapses the way the leading column does,
/// mirrored: the panel keeps the width it has when open and is cut by the edge
/// that faces the content as it shuts, so what it holds holds still against
/// the window's own edge while the divider slides over it. Gone, it is clipped
/// out of the pointer and the accessibility tree with it.
public struct Aside<Header: View, Content: View>: View {
  /// The width it has when open. Leave it out and the column takes the token's.
  public var width: CGFloat?
  /// Whether the panel is put away. The host owns the state and makes the
  /// change inside an animation — `withAnimation(theme.vars.motion)` — so
  /// the pane beside it takes its space frame by frame, the same contract
  /// the leading ``Sidebar`` has.
  public var collapsed: Bool
  /// Optional pinned header strip aligned to `vars.frameTitlebarSize`, matching
  /// ``Sidebar``'s header strip.
  public var header: (() -> Header)?
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  public init(
    width: CGFloat? = nil,
    collapsed: Bool = false,
    @ViewBuilder header: @escaping () -> Header,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.width = width
    self.collapsed = collapsed
    self.header = header
    self.content = content
  }

  init(
    width: CGFloat?,
    collapsed: Bool,
    headerBuilder: (() -> Header)?,
    content: @escaping () -> Content
  ) {
    self.width = width
    self.collapsed = collapsed
    self.header = headerBuilder
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars
    let open = width ?? vars.frameAsideWidth

    VStack(spacing: 0) {
      if let header {
        HStack(spacing: 0) {
          header()
          Spacer(minLength: 0)
        }
        .padding(.horizontal, vars.spacing4)
        .frame(height: vars.frameTitlebarSize)
        .rule(vars.colorBorder, edge: .bottom, width: hairline)
      }
      content()
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .top)
    }
    // Laid out at the width it has when open, so a panel that is closing is
    // cut rather than wrapped in front of the reader.
    .frame(width: open)
    .frame(maxHeight: .infinity, alignment: .top)
    .background(vars.colorSurfaceColumn)
    .rule(vars.colorBorder, edge: .leading, width: hairline)
    // Against the window's own edge, which is the mirror of the sidebar's
    // leading anchor: the cut comes off the side facing the content.
    .frame(width: collapsed ? 0 : open, alignment: .trailing)
    .clipped()
    .allowsHitTesting(!collapsed)
    .accessibilityHidden(collapsed)
  }
}

public extension Aside where Header == EmptyView {
  /// An aside panel with no pinned header strip.
  init(
    width: CGFloat? = nil,
    collapsed: Bool = false,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.init(width: width, collapsed: collapsed, headerBuilder: nil, content: content)
  }
}
