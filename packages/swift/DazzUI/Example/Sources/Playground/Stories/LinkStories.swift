import DazzUI
import SwiftUI

/// `Components/Link` — text that goes somewhere.
enum LinkStories {
  static var page: StoryPage {
    StoryPage("Link") {
      Story("Overview", layout: .page) { Overview() }
      Story("Underlines") {
        VStack(alignment: .leading, spacing: 8) {
          Link("Open the changelog", underline: .hover)
          Link("Open the changelog", underline: .always)
          Link("Open the changelog", underline: .none)
        }
      }
      Story("External") { Link("Base UI", external: true) }
      Story("Tints") {
        VStack(alignment: .leading, spacing: 8) {
          ForEach(Tint.allCases, id: \.self) { tint in
            Link("Continue", tint: tint)
          }
        }
      }
      Story("Disabled") { Link("Not available yet", enabled: false) }
      Story("In prose") { prose.frame(width: 416) }
    }
  }

  /// SwiftUI lays a paragraph out as one text run, and a view cannot sit inside
  /// one — so prose with links in it is a wrapping row of runs. The React page
  /// reads as a sentence; this one reads as its parts.
  static var prose: some View {
    HStack(alignment: .firstTextBaseline, spacing: 4) {
      Text("Behaviour comes from")
      Link("Base UI")
      Text("and the drawing is ours — or open the")
      Link("upstream docs", external: true)
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Link",
        description:
          "Text that goes somewhere. It is the one interactive thing in this package that is "
          + "genuinely inline, which is why it exists beside Button(variant: .plain) rather "
          + "than inside it: a button keeps its box at every variant, and a box is what a link "
          + "in a sentence must not have. It takes the shared state machine for its colour — "
          + "the plain recipe, a grade darker on hover — and none of its geometry."
      ) {
        Section(
          "Underline",
          description:
            "hover is the default, because a paragraph with a rule under every link reads as "
            + "struck through. always is for a link that has to be findable without a pointer "
            + "— a list of references, a footer. none is for one whose surroundings already "
            + "say it is a link."
        ) {
          Stack {
            Specimen("hover") { Link("Open the changelog", underline: .hover) }
            Specimen("always") { Link("Open the changelog", underline: .always) }
            Specimen("none") { Link("Open the changelog", underline: .none) }
          }
        }
        Section(
          "External",
          description:
            "Goes somewhere outside this window, and draws the arrow that says so before the "
            + "reader commits."
        ) {
          Stack {
            Specimen("same window") { Link("Release notes") }
            Specimen("external") { Link("Base UI", external: true) }
          }
        }
        Section(
          "Tints",
          description:
            "A link is accent text, so it reads the plain recipe's content grade off whichever "
            + "ramp it is tinted with. Danger is the one worth reaching for — a link that "
            + "deletes something."
        ) {
          VStack(alignment: .leading, spacing: 8) {
            ForEach(Tint.allCases, id: \.self) { tint in
              Link("Continue", tint: tint)
            }
          }
        }
      }
    }
  }
}
