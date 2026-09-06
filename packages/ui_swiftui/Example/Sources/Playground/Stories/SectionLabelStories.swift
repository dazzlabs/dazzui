import DazzUI
import SwiftUI

/// `Components/SectionLabel` — the heading over a run of rows.
enum SectionLabelStories {
  static var page: StoryPage {
    StoryPage("SectionLabel") {
      Story("Overview", layout: .page) { Overview() }
      Story("Default") { SectionLabel("Library") }
      Story("Faint") { SectionLabel("Library", faint: true) }
      Story("Tints") {
        Col {
          ForEach(Tint.allCases, id: \.self) { tint in
            SectionLabel(label(tint), tint: tint)
          }
        }
      }
      Story("In a panel") {
        Card {
          SectionLabel("Details")
          Text("aurora")
          DazzUI.Divider()
          SectionLabel("Timing")
          Text("2m 14s")
        }
        .frame(width: 260)
      }
      Story("In a column") {
        Frame(width: 520, height: 280) {
          Sidebar {
            SidebarGroup("Library") {
              NavItem("Projects", active: true, action: {})
              NavItem("Archive", action: {})
            }
            SidebarGroup("Recent") {
              NavItem("aurora", action: {})
            }
          }
          Pane { Text("Projects") }
        }
      }
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "SectionLabel",
        description: """
          The quiet heading that names a run of rows. A sidebar group already \
          wears one; this is the same label reached directly, for a panel or a \
          column that is not a sidebar.
          """
      ) {
        Section(
          "Tone",
          description: """
            It takes the subtle ink by default and the faint one where the \
            label is only there to break a long run. A tint colours it where a \
            section carries a meaning of its own — a danger zone in a settings \
            page.
            """
        ) {
          Stack {
            Specimen("default") { SectionLabel("Library") }
            Specimen("faint") { SectionLabel("Library", faint: true) }
            Specimen("tinted") {
              ForEach(Tint.allCases, id: \.self) { tint in
                SectionLabel(label(tint), tint: tint)
              }
            }
          }
        }

        Section(
          "Where it lands",
          description: """
            Over a run of rows in a panel or an aside. In a sidebar the group \
            draws it, so a label written by hand there would be a second \
            heading over the same rows.
            """
        ) {
          Row(alignment: .top) {
            Card {
              SectionLabel("Details")
              Text("aurora")
              DazzUI.Divider()
              SectionLabel("Timing")
              Text("2m 14s")
            }
            .frame(width: 240)

            Frame(width: 360, height: 220) {
              Sidebar {
                SidebarGroup("Library") {
                  NavItem("Projects", active: true, action: {})
                }
                SidebarGroup("Recent") {
                  NavItem("aurora", action: {})
                }
              }
              Pane { Text("Projects") }
            }
          }
        }
      }
    }
  }
}
