import DazzUI
import SwiftUI

/// The story tree: the two groups, the pages inside them, and a page's own
/// stories under it once it is open.
///
/// Storybook's sidebar is one tree with a filter over it, and this is the same
/// tree drawn with the package's own `Sidebar`, `SidebarGroup`, `NavItem` and
/// `SearchField` — the playground is built out of the system it documents, so
/// its furniture answers the theme menu like everything else.
struct Navigator: View {
  let groups: [StoryGroup]
  @Binding var selection: StoryRef

  @State private var query = ""
  /// Which pages are open. Storybook opens the page you select and leaves the
  /// rest as they were, so this is a set rather than one name.
  @State private var opened: Set<String> = []

  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars

    Sidebar(resizable: true) {
      SearchField(
        value: query,
        onValueChange: { query = $0 },
        placeholder: "Find a component",
        size: .small,
        onDismiss: query.isEmpty ? nil : { query = "" }
      )
      // The sidebar's header hands its slack to a trailing spacer, so the
      // field has to ask for the width rather than take its ideal one.
      .frame(maxWidth: .infinity)
    } content: {
      ForEach(matches) { group in
        SidebarGroup(group.name) {
          ForEach(group.pages) { page in
            NavItem(
              page.name,
              active: selection.page == page.name,
              action: { open(page) }
            )

            if isOpen(page) {
              ForEach(page.stories) { story in
                NavItem(
                  story.name,
                  active: selection == StoryRef(page: page.name, story: story.name),
                  size: .small,
                  action: {
                    selection = StoryRef(page: page.name, story: story.name)
                  }
                )
                // The story rows hang off their page rather than sitting
                // level with it, which is the whole of what makes the list a
                // tree.
                .padding(.leading, vars.spacing4)
              }
            }
          }
        }
      }
    }
  }

  /// The groups a query leaves standing.
  ///
  /// A page matches by its own name — and then keeps all of its stories,
  /// since the reader was looking for the component — or by any story's, in
  /// which case only the stories that matched are worth showing.
  private var matches: [StoryGroup] {
    let needle = query.trimmingCharacters(in: .whitespaces).lowercased()
    guard !needle.isEmpty else { return groups }

    return groups.compactMap { group in
      let pages = group.pages.compactMap { page -> StoryPage? in
        if page.name.lowercased().contains(needle) { return page }
        let stories = page.stories.filter {
          $0.name.lowercased().contains(needle)
        }
        guard !stories.isEmpty else { return nil }
        return StoryPage(page.name, stories: stories)
      }

      guard !pages.isEmpty else { return nil }
      return StoryGroup(group.name, pages: pages)
    }
  }

  /// A filtered list is already the answer to the question, so it opens
  /// itself; without a query a page is open because it was asked for.
  private func isOpen(_ page: StoryPage) -> Bool {
    !query.trimmingCharacters(in: .whitespaces).isEmpty
      || opened.contains(page.name)
  }

  /// Selecting a page opens it on its `Overview`, which is the one story that
  /// says what the component is. Selecting it again folds it away.
  private func open(_ page: StoryPage) {
    if opened.contains(page.name), selection.page == page.name {
      opened.remove(page.name)
      return
    }

    opened.insert(page.name)
    if let first = page.stories.first {
      selection = StoryRef(page: page.name, story: first.name)
    }
  }
}
