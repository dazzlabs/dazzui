import DazzUI
import SwiftUI

private let listWidth = ColumnWidth(initial: 272, min: 200, max: 420)
private let themesWidth = ColumnWidth(initial: 336, min: 280, max: 560)

/// The example, as a desktop application's window: the component list on
/// the left, the document under its toolbar, and the theme panel on the
/// right — the same window the React example draws, column for column.
///
/// It is handed the page and the theme and owns only the window's own
/// state: the search, which columns are open, and how wide.
struct Shell: View {
  @Binding var page: String?
  @Binding var theme: ThemeChoice
  @Binding var sizing: ThemeSizing

  @State private var query = ""
  @State private var listOpen = true
  @State private var themesOpen = true
  @State private var list = listWidth.initial
  @State private var themes = themesWidth.initial

  @Environment(\.theme) private var current

  var body: some View {
    let vars = current.vars
    let story = Stories.story(page)
    HStack(spacing: 0) {
      // The column is never unmounted: a toolbar toggle or ⌘B only flips
      // `listOpen`, and the kit's sidebar sweeps shut behind its own edge so
      // the document beside it holds still and the list keeps its width.
      ComponentList(
        query: $query,
        page: $page,
        width: $list,
        collapsed: !listOpen,
        onCollapse: { withAnimation(vars.motion) { listOpen = false } }
      )
      VStack(spacing: 0) {
        Toolbar(
          story: story,
          // The change that reflows the window is made in the kit's motion, so
          // the pane beside the column travels with its edge.
          toggleList: { withAnimation(vars.motion) { listOpen.toggle() } },
          toggleThemes: { withAnimation(vars.motion) { themesOpen.toggle() } }
        )
        Group {
          if let story {
            Document(story: story).id(story.id)
          } else {
            Overview(stories: Stories.matching(query), open: { page = $0 })
          }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
      }
      .frame(maxWidth: .infinity)
      // The panel is never unmounted: putting it away sweeps it shut, so the
      // document beside it holds still while its edge slides over it.
      Aside(width: themes, collapsed: !themesOpen) {
        ThemePanel(theme: $theme, sizing: $sizing)
      }
      .overlay(alignment: .leading) {
        if themesOpen {
          Resizer(outward: -1, column: themesWidth, width: $themes, label: "Resize the theme panel")
            .offset(x: -(vars.spacing1 + 1))
        }
      }
    }
    .background(vars.colorSurface)
    .font(vars.bodyMedium.font)
    .foregroundStyle(vars.colorContent)
    .background {
      // ⌘B puts the list away, as the web's sidebar does.
      SwiftUI.Button("") { withAnimation(vars.motion) { listOpen.toggle() } }
        .keyboardShortcut("b", modifiers: .command)
        .hidden()
    }
  }
}

/* The component list ----------------------------------------------------- */

private struct ComponentList: View {
  @Binding var query: String
  @Binding var page: String?
  @Binding var width: CGFloat
  /// Whether the column is put away — the sidebar travels between the two,
  /// the shell only says which one it is.
  let collapsed: Bool
  let onCollapse: () -> Void

  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars
    Sidebar(
      resizable: true,
      width: $width,
      defaultWidth: listWidth.initial,
      minWidth: listWidth.min,
      maxWidth: listWidth.max,
      collapsed: collapsed,
      onCollapse: onCollapse,
      resizeLabel: "Resize the component list"
    ) {
      Brand { page = nil }
    } content: {
      SidebarGroup {
        SearchField(
          value: query,
          onValueChange: { query = $0 },
          placeholder: "Search",
          size: .small
        ) {
          EmptyView()
        }
      }
      SidebarGroup {
        NavItem(active: page == nil, size: .small, action: { page = nil }) {
          Icon(.home20Regular)
        } label: {
          Text("Overview")
        }
        .overlay(alignment: .trailing) {
          Text("\(Stories.all.count)")
            .font(vars.labelSmall.font)
            .foregroundStyle(page == nil ? vars.colorOnAccent : vars.colorContentSubtle)
            .padding(.trailing, vars.spacing2)
            .allowsHitTesting(false)
        }
      }
      SidebarGroup("Components") {
        ForEach(Stories.matching(query)) { story in
          NavItem(active: story.id == page, size: .small, action: { page = story.id }) {
            Text(story.name)
          }
        }
      }
    }
  }
}

