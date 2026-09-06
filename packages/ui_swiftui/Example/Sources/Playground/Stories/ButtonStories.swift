import DazzUI
import FluentIcons
import SwiftUI

/// `Components/Button` — the SwiftUI half of
/// `apps/storybook/src/stories/button.stories.tsx`.
enum ButtonStories {
  static let variants: [DazzUI.ControlVariant] = [
    .normal, .recessed, .filled, .tinted, .outlined, .plain,
  ]

  /// The recipes that read a ramp. `recessed` and `normal` are the neutral
  /// controls: they read the theme's surface and ink roles rather than a shade
  /// off a ramp, so `tint` does nothing to them — a neutral chip that could
  /// turn red would no longer be neutral.
  static let tinted: [DazzUI.ControlVariant] = [.filled, .tinted, .outlined, .plain]

  static var page: StoryPage {
    StoryPage("Button") {
      Story("Overview", layout: .page) { Overview() }
      Story("Normal") { Button("Save changes", variant: .normal) {} }
      Story("Recessed") { Button("Save changes", variant: .recessed) {} }
      Story("Filled") { Button("Save changes", variant: .filled) {} }
      Story("Tinted") { Button("Save changes", variant: .tinted) {} }
      Story("Outlined") { Button("Save changes", variant: .outlined) {} }
      Story("Plain") { Button("Save changes", variant: .plain) {} }
      Story("All variants") {
        Row {
          ForEach(variants, id: \.self) { variant in
            Button(label(variant), variant: variant) {}
          }
        }
      }
      Story("Disabled") {
        Row {
          ForEach(variants, id: \.self) { variant in
            Button(label(variant), variant: variant, enabled: false) {}
          }
        }
      }
      Story("Sizes") {
        Row {
          ForEach(DazzUI.ControlSize.allCases, id: \.self) { size in
            Button(label(size), size: size, variant: .filled) {}
          }
        }
      }
      Story("Tints") {
        Col {
          ForEach(tinted, id: \.self) { variant in
            Row {
              ForEach(Tint.allCases, id: \.self) { tint in
                Button(label(tint), variant: variant, tint: tint) {}
              }
            }
          }
        }
      }
      Story("With an icon") {
        Button(variant: .filled, action: {}) {
          Icon(.arrowDownload16Regular, size: 14)
          Text("Export")
        }
      }
      Story("With a shortcut") {
        Row {
          Button("Translate", variant: .filled, shortcut: "⌘S") {}
          Button("Translate all", shortcut: "⇧⌘S") {}
        }
      }
      Story("Busy") {
        Button(variant: .filled, enabled: false, action: {}) {
          Spinner(size: .small)
          Text("Translating…")
        }
      }
      Story("Full width") {
        Col(width: 280) {
          Button("Continue", variant: .filled, fullWidth: true) {}
          Button("Not now", fullWidth: true) {}
        }
      }
      Story("Destructive") {
        Button("Delete glossary", variant: .filled, tint: .danger) {}
      }
      Story("In an action bar") {
        ActionBar(align: .end) {
          Button("Cancel", variant: .recessed) {}
          Button("Export", variant: .filled) {}
        }
        .frame(width: 320)
      }
    }
  }

  /// The documented page. Every story after this one is the component alone.
  private struct Overview: View {
    var body: some View {
      Page(
        "Button",
        description: """
          The six control recipes, each on any ramp. Hover, press and disabled \
          are not written here at all — the shared state machine puts the \
          recipe's own states on the box, so a button is only the box.
          """
      ) {
        Section(
          "Variants",
          description: """
            `normal` is the neutral control on the paper, edged by a hairline; \
            `recessed` is the same control one step into it — the workaday \
            grey chip a toolbar or a footer is full of, and the default. \
            `filled` is the one action a view points at, and the only one that \
            carries the hairline lift; `outlined` is its drawn twin; `tinted` \
            a quiet accent chip; `plain` looks like text at rest but keeps the \
            box, because its recipe washes on hover.
            """
        ) {
          Stack {
            ForEach(variants, id: \.self) { variant in
              Specimen(variant.rawValue) {
                Button("Save changes", variant: variant) {}
                Button("Disabled", variant: variant, enabled: false) {}
              }
            }
          }
        }

        Section(
          "Tints",
          description: """
            Nothing below sets a colour: each row is one variant read against \
            six ramps, and the hover and press states come from the same \
            recipe, so a danger button darkens the way a primary one does. \
            `recessed` and `normal` are absent on purpose — they are the \
            neutral controls, and `tint` leaves them untouched.
            """
        ) {
          Stack {
            ForEach(tinted, id: \.self) { variant in
              Specimen(variant.rawValue) {
                ForEach(Tint.allCases, id: \.self) { tint in
                  Button(label(tint), variant: variant, tint: tint) {}
                }
              }
            }
          }
        }

        Section(
          "Sizes",
          description: """
            Button keeps no per-size block at all: the size profile carries \
            the height, the padding, the radius and the type face, and the \
            view reads them once. `tiny` is the icon button's step — a labelled \
            button rarely wants it.
            """
        ) {
          Stack {
            ForEach(DazzUI.ControlSize.allCases, id: \.self) { size in
              Specimen(size.rawValue) {
                Button("Export", size: size, variant: .filled) {}
                Button("Cancel", size: size) {}
                Button("Learn more", size: size, variant: .plain) {}
              }
            }
          }
        }

        Section(
          "Content",
          description: """
            An icon is a child rather than a prop, so it costs the component \
            no API and takes the button's own colour. A shortcut is the one \
            trailing slot, set in that colour at a lowered opacity — a key cap \
            inside a button would read as a second control. There is no \
            `loading` flag either: work in flight is a spinner in the label and \
            an unusable button.
            """
        ) {
          Stack {
            Specimen("icon") {
              Button(variant: .filled, action: {}) {
                Icon(.arrowDownload16Regular, size: 14)
                Text("Export")
              }
            }
            Specimen("shortcut") {
              Button("Translate", variant: .filled, shortcut: "⌘S") {}
              Button("Translate all", shortcut: "⇧⌘S") {}
            }
            Specimen("busy") {
              Button(variant: .filled, enabled: false, action: {}) {
                Spinner(size: .small)
                Text("Translating…")
              }
            }
          }
        }

        Section(
          "In a row",
          description: """
            Where more than one button sits together the row is an `ActionBar`, \
            so the gap comes from the control scale. The confirming action goes \
            last and is the only filled one: two filled buttons side by side ask \
            the reader to choose which is the default.
            """
        ) {
          Stack {
            Specimen("confirming") {
              ActionBar(align: .end) {
                Button("Cancel", variant: .recessed) {}
                Button("Export", variant: .filled) {}
              }
              .frame(width: 320)
            }
            Specimen("destructive") {
              ActionBar(align: .end) {
                Button("Keep", variant: .recessed) {}
                Button("Delete glossary", variant: .filled, tint: .danger) {}
              }
              .frame(width: 320)
            }
          }
        }
      }
    }
  }
}
