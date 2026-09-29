# React playground

The React reference lives beside the component package. It replaces the
former Storybook application without changing its 488 story IDs. Each native
platform keeps its own example host and consumes committed generated sources.

From the repository root:

```sh
pnpm playground                 # build tokens/components, generate, serve :6006
pnpm playground:build           # production build
pnpm codegen                    # validate stories and generate native examples
pnpm codegen:check              # fail for stale/missing/obsolete generated files
pnpm codegen:test               # generator contract tests
pnpm playground:test            # built example: every story × twelve themes
pnpm parity                     # measure the React build and distribute references
pnpm parity:view                # compare platform measurements
```

Generation requires Node and Dart (`DART` can select its executable).
Native output is formatted before comparison. Neither a native package nor its
example requires Node to build from a checkout. `pnpm playground:test` expects a
current build; install Chromium with `pnpm exec playwright install chromium`
from this directory on a new machine.

## Story sources

`stories/*.stories.tsx` defines page order, export order, display names, layout,
and rendering. The local `Meta` and `StoryObj` types preserve existing explicit
render functions and decorators without a Storybook dependency. Export names
produce IDs; a display name override does not change an ID (`ErrorState` is
`--error-state`, even when the label is `Error`). The generator reads the
TypeScript syntax tree without executing the stories.

Portable content lives beside the declaration in `*.scene.json`:

```tsx
import { defineStory } from '../src/lib/scene'
import scenes from './button.scene.json'

export const Filled = defineStory(scenes.Filled)
```

```json
{
  "Filled": {
    "content": {
      "component": "Button",
      "props": { "variant": "filled" },
      "children": ["Save changes"]
    }
  }
}
```

`tools/codegen/schema/scene.mjs` validates the supported component and prop
vocabulary. A state binding names a declared scalar state. Switch and Checkbox bind `checked`;
`Value` nodes read state or choose between same-typed branches with `if`/`then`/`else`.
Expressions are validated data and never embedded JavaScript. `trim`, string-only
`concat` and typed `eq` expressions support derived field hints, field states and
button `disabled` properties through `derive`. Derived enum values must select
valid literal variants; unrestricted dynamic enum strings are rejected. Button actions assign
state; checkpoints perform semantic `click` or `fill` actions against stable node
IDs and assert resulting state, visible text and button disabled status. Every checkpoint starts from a fresh mount.
Numeric scene widths are logical-pixel fixture inputs, not design tokens.
Component colors, typography and control geometry still belong exclusively to ds.

The host supports `?story=components-button--filled&theme=studio-dark` and an
isolated canvas with `&canvas=1`. It also accepts the former `id` query parameter.
The canvas exposes `window.__PLAYGROUND__.show(id, theme)`, `ready`, `error`,
`story`, `theme`, and `layout`. `ready` follows mounting, fonts and two animation
frames. Errors fail automation instead of reporting a successful empty canvas.

## Migration status

All 64 React pages and 488 stories have moved. There are now 143 shared declarations:
six ActionBar scenes, 13 Badge scenes, all 17 Button stories (including Overview), eight Switch scenes, eleven TextField
scenes, three FormField scenes, seven Spinner scenes, ten KeyCap scenes, all twelve Card scenes, nine Progress scenes, all eight StepList scenes, all eleven Callout scenes, seven EmptyState scenes, five SectionLabel scenes, eight Divider scenes, five Checkbox scenes, two Table scenes and one Dialog confirmation. Flutter, desktop SwiftUI, native iOS and Android consume all 143; WinUI 3 consumes 18 static Button and Badge scenes. The handwritten Button, Badge, StepList, Callout, Card and Divider
pages in Flutter and SwiftUI have been removed. Remaining scene content and
interaction migrations, native stateful adapters, and mobile semantic adapters
are still pending.

