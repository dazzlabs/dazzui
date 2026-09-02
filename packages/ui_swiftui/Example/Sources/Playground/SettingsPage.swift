import DazzUI
import SwiftUI

/// A preferences screen, which is the composition most of these components
/// were drawn for: rows of one decision each, grouped under a label, with the
/// control pinned to the trailing edge.
///
/// `PreferenceRow` names its `trailing:` slot rather than passing a trailing
/// closure, because the row has two view slots — an icon and a control — and
/// an unlabelled closure could be either.
struct SettingsPage: View {
  @Environment(\.theme) private var theme

  @State private var launch = true
  @State private var updates = true
  @State private var sounds = false
  @State private var appearance = "auto"

  var body: some View {
    let vars = theme.vars

    PageTitle(
      "Settings",
      "One decision per row, and the control sits where the eye lands last."
    )

    // A group is the level above a section: sections stay one size
    // everywhere, and the title outranks them rather than being a section
    // shrunk to fit underneath one.
    PreferenceGroup(
      title: "Application",
      description: "Settings that apply to every window."
    ) {
      PreferenceSection(label: "General") {
        PreferenceRow(
          title: "Launch at login",
          trailing: { Switch(checked: launch, onCheckedChange: { launch = $0 }) }
        )
        PreferenceRow(
          title: "Check for updates",
          subtitle: "Downloads in the background and asks before installing.",
          trailing: { Switch(checked: updates, onCheckedChange: { updates = $0 }) }
        )
        PreferenceRow(
          title: "Play sounds",
          trailing: { Switch(checked: sounds, onCheckedChange: { sounds = $0 }) }
        )
      }

      PreferenceSection(
        label: "Appearance",
        footer: "Automatic follows the system between light and dark."
      ) {
        PreferenceRow(
          title: "Theme",
          trailing: {
            Select(
              options: [
                SelectOption(value: "auto", label: "Automatic"),
                SelectOption(value: "light", label: "Light"),
                SelectOption(value: "dark", label: "Dark"),
              ],
              selection: $appearance,
              size: .small
            )
          }
        )
      }
    }

    // A section on its own, with no group over it.
    PreferenceSection(label: "Shortcuts") {
      PreferenceRow(title: "Toggle the palette", trailing: { KeyCap("⌘K") })
      PreferenceRow(title: "Rebuild tokens", trailing: { KeyCap("⇧⌘B") })
    }

    Spacer(minLength: vars.spacing8)
  }
}
