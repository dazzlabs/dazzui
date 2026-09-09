import DazzUI
import SwiftUI

/// `Components/PreviewCard` — what is behind a link, without following it.
enum PreviewCardStories {
  static var page: StoryPage {
    StoryPage("PreviewCard") {
      Story("Overview", layout: .page) { Overview() }
      Story("On a link") { OnALink() }
      Story("A repository") { Repository() }
      Story("Sides") {
        Row {
          ForEach(PopoverSide.allCases, id: \.self) { side in
            PreviewCard(side: side) {
              Link(side.rawValue)
            } content: {
              Person()
            }
          }
        }
      }
      Story("Instant") {
        PreviewCard { Link("@ada") } content: { Person() }
      }
    }
  }

  /// A link, drawn the way the `plain` recipe draws one: the accent one grade
  /// darker than the fill, underlined.
  struct Link: View {
    let label: String

    @Environment(\.theme) private var theme

    init(_ label: String) { self.label = label }

    var body: some View {
      Text(label)
        .font(theme.vars.labelQuiet.font)
        .foregroundStyle(theme.vars.ramp(.primary)[700])
        .underline()
    }
  }

  struct Person: View {
    @Environment(\.theme) private var theme

    var body: some View {
      let vars = theme.vars

      Col {
        Row {
          Avatar("Ada Lovelace", size: .large)
          Col(spacing: vars.spacing05) {
            Text("Ada Lovelace")
              .font(.system(size: vars.labelStrong.size, weight: vars.labelStrong.weight))
              .foregroundStyle(vars.colorContent)
            Text("@ada")
              .font(vars.labelQuiet.font)
              .foregroundStyle(vars.colorContentSubtle)
          }
        }
        Text("Writes the notes that turn a machine into a program. 42 commits this month.")
          .font(vars.bodySmall.font)
          .foregroundStyle(vars.colorContentSecondary)
          .fixedSize(horizontal: false, vertical: true)
      }
    }
  }

  struct Repository: View {
    @Environment(\.theme) private var theme

    var body: some View {
      PreviewCard {
        Link("dazzlabs/dazzui")
      } content: {
        Col {
          Row {
            Avatar("DazzUI", size: .small, shape: .square)
            Text("dazzlabs/dazzui")
              .font(
                .system(
                  size: theme.vars.labelStrong.size,
                  weight: theme.vars.labelStrong.weight
                )
              )
            Badge("public", size: .small, tint: .neutral)
          }
          Text("The design system, its tokens, and four platform implementations of it.")
            .font(theme.vars.bodySmall.font)
            .foregroundStyle(theme.vars.colorContentSecondary)
            .fixedSize(horizontal: false, vertical: true)
        }
      }
    }
  }

  private struct OnALink: View {
    var body: some View {
      Row {
        Text("Reviewed by")
        PreviewCard { Link("@ada") } content: { Person() }
        Text("three days ago.")
      }
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "PreviewCard",
        description:
          "What is behind a link, shown without following it — a repository, a person, an "
          + "issue. It is a Tooltip that holds a card rather than a label, and the difference "
          + "is what may go in it: a tooltip must carry nothing a reader needs, because a "
          + "pointer never reaches it; a preview card stays open while the pointer travels, so "
          + "it can hold a link of its own. It is never the only way to reach what it shows."
      ) {
        Section(
          "A link, previewed",
          description:
            "It opens on hover — never on a press, which belongs to the link. The sheet is the "
            + "Popover’s, with a ceiling on its width: a popover is as wide as what a caller "
            + "put in it, a preview card is filled from whatever it previews."
        ) {
          OnALink()
        }
        Section(
          "Against a Tooltip",
          description:
            "If the panel has anything in it a reader might want to reach — a link, a button, a "
            + "value — it is a preview card. If it is a phrase naming what an icon does, it is "
            + "a tooltip, and it must stay one."
        ) {
          Repository()
        }
      }
    }
  }
}