/* The toolbar ---------------------------------------------------------- */

private struct Toolbar: View {
  let story: Story?
  let toggleList: () -> Void
  let toggleThemes: () -> Void

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  var body: some View {
    let vars = theme.vars
    HStack(spacing: vars.spacing2) {
      // The label is the accessible name and the tooltip both, so the button
      // surfaces it itself.
      IconButton(label: "Toggle sidebar ⌘B", action: toggleList) {
        Icon(.panelLeft20Regular)
      }
      HStack(alignment: .firstTextBaseline, spacing: vars.spacing2) {
        Text(story?.name ?? "Overview").fontWeight(vars.labelStrong.weight)
        Text(story.map { "\($0.examples.count) examples" } ?? "\(Stories.all.count) components")
          .font(.system(size: vars.captionLarge.size))
          .foregroundStyle(vars.colorContentSecondary)
      }
      .padding(.leading, vars.spacing1)
      Spacer()
      IconButton(label: "Toggle theme panel", action: toggleThemes) {
        Icon(.panelRight20Regular)
      }
    }
    .padding(.horizontal, vars.spacing4)
    .frame(height: vars.frameTitlebarSize)
    .overlay(alignment: .bottom) {
      Rectangle().fill(vars.colorBorder).frame(height: hairline)
    }
  }
}

/* The document ---------------------------------------------------------- */

extension View {
  fileprivate func page(_ vars: ThemeVariables) -> some View {
    padding(.top, vars.spacing5)
      .padding(.horizontal, vars.spacing6)
      .padding(.bottom, vars.spacing10)
  }
}

/// Every component at once, each by the one scene that stands for it.
private struct Overview: View {
  let stories: [Story]
  let open: (String) -> Void

  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars
    let gap = vars.spacing3
    GeometryReader { proxy in
      // The web's `repeat(auto-fill, minmax(16rem, 1fr))`. A grid rather than
      // rows of cells: a card keeps its identity when the window changes the
      // number of columns, so a width the sidebar is collapsing takes the
      // cards with it instead of crossfading one arrangement into the next.
      let room = proxy.size.width - vars.spacing6 * 2
      let columns = max(1, Int((room + gap) / (256 + gap)))
      ScrollView {
        LazyVGrid(
          columns: Array(
            repeating: GridItem(.flexible(), spacing: gap, alignment: .top),
            count: columns
          ),
          alignment: .leading,
          spacing: gap
        ) {
          ForEach(stories) { story in
            Tile(story: story, open: open)
          }
        }
        // The web reflows a grid instantly on every frame of a width change;
        // SwiftUI would animate each card from its old cell to its new one,
        // so two arrangements would be on screen at once for the length of the
        // animation. The page around it still travels with the column.
        .transaction { $0.animation = nil }
        .page(vars)
      }
    }
  }
}

private struct Tile: View {
  let story: Story
  let open: (String) -> Void

  var body: some View {
    Card(variant: .outlined) {
      SceneView(scene: story.preview)
        // The web's `min-height: 8rem`, and whatever the row adds.
        .frame(maxWidth: .infinity, minHeight: 8 * 16, maxHeight: .infinity)
      Button(variant: .plain, fullWidth: true, action: { open(story.id) }) {
        Text(story.name)
      }
    }
    .frame(maxHeight: .infinity)
  }
}

/// One component: what it is for, then each example on a stage of its own.
private struct Document: View {
  let story: Story

  @Environment(\.theme) private var theme

  var body: some View {
    let vars = theme.vars
    ScrollView {
      VStack(alignment: .leading, spacing: vars.spacing6) {
        Text(story.summary)
          .foregroundStyle(vars.colorContentSecondary)
          .frame(maxWidth: 640, alignment: .leading)
        ForEach(story.examples) { example in
          VStack(alignment: .leading, spacing: vars.spacing2) {
            SectionLabel { Text(example.title) }
            Card(size: .large, variant: .outlined) {
              SceneView(scene: example.demo)
                // The web's `min-height: 10rem`.
                .frame(maxWidth: .infinity, minHeight: 10 * 16)
            }
            if let note = example.note {
              Text(note)
                .font(.system(size: vars.captionLarge.size))
                .foregroundStyle(vars.colorContentSecondary)
            }
          }
        }
      }
      .frame(maxWidth: .infinity, alignment: .leading)
      .page(vars)
    }
  }
}
