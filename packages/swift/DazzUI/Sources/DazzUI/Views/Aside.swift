import SwiftUI

/// The trailing information column.
///
/// It carries what the current selection *is*, where the sidebar and the rail
/// carry what you can navigate to — which is why it hangs off the opposite
/// edge and takes no rows. It shares the sidebar's surface rather than the
/// rail's, so a window reads as content between two quieter edges rather than
/// as three different greys.
public struct Aside<Content: View>: View {
  @ViewBuilder public var content: () -> Content

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  public init(@ViewBuilder content: @escaping () -> Content) {
    self.content = content
  }

  public var body: some View {
    let vars = theme.vars

    ScrollView(.vertical) {
      VStack(alignment: .leading, spacing: vars.spacing5) {
        content()
      }
      .frame(maxWidth: .infinity, alignment: .leading)
      .padding(vars.spacing4)
    }
    .frame(width: vars.frameAsideWidth)
    .background(vars.colorSurfaceColumn)
    .rule(vars.colorBorder, edge: .leading, width: hairline)
  }
}
