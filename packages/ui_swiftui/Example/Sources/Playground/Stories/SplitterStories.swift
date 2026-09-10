import DazzUI
import SwiftUI

/// `Components/Splitter` — regions the reader can re-proportion.
enum SplitterStories {
  static var page: StoryPage {
    StoryPage("Splitter") {
      Story("Overview", layout: .page) { Overview() }
      Story("Horizontal") { window(544, 192) { pair } }
      Story("Vertical") { window(480, 224) { stack } }
      Story("Three panels") { window(608, 224) { three } }
      Story("Constrained") { window(544, 192) { constrained } }
      Story("Nested") { window(608, 256) { nested } }
    }
  }

  /// A region with a name in it, so a seam can be seen to move something.
  static func region(_ label: String) -> some View {
    VStack(alignment: .leading) {
      SectionLabel(label)
      Spacer(minLength: 0)
    }
    .padding(12)
    .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
  }

  /// The window the specimens sit in — a bordered box, since a splitter paints
  /// nothing of its own and would otherwise be seams in mid-air.
  static func window<Content: View>(
    _ width: CGFloat,
    _ height: CGFloat,
    @ViewBuilder content: () -> Content
  ) -> some View {
    content()
      .frame(width: width, height: height)
      .clipShape(RoundedRectangle(cornerRadius: 8, style: .circular))
  }

  static var pair: some View {
    Splitter(panels: [
      SplitterPanel(defaultSize: 50) { region("Before") },
      SplitterPanel(defaultSize: 50) { region("After") },
    ])
  }

  static var stack: some View {
    Splitter(
      orientation: .vertical,
      panels: [
        SplitterPanel(defaultSize: 40, minSize: 20) { region("Editor") },
        SplitterPanel(defaultSize: 60, minSize: 20) { region("Output") },
      ]
    )
  }

  static var three: some View {
    Splitter(panels: [
      SplitterPanel(defaultSize: 24, minSize: 15, maxSize: 40) { region("Files") },
      SplitterPanel(defaultSize: 52, minSize: 30) { region("Document") },
      SplitterPanel(defaultSize: 24, minSize: 15, maxSize: 40) { region("Inspector") },
    ])
  }

  static var constrained: some View {
    Splitter(panels: [
      SplitterPanel(defaultSize: 30, minSize: 25, maxSize: 45) { region("25–45%") },
      SplitterPanel(defaultSize: 70) { region("the rest") },
    ])
  }

  static var nested: some View {
    Splitter(panels: [
      SplitterPanel(defaultSize: 30, minSize: 20) { region("Files") },
      SplitterPanel(defaultSize: 70) {
        Splitter(
          orientation: .vertical,
          panels: [
            SplitterPanel(defaultSize: 65) { region("Editor") },
            SplitterPanel(defaultSize: 35) { region("Terminal") },
          ]
        )
      },
    ])
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Splitter",
        description:
          "Two or more regions the reader can re-proportion. A desktop window is mostly this — "
          + "a column beside a document beside an inspector — and which of the three deserves "
          + "the room is the reader's decision rather than the designer's."
      ) {
        Section(
          "Beside a Sidebar",
          description:
            "Where a column has one right width and the reader only opens or shuts it, that is "
            + "Sidebar instead: it carries its own width token and its own rail, and drawing "
            + "it with a splitter would offer a choice the design has already made."
        ) {
          SplitterStories.window(608, 208) { SplitterStories.three }
        }
        Section(
          "The seam",
          description:
            "The handle is drawn from the space between two panels rather than added to it: "
            + "the hit target is a whole step of the scale, so grabbing a seam does not demand "
            + "pixel accuracy, while the rule itself stays the hairline every other separator "
            + "in the system is. Under the pointer it takes the accent and swells — without "
            + "reflowing, since a seam that pushed its neighbours aside on the way to a drag "
            + "would be a layout jump."
        ) {
          SplitterStories.window(544, 160) { SplitterStories.pair }
        }
        Section(
          "Who owns the shares",
          description:
            "The splitter does. onSizesChange is for persisting them — a drag that had to "
            + "round-trip through a host's state would lag the pointer, and a seam that lags "
            + "the pointer feels broken."
        ) {
          SplitterStories.window(480, 176) { SplitterStories.stack }
        }
      }
    }
  }
}
