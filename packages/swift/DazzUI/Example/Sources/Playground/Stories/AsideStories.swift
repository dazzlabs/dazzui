import DazzUI
import SwiftUI

/// `Components/Aside` — the trailing information column.
enum AsideStories {
  static var page: StoryPage {
    StoryPage("Aside") {
      Story("Overview", layout: .page) { Overview() }
      Story("Beside a pane") {
        Frame(width: 640) {
          Pane {
            Text("The selection")
          }
          Aside { Inspector() }
        }
      }
      Story("An inspector") {
        Frame(width: 420, height: 320) {
          Aside { Inspector() }
        }
      }
      Story("With an action") {
        Frame(width: 420, height: 320) {
          Aside {
            Inspector()
            Button("Open in Finder", variant: .recessed, fullWidth: true) {}
          }
        }
      }
      Story("In a three-column window") {
        Frame(width: 900, height: 380) {
          Sidebar {
            SidebarGroup("Library") {
              NavItem("Projects", icon: .folder16Regular, active: true) {}
              NavItem("Archive", icon: .archive16Regular) {}
            }
          }
          Pane {
            Text("aurora")
              .font(.title3)
          }
          Aside { Inspector() }
        }
      }
    }
  }

  /// What the current selection *is* — a run of labelled facts rather than
  /// rows you can navigate to.
  private struct Inspector: View {
    @Environment(\.theme) private var theme

    var body: some View {
      let vars = theme.vars

      SectionLabel("Details")

      VStack(alignment: .leading, spacing: vars.spacing3) {
        Fact(name: "Branch", value: "main")
        Fact(name: "Commit", value: "785f949")
        Fact(name: "Status", value: nil) {
          Badge("Passed", size: .small, variant: .tinted, tint: .success)
        }
      }

      SectionLabel("Timing")

      VStack(alignment: .leading, spacing: vars.spacing3) {
        Fact(name: "Started", value: "09:41")
        Fact(name: "Duration", value: "2m 14s")
      }
    }
  }

  private struct Fact<Accessory: View>: View {
    let name: String
    let value: String?
    @ViewBuilder var accessory: () -> Accessory

    @Environment(\.theme) private var theme

    init(
      name: String,
      value: String?,
      @ViewBuilder accessory: @escaping () -> Accessory = { EmptyView() }
    ) {
      self.name = name
      self.value = value
      self.accessory = accessory
    }

    var body: some View {
      let vars = theme.vars

      VStack(alignment: .leading, spacing: vars.spacing05) {
        Text(name)
          .font(vars.captionSmall.font)
          .foregroundStyle(vars.colorContentSubtle)

        if let value {
          Text(value)
            .font(vars.labelMedium.font)
            .foregroundStyle(vars.colorContent)
        }

        accessory()
      }
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Aside",
        description: """
          The trailing information column. It carries what the current \
          selection *is*, where the sidebar and the rail carry what you can \
          navigate to — which is why it hangs off the opposite edge and takes \
          no rows.
          """
      ) {
        Section(
          "Where it sits",
          description: """
            It shares the sidebar's muted surface rather than the rail's, so a \
            window reads as content between two quieter edges rather than as \
            three different greys. Its width is the `frame.aside` metric, not \
            a value this view decides.
            """
        ) {
          Frame(width: 720) {
            Pane { Text("The selection") }
            Aside { Inspector() }
          }
        }

        Section(
          "What goes in it",
          description: """
            Labelled facts, a badge where a fact has a state, and at most one \
            action at the foot. A row you can select belongs in the sidebar or \
            the rail instead.
            """
        ) {
          Frame(width: 420, height: 320) {
            Aside {
              Inspector()
              Button("Open in Finder", variant: .recessed, fullWidth: true) {}
            }
          }
        }
      }
    }
  }
}
