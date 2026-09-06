import DazzUI
import SwiftUI

/// `Components/ShortcutRecorder` — the field you press a chord into.
enum ShortcutRecorderStories {
  static var page: StoryPage {
    StoryPage("ShortcutRecorder") {
      Story("Overview", layout: .page) { Overview() }
      Story("Bound") { Recorder(value: "⇧⌘B") }
      Story("Unbound") { Recorder(value: "") }
      Story("Error state") { Recorder(value: "⌘Q", state: .error) }
      Story("Disabled") { Recorder(value: "⌘K", disabled: true) }
      Story("States") {
        Col {
          Recorder(value: "⇧⌘B")
          Recorder(value: "")
          Recorder(value: "⌘Q", state: .error)
          Recorder(value: "⌘K", disabled: true)
        }
      }
      Story("Modified") {
        Col {
          Recorder(value: "⌘K")
          Recorder(value: "⇧⌘K")
          Recorder(value: "⌥⌘K")
          Recorder(value: "⌃⌥⇧⌘K")
        }
      }
      Story("Word keys") {
        Col {
          Recorder(value: "⌘↩")
          Recorder(value: "⌘⌫")
          Recorder(value: "⇧Space")
        }
      }
      Story("Against a key cap") {
        Row {
          KeyCap("⇧⌘B")
          Recorder(value: "⇧⌘B")
        }
      }
      Story("Tints") {
        Col {
          ForEach(Tint.allCases, id: \.self) { tint in
            Recorder(value: "⇧⌘B", tint: tint)
          }
        }
      }
      Story("Conflicts") { Conflicts() }
      Story("In settings") { InSettings() }
    }
  }

  private struct Recorder: View {
    let state: ShortcutRecorderState
    let tint: Tint
    let disabled: Bool

    @State private var value: String

    init(
      value: String,
      state: ShortcutRecorderState = .default,
      tint: Tint = .primary,
      disabled: Bool = false
    ) {
      self.state = state
      self.tint = tint
      self.disabled = disabled
      _value = State(initialValue: value)
    }

    var body: some View {
      ShortcutRecorder(
        value: value,
        onValueChange: { value = $0 },
        onClear: { value = "" },
        state: state,
        disabled: disabled,
        tint: tint
      )
    }
  }

  /// The error state is what a taken chord looks like: the field turns and the
  /// line under it says which command already has it.
  private struct Conflicts: View {
    @Environment(\.theme) private var theme

    var body: some View {
      FormField(
        label: "Rebuild tokens",
        hint: "⌘Q is already bound to Quit.",
        state: .error
      ) {
        Recorder(value: "⌘Q", state: .error)
      }
      .frame(width: 320)
    }
  }

  private struct InSettings: View {
    var body: some View {
      PreferenceSection(
        label: "Shortcuts",
        footer: "Press a chord in a field to rebind it."
      ) {
        PreferenceRow(
          title: "Toggle the palette",
          trailing: { Recorder(value: "⌘K") }
        )
        PreferenceRow(
          title: "Rebuild tokens",
          trailing: { Recorder(value: "⇧⌘B") }
        )
        PreferenceRow(
          title: "Open the playground",
          trailing: { Recorder(value: "") }
        )
      }
      .frame(width: 500)
    }
  }

  private struct Overview: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Page(
        "ShortcutRecorder",
        description: """
          A field whose value is a key chord: focus it and the next chord you \
          press becomes the binding. It prints what it holds the way the rest \
          of the system does — the same symbols a `KeyCap` and a menu's \
          shortcut column use.
          """
      ) {
        Section(
          "States",
          description: """
            Bound, unbound, recording, disabled, and the error state a taken \
            chord puts it in. Recording is not a value the host sets: it is \
            what focus means here, so the field says so itself.
            """
        ) {
          Stack {
            Specimen("bound") { Recorder(value: "⇧⌘B") }
            Specimen("unbound") { Recorder(value: "") }
            Specimen("error") { Recorder(value: "⌘Q", state: .error) }
            Specimen("disabled") { Recorder(value: "⌘K", disabled: true) }
          }
        }

        Section(
          "What it prints",
          description: """
            Modifiers in the platform's order, then the key. A word key stays a \
            word; a symbol key stays a symbol.
            """
        ) {
          Stack {
            Specimen("modifiers") {
              Recorder(value: "⌘K")
              Recorder(value: "⇧⌘K")
              Recorder(value: "⌃⌥⇧⌘K")
            }
            Specimen("words") {
              Recorder(value: "⌘↩")
              Recorder(value: "⇧Space")
            }
          }
        }

        Section(
          "In settings",
          description: """
            The trailing control of a preference row, beside the command it \
            binds. A conflict is a field in the error state with the reason \
            under it — the same shape any invalid field takes.
            """
        ) {
          Col(width: 500) {
            InSettings()
            Conflicts()
          }
        }

        Section("Tokens", description: "The field's own width.") {
          TokenTable([
            TokenRow("shortcut-recorder.width", theme.vars.shortcutRecorderWidth)
          ])
        }
      }
    }
  }
}
