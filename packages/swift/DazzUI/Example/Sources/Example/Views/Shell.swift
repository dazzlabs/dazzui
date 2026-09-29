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
      if listOpen {
        ComponentList(query: $query, page: $page)
          .frame(width: list)
      }
      VStack(spacing: 0) {
        Toolbar(
          story: story,
          toggleList: { listOpen.toggle() },
          toggleThemes: { themesOpen.toggle() }
        )
        Group {
          if let story {
            Document(story: story).id(story.id)
          } else {
            Overview(stories: Stories.matching(query), open: { page = $0 })
          }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .overlay(alignment: .leading) {
          if listOpen {
            Resizer(outward: 1, column: listWidth, width: $list, label: "Resize the component list", lineAtStart: true)
          }
        }
      }
      .frame(maxWidth: .infinity)
      if themesOpen {
        ThemePanel(theme: $theme, width: themes)
          .overlay(alignment: .leading) {
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
      SwiftUI.Button("") { listOpen.toggle() }
        .keyboardShortcut("b", modifiers: .command)
        .hidden()
    }
  }
}

/* The component list ----------------------------------------------------- */

private struct ComponentList: View {
  @Binding var query: String
  @Binding var page: String?

  @Environment(\.theme) private var theme
  @Environment(\.hairline) private var hairline

  var body: some View {
    let vars = theme.vars
    VStack(alignment: .leading, spacing: 0) {
      Brand { page = nil }
        .padding(.vertical, vars.spacing2)
        .padding(.horizontal, vars.spacing25)
        .frame(minHeight: vars.frameTitlebarSize, alignment: .leading)
      ScrollView {
        VStack(alignment: .leading, spacing: 0) {
          SearchField(value: query, onValueChange: { query = $0 }, placeholder: "Search", size: .small) {
            EmptyView()
          }
          .padding(.bottom, vars.spacing15)
          .padding(.horizontal, vars.spacing25)

          NavItem(active: page == nil, size: .small, action: { page = nil }) {
            Icon(.home20Regular, size: 16)
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
          .padding(.vertical, vars.spacing15)
          .padding(.horizontal, vars.spacing25)

          VStack(alignment: .leading, spacing: vars.frameNavGap) {
            SectionLabel { Text("Components") }
              .padding(.vertical, vars.spacing1)
              .padding(.horizontal, vars.spacing25)
            ForEach(Stories.matching(query)) { story in
              NavItem(active: story.id == page, size: .small, action: { page = story.id }) {
                Text(story.name)
              }
            }
          }
          .padding(.vertical, vars.spacing15)
          .padding(.horizontal, vars.spacing25)
        }
        .padding(.bottom, vars.spacing2)
      }
    }
    .frame(maxHeight: .infinity, alignment: .top)
    .background(vars.colorSurfaceColumn)
    .overlay(alignment: .trailing) {
      Rectangle().fill(vars.colorBorder).frame(width: hairline)
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
      Tooltip(side: .bottom) { Text("Toggle sidebar  ⌘B") } content: {
        IconButton(label: "Toggle sidebar", action: toggleList) {
          Icon(.panelLeft20Regular, size: 16)
        }
      }
      HStack(alignment: .firstTextBaseline, spacing: vars.spacing2) {
        Text(story?.name ?? "Overview").fontWeight(vars.labelStrong.weight)
        Text(story.map { "\($0.examples.count) examples" } ?? "\(Stories.all.count) components")
          .font(.system(size: vars.captionLarge.size))
          .foregroundStyle(vars.colorContentSecondary)
      }
      .padding(.leading, vars.spacing1)
      Spacer()
      Tooltip(side: .bottom) { Text("Toggle theme panel") } content: {
        IconButton(label: "Toggle theme panel", action: toggleThemes) {
          Icon(.panelRight20Regular, size: 16)
        }
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

private extension View {
  func page(_ vars: ThemeVariables) -> some View {
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
      // The web's `repeat(auto-fill, minmax(16rem, 1fr))`.
      let room = proxy.size.width - vars.spacing6 * 2
      let columns = max(1, Int((room + gap) / (256 + gap)))
      ScrollView {
        Grid(horizontalSpacing: gap, verticalSpacing: gap) {
          ForEach(Array(stride(from: 0, to: stories.count, by: columns)), id: \.self) { start in
            GridRow {
              ForEach(start..<(start + columns), id: \.self) { index in
                if index < stories.count {
                  Tile(story: stories[index], open: open)
                } else {
                  Color.clear.gridCellUnsizedAxes([.vertical])
                }
              }
            }
          }
        }
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
