import DazzUI
import SwiftUI

/// `Components/Autocomplete` — a text field that suggests as you type.
enum AutocompleteStories {
  static let branches = [
    "main",
    "release/4.2",
    "release/4.1",
    "fix/token-drift",
    "fix/popup-layer",
    "feat/slider",
    "feat/drawer",
  ]

  static let tags = ["design-system", "tokens", "flutter", "gpui", "swiftui", "parity"]

  static var page: StoryPage {
    StoryPage("Autocomplete") {
      Story("Overview", layout: .page) { Overview() }
      Story("Bare") { Live(width: 256, items: tags, placeholder: "Add a tag") }
      Story("Mono") { Live(width: 256, items: branches, placeholder: "Branch", mono: true) }
      Story("Sizes") {
        Col(width: 256) {
          ForEach(TextFieldSize.allCases, id: \.self) { size in
            Live(items: tags, size: size)
          }
        }
      }
      Story("States") {
        Col(width: 256) {
          Live(items: tags, start: "tokens")
          Live(items: tags, start: "tok", state: .error)
          Autocomplete(items: tags, text: .constant("tokens"), enabled: false)
        }
      }
      Story("In a form") { Branch() }
    }
  }

  struct Live: View {
    var width: CGFloat?
    var items: [String]
    var start = ""
    var placeholder = ""
    var size: TextFieldSize = .medium
    var state: TextFieldState = .default
    var mono = false

    @State private var text: String?

    var body: some View {
      Autocomplete(
        items: items,
        text: Binding(get: { text ?? start }, set: { text = $0 }),
        placeholder: placeholder,
        size: size,
        state: state,
        mono: mono
      )
      .frame(width: width)
    }
  }

  private struct Branch: View {
    @State private var branch = "feat/"

    private var known: Bool { AutocompleteStories.branches.contains(branch) }

    var body: some View {
      FormField(
        label: "Branch",
        hint: known ? "Checks out the existing branch" : "Creates a new branch"
      ) {
        Autocomplete(items: AutocompleteStories.branches, text: $branch, mono: true)
      }
      .frame(width: 320)
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Autocomplete",
        description:
          "A text field that suggests as you type. The difference from a Combobox is what the "
          + "value is: a combobox ends in one of its options and nothing else; an "
          + "autocomplete’s value is whatever was typed, and the list is a shortcut to a likely "
          + "answer rather than the set of allowed ones. A branch name, a search query, a tag "
          + "that may not exist yet."
      ) {
        Section(
          "Sizes",
          description:
            "It is a TextField wearing the shared popup sheet, and it is literally that: the "
            + "same box, the same panel, the same rows. There is no chevron and no clear — "
            + "nothing here is opened or chosen, and the field is emptied the way any text "
            + "field is."
        ) {
          Stack {
            ForEach(TextFieldSize.allCases, id: \.self) { size in
              Specimen(label(size)) {
                Live(width: 224, items: AutocompleteStories.branches, size: size, mono: true)
              }
            }
          }
        }
        Section(
          "Against a Combobox",
          description:
            "Ask what happens when the reader types something that is not on the list. If that "
            + "is an answer, this is the view. If it is a mistake, it is a Combobox."
        ) {
          Live(
            width: 256,
            items: AutocompleteStories.branches,
            placeholder: "Branch or new name",
            mono: true
          )
        }
        Section(
          "States",
          description:
            "Erroring re-tints to danger like every other text control, so the border, the wash "
            + "and the value’s ink move together."
        ) {
          Stack {
            Specimen("error") {
              Live(width: 224, items: AutocompleteStories.tags, start: "tok", state: .error)
            }
            Specimen("disabled") {
              Autocomplete(
                items: AutocompleteStories.tags,
                text: .constant("tokens"),
                enabled: false
              )
              .frame(width: 224)
            }
          }
        }
      }
    }
  }
}
