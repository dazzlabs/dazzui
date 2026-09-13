# AGENTS.md

Guidance for coding agents working in this repository.

## What this is

The DazzUI design system, as a pnpm workspace:

- `packages/ds` (`@dazzlabs/dazzui-ds`) — the design system: the tokens, built
  with [Terrazzo](https://terrazzo.app/). It holds no components, it is the
  only place a value is decided, and it is a **generator rather than a
  dependency**: its build writes into the platform packages, which nothing
  imports it from.
- `packages/ui_react` (`@dazzlabs/dazzui-react`) — the React implementation.
  `src/generated/` holds what `ds` wrote and is committed, so the package
  builds and ships on its own. Its behaviour comes from
  [Base UI](https://base-ui.com) — headless primitives that own the parts a
  design system has no opinion about, and that a hand-rolled component gets
  wrong for years: focus traps, roving arrow keys, type-ahead, collision-aware
  positioning, the hidden inputs a form needs. This package is what is drawn
  on top. Further platform packages sit beside it as they land
  (`ui_swiftui`), which is why the directory carries the platform in its
  name.
- `packages/ui_flutter` (`dazzui`) — the Flutter implementation,
  migrated from `dazzui-flutter`. A plain Dart/Flutter package, invisible to pnpm:
  run `flutter analyze` / `flutter test` from inside the package. Like the
  React package, `lib/src/generated/` holds what `ds` wrote and is committed.
  The ds vocabulary is the API: the color axis is `tint` (`WidgetTint`,
  `ButtonTint.neutral`), never dazzui-flutter's `kind`/`secondary`. It imports
  **nothing from `package:flutter/material.dart`** and ships
  `uses-material-design: false` — a design system that drew every one of its
  own controls has no use for a second one's, and material's own vocabulary
  (`ThemeExtension`, `Icons`, the `Material` that carried an ambient text
  style) is small enough to replace outright: the base class is
  `src/foundation/theme_extension.dart`, the ambient style is the package's
  own `Theme`, and the three glyphs the kit draws itself come from
  `fluentui_system_icons` — the same set `ui_react` takes them from. A host
  supplies the app plumbing with `WidgetsApp`; see `example/lib/host.dart`
  and `test/host.dart`. `example/` holds the playground app (`flutter run`
  from that directory, macOS and web).
- `packages/ui_gpui` (`dazzui`) — the [GPUI] implementation, in Rust.
  A Cargo workspace, invisible to pnpm: run `cargo test` / `cargo clippy` from
  inside the package. Like the others, `src/generated/` holds what `ds` wrote
  and is committed. GPUI has no cascade *and* no default arguments, so the
  generator splits the token set in two — `ThemeSeed` is what a theme
  re-points and a `const fn` derives the rest — which is what lets each theme
  be a `static` a component borrows rather than a value it rebuilds.
  `example/` holds the playground (`cargo run -p dazzui_playground`), a
  workspace member so the two share one `target/`.
- `packages/ui_inset` (`dazzui_inset`) — the Inset implementation in Rust,
  ported from Flutter. An independent Cargo workspace with a 54-page example;
  `pnpm inset`, `pnpm inset:test`, and `pnpm inset:check` run it and its checks.
  It uses the sibling `../inset` checkout and the package-pinned nightly toolchain.
  `ds/plugins/inset.mjs` shares the Rust token compiler with GPUI and emits
  committed native Inset sources under `src/generated/`; never edit them.
  See its README and PORTING.md for setup and remaining behavioral parity gaps.
- `packages/ui_winui3` (`DazzUI`) — the WinUI 3 implementation, in C#.
  Invisible to pnpm's workspace graph; `pnpm winui3` builds and opens the
  unpackaged, self-contained Windows playground. `pnpm winui3:test` runs
  real-window smoke tests. `src/DazzUI/Generated/tokens.json` is emitted
  by `ds/plugins/winui3.mjs`, distributed by the token build, embedded in the
  library and committed. Controls delegate behavior to native WinUI and draw
  the shared recipes through `Themes/Generic.xaml`. The playground includes
  the native component catalog and token pages, with story names synchronized
  by `scripts/sync-stories.mjs`. It is not yet part of the React pixel/reference
  parity suites; its own smoke suite renders every story in all twelve themes.
  See its README for the current scope and standalone .NET commands.
- `packages/mobile_ui_ios` (`DazzMobileUI`) — native SwiftUI for
  mobile. This package uses ds colours and entirely system components; it
  intentionally does not port React geometry or custom control rendering.
  `Sources/DazzMobileUI/Generated/` is committed output from ds.
  `mobileTheme` installs the accent and palette, following the OS appearance
  by default. Use SwiftUI controls and SF Symbols directly. Run `swift test`
  inside the package; `Example/Playground.xcodeproj` is the runnable iOS
  gallery. Open it, select an iPhone or iPad simulator, and run the Playground
  scheme. Its project configuration is generated from `Example/project.yml`.
  `Example/GeneratedStories.swift` and `Example/Tests/scenes.json` are generated
  from the React example's shared declarations by `pnpm playgrounds:generate`.
  `tools/playgrounds/ios-scene.mjs` maps content and events to system controls;
  mobile appearance remains native. The Xcode test scheme checks the registry,
  all-theme rendering and shared non-timed interaction checkpoints.
- `packages/ui_react/example` (`@dazzlabs/dazzui-react-playground`) — the React playground that
  documents both: `Tokens/*` for the token graph, `Components/*` for the
  components.
- `examples/workbench` (`@dazzlabs/dazzui-example-workbench`) — a Git-rooted
  agent workbench, built entirely out of `ui_react`: several projects open as
  titlebar tabs, a sidebar whose lower half is whatever the open page is
  *made of*, and seven pages on ⌘1–7 — chat with a dock beside it, a
  cross-project inbox, review, Git, tests, GitHub (issues and pull requests
  as two faces of one page) and release — with settings as a sheet on ⌘,
  rather than an eighth page. It is where the
  system is *used* rather than documented: a gallery shows every component on
  its own, and only an app has to make them agree with each other. It
  consumes the package the way any consumer would — the two published
  stylesheets and the components — and holds no literal values of its own. It
  runs in a browser tab and, through
  [Deno Desktop](https://docs.deno.com/runtime/desktop/), as a native window;
  `deno` is a devDependency of the package, so there is nothing to install
  for it.
- `examples/canvas` (`@dazzlabs/dazzui-example-canvas`) — a design canvas, built
  the same way and asking the design system a different question. The
  workbench is a reading app; this one has a *drawing* in the middle of it, which
  is the one region of a window whose colours are usually the user's rather
  than the designer's. Here they are not: a shape carries a `tint` and a
  `variant` exactly as a `Badge` does, and the token build resolves the pair,
  so the drawing follows the theme menu into Bright and into the dark. It runs
  on its own port and packages into its own window.

The split is the point: anything a platform package needs but cannot express
belongs in `packages/ds`, so the platforms stay implementations of one design
rather than four designs that resemble each other.

[GPUI]: https://gpui.rs

Every domain-free component from `beyondtranslate/packages/ui` has been
migrated. What is left there is the desktop shell — WindowFrame, Sidebar,
MiniWindow, BrowserFrame and friends — which is still pending a decision, and
the mobile device frames, which are presentation props rather than system
parts.

## Commands

`packages/mobile_ui_android` is an independent Android Gradle library with an
`example` app. It uses only stock AndroidX Compose Material 3 components and
ds colors; desktop geometry and React component parity do not apply. Its
`DazzTheme` changes only `MaterialTheme.colorScheme`. Do not add custom
control drawing or wrappers. `packages/ds/plugins/compose.mjs` generates its
committed `src/main/kotlin/dev/dazzlabs/dazzui/mobile/generated/` file with all
theme families. Run `pnpm tokens` to regenerate; never edit the output.
From that package, run `./gradlew assembleDebug testDebugUnitTest :example:assembleDebug`
with JDK 17 and Android SDK 36. The package is invisible to pnpm. Its example
consumes generated `example/src/main/assets/scenes.json` from the React example's
shared declarations. The generic Compose renderer and state/clock interpreter
contain no story-specific fixtures. Run `:example:connectedDebugAndroidTest` on a
connected device to exercise shared checkpoints and all-theme native rendering.

Run these from the repository root.

```bash
pnpm install
```

```bash
pnpm tokens            # build packages/ds into its dist/
```

```bash
pnpm components        # build packages/ui_react into its dist/
```

```bash
pnpm playground         # build both, then serve the React playground on :6006
```

```bash
pnpm workbench         # build both, then serve the example on :5180
```

```bash
pnpm workbench:desktop  # build the example, then open it as a native window
```

```bash
pnpm canvas            # build both, then serve the canvas example on :5181
```

```bash
pnpm canvas:desktop    # build the canvas, then open it as a native window
```

```bash
pnpm parity            # build the React playground, then extract the React reference and distribute it
```

```bash
pnpm parity:view       # the drift viewer on :5182 — what each platform measured beside React
```

```bash
pnpm parity:flutter    # draw the Flutter specimens in a real window, where the glyphs are real
```

```bash
pnpm typecheck         # tsc over every workspace package
```

The Rust package is invisible to pnpm, the way the Flutter one is. From
`packages/ui_gpui`:

```bash
cargo test             # the token assertions, and a headless draw of every component
```

```bash
cargo run -p dazzui_playground   # the playground, in a window
```

Add `--features runtime-shaders` on a Mac without the Metal toolchain
installed; the package README says why.

```bash
pnpm lint              # oxlint over the workspace
```

```bash
pnpm format            # oxfmt over the workspace
```

## Token pipeline

1. `scripts/build.mjs` reads the files listed in `terrazzo.config.mjs` and
   hands them to Terrazzo's own `parse` and `build` — the two calls
   `tz build` makes — with one step of its own in between: the recipe table
   under `control.color` writes only its decisions, and
   `scripts/recipes.mjs` fills in the derived cells before the graph is
   parsed (see the token conventions below). The plugins then stage
   `packages/ds/dist/`:
   - `style.css` — CSS custom properties (the contract consumers use)
   - `control.css` — the ramp bindings and the resolved interactive color
     recipes, from `plugins/control.mjs`
   - `flutter/colors.dart` + `flutter/theme_variables.dart` — the Dart token
     sources, from `plugins/flutter.mjs`
   - `swiftui/Colors.swift` + `swiftui/ThemeVariables.swift` — the Swift token
     sources, from `plugins/swiftui.mjs`
   - `gpui/colors.rs` + `gpui/theme_variables.rs` — the Rust token sources,
     from `plugins/gpui.mjs`

   `plugins/contrast.mjs` runs alongside and emits nothing: it holds the ink
   ladder to WCAG contrast in every theme and fails the build when a
   re-pointed role lands below the floor. (Terrazzo's own lint rule cannot
   do this — 0.10 lints each token file with only that file's tokens.)
2. `distribute()` in `scripts/build.mjs` then copies each platform's files out
   of that staging directory into the platform package itself — the two
   stylesheets into `packages/ui_react/src/generated/`, the two Dart files
   into `packages/ui_flutter/lib/src/generated/`, and so on for Swift and
   Rust. The `TARGETS` list is where a new platform package is added. The
   Dart copies are then run through `dart format` (PATH first, fvm's stable
   as fallback), because the committed files are formatted and a rebuild
   must not dirty them.

Nothing writes a token file: every source, `_base.yaml` included, is
hand-authored. (The Tailwind sync that used to write `_base.yaml` is gone —
the design is drawn to BeyondTranslate, so there is no upstream to track, and
what Tailwind contributed is frozen in `_base.yaml` at the values it had.)
The recipe expansion is the one transformation between a file and the graph,
and it lives only in the build process — the expanded document is never
written anywhere.

A new token file is only built once it is added to `terrazzo.config.mjs`.
`dist/` is gitignored and is staging only — the distributed copies are what a
platform package reads, and they are committed.

## Component pipeline

`vite build` bundles `packages/ui_react/src/` into that package's own `dist/`:

- `index.js` + `*.d.ts` — the components; React and Base UI stay external
- `components.css` — every component stylesheet, via the `styles.ts` entry

Base UI is external for the reason React is, and one more: it holds the open
popup, the toast queue and the field's wiring in React context, so a second
copy in the bundle would be a second set of contexts and a provider a
consumer's own components could not see.

`src/styles/components.css` is the registry, the way `terrazzo.config.mjs` is
for tokens: a component's stylesheet only ships once it is imported there. It
also `@import`s `../generated/control.css`, so the generated recipes are
bundled in rather than pushed onto consumers as a third stylesheet.

Because `src/generated/` is committed, this package builds without `ds` having
run — there is no dependency edge and so no build order to get wrong. The cost
is that editing a token file does nothing until the generator runs, which is
why `pnpm packages` runs both and the playground scripts call it.

Never hand-edit `src/generated/`. A value a component needs belongs in
`packages/ds`, and anything written there is overwritten on the next build.

## Parity pipeline

`packages/ui_react` is the standard the other platforms are held to, and the
standard is written in stylesheets — which a port reads with its eyes. The
parity pipeline reads it with a browser instead, so that what a port is held
to is a number rather than an impression.

1. `packages/ui_react/example/scripts/parity.mjs` serves the React playground build and opens
   every story it lists, in every theme, in a headless Chromium at a fixed
   window (960×640 at 2x, reduced motion). For each it records every
   component root on the page — anything carrying a bare `ff-*` class — with
   its `data-*` axes, its box, and the computed border, corner, colour, type
   and shadow; and takes a screenshot. It stages three things under
   `packages/ui_react/example/parity/` (gitignored, the way `dist/` is for tokens):
   - `stories.json` — the story list, and the cross-platform key
   - `reference/<page>.json` — what every story rendered, per theme
   - `screenshots/<theme>/<id>.png` — for the side-by-side report; never
     distributed, since a screenshot is only reproducible on one machine
2. It then distributes `stories.json` and `reference/` into each platform's
   parity directory — `packages/ui_flutter/example/test/parity/`,
   `packages/ui_swiftui/Example/Tests/PlaygroundTests/Parity/`,
   `packages/ui_gpui/tests/parity/` — so a platform's tests run from inside
   the package. Unlike the generated token files, **none of this is
   committed**: it is a measurement, not a contract, and it is regenerated
   by `pnpm parity`. A checkout without it skips the parity tests rather
   than failing them. The `TARGETS` list in the script is where a platform
   is added.

**The story id is the key.** A story is `components-button--all-variants`
on every platform: Storybook's own derivation (title and name lower-cased,
runs of anything but letters and digits collapsed to a dash), reproduced by
`storyId()` in each playground's storybook framework. That is what lets a
name written `All variants` in Dart and `All Variants` in React be one story,
and it is why page and story names in a port follow the React file exactly.

Each playground carries a registry test (`stories_test.dart`,
`StoriesTests.swift`, `stories_test.rs`) that holds its own index to the
React list, both ways:
a React story the port does not draw fails unless it is named in
`pending.txt`, and a story the port draws that React does not have fails
unless it is named in `extra.txt`. Both lists are ratchets — an entry that is
no longer true fails too, so a pending story that lands has to leave the
list in the same change. The lists are recorded by the first run that finds
them missing and are not committed either; they are a worklist kept beside
the work, and the test is what keeps them honest. Adding a story on the
React side is therefore a change every port sees on its next test run.

**The reference is measured against, not read.** Beside the registry test,
each platform carries a reference test (`reference_test.dart`,
`ReferenceTests.swift`, `tests/parity.rs`) that folds the probes into
*specimens* — a class and its axes, `dz-button size=small tint=danger
variant=filled` — builds each one with the matching component through a
small adapter (`build()` / `specimens.dart`), lays it out under each theme,
and compares what it can read without a screen: the box, and where the
platform exposes it, the decoration, the type, or the recipe colours the
theme resolves for that variant, tint and state. Only leaf components are
built; a container's size is its story's, and belongs to the screenshot
layer. A width that depends on text is skipped where the test font is not
the browser's, and a value the specimen's appearances disagree on is
skipped as the story's rather than the component's.

**And every side draws.** A value table is as complete as the framework
under it allows, and only one of the three will say what colour a view
painted or what face it set a label in: a SwiftUI `View` is a value rather
than a tree, and GPUI's scene is `pub(crate)` and gone by the time a test
could look at it. So each side also *draws* every specimen it can build, at
the reference's scale, and the viewer puts the pictures side by side with
how much of each differs from React's. The extractor draws one picture per
specimen beside its screenshots; SwiftUI draws with `ImageRenderer` in the
suite; Flutter draws in a **real window** (`pnpm parity:flutter`), because a
widget test resolves a face-less `TextStyle` to a font whose every glyph is
a filled rectangle, and the generated type styles name only a fallback
family on purpose. GPUI cannot draw at all in a headless test, so it has
pictures from no one and its table is what it has.

A specimen whose React element holds other elements rather than one run of
text — a card's heading over a paragraph — is drawn but not counted: an
adapter is given the axes and the text and nothing else, so the difference
between those two pictures is the story's content rather than the
component's drawing.

`apps/parity` (`pnpm parity:view`) is the viewer over all of this: each
platform's reference test also writes `measured.json` beside its drift
list (gitignored — it is an output), and the viewer shows React's value
beside each platform's for every specimen, with the React rendering cut
out of the story's screenshot. Three things the reference cannot state, and so no port is held to. A
**border's width**: Chromium rounds every border to a whole CSS pixel in
both directions, so the pixel and a half a control stroke asks for comes
back as 1, and the two strokes this system deliberately tells apart are one
number in the browser. A **length inside half a pixel**: two text systems do not agree
that finely, and a badge whose line box is 1.4 of an 11pt face is 21.39
here and 21.4 there. Half a pixel *exactly* is held to, because on a 2x
display that is a whole device pixel, and a length the design lands on a
half step — the `0.5` of a `spacing.0-5` — is a decision rather than noise.

`drift.txt` beside the data is the worklist: every check that fails today,
as `<specimen> · <field>`, held both ways so it can only shrink. It is
recorded by the first run that finds it missing, and not committed.
Bringing a component into line means deleting its lines in the same change;
the test prints the value on each side. Never add a line to make a test
pass — an adapter that builds the wrong thing is fixed in the adapter.

```bash
cd packages/ui_flutter/example && flutter test test/parity/
```

```bash
cd packages/ui_gpui && cargo test --test parity -- --nocapture
```

```bash
cd packages/ui_swiftui/Example && xcodebuild -project Playground.xcodeproj -scheme Playground -destination 'platform=macOS' -derivedDataPath .build/xcode test
```

Never hand-edit a distributed `stories.json` or `reference/`; rerun
`pnpm parity`. After a change to a React stylesheet, rerun it and the three
suites: the new lines each one prints are the work the change leaves the
ports.

## Token conventions

Three layers, each referencing the one above it:

- **Base primitives** (`tokens/global/_base.yaml`) — the one place in the
  graph a raw *palette* value exists, all of it hand-authored: eleven colour
  ramps (the three brand ladders, the five semantic hues frozen from
  Tailwind, and the three dark companions that lift the status hues on a
  dark canvas — whole ramps, because a platform that carries a ramp as one
  swatch can only move a ramp whole), the font weights, and the AppKit
  metrics the design is drawn to (`base.corner`, `base.type`,
  `base.leading`, `base.control-height`, `base.font`). One scale per axis —
  there is no second radius or type scale to alias by accident. Components
  must never reference `base.*` directly. The underscore marks the layer,
  not a generator: nothing writes this file but a person. (The semantic
  layer holds raw values of its own kind — each theme's paper, ink and
  shadow are decided there — but never a hue's ladder.)
- **Semantic globals** (`color`, `radius`, `spacing`, `typography`, `effect`,
  and the `control.*` size profiles) — named roles that alias primitives, so a
  theme can re-map a whole ramp by changing one reference. (`spacing` is the
  one exception that holds values directly: its semantic names *are* the grid
  steps, and a base twin under the same names would only be duplication.)
- Themes are **not** a layer or a directory — they are the modes carried
  inline on whichever token varies, and that is forced rather than chosen:
  Terrazzo merges token files by replacing a whole token, so a per-theme file
  adding a mode to `color.surface` would replace the base declaration
  outright, and a file carrying only `$extensions` is dropped with no error.
- **Component tokens** (`tokens/component/*.yaml`) — only for values that are
  genuinely the component's own: a part no shared role covers (`checkbox.box`,
  `switch.thumb`, `dialog.width`) or a `color`+`alpha` wash (`dialog.scrim`).
  A token that would merely forward a shared role (`{control.small.size}`,
  `{radius.medium}`, `{labelSmall}`) must not exist — the component's CSS
  reads the shared variable directly, so most components have no token file
  at all.

The colour system reads in washes and grades:

- **Washes are alphas of the tint, not opaque ramp steps.** A tinted chip is
  the tint at 12%, a plain hover the tint at 8%, a banner's border the tint
  at 25% — so one wash sits correctly on paper, on a card, and on a dark
  canvas without per-theme values.
- **Accent text is a grade darker than accent fill**: content recipes read
  700 where surfaces fill 600, deepening to 800 on hover (300/200 on a dark
  canvas). A label set in the fill colour is the tell of a port.
- **Every surface is a fill an artboard draws, and the ladder is guarded
  at the design's own steps.** Studio's greys are 4a and 4c of the
  BeyondTranslate design, Bright's are 4b and 4e, and the four families with
  no artboard keep Studio's intervals. Three roles come straight off those
  boards and are easy to conflate: `surface-column` is the nav, aside or
  dock — a hair off the paper on 4c, *under* it on 4a and on both Bright
  boards; `surface-muted` is the card, one step off the paper in whichever
  direction the family leans (down in Studio, up to white in Bright); and
  `surface-overlay` is the floating sheet — the paper in a light theme, the
  titlebar's step in a dark one, behind a `.09` edge and a deep shadow. The
  hover wash (`surface-subtle`, what the `normal` recipe hovers to) is an
  alpha of the ink — 6% over paper, 7% over a dark ground — rather than an
  opaque rung, so it moves paper, card and column by the same amount.
  `plugins/contrast.mjs` holds the adjacent rungs to a minimum L\* step read
  off the same boards, so a re-pointed theme cannot collapse two of them or
  trade their places, the way Bright Light's hover once landed on its card.
- **Disabled flattens to the groove** — `surface-sunken` fill,
  `content-faint` text, no border — one dead look for every variant, never a
  translucent ghost of the live one.
- **Selection shifts weight as well as colour.** Rows rest at `labelQuiet`
  (500) and the selected entry takes the label weight (600); a pill tab's
- **Every surface is a fill an artboard draws, and the ladder is guarded
  at the design's own steps.** Studio's greys are 4a and 4c of the
  BeyondTranslate design, Bright's are 4b and 4e, and the four families with
  no artboard keep Studio's intervals. Three roles come straight off those
  boards and are easy to conflate: `surface-column` is the nav, aside or
  dock — a hair off the paper on 4c, *under* it on 4a and on both Bright
  boards; `surface-muted` is the card, one step off the paper in whichever
  direction the family leans (down in Studio, up to white in Bright); and
  `surface-overlay` is the floating sheet — the paper in a light theme, the
  titlebar's step in a dark one, behind a `.09` edge and a deep shadow. The
  hover wash (`surface-subtle`, what the `normal` recipe hovers to) is an
  alpha of the ink — 6% over paper, 7% over a dark ground — rather than an
  opaque rung, so it moves paper, card and column by the same amount.
  `plugins/contrast.mjs` holds the adjacent rungs to a minimum L\* step read
  off the same boards, so a re-pointed theme cannot collapse two of them or
  trade their places, the way Bright Light's hover once landed on its card.
  active label and an option card's title go to `labelStrong` (700, display
  cut). `stroke.control` (1.5px) draws control outlines; `stroke.hairline`
  (1px) draws separators, on every platform and at every display scale — it
  used to halve on Retina to land on one device pixel, and against the
  mockups the system is drawn to a halved rule read as too thin to be a
  decision. `ui_react`'s `src/styles/base.css` carries the box-sizing and
  reduced-motion ground rules.
- **A separator is ink at 13% over paper, 16% over a dark ground** —
  `color.border`, the same two numbers in every family, since the ink each
  one substitutes already carries the difference. `border-strong` keeps its
  rung above (17% / 20%) so a popover's edge still separates from what it
  floats over, which lands it beside the opaque `border-muted` a checkbox or
  a radio is outlined in rather than under it.
- **The focus ring is the system accent, always**: `--focus-ring` on
  `:root` — 3px of primary at a per-theme alpha, offset 0, hugging the
  control the way macOS draws it. It never follows `data-tint`.

Four things that surprise people:

- Interactive colors are stored as *recipes*, not finished colors: a `shade` to
  pick off a ramp plus an `alpha` to apply. Each platform resolves them against
  whichever ramp the control is tinted with.
- The recipe table writes only its decisions. `control.color` is six variants
  by three parts by four states, and most of those cells are one rule
  applied, so a variant writes what it decides and `scripts/recipes.mjs`
  fills in the rest before the graph is parsed: `hovered` unwritten is
  `normal`, `pressed` unwritten is `hovered` dimmed by `$defaults.pressed`,
  `disabled` unwritten is `$defaults.disabled` for that part, a part left
  out entirely is `$defaults.undrawn` in every state, and a shade with no
  alpha is opaque. The names the rules fill in with live under
  `control.color.$defaults`, consumed by the expansion and never a token.
  Every plugin reads the full table, so no platform generator carries a
  rule — and a cell that is only the rule applied must not be written, since
  writing it hides which cells are decisions. A misspelt part or state
  fails the build rather than falling through to a default.
- A ramp is a **role ladder, not a lightness ramp**. Read `base.color.ink`
  before assuming otherwise: its 700 is *lighter* than its 600, because
  Bright's primary action is ink navy whose hover lifts, and its 50–200 are
  acid-tinted washes rather than pale navy. A step is only ever reached by a
  recipe, so what a step means is the role that recipe gives it. This is what
  lets twelve themes this different share one recipe table untouched — and why a
  shade written into a stylesheet would be a lie rather than merely a
  duplicate.
- A recipe can re-pick its step per theme (`shade` carries a mode), because a
  fixed step cannot be right in both directions: `tinted` wants the palest
  step of the ramp on paper and the darkest on a near-black canvas.
  `plugins/control.mjs` resolves those into a block per theme, since the shade
  is baked into the emitted value and nothing downstream could re-pick it.

### Themes

Twelve, as six families over one default, defined once in `terrazzo.config.mjs`
and passed to all three plugins so the platforms cannot drift:

| `data-theme`     | modes applied                     |
| ---------------- | --------------------------------- |
| *(none)*         | — Studio Light, `:root`           |
| `studio-dark`    | `dark`                            |
| `bright-light`   | `bright`                          |
| `bright-dark`    | `bright`, `dark`, `bright-dark`   |
| `frost-light`    | `frost`                           |
| `frost-dark`     | `frost`, `dark`                   |
| `graphite-light` | `graphite`                        |
| `graphite-dark`  | `graphite`, `dark`, `graphite-dark` |
| `ember-light`    | `ember`                           |
| `ember-dark`     | `ember`, `dark`, `ember-dark`     |
| `nocturne-light` | `nocturne`                        |
| `nocturne-dark`  | `nocturne`, `dark`, `nocturne-dark` |

The modes compose rather than repeat, so a value shared along the light/dark
axis or along a family is written once. A family whose accent reads correctly
in both directions needs no dark corner at all — Frost is the one that does
not have one, because a teal is a teal on either canvas, where Bright,
| `nocturne-light` | `nocturne`                        |
| `nocturne-dark`  | `nocturne`, `dark`, `nocturne-dark` |
Graphite and Ember each invert or lift their ramp and so carry a second
ladder. Nocturne's blurple is written once the same way, and its dark corner
carries only the surfaces and ink of the design system it is read off — the
Tiller mockups' own ground, `#161826` under `#e9e9ed`. The order in that config is load bearing: every selector is a single
attribute and so has equal specificity, and the later block wins — the family
modes first, then `dark`, then the corners.

Graphite and Nocturne are the two families that move geometry: `radius.small`
through `radius.big` and both `frame` corners step down one rung of the same
ladder. Graphite does it to draw a crisper monochrome corner; Nocturne because
the mockups round a row at 7 and a card at 8, which is exactly where that rung
lands.

Studio Light sets **no attribute at all** — selecting it means removing
`data-theme`, not writing `studio-light`. Writing it would leave the page
looking right for the wrong reason, since none of the theme blocks would
match. The React playground does exactly this; see `packages/ui_react/example/src/lib/themes.ts`.

Flutter has no cascade, so `plugins/flutter.mjs` collapses each theme's mode
chain into a whole `ThemeVariables` instance — `themeVariablesStudioDark`,
`themeVariablesBrightLight`, `themeVariablesBrightDark` — beside the default
`themeVariables`. It reads the themes back out of the same `modeSelectors`,
so adding a theme reaches Dart without the plugin being touched. Swift and
Rust do the same thing in their own idiom: an initialiser with defaults and
computed properties there, a `ThemeSeed` with `..STUDIO_LIGHT_SEED` and a
`const fn` here.

The Dart class also carries a generated `copyWith` over every stored field,
and each type *face* is one of those fields — `fontUi`, `fontDisplay`, built
from `base.font.*` and read by every style set in them. A design drawn to the
platform's own UI face is `fontFamily: null`, which is right on a Mac and
nothing at all on Windows or in a `flutter test`, so a host has to be able to
say what the kit is set in without re-typing a hundred-parameter constructor.
The components meet it halfway: every ambient style they set is a
`DefaultTextStyle.merge`, never a replacement, so a family set above them
reaches a `Badge`'s label as well as a `Text`'s.

A recipe that names a semantic role in any state (every variant's
`disabled` does, through `$defaults`) is a computed member on those three
platforms, since it reads roles of the same instance. A *shade* on it that a
theme re-picks cannot be baked into a computed member, so each such shade is
a stored member of its own — `controlColorTintedContentNormalShade`,
`control_color_filled_surface_normal_shade` — which the theme instance
re-points and the computed descriptor reads. Only shades that carry a mode
get one; the rest stay literal. This is what the parity tests caught first:
every dark-theme control colour on Swift and GPUI was one step off.

## Component conventions

- **Behaviour is Base UI's, drawing is ours.** A component that has state a
  reader can get wrong — a popup that has to close, a group the arrow keys
  move through, a control that has to reach a form — is a thin skin over the
  matching Base UI part rather than a re-implementation. Reach for a plain
  element only where there is genuinely nothing to get right (Badge, Card,
  Divider's tone). What this buys is not shorter files: it is the focus trap,
  the type-ahead, the `aria-*` wiring and the collision handling that a
  hand-rolled version never quite finishes.
- **Where Base UI has no part, the behaviour is written out and the reason is
  written down.** Three components own their own: `Tree`, whose keyboard is
  not a list's — `ArrowRight` opens a shut branch and descends into an open
  one, `ArrowLeft` shuts one and climbs out of a leaf, and the whole tree is
  one tab stop, none of which falls out of a `Collapsible` per node;
  `Splitter`, whose seams are a pointer gesture with an `aria-valuenow`
  rather than a state machine; and `Skeleton`, which has no behaviour at all.
  Writing one of these is the exception, not a precedent: check
  [base-ui.com](https://base-ui.com) first, and if a part exists, wear it.
- **State comes off Base UI's attributes, not the DOM's pseudo-classes.**
  A checkbox root is a focusable `span` over a hidden input, so `:checked` and
  `:disabled` never match — the CSS reads `[data-checked]` and
  `[data-disabled]`. A popup row reads `[data-highlighted]` rather than
  `:hover`, which is what makes the keyboard and the pointer land on the same
  state. `[data-*]` was already the convention for variants; this is the same
  rule reaching the states.
- One folder-level file pair per component: `src/components/<name>.tsx` and
  `<name>.css`, both listed in their registries (`components/index.ts`,
  `styles/components.css`).
- **A family is named after its root, and the root is a component.** `Dialog`
  holds `DialogHeader`; `Table` holds `TableRow`; `Preferences` — the settings
  column — holds `PreferenceGroup`, `PreferenceSection` and `PreferenceRow`.
  The file, the stylesheet, the story page and the port's file all take the
  root's name, so a name that names nothing (`Preference`, when only its parts
  existed) is the tell that a root is missing rather than a family that has
  none. What the root owns is whatever the parts cannot — the run between
  groups, the column's width — which is usually exactly what the stories and
  the examples were each writing for themselves.
- Components are styled by plain CSS reading the token custom properties —
  there is no Tailwind here. A value a component needs is a shared variable
  read directly (`--spacing-4`, `--radius-medium`, `--label-small-font-size`,
  `--control-small-padding-block`); only a value no shared role covers becomes
  a token in `tokens/component/<name>.yaml`. Never a literal in the CSS.
- A value the design lands between two steps of a scale is composed out of
  them rather than written down: the 18px a dialog body and a preference group
  both pad by is `calc(var(--spacing-4) + var(--spacing-0-5))`. That keeps the
  scale the only source of lengths, and it keeps the value honest — a step and
  a half says what it is, where `18px` only says what it measures.
- **A class on an icon is one class too few for `display`.** A Fluent icon
  brings its own sheet — `display: inline`, `line-height: 0`, the RTL flip —
  and appends it to the head as the first icon renders, which is after every
  stylesheet linked here, so it takes a one-class tie. A rule that sets one of
  those on the icon itself names the parent as well, the way `checkbox.css`
  hides its dash (`.dz-checkbox .dz-checkbox__dash`); every other part puts
  the glyph in a span of its own — `.dz-checkbox__mark`, `.dz-select__chevron`
  — and styles that.
- Sized controls read the `[data-size]` profile bindings — `--control-size`,
  `--control-gap`, `--control-padding-*`, `--control-radius`,
  `--control-font-*`, `--control-line-height` — once in their base rule
  instead of restating geometry per size (Button and IconButton are the
  model — neither keeps a per-size block at all, since the profile carries the
  design's own control type). A deviation from the profile keeps its size
  block and overrides the bound variable with another token's variable.
  Non-controls with sizes (Badge, KeyCap, Card) skip the bindings and
  read the profile steps and typography faces directly per size block —
  `badge.css` is that model.
- Variants are `data-*` attributes, not classes, so the CSS selector names the
  axis (`[data-variant='tinted']`) and consumers can see the state in the DOM.
- **The floating sheet is one drawing, in `popup.css`.** A Menu's panel, a
  Select's list and a Tooltip all wear it, and `menu.css` and the select rules
  in `text-field.css` add only what is genuinely theirs — which check
  attribute reveals the tick column. Gaps between a popup and its trigger are
  padding on the positioner rather than an offset passed in code, so the gap
  is a token and the collision logic counts it when the panel flips.
- CSS cannot index a ramp by a variable, so the shade/alpha recipes are
  resolved in the token build instead — `plugins/control.mjs` emits both the
  `[data-tint]` ramp bindings (`--tint-50` … `--tint-950`) and a finished color
  per variant/part/state (`--control-color-filled-surface-hovered`). Component
  CSS names a state, never a shade; a shade written into a stylesheet would
  drift the moment a recipe changed.
- Both halves land on `[data-tint]`, not `:root`, and that is load bearing: a
  custom property holding `var()` is substituted where it is *declared*, so
  `:root { --x: var(--tint-600) }` would resolve against a root with no ramp
  bound and every descendant would inherit the dead value. **Any component
  reading `--control-color-*` must therefore carry `data-tint`.**
- An interactive control opts into the shared state machine with `data-control`
  plus `data-variant`, then reads `--control-surface` / `--control-content` /
  `--control-border` and gets hover, press, and disabled for free. Button and
  IconButton are the model. A component that is *not* interactive (Badge, Card)
  must stay out of it and read the `-normal` recipes directly — `:hover` would
  fire on it just the same, and a label that changes under the pointer reads as
  something you can press.
- **One drawing for "on".** A selection is a tinted wash and a heavier label,
  everywhere it appears — a sidebar row, a tree row, a pill tab, a `Tag`, a
  `Toggle`. So a glyph button that stays down is a `Toggle` and not an
  `IconButton`: that component acts and returns, and a second on-state living
  there would be a second drawing for one state. A `Toggle` whose whole label
  is a lone glyph is square at the control height, which is what lets the two
  sit in one toolbar without a seam. A trigger that opens something is neither
  — a disclosure is `aria-expanded` and a dialog trigger is the dialog.
- Pick the recipe by what the part *is*, not by which colour looks right.
  `normal` is the neutral control — its hover wash is `surface-muted`, which is
  what a menu item, a table row, or a preference row wants. `plain` is the
  quiet *accent* control, so its wash comes off the tinted ramp and would sit
  heavier on a row than a `tinted` selection one step below it.
- `tint` is a prop only where a component genuinely re-tints. Components whose
  colors are already in the token graph (KeyCap, Progress's track) read those
  directly.
- A `color` paired with an `alpha` anywhere else in the graph (`dialog.scrim`)
  is resolved by the same plugin onto `:root`, so no component has to apply an
  alpha in CSS.

## GPUI conventions

The Rust port follows the component conventions above; what is worth knowing
is where the platform forced a different shape.

- A component is a `RenderOnce` builder struct — `Button::new(id, label)` then
  `.variant()`, `.tint()`, `.on_click()` — and reads the theme through
  `cx.theme()`. It holds no state, so anything it has to remember between
  frames goes in GPUI's element state through `pressable` or `latched` rather
  than into a host's view.
- **`pressable` is the state machine.** GPUI's own `hover()` and `active()`
  refinements resolve a phase after a text run's colour is baked, so a recipe
  whose content darkens under the pointer would move its surface and leave its
  label behind. `pressable` decides the state before the subtree is built,
  which is why every interactive component is wrapped in one.
- Colour and geometry come off `cx.theme().vars` — never a literal. Where the
  React stylesheet itself holds one (a spinner's 25% track, a dangerous
  dialog's 34% edge), it is a named `const` with the same reason beside it.
- `src/components/mod.rs` is the registry, the way `styles/components.css` is
  on the React side: a component ships once it is listed there.
- Tests are the substitute for a Storybook. `tests/render.rs` draws every
  component in every theme headlessly, measures the control heights
  against the size profile, asserts on the recipes, and simulates the pointer
  and the keyboard — which is the part of a screenshot that can be checked
  without a screen. It is also what catches a wiring bug: a key listener on a
  descendant of the focused element never fires, and only a simulated
  backspace says so.

## Playground conventions

- Stories live in `packages/ui_react/example/stories/*.stories.tsx`, titled
  `Tokens/<Group>` or `Components/<Name>`. `tools/playgrounds/catalog.mjs`
  reads declarations in source order, sorts pages, and generates the catalog.
  Export names determine stable IDs independently of display-name overrides.
- The React example is a Vite application, with local `Meta` and `StoryObj`
  types; it has no Storybook runtime or dependency. Existing explicit render
  functions and decorators remain supported during scene migration.
- Portable story content lives beside the declaration in `*.scene.json` and
  is consumed through `defineStory`. Run `pnpm playgrounds:generate` after
  changing declarations or scenes; `pnpm playgrounds:check` detects stale
  output. Generation needs Dart and rustfmt to format native output.
- All story content, fixtures, state and interactions must be expressed by shared
  declarations. Platform adapters contain framework plumbing and component mappings
  only. Native story-specific implementations are temporary migration debt.
  `pnpm playgrounds:complete` is the all-story/all-platform acceptance gate.
- Never hand-edit playground generated files. Their ownership and hashes are
  recorded in `tools/playgrounds/outputs.json`. Source changes belong in the
  React scene declarations or the platform adapters. `coverage.json` records
  pending scene/adapter migrations; a legacy native implementation is not
  generated content. See `packages/ui_react/example/README.md` for current scope.
- The isolated canvas exposes `window.__PLAYGROUND__` for automation. Stable
  `story`, `theme`, `canvas` query parameters replace Storybook's iframe and
  private channel. Golden baselines require explicit review/update; React
  measurement parity remains independent of those per-platform baselines.
- **One documented page per component, and it is `Overview`.** It is the only
  story that renders a `Page`, and it carries the whole write-up: what the
  component is, the axes it varies along, and the token table where the
  component owns tokens. It overrides `parameters: { layout: 'fullscreen' }`,
  because the `Page` brings its own padding.
- **Every other story is the component and nothing else** — no `Page`, no
  `Section`, no `Specimen`, no prose, no headings. A gallery of variants is not
  a place to read; it is a place to look at the thing, and a reader who wants
  the argument opens `Overview`. Give a component as many of these as it has
  states worth seeing on their own (`Filled`, `Disabled`, `Sizes`, `Tints`),
  then the compositions that show where it lands (`InAForm`, `AToolbar`,
  `InSettings`).
- `meta.parameters.layout` is `'centered'` on component files, so a bare
  specimen sits in the middle of the canvas rather than in its corner. A story
  whose subject is fixed to the viewport — a scrim, a toast viewport — sets
  `'fullscreen'` back on itself.
- The two component directories are not interchangeable.
  `src/components/gallery.tsx` is **documentation** presentation — `Page`,
  `Section`, `Specimen`, `TokenTable`, the swatches and ramps — and belongs to
  `Overview` and the `Tokens/*` pages only. `src/components/stage.tsx` is what
  bare stories use: `Row`, `Col`, `Frame`, `Pane`, which arrange instances and
  give a component the width or the window it needs, and draw nothing
  themselves.
- The token table appears **only** where the component owns tokens — the
  thirteen files in `packages/ds/tokens/component/`, plus the `frame` metrics
  the shell columns are sized by. Most components own none: they read the shared
  vocabulary directly, which is documented on `Tokens/Control`, and a table
  that would come back empty is not worth a section.
- The gallery reads the **built** `dist/style.css`, not the token sources. It
  parses the `:root` block for each alias and resolves the `var()` chain via
  `getComputedStyle`, so every token shows both what it points at and what it
  evaluates to.
- Because of that, adding or renaming tokens usually needs no story changes —
  the existing pages pick them up. Reach for `byPrefix` / `groupScales` /
  `componentTokens` in `src/lib/tokens.ts` before hardcoding token names.
- Shared presentation lives in `src/components/gallery.tsx`. The gallery styles
  itself with DazzUI tokens; keep it that way.

**The platform playgrounds are organised the same way**, in their own dialect:
`packages/ui_swiftui/Example`, `packages/ui_flutter/example` and
`packages/ui_gpui/example` each carry a `Storybook/` (`storybook/`) framework —
a `Story`, a `StoryPage`, a `StoryGroup`, a registry, the sidebar tree, and the
same split between documentation furniture (`Gallery`) and bare-story
arrangement (`Stage`) — and one story file per component beside it. A page
ships once it is named in the registry. The GPUI index is a `const`, because a
story there is a plain `fn` pointer with nothing to capture.

Two of them are also where the coverage is: the Flutter example's widget test
pumps every story in every theme, and the GPUI example's `cargo test` draws
every story in every theme headlessly. Both are what catch a story nobody has
opened lately — an overflow, a bad length, a lost element state.

## Example conventions

`examples/*` are apps, not packages: private, never published, and each one
built and served on its own port. An example is added to `pnpm-workspace.yaml`
by the `examples/*` glob and given a root script beside the playground's.

The rules an example does keep are the ones that make it worth having:

- **Not a literal in the stylesheets.** Every colour, radius, gap, face and
  height is a token read from `@dazzlabs/dazzui-react/style.css`. An example
  that reached for a hex would be proving the opposite of its point. A layout
  value with no token behind it — a breakpoint, a preview's maximum height — is
  derived from one (`calc(var(--spacing-20) * 3)`) or stated once as a named
  constant with the reason beside it.
- **Interactive colour comes off the recipes.** A part that is tinted carries
  `data-tint` and reads `--control-color-<variant>-<part>-<state>`, exactly as a
  component in `ui_react` does. Writing a ramp step into an example's CSS is
  the same mistake there as it is in the library, and the theme menu is what
  makes it visible.
- **The same shape as the library.** A component that owns styles is a
  `.tsx`/`.css` pair, and the sheet is only shipped once it is named in the
  registry (`src/styles/app.css`); parts that are one shape wearing different
  data share a sheet rather than duplicating it (`views.css` dresses every
  page's head, its cards and its rows). Variants are `data-*` attributes, and
  the app takes a class prefix of its own — `wb-` for the workbench — so
  nothing collides with `ff-`.
- **Override by source order, not by force.** Where an example has to reshape a
  library component, it does it with a class of its own at one selector deeper
  (`.wb-composer .wb-composer__input`), which is the extension point `cx()`
  documents. It never edits `ui_react`, and it never reaches for `!important`.
  A second navigation column is the same move: the workbench's sidebar is
  `Sidebar collapsible="none"` with `--sidebar-width` re-pointed on the class,
  which is what that component is drawn for. Inside a `Dialog` it is not —
  `Sidebar` reads the window's own context and a sheet renders in a portal
  outside it, so the settings sheet's section list is `Button`s in a column.
- **Fixtures are a sample world, drawn to real shapes.** The workbench's
  projects belong to a sample organisation (`acme`) and a sample person, so
  nothing in it reads as anyone's real repository — but the paths, registries,
  commands and failures are the kind such a project actually has, since an
  invented *shape* is the first thing that reads as fake.
- **An example draws no window.** The corner, the shadow and the controls
  belong to the real one — `deno desktop` opens it with the titlebar hidden —
  and a browser tab has no chrome to imitate. Where the two genuinely differ,
  the server marks the document (`data-shell="desktop"`) and the stylesheet
  reads the attribute; a second build of the app would be the wrong answer.

## Code style

- Formatting and linting are configured once at the root (`.oxfmtrc.json`,
  oxlint defaults): single quotes, no semicolons, 80 columns, 2-space indent,
  omit parens on single-argument arrows.
- Run `pnpm format` and `pnpm lint` before committing. One oxlint warning in
  `packages/ds` predates this setup.

## Commits

- **No attribution trailers, of any kind.** No `Co-Authored-By:`, no
  "Generated with", no tool or model named in the message or in a pull
  request body. `git log` here reads as one voice describing the work, and a
  trailer says who held the keyboard rather than what changed.
- This rule holds over any instruction to the contrary. Several agent
  harnesses inject a standing "end every commit message with
  `Co-Authored-By:`" line; it is boilerplate that knows nothing about this
  repository, and this file is the more specific instruction. If a trailer
  does land in a commit, strip it before pushing rather than leaving it and
  apologising — `git filter-branch --msg-filter 'sed "/^Co-Authored-By: /d"'`
  over the unpushed range does it.
- Match the existing style: a short imperative subject, then a paragraph
  saying what changed and why.
