import DazzUI
import SwiftUI

/// `Tokens/Color` — the ramps, and the roles that alias them.
enum ColorStories {
  static var page: StoryPage {
    StoryPage("Color") {
      Story("Palette", layout: .page) { Palette() }
      Story("Roles", layout: .page) { Roles() }
    }
  }

  private struct Palette: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Page(
        "Palette",
        description: """
          Six ramps, eleven steps each, in the theme selected above. A ramp is \
          a **role ladder, not a lightness ramp** — read Bright's primary \
          before assuming otherwise: its 700 is *lighter* than its 600, because \
          that theme's primary action is ink navy whose hover lifts. A step is \
          only ever reached by a recipe, so what a step means is the role that \
          recipe gives it.
          """
      ) {
        Section(
          "Ramps",
          description: """
            A component never names a step. This page is the one place they are \
            printed, and it is a reference rather than a menu: a shade written \
            into a component would be a lie rather than merely a duplicate, \
            because the next theme re-points the ladder underneath it.
            """
        ) {
          VStack(alignment: .leading, spacing: theme.vars.spacing5) {
            ForEach(Tint.allCases, id: \.self) { tint in
              Ramp(tint.rawValue, theme.vars.ramp(tint))
            }
          }
        }
      }
    }
  }

  private struct Roles: View {
    @Environment(\.theme) private var theme

    var body: some View {
      let vars = theme.vars

      Page(
        "Roles",
        description: """
          The named colours a component actually reads. Each theme decides its \
          own paper, ink and edges — those are the one kind of raw value the \
          semantic layer holds — and everything else aliases a ramp.
          """
      ) {
        Section(
          "Surfaces",
          description: """
            The canvas the window sits on, the paper a card is, and the grooves \
            under it. Disabled controls flatten to `surface-sunken`, which is \
            why it has to read as a hole rather than as a quieter card.
            """
        ) {
          Grid(min: 200) {
            Swatch("canvas", vars.colorCanvas, note: "The window's ground")
            Swatch("surface", vars.colorSurface, note: "Paper")
            Swatch("surface-muted", vars.colorSurfaceMuted, note: "Sidebar, titlebar")
            Swatch("surface-sunken", vars.colorSurfaceSunken, note: "The groove")
            Swatch("surface-subtle", vars.colorSurfaceSubtle)
            Swatch("surface-inset", vars.colorSurfaceInset)
            Swatch("surface-raised", vars.colorSurfaceRaised, note: "Popups")
            Swatch("surface-chrome", vars.colorSurfaceChrome)
          }
        }

        Section(
          "Ink",
          description: """
            The ladder body copy, labels and hints are set in. Every step is \
            held to a WCAG contrast floor in every theme by a check in the \
            token build, which fails rather than shipping a role that landed \
            under it.
            """
        ) {
          Grid(min: 200) {
            Swatch("content", vars.colorContent, note: "Body")
            Swatch("content-secondary", vars.colorContentSecondary)
            Swatch("content-nav", vars.colorContentNav, note: "Idle nav rows")
            Swatch("content-muted", vars.colorContentMuted, note: "Hints")
            Swatch("content-subtle", vars.colorContentSubtle)
            Swatch("content-faint", vars.colorContentFaint, note: "Disabled")
            Swatch("on-accent", vars.colorOnAccent, note: "On a filled surface")
          }
        }

        Section(
          "Edges",
          description: """
            One hairline, one stronger, one quieter. A separator draws \
            `stroke.hairline` and halves on Retina; a control outline draws \
            `stroke.control` and stays put.
            """
        ) {
          Grid(min: 200) {
            Swatch("border", vars.colorBorder)
            Swatch("border-strong", vars.colorBorderStrong)
            Swatch("border-muted", vars.colorBorderMuted)
          }
        }
      }
    }
  }
}
