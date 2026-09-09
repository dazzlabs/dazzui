import DazzUI
import SwiftUI

/// `Components/NumberField` — a number, with a stepper.
enum NumberFieldStories {
  static var page: StoryPage {
    StoryPage("NumberField") {
      Story("Overview", layout: .page) { Overview() }
      Story("Bare") { Live(width: 160) }
      Story("With unit") { Live(width: 160, start: 13, minimum: 8, maximum: 24, unit: "px") }
      Story("Sizes") {
        Col(width: 160) {
          ForEach(TextFieldSize.allCases, id: \.self) { size in
            Live(size: size)
          }
        }
      }
      Story("States") {
        Col(width: 160) {
          Live()
          Live(state: .error)
          NumberField(value: .constant(12), enabled: false)
        }
      }
      Story("In a form") { Port() }
      Story("In settings") { Settings() }
    }
  }

  struct Live: View {
    var width: CGFloat?
    var start: Double = 12
    var minimum: Double?
    var maximum: Double?
    var unit: String?
    var size: TextFieldSize = .medium
    var state: TextFieldState = .default

    @State private var value: Double?

    var body: some View {
      NumberField(
        value: Binding(get: { value ?? start }, set: { value = $0 }),
        minimum: minimum,
        maximum: maximum,
        unit: unit,
        size: size,
        state: state
      )
      .frame(width: width)
    }
  }

  private struct Port: View {
    @State private var port: Double? = 5173

    var body: some View {
      FormField(
        label: "Dev server port",
        hint: port == nil ? "Required" : "Restart to apply",
        state: port == nil ? .error : .default
      ) {
        NumberField(
          value: $port,
          minimum: 1024,
          maximum: 65535,
          state: port == nil ? .error : .default
        )
      }
      .frame(width: 288)
    }
  }

  private struct Settings: View {
    @State private var size: Double? = 13
    @State private var tab: Double? = 2

    var body: some View {
      PreferenceSection(label: "Editor") {
        PreferenceRow(
          title: "Font size",
          subtitle: "Applies to every project.",
          trailing: {
            NumberField(value: $size, minimum: 8, maximum: 24, unit: "px", size: .small)
              .frame(width: 112)
          }
        )
        PreferenceRow(
          title: "Tab width",
          trailing: {
            NumberField(value: $tab, minimum: 1, maximum: 8, size: .small).frame(width: 112)
          }
        )
      }
      .frame(width: 480)
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "NumberField",
        description:
          "A number, with the two things a plain text control never gets right: a stepper you "
          + "can hold down, and a value that stays a number. The box is the TextField’s — one "
          + "style draws it for both — so a form mixing the two does not step. What is this "
          + "view’s own is the stepper at the trailing edge, and the clamp that keeps the value "
          + "inside its bounds."
      ) {
        Section(
          "Sizes",
          description:
            "The stepper takes its room out of the trailing padding rather than out of the "
            + "value, and it is stacked rather than a minus and a plus on either side: a value "
            + "with a control on both sides of it reads as three things."
        ) {
          Stack {
            ForEach(TextFieldSize.allCases, id: \.self) { size in
              Specimen(label(size)) { Live(width: 128, size: size) }
            }
          }
        }
        Section(
          "Bounds and units",
          description:
            "A unit inside the box is a hint rather than part of the value — it is the faint "
            + "ink, and it never enters what the field holds."
        ) {
          Col(width: 192) {
            Live(start: 13, minimum: 8, maximum: 24, unit: "px")
            Live(start: 50, unit: "%")
          }
        }
        Section(
          "States",
          description:
            "Erroring re-tints the control to danger rather than bolting on a second palette, "
            + "exactly as a TextField does — the border, the wash and the value’s ink all move "
            + "together."
        ) {
          Stack {
            Specimen("default") { Live(width: 128) }
            Specimen("error") { Live(width: 128, state: .error) }
            Specimen("disabled") {
              NumberField(value: .constant(12), enabled: false).frame(width: 128)
            }
          }
        }
      }
    }
  }
}
