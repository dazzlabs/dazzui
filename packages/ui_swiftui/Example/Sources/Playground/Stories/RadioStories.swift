import DazzUI
import SwiftUI

/// `Components/Radio` — one of several.
enum RadioStories {
  static var page: StoryPage {
    StoryPage("Radio") {
      Story("Overview", layout: .page) { Overview() }
      Story("Checked") { Radio("Team", checked: true) }
      Story("Unchecked") { Radio("Team", checked: false) }
      Story("A group") { Plans() }
      Story("Disabled") {
        Col {
          Radio("Unavailable", checked: false, enabled: false)
          Radio("Fixed for this plan", checked: true, enabled: false)
        }
      }
      Story("Sizes") {
        Col {
          ForEach(RadioSize.allCases, id: \.self) { size in
            Radio(label(size), checked: true, size: size)
          }
        }
      }
      Story("Tints") {
        Col {
          ForEach(Tint.allCases, id: \.self) { tint in
            Radio(label(tint), checked: true, tint: tint)
          }
        }
      }
      Story("Against a checkbox") {
        Col {
          Radio("A radio picks one of several", checked: true)
          Checkbox("A checkbox is one decision", checked: true)
        }
      }
      Story("In a form") {
        Col(width: 320) {
          FormField(label: "Plan") {
            Plans()
          }
        }
      }
    }
  }

  private struct Plans: View {
    @State private var plan = "team"

    var body: some View {
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
  }

  private struct Overview: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Page(
        "Radio",
        description: """
          One of several. The ring draws its own mark rather than stacking a \
          dot on a circle, so the two never disagree about where the centre is. \
          A single radio is rarely right — `RadioGroup` is the usual form, and \
          it owns the value.
          """
      ) {
        Section(
          "A group",
          description: """
            The group carries the options, the selection and the tint, so a row \
            cannot disagree with its neighbours. A disabled option flattens to \
            the groove, whether or not it is the one selected.
            """
        ) {
          Plans()
        }

        Section(
          "Sizes and tints",
          description: """
            The box and the dot are both the radio's own tokens, sized together \
            so the mark stays centred at every step.
            """
        ) {
          Stack {
            ForEach(RadioSize.allCases, id: \.self) { size in
              Specimen(size.rawValue) {
                ForEach(Tint.allCases, id: \.self) { tint in
                  Radio(label(tint), checked: true, size: size, tint: tint)
                }
              }
            }
          }
        }

        Section(
          "Against a checkbox",
          description: """
            A radio is one of several; a checkbox is one decision on its own. \
            The shapes say which: a circle belongs to a set, a square does not.
            """
        ) {
          Col {
            Radio("A radio picks one of several", checked: true)
            Checkbox("A checkbox is one decision", checked: true)
          }
        }

        Section("Tokens", description: "The ring and the mark inside it.") {
          TokenTable([
            TokenRow("radio.thickness", theme.vars.radioThickness),
            TokenRow("radio.box.small", theme.vars.radioSmallBox),
            TokenRow("radio.dot.small", theme.vars.radioSmallDot),
            TokenRow("radio.box.medium", theme.vars.radioMediumBox),
            TokenRow("radio.dot.medium", theme.vars.radioMediumDot),
            TokenRow("radio.box.large", theme.vars.radioLargeBox),
            TokenRow("radio.dot.large", theme.vars.radioLargeDot),
          ])
        }
      }
    }
  }
}
