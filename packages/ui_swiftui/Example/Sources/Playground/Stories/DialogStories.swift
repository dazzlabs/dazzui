import DazzUI
import SwiftUI

/// `Components/Dialog` — the panel, and the scrim it is centred in.
enum DialogStories {
  static var page: StoryPage {
    StoryPage("Dialog") {
      Story("Overview", layout: .page) { Overview() }
      Story("A sheet", layout: .fullscreen) {
        DialogScrim {
          Dialog {
            DialogHeader(
              "Rebuild the token graph?",
              subtitle: "Four platform packages will be rewritten from packages/ds."
            )
            DialogBody {
              Text("The generated files are committed, so the diff is the review.")
            }
            DialogFooter {
              Button("Cancel", variant: .recessed) {}
              Button("Rebuild", variant: .filled) {}
            }
          }
        }
      }
      Story("A confirmation") {
        Dialog {
          DialogHeader("Discard these changes?")
          DialogBody {
            Text("The three files you edited go back to their last saved state.")
          }
          DialogFooter {
            Button("Keep editing", variant: .recessed) {}
            Button("Discard", variant: .filled) {}
          }
        }
      }
      Story("Danger") {
        Dialog(tone: .danger) {
          DialogHeader(
            "Delete the glossary?",
            subtitle: "1,204 entries. This cannot be undone."
          )
          DialogFooter {
            Button("Keep", variant: .recessed) {}
            Button("Delete", variant: .filled, tint: .danger) {}
          }
        }
      }
      Story("A form") { FormDialog() }
      Story("Widths") {
        Col {
          Dialog(width: 320) {
            DialogHeader("320")
            DialogBody { Text("A narrow sheet.") }
          }
          Dialog {
            DialogHeader("dialog.width")
            DialogBody { Text("The token, which is the default.") }
          }
        }
      }
      Story("Body only") {
        Dialog {
          DialogBody {
            Text("A sheet with nothing to confirm — only something to read.")
          }
        }
      }
      Story("Without a subtitle") {
        Dialog {
          DialogHeader("Rename this project")
          DialogFooter {
            Button("Cancel", variant: .recessed) {}
            Button("Rename", variant: .filled) {}
          }
        }
      }
      Story("Behind a trigger", layout: .fullscreen) { Triggered() }
    }
  }

  /// What a dialog looks like from the app's side: a control opens it, and
  /// the scrim covers the surface it was opened from.
  private struct Triggered: View {
    @State private var open = false

    var body: some View {
      ZStack {
        Button("Show a dialog", variant: .filled) { open = true }
          .frame(maxWidth: .infinity, maxHeight: .infinity)

        if open {
          DialogScrim {
            Dialog {
              DialogHeader(
                "Discard these changes?",
                subtitle: "The three files you edited will go back to their last saved state."
              )
              DialogBody { Text("This cannot be undone.") }
              DialogFooter {
                Button("Keep editing", variant: .recessed) { open = false }
                Button("Discard", variant: .filled, tint: .danger) { open = false }
              }
            }
          }
        }
      }
    }
  }

  private struct FormDialog: View {
    @State private var name = "DazzUI"
    @State private var region = "us"

    var body: some View {
      Dialog {
        DialogHeader("New workspace", subtitle: "It can be renamed later.")
        DialogBody {
          FormField(label: "Name") {
            DazzUI.TextField(text: $name, placeholder: "Acme")
          }
          FormField(label: "Region", hint: "Where the documents are stored.") {
            Select(
              options: [
                SelectOption(value: "us", label: "United States"),
                SelectOption(value: "eu", label: "Europe"),
                SelectOption(value: "ap", label: "Asia Pacific"),
              ],
              selection: $region
            )
          }
        }
        DialogFooter {
          Button("Cancel", variant: .recessed) {}
          Button("Create", variant: .filled) {}
        }
      }
    }
  }

  private struct Overview: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Page(
        "Dialog",
        description: """
          A panel that blocks: the header says what is being asked, the body \
          says what it costs, and the footer is the answer. The scrim is the \
          content colour at its own alpha rather than black at an opacity, so \
          it dims the same amount in every theme.
          """
      ) {
        Section(
          "The parts",
          description: """
            `DialogHeader`, `DialogBody` and `DialogFooter` are separate views \
            rather than slots, so a sheet that only reads can leave the footer \
            out and a confirmation can leave the body out. The footer is an \
            action row: the confirming action last, and the only filled one.
            """
        ) {
          Row(alignment: .top) {
            Dialog(width: 320) {
              DialogHeader("Discard these changes?")
              DialogBody { Text("This cannot be undone.") }
              DialogFooter {
                Button("Keep editing", variant: .recessed) {}
                Button("Discard", variant: .filled) {}
              }
            }
            Dialog(width: 320) {
              DialogBody { Text("A sheet with only something to read.") }
            }
          }
        }

        Section(
          "Tone",
          description: """
            `danger` is the one tone: the sheet's own edge takes the danger \
            wash so the panel reads as dangerous before the reader gets to the \
            button. The button still has to be tinted itself — the tone colours \
            the frame, not the actions in it.
            """
        ) {
          Dialog(tone: .danger, content: {
            DialogHeader(
              "Delete the glossary?",
              subtitle: "1,204 entries. This cannot be undone."
            )
            DialogFooter {
              Button("Keep", variant: .recessed) {}
              Button("Delete", variant: .filled, tint: .danger) {}
            }
          })
          .frame(width: 380)
        }

        Section(
          "Tokens",
          description: "The dialog's own; the scrim is a colour and an alpha resolved together."
        ) {
          TokenTable([
            TokenRow("dialog.width", theme.vars.dialogWidth),
            TokenRow("dialog.scrim.alpha", String(format: "%g", Double(theme.vars.dialogScrimAlpha))),
          ])
        }
      }
    }
  }
}