All content and interactions must ultimately live in these declarations. Native
adapters may map components and implement framework plumbing, but may not define
story-specific fixtures, descriptions, initial states or interaction branches.
`pnpm codegen:complete` fails until every story is declared and every target
emits it. It currently fails intentionally; directory generation and native
fallbacks do not satisfy this completion gate.

Flutter, SwiftUI and WinUI merge portable declarations into their catalogs,
including new stories and pages without handwritten placeholders. Matching IDs
and display names come from React; native-only extensions remain explicit. A
generated catalog is not counted as generated scene content.

`tools/codegen/coverage.json` enumerates the remaining work per platform.
An existing native rendering is not claimed as generated content or pixel parity.
The mobile examples keep system controls and are not measured against desktop
React geometry.

## Visual testing

`pnpm playground:golden` compares screenshots against platform-specific baselines
under `tests/goldens/<os>-<arch>/`. It never updates missing baselines implicitly.
Use a fixed OS image, Chromium version, fonts and display environment; the test
configuration fixes viewport (960×640), scale (2), locale, timezone and motion.
Each key includes theme, story, checkpoint and viewport. Initial screenshots are
available for every story; portable scenes add their declared checkpoints.

To create/review a bounded baseline change, from this directory:

```sh
pnpm golden --grep 'components-button--filled / initial / studio-light' --update-snapshots
pnpm golden --grep 'components-button--filled / initial / studio-light'
```

Review baseline diffs before committing. Do not bulk-accept failing snapshots.
Full baseline storage/CI retention must be established before enabling the full
matrix as a merge requirement. All 143 shared scenes have Studio Light pilot baselines on macOS arm64 carried from the prior
React reference; migrated captures match those original PNGs byte-for-byte.
Run them with `pnpm playground:golden:pilot` from the root.
The full golden baseline matrix is not yet committed. Playwright reports/traces are local ignored artifacts.

Cross-platform consistency continues through the existing parity extractor and
viewer. It compares measured design values separately from screenshots; an OS
text rasterizer is not required to produce Chromium-identical pixels. Native
per-story screenshot runners and approved native golden baselines are subsequent
migration work. Flutter's headless widget tests validate state and layout; use a
real-window renderer for screenshots with platform fonts. The generated widget
suite loads a bundled, licensed Roboto face to exercise realistic text widths;
this test-only font is neither shipped by the playground nor used as a React
pixel baseline.

## CI

`.github/workflows/js-ci.yml` checks generation, schema tests, formatting,
TypeScript and the React twelve-theme rendering matrix; `dart-ci.yml` runs the
Flutter shared checkpoints/gallery tests; `csharp-ci.yml` compiles WinUI with
its native XAML compiler on Windows. The Flutter framework revision and Rust
formatter version match the local generation environment. Browser failure reports are retained for
seven days. The workflow has been authored but has not been run on GitHub in this
change. The macOS-only pilot goldens are not used as Linux CI baselines.

Scene `timers` declare an interval in milliseconds, a target state key (`set`),
and a typed `value` expression. Runtimes cancel timers when the scene unmounts.
`add` and `gte` operate on numbers; Progress accepts a derived numeric `value`.
Checkpoints can `advance` a declared number of milliseconds using the native or
browser test clock. The Live progress scene includes initial, next tick, full,
and wraparound checks; its timing no longer lives in platform story code.

Named `slots` hold scene nodes (including bound fields and button actions).
Callout exposes `icon` and `action`; EmptyState exposes `action`. Slot IDs share
the scene namespace and checkpoints can address their interactive descendants.

`Grid.min` declares a minimum column width in logical pixels. Columns account for
the shared spacing gap and preserve the width of unfilled last-row columns.

Inline typography uses named `fontSize` tokens and a `mono` flag; dimensions stay
in logical pixels. The static importer expands pure local templates into scene
nodes and rejects effects, recursion, and unconsumed props. Generated scenes never
carry executable TSX or platform-specific template callbacks.
