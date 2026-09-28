import DazzUI
import SwiftUI

/// `Tokens/Radius` — one corner scale, and where each step lands.
enum RadiusStories {
  static var page: StoryPage {
    StoryPage("Radius") {
      Story("Scale", layout: .page) { Scale() }
    }
  }

  private struct Scale: View {
    @Environment(\.theme) private var theme

    var body: some View {
      let vars = theme.vars

      Page(
        "Radius",
        description: """
          One scale per axis, so there is no second radius to alias by \
          accident. The corners are the AppKit metrics the design is drawn to — \
          which is why Bright's pill controls are a theme re-pointing this \
          scale rather than a component choosing a different one.
          """
      ) {
        Section(
          "Steps",
          description: "Each drawn on the surface a component of that corner sits on."
        ) {
          Grid(min: 180) {
            ForEach(steps, id: \.0) { name, radius, note in
              Tile(name, note: note) {
                RoundedRectangle(cornerRadius: radius, style: .circular)
                  .fill(vars.ramp(.primary)[500])
                  .frame(height: vars.spacing16)
                  .frame(maxWidth: .infinity)
              }
            }
          }
        }
      }
    }

    private var steps: [(String, CGFloat, String)] {
      let vars = theme.vars
      return [
        ("radius.none", vars.radiusNone, "Square"),
        ("radius.tiny", vars.radiusTiny, "A badge, a swatch"),
        ("radius.small", vars.radiusSmall, "Controls, nav rows"),
        ("radius.medium", vars.radiusMedium, "Cards, popups"),
        ("radius.large", vars.radiusLarge, "The large control"),
        ("radius.big", vars.radiusBig, "A window"),
        ("radius.full", vars.radiusFull, "A pill"),
      ]
    }
  }
}
