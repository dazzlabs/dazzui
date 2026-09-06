import DazzUI
import SwiftUI

/// `Components/Card` — a surface, and nothing that answers the pointer.
enum CardStories {
  static var page: StoryPage {
    StoryPage("Card") {
      Story("Overview", layout: .page) { Overview() }
      Story("Raised") { Sample(variant: .raised) }
      Story("Sunken") { Sample(variant: .sunken) }
      Story("Outlined") { Sample(variant: .outlined) }
      Story("Tinted") { Sample(variant: .tinted) }
      Story("Variants") {
        Row {
          ForEach(CardVariant.allCases, id: \.self) { variant in
            Sample(variant: variant)
          }
        }
      }
      Story("Tints") {
        Row {
          ForEach(Tint.allCases, id: \.self) { tint in
            Card(variant: .tinted, tint: tint) {
              Text(label(tint)).frame(width: 96, alignment: .leading)
            }
          }
        }
      }
      Story("Sizes") {
        Row(alignment: .top) {
          ForEach(CardSize.allCases, id: \.self) { size in
            Card(size: size) {
              Text(label(size)).frame(width: 96, alignment: .leading)
            }
          }
        }
      }
      Story("With an action") {
        Card {
          Blurb(
            title: "Tokens are stale",
            note: "packages/ds changed after the last build."
          )
          ActionBar(align: .end) {
            Button("Rebuild", variant: .filled) {}
          }
        }
        .frame(width: 320)
      }
      Story("As a grid") {
        Grid(min: 200) {
          ForEach(["ui_react", "ui_flutter", "ui_gpui", "ui_swiftui"], id: \.self) { name in
            Card {
              Blurb(title: name, note: "Reads what ds wrote.")
            }
          }
        }
        .frame(width: 640)
      }
      Story("As a well") {
        Card(variant: .sunken) {
          Text("swift build")
            .font(.system(.body, design: .monospaced))
        }
        .frame(width: 320)
      }
      Story("Nested") {
        Card(variant: .sunken) {
          Blurb(title: "Queue", note: "Two jobs waiting.")
          Card {
            Blurb(title: "aurora", note: "main · 2m 14s")
          }
          Card {
            Blurb(title: "beacon", note: "release/2.1 · queued")
          }
        }
        .frame(width: 340)
      }
    }
  }

  /// One card of each variant, with the same thing inside it.
  private struct Sample: View {
    let variant: CardVariant

    var body: some View {
      Card(variant: variant) {
        Blurb(title: label(variant), note: "A surface.")
      }
      .frame(width: 180)
    }
  }

  private struct Blurb: View {
    let title: String
    let note: String

    @Environment(\.theme) private var theme

    var body: some View {
      let vars = theme.vars

      VStack(alignment: .leading, spacing: vars.spacing1) {
        Text(title)
          .font(vars.labelMedium.font)
          .foregroundStyle(vars.colorContent)
        Text(note)
          .font(vars.captionSmall.font)
          .foregroundStyle(vars.colorContentMuted)
      }
      .frame(maxWidth: .infinity, alignment: .leading)
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Card",
        description: """
          A surface with a corner and a pad. It is not interactive and stays \
          out of the shared state machine: `:hover` would fire on it just the \
          same, and a panel that lit up under the pointer would read as \
          something you can press. Where a card has to be selectable, that is \
          an `OptionCard`.
          """
      ) {
        Section(
          "Variants",
          description: """
            `raised` is the paper card on the canvas; `sunken` is the groove a \
            well or a code block sits in; `outlined` is the drawn edge with no \
            fill; `tinted` is the wash that carries a status.
            """
        ) {
          Row(alignment: .top) {
            ForEach(CardVariant.allCases, id: \.self) { variant in
              Sample(variant: variant)
            }
          }
        }

        Section(
          "Tints",
          description: """
            Only `tinted` reads a ramp — the others are the theme's own surface \
            roles, so a tint leaves them where they are. The wash is an alpha \
            of the tint rather than an opaque step, which is what lets the same \
            card sit correctly on paper and on a near-black canvas.
            """
        ) {
          Row {
            ForEach(Tint.allCases, id: \.self) { tint in
              Card(variant: .tinted, tint: tint) {
                Text(label(tint)).frame(width: 88, alignment: .leading)
              }
            }
          }
        }

        Section(
          "Sizes",
          description: """
            Three pads on one corner. A card's size is the density of what it \
            holds, not the importance of it.
            """
        ) {
          Row(alignment: .top) {
            ForEach(CardSize.allCases, id: \.self) { size in
              Card(size: size) {
                Text(label(size)).frame(width: 96, alignment: .leading)
              }
            }
          }
        }

        Section(
          "Compositions",
          description: """
            A card holds a heading, its detail and at most one action row. A \
            sunken card inside a raised one is the well pattern — the groove \
            reads as inside the panel rather than as a second panel.
            """
        ) {
          Row(alignment: .top) {
            Card {
              Blurb(
                title: "Tokens are stale",
                note: "packages/ds changed after the last build."
              )
              ActionBar(align: .end) {
                Button("Rebuild", variant: .filled) {}
              }
            }
            .frame(width: 300)

            Card(variant: .sunken) {
              Blurb(title: "Queue", note: "Two jobs waiting.")
              Card { Blurb(title: "aurora", note: "main · 2m 14s") }
              Card { Blurb(title: "beacon", note: "release/2.1 · queued") }
            }
            .frame(width: 300)
          }
        }
      }
    }
  }
}
