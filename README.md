# DazzUI

The DazzUI design system: the design tokens, and the app that documents
them.

## Workspace

| Path             | Package                      | What it is                                                                                                                          |
| ---------------- | ---------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `packages/ui`    | `@dazzlabs/dazzui`           | Design tokens built with [Terrazzo](https://terrazzo.app/). Emits CSS custom properties and Flutter sources.                        |
| `apps/storybook` | `@dazzlabs/dazzui-storybook` | A Storybook gallery of every token: color ramps, the type scale, spacing, radius, effects, control sizing, and per-component specs. |

## Getting started

```bash
pnpm install
```

Build the tokens:

```bash
pnpm tokens
```

Browse them at <http://localhost:6006>:

```bash
pnpm storybook
```

## Scripts

| Script                 | What it does                                        |
| ---------------------- | --------------------------------------------------- |
| `pnpm tokens`          | Build `packages/ui` into `dist/`                    |
| `pnpm storybook`       | Build the tokens, then start Storybook on port 6006 |
| `pnpm storybook:build` | Build a static Storybook into `apps/storybook/dist` |
| `pnpm build`           | Build the tokens and the static Storybook           |
| `pnpm lint`            | Lint the workspace with [oxlint](https://oxc.rs/)   |
| `pnpm format`          | Format the workspace with [oxfmt](https://oxc.rs/)  |

The Storybook reads the generated `dist/style.css` rather than the token
sources, so it always shows what a consumer actually gets — including how each
token resolves once its `var()` chain is followed.

## License

MIT
