import SwiftUI

/// The playground is organised the way `packages/ui_react/example` is, and for the same
/// reason: a gallery of six themed pages tells you what the system looks like,
/// but not where a component's states are written down. One page per
/// component, an `Overview` that argues for it, and every other story the
/// component and nothing else.
///
/// The three types below are what Storybook's file conventions are here: a
/// ``Story`` is one export, a ``StoryPage`` is one `*.stories.swift` file, and
/// a ``StoryGroup`` is the `Tokens/` or `Components/` half of the sidebar.

/// How the canvas presents a story.
///
/// The counterpart of Storybook's `parameters.layout`, and it carries the same
/// three cases: a bare specimen sits in the middle of the canvas, a page
/// brings its own padding, and a story whose subject is fixed to the viewport —
/// a scrim, a toast viewport — takes the canvas whole.
enum StoryLayout {
  /// A bare specimen, centred. The default, as it is on a component file.
  case centered
  /// A documented page. It scrolls, and pads itself.
  case page
  /// The canvas, filled and unscrolled — for what a portal target would hold.
  case fullscreen
}

/// A story's Storybook id — `components-button--all-variants` — derived the
/// way Storybook derives it from a title and an export name: lower-cased, with
/// every run of anything but a letter or a digit collapsed to one dash.
///
/// The id is the key the React reference is written under, so it is what
/// makes a story here and the story in `packages/ui_react/example` the same story. A
/// name that differs only in case or spacing (`All variants`, `All Variants`)
/// lands on the same id; a name that differs in a word does not.
func storyId(group: String, page: String, story: String) -> String {
  "\(sanitize("\(group)/\(page)"))--\(sanitize(story))"
}

private func sanitize(_ text: String) -> String {
  text
    .lowercased()
    .replacingOccurrences(of: "[^a-z0-9]+", with: "-", options: .regularExpression)
    .replacingOccurrences(of: "^-+|-+$", with: "", options: .regularExpression)
}

/// One story: a name in the sidebar and the view it draws.
struct Story: Identifiable {
  let name: String
  let sourceId: String?
  let layout: StoryLayout
  let content: () -> AnyView

  var id: String { name }

  init<Content: View>(
    _ name: String,
    layout: StoryLayout = .centered,
    sourceId: String? = nil,
    @ViewBuilder content: @escaping () -> Content
  ) {
    self.name = name
    self.layout = layout
    self.sourceId = sourceId
    // Type-erased at the boundary rather than at the call site, so a story is
    // written as the component and nothing else.
    self.content = { AnyView(content()) }
  }
}

/// One component's stories — the `Overview` first, then the axes, then the
/// compositions, in the order they are written.
struct StoryPage: Identifiable {
  let name: String
  let stories: [Story]

  var id: String { name }

  init(_ name: String, @StoryBuilder stories: () -> [Story]) {
    self.init(name, stories: stories())
  }

  /// The same page from a list that has already been made — what the
  /// sidebar's filter hands back.
  init(_ name: String, stories: [Story]) {
    self.name = name
    self.stories = stories
  }
}

/// A half of the sidebar: `Tokens` or `Components`.
struct StoryGroup: Identifiable {
  let name: String
  let pages: [StoryPage]

  var id: String { name }

  init(_ name: String, pages: [StoryPage]) {
    self.name = name
    self.pages = pages
  }
}

/// Lets a page list its stories as a block rather than as an array literal,
/// which is what keeps a story's `{ }` reading like a view body.
@resultBuilder
enum StoryBuilder {
  static func buildBlock(_ stories: Story...) -> [Story] { stories }
}

/// Where the canvas is pointed: a page and one of its stories.
///
/// A page is addressed by name rather than by index, because the sidebar
/// filters and a filtered list would renumber underneath the selection.
struct StoryRef: Hashable {
  let page: String
  let story: String
}
