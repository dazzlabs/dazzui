# Parity

How the ports are held to React: the extracted reference, the registry and
reference tests, and the drift lists.

`packages/js/dazzui-react` is the standard the other platforms are held to, and the
standard is written in stylesheets — which a port reads with its eyes. The
parity pipeline reads it with a browser instead, so that what a port is held
to is a number rather than an impression.

1. `packages/js/dazzui-react/example/scripts/parity.mjs` serves the React playground build and opens
   every story it lists, in every theme, in a headless Chromium at a fixed
   window (960×640 at 2x, reduced motion). For each it records every
   component root on the page — anything carrying a bare `ff-*` class — with
   its `data-*` axes, its box, and the computed border, corner, colour, type
   and shadow; and takes a screenshot. It stages three things under
   `packages/js/dazzui-react/example/parity/` (gitignored, the way `dist/` is for tokens):
   - `stories.json` — the story list, and the cross-platform key
   - `reference/<page>.json` — what every story rendered, per theme
   - `screenshots/<theme>/<id>.png` — for the side-by-side report; never
     distributed, since a screenshot is only reproducible on one machine
2. It then distributes `stories.json` and `reference/` into each platform's
   parity directory — `packages/dart/dazzui/example/test/parity/` and
   `packages/swift/DazzUI/Example/Tests/PlaygroundTests/Parity/` — so a
   platform's tests run from inside
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
under it allows, and not every one of them will say what colour a view
painted or what face it set a label in: a SwiftUI `View` is a value rather
than a tree. So each side also *draws* every specimen it can build, at
the reference's scale, and the viewer puts the pictures side by side with
how much of each differs from React's. The extractor draws one picture per
specimen beside its screenshots; SwiftUI draws with `ImageRenderer` in the
suite; Flutter draws in a **real window** (`pnpm parity:flutter`), because a
widget test resolves a face-less `TextStyle` to a font whose every glyph is
a filled rectangle, and the generated type styles name only a fallback
family on purpose.

A specimen whose React element holds other elements rather than one run of
text — a card's heading over a paragraph — is drawn but not counted: an
adapter is given the axes and the text and nothing else, so the difference
between those two pictures is the story's content rather than the
component's drawing.

`tools/parity` (`pnpm parity:view`) is the viewer over all of this: each
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
cd packages/dart/dazzui/example && flutter test test/parity/
```

```bash
cd packages/swift/DazzUI/Example && xcodebuild -project Playground.xcodeproj -scheme Playground -destination 'platform=macOS' -derivedDataPath .build/xcode test
```

Never hand-edit a distributed `stories.json` or `reference/`; rerun
`pnpm parity`. After a change to a React stylesheet, rerun it and the three
suites: the new lines each one prints are the work the change leaves the
ports.
