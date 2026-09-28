import DazzUI
import SwiftUI

/// `Components/ContextMenu` — the menu a right-click opens.
enum ContextMenuStories {
  static var items: [MenuItem] {
    [
      MenuItem("Open", shortcut: "↩"),
      MenuItem("Rename", shortcut: "F2"),
      MenuItem("Duplicate", shortcut: "⌘D"),
      MenuItem("Reveal in Finder"),
      MenuItem("Delete", disabled: true),
    ]
  }

  static var page: StoryPage {
    StoryPage("ContextMenu") {
      Story("Overview", layout: .page) { Overview() }
      Story("Bare") { ContextMenu(items: items) { Target() } }
      Story("Checked") { Sorted() }
      Story("On a row") { Rows() }
    }
  }

  /// The region the menu belongs to. It draws nothing the menu needs — the
  /// catcher sits behind whatever it is given.
  struct Target: View {
    var label = "Right-click anywhere in here"

    @Environment(\.theme) private var theme

    var body: some View {
      Card {
        Text(label)
          .font(theme.vars.labelQuiet.font)
          .foregroundStyle(theme.vars.colorContentSubtle)
          .frame(width: 352, height: 144)
      }
    }
  }

  private struct Sorted: View {
    @State private var sort = "name"

    var body: some View {
      ContextMenu(
        items: ["name", "date", "size"].map { key in
          MenuItem("Sort by \(key)", checked: sort == key) { sort = key }
        }
      ) {
        Target(label: "Sorted by \(sort)")
      }
    }
  }

  private struct Rows: View {
    @Environment(\.theme) private var theme

    var body: some View {
      let vars = theme.vars

      Col(spacing: vars.spacing1, width: 352) {
        ForEach(["main", "release/4.2", "fix/token-drift"], id: \.self) { branch in
          ContextMenu(
            items: [
              MenuItem("Check out"),
              MenuItem("Rename", shortcut: "F2"),
              MenuItem("Delete", disabled: branch == "main"),
            ]
          ) {
            Text(branch)
              .font(.system(size: vars.labelSmall.size, design: .monospaced))
              .foregroundStyle(vars.colorContent)
              .frame(maxWidth: .infinity, alignment: .leading)
              .padding(.vertical, vars.spacing2)
              .padding(.horizontal, vars.spacing3)
              .background(vars.colorSurfaceMuted)
              .clipShape(
                RoundedRectangle(cornerRadius: vars.radiusMedium, style: .circular)
              )
          }
        }
      }
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "ContextMenu",
        description:
          "The menu a right-click opens, over the thing that was clicked. It is the same menu a "
          + "button opens — the panel and the rows are the menu's own, so this shares the "
          + "drawing rather than resembling it. What is its own is where it opens: at the "
          + "pointer rather than against a trigger’s edge."
      ) {
        Section(
          "A region that has a menu",
          description:
            "SwiftUI’s own `.contextMenu` draws AppKit’s menu, which is a different design; the "
            + "secondary click is caught here instead so the panel is the system’s own. Every "
            + "affordance in a context menu must also be reachable somewhere a pointer can see "
            + "— a right-click is a shortcut, never the only door."
        ) {
          ContextMenu(items: ContextMenuStories.items) { Target() }
        }
        Section(
          "Checked rows",
          description:
            "Set `checked` on every row of a group, including the unchecked ones, so the "
            + "leading column holds its width and the labels stay on one left edge."
        ) {
          Sorted()
        }
      }
    }
  }
}
