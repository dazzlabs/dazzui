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
- `packages/ui_gpui` (`dazzui`) — the [GPUI] implementation, in Rust.
  A Cargo workspace, invisible to pnpm: run `cargo test` / `cargo clippy` from
  inside the package. Like the others, `src/generated/` holds what `ds` wrote
  and is committed. GPUI has no cascade *and* no default arguments, so the
  generator splits the token set in two — `ThemeSeed` is what a theme
  re-points and a `const fn` derives the rest — which is what lets each theme
  be a `static` a component borrows rather than a value it rebuilds.
  `example/` holds the playground (`cargo run -p dazzui_playground`), a
  workspace member so the two share one `target/`.
- `apps/storybook` (`@dazzlabs/dazzui-storybook`) — a Storybook gallery that
  documents both: `Tokens/*` for the token graph, `Components/*` for the
  components.
- `examples/coding-agent` (`@dazzlabs/dazzui-example-coding-agent`) — a coding
  agent's desktop interface, built entirely out of `ui_react`. It is where the
  system is *used* rather than documented: a gallery shows every component on
  its own, and only an app has to make them agree with each other. It consumes
  the package the way any consumer would — the two published stylesheets and
  the components — and holds no literal values of its own. It runs in a
  browser tab and, through [Deno Desktop](https://docs.deno.com/runtime/desktop/),
  as a native window; `deno` is a devDependency of the package, so there is
  nothing to install for it.
- `examples/canvas` (`@dazzlabs/dazzui-example-canvas`) — a design canvas, built
  the same way and asking the design system a different question. The coding
  agent is a reading app; this one has a *drawing* in the middle of it, which
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
pnpm coding-agent      # build both, then serve the example on :5180
```

```bash
pnpm coding-agent:desktop  # build the example, then open it as a native window
```

```bash
pnpm canvas            # build both, then serve the canvas example on :5181
```

```bash
pnpm canvas:desktop    # build the canvas, then open it as a native window
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
- **Disabled flattens to the groove** — `surface-sunken` fill,
  `content-faint` text, no border — one dead look for every variant, never a
  translucent ghost of the live one.
- **Selection shifts weight as well as colour.** Rows rest at `labelQuiet`
  (500) and the selected entry takes the label weight (600); a pill tab's
  active label and an option card's title go to `labelStrong` (700, display
  cut). `stroke.control` (1.5px) draws control outlines; `stroke.hairline`
  draws separators and halves on Retina (see `ui_react`'s
  `src/styles/base.css`, which also carries the box-sizing and
  reduced-motion ground rules).
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
so adding a theme reaches Dart without the plugin being touched. Swift and
Rust do the same thing in their own idiom: an initialiser with defaults and
computed properties there, a `ThemeSeed` with `..STUDIO_LIGHT_SEED` and a
`const fn` here.

## Component conventions

- One folder-level file pair per component: `src/components/<name>.tsx` and
  `<name>.css`, both listed in their registries (`components/index.ts`,
  `styles/components.css`).
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
  component in all four themes headlessly, measures the control heights
  against the size profile, asserts on the recipes, and simulates the pointer
  and the keyboard — which is the part of a screenshot that can be checked
  without a screen. It is also what catches a wiring bug: a key listener on a
  descendant of the focused element never fires, and only a simulated
  backspace says so.

## Storybook conventions

- Stories live in `apps/storybook/src/stories/*.stories.tsx`, titled
  `Tokens/<Group>` or `Components/<Name>`. Sidebar order is set by the
  `storySort` comparator in `.storybook/preview.ts`, which sorts groups and
  pages but returns 0 for two entries sharing a title — so the stories inside
  one component keep the order they are written in. Adding a story means
  putting the export where it belongs in the file.
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
- The token table appears **only** where the component owns tokens — the nine
  files in `packages/ds/tokens/component/`, plus the `frame` metrics the shell
  columns are sized by. Most components own none: they read the shared
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

## Example conventions

`examples/*` are apps, not packages: private, never published, and each one
built and served on its own port. An example is added to `pnpm-workspace.yaml`
by the `examples/*` glob and given a root script beside the Storybook's.

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
  data share a sheet rather than duplicating it (`views.css` dresses both the
  issue and pull panes). Variants are `data-*` attributes, and the app takes a
  class prefix of its own — `ca-` for this one — so nothing collides with
  `ff-`.
- **Override by source order, not by force.** Where an example has to reshape a
  library component, it does it with a class of its own at one selector deeper
  (`.ca-composer .ca-composer__input`), which is the extension point `cx()`
  documents. It never edits `ui_react`, and it never reaches for `!important`.
- **Fixtures name real things.** The scripted data points at paths, registries
  and commands this workspace actually has. Invented paths are the first thing
  that reads as fake.
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

- Do not add a `Co-Authored-By:` trailer. This history keeps no attribution
  trailers of any kind.
- Match the existing style: a short imperative subject, then a paragraph
  saying what changed and why.
