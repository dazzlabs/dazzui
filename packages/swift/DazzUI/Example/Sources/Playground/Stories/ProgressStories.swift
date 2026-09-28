import DazzUI
import SwiftUI

/// `Components/Progress` — the bar, determinate and not.
enum ProgressStories {
  static var page: StoryPage {
    StoryPage("Progress") {
      Story("Overview", layout: .page) { Overview() }
      Story("Default") { Progress(value: 0.35).frame(width: 320) }
      Story("Values") {
        Col(width: 320) {
          ForEach([0.0, 0.25, 0.5, 0.75, 1.0], id: \.self) { value in
            Progress(value: value)
          }
        }
      }
      Story("Indeterminate") { Progress().frame(width: 320) }
      Story("Sizes") {
        Col(width: 320) {
          ForEach(ProgressSize.allCases, id: \.self) { size in
            Progress(value: 0.6, size: size)
          }
        }
      }
      Story("Tints") {
        Col(width: 320) {
          ForEach(Tint.allCases, id: \.self) { tint in
            Progress(value: 0.6, tint: tint)
          }
        }
      }
      Story("Gradient") {
        Col(width: 320) {
          Progress(value: 0.6)
          Progress(value: 0.6, gradient: true)
        }
      }
      Story("Live") { Live() }
      Story("With a count") { Counted() }
      Story("Beside stages") {
        Card {
          Counted()
          StepList {
            Step("Resolve the token graph", status: .done)
            Step("Write the platform sources", status: .active)
            Step("Publish the package", status: .idle)
          }
        }
        .frame(width: 360)
      }
    }
  }

  private struct Live: View {
    @State private var value = 0.15

    var body: some View {
      Col(width: 320) {
        Progress(value: value, size: .medium, gradient: true)
        ActionBar {
          Button("Advance", variant: .recessed) {
            value = value >= 1 ? 0 : min(1, value + 0.2)
          }
        }
      }
    }
  }

  private struct Counted: View {
    @Environment(\.theme) private var theme

    var body: some View {
      let vars = theme.vars

      VStack(alignment: .leading, spacing: vars.spacing2) {
        HStack {
          Text("Rebuilding tokens")
            .font(vars.labelMedium.font)
            .foregroundStyle(vars.colorContent)
          Spacer()
          Text("3 of 4")
            .font(vars.captionSmall.font)
            .foregroundStyle(vars.colorContentMuted)
        }
        Progress(value: 0.75)
      }
      .frame(width: 320)
    }
  }

  private struct Overview: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Page(
        "Progress",
        description: """
          A bar. With a value it fills; without one it sweeps, which is the \
          honest drawing for work whose end is not known. The track is a token \
          rather than the tint washed down, so a bar reads the same on every \
          surface.
          """
      ) {
        Section(
          "Values",
          description: "Zero draws the track alone; one fills it."
        ) {
          Col(width: 420) {
            ForEach([0.0, 0.25, 0.5, 0.75, 1.0], id: \.self) { value in
              Progress(value: value)
            }
          }
        }

        Section(
          "Sizes and tints",
          description: """
            Three heights on the same corner. The tint is the fill's ramp; the \
            gradient is the two-stop version of it, for the one bar a view \
            points at.
            """
        ) {
          Stack {
            ForEach(ProgressSize.allCases, id: \.self) { size in
              Specimen(size.rawValue) {
                Progress(value: 0.6, size: size).frame(width: 160)
                Progress(value: 0.6, size: size, tint: .success).frame(width: 160)
                Progress(value: 0.6, size: size, gradient: true).frame(width: 160)
              }
            }
          }
        }

        Section(
          "Indeterminate",
          description: """
            No value at all is the sweep. It is the same bar: nothing about the \
            box changes, only what is drawn inside it.
            """
        ) {
          Progress(size: .medium).frame(width: 420)
        }

        Section(
          "Tokens",
          description: "The two stops the gradient is drawn between."
        ) {
          TokenTable([
            TokenRow("progress.gradient.from", "the tint's own step"),
            TokenRow("progress.gradient.to", "the step it lifts to"),
            TokenRow("control.small.size", theme.vars.controlSmallSize),
          ])
        }
      }
    }
  }
}
