import DazzUI
import SwiftUI

/// `Components/Spinner` — work in flight, with no end in sight.
enum SpinnerStories {
  static var page: StoryPage {
    StoryPage("Spinner") {
      Story("Overview", layout: .page) { Overview() }
      Story("Default") { Spinner() }
      Story("Sizes") {
        Row {
          ForEach(SpinnerSize.allCases, id: \.self) { size in
            Spinner(size: size)
          }
        }
      }
      Story("Tints") {
        Row {
          ForEach(Tint.allCases, id: \.self) { tint in
            Spinner(tint: tint)
          }
        }
      }
      Story("On an accent") {
        Button(variant: .filled, enabled: false, action: {}) {
          Spinner(size: .small, onAccent: true)
          Text("Translating…")
        }
      }
      Story("In a button") {
        Button(variant: .filled, enabled: false, action: {}) {
          Spinner(size: .small)
          Text("Translating…")
        }
      }
      Story("In a callout") { InACallout() }
      Story("In a toast") {
        ToastViewport {
          Toast(
            tint: .info,
            leading: { Spinner(size: .small, tint: .info) },
            message: { Text("Rebuilding the token graph…") },
            trailing: { EmptyView() }
          )
        }
        .frame(width: 420, height: 120)
      }
      Story("In a panel") { InAPanel() }
    }
  }

  private struct InAPanel: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Card {
        HStack(spacing: theme.vars.spacing3) {
          Spinner()
          Text("Resolving packages…")
        }
        .frame(width: 260, alignment: .leading)
      }
    }
  }

  private struct InACallout: View {
    var body: some View {
      Callout(
        tint: .info,
        title: "Rebuilding",
        leading: { Spinner(size: .small, tint: .info) },
        description: { Text("Four platform packages are being rewritten.") },
        trailing: { EmptyView() }
      )
      .frame(width: 420)
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Spinner",
        description: """
          The indeterminate mark. Where a `Progress` bar without a value sweeps \
          along a track, a spinner has no track at all — it is what a label, a \
          button or a toast carries while it waits.
          """
      ) {
        Section(
          "Sizes and tints",
          description: """
            Three sizes; the track is the tint at a quarter, which is the one \
            value the React stylesheet holds as a literal and this port keeps \
            as a named constant beside the same reason.
            """
        ) {
          Stack {
            ForEach(SpinnerSize.allCases, id: \.self) { size in
              Specimen(size.rawValue) {
                ForEach(Tint.allCases, id: \.self) { tint in
                  Spinner(size: size, tint: tint)
                }
              }
            }
          }
        }

        Section(
          "On an accent",
          description: """
            `onAccent` is the spinner drawn to sit on a filled surface rather \
            than on paper. A spinner inside a *disabled* filled button is on \
            the groove instead, so it takes the ordinary colours — which is why \
            this is a flag rather than something the component guesses.
            """
        ) {
          Row {
            Button(variant: .filled, action: {}) {
              Spinner(size: .small, onAccent: true)
              Text("Translating…")
            }
            Button(variant: .filled, enabled: false, action: {}) {
              Spinner(size: .small)
              Text("Translating…")
            }
          }
        }

        Section(
          "Where it lands",
          description: """
            In a button's label, in a callout's leading slot, in a toast, or on \
            its own in a panel that is still filling.
            """
        ) {
          Col {
            InACallout()
            InAPanel()
          }
        }
      }
    }
  }
}
