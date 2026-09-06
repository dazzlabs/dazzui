import DazzUI
import SwiftUI

/// `Components/ActionBar` — the row a dialog footer, a toolbar and a form's
/// last line are all the same thing.
enum ActionBarStories {
  static var page: StoryPage {
    StoryPage("ActionBar") {
      Story("Overview", layout: .page) { Overview() }
      Story("Start") {
        Bar(align: .start)
      }
      Story("End") {
        Bar(align: .end)
      }
      Story("Between") {
        Bar(align: .between)
      }
      Story("As a toolbar") {
        ActionBar {
          IconButton(label: "Back", icon: .chevronLeft16Regular) {}
          IconButton(label: "Forward", icon: .chevronRight16Regular) {}
          ToolbarSeparator()
          IconButton(label: "Refresh", icon: .arrowClockwise16Regular) {}
          IconButton(label: "Share", icon: .share16Regular) {}
        }
        .frame(width: 320)
      }
      Story("An action row") {
        ActionBar(align: .end) {
          Button("Cancel", variant: .recessed) {}
          Button("Save", variant: .filled) {}
        }
        .frame(width: 380)
      }
      Story("With a key hint") {
        ActionBar(align: .between) {
          KeyCap("⌘↩", size: .small)
          Button("Send", variant: .filled, shortcut: "⌘↩") {}
        }
        .frame(width: 380)
      }
      Story("Mixed controls") {
        ActionBar(align: .between) {
          Badge("Draft", size: .small, variant: .tinted, tint: .warning)
          Row {
            Button("Discard", variant: .plain, tint: .danger) {}
            Button("Publish", variant: .filled) {}
          }
        }
        .frame(width: 420)
      }
    }
  }

  /// The three distributions, over the same three controls.
  private struct Bar: View {
    let align: ActionBarAlign

    var body: some View {
      ActionBar(align: align) {
        Button("Reset", variant: .plain) {}
        Button("Cancel", variant: .recessed) {}
        Button("Apply", variant: .filled) {}
      }
      .frame(width: 420)
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "ActionBar",
        description: """
          A row of controls kept on one rhythm. It owns no colour and no box — \
          only the gap — so it takes the shared control gap rather than a token \
          of its own: the small profile's, whatever the children's size, since \
          the design draws every action row on that one step.
          """
      ) {
        Section(
          "Alignment",
          description: """
            `start` is the default. `end` is a dialog footer's usual shape, \
            with the confirming action last. `between` is the case an `HStack` \
            cannot express: the slack goes to the gaps *between* the children, \
            which is what puts a destructive action opposite the pair that \
            confirms.
            """
        ) {
          Stack {
            ForEach(ActionBarAlign.allCases, id: \.self) { align in
              Specimen(align.rawValue) { Bar(align: align) }
            }
          }
        }

        Section(
          "What it holds",
          description: """
            Anything on the control rhythm: buttons, icon buttons with a \
            vertical divider between runs, a key hint, a badge that says what \
            the row is about. The bar never colours or sizes what it holds.
            """
        ) {
          Stack {
            Specimen("toolbar") {
              ActionBar {
                IconButton(label: "Back", icon: .chevronLeft16Regular) {}
                IconButton(label: "Forward", icon: .chevronRight16Regular) {}
                ToolbarSeparator()
                IconButton(label: "Refresh", icon: .arrowClockwise16Regular) {}
              }
              .frame(width: 240)
            }
            Specimen("confirming") {
              ActionBar(align: .end) {
                Button("Cancel", variant: .recessed) {}
                Button("Save", variant: .filled) {}
              }
              .frame(width: 300)
            }
            Specimen("mixed") {
              ActionBar(align: .between) {
                Badge("Draft", size: .small, variant: .tinted, tint: .warning)
                Button("Publish", variant: .filled) {}
              }
              .frame(width: 300)
            }
          }
        }
      }
    }
  }
}
