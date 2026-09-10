import DazzUI
import SwiftUI

/// `Components/Tree` — a hierarchy the reader walks.
enum TreeStories {
  static func file(_ id: String, _ label: String) -> TreeNode {
    TreeNode(id: id, label: label, icon: .document16Regular)
  }

  static func folder(_ id: String, _ label: String, _ children: [TreeNode]) -> TreeNode {
    TreeNode(id: id, label: label, icon: .folder16Regular, children: children)
  }

  static var files: [TreeNode] {
    [
      folder("src", "src", [
        folder("components", "components", [
          file("badge.tsx", "badge.tsx"),
          file("button.tsx", "button.tsx"),
          file("tree.tsx", "tree.tsx"),
        ]),
        folder("styles", "styles", [
          file("base.css", "base.css"),
          file("components.css", "components.css"),
        ]),
        file("index.ts", "index.ts"),
      ]),
      folder("empty", "generated", []),
      file("package.json", "package.json"),
      file("README.md", "README.md"),
    ]
  }

  static var changed: [TreeNode] {
    [
      folder("src2", "src", [
        TreeNode(id: "a", label: "tree.tsx", icon: .document16Regular, meta: "M"),
        TreeNode(id: "b", label: "tree.css", icon: .document16Regular, meta: "A"),
        TreeNode(id: "c", label: "menu.tsx", icon: .document16Regular, meta: "+12 −3"),
      ]),
      TreeNode(
        id: "d",
        label: "AGENTS.md",
        icon: .document16Regular,
        meta: "locked",
        enabled: false
      ),
    ]
  }

  static var page: StoryPage {
    StoryPage("Tree") {
      Story("Overview", layout: .page) { Overview() }
      Story("File tree") { FileTree().frame(width: 288) }
      Story("Sizes") {
        VStack(alignment: .leading, spacing: 16) {
          FileTree(size: .small)
          FileTree()
        }
        .frame(width: 288)
      }
      Story("With meta") {
        Tree(items: changed, expanded: ["src2"], selected: "a").frame(width: 288)
      }
      Story("Tints") {
        VStack(alignment: .leading, spacing: 16) {
          ForEach(Tint.allCases, id: \.self) { tint in
            Tree(items: files, expanded: ["src"], selected: "index.ts", tint: tint)
          }
        }
        .frame(width: 288)
      }
      Story("Collapsed") { Tree(items: files).frame(width: 288) }
      Story("Disabled") { Tree(items: changed, expanded: ["src2"]).frame(width: 288) }
    }
  }

  struct FileTree: View {
    var size: TreeSize = .medium
    @State private var expanded = ["src", "components"]
    @State private var selected: String? = "tree.tsx"

    var body: some View {
      Tree(
        items: TreeStories.files,
        expanded: expanded,
        selected: selected,
        size: size,
        onSelect: { selected = $0 },
        onToggle: { id in
          if let at = expanded.firstIndex(of: id) {
            expanded.remove(at: at)
          } else {
            expanded.append(id)
          }
        }
      )
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Tree",
        description:
          "A hierarchy the reader walks. React's is the one component in that package with no "
          + "Base UI part under it, because a tree's keyboard is not a list's. SwiftUI hands "
          + "focus and the arrow keys to the platform, so what is here is the part the design "
          + "decides — the indent, the twisty's column, the selection's wash and weight."
      ) {
        Section(
          "Branches and leaves",
          description:
            "children present at all — an empty array included — is what makes a node a "
            + "branch. That is how an empty folder still draws a twisty and still opens, which "
            + "is the difference between “nothing in here” and “not a folder”. The twisty "
            + "keeps its column on a leaf too, empty: files and folders at one level start at "
            + "one x."
        ) {
          TreeStories.FileTree().frame(width: 288)
        }
        Section(
          "Two targets, two meanings",
          description:
            "Clicking a folder opens it and selects it, because in a file tree those are one "
            + "gesture. Clicking the twisty alone opens it and leaves the selection where it "
            + "was — which is how a reader looks inside a folder without losing the file they "
            + "were reading."
        ) {
          TreeStories.FileTree().frame(width: 288)
        }
        Section(
          "Selection",
          description:
            "Drawn the way every selected thing in this system is: the tinted wash, and the "
            + "label at the label weight rather than the quiet one. On a column of forty rows "
            + "the weight is doing most of the work — one grade of colour is hardest to find "
            + "in exactly this shape."
        ) {
          Tree(items: TreeStories.changed, expanded: ["src2"], selected: "a")
            .frame(width: 288)
        }
      }
    }
  }
}
