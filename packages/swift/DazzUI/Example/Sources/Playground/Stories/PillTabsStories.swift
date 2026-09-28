import DazzUI
import SwiftUI

/// `Components/PillTabs` — the filter row over a list.
enum PillTabsStories {
  static var page: StoryPage {
    StoryPage("PillTabs") {
      Story("Overview", layout: .page) { Overview() }
      Story("Default") { Tabs(counts: false) }
      Story("With counts") { Tabs(counts: true) }
      Story("Disabled") {
        PillTabs(
          items: [
            PillTabItem(value: "all", label: "All", count: 24),
            PillTabItem(value: "open", label: "Open", count: 3),
            PillTabItem(value: "archived", label: "Archived", disabled: true),
          ],
          value: "all"
        )
      }
      Story("Tints") {
        Col {
          ForEach(Tint.allCases, id: \.self) { tint in
            PillTabs(
              items: [
                PillTabItem(value: "all", label: "All"),
                PillTabItem(value: "open", label: label(tint)),
              ],
              value: "open",
              tint: tint
            )
          }
        }
      }
      Story("Wrapping") {
        PillTabs(
          items: [
            PillTabItem(value: "all", label: "All", count: 124),
            PillTabItem(value: "open", label: "Open", count: 12),
            PillTabItem(value: "review", label: "In review", count: 4),
            PillTabItem(value: "blocked", label: "Blocked", count: 2),
            PillTabItem(value: "done", label: "Done", count: 106),
            PillTabItem(value: "archived", label: "Archived"),
          ],
          value: "all"
        )
        .frame(width: 380)
      }
      Story("In a pane header") {
        Frame(width: 560, height: 260) {
          Pane {
            Text("Builds")
            Tabs(counts: true)
            DazzUI.Divider()
          }
        }
      }
    }
  }

  private struct Tabs: View {
    let counts: Bool
    @State private var tab = "all"

    var body: some View {
      PillTabs(
        items: [
          PillTabItem(value: "all", label: "All", count: counts ? 24 : nil),
          PillTabItem(value: "open", label: "Open", count: counts ? 3 : nil),
          PillTabItem(value: "done", label: "Done", count: counts ? 21 : nil),
        ],
        value: tab,
        onValueChange: { tab = $0 }
      )
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "PillTabs",
        description: """
          A run of pills that filters the list under it. Unlike a segmented \
          control it has no track: the pills sit on the page, so the row can \
          wrap and can carry counts without turning into a toolbar.
          """
      ) {
        Section(
          "Selection",
          description: """
            The active pill takes the strong label face as well as the tint's \
            wash — the same rule the nav row follows, and the reason the row \
            still reads when the theme's accent is quiet.
            """
        ) {
          Stack {
            Specimen("plain") { Tabs(counts: false) }
            Specimen("counts") { Tabs(counts: true) }
            Specimen("disabled") {
              PillTabs(
                items: [
                  PillTabItem(value: "all", label: "All", count: 24),
                  PillTabItem(value: "archived", label: "Archived", disabled: true),
                ],
                value: "all"
              )
            }
          }
        }

        Section(
          "Tints",
          description: """
            One recipe table, six ramps — the pill's wash is the tint's, so a \
            filter row can carry the meaning of what it filters.
            """
        ) {
          Col {
            ForEach(Tint.allCases, id: \.self) { tint in
              PillTabs(
                items: [
                  PillTabItem(value: "all", label: "All"),
                  PillTabItem(value: "open", label: label(tint)),
                ],
                value: "open",
                tint: tint
              )
            }
          }
        }
      }
    }
  }
}
