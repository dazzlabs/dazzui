import DazzUI
import SwiftUI

/// `Components/SearchField` — a text field that knows what it is for.
enum SearchFieldStories {
  static var page: StoryPage {
    StoryPage("SearchField") {
      Story("Overview", layout: .page) { Overview() }
      Story("Empty") { Field(query: "") }
      Story("With a query") { Field(query: "button") }
      Story("With a hint") { Hinted() }
      Story("Sizes") {
        Col(width: 320) {
          ForEach(TextFieldSize.allCases, id: \.self) { size in
            Field(query: label(size), size: size)
          }
        }
      }
      Story("Tints") {
        Col(width: 320) {
          ForEach(Tint.allCases, id: \.self) { tint in
            Field(query: label(tint), tint: tint)
          }
        }
      }
      Story("Dismissable") { Dismissable() }
      Story("Filtering a list") { Filtering() }
      Story("In a pane header") {
        Frame(width: 560, height: 260) {
          Pane {
            Filtering()
          }
        }
      }
    }
  }

  private struct Field: View {
    let size: TextFieldSize
    let tint: Tint

    @State private var value: String

    init(query: String, size: TextFieldSize = .medium, tint: Tint = .primary) {
      self.size = size
      self.tint = tint
      _value = State(initialValue: query)
    }

    var body: some View {
      SearchField(
        value: value,
        onValueChange: { value = $0 },
        placeholder: "Search components",
        size: size,
        tint: tint
      )
      .frame(width: 320)
    }
  }

  private struct Hinted: View {
    @State private var value = ""

    var body: some View {
      SearchField(
        value: value,
        onValueChange: { value = $0 },
        placeholder: "Search components",
        hint: { KeyCap("⌘K", size: .small) }
      )
      .frame(width: 320)
    }
  }

  private struct Dismissable: View {
    @State private var value = "tokens"

    var body: some View {
      SearchField(
        value: value,
        onValueChange: { value = $0 },
        placeholder: "Search components",
        onDismiss: { value = "" }
      )
      .frame(width: 320)
    }
  }

  /// The field doing its job: what it filters is the playground's own index.
  private struct Filtering: View {
    @State private var query = ""

    @Environment(\.theme) private var theme

    private var matches: [String] {
      let needle = query.lowercased()
      guard !needle.isEmpty else { return StoryIndex.componentNames }
      return StoryIndex.componentNames.filter { $0.lowercased().contains(needle) }
    }

    var body: some View {
      VStack(alignment: .leading, spacing: theme.vars.spacing3) {
        SearchField(
          value: query,
          onValueChange: { query = $0 },
          placeholder: "Search components",
          onDismiss: query.isEmpty ? nil : { query = "" }
        )

        if matches.isEmpty {
          EmptyState("Nothing matched “\(query)”")
            .frame(maxWidth: .infinity)
        } else {
          VStack(alignment: .leading, spacing: 0) {
            ForEach(matches.prefix(5), id: \.self) { name in
              NavItem(name, action: {})
            }
          }
        }
      }
      .frame(width: 320)
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "SearchField",
        description: """
          The text field with a glyph in front of it, a clear button behind, \
          and a slot for the key that focuses it. It is the same box as \
          `TextField` — same heights, same states — so a search field in a \
          toolbar lines up with everything else on that row.
          """
      ) {
        Section(
          "States",
          description: """
            Empty, with a query, and with something bound to it. The clear \
            button appears only once there is something to clear, so an empty \
            field is not offering an action that does nothing.
            """
        ) {
          Col(width: 420) {
            Field(query: "")
            Field(query: "button")
            Dismissable()
          }
        }

        Section(
          "The hint",
          description: """
            The trailing slot takes the key that focuses the field. It is a \
            `KeyCap` at the small step — a hint rather than a control, so it \
            never answers the pointer.
            """
        ) {
          Hinted()
        }

        Section(
          "Filtering",
          description: """
            What a search field is actually for. This one filters the \
            playground's own component index, and falls back to an `EmptyState` \
            when nothing matches.
            """
        ) {
          Filtering()
        }
      }
    }
  }
}
