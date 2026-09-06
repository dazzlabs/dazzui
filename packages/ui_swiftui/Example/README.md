# Playground

The DazzUI playground, the SwiftUI counterpart of
`packages/ui_flutter/example`.

Open it in Xcode and press ⌘R:

```bash
open Playground.xcodeproj
```

That is the intended way in. The project builds a real `.app` bundle, so the
playground runs, debugs and profiles like any other Mac app.

From the command line, the same scheme:

```bash
xcodebuild -project Playground.xcodeproj -scheme Playground -destination 'platform=macOS' -derivedDataPath .build/xcode build
open .build/xcode/Build/Products/Debug/Playground.app
```

Sources are picked up from `Sources/Playground` by a synchronised folder
reference, so a new file in that directory joins the target with no project
edit.

The app resolves `DazzUI` by path, so it always builds the working copy
beside it rather than a published version.

**`swift build` and `swift run` do not work here**, and the reason is the icon
dependency rather than anything about this app: Fluent's Swift package never
declares its asset catalog as a resource, so SwiftPM alone leaves
`Bundle.module` unsynthesised and fails to compile it. Xcode's build system
processes the catalog, which is why the project exists. The package README has
the detail.

## How it is organised

The same way `apps/storybook` is, and for the same reason: a gallery of themed
pages tells you what the system looks like, but not where a component's states
are written down. So there is **one page per component**, reached from a story
tree, and the theme picker sits over all of it.

```
Sources/Playground/
  PlaygroundApp.swift    the window: toolbar, story tree, canvas
  Storybook/             the framework the stories are written against
    Story.swift          Story, StoryPage, StoryGroup — the file conventions, as types
    StoryIndex.swift     the registry: a page ships once it is named here
    Navigator.swift      the sidebar tree, and the filter over it
    Gallery.swift        documentation presentation — Page, Section, Specimen, TokenTable
    Stage.swift          what a bare story arranges itself with — Row, Col, Frame, Pane
  Stories/               one file per component
    Tokens/              the token pages
Tests/PlaygroundTests/
  StoriesTests.swift     holds the story index to the React story list
  ReferenceTests.swift   measures each component against what React drew
  Parity/                what apps/storybook's parity extractor distributed here (not committed)
```

The tests run from the same scheme:

```bash
xcodebuild -project Playground.xcodeproj -scheme Playground -destination 'platform=macOS' -derivedDataPath .build/xcode test
```

The Storybook's own conventions carry over:

- **One documented page per component, and it is `Overview`.** It is the only
  story that renders a `Page`, and it carries the whole write-up: what the
  component is, the axes it varies along, and the token table where the
  component owns tokens.
- **Every other story is the component and nothing else** — no page, no
  section, no prose. A gallery of variants is not a place to read; it is a
  place to look at the thing. Give a component as many of these as it has
  states worth seeing on their own (`Filled`, `Disabled`, `Sizes`, `Tints`),
  then the compositions that show where it lands (`In a form`, `A toolbar`,
  `In settings`).
- **A story's layout says how the canvas presents it.** `.centered` is the
  default and puts a bare specimen in the middle; `.page` is an `Overview`,
  which brings its own padding; `.fullscreen` is for a subject fixed to the
  viewport — a scrim, a toast viewport — and takes the canvas whole.
- **The two presentation files are not interchangeable.** `Gallery.swift` is
  documentation furniture and belongs to `Overview` and the token pages only.
  `Stage.swift` is what bare stories use: it arranges instances and gives a
  component the width or the window it needs, and draws nothing itself.
- The token table appears **only** where the component owns tokens — the files
  in `packages/ds/tokens/component/`, plus the `frame` metrics the shell
  columns are sized by. Most components own none: they read the shared
  vocabulary directly, which is documented on `Tokens/Control`.

Adding a component page is two steps: write
`Sources/Playground/Stories/<Name>Stories.swift` with a `static var page`, then
name it in `StoryIndex.groups`. Nothing else knows the list.

The playground is drawn with the design system it documents — the tree is the
package's own `Sidebar`, `NavItem` and `SearchField`, and the canvas furniture
reads the same tokens — so a theme switch carries the furniture with it too.

## Where to start

`Tokens/Themes` → **Four at once**. A theme here is a value scoped to a subtree
with `.theme(_:)` rather than a process-wide mode, so the four cards on that
page are the same view rendered four times, each resolving against its own
variables. `Tokens/Color` → **Palette** is the reminder that a ramp is a role
ladder rather than a lightness ramp: Bright's ink ladder puts a lighter colour
at 700 than at 600.

`Tokens/Control` → **Colors** is worth the click too: it resolves every
variant, part and state live against whichever ramp you pick, which is the
recipe system doing its job in front of you.

## Why it is a separate package

The library must not carry an entry point, a window, or a dependency that only
the playground needs — the same reason the Flutter example is its own app
rather than a target of `dazzui`.

The build produces a plain executable rather than an app bundle, which
launches without a dock presence and behind whatever window has focus. The
`AppDelegate` asks for the regular activation policy, which is what brings it
forward.
