import DazzUI
import SwiftUI

/// The form vocabulary, shown as a form rather than as a shelf of specimens —
/// the field chrome, the label and hint above and below it, and the error
/// state are only really legible next to each other.
struct FormsPage: View {
  @Environment(\.theme) private var theme

  @State private var name = "DazzUI"
  @State private var email = ""
  @State private var notes = ""
  @State private var region = "us"
  @State private var search = ""
  @State private var newsletter = true
  @State private var telemetry = false
  @State private var plan = "team"
  @State private var notify = true

  var body: some View {
    let vars = theme.vars

    PageTitle(
      "Forms",
      "Every field is pinned to the same control height, so a row of them lines up."
    )

    Specimen("Fields") {
      VStack(alignment: .leading, spacing: vars.spacing4) {
        FormField(label: "Workspace name") {
          TextField(text: $name, placeholder: "Acme")
        }

        FormField(
          label: "Email",
          hint: "We could not reach this address.",
          state: .error
        ) {
          TextField(text: $email, placeholder: "you@example.com", state: .error)
        }

        FormField(label: "Region") {
          Select(
            options: [
              SelectOption(value: "us", label: "United States"),
              SelectOption(value: "eu", label: "Europe"),
              SelectOption(value: "ap", label: "Asia Pacific"),
            ],
            selection: $region
          )
        }

        FormField(label: "Notes", hint: "Markdown is supported.") {
          TextField(
            text: $notes,
            placeholder: "Anything the team should know…",
            multiline: true
          )
        }
      }
      .frame(maxWidth: vars.dialogWidth)
    }

    Specimen("Search") {
      SearchField(
        value: search,
        onValueChange: { search = $0 },
        placeholder: "Search components"
      )
      .frame(maxWidth: vars.dialogWidth)
    }

    Specimen("Checkboxes", note: "a note sits under the label, not beside it") {
      VStack(alignment: .leading, spacing: vars.spacing2) {
        Checkbox(
          "Send me the monthly newsletter",
          checked: newsletter,
          onCheckedChange: { newsletter = $0 }
        )
        Checkbox(
          "Share anonymous usage data",
          checked: telemetry,
          onCheckedChange: { telemetry = $0 },
          note: "Crash reports only. Never the contents of a document."
        )
        Checkbox("Unavailable on this plan", checked: false, enabled: false)
      }
    }

    Specimen("Radio group") {
      RadioGroup(
        options: [
          RadioGroupOption(value: "solo", label: "Solo"),
          RadioGroupOption(value: "team", label: "Team"),
          RadioGroupOption(value: "enterprise", label: "Enterprise", enabled: false),
        ],
        value: plan,
        onValueChange: { plan = $0 }
      )
    }

    Specimen("Switches") {
      Row(spacing: vars.spacing4) {
        Switch(checked: notify, onCheckedChange: { notify = $0 }, size: .small)
        Switch(checked: notify, onCheckedChange: { notify = $0 })
        Switch(checked: notify, onCheckedChange: { notify = $0 }, size: .large)
        Switch(checked: false, enabled: false)
      }
    }

    Specimen("Option cards", note: "selection shifts weight as well as colour") {
      Row(spacing: vars.spacing3) {
        OptionCard(
          "Automatic",
          description: "Follow the system appearance.",
          selected: true
        ) {}
        OptionCard(
          "Manual",
          description: "Pick a theme and keep it.",
          selected: false
        ) {}
      }
    }

    Spacer(minLength: vars.spacing8)
  }
}
