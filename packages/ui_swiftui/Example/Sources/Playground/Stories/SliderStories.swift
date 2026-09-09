import DazzUI
import SwiftUI

/// `Components/Slider` — a value picked along a range.
enum SliderStories {
  static var page: StoryPage {
    StoryPage("Slider") {
      Story("Overview", layout: .page) { Overview() }
      Story("Bare") { Live(width: 288) }
      Story("With value") { Live(width: 320, printed: true) }
      Story("Range") { Band(width: 320) }
      Story("Sizes") {
        Col(width: 320) {
          ForEach(SliderSize.allCases, id: \.self) { size in
            Live(size: size)
          }
        }
      }
      Story("Tints") {
        Col(width: 320) {
          ForEach(Tint.allCases, id: \.self) { tint in
            Live(tint: tint)
          }
        }
      }
      Story("Disabled") {
        Slider(value: .constant(40), valueLabel: "40", enabled: false).frame(width: 320)
      }
      Story("Interactive") { FontSize() }
      Story("In settings") { Settings() }
    }
  }

  /// Every specimen is stateful: a control that did not move under the pointer
  /// would be a picture of one.
  struct Live: View {
    var width: CGFloat?
    var size: SliderSize = .medium
    var tint: Tint = .primary
    var printed = false

    @State private var value: Double = 40

    var body: some View {
      Slider(
        value: $value,
        size: size,
        tint: tint,
        valueLabel: printed ? "\(Int(value))" : nil
      )
      .frame(width: width)
    }
  }

  struct Band: View {
    var width: CGFloat?

    @State private var values: [Double] = [20, 70]

    var body: some View {
      Slider(
        values: $values,
        valueLabel: "\(Int(values[0])) – \(Int(values[1]))"
      )
      .frame(width: width)
    }
  }

  private struct FontSize: View {
    @State private var size: Double = 13

    var body: some View {
      FormField(label: "Editor font size", hint: "\(Int(size))px") {
        Slider(value: $size, minimum: 8, maximum: 24, valueLabel: "\(Int(size))px")
      }
      .frame(width: 352)
    }
  }

  private struct Settings: View {
    @State private var font: Double = 13
    @State private var width: Double = 240

    var body: some View {
      PreferenceSection(label: "Appearance") {
        PreferenceRow(
          title: "Editor font size",
          subtitle: "Applies to every open project.",
          trailing: {
            Slider(value: $font, minimum: 8, maximum: 24, size: .small).frame(width: 160)
          }
        )
        PreferenceRow(
          title: "Sidebar width",
          trailing: {
            Slider(value: $width, minimum: 180, maximum: 360, size: .small).frame(width: 160)
          }
        )
      }
      .frame(width: 480)
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Slider",
        description:
          "A value picked along a range. It owns the parts that make one usable rather than "
          + "merely draggable: the press on the track that jumps the nearest thumb, the drag, "
          + "and — on a range — which thumb the pointer actually grabbed when the two are on "
          + "top of each other. Reach for it where the reader is feeling for a value rather "
          + "than stating one; a number they know is a NumberField."
      ) {
        Section(
          "Sizes",
          description:
            "The row’s height comes off the control profile, so a slider in a form sits on the "
            + "same rhythm as the fields around it. The groove and the thumb are the view’s own "
            + "tokens, and both are even on every size for the reason the radio’s ring and dot "
            + "are."
        ) {
          Stack {
            ForEach(SliderSize.allCases, id: \.self) { size in
              Specimen(label(size)) { Live(width: 240, size: size) }
            }
          }
        }
        Section(
          "Range",
          description:
            "A range is the same view: pass more than one value and it draws a thumb per value. "
            + "There is no second view, because everything but the number of thumbs is "
            + "identical."
        ) {
          Col(width: 320) {
            Live(printed: true)
            Band()
          }
        }
        Section(
          "Disabled",
          description:
            "It flattens to the groove like every other control — no lift, no accent, one dead "
            + "look — rather than becoming a translucent ghost of the live one."
        ) {
          Slider(value: .constant(40), valueLabel: "40", enabled: false).frame(width: 288)
        }
      }
    }
  }
}
