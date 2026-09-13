import DazzUI
import SwiftUI

/// `Components/EmptyState` — what stands in for content that is not there.
enum EmptyStateStories {
  static var page: StoryPage {
    StoryPage("EmptyState") {
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


}
