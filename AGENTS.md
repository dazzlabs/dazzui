# AGENTS.md

Guidance for coding agents working in this repository.

## What this is

The DazzUI design system, as a pnpm workspace:

- `packages/tokens` (`@dazzlabs/dazzui-tokens`) — the design tokens, built with
  [Terrazzo](https://terrazzo.app/). This is the source of truth; everything
  else consumes its build output.
- `apps/storybook` (`@dazzlabs/dazzui-storybook`) — a Storybook gallery that
  documents the tokens. Tokens only: there are no web components in this repo.

## Commands

Run these from the repository root.

```bash
pnpm install
```

```bash
pnpm tokens            # build packages/tokens into dist/
```

```bash
pnpm storybook         # build tokens, then serve Storybook on :6006
```

```bash
pnpm lint              # oxlint over the workspace
```

```bash
pnpm format            # oxfmt over the workspace
```

Typecheck the app with `pnpm --filter @dazzlabs/dazzui-storybook typecheck`.

## Token pipeline

1. `packages/tokens/tokens/global/_base.json` holds the primitive layer and is
   **generated** — `pnpm --filter @dazzlabs/dazzui-tokens sync` downloads
   Tailwind's `theme.css` and rewrites it. Never hand-edit that file.
2. `tz build` reads the files listed in `terrazzo.config.mjs` and emits
   `packages/tokens/dist/`:
   - `style.css` — CSS custom properties (the contract consumers use)
   - `flutter/*.dart` — Dart sources for the sibling `dazzui-flutter` repo
3. `node scripts/build.mjs --copy` copies the Dart output into
   `../../../dazzui-flutter`. That path assumes the sibling repo is checked out
   next to this one; the flag is opt-in and safe to ignore otherwise.

A new token file is only built once it is added to `terrazzo.config.mjs`.
`dist/` is gitignored, so build before touching anything that reads it.

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

## Storybook conventions

- Stories live in `apps/storybook/src/stories/*.stories.tsx`, titled
  `Tokens/<Group>` or `Base/<Group>`. Sidebar order is set in
  `.storybook/preview.ts`.
- The gallery reads the **built** `dist/style.css`, not the token sources. It
  parses the `:root` block for each alias and resolves the `var()` chain via
  `getComputedStyle`, so every token shows both what it points at and what it
  evaluates to.
- Because of that, adding or renaming tokens usually needs no story changes —
  the existing pages pick them up. Reach for `byPrefix` / `groupScales` in
  `src/lib/tokens.ts` before hardcoding a list of token names.
- Shared presentation lives in `src/components/gallery.tsx`. The gallery styles
  itself with DazzUI tokens; keep it that way.

## Code style

- Formatting and linting are configured once at the root (`.oxfmtrc.json`,
  oxlint defaults): single quotes, no semicolons, 80 columns, 2-space indent,
  omit parens on single-argument arrows.
- Run `pnpm format` and `pnpm lint` before committing. One oxlint warning in
  `packages/tokens` predates this setup.

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
