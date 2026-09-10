import DazzUI
import SwiftUI

/// `Components/Menu` — the floating sheet, and the rows in it.
enum MenuStories {
  static var page: StoryPage {
    StoryPage("Menu") {
      Story("Overview", layout: .page) { Overview() }
      Story("Default") {
        DazzUI.Menu(items: [
          MenuItem("Rename") {},
          MenuItem("Duplicate") {},
          MenuItem("Delete") {},
        ]) { state in
          Button("Actions", variant: .recessed, shortcut: "▾") { state.toggle() }
        }
      }
      Story("With shortcuts") {
        DazzUI.Menu(items: [
          MenuItem("Rename", shortcut: "⌘R") {},
          MenuItem("Duplicate", shortcut: "⇧⌘D") {},
          MenuItem("Delete", shortcut: "⌫") {},
        ]) { state in
          Button("Actions", variant: .recessed, shortcut: "▾") { state.toggle() }
        }
      }
      Story("With icons") {
        DazzUI.Menu(items: [
          MenuItem("Rename", icon: .rename16Regular, shortcut: "⌘R") {},
          MenuItem("Duplicate", icon: .squareMultiple16Regular) {},
          MenuItem("Delete", icon: .delete16Regular) {},
        ]) { state in
          Button("Actions", variant: .recessed, shortcut: "▾") { state.toggle() }
        }
      }
      Story("Checked") { Checked() }
      Story("Disabled") {
        DazzUI.Menu(items: [
          MenuItem("Rename", icon: .rename16Regular) {},
          MenuItem("Delete", icon: .delete16Regular, disabled: true) {},
        ]) { state in
          Button("Actions", variant: .recessed, shortcut: "▾") { state.toggle() }
        }
      }
      Story("Alignment") { Alignments() }
      Story("From an icon button") {
        DazzUI.Menu(items: [
          MenuItem("Rename", icon: .rename16Regular) {},
          MenuItem("Duplicate", icon: .squareMultiple16Regular) {},
          MenuItem("Delete", icon: .delete16Regular) {},
        ]) { state in
          IconButton(label: "Actions", icon: .moreHorizontal16Regular) {
            state.toggle()
          }
        }
      }
      Story("In a list header") {
        Card {
          HStack {
            Text("Builds")
            Spacer()
            DazzUI.Menu(items: [
              MenuItem("Sort by name") {},
              MenuItem("Sort by date", checked: true) {},
            ]) { state in
              IconButton(label: "Sort", icon: .arrowSort16Regular) {
                state.toggle()
              }
            }
          }
          .frame(width: 280)
        }
      }
    }
  }

  /// Both edges the sheet can hang from, far enough apart to see which one
  /// it chose.
  private struct Alignments: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Row(spacing: theme.vars.spacing20) {
        ForEach(MenuAlign.allCases, id: \.self) { align in
          DazzUI.Menu(
            items: [MenuItem("Rename") {}, MenuItem("Duplicate") {}],
            align: align
          ) { state in
            Button(label(align), variant: .recessed, shortcut: "▾") { state.toggle() }
          }
        }
      }
    }
  }

  /// The tick column: a menu that carries any checked item reserves the
  /// column for all of them, so the labels stay on one edge.
  private struct Checked: View {
    @State private var hidden = true
    @State private var sidebar = false

    var body: some View {
      DazzUI.Menu(items: [
        MenuItem("Show hidden files", checked: hidden) { hidden.toggle() },
        MenuItem("Show the sidebar", checked: sidebar) { sidebar.toggle() },
      ]) { state in
        Button("View", variant: .recessed, shortcut: "▾") { state.toggle() }
      }
    }
  }

  private struct Overview: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Page(
        "Menu",
        description: """
          A trigger and a floating sheet of rows. The sheet is one drawing \
          shared with the select's list and the tooltip; the menu adds only \
          what is genuinely its own — the icon slot, the shortcut column, and \
          the tick that appears once any row is checkable.
          """
      ) {
        Section(
          "Rows",
          description: """
            A row can carry a leading glyph, a trailing shortcut, a check, and \
            a disabled state. The row reads `highlighted` rather than `hover`, \
            which is what makes the keyboard and the pointer land on the same \
            state.
            """
        ) {
          Stack {
            Specimen("plain") {
              DazzUI.Menu(items: [
                MenuItem("Rename") {}, MenuItem("Duplicate") {},
              ]) { state in
                Button("Actions", variant: .recessed, shortcut: "▾") { state.toggle() }
              }
            }
            Specimen("icons") {
              DazzUI.Menu(items: [
                MenuItem("Rename", icon: .rename16Regular, shortcut: "⌘R") {},
                MenuItem("Delete", icon: .delete16Regular, disabled: true) {},
              ]) { state in
                Button("Actions", variant: .recessed, shortcut: "▾") { state.toggle() }
              }
            }
            Specimen("checked") { Checked() }
          }
        }

        Section(
          "Alignment",
          description: """
            The sheet hangs from the trigger's trailing edge by default, which \
            is where a `▾` button wants it. `start` hangs it from the leading \
            edge instead. The gap between the trigger and the sheet is padding \
            on the positioner rather than an offset passed in code, so the gap \
            is a token.
            """
        ) {
          Alignments()
        }

        Section(
          "Tokens",
          description: "The sheet's own metrics."
        ) {
          TokenTable([
            TokenRow("menu.min-width", theme.vars.menuMinWidth),
            TokenRow("menu.item.padding", theme.vars.menuItemPadding),
          ])
        }
      }
    }
  }
}
