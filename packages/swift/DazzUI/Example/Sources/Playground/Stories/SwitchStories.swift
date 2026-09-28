import DazzUI
import SwiftUI

/// `Components/Switch` — a decision that takes effect at once.
enum SwitchStories {
  static var page: StoryPage {
    StoryPage("Switch") {
      Story("Overview", layout: .page) { Overview() }
      Story("On") { Switch(checked: true) }
      Story("Off") { Switch(checked: false) }
      Story("Interactive") { Live() }
      Story("Disabled") {
        Row {
          Switch(checked: false, enabled: false)
          Switch(checked: true, enabled: false)
        }
      }
      Story("Sizes") {
        Row {
          ForEach(SwitchSize.allCases, id: \.self) { size in
            Switch(checked: true, size: size)
          }
        }
      }
      Story("Tints") {
        Row {
          ForEach(Tint.allCases, id: \.self) { tint in
            Switch(checked: true, tint: tint)
          }
        }
      }
      Story("Against a checkbox") { Against() }
      Story("In settings") { InSettings() }
    }
  }

  private struct Live: View {
    @State private var on = true

    var body: some View {
      Switch(checked: on, onCheckedChange: { on = $0 })
    }
  }

  private struct Against: View {
    @Environment(\.theme) private var theme

    var body: some View {
      let vars = theme.vars

      VStack(alignment: .leading, spacing: vars.spacing3) {
        HStack(spacing: vars.spacing3) {
          Switch(checked: true)
          Text("Takes effect the moment it moves")
            .font(vars.labelQuiet.font)
            .foregroundStyle(vars.colorContent)
        }
        Checkbox("Takes effect when the form is submitted", checked: true)
      }
    }
  }

  private struct InSettings: View {
    @State private var launch = true
    @State private var updates = true
    @State private var sounds = false

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
        PreferenceRow(
          title: "Play sounds",
          trailing: { Switch(checked: sounds, onCheckedChange: { sounds = $0 }) }
        )
      }
      .frame(width: 460)
    }
  }

  private struct Overview: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Page(
        "Switch",
        description: """
          A track and a thumb. It is the control for a setting that applies the \
          moment it moves — which is what separates it from a checkbox, and why \
          it belongs at the trailing edge of a preference row rather than in a \
          form you submit.
          """
      ) {
        Section(
          "Sizes and tints",
          description: """
            The track, the height and the thumb are three tokens moved \
            together, so the thumb keeps its inset at every size. The tint is \
            the on-state's fill; off is a surface role and never follows it.
            """
        ) {
          Stack {
            ForEach(SwitchSize.allCases, id: \.self) { size in
              Specimen(size.rawValue) {
                ForEach(Tint.allCases, id: \.self) { tint in
                  Switch(checked: true, size: size, tint: tint)
                }
              }
            }
          }
        }

        Section(
          "Against a checkbox",
          description: """
            A switch says the change is already made; a checkbox says it will \
            be made when the form is. Choosing by which one looks better is how \
            a settings screen ends up with both.
            """
        ) {
          Against()
        }

        Section(
          "In settings",
          description: "Where nearly every switch in an app actually lives."
        ) {
          InSettings()
        }

        Section("Tokens", description: "The three the switch owns, per size.") {
          TokenTable([
            TokenRow("switch.small.width", theme.vars.switchSmallWidth),
            TokenRow("switch.small.height", theme.vars.switchSmallHeight),
            TokenRow("switch.small.thumb", theme.vars.switchSmallThumb),
            TokenRow("switch.medium.width", theme.vars.switchMediumWidth),
            TokenRow("switch.medium.height", theme.vars.switchMediumHeight),
            TokenRow("switch.medium.thumb", theme.vars.switchMediumThumb),
            TokenRow("switch.large.width", theme.vars.switchLargeWidth),
            TokenRow("switch.large.height", theme.vars.switchLargeHeight),
            TokenRow("switch.large.thumb", theme.vars.switchLargeThumb),
          ])
        }
      }
    }
  }
}
