import DazzUI
import SwiftUI

/// `Components/Popover` — a panel of anything, anchored to what opened it.
enum PopoverStories {
  static var page: StoryPage {
    StoryPage("Popover") {
      Story("Overview", layout: .page) { Overview() }
      Story("Bare") { about() }
      Story("Sides") {
        Row {
          ForEach(PopoverSide.allCases, id: \.self) { side in
            about(side: side, label: side.rawValue)
          }
        }
      }
      Story("With controls") { Filter() }
      Story("A form") { Rename() }
      Story("In a toolbar") {
        Row {
          Filter()
          Popover(
            title: "About",
            description: "42 pull requests, 8 of them yours.",
            align: .end
          ) { state in
            IconButton(label: "About", icon: .info16Regular, action: state.toggle)
          }
        }
      }
    }
  }

  static func about(side: PopoverSide = .bottom, label: String = "About this build") -> some View {
    Popover(
      title: "Build 4.2.1",
      description: "Published 3 days ago from main.",
      side: side
    ) { state in
      Button(label, action: state.toggle)
    }
  }

  struct Filter: View {
    @State private var states: Set<String> = ["Open"]

    var body: some View {
      Popover(
        title: "Filter",
        align: .start,
        width: 256,
        trigger: { state in
          IconButton(label: "Filter", icon: .filter16Regular, action: state.toggle)
        },
        content: {
          Col {
            ForEach(["Open", "Merged", "Closed"], id: \.self) { name in
              Checkbox(
                name,
                checked: states.contains(name),
                onCheckedChange: { on in
                  if on { states.insert(name) } else { states.remove(name) }
                }
              )
            }
          }
        }
      )
    }
  }

  private struct Rename: View {
    @State private var name = "release/4.2"

    var body: some View {
      Popover(
        title: "Rename branch",
        description: "The remote keeps its own name until you push.",
        width: 320,
        trigger: { state in
          Button("Rename branch", variant: .filled, action: state.toggle)
        },
        content: {
          FormField(label: "Branch name") { TextField(text: $name, mono: true) }
        }
      )
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Popover",
        description:
          "A panel of anything, anchored to what opened it. It is the sibling of Menu rather "
          + "than of Dialog: it wears the same kind of floating sheet and is positioned against "
          + "its trigger, but it holds content rather than rows. Reach for a dialog instead "
          + "when the reader has to finish before doing anything else — a popover closes on "
          + "Escape, which makes it the wrong home for something half-typed."
      ) {
        Section(
          "A heading and a line",
          description:
            "The title names the panel and the description describes it. The gap between the "
            + "panel and its trigger is a token rather than a number passed in."
        ) {
          PopoverStories.about()
        }
        Section(
          "Sides",
          description: "Anchored to whichever edge of the trigger it is told."
        ) {
          Stack {
            ForEach(PopoverSide.allCases, id: \.self) { side in
              Specimen(side.rawValue) {
                PopoverStories.about(side: side, label: side.rawValue)
              }
            }
          }
        }
        Section(
          "A panel with controls in it",
          description:
            "This is what separates it from a Tooltip: a popover can be reached and pressed. A "
            + "tooltip must carry nothing a reader needs, because a pointer never gets there."
        ) {
          Filter()
        }
      }
    }
  }
}
