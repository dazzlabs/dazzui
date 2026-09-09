import DazzUI
import SwiftUI

/// `Components/Collapsible` — one thing that opens.
enum CollapsibleStories {
  static let log = """
    $ pnpm test --filter core
      ✓ resolves a relative import  (4ms)
      ✓ resolves a bare specifier   (2ms)
      ✓ falls back to the index     (3ms)
    """

  static var page: StoryPage {
    StoryPage("Collapsible") {
      Story("Overview", layout: .page) { Overview() }
      Story("Shut") { environment(open: false).frame(width: 352) }
      Story("Open") { environment(open: true).frame(width: 352) }
      Story("With meta") { tests.frame(width: 416) }
      Story("Disabled") {
        Collapsible(title: "Environment", initiallyOpen: false, enabled: false) {
          Text("Node 24.20, pnpm 10.4, macOS 15.5")
        }
        .frame(width: 352)
      }
      Story("Interactive") { Live() }
    }
  }

  static func environment(open: Bool) -> some View {
    Collapsible(title: "Environment", initiallyOpen: open) {
      Text("Node 24.20, pnpm 10.4, macOS 15.5")
    }
  }

  static var tests: some View {
    Collapsible(
      title: "Ran 3 tests",
      icon: .beaker16Regular,
      initiallyOpen: true,
      meta: { Badge("passed", tint: .success) },
      content: { Text(log).font(.system(size: 12, design: .monospaced)) }
    )
  }

  private struct Live: View {
    @State private var open = false

    var body: some View {
      Collapsible(
        title: "Request headers",
        open: open,
        onOpenChange: { open = $0 },
        meta: { Text(open ? "hide" : "show") },
        content: {
          Text("accept: application/json\nauthorization: Bearer ****")
            .font(.system(size: 12, design: .monospaced))
        }
      )
      .frame(width: 384)
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Collapsible",
        description:
          "One thing that opens. The header row is the affordance and the panel is what it "
          + "discloses — a tool call’s output, a stack trace, the rest of a long description. "
          + "It is the smallest of the three ways this system hides something: a collapsible "
          + "keeps its subject in place and only folds it, where a Popover moves it off the "
          + "page and a Dialog stops the page."
      ) {
        Section(
          "Shut and open",
          description:
            "The chevron points along the row when shut and down the page when open — one glyph "
            + "rotated rather than two swapped, so the turn is the animation. It is the only "
            + "moving part."
        ) {
          Stack {
            Specimen("shut") { CollapsibleStories.environment(open: false).frame(width: 352) }
            Specimen("open") { CollapsibleStories.environment(open: true).frame(width: 352) }
          }
        }
        Section(
          "The header row",
          description:
            "A glyph before the title and an aside after it. The aside is what makes a shut row "
            + "worth reading — a count, a duration, a status — so the reader can decide whether "
            + "to open it at all."
        ) {
          CollapsibleStories.tests.frame(width: 416)
        }
        Section(
          "Against an Accordion",
          description:
            "Several of these that know about each other are an Accordion, where opening one "
            + "shuts the last. A run of separate collapsibles does not, which is right when the "
            + "sections are unrelated and wrong when they are one list."
        ) {
          CollapsibleStories.environment(open: false).frame(width: 416)
        }
      }
    }
  }
}
