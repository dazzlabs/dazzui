import DazzUI
import SwiftUI

/// `Components/OptionCard` — a card you can choose.
enum OptionCardStories {
  static var page: StoryPage {
    StoryPage("OptionCard") {
      Story("Overview", layout: .page) { Overview() }
      Story("Selected") {
        OptionCard(
          "Automatic",
          description: "Follow the system appearance.",
          selected: true
        ) {}
        .frame(width: 240)
      }
      Story("Unselected") {
        OptionCard(
          "Manual",
          description: "Pick a theme and keep it.",
          selected: false
        ) {}
        .frame(width: 240)
      }
      Story("A group") { Appearance() }
      Story("Without a description") {
        Row {
          OptionCard("Light", selected: true) {}
          OptionCard("Dark", selected: false) {}
        }
      }
      Story("Disabled") {
        Row(alignment: .top) {
          OptionCard(
            "Team",
            description: "Everyone in the workspace.",
            selected: false
          ) {}
          .frame(width: 220)
          OptionCard(
            "Enterprise",
            description: "Not on this plan.",
            selected: false,
            enabled: false
          ) {}
          .frame(width: 220)
        }
      }
      Story("Tints") {
        Row(alignment: .top) {
          ForEach(Tint.allCases, id: \.self) { tint in
            OptionCard(label(tint), selected: true, tint: tint) {}
              .frame(width: 130)
          }
        }
      }
      Story("In a grid") {
        Grid(min: 200) {
          ForEach(Platform.all) { platform in
            OptionCard(
              platform.name,
              description: platform.note,
              selected: platform.id == "swiftui"
            ) {}
          }
        }
        .frame(width: 640)
      }
    }
  }

  private struct Platform: Identifiable {
    let id: String
    let name: String
    let note: String

    static let all = [
      Platform(id: "react", name: "React", note: "Base UI under the drawing."),
      Platform(id: "flutter", name: "Flutter", note: "One theme value, no cascade."),
      Platform(id: "swiftui", name: "SwiftUI", note: "Per-subtree themes."),
      Platform(id: "gpui", name: "GPUI", note: "A seed and a const fn."),
    ]
  }

  private struct Appearance: View {
    @State private var mode = "auto"

    var body: some View {
      Row(alignment: .top) {
        OptionCard(
          "Automatic",
          description: "Follow the system appearance.",
          selected: mode == "auto"
        ) { mode = "auto" }
        .frame(width: 220)

        OptionCard(
          "Manual",
          description: "Pick a theme and keep it.",
          selected: mode == "manual"
        ) { mode = "manual" }
        .frame(width: 220)
      }
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "OptionCard",
        description: """
          The choosable card: a radio's job in a card's shape, for a decision \
          that needs a sentence of explanation rather than a word. It is \
          interactive, so unlike `Card` it is in the shared state machine and \
          answers the pointer.
          """
      ) {
        Section(
          "Selection",
          description: """
            Selection shifts weight as well as colour — the title goes to the \
            strong label face, the edge takes the tint, and the fill takes the \
            tint's wash. That is what makes a chosen card readable at a glance \
            in a grid of four.
            """
        ) {
          Appearance()
        }

        Section(
          "Shapes",
          description: """
            The description is optional: without one the card is a wide radio \
            row, which is the right shape for two short answers. Disabled \
            flattens to the groove, like every other control.
            """
        ) {
          Row(alignment: .top) {
            OptionCard("Light", selected: true) {}
            OptionCard("Dark", selected: false) {}
            OptionCard(
              "Enterprise",
              description: "Not on this plan.",
              selected: false,
              enabled: false
            ) {}
            .frame(width: 220)
          }
        }

        Section(
          "In a grid",
          description: """
            Where the options are more than two, they go in a grid that fills \
            the width rather than in a row that runs off it.
            """
        ) {
          Grid(min: 200) {
            ForEach(Platform.all) { platform in
              OptionCard(
                platform.name,
                description: platform.note,
                selected: platform.id == "swiftui"
              ) {}
            }
          }
        }
      }
    }
  }
}
