import DazzUI
import SwiftUI

/// `Components/Code` — a run of code inside a sentence.
enum CodeStories {
  static var page: StoryPage {
    StoryPage("Code") {
      Story("Overview", layout: .page) { Overview() }
      Story("Inline") { Code("pnpm parity") }
      Story("In prose") { sentence.frame(width: 416) }
    }
  }

  /// SwiftUI lays a paragraph out as one text run, and a view cannot sit inside
  /// one — so a sentence with code in it is a row of runs.
  static var sentence: some View {
    HStack(alignment: .firstTextBaseline, spacing: 4) {
      Text("Pass")
      Code("--features runtime-shaders")
      Text("then run")
      Code("cargo test")
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Code",
        description:
          "A run of code inside a sentence — a flag, a path, an identifier. It is a wash and a "
          + "face, not a box: the chip corner and the half-step inset keep it separable from "
          + "the prose around it without breaking the line's rhythm, which is what a border at "
          + "this size always does."
      ) {
        Section(
          "In a sentence",
          description:
            "The face is the code face, the one place a view reaches a base primitive — "
            + "exactly as a mono TextField does. The size steps down one from the prose it "
            + "sits in: a monospace face at the same nominal size always reads larger, and the "
            + "run would ride above the line."
        ) {
          CodeStories.sentence
        }
        Section(
          "What it is not",
          description:
            "A run of code is not a KeyCap and not a Badge. A KeyCap is a key you press and is "
            + "drawn as one; a Badge is an annotation about something else. This is the thing "
            + "itself, quoted."
        ) {
          Stack {
            Specimen("an identifier") { Code("ThemeVariables") }
            Specimen("a path") { Code("ds/tokens/global/_base.yaml") }
            Specimen("a flag") { Code("--no-verify") }
          }
        }
      }
    }
  }
}
