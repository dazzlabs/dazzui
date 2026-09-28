import DazzUI
import SwiftUI

/// `Tokens/Typography` — the faces, and what each is for.
enum TypographyStories {
  static let sample = "Sharp, considered interfaces"

  static var page: StoryPage {
    StoryPage("Typography") {
      Story("Scale", layout: .page) { Scale() }
    }
  }

  private struct Scale: View {
    @Environment(\.theme) private var theme

    var body: some View {
      let vars = theme.vars

      Page(
        "Typography",
        description: """
          Four families of face — headline, title, body, label — plus the \
          caption. Leading is stated absolutely rather than as a multiple, \
          because that is how the design states it: a label face sets its line \
          height to its own size so the text sits on the centre of a 26pt box, \
          and a multiple would round that away.
          """
      ) {
        Section(
          "Headline",
          description: "The page's own name, used once."
        ) {
          faces([
            ("headline.small", vars.headlineSmall),
            ("headline.medium", vars.headlineMedium),
            ("headline.large", vars.headlineLarge),
          ])
        }

        Section("Title", description: "A section's heading.") {
          faces([
            ("title.small", vars.titleSmall),
            ("title.medium", vars.titleMedium),
            ("title.large", vars.titleLarge),
          ])
        }

        Section("Body", description: "Prose, and anything a reader reads.") {
          faces([
            ("body.small", vars.bodySmall),
            ("body.medium", vars.bodyMedium),
            ("body.large", vars.bodyLarge),
          ])
        }

        Section(
          "Label",
          description: """
            The display cut a control is set in. `quiet` and `strong` are the \
            two weights selection moves between: a row rests at `quiet` and the \
            selected entry takes the label weight, while a pill tab's active \
            label and an option card's title go to `strong`.
            """
        ) {
          faces([
            ("label.quiet", vars.labelQuiet),
            ("label.small", vars.labelSmall),
            ("label.medium", vars.labelMedium),
            ("label.large", vars.labelLarge),
            ("label.strong", vars.labelStrong),
          ])
        }

        Section("Caption", description: "The line under something else.") {
          faces([
            ("caption.small", vars.captionSmall),
            ("caption.medium", vars.captionMedium),
            ("caption.large", vars.captionLarge),
          ])
        }
      }
    }

    private func faces(_ entries: [(String, TypeStyle)]) -> some View {
      let vars = theme.vars

      return VStack(alignment: .leading, spacing: vars.spacing4) {
        ForEach(entries, id: \.0) { name, style in
          VStack(alignment: .leading, spacing: vars.spacing1) {
            Text(TypographyStories.sample)
              .font(style.font)
              .lineSpacing(style.lineSpacing)
              .foregroundStyle(vars.colorContent)

            Text("\(name) · \(px(style.size)) / \(px(style.lineHeight))")
              .font(.system(size: vars.captionSmall.size, design: .monospaced))
              .foregroundStyle(vars.colorContentMuted)
          }
        }
      }
    }
  }
}
