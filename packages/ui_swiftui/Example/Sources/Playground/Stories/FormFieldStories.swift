import DazzUI
import SwiftUI

/// `Components/FormField` — the label over a control and the hint under it.
enum FormFieldStories {
  static var page: StoryPage {
    StoryPage("FormField") {
      Story("Overview", layout: .page) { Overview() }
      Story("With a hint") { Field(hint: "Shown to everyone in the workspace.") }
      Story("Without a hint") { Field(hint: nil) }
      Story("Error state") { Invalid() }
      Story("Tints") {
        Col(width: 320) {
          ForEach(Tint.allCases, id: \.self) { tint in
            FormField(label: label(tint), tint: tint) {
              FieldValue("Focus me to see the ring")
            }
          }
        }
      }
      Story("Over a select") { OverASelect() }
      Story("Over a multiline field") { Multiline() }
      Story("With a field value") {
        FormField(label: "Commit", hint: "Read from the last build.") {
          FieldValue("785f949", mono: true)
        }
        .frame(width: 320)
      }
      Story("Over any control") {
        Col(width: 320) {
          FormField(label: "Notifications") {
            Switch(checked: true)
          }
          FormField(label: "Shortcut") {
            KeyCap("⇧⌘B")
          }
        }
      }
      Story("A full form") { FullForm() }
    }
  }

  private struct Field: View {
    let hint: String?
    @State private var name = "DazzUI"

    var body: some View {
      FormField(label: "Workspace name", hint: hint) {
        DazzUI.TextField(text: $name, placeholder: "Acme")
      }
      .frame(width: 320)
    }
  }

  private struct Invalid: View {
    @State private var email = "you@"

    var body: some View {
      FormField(
        label: "Email",
        hint: "We could not reach this address.",
        state: .error
      ) {
        DazzUI.TextField(
          text: $email,
          placeholder: "you@example.com",
          state: .error
        )
      }
      .frame(width: 320)
    }
  }

  private struct OverASelect: View {
    @State private var region = "eu"

    var body: some View {
      FormField(label: "Region", hint: "Where the documents are stored.") {
        Select(
          options: [
            SelectOption(value: "us", label: "United States"),
            SelectOption(value: "eu", label: "Europe"),
            SelectOption(value: "ap", label: "Asia Pacific"),
          ],
          selection: $region,
          fullWidth: true
        )
      }
      .frame(width: 320)
    }
  }

  private struct Multiline: View {
    @State private var notes = ""

    var body: some View {
      FormField(label: "Notes", hint: "Markdown is supported.") {
        DazzUI.TextField(
          text: $notes,
          placeholder: "Anything the team should know…",
          multiline: true
        )
      }
      .frame(width: 320)
    }
  }

  private struct FullForm: View {
    @State private var name = "DazzUI"
    @State private var email = ""
    @State private var region = "us"
    @State private var notes = ""

    @Environment(\.theme) private var theme

    var body: some View {
      VStack(alignment: .leading, spacing: theme.vars.spacing4) {
        FormField(label: "Workspace name") {
          DazzUI.TextField(text: $name, placeholder: "Acme")
        }
        FormField(
          label: "Email",
          hint: "We could not reach this address.",
          state: .error
        ) {
          DazzUI.TextField(
            text: $email,
            placeholder: "you@example.com",
            state: .error
          )
        }
        FormField(label: "Region") {
          Select(
            options: [
              SelectOption(value: "us", label: "United States"),
              SelectOption(value: "eu", label: "Europe"),
              SelectOption(value: "ap", label: "Asia Pacific"),
            ],
            selection: $region,
            fullWidth: true
          )
        }
        FormField(label: "Notes", hint: "Markdown is supported.") {
          DazzUI.TextField(
            text: $notes,
            placeholder: "Anything the team should know…",
            multiline: true
          )
        }
      }
      .frame(width: 380)
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "FormField",
        description: """
          The chrome around a control: a label above it, a hint below, and the \
          error state that turns both. It takes any control as its child rather \
          than knowing about text fields, so a switch or a key cap gets the \
          same label treatment as an input.
          """
      ) {
        Section(
          "Label and hint",
          description: """
            The hint is the field's ordinary explanation. In the error state \
            it becomes the message, in the tint's ink — one line rather than a \
            hint and an error stacked, because a field that explains itself \
            twice is a field the reader stops reading.
            """
        ) {
          Row(alignment: .top) {
            Field(hint: "Shown to everyone in the workspace.")
            Field(hint: nil)
            Invalid()
          }
        }

        Section(
          "Over any control",
          description: """
            A select, a multiline field, a read-only `FieldValue`, or a control \
            that is not a field at all. The label is a label for whatever is \
            underneath it.
            """
        ) {
          Row(alignment: .top) {
            OverASelect()
            Multiline()
            FormField(label: "Commit", hint: "Read from the last build.") {
              FieldValue("785f949", mono: true)
            }
            .frame(width: 300)
          }
        }
      }
    }
  }
}
