# DazzUI

The DazzUI design system: one set of design tokens, a package per
platform that implements them, and an app that documents both.

## Workspace

| Path                 | Package                      | What it is                                                                                                       |
| -------------------- | ---------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `packages/ds`        | `@dazzlabs/dazzui-ds`           | The design system itself: tokens built with [Terrazzo](https://terrazzo.app/). A generator, not a dependency.    |
| `packages/ui_react`  | `@dazzlabs/dazzui-react`     | The React implementation. Plain CSS over the tokens — no Tailwind, no icon set.                                  |
| `apps/storybook`     | `@dazzlabs/dazzui-storybook` | A gallery of both: every token, and every component with the tokens it is drawn from.                            |

Further platform packages sit beside `ui_react` as they land — `ui_flutter`,
`ui_swiftui` — each implementing the same tokens for its own platform. Only
`packages/ds` decides what a value is, and it hands each platform its own
generated sources rather than being installed alongside them: a platform
package builds, ships and is consumed on its own.

## Getting started

```bash
pnpm install
```

Build the tokens and the React components:

```bash
pnpm packages
```

Browse them at <http://localhost:6006>:

```bash
pnpm storybook
```

## Scripts

| Script                 | What it does                                            |
| ---------------------- | ------------------------------------------------------- |
| `pnpm tokens`          | Build `packages/ds`, generating into the platform packages |
| `pnpm components`      | Build `packages/ui_react` into its `dist/`              |
| `pnpm packages`        | Both of the above, in order                             |
| `pnpm storybook`       | Build both packages, then start Storybook on port 6006  |
| `pnpm storybook:build` | Build a static Storybook into `apps/storybook/dist`     |
| `pnpm build`           | Build everything and the static Storybook               |
| `pnpm typecheck`       | Run `tsc` across the workspace                          |
| `pnpm lint`            | Lint the workspace with [oxlint](https://oxc.rs/)       |
| `pnpm format`          | Format the workspace with [oxfmt](https://oxc.rs/)      |

The Storybook reads the generated `style.css` rather than the token sources, so
it always shows what a consumer actually gets — including how each token
resolves once its `var()` chain is followed.

## License

MIT
