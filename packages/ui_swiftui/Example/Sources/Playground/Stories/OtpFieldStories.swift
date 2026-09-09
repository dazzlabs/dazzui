import DazzUI
import SwiftUI

/// `Components/OtpField` — the one-time code a reader is copying.
enum OtpFieldStories {
  static var page: StoryPage {
    StoryPage("OtpField") {
      Story("Overview", layout: .page) { Overview() }
      Story("Bare") { Live() }
      Story("Filled") { Live(start: "204815") }
      Story("Grouped") { Live(start: "204815", groupAfter: 3) }
      Story("Sizes") {
        Col {
          ForEach(TextFieldSize.allCases, id: \.self) { size in
            Live(start: "204", size: size)
          }
        }
      }
      Story("Masked") { Live(start: "2048", length: 4, mask: true) }
      Story("Disabled") {
        OtpField(value: .constant("204815"), length: 6, enabled: false)
      }
      Story("In a form") { Verify() }
    }
  }

  struct Live: View {
    var start = ""
    var length = 6
    var groupAfter: Int?
    var mask = false
    var size: TextFieldSize = .medium

    @State private var value: String?

    var body: some View {
      OtpField(
        value: Binding(get: { value ?? start }, set: { value = $0 }),
        length: length,
        groupAfter: groupAfter,
        mask: mask,
        size: size
      )
    }
  }

  private struct Verify: View {
    @State private var code = ""

    private var wrong: Bool { code.count == 6 && code != "204815" }

    var body: some View {
      Col(spacing: 16, width: 320) {
        Callout(tint: .info) { Text("We sent a six-digit code to ada@example.com.") }
        FormField(
          label: "Verification code",
          hint: wrong ? "That code did not match" : "It expires in 10 minutes",
          state: wrong ? .error : .default
        ) {
          OtpField(
            value: $code,
            length: 6,
            groupAfter: 3,
            state: wrong ? .error : .default
          )
        }
        Button("Verify", variant: .filled, enabled: code.count == 6) {}
      }
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "OtpField",
        description:
          "The one-time code a reader is copying out of a message. It is drawn as a run of "
          + "cells but it is one field underneath, and that is deliberate: a paste of the whole "
          + "code fills every cell at once, the platform’s own code autofill has somewhere to "
          + "put it, and the caret only ever has one place to be. A run of separate inputs gets "
          + "all three wrong and has to reinvent the moves between them."
      ) {
        Section(
          "Sizes",
          description:
            "Each cell is the text controls’ box at one character wide, so a code sits on the "
            + "same rhythm as the fields around it — a code entered in a box of its own design "
            + "would be the only control on the page that was."
        ) {
          Stack {
            ForEach(TextFieldSize.allCases, id: \.self) { size in
              Specimen(label(size)) { Live(start: "204", size: size) }
            }
          }
        }
        Section(
          "Grouping",
          description:
            "`groupAfter` breaks the run where a printed code breaks — three and three is how a "
            + "six-digit code is read aloud. It is a step of air rather than a rule, because a "
            + "rule would read as two fields."
        ) {
          Stack {
            Specimen("ungrouped") { Live(start: "204815") }
            Specimen("groupAfter 3") { Live(start: "204815", groupAfter: 3) }
          }
        }
        Section(
          "Masked and disabled",
          description:
            "`mask` is for a code that is a secret rather than a one-time token — a PIN. A "
            + "filled cell lifts to the paper and keeps it, so the run says how far through the "
            + "code the reader is."
        ) {
          Stack {
            Specimen("masked") { Live(start: "20", length: 4, mask: true) }
            Specimen("disabled") {
              OtpField(value: .constant("204815"), length: 6, enabled: false)
            }
          }
        }
      }
    }
  }
}
