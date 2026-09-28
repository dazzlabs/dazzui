import DazzUI
import SwiftUI

/// `Tokens/Control` — the shared vocabulary most components read instead of
/// owning tokens of their own.
enum ControlStories {
  static var page: StoryPage {
    StoryPage("Control") {
      Story("Sizing", layout: .page) { Sizing() }
      Story("Colors", layout: .page) { Colors() }
      Story("Focus", layout: .page) { Focus() }
    }
  }

  private struct Sizing: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Page(
        "Control sizing",
        description: """
          Four size profiles, each a height, a gap, two pads, a corner and a \
          type face. A sized control reads the profile once rather than \
          restating geometry per size — Button and IconButton keep no per-size \
          block at all, because the profile carries the design's own control \
          type.
          """
      ) {
        Section(
          "Profiles",
          description: "What every control on that step is built from."
        ) {
          DazzUI.Table {
            TableHead {
              TableCell(head: true) { Text("Size") }
              TableCell(head: true, align: .end) { Text("Height") }
              TableCell(head: true, align: .end) { Text("Pad") }
              TableCell(head: true, align: .end) { Text("Gap") }
              TableCell(head: true, align: .end) { Text("Radius") }
              TableCell(head: true, align: .end) { Text("Face") }
            }
            ForEach(DazzUI.ControlSize.allCases, id: \.self) { size in
              let metrics = theme.vars.metrics(size)
              TableRow {
                TableCell { Text(size.rawValue) }
                TableCell(align: .end) { Text(px(metrics.size)) }
                TableCell(align: .end) {
                  Text("\(px(metrics.paddingBlock)) · \(px(metrics.paddingInline))")
                }
                TableCell(align: .end) { Text(px(metrics.gap)) }
                TableCell(align: .end) { Text(px(metrics.radius)) }
                TableCell(align: .end) { Text(px(metrics.content.size)) }
              }
            }
          }
        }

        Section(
          "On the controls themselves",
          description: """
            A field, a select, a button and an icon button on the same step are \
            the same height, which is the whole point of the profile.
            """
        ) {
          Stack {
            ForEach(DazzUI.ControlSize.allCases, id: \.self) { size in
              Specimen(size.rawValue) {
                Button("Export", size: size, variant: .filled) {}
                Button("Cancel", size: size) {}
                IconButton(label: "Star", icon: .star16Regular, size: size) {}
              }
            }
          }
        }
      }
    }
  }

  private struct Colors: View {
    @Environment(\.theme) private var theme

    @State private var tint: Tint = .primary

    private let states: [(String, ControlState)] = [
      ("normal", .normal),
      ("hovered", .hovered),
      ("pressed", .pressed),
      ("disabled", .disabled),
    ]

    var body: some View {
      let vars = theme.vars

      Page(
        "Control colors",
        description: """
          Interactive colours are stored as *recipes* rather than finished \
          colours: a shade to pick off a ramp plus an alpha to apply, resolved \
          against whichever ramp the control is tinted with. Everything below \
          is resolved live — nothing here is a written-down colour.
          """
      ) {
        Section(
          "The tint",
          description: """
            Pick a ramp and every recipe under it re-resolves. `recessed` and \
            `normal` do not move: they are the neutral controls, reading the \
            theme's surface and ink roles rather than a ramp.
            """
        ) {
          SegmentedControl(
            items: Tint.allCases.map { SegmentedItem(value: $0, label: label($0)) },
            value: tint,
            onValueChange: { tint = $0 },
            size: .small
          )
        }

        Section(
          "Resolved",
          description: """
            Surface, content and border, in each of the four states. A cell \
            most recipes never write is filled in by the expansion — `hovered` \
            unwritten is `normal`, `pressed` unwritten is `hovered` dimmed — so \
            a written cell is always a decision.
            """
        ) {
          VStack(alignment: .leading, spacing: vars.spacing5) {
            ForEach(DazzUI.ControlVariant.allCases, id: \.self) { variant in
              VStack(alignment: .leading, spacing: vars.spacing2) {
                Text(variant.rawValue)
                  .font(vars.labelMedium.font)
                  .foregroundStyle(vars.colorContent)

                Row(spacing: vars.spacing3) {
                  ForEach(states, id: \.0) { name, state in
                    let colors = vars.controlColors(
                      variant: variant,
                      tint: tint,
                      state: state
                    )
                    VStack(alignment: .leading, spacing: vars.spacing1) {
                      RoundedRectangle(cornerRadius: vars.radiusSmall, style: .circular)
                        .fill(colors.surface)
                        .frame(width: vars.spacing20, height: vars.spacing8)
                        .border(
                          colors.border,
                          width: vars.strokeControl,
                          cornerRadius: vars.radiusSmall
                        )
                        .overlay {
                          Text("Aa")
                            .font(vars.labelSmall.font)
                            .foregroundStyle(colors.content)
                        }

                      Text(name)
                        .font(.system(size: vars.captionSmall.size, design: .monospaced))
                        .foregroundStyle(vars.colorContentMuted)
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }

  private struct Focus: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Page(
        "Focus",
        description: """
          The focus ring is the system accent, always. It never follows the \
          control's tint — a ring that changed colour per control would stop \
          meaning focus — and it hugs the control with no offset, the way \
          macOS draws it.
          """
      ) {
        Section(
          "Try it",
          description: """
            Tab through these. Each is a different tint, and every ring is the \
            same colour.
            """
        ) {
          Row {
            ForEach(Tint.allCases, id: \.self) { tint in
              Button(label(tint), variant: .tinted, tint: tint) {}
            }
          }
        }

        Section("Tokens", description: "What the ring is made of.") {
          TokenTable([
            TokenRow("focus.width", theme.vars.focusWidth),
            TokenRow("focus.offset", theme.vars.focusOffset),
            TokenRow("focus.ring.shade", String(theme.vars.focusRingShade)),
            TokenRow(
              "focus.ring.alpha",
              String(format: "%g", Double(theme.vars.focusRingAlpha))
            ),
            TokenRow("stroke.control", theme.vars.strokeControl),
            TokenRow("stroke.hairline", theme.vars.strokeHairline),
          ])
        }
      }
    }
  }
}
