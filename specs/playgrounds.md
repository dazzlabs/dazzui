# Playgrounds

How stories are declared once and generated into every playground.

- Stories live in `packages/js/dazzui-react/example/stories/*.stories.tsx`, titled
  `Tokens/<Group>` or `Components/<Name>`. `tools/codegen/catalog.mjs`
  reads declarations in source order, sorts pages, and generates the catalog.
  Export names determine stable IDs independently of display-name overrides.
- The React example is a Vite application, with local `Meta` and `StoryObj`
  types; it has no Storybook runtime or dependency. Existing explicit render
  functions and decorators remain supported during scene migration.
- Portable story content lives beside the declaration in `*.scene.json` and
  is consumed through `defineStory`. Run `pnpm codegen` after
  changing declarations or scenes; `pnpm codegen:check` detects stale
  output. Generation needs Dart to format native output.
- All story content, fixtures, state and interactions must be expressed by shared
  declarations. Platform adapters contain framework plumbing and component mappings
  only. Native story-specific implementations are temporary migration debt.
  `pnpm codegen:complete` is the all-story/all-platform acceptance gate.
- Never hand-edit playground generated files. Their ownership and hashes are
  recorded in `tools/codegen/outputs.json`. Source changes belong in the
  React scene declarations or the platform adapters. `coverage.json` records
  pending scene/adapter migrations; a legacy native implementation is not
  generated content. See `packages/js/dazzui-react/example/README.md` for current scope.
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
  thirteen files in `ds/tokens/component/`, plus the `frame` metrics
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
`packages/swift/DazzUI/Example` and `packages/dart/dazzui/example` each carry a
`Storybook/` (`storybook/`) framework —
a `Story`, a `StoryPage`, a `StoryGroup`, a registry, the sidebar tree, and the
same split between documentation furniture (`Gallery`) and bare-story
arrangement (`Stage`) — and one story file per component beside it. A page
ships once it is named in the registry.

The Flutter one is also where the coverage is: its widget test pumps every
story in every theme, which is what catches a story nobody has opened lately —
an overflow, a bad length, a lost element state.
