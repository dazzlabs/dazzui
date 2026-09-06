import DazzUI
import SwiftUI

/// `Components/Callout` — a tinted notice that stays.
enum CalloutStories {
  static var page: StoryPage {
    StoryPage("Callout") {
      Story("Overview", layout: .page) { Overview() }
      Story("Info") {
        Callout(
          "Restart to finish updating to 2.1.0.",
          tint: .info,
          title: "A new version is available"
        )
        .frame(width: 460)
      }
      Story("Success") {
        Callout(
          "Three platform packages were refreshed.",
          tint: .success,
          title: "Tokens rebuilt"
        )
        .frame(width: 460)
      }
      Story("Warning") {
        Callout(
          "color.warning cannot move whole under bright-dark.",
          tint: .warning,
          title: "Ramp moves only part-way"
        )
        .frame(width: 460)
      }
      Story("Danger") {
        Callout(
          "DazzUI: 4 errors in Sidebar.swift.",
          tint: .danger,
          title: "The build failed"
        )
        .frame(width: 460)
      }
      Story("Tints") {
        Col(width: 460) {
          ForEach(Tint.allCases, id: \.self) { tint in
            Callout("The notice reads on any ramp.", tint: tint, title: label(tint))
          }
        }
      }
      Story("Without a title") {
        Callout("The glossary has not been indexed yet.", tint: .info)
          .frame(width: 460)
      }
      Story("With an icon") {
        Notice()
      }
      Story("With an action") {
        Actioned()
      }
      Story("Sizes") {
        Col(width: 460) {
          ForEach(CalloutSize.allCases, id: \.self) { size in
            Callout(
              "The box changes; the type never does.",
              size: size,
              tint: .info,
              title: label(size)
            )
          }
        }
      }
      Story("In a pane") {
        Frame(width: 560, height: 260) {
          Pane {
            Callout(
              "Rebuild the tokens before this page will be right.",
              tint: .warning,
              title: "Stale build"
            )
            Text("Components")
          }
        }
      }
    }
  }

  private struct Notice: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Callout(
        tint: .info,
        title: "A new version is available",
        leading: { Icon(.info16Regular, size: theme.vars.labelMedium.size) },
        description: { Text("Restart to finish updating to 2.1.0.") },
        trailing: { EmptyView() }
      )
      .frame(width: 460)
    }
  }

  private struct Actioned: View {
    var body: some View {
      Callout(
        tint: .warning,
        title: "Tokens are stale",
        leading: { EmptyView() },
        description: { Text("packages/ds changed after the last build.") },
        trailing: { Button("Rebuild", variant: .tinted, tint: .warning) {} }
      )
      .frame(width: 460)
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Callout",
        description: """
          A notice that stays on the page, where a toast passes. It always \
          wears a tinted treatment — a notice that needed a fill louder than \
          this would be a dialog — so `tint` is the only colour axis it has.
          """
      ) {
        Section(
          "Tints",
          description: """
            The fill and the edge are washes of the tint rather than opaque \
            steps, which is what lets the same callout sit correctly on paper, \
            on a card and on a near-black canvas without per-theme values.
            """
        ) {
          Col(width: 560) {
            ForEach(Tint.allCases, id: \.self) { tint in
              Callout(
                "The notice reads on any ramp.",
                tint: tint,
                title: label(tint)
              )
            }
          }
        }

        Section(
          "Shapes",
          description: """
            A title is optional: without one the description carries the whole \
            notice. The leading slot takes a glyph, the trailing slot takes at \
            most one action — and that action is tinted to match, so the notice \
            reads as one object rather than as a banner with a button on it.
            """
        ) {
          Col(width: 560) {
            Callout("The glossary has not been indexed yet.", tint: .info)
            Notice()
            Actioned()
          }
        }

        Section(
          "Sizes",
          description: """
            One shape, three densities. The type never changes, only the box \
            around it — a callout that grew its type would be competing with \
            the content it interrupts.
            """
        ) {
          Col(width: 560) {
            ForEach(CalloutSize.allCases, id: \.self) { size in
              Callout(
                "The box changes; the type never does.",
                size: size,
                tint: .info,
                title: label(size)
              )
            }
          }
        }
      }
    }
  }
}
