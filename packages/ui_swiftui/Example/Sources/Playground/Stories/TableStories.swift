import DazzUI
import SwiftUI

/// `Components/Table` — rows of one thing.
enum TableStories {
  static var page: StoryPage {
    StoryPage("Table") {
      Story("Overview", layout: .page) { Overview() }
      Story("Selectable") { Builds(interactive: true) }
      Story("Inert") { Builds(interactive: false) }
      Story("Alignment") {
        DazzUI.Table {
          TableHead {
            TableCell(head: true) { Text("Package") }
            TableCell(head: true, align: .end) { Text("Tokens") }
          }
          TableRow {
            TableCell { Text("ui_react") }
            TableCell(align: .end) { Text("312") }
          }
          TableRow {
            TableCell { Text("ui_swiftui") }
            TableCell(align: .end) { Text("312") }
          }
        }
        .frame(width: 380)
      }
      Story("With controls") { WithControls() }
      Story("Tints") {
        Col {
          ForEach(Tint.allCases, id: \.self) { tint in
            DazzUI.Table(tint: tint) {
              TableRow(active: true, interactive: true, action: {}) {
                TableCell { Text(label(tint)) }
                TableCell(align: .end) { Text("selected") }
              }
            }
            .frame(width: 320)
          }
        }
      }
      Story("Empty") {
        Card(variant: .sunken) {
          EmptyState("No builds have run on this branch")
            .frame(maxWidth: .infinity)
        }
        .frame(width: 420)
      }
    }
  }

  private struct Build: Identifiable {
    let id: String
    let branch: String
    let status: String
    let tint: Tint

    static let all = [
      Build(id: "aurora", branch: "main", status: "Passed", tint: .success),
      Build(id: "beacon", branch: "release/2.1", status: "Running", tint: .info),
      Build(id: "cinder", branch: "fix/tokens", status: "Failed", tint: .danger),
    ]
  }

  private struct Builds: View {
    let interactive: Bool
    @State private var selected = "aurora"

    var body: some View {
      DazzUI.Table {
        TableHead {
          TableCell(head: true) { Text("Build") }
          TableCell(head: true) { Text("Branch") }
          TableCell(head: true, align: .end) { Text("Status") }
        }

        ForEach(Build.all) { build in
          TableRow(
            active: interactive && selected == build.id,
            interactive: interactive,
            action: interactive ? { selected = build.id } : nil
          ) {
            TableCell { Text(build.id) }
            TableCell { Text(build.branch) }
            TableCell(align: .end) {
              Badge(build.status, size: .small, variant: .tinted, tint: build.tint)
            }
          }
        }
      }
      .frame(width: 420)
    }
  }

  private struct WithControls: View {
    @State private var enabled: Set<String> = ["aurora"]

    var body: some View {
      DazzUI.Table {
        TableHead {
          TableCell(head: true) { Text("Build") }
          TableCell(head: true, align: .end) { Text("Notify") }
        }
        ForEach(Build.all) { build in
          TableRow {
            TableCell { Text(build.id) }
            TableCell(align: .end) {
              Switch(
                checked: enabled.contains(build.id),
                onCheckedChange: { on in
                  if on { enabled.insert(build.id) } else { enabled.remove(build.id) }
                },
                size: .small
              )
            }
          }
        }
      }
      .frame(width: 380)
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Table",
        description: """
          A header row and a run of rows under it. The parts are separate views \
          rather than a data source, so a cell holds whatever a cell should \
          hold — a badge, a switch, a number set to the trailing edge.
          """
      ) {
        Section(
          "Rows",
          description: """
            A row is inert unless it is told otherwise. `interactive` gives it \
            the hover wash and a click; `active` marks the selected one. Hover \
            outranks selection, so moving over a selected row still says which \
            row the pointer is on.
            """
        ) {
          Row(alignment: .top) {
            Builds(interactive: true)
            Builds(interactive: false)
          }
        }

        Section(
          "Cells",
          description: """
            A cell aligns to the start by default and to the end where it holds \
            a number or a control — which is what keeps a column of figures \
            readable and a column of switches on one edge.
            """
        ) {
          WithControls()
        }

        Section(
          "Empty",
          description: """
            A table with no rows is not a table with a message in it: the empty \
            state replaces the table, in the well the table would have sat in.
            """
        ) {
          Card(variant: .sunken) {
            EmptyState("No builds have run on this branch")
              .frame(maxWidth: .infinity)
          }
          .frame(width: 420)
        }
      }
    }
  }
}
