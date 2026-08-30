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
  builds and ships on its own. Further platform packages sit beside it as they
  land (`ui_swiftui`), which is why the directory carries the platform in its
  name.
- `packages/ui_flutter` (`dazzui`) — the Flutter implementation,
  migrated from `dazzui-flutter`. A plain Dart/Flutter package, invisible to pnpm:
  run `flutter analyze` / `flutter test` from inside the package. Like the
  React package, `lib/src/generated/` holds what `ds` wrote and is committed.
  The ds vocabulary is the API: the color axis is `tint` (`WidgetTint`,
  `ButtonTint.neutral`), never dazzui-flutter's `kind`/`secondary`. `example/` holds
  the playground app (`flutter run` from that directory, macOS and web).
- `apps/storybook` (`@dazzlabs/dazzui-storybook`) — a Storybook gallery that
  documents both: `Tokens/*` and `Base/*` for the token graph, `Components/*`
  for the components.

The split is the point: anything a platform package needs but cannot express
belongs in `packages/ds`, so the platforms stay implementations of one design
rather than three designs that resemble each other.

Every domain-free component from `beyondtranslate/packages/ui` has been
migrated. What is left there is the desktop shell — WindowFrame, Sidebar,
MiniWindow, BrowserFrame and friends — which is still pending a decision, and
the mobile device frames, which are presentation props rather than system
parts.

