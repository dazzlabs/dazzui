import DazzUI
import SwiftUI

/// `Components/Select` — one of many, in a field's box.
enum SelectStories {
  static var page: StoryPage {
    StoryPage("Select") {
      Story("Overview", layout: .page) { Overview() }
      Story("Default") { Region() }
      Story("Disabled") { Region(enabled: false) }
      Story("Error state") { Region(state: .error) }
      Story("Sizes") {
        Col(width: 260) {
          ForEach(TextFieldSize.allCases, id: \.self) { size in
            Region(size: size)
          }
        }
      }
      Story("Tints") {
        Col(width: 260) {
          ForEach(Tint.allCases, id: \.self) { tint in
            Region(tint: tint)
          }
        }
      }
      Story("Beside a button") {
        Row {
          Region()
          Button("Apply", variant: .filled) {}
        }
      }
      Story("In a form field") {
        FormField(label: "Region", hint: "Where the documents are stored.") {
          Region()
        }
        .frame(width: 320)
      }
      Story("In a settings row") {
        PreferenceSection(label: "Appearance") {
          PreferenceRow(title: "Theme", trailing: { Appearance(size: .small) })
        }
        .frame(width: 460)
      }
    }
  }

  private struct Region: View {
    var size: TextFieldSize = .medium
    var state: TextFieldState = .default
    var tint: Tint = .primary
    var enabled = true

    @State private var region = "us"

    var body: some View {
      Select(
        options: [
          SelectOption(value: "us", label: "United States"),
          SelectOption(value: "eu", label: "Europe"),
          SelectOption(value: "ap", label: "Asia Pacific"),
          SelectOption(value: "sa", label: "South America", enabled: false),
        ],
        selection: $region,
        size: size,
        state: state,
        tint: tint,
        enabled: enabled,
        fullWidth: true
      )
      .frame(width: 220)
    }
  }

  private struct Appearance: View {
    var size: TextFieldSize = .medium
    @State private var appearance = "auto"

    var body: some View {
      Select(
        options: [
          SelectOption(value: "auto", label: "Automatic"),
          SelectOption(value: "light", label: "Light"),
          SelectOption(value: "dark", label: "Dark"),
        ],
        selection: $appearance,
        size: size
      )
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Select",
        description: """
          One value out of many. It wears the text field's box — the same \
          heights, the same states, the same focus ring — so a select and an \
          input in the same form line up. Its list is the shared floating \
          sheet, the one a menu and a tooltip also wear.
          """
      ) {
        Section(
          "States",
          description: """
            Default, error and disabled, exactly as the field has them. An \
            option can be disabled on its own without the control being.
            """
        ) {
          Row(alignment: .top) {
            Region()
            Region(state: .error)
            Region(enabled: false)
          }
        }

        Section(
          "Sizes",
          description: """
            The three field sizes, which are the control profile's under \
            another name. `small` is the one a settings row wants at its \
            trailing edge.
            """
        ) {
          Col(width: 260) {
            ForEach(TextFieldSize.allCases, id: \.self) { size in
              Region(size: size)
            }
          }
        }

        Section(
          "In a form",
          description: """
            Under a `FormField` label like any other control, or pinned to the \
            trailing edge of a settings row at the small step.
            """
        ) {
          Col(width: 460) {
            FormField(label: "Region", hint: "Where the documents are stored.") {
              Region()
            }
            PreferenceSection(label: "Appearance") {
              PreferenceRow(title: "Theme", trailing: { Appearance(size: .small) })
            }
          }
        }
      }
    }
  }
}
