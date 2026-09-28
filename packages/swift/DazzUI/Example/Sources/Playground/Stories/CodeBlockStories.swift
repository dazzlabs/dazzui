import DazzUI
import SwiftUI

/// `Components/CodeBlock` — a block of code, read rather than edited.
enum CodeBlockStories {
  static let sample = """
    export function Badge({ size = 'medium', tint = 'primary' }: BadgeProps) {
      return <span data-size={size} data-tint={tint} className="dz-badge" />
    }
    """

  static let shell = "pnpm install\npnpm tokens\npnpm parity"

  static let wide =
    "const themes = ['studio-dark', 'bright-light', 'bright-dark', 'frost-light', "
    + "'frost-dark', 'graphite-light']"

  static var page: StoryPage {
    StoryPage("CodeBlock") {
      Story("Overview", layout: .page) { Overview() }
      Story("Plain") { CodeBlock(code: shell).frame(width: 480) }
      Story("With head") {
        CodeBlock(code: sample, title: "src/components/badge.tsx", language: "tsx")
          .frame(width: 480)
      }
      Story("Line numbers") {
        CodeBlock(code: sample, lineNumbers: true).frame(width: 480)
      }
      Story("Copyable") {
        CodeBlock(code: shell, language: "bash", copyable: true).frame(width: 480)
      }
      Story("Wrapped") {
        CodeBlock(code: wide, language: "ts", wrap: true).frame(width: 384)
      }
      Story("With actions") {
        CodeBlock(code: shell, title: "install.sh", copyable: true) {
          Button("Run", size: .tiny, variant: .plain) {}
        }
        .frame(width: 480)
      }
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "CodeBlock",
        description:
          "A block of code, read rather than edited. It takes the card's materials rather than "
          + "the popup's — this sits in the page, not over it — and its head is drawn only "
          + "where there is something to put in it. A bar holding one label is furniture that "
          + "says less than the space it takes."
      ) {
        Section(
          "The gutter is a column",
          description:
            "Line numbers live in their own column rather than in the same run as the code, so "
            + "a selection that crosses ten lines takes ten lines of code and not ten line "
            + "numbers — the single thing a reader does with one of these."
        ) {
          CodeBlock(
            code: CodeBlockStories.sample,
            title: "src/components/badge.tsx",
            lineNumbers: true
          )
          .frame(width: 480)
        }
        Section(
          "No highlighter",
          description:
            "language names the grammar at the trailing edge of the head and does nothing "
            + "else. This package ships no highlighter, because a grammar per language is a "
            + "dependency a design system has no opinion about."
        ) {
          CodeBlock(code: CodeBlockStories.shell, language: "bash", copyable: true)
            .frame(width: 480)
        }
        Section(
          "Long lines",
          description:
            "Scrolling is the default: a wrapped line of code has lost the one thing its "
            + "indentation was carrying. wrap is there for a block in a narrow column, where a "
            + "sideways scroller would be missed entirely."
        ) {
          VStack(alignment: .leading, spacing: 12) {
            CodeBlock(code: CodeBlockStories.wide, language: "ts")
            CodeBlock(code: CodeBlockStories.wide, language: "ts", wrap: true)
          }
          .frame(width: 480)
        }
      }
    }
  }
}
