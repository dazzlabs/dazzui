import DazzUI
import SwiftUI

/// `Components/TextField` — the box a value is typed into.
enum TextFieldStories {
  static var page: StoryPage {
    StoryPage("TextField") {
      Story("Overview", layout: .page) { Overview() }
      Story("Default") { Field(text: "DazzUI") }
      Story("Placeholder") { Field(text: "") }
      Story("Disabled") { Field(text: "DazzUI", enabled: false) }
      Story("Error state") { Field(text: "you@", state: .error) }
      Story("States") {
        Col(width: 320) {
          Field(text: "DazzUI")
          Field(text: "")
          Field(text: "you@", state: .error)
          Field(text: "DazzUI", enabled: false)
        }
      }
      Story("Sizes") {
        Col(width: 320) {
          ForEach(TextFieldSize.allCases, id: \.self) { size in
            Field(text: label(size), size: size)
          }
        }
      }
      Story("Tints") {
        Col(width: 320) {
          ForEach(Tint.allCases, id: \.self) { tint in
            Field(text: label(tint), tint: tint)
          }
        }
      }
      Story("Mono") { Field(text: "785f949", mono: true) }
      Story("Beside a button") {
        Row {
          Field(text: "")
          Button("Add", variant: .filled) {}
        }
      }
      Story("A read-only value") {
        Col(width: 320) {
          FieldValue("DazzUI")
          FieldValue("785f949", mono: true)
        }
      }
      Story("Multiline") { Multiline(text: "") }
      Story("Multiline mono") { Multiline(text: "swift build", mono: true) }
      Story("Multiline in a form") {
        Col(width: 380) {
          FormField(label: "Notes", hint: "Markdown is supported.") {
            Multiline(text: "")
          }
        }
      }
    }
  }

  private struct Field: View {
    let size: TextFieldSize
    let state: TextFieldState
    let tint: Tint
    let mono: Bool
    let enabled: Bool

    @State private var text: String

    init(
      text: String,
      size: TextFieldSize = .medium,
      state: TextFieldState = .default,
      tint: Tint = .primary,
      mono: Bool = false,
      enabled: Bool = true
    ) {
      self.size = size
      self.state = state
      self.tint = tint
      self.mono = mono
      self.enabled = enabled
      _text = State(initialValue: text)
    }

    var body: some View {
      DazzUI.TextField(
        text: $text,
        placeholder: "Acme",
        size: size,
        state: state,
        tint: tint,
        mono: mono,
        enabled: enabled
      )
      .frame(width: 240)
    }
  }

  private struct Multiline: View {
    let mono: Bool

    @State private var text: String

    init(text: String, mono: Bool = false) {
      self.mono = mono
      _text = State(initialValue: text)
    }

    var body: some View {
      DazzUI.TextField(
        text: $text,
        placeholder: "Anything the team should know…",
        mono: mono,
        multiline: true
      )
      .frame(width: 320)
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "TextField",
        description: """
          The box a value is typed into, on the control heights everything else \
          uses. The focus ring is the system accent in every tint — it never \
          follows `data-tint`, because a focus ring that changed colour per \
          field would stop meaning focus.
          """
      ) {
        Section(
          "States",
          description: """
            Default, empty with a placeholder, invalid, and disabled. The error \
            state turns the field's own edge; the message under it is the \
            `FormField`'s hint, not a second line the field draws.
            """
        ) {
          Col(width: 420) {
            Field(text: "DazzUI")
            Field(text: "")
            Field(text: "you@", state: .error)
            Field(text: "DazzUI", enabled: false)
          }
        }

        Section(
          "Sizes",
          description: """
            The three field sizes are the control profile's, so a field, a \
            select and a button on the same row are the same height.
            """
        ) {
          Col(width: 420) {
            ForEach(TextFieldSize.allCases, id: \.self) { size in
              Field(text: label(size), size: size)
            }
          }
        }

        Section(
          "Mono",
          description: """
            `mono` is for a value that is code — a hash, a path, a command. The \
            face is the system monospaced one rather than a family looked up by \
            name: the graph's code stack leads with `ui-monospace`, which on \
            this platform is a request rather than a family.
            """
        ) {
          Col(width: 420) {
            Field(text: "785f949", mono: true)
            Multiline(text: "swift build", mono: true)
          }
        }

        Section(
          "Multiline and read-only",
          description: """
            `multiline` grows the same box into a text area. `FieldValue` is \
            the other end of it: a value that cannot be edited, in the field's \
            own shape, so a read-only row in a form does not become a paragraph.
            """
        ) {
          Col(width: 420) {
            Multiline(text: "")
            FieldValue("DazzUI")
            FieldValue("785f949", mono: true)
          }
        }
      }
    }
  }
}
