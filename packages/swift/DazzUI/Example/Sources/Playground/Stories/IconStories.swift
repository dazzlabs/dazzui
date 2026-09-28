import DazzUI
import FluentIcons
import SwiftUI

/// `Components/Icon` — the glyph box.
enum IconStories {
  static var page: StoryPage {
    StoryPage("Icon") {
      Story("Overview", layout: .page) { Overview() }
      Story("Default") { Icon(.folder16Regular) }
      Story("Sizes") { Sizes() }
      Story("Sized by a token") { Sized() }
      Story("Inherited colour") { Inherited() }
      Story("A set") {
        Row {
          Icon(.folder16Regular)
          Icon(.document16Regular)
          Icon(.search16Regular)
          Icon(.settings16Regular)
          Icon(.person16Regular)
          Icon(.star16Regular)
        }
      }
      Story("In a button") {
        Button(variant: .filled, action: {}) {
          Icon(.arrowDownload16Regular, size: 14)
          Text("Export")
        }
      }
      Story("In a toolbar") {
        ActionBar {
          IconButton(label: "Back", icon: .chevronLeft16Regular) {}
          IconButton(label: "Forward", icon: .chevronRight16Regular) {}
          IconButton(label: "Refresh", icon: .arrowClockwise16Regular) {}
        }
        .frame(width: 200)
      }
      Story("In a nav row") {
        Col(width: 200) {
          NavItem("Projects", icon: .folder16Regular, active: true) {}
          NavItem("Settings", icon: .settings16Regular) {}
        }
      }
    }
  }

  private struct Sizes: View {
    @Environment(\.theme) private var theme

    var body: some View {
      Row(spacing: theme.vars.spacing4) {
        ForEach([12, 14, 16, 20, 24], id: \.self) { size in
          Icon(.star16Regular, size: CGFloat(size))
        }
      }
    }
  }

  /// The sizes a component actually asks for: a face's own size, so the glyph
  /// sits on the same line as the label beside it.
  private struct Sized: View {
    @Environment(\.theme) private var theme

    var body: some View {
      let vars = theme.vars

      Row(spacing: vars.spacing4) {
        Icon(.star16Regular, size: vars.captionSmall.size)
        Icon(.star16Regular, size: vars.labelSmall.size)
        Icon(.star16Regular, size: vars.labelMedium.size)
        Icon(.star16Regular, size: vars.titleSmall.size)
      }
    }
  }

  /// The glyph takes the colour of whatever it is inside, which is what makes
  /// it a child rather than a prop.
  private struct Inherited: View {
    @Environment(\.theme) private var theme

    var body: some View {
      let vars = theme.vars

      Row(spacing: vars.spacing4) {
        Icon(.star16Regular).foregroundStyle(vars.colorContent)
        Icon(.star16Regular).foregroundStyle(vars.colorContentMuted)
        Icon(.star16Regular).foregroundStyle(vars.colorContentFaint)
        Icon(.star16Regular).foregroundStyle(vars.ramp(.danger)[600])
      }
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Icon",
        description: """
          A glyph in a box. The set is Fluent — Microsoft's own Swift package, \
          the same set the React package pulls from `@fluentui/react-icons` — \
          because the React components name specific glyphs, and a port drawing \
          its own checkmark would be a different mark at a different weight in \
          the same box.
          """
      ) {
        Section(
          "Size",
          description: """
            The size is a number of points rather than a scale, because a glyph \
            is sized to the type beside it: a face's own size is the value to \
            pass, so the mark and the label share a line.
            """
        ) {
          Stack {
            Specimen("points") { Sizes() }
            Specimen("faces") { Sized() }
          }
        }

        Section(
          "Colour",
          description: """
            The glyph inherits, so an icon inside a button is the button's own \
            colour and an icon in a nav row follows the row through its states. \
            It never carries a tint of its own.
            """
        ) {
          Stack {
            Specimen("inherited") { Inherited() }
            Specimen("in a control") {
              Button(variant: .filled, action: {}) {
                Icon(.arrowDownload16Regular, size: 14)
                Text("Export")
              }
              Button(variant: .plain, action: {}) {
                Icon(.arrowDownload16Regular, size: 14)
                Text("Export")
              }
            }
          }
        }
      }
    }
  }
}
