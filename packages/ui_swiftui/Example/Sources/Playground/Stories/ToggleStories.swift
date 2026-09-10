import DazzUI
import SwiftUI

/// `Components/Toggle` — a button that stays down.
enum ToggleStories {
  static var page: StoryPage {
    StoryPage("Toggle") {
      Story("Overview", layout: .page) { Overview() }
      Story("Pressed") {
        HStack(spacing: 6) {
          Toggle("Wrap")
          Toggle("Wrap", pressed: true)
        }
      }
      Story("Sizes") {
        HStack(spacing: 6) {
          ForEach(ControlSize.allCases, id: \.self) { size in
            Toggle(size.rawValue, pressed: true, size: size, variant: .recessed)
          }
        }
      }
      Story("Variants") {
        VStack(alignment: .leading, spacing: 8) {
          HStack(spacing: 6) {
            ForEach(ControlVariant.allCases, id: \.self) { variant in
              Toggle(variant.rawValue, variant: variant)
            }
          }
          HStack(spacing: 6) {
            ForEach(ControlVariant.allCases, id: \.self) { variant in
              Toggle(variant.rawValue, pressed: true, variant: variant)
            }
          }
        }
      }
      Story("Tints") {
        HStack(spacing: 6) {
          ForEach(Tint.allCases, id: \.self) { tint in
            Toggle(tint.rawValue, pressed: true, tint: tint)
          }
        }
      }
      Story("Disabled") {
        HStack(spacing: 6) {
          Toggle("Wrap", enabled: false)
          Toggle("Wrap", pressed: true, enabled: false)
        }
      }
      Story("A toolbar") { Toolbar() }
    }
  }

  struct Toolbar: View {
    @State private var on: Set<String> = ["bold"]

    var body: some View {
      HStack(spacing: 4) {
        ForEach(["B", "I", "U"], id: \.self) { glyph in
          Toggle(
            glyph,
            pressed: on.contains(key(glyph)),
            action: {
              let value = key(glyph)
              if on.contains(value) { on.remove(value) } else { on.insert(value) }
            }
          )
        }
      }
    }

    private func key(_ glyph: String) -> String {
      switch glyph {
      case "B": return "bold"
      case "I": return "italic"
      default: return "underline"
      }
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Toggle",
        description:
          "A button that stays down. Three views in this package are one press away from each "
          + "other: a Switch commits immediately and reads as a setting; a SegmentedControl is "
          + "a choice between its segments and always has one; a Toggle is neither — a single "
          + "property that is on or off, in a row where none of them may be on."
      ) {
        Section(
          "Beside its neighbours",
          description:
            "Reach for a Switch where the thing is a setting in a list and the change takes "
            + "effect on the spot. Reach for a SegmentedControl where the reader is choosing "
            + "one of several alternatives and cannot choose none. Reach for this where the "
            + "answer is yes or no and the control lives in a toolbar rather than a form."
        ) {
          Stack {
            Specimen("Switch") { Switch(checked: true) }
            Specimen("SegmentedControl") {
              SegmentedControl(
                items: [
                  SegmentedItem(value: "left", label: "Left"),
                  SegmentedItem(value: "center", label: "Center"),
                ],
                value: "left"
              )
            }
            Specimen("Toggle") { Toggle("Bold", pressed: true) }
          }
        }
        Section(
          "Pressed",
          description:
            "Drawn the way the system draws every selection: the tinted wash, and the label a "
            + "weight heavier. Colour alone would be one grade in a toolbar of glyphs, which "
            + "is exactly where it is hardest to see. The recipe stays tinted when it is "
            + "disabled and only the weight drops, so a dead toggle still flattens to the "
            + "groove."
        ) {
          Stack {
            Specimen("up") { Toggle("Wrap") }
            Specimen("down") { Toggle("Wrap", pressed: true) }
            Specimen("disabled, down") { Toggle("Wrap", pressed: true, enabled: false) }
          }
        }
        Section(
          "In a toolbar",
          description:
            "Where most of them live. The label takes the control type's size, so a row of "
            + "these lines up with the buttons beside it."
        ) {
          ToggleStories.Toolbar()
        }
      }
    }
  }
}
