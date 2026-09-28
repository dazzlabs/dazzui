import DazzUI
import SwiftUI

/// `Components/Menubar` — several menus that behave as one.
enum MenubarStories {
  static var menus: [MenubarMenu] {
    [
      MenubarMenu(
        "File",
        items: [
          MenuItem("New project", shortcut: "⌘N"),
          MenuItem("Open…", shortcut: "⌘O"),
          MenuItem("Save", shortcut: "⌘S"),
          MenuItem("Close window", shortcut: "⌘W"),
        ]
      ),
      MenubarMenu(
        "Edit",
        items: [
          MenuItem("Undo", shortcut: "⌘Z"),
          MenuItem("Redo", shortcut: "⇧⌘Z", disabled: true),
          MenuItem("Find…", shortcut: "⌘F"),
        ]
      ),
      MenubarMenu(
        "View",
        items: [
          MenuItem("Sidebar", shortcut: "⌘1"),
          MenuItem("Dock", shortcut: "⌘2"),
          MenuItem("Full screen", shortcut: "⌃⌘F"),
        ]
      ),
    ]
  }

  static var page: StoryPage {
    StoryPage("Menubar") {
      Story("Overview", layout: .page) { Overview() }
      Story("Bare") { Menubar(menus: menus) }
      Story("With checks") { Density() }
      Story("Disabled") {
        Menubar(
          menus: menus + [
            MenubarMenu("Window", items: [MenuItem("Minimise")], disabled: true)
          ]
        )
      }
    }
  }

  private struct Density: View {
    @State private var density = "Compact"

    var body: some View {
      Menubar(
        menus: [
          MenubarMenu(
            "View",
            items: ["Compact", "Comfortable"].map { name in
              MenuItem(name, checked: density == name) { density = name }
            }
          )
        ]
      )
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Menubar",
        description:
          "The application menu bar: several menus that behave as one. That is the whole reason "
          + "it exists rather than a row of Menus — once one of them is open, moving the "
          + "pointer along the bar opens the next without a second click. It is the behaviour "
          + "every desktop menu bar has and none of them documents."
      ) {
        Section(
          "The bar",
          description:
            "Each panel is the menu’s own sheet of rows, so the drawing is the same everywhere; "
            + "the bar adds only its triggers. They are chrome rather than controls — no box at "
            + "rest, and a wash only under the pointer or while their own menu is open."
        ) {
          Menubar(menus: MenubarStories.menus)
        }
        Section(
          "Checked rows",
          description:
            "A menu that reports a choice reveals the leading column on every row of the group. "
            + "Set `checked` on all of them, including the unchecked ones, so the labels stay "
            + "on one edge."
        ) {
          Density()
        }
      }
    }
  }
}
