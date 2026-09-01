# DazzUI

The DazzUI design system: one set of design tokens, a package per
platform that implements them, an app that documents both, and examples that
put them to work.

## Workspace

| Path                 | Package                      | What it is                                                                                                       |
| -------------------- | ---------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `packages/ds`        | `@dazzlabs/dazzui-ds`           | The design system itself: tokens built with [Terrazzo](https://terrazzo.app/). A generator, not a dependency.    |
| `packages/ui_react`  | `@dazzlabs/dazzui-react`     | The React implementation. Plain CSS over the tokens — no Tailwind, no icon set.                                  |
| `apps/storybook`     | `@dazzlabs/dazzui-storybook` | A gallery of both: every token, and every component with the tokens it is drawn from.                            |
| `examples/coding-agent` | `@dazzlabs/dazzui-example-coding-agent` | A coding agent's desktop interface, built entirely out of the React components. |
| `examples/canvas`    | `@dazzlabs/dazzui-example-canvas` | A design canvas, where the drawing itself is painted out of the tint recipes. |

Further platform packages sit beside `ui_react` as they land — `ui_flutter`,
`ui_swiftui` — each implementing the same tokens for its own platform. Only
`packages/ds` decides what a value is, and it hands each platform its own
generated sources rather than being installed alongside them: a platform
package builds, ships and is consumed on its own.

`examples/` is where the system is used rather than documented. A gallery shows
every component on its own; an example has to make them agree with each other,
which is the only way to find out whether the vocabulary is enough to build an
app out of. The two ask different questions of it: the coding agent is a
reading app made of rows and forms, and the canvas has a drawing in the middle
of it — the one region of a window whose colours are usually the user's rather
than the designer's, and which here is painted out of the same tint recipes a
button is.

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

Or see them composed into an app, at <http://localhost:5180>:

```bash
pnpm coding-agent
```

And into a design canvas, at <http://localhost:5181>:

```bash
pnpm canvas
```

Either one in a native window packaged with
[Deno Desktop](https://docs.deno.com/runtime/desktop/):

```bash
pnpm coding-agent:desktop
```

```bash
pnpm canvas:desktop
```

## Scripts

| Script                 | What it does                                            |
| ---------------------- | ------------------------------------------------------- |
| `pnpm tokens`          | Build `packages/ds`, generating into the platform packages |
| `pnpm components`      | Build `packages/ui_react` into its `dist/`              |
| `pnpm packages`        | Both of the above, in order                             |
| `pnpm storybook`       | Build both packages, then start Storybook on port 6006  |
| `pnpm storybook:build` | Build a static Storybook into `apps/storybook/dist`     |
| `pnpm coding-agent`    | Build both packages, then serve the example on port 5180 |
| `pnpm coding-agent:desktop` | Build the example, then open it as a native window   |
| `pnpm coding-agent:desktop:build` | Package the window into `dist-desktop/`        |
| `pnpm canvas`          | Build both packages, then serve the canvas on port 5181 |
| `pnpm canvas:desktop`  | Build the canvas, then open it as a native window       |
| `pnpm canvas:desktop:build` | Package that window into `dist-desktop/`           |
| `pnpm build`           | Build everything and the static Storybook               |
| `pnpm typecheck`       | Run `tsc` across the workspace                          |
| `pnpm lint`            | Lint the workspace with [oxlint](https://oxc.rs/)       |
| `pnpm format`          | Format the workspace with [oxfmt](https://oxc.rs/)      |

The Storybook reads the generated `style.css` rather than the token sources, so
it always shows what a consumer actually gets — including how each token
resolves once its `var()` chain is followed.

## License

MIT
