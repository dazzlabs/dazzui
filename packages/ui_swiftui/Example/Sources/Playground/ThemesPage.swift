import DazzUI
import SwiftUI

/// The four themes at once.
///
/// This is the page that argues for the design of the thing: a theme is a
/// value scoped to a subtree with `.theme(_:)`, not a process-wide mode, so
/// four of them can sit side by side and every component inside each one
/// resolves against its own variables. The cards below are the same view
/// rendered four times.
struct ThemesPage: View {
  @Environment(\.theme) private var theme

  private let columns = [
    GridItem(.adaptive(minimum: 300), spacing: 16)
  ]

  var body: some View {
    let vars = theme.vars

    PageTitle(
      "Themes",
      "Four themes, one component tree — each card scopes its own with .theme(_:)."
    )

    LazyVGrid(columns: columns, spacing: vars.spacing4) {
      ForEach(ThemeName.allCases, id: \.self) { name in
        ThemeCard(name: name)
          .theme(name)
      }
    }

    Specimen(
      "Ramps",
      note: "a role ladder, not a lightness ramp — read the ink ladder before assuming"
    ) {
      VStack(alignment: .leading, spacing: vars.spacing2) {
        ForEach(Tint.allCases, id: \.self) { tint in
          RampRow(tint: tint)
        }
      }
    }

    Spacer(minLength: vars.spacing8)
  }
}

/// One theme's own little window: the surfaces stacked, a control on each, and
/// the accent that separates the families.
private struct ThemeCard: View {
  let name: ThemeName

  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars

    VStack(alignment: .leading, spacing: vars.spacing3) {
      HStack {
        Text(name.title)
          .font(vars.labelLarge.font)
          .foregroundStyle(vars.colorContent)
        Spacer()
        Badge(name.rawValue, size: .small, variant: .tinted)
      }

      Text("The quick brown fox jumps over the lazy dog.")
        .font(vars.bodySmall.font)
        .lineSpacing(vars.bodySmall.lineSpacing)
        .foregroundStyle(vars.colorContentMuted)

      Row {
        Button("Primary", variant: .filled) {}
        Button("Quiet", variant: .plain) {}
      }

      Divider()

      Row {
        Badge("Passed", size: .small, variant: .tinted, tint: .success)
        Badge("Failed", size: .small, variant: .tinted, tint: .danger)
        Kbd("⌘S")
      }

      Progress(value: 0.6, gradient: true)
    }
    .padding(vars.spacing4)
    .frame(maxWidth: .infinity, alignment: .leading)
    .background(vars.colorSurface)
    .clipShape(RoundedRectangle(cornerRadius: vars.frameWindowRadius, style: .continuous))
    .border(vars.colorBorder, width: vars.spacingPx, cornerRadius: vars.frameWindowRadius)
    .tokenShadow(vars.shadowSm)
  }
}

/// The eleven steps of one ramp, in the active theme.
private struct RampRow: View {
  let tint: Tint

  @Environment(\.theme) private var theme

  private let shades = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950]

  var body: some View {
    let vars = theme.vars
    let ramp = vars.ramp(tint)

    HStack(spacing: vars.spacing1) {
      Text(tint.rawValue)
        .font(vars.captionSmall.font)
        .foregroundStyle(vars.colorContentMuted)
        .frame(width: vars.spacing16, alignment: .leading)

      ForEach(shades, id: \.self) { shade in
        RoundedRectangle(cornerRadius: vars.radiusTiny, style: .continuous)
          .fill(ramp[shade])
          .frame(width: vars.spacing8, height: vars.spacing6)
          .border(vars.colorBorder, width: vars.spacingPx, cornerRadius: vars.radiusTiny)
      }
    }
  }
}
