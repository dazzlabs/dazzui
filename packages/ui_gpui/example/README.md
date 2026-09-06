# Playground

The `dazzui` playground, the GPUI counterpart of
`packages/ui_flutter/example` and `packages/ui_swiftui/Example`.

```bash
cargo run -p dazzui_playground
```

From either this directory or the one above it — the two crates share a
workspace, so the binary lands in one `target/`. On a Mac without the Metal
toolchain, add `--features runtime-shaders`; the package README says why.

The app resolves `dazzui` by path, so it always builds the working copy
beside it rather than a published version.

## How it is organised

The same way `apps/storybook` is, and for the same reason: a gallery of six
themed pages tells you what the system looks like, but not where a component's
states are written down. So there is **one page per component**, reached from a
story tree, and the theme picker sits over all of it.

```
src/
  main.rs            the window: toolbar, story tree, canvas — and the state
  storybook/
    mod.rs           Story, StoryPage, StoryGroup — the file conventions, as types
    gallery.rs       documentation presentation — DocPage, Section, Specimen, TokenTable
    stage.rs         what a bare story arranges itself with — Row, Col, Frame, Pane
  stories/
    mod.rs           the registry: a page ships once it is named here
    tokens/          the token pages
```

The Storybook's own conventions carry over:

- **One documented page per component, and it is `Overview`.** It is the only
  story that renders a `DocPage`, and it carries the whole write-up: what the
  component is, the axes it varies along, and the token table where the
  component owns tokens.
- **Every other story is the component and nothing else** — no page, no
  section, no prose. A gallery of variants is not a place to read; it is a
  place to look at the thing. Give a component as many of these as it has
  states worth seeing on their own (`Filled`, `Disabled`, `Sizes`, `Tints`),
  then the compositions that show where it lands (`In a form`, `A toolbar`,
  `In settings`).
- **A story's layout says how the canvas presents it.** `Centered` is the
  default and puts a bare specimen in the middle; `Page` is an `Overview`,
  which brings its own padding; `Fullscreen` is for a subject fixed to the
  viewport — a scrim, a toast viewport — and takes the canvas whole.
- **The two presentation files are not interchangeable.** `gallery.rs` is
  documentation furniture and belongs to `Overview` and the token pages only.
  `stage.rs` is what bare stories use: it arranges instances and gives a
  component the width or the window it needs, and draws nothing itself.
- The token table appears **only** where the component owns tokens — the files
  in `packages/ds/tokens/component/`, plus the `frame` metrics the shell
  columns are sized by. Most components own none: they read the shared
  vocabulary directly, which is documented on `Tokens/Control`.

The whole index is a `const`: a story is a plain `fn` pointer, so a page is a
`&'static [Story]` and nothing is allocated to describe the tree. Adding a page
is two steps — write `src/stories/<name>.rs` with a `pub const PAGE`, then name
it in `stories::GROUPS`.

The playground is drawn with the design system it documents — the tree is the
crate's own `Sidebar`, `NavItem` and `SearchField`, and the canvas furniture
reads the same tokens — so a theme switch carries the furniture with it too.

## The test is the coverage

`cargo test -p dazzui_playground` walks the index and draws **every story
in every theme**, headlessly, asserting that each one actually painted. GPUI
does the real layout and paint there, so a story that asks for a bad length,
loses its element state or panics in a custom element fails the test rather
than the window. A second test holds the sidebar and the canvas to the same
index, so a renamed story cannot become an empty canvas.

## Where to start

`Tokens/Themes` → **Four at once**. A theme here is a value scoped to a subtree
with `themed(…)` rather than a process-wide mode, so the four cards on that
page are the same view rendered four times, each resolving against its own
variables. `Tokens/Color` → **Palette** is the reminder that a ramp is a role
ladder rather than a lightness ramp: Bright's ink ladder puts a lighter colour
at 700 than at 600.

`Tokens/Control` → **Colors** is worth the click too: it resolves every
variant, part and state live against whichever ramp you pick, which is the
recipe system doing its job in front of you.

The fields are real: type into them, select with the mouse or with shift and
the arrow keys, and paste. `ShortcutRecorder` listens for a chord and prints it
the way a menu would.

## Why it is a separate package

The library must not carry an entry point, a window, or a dependency that only
the playground needs — the same reason the Flutter and SwiftUI examples are
their own apps rather than targets of the library. It is a workspace member
rather than a detached crate so the two share one lockfile and one `target/`.

## Where the state lives

One view holds all of it. The stories are specimens rather than screens: a
story has no state a neighbouring story could not read, and threading an entity
per component through the window would say otherwise. A story is a function of
that state and the entity to update it, which keeps the story files about what
the components look like rather than about how GPUI routes a callback.
