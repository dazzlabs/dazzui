import DazzUI
import SwiftUI

/// `Components/Accordion` — several disclosures that know about each other.
enum AccordionStories {
  static let faq: [(String, String, String)] = [
    (
      "where", "Where do the values come from?",
      "Every colour, length and face is a token in ds. A view reads the variable; it "
        + "never writes a value of its own."
    ),
    (
      "themes", "How many themes are there?",
      "Twelve, as six families over one default. The modes compose, so a value shared along the "
        + "light/dark axis is written once."
    ),
    (
      "ports", "What keeps the platforms in line?",
      "The parity pipeline measures the React build in a browser and holds every port to those "
        + "numbers."
    ),
  ]

  static var items: [AccordionItem] {
    faq.map { id, title, body in
      AccordionItem(id, title: title) { Text(body) }
    }
  }

  static var page: StoryPage {
    StoryPage("Accordion") {
      Story("Overview", layout: .page) { Overview() }
      Story("One at a time") { Sections() }
      Story("Multiple") { Sections(multiple: true) }
      Story("Divided") { Sections(divided: true) }
      Story("Disabled") {
        Accordion(
          items: [
            AccordionItem(faq[0].0, title: faq[0].1) { Text(faq[0].2) },
            AccordionItem("locked", title: "Managed by your admin", disabled: true) {
              Text("Nothing here can be changed on this device.")
            },
          ],
          initiallyOpen: ["where"],
          divided: true
        )
        .frame(width: 448)
      }
    }
  }

  struct Sections: View {
    var multiple = false
    var divided = false

    @State private var open: Set<String> = ["where"]

    var body: some View {
      Accordion(
        items: AccordionStories.items,
        open: open,
        onOpenChange: { open = $0 },
        multiple: multiple,
        divided: divided
      )
      .frame(width: 448)
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Accordion",
        description:
          "Several disclosures that know about each other. That knowledge is the whole "
          + "difference from a run of Collapsibles: by default opening one shuts the last, so a "
          + "reader is always looking at one section rather than at a page that grew while they "
          + "were reading it."
      ) {
        Section(
          "One at a time",
          description:
            "The default, and the one to reach for when the sections are alternatives — a FAQ, "
            + "a set of steps, anything where two open at once would push the third off the "
            + "screen."
        ) {
          Sections()
        }
        Section(
          "Several at once",
          description:
            "`multiple` is for the case where the reader is comparing rather than choosing — "
            + "two log sections side by side, two diffs. It is a deliberate opt-in, because a "
            + "list that can be opened all the way is a list that can be scrolled past."
        ) {
          Sections(multiple: true)
        }
        Section(
          "Divided",
          description:
            "Undivided, the sections sit on one run and the rows are separated by their own "
            + "hover wash alone. `divided` draws the hairline, for the case where they are "
            + "genuinely a list and the reader is counting them."
        ) {
          Sections(divided: true)
        }
      }
    }
  }
}
