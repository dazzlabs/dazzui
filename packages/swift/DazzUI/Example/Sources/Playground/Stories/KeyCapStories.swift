import DazzUI
import SwiftUI

/// `Components/KeyCap` — a key, printed.
enum KeyCapStories {
  static var page: StoryPage {
    StoryPage("KeyCap") {
      Story("Overview", layout: .page) { Overview() }
      Story("Hint") { KeyCap("⌘K") }
      Story("Strong") { KeyCap("⌘K", variant: .strong) }
      Story("Key") { KeyCap("⌘K", variant: .key) }
      Story("All variants") {
        Row {
          ForEach(KeyCapVariant.allCases, id: \.self) { variant in
            KeyCap("⌘K", variant: variant)
          }
        }
      }
      Story("Sizes") {
        Row {
          ForEach(KeyCapSize.allCases, id: \.self) { size in
            KeyCap("⌘K", size: size)
          }
        }
      }
      Story("A chord") {
        Row {
          KeyCap("⇧")
          KeyCap("⌘")
          KeyCap("P")
        }
      }
      Story("A joined chord") { KeyCap("⇧⌘P") }
      Story("Word keys") {
        Row {
          KeyCap("Esc", variant: .key)
          KeyCap("Tab", variant: .key)
          KeyCap("Enter", variant: .key)
          KeyCap("Space", variant: .key)
        }
      }
      Story("As a search hint") { SearchHint() }
      Story("In a settings row") {
        PreferenceSection(label: "Shortcuts") {
          PreferenceRow(title: "Toggle the palette", trailing: { KeyCap("⌘K") })
          PreferenceRow(title: "Rebuild tokens", trailing: { KeyCap("⇧⌘B") })
        }
        .frame(width: 460)
      }
      Story("In a menu") {
        DazzUI.Menu(items: [
          MenuItem("Rename", shortcut: "⌘R") {},
          MenuItem("Duplicate", shortcut: "⇧⌘D") {},
          MenuItem("Delete", shortcut: "⌫") {},
        ]) { state in
          Button("Actions", variant: .recessed, shortcut: "▾") { state.toggle() }
        }
      }
    }
  }

  private struct SearchHint: View {
    @State private var query = ""

    var body: some View {
      SearchField(
        value: query,
        onValueChange: { query = $0 },
        placeholder: "Search components",
        hint: { KeyCap("⌘K", size: .small) }
      )
      .frame(width: 320)
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "KeyCap",
        description: """
          The printed form of a key. It is not a control and never reacts to \
          the pointer — it is a label saying which key does the thing beside \
          it. Its colours are already in the token graph, so it takes no tint.
          """
      ) {
        Section(
          "Variants",
          description: """
            `hint` is the quiet one that sits inside a field or beside a menu \
            row. `strong` is the one in prose, where it has to hold its own \
            against body copy. `key` is the drawn cap, for a word key that has \
            to read as a physical key rather than as a symbol.
            """
        ) {
          Stack {
            ForEach(KeyCapVariant.allCases, id: \.self) { variant in
              Specimen(variant.rawValue) {
                KeyCap("⌘K", variant: variant)
                KeyCap("⇧⌘P", variant: variant)
                KeyCap("Esc", variant: variant)
              }
            }
          }
        }

        Section(
          "Chords",
          description: """
            A chord can be one cap with the symbols joined, or a cap per key. \
            One cap is the usual form — it is one shortcut, and three boxes \
            read as three keys to press in turn.
            """
        ) {
          Stack {
            Specimen("joined") { KeyCap("⇧⌘P") }
            Specimen("separate") {
              KeyCap("⇧")
              KeyCap("⌘")
              KeyCap("P")
            }
          }
        }

        Section(
          "Where it lands",
          description: """
            The trailing hint in a search field, the shortcut column of a menu, \
            the trailing slot of a settings row. Inside a button the shortcut \
            is *not* a cap — it is the button's own colour at a lowered \
            opacity, because a cap inside a control reads as a second control.
            """
        ) {
          Stack {
            Specimen("field") { SearchHint() }
            Specimen("settings") {
              PreferenceSection(label: "Shortcuts") {
                PreferenceRow(title: "Toggle the palette", trailing: { KeyCap("⌘K") })
              }
              .frame(width: 420)
            }
            Specimen("button") {
              Button("Translate", variant: .filled, shortcut: "⌘S") {}
            }
          }
        }
      }
    }
  }
}
