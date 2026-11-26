import DazzUI
import SwiftUI

/// The five colours a miniature theme preview paints — avoids copying the full
/// `ThemeVariables` struct for every tile in the theme panel.
struct ThemePreviewPalette: Equatable {
  let surface: Color
  let surfaceColumn: Color
  let borderStrong: Color
  let content: Color
  let accent: Color

  init(vars: ThemeVariables) {
    self.surface = vars.colorSurface
    self.surfaceColumn = vars.colorSurfaceColumn
    self.borderStrong = vars.colorBorderStrong
    self.content = vars.colorContent
    self.accent = vars.colorPrimary[500]
  }

  private static let byName: [ThemeName: ThemePreviewPalette] = Dictionary(
    uniqueKeysWithValues: ThemeName.allCases.map { ($0, ThemePreviewPalette(vars: $0.variables)) }
  )

  static func forTheme(_ name: ThemeName) -> ThemePreviewPalette {
    byName[name] ?? ThemePreviewPalette(vars: name.variables)
  }
}

/// Splits a preview box into a 28% sidebar column and 72% body without a
/// `GeometryReader`.
private struct SplitRatioLayout: Layout {
  let leadingFraction: CGFloat

  func sizeThatFits(proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) -> CGSize {
    CGSize(
      width: proposal.width ?? 0,
      height: proposal.height ?? 0
    )
  }

  func placeSubviews(
    in bounds: CGRect, proposal: ProposedViewSize, subviews: Subviews, cache: inout ()
  ) {
    guard subviews.count == 2 else { return }
    let leadingWidth = bounds.width * leadingFraction
    let trailingWidth = max(0, bounds.width - leadingWidth)
    subviews[0].place(
      at: bounds.origin,
      proposal: ProposedViewSize(width: leadingWidth, height: bounds.height)
    )
    subviews[1].place(
      at: CGPoint(x: bounds.minX + leadingWidth, y: bounds.minY),
      proposal: ProposedViewSize(width: trailingWidth, height: bounds.height)
    )
  }
}

/// Places a single child at a fraction of the offered width without a
/// `GeometryReader`.
private struct FractionWidthLayout: Layout {
  let fraction: CGFloat

  func sizeThatFits(proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) -> CGSize {
    let childWidth: CGFloat? = proposal.width.map { $0.isFinite ? $0 * fraction : $0 }
    let child = subviews.first?.sizeThatFits(
      ProposedViewSize(width: childWidth, height: proposal.height)
    ) ?? .zero
    return CGSize(width: proposal.width ?? child.width, height: child.height)
  }

  func placeSubviews(
    in bounds: CGRect, proposal: ProposedViewSize, subviews: Subviews, cache: inout ()
  ) {
    let width = bounds.width.isFinite ? bounds.width * fraction : 0
    subviews.first?.place(
      at: bounds.origin,
      proposal: ProposedViewSize(width: width, height: bounds.height)
    )
  }
}

/// A theme in miniature: a window of its own, drawn in that theme's colours
/// — the column, the paper, two lines of its ink, a button in its accent.
struct ThemePreview: View {
  let palette: ThemePreviewPalette
  var height: CGFloat?
  /// A framed preview has its own border and corner; an unframed one runs to
  /// its container's edge and draws only the rule under it.
  var framed = true

  init(palette: ThemePreviewPalette, height: CGFloat? = nil, framed: Bool = true) {
    self.palette = palette
    self.height = height
    self.framed = framed
  }

  init(vars: ThemeVariables, height: CGFloat? = nil, framed: Bool = true) {
    self.init(palette: ThemePreviewPalette(vars: vars), height: height, framed: framed)
  }

  @Environment(\.theme) private var page

  var body: some View {
    let outer = page.vars
    let hairline = outer.strokeHairline
    let content = SplitRatioLayout(leadingFraction: 0.28) {
      palette.surfaceColumn
        .overlay(alignment: .trailing) {
          Rectangle().fill(palette.borderStrong).frame(width: hairline)
        }
      VStack(alignment: .leading, spacing: outer.spacing1) {
        ink(palette.content)
        FractionWidthLayout(fraction: 0.6) {
          ink(palette.content).opacity(0.4)
        }
        Spacer(minLength: 0)
        RoundedRectangle(cornerRadius: outer.spacing1)
          .fill(palette.accent)
          .frame(width: outer.spacing6, height: outer.spacing25)
      }
      .padding(pad)
      .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
    }
    .frame(height: height ?? outer.spacing14)
    .background(palette.surface)

    if framed {
      content
        .clipShape(RoundedRectangle(cornerRadius: outer.spacing15))
        .border(palette.borderStrong, width: hairline, cornerRadius: outer.spacing15)
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
