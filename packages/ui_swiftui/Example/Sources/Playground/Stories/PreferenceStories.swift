import DazzUI
import SwiftUI

/// `Components/Preference` — the group, the section and the row a settings
/// screen is built out of.
enum PreferenceStories {
  static var page: StoryPage {
    StoryPage("Preference") {
      Story("Overview", layout: .page) { Overview() }
      Story("A section") { General() }
      Story("Row shapes") { Shapes() }
      Story("Controls") { Controls() }
      Story("Section headings") {
        Col(width: 460) {
          PreferenceSection(
            label: "Appearance",
            footer: "Automatic follows the system between light and dark."
          ) {
            PreferenceRow(title: "Theme", trailing: { KeyCap("⌘,") })
          }
        }
      }
      Story("Unlabelled") {
        PreferenceSection {
          PreferenceRow(title: "Launch at login", trailing: { Switch(checked: true) })
          PreferenceRow(title: "Play sounds", trailing: { Switch(checked: false) })
        }
        .frame(width: 460)
      }
      Story("A group") { Grouped() }
      Story("A settings page") { SettingsPage() }
    }
  }

  private struct General: View {
    @State private var launch = true
    @State private var updates = true

    var body: some View {
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
      }
      .frame(width: 460)
    }
  }

  private struct Shapes: View {
    @Environment(\.theme) private var theme

    var body: some View {
      PreferenceSection(label: "Rows") {
        PreferenceRow(title: "Title only")
        PreferenceRow(title: "With a subtitle", subtitle: "The line that explains it.")
        PreferenceRow(
          title: "With an icon",
          icon: { Icon(.folder16Regular, size: theme.vars.labelMedium.size) }
        )
        PreferenceRow(
          title: "With a control",
          trailing: { Switch(checked: true) }
        )
        PreferenceRow(
          title: "That opens something",
          subtitle: "A row with an action is a row you can click.",
          onOpen: {}
        )
      }
      .frame(width: 460)
    }
  }

  private struct Controls: View {
    @State private var appearance = "auto"
    @State private var sounds = false

    var body: some View {
      PreferenceSection(label: "Controls") {
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
        PreferenceRow(
          title: "Play sounds",
          trailing: { Switch(checked: sounds, onCheckedChange: { sounds = $0 }) }
        )
        PreferenceRow(title: "Rebuild tokens", trailing: { KeyCap("⇧⌘B") })
        PreferenceRow(
          title: "Cache",
          trailing: { Button("Clear", variant: .recessed) {} }
        )
      }
      .frame(width: 460)
    }
  }

  private struct Grouped: View {
    var body: some View {
      PreferenceGroup(
        title: "Application",
        description: "Settings that apply to every window."
      ) {
        General()
        PreferenceSection(label: "Shortcuts") {
          PreferenceRow(title: "Toggle the palette", trailing: { KeyCap("⌘K") })
          PreferenceRow(title: "Rebuild tokens", trailing: { KeyCap("⇧⌘B") })
        }
      }
      .frame(width: 460)
    }
  }

  private struct SettingsPage: View {
    @Environment(\.theme) private var theme

    var body: some View {
      VStack(alignment: .leading, spacing: theme.vars.spacing6) {
        Grouped()
        PreferenceSection(label: "Danger zone") {
          PreferenceRow(
            title: "Reset every preference",
            subtitle: "Windows, shortcuts and appearance go back to their defaults.",
            trailing: { Button("Reset", variant: .tinted, tint: .danger) {} }
          )
        }
      }
      .frame(width: 460)
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Preference",
        description: """
          A settings screen is one decision per row, grouped under a label, \
          with the control pinned to the trailing edge — where the eye lands \
          last. Three views: a group is the level above a section, a section is \
          a run of rows under one heading, and a row is the decision.
          """
      ) {
        Section(
          "Rows",
          description: """
            A row is a title, optionally a subtitle under it, optionally a \
            leading glyph, and optionally a trailing control. `trailing:` is \
            named rather than passed as a trailing closure, because the row has \
            two view slots and an unlabelled one could be either.
            """
        ) {
          Shapes()
        }

        Section(
          "What sits at the trailing edge",
          description: """
            A switch for a decision that takes effect at once, a select for one \
            of several, a key cap for something bound elsewhere, a button for \
            an action. Only one of them per row.
            """
        ) {
          Controls()
        }

        Section(
          "Sections and groups",
          description: """
            A section carries a label above its rows and an optional footer \
            under them. A group is the level above: sections stay one size \
            everywhere, and the group's title outranks them rather than being \
            a section shrunk to fit underneath one.
            """
        ) {
          Grouped()
        }
      }
    }
  }
}
