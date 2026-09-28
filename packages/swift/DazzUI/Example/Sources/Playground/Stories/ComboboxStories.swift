import DazzUI
import SwiftUI

/// `Components/Combobox` — a choice out of a list too long to scroll.
enum ComboboxStories {
  static let zones: [(String, String, String)] = [
    ("lisbon", "Europe/Lisbon", "Europe"),
    ("london", "Europe/London", "Europe"),
    ("berlin", "Europe/Berlin", "Europe"),
    ("shanghai", "Asia/Shanghai", "Asia"),
    ("tokyo", "Asia/Tokyo", "Asia"),
    ("sydney", "Australia/Sydney", "Asia"),
    ("new_york", "America/New_York", "Americas"),
    ("sao_paulo", "America/Sao_Paulo", "Americas"),
  ]

  static func options(grouped: Bool) -> [ComboboxOption<String>] {
    zones.map { value, label, group in
      ComboboxOption(value, label: label, group: grouped ? group : nil)
    }
  }

  static var page: StoryPage {
    StoryPage("Combobox") {
      Story("Overview", layout: .page) { Overview() }
      Story("Bare") { Live(width: 256) }
      Story("Grouped") { Live(width: 256, grouped: true) }
      Story("Sizes") {
        Col(width: 256) {
          ForEach(TextFieldSize.allCases, id: \.self) { size in
            Live(start: "london", size: size)
          }
        }
      }
      Story("States") {
        Col(width: 256) {
          Live(start: "london")
          Live(state: .error)
          Combobox(
            options: options(grouped: false),
            value: .constant("london"),
            query: .constant("Europe/London"),
            enabled: false
          )
        }
      }
      Story("In a form") { Zone() }
    }
  }

  struct Live: View {
    var width: CGFloat?
    var start: String?
    var grouped = false
    var size: TextFieldSize = .medium
    var state: TextFieldState = .default

    @State private var value: String?
    @State private var query = ""
    @State private var seeded = false

    var body: some View {
      Combobox(
        options: ComboboxStories.options(grouped: grouped),
        value: $value,
        query: $query,
        placeholder: "Search time zones…",
        size: size,
        state: state
      )
      .frame(width: width)
      .onAppear {
        guard !seeded, let start else { return }
        seeded = true
        value = start
        query = ComboboxStories.zones.first { $0.0 == start }?.1 ?? ""
      }
    }
  }

  private struct Zone: View {
    @State private var value: String? = "london"
    @State private var query = "Europe/London"

    var body: some View {
      FormField(
        label: "Time zone",
        hint: value == nil ? "Pick one" : "Used for every schedule",
        state: value == nil ? .error : .default
      ) {
        Combobox(
          options: ComboboxStories.options(grouped: true),
          value: $value,
          query: $query,
          state: value == nil ? .error : .default
        )
      }
      .frame(width: 288)
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Combobox",
        description:
          "A choice out of a list too long to scroll: the reader types to narrow it, and picks "
          + "one of what is left. It is a Select with a query in front of it, and that is the "
          + "only question worth asking when choosing between them — a list of six is a select, "
          + "a list of six hundred is this. Both end in one of the options; neither lets a "
          + "reader enter something that is not on the list, which is what separates them from "
          + "an Autocomplete."
      ) {
        Section(
          "Sizes",
          description:
            "The box is the TextField’s and the list is the menu’s own sheet, so a combobox in "
            + "a form is the same box as the field beside it and the same panel a Menu opens."
        ) {
          Stack {
            ForEach(TextFieldSize.allCases, id: \.self) { size in
              Specimen(label(size)) { Live(width: 224, start: "london", size: size) }
            }
          }
        }
        Section(
          "Grouped",
          description:
            "The headings are drawn from what survived the query rather than from the list as "
            + "it was given: a group the query emptied is not a heading over nothing."
        ) {
          Live(width: 256, grouped: true)
        }
        Section(
          "States",
          description:
            "Erroring re-tints the control to danger, exactly as a TextField does. The empty "
            + "line is prose in a panel of rows, so it takes the row’s inset and the faint ink "
            + "rather than looking like a row that cannot be picked."
        ) {
          Stack {
            Specimen("error") { Live(width: 224, state: .error) }
            Specimen("disabled") {
              Combobox(
                options: ComboboxStories.options(grouped: false),
                value: .constant("london"),
                query: .constant("Europe/London"),
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
