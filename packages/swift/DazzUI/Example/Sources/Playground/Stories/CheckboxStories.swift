import DazzUI
import SwiftUI

/// `Components/Checkbox` — a box, its label, and the note under it.
enum CheckboxStories {
  static var page: StoryPage {
    StoryPage("Checkbox") {
      Story("Overview", layout: .page) { Overview() }
      Story("Checked") { Checkbox("Send me the newsletter", checked: true) }
      Story("Unchecked") { Checkbox("Send me the newsletter", checked: false) }
      Story("Interactive") { Live() }
      Story("A group") { Platforms() }
      Story("Disabled") {
        Col {
          Checkbox("Unavailable on this plan", checked: false, enabled: false)
          Checkbox("Always on for this plan", checked: true, enabled: false)
        }
      }
      Story("Sizes") {
        Col {
          ForEach(CheckboxSize.allCases, id: \.self) { size in
            Checkbox(label(size), checked: true, size: size)
          }
        }
      }
      Story("Tints") {
        Col {
          ForEach(Tint.allCases, id: \.self) { tint in
            Checkbox(label(tint), checked: true, tint: tint)
          }
        }
      }
      Story("With a note") {
        Checkbox(
          "Share anonymous usage data",
          checked: true,
          note: "Crash reports only. Never the contents of a document."
        )
        .frame(width: 380, alignment: .leading)
      }
      Story("A checklist") { Checklist() }
      Story("In a form") {
        Col(width: 380) {
          FormField(label: "Workspace") {
            FieldValue("DazzUI")
          }
          Checklist()
        }
      }
    }
  }

  /// The box is a focusable control over a hidden input, so the state comes
  /// off the component rather than off a pseudo-class — here, off `@State`.
  /// The parent is what a lone checkbox can never be: mixed. It reads the
  /// group rather than a value of its own, and pressing it sets or clears the
  /// lot.
  private struct Platforms: View {
    private static let all = ["react", "flutter", "swiftui", "gpui"]

    @State private var chosen: Set<String> = ["react", "flutter"]

    var body: some View {
      CheckboxGroup {
        Checkbox(
          "Every platform",
          checked: chosen.count == Self.all.count,
          indeterminate: !chosen.isEmpty && chosen.count < Self.all.count,
          onCheckedChange: { _ in
            chosen = chosen.count == Self.all.count ? [] : Set(Self.all)
          },
          note: "\(chosen.count) of \(Self.all.count)"
        )
      } content: {
        ForEach(Self.all, id: \.self) { name in
          Checkbox(
            name,
            checked: chosen.contains(name),
            onCheckedChange: { on in
              if on { chosen.insert(name) } else { chosen.remove(name) }
            }
          )
        }
      }
      .frame(width: 288)
    }
  }

  private struct Live: View {
    @State private var checked = true

    var body: some View {
      Checkbox(
        "Send me the monthly newsletter",
        checked: checked,
        onCheckedChange: { checked = $0 }
      )
    }
  }

  private struct Checklist: View {
    @State private var selected: Set<String> = ["react", "flutter"]

    private let platforms = [
      ("react", "React"),
      ("flutter", "Flutter"),
      ("swiftui", "SwiftUI"),
      ("gpui", "GPUI"),
    ]

    @Environment(\.theme) private var theme

    var body: some View {
      VStack(alignment: .leading, spacing: theme.vars.spacing2) {
        ForEach(platforms, id: \.0) { id, name in
          Checkbox(
            name,
            checked: selected.contains(id),
            onCheckedChange: { on in
              if on { selected.insert(id) } else { selected.remove(id) }
            }
          )
        }
      }
    }
  }

  private struct Overview: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Page(
        "Checkbox",
        description: """
          A box, a label beside it and an optional note under that. The whole \
          row is the target, and the box carries the tint — so a checklist \
          reads as one column of decisions rather than as a column of squares.
          """
      ) {
        Section(
          "States",
          description: """
            Checked, unchecked and disabled. Disabled flattens to the groove — \
            one dead look whatever the state under it — rather than a \
            translucent ghost of the live control.
            """
        ) {
          Stack {
            Specimen("checked") { Checkbox("Newsletter", checked: true) }
            Specimen("unchecked") { Checkbox("Newsletter", checked: false) }
            Specimen("disabled") {
              Checkbox("Newsletter", checked: false, enabled: false)
              Checkbox("Newsletter", checked: true, enabled: false)
            }
          }
        }

        Section(
          "Sizes and tints",
          description: """
            The box is a component token of its own — a checkbox is squarer \
            than the control radius — and its three sizes are the checkbox's, \
            not the control profile's.
            """
        ) {
          Stack {
            ForEach(CheckboxSize.allCases, id: \.self) { size in
              Specimen(size.rawValue) {
                ForEach(Tint.allCases, id: \.self) { tint in
                  Checkbox(label(tint), checked: true, size: size, tint: tint)
                }
              }
            }
          }
        }

        Section(
          "The note",
          description: """
            A note sits *under* the label rather than beside it, indented to \
            the label's own edge — so the second line reads as the same \
            decision explained, not as a second decision.
            """
        ) {
          Checkbox(
            "Share anonymous usage data",
            checked: true,
            note: "Crash reports only. Never the contents of a document."
          )
          .frame(width: 420, alignment: .leading)
        }

        Section(
          "Tokens",
          description: "The two the checkbox owns; everything else is shared."
        ) {
          TokenTable(tokens)
        }
      }
    }

    private var tokens: [TokenRow] {
      let vars = theme.vars
      return [
        TokenRow("checkbox.radius", vars.checkboxRadius),
        TokenRow("checkbox.thickness", vars.checkboxThickness),
        TokenRow("checkbox.box.small", vars.checkboxSmallBox),
        TokenRow("checkbox.box.medium", vars.checkboxMediumBox),
        TokenRow("checkbox.box.large", vars.checkboxLargeBox),
      ]
    }
  }
}
