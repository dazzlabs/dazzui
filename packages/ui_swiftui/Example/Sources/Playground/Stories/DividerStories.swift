import DazzUI
import SwiftUI

/// `Components/Divider` — one device pixel, in two orientations and two tones.
enum DividerStories {
  static var page: StoryPage {
    StoryPage("Divider") {
      Story("Overview", layout: .page) { Overview() }
      Story("Horizontal") {
        DazzUI.Divider().frame(width: 320)
      }
      Story("Muted") {
        DazzUI.Divider(tone: .muted).frame(width: 320)
      }
      Story("Tones") {
        Col(width: 320) {
          ForEach(DividerTone.allCases, id: \.self) { tone in
            DazzUI.Divider(tone: tone)
          }
        }
      }
      Story("Vertical") {
        Row {
          Text("Left")
          ToolbarSeparator(size: .small)
          Text("Right")
        }
      }
      Story("In a toolbar") {
        ActionBar {
          IconButton(label: "Back", icon: .chevronLeft16Regular) {}
          IconButton(label: "Forward", icon: .chevronRight16Regular) {}
          ToolbarSeparator()
          IconButton(label: "Refresh", icon: .arrowClockwise16Regular) {}
        }
        .frame(width: 260)
      }
      Story("In a card") {
        Card {
          Text("Tokens")
          DazzUI.Divider()
          Text("Components")
        }
        .frame(width: 260)
      }
      Story("Between rows") { Rows() }
    }
  }

  private struct Rows: View {
    @Environment(\.theme) private var theme

    private let entries = ["aurora", "beacon", "cinder"]

    var body: some View {
      let vars = theme.vars

      VStack(spacing: 0) {
        ForEach(Array(entries.enumerated()), id: \.element) { index, name in
          if index > 0 { DazzUI.Divider() }
          Text(name)
            .font(vars.labelQuiet.font)
            .foregroundStyle(vars.colorContent)
            .frame(maxWidth: .infinity, alignment: .leading)
            .padding(.vertical, vars.spacing2)
            .padding(.horizontal, vars.spacing3)
        }
      }
      .frame(width: 260)
      .background(vars.colorSurface)
      .clipShape(RoundedRectangle(cornerRadius: vars.radiusMedium, style: .continuous))
      .border(vars.colorBorder, width: vars.spacingPx, cornerRadius: vars.radiusMedium)
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Divider",
        description: """
          A rule. It is one *device* pixel rather than one point: the hairline \
          halves on a Retina display, the way `base.css` does with its \
          `min-resolution` query, so a separator stays as thin as the screen \
          can draw it. A drawn control outline is a different thing and reads \
          `stroke.control` instead.
          """
      ) {
        Section(
          "Tones",
          description: """
            `default` is the ordinary border; `muted` is the stronger one, for \
            the rule that has to survive on a busier surface. Both are semantic \
            roles rather than steps off a ramp, so a theme moves them together.
            """
        ) {
          Col(width: 420) {
            ForEach(DividerTone.allCases, id: \.self) { tone in
              DazzUI.Divider(tone: tone)
            }
          }
        }

        Section(
          "Orientation",
          description: """
            A horizontal rule fills its parent. A vertical one stretches to the \
            row it sits in under CSS; here a row laid out by a `Layout` \
            proposes each child its ideal size, so a vertical rule in a toolbar \
            is given the height of the controls beside it.
            """
        ) {
          Row {
            ActionBar {
              IconButton(label: "Back", icon: .chevronLeft16Regular) {}
              IconButton(label: "Forward", icon: .chevronRight16Regular) {}
              ToolbarSeparator()
              IconButton(label: "Refresh", icon: .arrowClockwise16Regular) {}
            }
            .frame(width: 220)
          }
        }

        Section(
          "Between rows",
          description: """
            A list separates its rows with a rule between them rather than \
            under each, so the run does not end in a line hanging off the last \
            entry.
            """
        ) {
          Rows()
        }
      }
    }
  }
}
