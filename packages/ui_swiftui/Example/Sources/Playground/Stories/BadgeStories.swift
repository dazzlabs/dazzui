import DazzUI
import SwiftUI

/// `Components/Badge` — an annotation, not a control.
enum BadgeStories {
  static var page: StoryPage {
    StoryPage("Badge") {
      Story("Overview", layout: .page) { Overview() }
      Story("Tinted") { Badge("Beta") }
      Story("Filled") { Badge("Beta", variant: .filled) }
      Story("Outlined") { Badge("Beta", variant: .outlined) }
      Story("Raised") { Badge("Beta", variant: .raised) }
      Story("All variants") {
        Row {
          ForEach(BadgeVariant.allCases, id: \.self) { variant in
            Badge(label(variant), variant: variant)
          }
        }
      }
      Story("Tints") {
        Col {
          ForEach(BadgeVariant.allCases, id: \.self) { variant in
            Row {
              ForEach(Tint.allCases, id: \.self) { tint in
                Badge(label(tint), variant: variant, tint: tint)
              }
            }
          }
        }
      }
      Story("Sizes") {
        Row {
          ForEach(BadgeSize.allCases, id: \.self) { size in
            Badge(label(size), size: size)
          }
        }
      }
      Story("States") {
        Row {
          Badge("Passed", variant: .tinted, tint: .success)
          Badge("Running", variant: .tinted, tint: .info)
          Badge("Queued", variant: .tinted, tint: .neutral)
          Badge("Failed", variant: .tinted, tint: .danger)
          Badge("Deprecated", variant: .tinted, tint: .warning)
        }
      }
      Story("Counts") {
        Row {
          Badge("3", size: .small, variant: .filled)
          Badge("12", size: .small, variant: .tinted)
          Badge("99+", size: .small, variant: .outlined)
        }
      }
      Story("With a glyph") {
        Verified()
      }
      Story("In a table") {
        DazzUI.Table {
          TableHead {
            TableCell(head: true) { Text("Build") }
            TableCell(head: true, align: .end) { Text("Status") }
          }
          TableRow {
            TableCell { Text("aurora") }
            TableCell(align: .end) {
              Badge("Passed", size: .small, variant: .tinted, tint: .success)
            }
          }
          TableRow {
            TableCell { Text("cinder") }
            TableCell(align: .end) {
              Badge("Failed", size: .small, variant: .tinted, tint: .danger)
            }
          }
        }
        .frame(width: 380)
      }
      Story("On a card") {
        Card(variant: .sunken) {
          HStack {
            Text("Tokens")
            Spacer()
            Badge("Generated", size: .small, variant: .raised)
          }
          .frame(width: 260)
        }
      }
    }
  }

  /// A badge whose label is a glyph and a word. The label is a builder, so
  /// the glyph costs the component no API and takes the badge's own colour.
  private struct Verified: View {
    @Environment(\.theme) private var theme

    var body: some View {
      let vars = theme.vars

      Badge(variant: .tinted, tint: .success) {
        HStack(spacing: vars.spacing1) {
          Icon(.checkmark12Regular, size: vars.labelSmall.size)
          Text("Verified")
        }
      }
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Badge",
        description: """
          A tag: the display cut at label weight in a tight chip-cornered \
          capsule. A badge is not interactive, so it deliberately skips the \
          shared state machine and reads the resting recipes directly — a label \
          that changed colour under the pointer would read as something you can \
          press.
          """
      ) {
        Section(
          "Variants",
          description: """
            Three of them are the control recipes read at rest. `raised` is the \
            one a badge owns: paper fill and quiet ink, the chip that has to \
            stay legible on a grey card where a tinted wash would disappear.
            """
        ) {
          Stack {
            ForEach(BadgeVariant.allCases, id: \.self) { variant in
              Specimen(variant.rawValue) {
                ForEach(Tint.allCases, id: \.self) { tint in
                  Badge(label(tint), variant: variant, tint: tint)
                }
              }
            }
          }
        }

        Section(
          "Sizes",
          description: """
            A badge is an annotation rather than a control, so it has three \
            sizes of its own and never reaches control height. The face and the \
            weight never soften: only the box and, at `large`, one step of type.
            """
        ) {
          Stack {
            ForEach(BadgeSize.allCases, id: \.self) { size in
              Specimen(size.rawValue) {
                Badge("Beta", size: size)
                Badge("Passed", size: size, variant: .tinted, tint: .success)
                Badge("12", size: size, variant: .filled)
              }
            }
          }
        }

        Section(
          "Where it lands",
          description: """
            A status in a table row, a count beside a label, a state on a card. \
            Its tint carries the meaning, which is why the status ramps are the \
            ones it is usually read against.
            """
        ) {
          Stack {
            Specimen("states") {
              Badge("Passed", variant: .tinted, tint: .success)
              Badge("Running", variant: .tinted, tint: .info)
              Badge("Failed", variant: .tinted, tint: .danger)
              Badge("Deprecated", variant: .tinted, tint: .warning)
            }
            Specimen("glyph") {
              Verified()
            }
            Specimen("counts") {
              Badge("3", size: .small, variant: .filled)
              Badge("12", size: .small, variant: .tinted)
              Badge("99+", size: .small, variant: .outlined)
            }
            Specimen("on grey") {
              Card(variant: .sunken) {
                HStack {
                  Text("Tokens")
                  Spacer()
                  Badge("Generated", size: .small, variant: .raised)
                }
                .frame(width: 240)
              }
            }
          }
        }
      }
    }
  }
}
