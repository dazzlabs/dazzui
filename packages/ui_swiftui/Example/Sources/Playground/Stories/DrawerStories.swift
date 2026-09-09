import DazzUI
import SwiftUI

/// `Components/Drawer` — a sheet that comes in from an edge.
enum DrawerStories {
  static var page: StoryPage {
    StoryPage("Drawer") {
      Story("Overview", layout: .page) { Overview() }
      Story("Bare") { frame(480, 320) { filters } }
      Story("Sides") {
        Stack {
          ForEach(DrawerSide.allCases, id: \.self) { side in
            Specimen(side.rawValue) {
              frame(280, 200) {
                Drawer(side: side, size: 160, title: "Filters") { checks }
              }
            }
          }
        }
      }
      Story("With footer") { frame(480, 320) { Remote() } }
      Story("Controlled") {
        // A SwiftUI drawer is a view rather than a route, so a host shows it
        // by rendering it: there is nothing to open.
        frame(480, 320) { filters }
      }
      Story("Wide") {
        frame(560, 320) {
          Drawer(
            size: 400,
            title: "Review",
            description: "A panel that needs more room than the token gives it."
          ) {
            Text("The width is a token, and `size` is the override.")
          }
        }
      }
    }
  }

  /// A drawer fills whatever it is placed in, so the page has to give it a box
  /// — the counterpart of the window it covers in an app.
  static func frame<Content: View>(
    _ width: CGFloat,
    _ height: CGFloat,
    @ViewBuilder content: () -> Content
  ) -> some View {
    content()
      .frame(width: width, height: height)
      .clipped()
  }

  static var checks: some View {
    Col {
      Checkbox("Open", checked: true)
      Checkbox("Merged", checked: false)
      Checkbox("Closed", checked: false)
    }
  }

  static var filters: some View {
    Drawer(title: "Filters", description: "Narrow what the list is showing.") { checks }
  }

  struct Remote: View {
    @State private var name = "origin"
    @State private var url = "git@github.com:acme/web.git"

    var body: some View {
      Drawer(
        title: "Edit remote",
        description: "origin · git@github.com:acme/web.git",
        footer: {
          Button("Save", variant: .filled) {}
          Button("Cancel") {}
        },
        content: {
          FormField(label: "Name") { TextField(text: $name, mono: true) }
          FormField(label: "URL", hint: "SSH or HTTPS") { TextField(text: $url, mono: true) }
        }
      )
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Drawer",
        description:
          "A sheet that comes in from an edge and can be pushed back out. It is the Dialog’s "
          + "sibling, and the choice between them is about where the reader’s attention has to "
          + "go: a dialog takes the middle of the view and asks a question; a drawer takes a "
          + "side of it and holds a panel the reader dips into — filters, a detail pane — while "
          + "what they were reading stays where it was."
      ) {
        Section(
          "Sides",
          description:
            "It keeps a corner only on the two sides still inside the view. A sheet flush to an "
            + "edge with all four corners rounded reads as floating just off the screen rather "
            + "than as attached to it."
        ) {
          Stack {
            ForEach(DrawerSide.allCases, id: \.self) { side in
              Specimen(side.rawValue) {
                DrawerStories.frame(280, 200) {
                  Drawer(side: side, size: 160, title: "Filters") { DrawerStories.checks }
                }
              }
            }
          }
        }
        Section(
          "The three rows",
          description:
            "Header and footer are the dialog’s own parts rather than a second set: a drawer "
            + "and a sheet in the same app are set the same way. Only the body scrolls, so a "
            + "long panel keeps its heading and its actions in view."
        ) {
          DrawerStories.frame(480, 320) { Remote() }
        }
      }
    }
  }
}
