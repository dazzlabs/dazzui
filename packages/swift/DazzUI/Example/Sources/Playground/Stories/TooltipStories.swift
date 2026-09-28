import DazzUI
import SwiftUI

/// `Components/Tooltip` — the label a control surfaces on hover.
enum TooltipStories {
  static var page: StoryPage {
    StoryPage("Tooltip") {
      Story("Overview", layout: .page) { Overview() }
      Story("Default") {
        Tooltip("Rebuild the token graph") {
          Button("Rebuild", variant: .recessed) {}
        }
      }
      Story("Open") {
        Tooltip("It is open because the story says so", presented: true) {
          Button("Rebuild", variant: .recessed) {}
        }
      }
      Story("Sides") { Sides() }
      Story("On icon buttons") {
        TooltipProvider {
          Row {
            Tooltip("Back") {
              IconButton(label: "Back", icon: .chevronLeft16Regular) {}
            }
            Tooltip("Forward") {
              IconButton(label: "Forward", icon: .chevronRight16Regular) {}
            }
            Tooltip("Refresh") {
              IconButton(label: "Refresh", icon: .arrowClockwise16Regular) {}
            }
          }
        }
      }
      Story("A toolbar") { Toolbar() }
      Story("A long label") {
        Tooltip(
          "The token graph is rebuilt from ds and rewritten into every platform package.",
          presented: true
        ) {
          Button("Rebuild", variant: .recessed) {}
        }
      }
    }
  }

  private struct Sides: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Row(spacing: theme.vars.spacing8) {
        ForEach(TooltipSide.allCases, id: \.self) { side in
          Tooltip(label(side), side: side, presented: true) {
            Button(label(side), variant: .recessed) {}
          }
        }
      }
      .padding(.vertical, theme.vars.spacing10)
    }
  }

  /// The provider is what makes a toolbar feel like a toolbar: once one label
  /// has appeared, moving along the row opens the next one immediately.
  private struct Toolbar: View {
    var body: some View {
      TooltipProvider {
        ActionBar {
          Tooltip("Back") {
            IconButton(label: "Back", icon: .chevronLeft16Regular) {}
          }
          Tooltip("Forward") {
            IconButton(label: "Forward", icon: .chevronRight16Regular) {}
          }
          ToolbarSeparator()
          Tooltip("Refresh") {
            IconButton(label: "Refresh", icon: .arrowClockwise16Regular) {}
          }
          Tooltip("Delete") {
            IconButton(label: "Delete", icon: .delete16Regular, tint: .danger) {}
          }
        }
        .frame(width: 260)
      }
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Tooltip",
        description: """
          The short label a control surfaces on hover or keyboard focus. It is \
          only a reminder of what the control means: the control still carries \
          its own accessible name, and the tooltip is hidden from \
          accessibility so that name is not announced twice.
          """
      ) {
        Section(
          "Sides",
          description: """
            Four sides and three alignments. The panel is the shared floating \
            sheet — the same drawing a menu and a select's list wear — and the \
            gap between it and the control is a token rather than an offset \
            passed in code.
            """
        ) {
          Sides()
        }

        Section(
          "The provider",
          description: """
            A tooltip waits before it opens, so the pointer passing over a \
            toolbar does not set off a row of labels. `TooltipProvider` makes \
            that delay a property of the *group*: once one label is open, the \
            next opens at once.
            """
        ) {
          Toolbar()
        }
      }
    }
  }
}
