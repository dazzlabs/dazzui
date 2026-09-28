import DazzUI
import FluentIcons
import SwiftUI

/// `Components/Sidebar` — the first navigation column.
enum SidebarStories {
  static var page: StoryPage {
    StoryPage("Sidebar") {
      Story("Overview", layout: .page) { Overview() }
      Story("A column") {
        Frame(width: 560, height: 320) {
          Sidebar { Library() }
          Pane { Text("Projects") }
        }
      }
      Story("Groups") {
        Frame(width: 560, height: 320) {
          Sidebar {
            Library()
            SidebarGroup("Recent") {
              NavItem("aurora", action: {})
              NavItem("beacon", action: {})
            }
          }
          Pane { Text("Projects") }
        }
      }
      Story("With a header") {
        Frame(width: 560, height: 320) {
          Sidebar {
            Badge("DazzUI", size: .small, variant: .tinted)
          } content: {
            Library()
          }
          Pane { Text("Projects") }
        }
      }
      Story("With a card") {
        Frame(width: 560, height: 320) {
          Sidebar {
            Library()
            Spacer(minLength: 0)
            SidebarCard("Build") {
              Progress(value: 0.75)
              Text("3 of 4 packages")
            }
          }
          Pane { Text("Projects") }
        }
      }
      Story("With an action") {
        Frame(width: 560, height: 320) {
          Sidebar {
            Library()
            NavAction("New project", action: {})
          }
          Pane { Text("Projects") }
        }
      }
      Story("Resizable") {
        Frame(width: 620, height: 320) {
          Sidebar(resizable: true) { Library() }
          Pane { Text("Drag the trailing edge.") }
        }
      }
      Story("Collapsing") { Collapsing() }
    }
  }

  private struct Library: View {
    @State private var selected = "projects"

    private let rows: [(String, String, FluentIcon)] = [
      ("projects", "Projects", .folder16Regular),
      ("archive", "Archive", .archive16Regular),
      ("settings", "Settings", .settings16Regular),
    ]

    var body: some View {
      SidebarGroup("Library") {
        ForEach(rows, id: \.0) { id, name, icon in
          NavItem(name, icon: icon, active: selected == id) { selected = id }
        }
      }
    }
  }

  private struct Collapsing: View {
    @State private var open = true

    var body: some View {
      Frame(width: 620, height: 320) {
        if open {
          Sidebar(resizable: true, onCollapse: { open = false }) { Library() }
        }
        Pane {
          Button(open ? "Hide the sidebar" : "Show the sidebar", variant: .recessed) {
            open.toggle()
          }
        }
      }
    }
  }

  private struct Overview: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Page(
        "Sidebar",
        description: """
          The first navigation column: what the window is *about*, in groups of \
          rows. It carries the muted surface — the same one the aside wears — so \
          a window reads as content between two quieter edges.
          """
      ) {
        Section(
          "Groups",
          description: """
            Rows are broken into groups rather than stacked. The gap does most \
            of the work and the label only names it, which is why a first group \
            can go unlabelled where the window's title already says what it is.
            """
        ) {
          Frame(width: 620, height: 300) {
            Sidebar {
              Library()
              SidebarGroup("Recent") {
                NavItem("aurora", action: {})
                NavItem("beacon", action: {})
              }
            }
            Pane { Text("Projects") }
          }
        }

        Section(
          "Header, card and action",
          description: """
            The header sits on the titlebar's own height, so it lines up with \
            whatever is across the top of the window. A pinned `SidebarCard` \
            stays put while the rows scroll — only the row list scrolls — and a \
            trailing `NavAction` sinks to the foot.
            """
        ) {
          Row(alignment: .top) {
            Frame(width: 300, height: 280) {
              Sidebar {
                Badge("DazzUI", size: .small, variant: .tinted)
              } content: {
                Library()
                Spacer(minLength: 0)
                SidebarCard("Build") {
                  Progress(value: 0.75)
                  Text("3 of 4 packages")
                }
              }
            }
            Frame(width: 300, height: 280) {
              Sidebar {
                Library()
                NavAction("New project", action: {})
              }
            }
          }
        }

        Section(
          "Resizing",
          description: """
            The trailing edge is a drag handle when `resizable` is set, and a \
            double-click sends it home to the `frame.sidebar` width. The token \
            is already a number of points here, so the natural width can simply \
            be read rather than measured off the rendered column.
            """
        ) {
          Collapsing()
        }

        Section("Tokens", description: "The column widths a shell is built on.") {
          TokenTable([
            TokenRow("frame.sidebar-width", theme.vars.frameSidebarWidth),
            TokenRow("frame.rail-width", theme.vars.frameRailWidth),
            TokenRow("frame.aside-width", theme.vars.frameAsideWidth),
            TokenRow("frame.titlebar-size", theme.vars.frameTitlebarSize),
            TokenRow("frame.nav-gap", theme.vars.frameNavGap),
          ])
        }
      }
    }
  }
}