## Commands

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
pnpm storybook         # build both, then serve Storybook on :6006
```

```bash
pnpm typecheck         # tsc over every workspace package
```

```bash
pnpm lint              # oxlint over the workspace
```

```bash
pnpm format            # oxfmt over the workspace
```

## Token pipeline

1. `packages/ds/tokens/global/_base.yaml` holds the primitive layer and is
   **generated** — `pnpm --filter @dazzlabs/dazzui-ds sync` downloads
   Tailwind's `theme.css` and rewrites it. Never hand-edit that file.
2. `tz build` reads the files listed in `terrazzo.config.mjs` and stages
   `packages/ds/dist/`:
   - `style.css` — CSS custom properties (the contract consumers use)
   - `control.css` — the ramp bindings and the resolved interactive color
     recipes, from `plugins/control.mjs`
   - `flutter/colors.dart` + `flutter/theme_variables.dart` — the Dart token
     sources, from `plugins/flutter.mjs`
3. `distribute()` in `scripts/build.mjs` then copies each platform's files out
   of that staging directory into the platform package itself — the two
   stylesheets into `packages/ui_react/src/generated/`, the two Dart files
   into `packages/ui_flutter/lib/src/generated/`. The `TARGETS` list is where
   a new platform package is added.

A new token file is only built once it is added to `terrazzo.config.mjs`.
`dist/` is gitignored and is staging only — the distributed copies are what a
platform package reads, and they are committed.

## Component pipeline

`vite build` bundles `packages/ui_react/src/` into that package's own `dist/`:

- `index.js` + `*.d.ts` — the components; React stays external
- `components.css` — every component stylesheet, via the `styles.ts` entry

`src/styles/components.css` is the registry, the way `terrazzo.config.mjs` is
for tokens: a component's stylesheet only ships once it is imported there. It
also `@import`s `../generated/control.css`, so the generated recipes are
bundled in rather than pushed onto consumers as a third stylesheet.

Because `src/generated/` is committed, this package builds without `ds` having
run — there is no dependency edge and so no build order to get wrong. The cost
is that editing a token file does nothing until the generator runs, which is
why `pnpm packages` runs both and the storybook scripts call it.

Never hand-edit `src/generated/`. A value a component needs belongs in
`packages/ds`, and anything written there is overwritten on the next build.

## Token conventions

Three layers, each referencing the one above it:

- **Base primitives** (`base.*`) — two files at one layer. `_base.yaml` is the
  generated Tailwind-derived half; `_design.yaml` is the hand-authored half
  and holds what Tailwind does not supply: the `base.color.brand` ramp, and
  the AppKit metrics the design is drawn to (`base.corner`, `base.type`,
  `base.leading`, `base.control-height`, `base.font`). The namespaces sit
  beside Tailwind's rather than replacing them, so a token that aliases the
  wrong scale cannot look right by accident. Components must never reference
  either file directly.
- **Semantic globals** (`color`, `radius`, `spacing`, `typography`, `effect`,
  and the `control.*` size profiles) — named roles that alias primitives, so a
  theme can re-map a whole ramp by changing one reference.
- **Theme** (`tokens/theme/default/colors.yaml`) — the colour roles a theme
  re-points: the surface, content and border families. Both light and dark
  live in this one file, and that is forced rather than chosen: Terrazzo
  merges token files by replacing a whole token, so a second file adding a
  mode to `color.surface` replaces the first outright, and a file carrying
  only `$extensions` is dropped with no error.
- **Component tokens** (`tokens/component/*.yaml`) — only for values that are
  genuinely the component's own: a part no shared role covers (`checkbox.box`,
  `toggle.thumb`, `dialog.width`) or a `color`+`alpha` wash (`dialog.scrim`).
  A token that would merely forward a shared role (`{control.small.size}`,
  `{radius.medium}`, `{labelSmall}`) must not exist — the component's CSS
  reads the shared variable directly, so most components have no token file
  at all.

Three things that surprise people:

- Interactive colors are stored as *recipes*, not finished colors: a `shade` to
  pick off a ramp plus an `alpha` to apply. Each platform resolves them against
  whichever ramp the control is tinted with.
- A ramp is a **role ladder, not a lightness ramp**. Read `base.color.ink`
  before assuming otherwise: its 700 is *lighter* than its 600, because
  Bright's primary action is ink navy whose hover lifts, and its 50–200 are
  acid-tinted washes rather than pale navy. A step is only ever reached by a
  recipe, so what a step means is the role that recipe gives it. This is what
  lets four themes this different share one recipe table untouched — and why a
  shade written into a stylesheet would be a lie rather than merely a
  duplicate.
- A recipe can re-pick its step per theme (`shade` carries a mode), because a
  fixed step cannot be right in both directions: `tinted` wants the palest
  step of the ramp on paper and the darkest on a near-black canvas.
  `plugins/control.mjs` resolves those into a block per theme, since the shade
  is baked into the emitted value and nothing downstream could re-pick it.

### Themes

Four, as three modes over one default, defined once in `terrazzo.config.mjs`
and passed to all three plugins so the platforms cannot drift:

| `data-theme`   | modes applied            |
| -------------- | ------------------------ |
| *(none)*       | — Studio Light, `:root`  |
| `studio-dark`  | `dark`                   |
| `bright-light` | `bright`                 |
| `bright-dark`  | `bright`, `dark`, `bright-dark` |

The modes compose rather than repeat, so a value shared along the light/dark
axis or the Studio/Bright axis is written once. The order in that config is
load bearing: every selector is a single attribute and so has equal
specificity, and the later block wins.

Studio Light sets **no attribute at all** — selecting it means removing
`data-theme`, not writing `studio-light`. Writing it would leave the page
looking right for the wrong reason, since none of the three blocks would
match. Storybook's theme toolbar does exactly this; see `.storybook/preview.ts`.

Flutter has no cascade, so `plugins/flutter.mjs` collapses each theme's mode
chain into a whole `ThemeVariables` instance — `themeVariablesStudioDark`,
`themeVariablesBrightLight`, `themeVariablesBrightDark` — beside the default
`themeVariables`. It reads the themes back out of the same `modeSelectors`,
so adding a theme reaches Dart without the plugin being touched.

## Component conventions

- One folder-level file pair per component: `src/components/<name>.tsx` and
  `<name>.css`, both listed in their registries (`components/index.ts`,
  `styles/components.css`).
- Components are styled by plain CSS reading the token custom properties —
  there is no Tailwind here. A value a component needs is a shared variable
  read directly (`--spacing-4`, `--radius-medium`, `--label-small-font-size`,
  `--control-small-padding-block`); only a value no shared role covers becomes
  a token in `tokens/component/<name>.yaml`. Never a literal in the CSS.
- Sized controls read the `[data-size]` profile bindings — `--control-size`,
  `--control-gap`, `--control-padding-*`, `--control-radius`,
  `--control-font-*`, `--control-line-height` — once in their base rule
  instead of restating geometry per size (Button and IconButton are the
  model — neither keeps a per-size block at all, since the profile carries the
  design's own control type). A deviation from the profile keeps its size
  block and overrides the bound variable with another token's variable.
  Non-controls with sizes (Badge, Kbd, Card) skip the bindings and
  read the profile steps and typography faces directly per size block —
  `badge.css` is that model.
- Variants are `data-*` attributes, not classes, so the CSS selector names the
  axis (`[data-variant='tinted']`) and consumers can see the state in the DOM.
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
- Pick the recipe by what the part *is*, not by which colour looks right.
  `normal` is the neutral control — its hover wash is `surface-muted`, which is
  what a menu item, a table row, or a preference row wants. `plain` is the
  quiet *accent* control, so its wash comes off the tinted ramp and would sit
  heavier on a row than a `tinted` selection one step below it.
- `tint` is a prop only where a component genuinely re-tints. Components whose
  colors are already in the token graph (Kbd, Progress's track) read those
  directly.
- A `color` paired with an `alpha` anywhere else in the graph (`dialog.scrim`)
  is resolved by the same plugin onto `:root`, so no component has to apply an
  alpha in CSS.

## Storybook conventions

- Stories live in `apps/storybook/src/stories/*.stories.tsx`, titled
  `Tokens/<Group>`, `Components/<Name>`, or `Base/<Group>`. Sidebar order is
  set in `.storybook/preview.ts`.
- A component page ends with the token table it is drawn from
  (`componentTokens('<name>')`), so the specimens and their source sit on one
  page. For the many components that own no tokens the table renders a pointer
  to the shared vocabulary instead — the profile itself is documented on
  `Tokens/Control`.
- The gallery reads the **built** `dist/style.css`, not the token sources. It
  parses the `:root` block for each alias and resolves the `var()` chain via
  `getComputedStyle`, so every token shows both what it points at and what it
  evaluates to.
- Because of that, adding or renaming tokens usually needs no story changes —
  the existing pages pick them up. Reach for `byPrefix` / `groupScales` /
  `componentTokens` in `src/lib/tokens.ts` before hardcoding token names.
- Shared presentation lives in `src/components/gallery.tsx`. The gallery styles
  itself with DazzUI tokens; keep it that way.

## Code style

- Formatting and linting are configured once at the root (`.oxfmtrc.json`,
  oxlint defaults): single quotes, no semicolons, 80 columns, 2-space indent,
  omit parens on single-argument arrows.
- Run `pnpm format` and `pnpm lint` before committing. One oxlint warning in
  `packages/ds` predates this setup.

## Commits

- Do not add a `Co-Authored-By:` trailer. This history keeps no attribution
  trailers of any kind.
- Match the existing style: a short imperative subject, then a paragraph
  saying what changed and why.

## Known token issues

Worth knowing before "fixing" what looks like a Storybook bug:

- `tokens/global/effect.yaml` points `inset-shadow`, `drop-shadow`, and
  `text-shadow` at `{base.shadow.*}`, so the `base.inset-shadow`,
  `base.drop-shadow`, and `base.text-shadow` primitives are unused, and inset
  shadows lose their `inset` keyword.
- Shadow values carry a spread (a fourth length), which `text-shadow` and
  `filter: drop-shadow()` do not accept — those previews render with no shadow.
