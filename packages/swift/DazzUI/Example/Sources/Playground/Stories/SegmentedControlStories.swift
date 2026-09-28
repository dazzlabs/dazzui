import DazzUI
import SwiftUI

/// `Components/SegmentedControl` — one choice, on a track.
enum SegmentedControlStories {
  static var page: StoryPage {
    StoryPage("SegmentedControl") {
      Story("Overview", layout: .page) { Overview() }
      Story("Raised") { Modes(style: .raised) }
      Story("Filled") { Modes(style: .filled) }
      Story("Sizes") {
        Col {
          ForEach(SegmentedControlSize.allCases, id: \.self) { size in
            Modes(style: .raised, size: size)
          }
        }
      }
      Story("Tints") {
        Col {
          ForEach(Tint.allCases, id: \.self) { tint in
            Modes(style: .raised, tint: tint)
          }
        }
      }
      Story("Filled tints") {
        Col {
          ForEach(Tint.allCases, id: \.self) { tint in
            Modes(style: .filled, tint: tint)
          }
        }
      }
      Story("Stretch") {
        Modes(style: .raised, stretch: true).frame(width: 380)
      }
      Story("Disabled") {
        SegmentedControl(
          items: [
            SegmentedItem(value: "design", label: "Design"),
            SegmentedItem(value: "code", label: "Code"),
            SegmentedItem(value: "preview", label: "Preview", disabled: true),
          ],
          value: "design"
        )
      }
      Story("Two segments") {
        SegmentedControl(
          items: [
            SegmentedItem(value: "light", label: "Light"),
            SegmentedItem(value: "dark", label: "Dark"),
          ],
          value: "light"
        )
      }
      Story("In a form") {
        Col(width: 380) {
          FormField(label: "Editor") {
            Modes(style: .raised, stretch: true)
          }
        }
      }
    }
  }

  private struct Modes: View {
    let style: SegmentedActiveStyle
    var size: SegmentedControlSize = .medium
    var tint: Tint = .primary
    var stretch = false

    @State private var mode = "design"

    var body: some View {
      SegmentedControl(
        items: [
          SegmentedItem(value: "design", label: "Design"),
          SegmentedItem(value: "code", label: "Code"),
          SegmentedItem(value: "preview", label: "Preview"),
        ],
        value: mode,
        onValueChange: { mode = $0 },
        size: size,
        tint: tint,
        activeStyle: style,
        stretch: stretch
      )
    }
  }

  private struct Overview: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Page(
        "SegmentedControl",
        description: """
          One choice out of a few, on a sunken track. Where pill tabs filter a \
          list under them, this switches what a view *is* — a mode, an \
          appearance, a unit — which is why it keeps the track: the unchosen \
          options stay visible as part of one control.
          """
      ) {
        Section(
          "Active style",
          description: """
            `raised` is the AppKit shape: the chosen segment is a paper chip \
            lifted off the track, and the label goes to the strong face. \
            `filled` takes the tint instead, for a control that has to say \
            which mode it is in from across the window.
            """
        ) {
          Stack {
            ForEach(SegmentedActiveStyle.allCases, id: \.self) { style in
              Specimen(style.rawValue) { Modes(style: style) }
            }
          }
        }

        Section(
          "Sizes and tints",
          description: """
            Three heights off the control profile. A tint reaches the raised \
            style's label and the filled style's chip — the track itself is a \
            surface role and never moves.
            """
        ) {
          Stack {
            ForEach(SegmentedControlSize.allCases, id: \.self) { size in
              Specimen(size.rawValue) {
                Modes(style: .raised, size: size)
                Modes(style: .filled, size: size, tint: .success)
              }
            }
          }
        }

        Section(
          "Stretch",
          description: """
            By default a segment is as wide as its label. `stretch` divides the \
            width evenly instead, which is what a control inside a form field \
            wants so it lines up with the fields above it.
            """
        ) {
          Col(width: 420) {
            Modes(style: .raised)
            Modes(style: .raised, stretch: true)
          }
        }

        Section("Tokens", description: "The track's own inset.") {
          TokenTable([
            TokenRow("segmented-control.inset", theme.vars.segmentedControlInset)
          ])
        }
      }
    }
  }
}
