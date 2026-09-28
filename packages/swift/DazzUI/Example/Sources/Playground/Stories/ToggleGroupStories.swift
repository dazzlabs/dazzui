import DazzUI
import SwiftUI

/// `Components/ToggleGroup` — several toggles joined into one control.
enum ToggleGroupStories {
  static let show = [
    ToggleItem(value: "whitespace", label: "Whitespace"),
    ToggleItem(value: "minimap", label: "Minimap"),
    ToggleItem(value: "blame", label: "Blame"),
  ]

  static let styles = [
    ToggleItem(value: "bold", label: "B"),
    ToggleItem(value: "italic", label: "I"),
    ToggleItem(value: "underline", label: "U"),
  ]

  static var page: StoryPage {
    StoryPage("ToggleGroup") {
      Story("Overview", layout: .page) { Overview() }
      Story("Glyphs") { Live(items: styles, on: ["bold"]) }
      Story("Words") { Live(items: show, on: ["whitespace"]) }
      Story("Single") { Live(items: show, on: ["minimap"], multiple: false) }
      Story("Variants") {
        VStack(alignment: .leading, spacing: 8) {
          ForEach(
            [ControlVariant.recessed, .normal, .outlined, .plain],
            id: \.self
          ) { variant in
            ToggleGroup(items: show, value: ["whitespace"], variant: variant)
          }
        }
      }
      Story("Tints") {
        VStack(alignment: .leading, spacing: 8) {
          ForEach(Tint.allCases, id: \.self) { tint in
            ToggleGroup(items: show, value: ["whitespace"], tint: tint)
          }
        }
      }
      Story("Disabled") {
        ToggleGroup(
          items: [
            ToggleItem(value: "whitespace", label: "Whitespace"),
            ToggleItem(value: "minimap", label: "Minimap"),
            ToggleItem(value: "blame", label: "Blame", enabled: false),
          ],
          value: ["whitespace"]
        )
      }
    }
  }

  struct Live: View {
    let items: [ToggleItem]
    @State var on: [String]
    var multiple = true

    var body: some View {
      ToggleGroup(items: items, value: on, multiple: multiple) { next in on = next }
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "ToggleGroup",
        description:
          "Several toggles that belong together, joined into one control. Joined rather than "
          + "merely adjacent: the corners are rounded at the ends of the run only and the "
          + "shared edges collapse to one, which is what says the group is one thing with "
          + "several states rather than three buttons that happen to touch."
      ) {
        Section(
          "Beside a SegmentedControl",
          description:
            "This is where the two part company. A segmented control is a choice between its "
            + "segments and always has exactly one; a toggle group can have several on at "
            + "once, or none. Drawing a mandatory single choice with this one would let the "
            + "reader clear a choice that cannot be cleared."
        ) {
          Stack {
            Specimen("SegmentedControl — one, always") {
              SegmentedControl(
                items: [
                  SegmentedItem(value: "source", label: "Source"),
                  SegmentedItem(value: "preview", label: "Preview"),
                ],
                value: "source"
              )
            }
            Specimen("ToggleGroup — any number, including none") {
              ToggleGroup(items: ToggleGroupStories.styles, value: ["bold"])
            }
          }
        }
        Section(
          "The collapsed edge",
          description:
            "The ring belongs to the group and the members draw none of their own, so the rule "
            + "between a pair is a single hairline rather than two stacked. That is "
            + "border-collapse done the only way a row of boxes can do it, and it is what "
            + "makes the run read as one control."
        ) {
          ToggleGroup(items: ToggleGroupStories.show, value: ["whitespace"])
        }
      }
    }
  }
}
