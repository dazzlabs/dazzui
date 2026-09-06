import DazzUI
import SwiftUI

/// `Components/EmptyState` — what stands in for content that is not there.
enum EmptyStateStories {
  static var page: StoryPage {
    StoryPage("EmptyState") {
      Story("Overview", layout: .page) { Overview() }
      Story("On its own") {
        EmptyState("No components match this filter")
      }
      Story("With an action") {
        EmptyState(title: "No components match this filter") {
          Button("Clear the filter", variant: .tinted) {}
        }
      }
      Story("With two actions") {
        EmptyState(title: "This project has no glossary yet") {
          ActionBar {
            Button("Import", variant: .recessed) {}
            Button("Create one", variant: .filled) {}
          }
        }
      }
      Story("Nothing yet") {
        EmptyState(title: "No builds have run on this branch") {
          Button("Run one", variant: .filled) {}
        }
      }
      Story("Nothing matched") {
        EmptyState("Nothing matched “tokens”")
      }
      Story("In a well") {
        Card(variant: .sunken) {
          EmptyState(title: "No components match this filter") {
            Button("Clear the filter", variant: .tinted) {}
          }
          .frame(maxWidth: .infinity)
        }
        .frame(width: 460)
      }
      Story("In a pane") {
        Frame(width: 560, height: 300) {
          Pane {
            EmptyState(title: "Pick a build to see its log") {
              Button("Run one", variant: .recessed) {}
            }
            .frame(maxWidth: .infinity, maxHeight: .infinity)
          }
        }
      }
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "EmptyState",
        description: """
          A line saying what is missing, and at most the one action that would \
          fix it. It is the pane's content rather than a notice about the \
          pane, so it centres in the space it was given instead of hanging \
          from the top of it.
          """
      ) {
        Section(
          "Shapes",
          description: """
            The title alone is enough where nothing can be done — a filter that \
            matched nothing is fixed by changing the filter, not by a button. \
            Where there is something to do, the action slot takes it; two \
            actions go in an `ActionBar` so the gap stays on the control scale.
            """
        ) {
          Row(alignment: .top) {
            EmptyState("Nothing matched “tokens”")
            EmptyState(title: "No builds on this branch") {
              Button("Run one", variant: .filled) {}
            }
            EmptyState(title: "No glossary yet") {
              ActionBar {
                Button("Import", variant: .recessed) {}
                Button("Create one", variant: .filled) {}
              }
            }
          }
        }

        Section(
          "Where it lands",
          description: """
            A well, when the empty thing is a section of a page; the pane \
            itself, when the empty thing is the whole view.
            """
        ) {
          Frame(width: 620, height: 260) {
            Pane {
              EmptyState(title: "Pick a build to see its log") {
                Button("Run one", variant: .recessed) {}
              }
              .frame(maxWidth: .infinity, maxHeight: .infinity)
            }
          }
        }
      }
    }
  }
}
