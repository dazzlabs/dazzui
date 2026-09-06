import DazzUI
import SwiftUI

/// `Components/Rail` — the second navigation column.
enum RailStories {
  static var page: StoryPage {
    StoryPage("Rail") {
      Story("Overview", layout: .page) { Overview() }
      Story("A column") {
        Frame(width: 520, height: 300) {
          Rail { Rows() }
          Pane { Text("Studio Light") }
        }
      }
      Story("Groups") {
        Frame(width: 520, height: 300) {
          Rail {
            RailGroup {
              NavItem("Overview", active: true, size: .small, action: {})
            }
            RailGroup("Themes") { Rows() }
          }
          Pane { Text("Overview") }
        }
      }
      Story("With an action") {
        Frame(width: 520, height: 300) {
          Rail {
            RailGroup("Themes") { Rows() }
            NavAction("New theme", size: .small, action: {})
          }
          Pane { Text("Studio Light") }
        }
      }
      Story("Beside a sidebar") {
        Frame(width: 780, height: 320) {
          Sidebar {
            SidebarGroup("Library") {
              NavItem("Design system", icon: .paintBrush16Regular, active: true, action: {})
              NavItem("Examples", icon: .folder16Regular, action: {})
            }
          }
          Rail {
            RailGroup("Themes") { Rows() }
          }
          Pane { Text("Studio Light") }
        }
      }
      Story("Scrolling") {
        Frame(width: 420, height: 220) {
          Rail {
            RailGroup("Components") {
              ForEach(StoryIndex.componentNames, id: \.self) { name in
                NavItem(name, active: name == "Rail", size: .small, action: {})
              }
            }
          }
          Pane { Text("Rail") }
        }
      }
    }
  }

  private struct Rows: View {
    @State private var selected = "studio-light"

    private let themes = [
      ("studio-light", "Studio Light"),
      ("studio-dark", "Studio Dark"),
      ("bright-light", "Bright"),
      ("bright-dark", "Bright Dark"),
    ]

    var body: some View {
      ForEach(themes, id: \.0) { id, name in
        NavItem(name, active: selected == id, size: .small) { selected = id }
      }
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Rail",
        description: """
          The column between the sidebar and the content: the groups, books or \
          pages inside whatever the sidebar selected. It is lighter than the \
          sidebar because it sits closer to the content — the two columns are \
          not the same grey, and which is which is what tells you where you are.
          """
      ) {
        Section(
          "Rows",
          description: """
            A rail's rows are `NavItem`s at the `small` step, a size tighter \
            than the sidebar's on both axes. The first group usually goes \
            unlabelled: it is what the pane is already called, and naming it \
            twice says nothing.
            """
        ) {
          Frame(width: 620, height: 300) {
            Rail {
              RailGroup { NavItem("Overview", active: true, size: .small, action: {}) }
              RailGroup("Themes") { Rows() }
            }
            Pane { Text("Overview") }
          }
        }

        Section(
          "With an action",
          description: """
            A trailing `NavAction` sinks to the foot of the column: the run is \
            given the height to sink through even when it does not fill it.
            """
        ) {
          Frame(width: 620, height: 300) {
            Rail {
              RailGroup("Themes") { Rows() }
              NavAction("New theme", size: .small, action: {})
            }
            Pane { Text("Studio Light") }
          }
        }

        Section(
          "Three columns",
          description: """
            Sidebar, rail, content — and the aside on the far edge when the \
            selection has details worth carrying. Each column's width is a \
            `frame.*` metric rather than a value the view decides.
            """
        ) {
          Frame(width: 820, height: 320) {
            Sidebar {
              SidebarGroup("Library") {
                NavItem("Design system", icon: .paintBrush16Regular, active: true, action: {})
              }
            }
            Rail { RailGroup("Themes") { Rows() } }
            Pane { Text("Studio Light") }
          }
        }
      }
    }
  }
}
