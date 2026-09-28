import DazzUI
import SwiftUI

/// `Components/Skeleton` — the shape of something that has not arrived.
enum SkeletonStories {
  static var page: StoryPage {
    StoryPage("Skeleton") {
      Story("Overview", layout: .page) { Overview() }
      Story("Shapes") {
        HStack(spacing: 12) {
          Skeleton(shape: .circle, width: 40)
          Skeleton(shape: .rect, width: 80, height: 48)
        }
      }
      Story("Text") { SkeletonText().frame(width: 320) }
      Story("Lines") { SkeletonText(lines: 5).frame(width: 320) }
      Story("A list row") { placeholder.frame(width: 320) }
      Story("In a card") { inACard.frame(width: 352) }
    }
  }

  /// An avatar and two lines: the shape a list row will land in.
  static var placeholder: some View {
    HStack(alignment: .top, spacing: 12) {
      Skeleton(shape: .circle, width: 40)
      VStack(alignment: .leading, spacing: 8) {
        Skeleton(width: 180)
        Skeleton(width: 120)
      }
    }
  }

  static var inACard: some View {
    Card {
      VStack(alignment: .leading, spacing: 12) {
        placeholder
        SkeletonText(lines: 2)
      }
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Skeleton",
        description:
          "The shape of something that has not arrived. A Spinner says the app is busy; a "
          + "skeleton says what is coming, and where it will be — which is why it belongs on "
          + "the first load of a view whose layout is already known, and a spinner belongs on "
          + "an action whose result is not. Drawing one for content that then lands somewhere "
          + "else is worse than drawing nothing."
      ) {
        Section(
          "Shapes",
          description:
            "text is a bar on the line box of the type it stands in for, so it needs no "
            + "height. circle is an avatar or a glyph and squares itself from its width "
            + "alone. rect is everything with a box of its own — a thumbnail, a chart, a card."
        ) {
          Stack {
            Specimen("text") { Skeleton(width: 80) }
            Specimen("circle") { Skeleton(shape: .circle, width: 40) }
            Specimen("rect") { Skeleton(shape: .rect, width: 80, height: 48) }
          }
        }
        Section(
          "Lines",
          description:
            "A paragraph's worth of bars, with the last one short. The ragged edge is most of "
            + "what makes a block of bars legible as text at all — a full-width final line "
            + "reads as a table."
        ) {
          SkeletonText().frame(width: 320)
        }
        Section(
          "Materials",
          description:
            "The fill is surface-sunken — the same dead step a disabled control flattens to, "
            + "which is exactly what a skeleton is. The sweep over it is surface-subtle, the "
            + "ink wash the whole system already hovers with. Unlike the spinner it takes no "
            + "reduced-motion exception: a still spinner reads as a hung app, where a still "
            + "skeleton still says everything it has to say."
        ) {
          SkeletonStories.inACard.frame(width: 352)
        }
      }
    }
  }
}
