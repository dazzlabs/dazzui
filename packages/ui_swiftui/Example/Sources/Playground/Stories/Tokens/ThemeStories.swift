import DazzUI
import SwiftUI

/// `Tokens/Themes` — all ten, and the argument for how they are carried.
enum ThemeStories {
  static var page: StoryPage {
    StoryPage("Themes") {
      Story("All at once", layout: .page) { AllAtOnce() }
      Story("Ramps", layout: .page) { Ramps() }
    }
  }

  private struct AllAtOnce: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Page(
        "Themes",
        description: """
          Ten themes, one component tree. A theme here is a value scoped to a \
          subtree with `.theme(_:)` rather than a process-wide mode, so the \
          cards below are the same view rendered once per theme, each \
          resolving against its own variables.
          """
      ) {
        Section(
          "The families",
          description: """
            Studio Light is the baseline the others are modes over: Studio Dark \
            is the same skeleton on a near-black canvas, Bright is warm paper \
            with ink navy and an acid marker, Frost is a cool teal, Graphite is \
            monochrome and the only family that also tightens the corners, and \
            Ember is warm paper with copper.
            """
        ) {
          Grid(min: 300) {
            ForEach(ThemeName.allCases, id: \.self) { name in
              // Raised rather than outlined: the card has to bring its own
              // theme's paper with it, or a dark theme's ink would be drawn on
              // the light page underneath.
              Card {
                Sampler(name: name)
              }
              .theme(name)
            }
          }
        }
      }
    }
  }

  /// One theme's own little window: the surfaces stacked, a control on each,
  /// and the accent that separates the families.
  private struct Sampler: View {
    let name: ThemeName

    @Environment(\.theme) private var theme

    var body: some View {
      let vars = theme.vars

      HStack(spacing: vars.spacing2) {
        Text(name.title)
          .font(vars.labelLarge.font)
          .foregroundStyle(vars.colorContent)
          .lineLimit(1)
        Spacer(minLength: 0)
        Badge(
          name.colorScheme == .dark ? "dark" : "light",
          size: .small,
          variant: .tinted
        )
      }

      Text("The quick brown fox jumps over the lazy dog.")
        .font(vars.bodySmall.font)
        .lineSpacing(vars.bodySmall.lineSpacing)
        .foregroundStyle(vars.colorContentMuted)
        .fixedSize(horizontal: false, vertical: true)

      Row {
        Button("Primary", variant: .filled) {}
        Button("Quiet", variant: .plain) {}
      }

      DazzUI.Divider()

      Row {
        Badge("Passed", size: .small, variant: .tinted, tint: .success)
        Badge("Failed", size: .small, variant: .tinted, tint: .danger)
        KeyCap("⌘S")
      }

      Progress(value: 0.6, gradient: true)
    }
  }

  private struct Ramps: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Page(
        "Ramps in every theme",
        description: """
          The same six ramps, under each theme in turn. This is the page that \
          says why a component must never name a shade: the ladder under the \
          recipe is a different ladder in every block below, and the recipe \
          table above them is untouched.
          """
      ) {
        ForEach(ThemeName.allCases, id: \.self) { name in
          Section(name.title) {
            VStack(alignment: .leading, spacing: theme.vars.spacing4) {
              ForEach(Tint.allCases, id: \.self) { tint in
                Ramp(tint.rawValue, Theme(name).vars.ramp(tint))
              }
            }
            .theme(name)
            .padding(theme.vars.spacing4)
            .background(Theme(name).vars.colorSurface)
            .clipShape(
              RoundedRectangle(
                cornerRadius: theme.vars.radiusMedium,
                style: .continuous
              )
            )
          }
        }
      }
    }
  }
}
