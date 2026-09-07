# Playground

The DazzUI playground for Flutter, the sibling of
`packages/ui_swiftui/Example`.

```bash
flutter run            # macOS
flutter run -d chrome  # the web build
```

```bash
flutter analyze
flutter test
```

## How it is organised

The same way `apps/storybook` is, and for the same reason: a gallery of themed
pages tells you what the system looks like, but not where a widget's states are
written down. So there is **one page per widget**, reached from a story tree,
and the theme picker sits over all of it.

```
lib/
  main.dart              the window: toolbar, story tree, canvas
  playground_theme.dart  the ten themes, as the picker names them
  storybook/             the framework the stories are written against
    story.dart           Story, StoryPage, StoryGroup — the file conventions, as types
    story_index.dart     the registry: a page ships once it is named here
    navigator.dart       the sidebar tree, and the filter over it
    gallery.dart         documentation presentation — DocPage, DocSection, Specimen, TokenTable
    stage.dart           what a bare story arranges itself with — RowWrap, Col, Frame, Pane
  stories/               one file per widget
    tokens/              the token pages
```

The Storybook's own conventions carry over:

- **One documented page per widget, and it is `Overview`.** It is the only
  story that renders a `DocPage`, and it carries the whole write-up: what the
  widget is, the axes it varies along, and the token table where the widget
  owns tokens.
- **Every other story is the widget and nothing else** — no page, no section,
  no prose. A gallery of variants is not a place to read; it is a place to look
  at the thing. Give a widget as many of these as it has states worth seeing on
  their own (`Filled`, `Disabled`, `Sizes`, `Tints`), then the compositions
  that show where it lands (`In a form`, `A toolbar`, `In settings`).
- **A story's layout says how the canvas presents it.** `centered` is the
  default and puts a bare specimen in the middle; `page` is an `Overview`,
  which brings its own padding; `fullscreen` is for a subject fixed to the
  viewport — a scrim — and takes the canvas whole.
- **The two presentation files are not interchangeable.** `gallery.dart` is
  documentation furniture and belongs to `Overview` and the token pages only.
  `stage.dart` is what bare stories use: it arranges instances and gives a
  widget the width or the window it needs, and draws nothing itself.
- The token table appears **only** where the widget owns tokens — the files in
  `packages/ds/tokens/component/`, plus the `frame` metrics the shell columns
  are sized by. Most widgets own none: they read the shared vocabulary
  directly, which is documented on `Tokens/Control`.

Adding a page is two steps: write `lib/stories/<name>_stories.dart` with a
`static StoryPage get page`, then name it in `StoryIndex.groups`. Nothing else
knows the list.

The playground is drawn with the design system it documents — the tree is the
package's own `Sidebar`, `NavItem` and `SearchField`, and the canvas furniture
reads the same tokens — so a theme switch carries the furniture with it too.

## The test is the coverage

`test/playground_test.dart` pumps **every story in every theme** and fails on
any exception, which is what catches an overflow or a null read in a story
nobody has opened lately. It pumps the canvas directly rather than walking the
sidebar to each story: what is being tested is that a story draws, and going
through the tree would only be testing the tree.

## Where to start

`Tokens/Themes` → **All at once**. A theme here is a value scoped to a subtree
with the package's own `Theme` widget rather than a process-wide mode, so the
four cards on that page are the same widget rendered four times, each resolving
against its own variables. `Tokens/Color` → **Palette** is the reminder that a
ramp is a role ladder rather than a lightness ramp: Bright's ink ladder puts a
lighter colour at 700 than at 600.

`Tokens/Control` → **Colors** is worth the click too: it resolves every variant,
part and state live against whichever ramp you pick, which is the recipe system
doing its job in front of you.
