import DazzUI
import SwiftUI

/// A theme in miniature: a window of its own, drawn in that theme's colours
/// — the column, the paper, two lines of its ink, a button in its accent.
struct ThemePreview: View {
  let theme: ThemeChoice
  var height: CGFloat?
  /// A framed preview has its own border and corner; an unframed one runs to
  /// its container's edge and draws only the rule under it.
  var framed = true

  @Environment(\.theme) private var page
  @Environment(\.hairline) private var hairline

  var body: some View {
    let outer = page.vars
    let vars = theme.name.variables
    let content = GeometryReader { proxy in
      HStack(spacing: 0) {
        vars.colorSurfaceColumn
          .frame(width: proxy.size.width * 0.28)
          .overlay(alignment: .trailing) {
            Rectangle().fill(vars.colorBorderStrong).frame(width: hairline)
          }
        VStack(alignment: .leading, spacing: outer.spacing1) {
          ink(vars.colorContent)
          ink(vars.colorContent).opacity(0.4).frame(width: (proxy.size.width * 0.72 - pad * 2) * 0.6)
          Spacer(minLength: 0)
          RoundedRectangle(cornerRadius: outer.spacing1)
            .fill(vars.colorPrimary[500])
            .frame(width: outer.spacing6, height: outer.spacing25)
        }
        .padding(pad)
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
      }
    }
    .frame(height: height ?? outer.spacing14)
    .background(vars.colorSurface)

    if framed {
      content
        .clipShape(RoundedRectangle(cornerRadius: outer.spacing15))
        .border(vars.colorBorderStrong, width: hairline, cornerRadius: outer.spacing15)
    } else {
      content.overlay(alignment: .bottom) {
        Rectangle().fill(outer.colorBorder).frame(height: hairline)
      }
    }
  }

  private var pad: CGFloat { framed ? page.vars.spacing2 : page.vars.spacing15 }

  private func ink(_ color: Color) -> some View {
    RoundedRectangle(cornerRadius: 1)
      .fill(color)
      .frame(height: page.vars.spacing05)
  }
}
