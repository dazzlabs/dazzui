import DazzUI
import SwiftUI

/// `Components/NavItem` — the row a sidebar and a rail are made of.
enum NavItemStories {
  static var page: StoryPage {
    StoryPage("NavItem") {
      Story("Overview", layout: .page) { Overview() }
      Story("Idle") {
        NavItem("Projects", action: {}).frame(width: 200)
      }
      Story("Active") {
        NavItem("Projects", active: true, action: {}).frame(width: 200)
      }
      Story("Disabled") {
        NavItem("Projects", enabled: false, action: {}).frame(width: 200)
      }
      Story("States") {
        Col(width: 200) {
          NavItem("Idle", action: {})
          NavItem("Active", active: true, action: {})
          NavItem("Disabled", enabled: false, action: {})
        }
      }
      Story("Sizes") {
        Col(width: 200) {
          ForEach(NavItemSize.allCases, id: \.self) { size in
            NavItem(label(size), active: size == .medium, size: size, action: {})
          }
        }
      }
      Story("With an icon") {
        Col(width: 200) {
          NavItem("Projects", icon: .folder16Regular, active: true, action: {})
          NavItem("Archive", icon: .archive16Regular, action: {})
          NavItem("Settings", icon: .settings16Regular, action: {})
        }
      }
      Story("Tints") {
        Col(width: 200) {
          ForEach(Tint.allCases, id: \.self) { tint in
            NavItem(label(tint), active: true, tint: tint, action: {})
          }
        }
      }
      Story("Action") {
        Col(width: 200) {
          NavItem("Projects", icon: .folder16Regular, action: {})
          NavAction("New project", action: {})
        }
      }
      Story("Selecting") { Selecting() }
      Story("In a sidebar") {
        Frame(width: 520, height: 300) {
          Sidebar {
            SidebarGroup("Library") {
              NavItem("Projects", icon: .folder16Regular, active: true, action: {})
              NavItem("Archive", icon: .archive16Regular, action: {})
            }
          }
          Pane { Text("Projects") }
        }
      }
    }
  }

  private struct Selecting: View {
    @State private var selected = "projects"

    private let rows = [
      ("projects", "Projects"),
      ("archive", "Archive"),
      ("settings", "Settings"),
    ]

    var body: some View {
      Col(width: 200) {
        ForEach(rows, id: \.0) { id, name in
          NavItem(name, active: selected == id) { selected = id }
        }
      }
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "NavItem",
        description: """
          One navigable row. Selection shifts weight as well as colour: a row \
          rests at the quiet label face and the selected one takes the filled \
          recipe, so a column reads as one selection rather than as a list of \
          differently coloured rows.
          """
      ) {
        Section(
          "States",
          description: """
            Idle rows sit in the navigation ink, a step back from body copy. \
            Hover brings the row to full ink — the hover rule outranks the \
            row's own colour, so both kinds of row answer the pointer the same \
            way. Active takes the filled recipe on the tint.
            """
        ) {
          Col(width: 220) {
            NavItem("Idle", action: {})
            NavItem("Active", active: true, action: {})
            NavItem("Disabled", enabled: false, action: {})
          }
        }

        Section(
          "Sizes",
          description: """
            `medium` is the sidebar row — 28px, the regular control height. \
            `small` is the rail row, a step tighter on both axes because it \
            sits closer to the content.
            """
        ) {
          Col(width: 220) {
            ForEach(NavItemSize.allCases, id: \.self) { size in
              NavItem(label(size), active: size == .medium, size: size, action: {})
            }
          }
        }

        Section(
          "The glyph",
          description: """
            An absent icon stays absent rather than becoming an empty one: the \
            glyph slot is a fixed width, and a row without an icon would \
            otherwise be indented past a row with one.
            """
        ) {
          Col(width: 220) {
            NavItem("Projects", icon: .folder16Regular, active: true, action: {})
            NavItem("Archive", action: {})
          }
        }

        Section(
          "NavAction",
          description: """
            The row that adds rather than navigates. It takes the accent ink \
            and the label weight at rest, which is what separates it from the \
            rows above it without giving it a box of its own.
            """
        ) {
          Col(width: 220) {
            NavItem("Projects", icon: .folder16Regular, action: {})
            NavItem("Archive", icon: .archive16Regular, action: {})
            NavAction("New project", action: {})
          }
        }
      }
    }
  }
}
