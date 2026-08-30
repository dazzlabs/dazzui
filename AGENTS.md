# AGENTS.md

Guidance for coding agents working in this repository.

## What this is

The DazzUI design system, as a pnpm workspace:

- `packages/ui` (`@dazzlabs/dazzui`) — the design tokens, built with
  [Terrazzo](https://terrazzo.app/), and the React components that consume
  them. The tokens are the source of truth; everything else, the components
  included, reads their build output.
- `apps/storybook` (`@dazzlabs/dazzui-storybook`) — a Storybook gallery that
  documents both: `Tokens/*` and `Base/*` for the token graph, `Components/*`
  for the components.

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
pnpm tokens            # build the token graph into packages/ui/dist/
```

```bash
pnpm components        # build the component bundle into the same dist/
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

1. `packages/ui/tokens/global/_base.json` holds the primitive layer and is
   **generated** — `pnpm --filter @dazzlabs/dazzui sync` downloads
   Tailwind's `theme.css` and rewrites it. Never hand-edit that file.
2. `tz build` reads the files listed in `terrazzo.config.mjs` and emits
   `packages/ui/dist/`:
   - `style.css` — CSS custom properties (the contract consumers use)
   - `flutter/*.dart` — Dart sources for the sibling `dazzui-flutter` repo
   - `control.css` — the ramp bindings and the resolved interactive color
     recipes, from `plugins/control.mjs`; bundled into `components.css`
3. `node scripts/build.mjs --copy` copies the Dart output into
   `../../../dazzui-flutter`. That path assumes the sibling repo is checked out
   next to this one; the flag is opt-in and safe to ignore otherwise.

A new token file is only built once it is added to `terrazzo.config.mjs`.
`dist/` is gitignored, so build before touching anything that reads it.

## Component pipeline

`vite build` bundles `packages/ui/src/` into the *same* `dist/`, which is why
its config sets `emptyOutDir: false` — wiping the directory would take the
freshly built tokens with it. It emits:

- `index.js` + `*.d.ts` — the components; React stays external
- `components.css` — every component stylesheet, via the `styles.ts` entry

`src/styles/components.css` is the registry, the way `terrazzo.config.mjs` is
for tokens: a component's stylesheet only ships once it is imported there.

## Token conventions

Three layers, each referencing the one above it:

- **Base primitives** (`base.*`) — raw Tailwind-derived values. Components must
  never reference these directly.
- **Semantic globals** (`color`, `radius`, `spacing`, `typography`, `effect`) —
  named roles that alias primitives, so a theme can re-map a whole ramp by
  changing one reference.
- **Component tokens** (`tokens/component/*.json`) — alias the shared
  `control.*` sizing scale rather than defining their own dimensions.

Two things that surprise people:

- Interactive colors are stored as *recipes*, not finished colors: a `shade` to
  pick off a ramp plus an `alpha` to apply. Each platform resolves them against
  whichever ramp the control is tinted with.
- `tokens/theme/default/{light,dark}.json` are still empty, so there is no dark
  theme yet and the CSS output has no theme scoping.

## Component conventions

- One folder-level file pair per component: `src/components/<name>.tsx` and
  `<name>.css`, both listed in their registries (`components/index.ts`,
  `styles/components.css`).
- Components are styled by plain CSS reading the token custom properties —
  there is no Tailwind here. A value a component needs but no token carries is
  a missing token: add it to `tokens/component/<name>.json` rather than writing
  a literal into the CSS.
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
  page.
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
  `packages/ui` predates this setup.

## Commits

- Do not add a `Co-Authored-By:` trailer. This history keeps no attribution
  trailers of any kind.
- Match the existing style: a short imperative subject, then a paragraph
  saying what changed and why.

## Known token issues

Worth knowing before "fixing" what looks like a Storybook bug:

- `tokens/global/effect.json` points `inset-shadow`, `drop-shadow`, and
  `text-shadow` at `{base.shadow.*}`, so the `base.inset-shadow`,
  `base.drop-shadow`, and `base.text-shadow` primitives are unused, and inset
  shadows lose their `inset` keyword.
- Shadow values carry a spread (a fourth length), which `text-shadow` and
  `filter: drop-shadow()` do not accept — those previews render with no shadow.
