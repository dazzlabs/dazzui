import DazzUI
import SwiftUI

/// `Tokens/Effect` — shadow, and the motion curve.
enum EffectStories {
  static var page: StoryPage {
    StoryPage("Effect") {
      Story("Shadow", layout: .page) { Shadow() }
    }
  }

  private struct Shadow: View {
    @Environment(\.theme) private var theme

    var body: some View {
      let vars = theme.vars

      Page(
        "Effect",
        description: """
          Seven shadow steps and one motion curve. A shadow is stored as its \
          layers rather than as a finished blur, because a token describes a \
          CSS shadow — blur *diameter* and all — and each platform draws it in \
          its own units.
          """
      ) {
        Section(
          "Shadow",
          description: """
            The hairline lift a filled button carries is the smallest step; a \
            popup takes the middle of the scale; a dialog the top of it. \
            Nothing in the system uses a shadow to say something is \
            interactive — that is the recipe's job.
            """
        ) {
          Grid(min: 200) {
            ForEach(steps, id: \.0) { name, shadows in
              Tile(name) {
                RoundedRectangle(cornerRadius: vars.radiusMedium, style: .continuous)
                  .fill(vars.colorSurface)
                  .frame(height: vars.spacing12)
                  .frame(maxWidth: .infinity)
                  .border(
                    vars.colorBorder,
                    width: vars.spacingPx,
                    cornerRadius: vars.radiusMedium
                  )
                  .tokenShadow(shadows)
                  .padding(vars.spacing2)
              }
            }
          }
        }

        Section(
          "Motion",
          description: """
            One duration and one curve, which every state change animates on. \
            A component that wanted its own timing would be saying its states \
            mean something different from every other component's.
            """
        ) {
          TokenTable([
            TokenRow("motion.duration", "\(Int(vars.motionDuration * 1000))ms"),
            TokenRow(
              "motion.easing",
              "cubic-bezier(\(g(vars.motionEasing.x1)), \(g(vars.motionEasing.y1)), \(g(vars.motionEasing.x2)), \(g(vars.motionEasing.y2)))"
            ),
          ])
        }
      }
    }

    private func g(_ value: CGFloat) -> String {
      String(format: "%g", Double(value))
    }

    private var steps: [(String, [TokenShadow])] {
      let vars = theme.vars
      return [
        ("shadow.2xs", vars.shadow2xs),
        ("shadow.xs", vars.shadowXs),
        ("shadow.sm", vars.shadowSm),
        ("shadow.md", vars.shadowMd),
        ("shadow.lg", vars.shadowLg),
        ("shadow.xl", vars.shadowXl),
        ("shadow.2xl", vars.shadow2xl),
      ]
    }
  }
}
