# DazzUI

The DazzUI design system: one set of design tokens, a package per
platform that implements them, an app that documents both, and examples that
put them to work.

## Workspace

`packages/mobile_ui_android` is the Android mobile package: ds colors applied
to official Compose Material 3 components, retaining their system typography,
shapes and behavior. See its [setup and playground](packages/mobile_ui_android/README.md).

| Path                 | Package                      | What it is                                                                                                       |
| -------------------- | ---------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `packages/ds`        | `@dazzlabs/dazzui-ds`           | The design system itself: tokens built with [Terrazzo](https://terrazzo.app/). A generator, not a dependency.    |
| `packages/ui_react`  | `@dazzlabs/dazzui-react`     | The React implementation. Plain CSS over the tokens — no Tailwind, no icon set.                                  |
| `packages/mobile_ui_ios` | `DazzMobileUI` | Native SwiftUI mobile controls with ds colours; system typography, geometry and behaviour. |
| `packages/ui_gpui`   | `dazzui`               | The [GPUI](https://gpui.rs) implementation, in Rust, with a playground beside it.                                |
| `packages/ui_inset` | `dazzui_inset` | Inset implementation in Rust, with generated themes and a native gallery. See its [setup and scope](packages/ui_inset/README.md). |
| `packages/ui_winui3` | `DazzUI` | The WinUI 3 implementation, generated themes and native Windows playground. See its [README](packages/ui_winui3/README.md). |
| `apps/storybook`     | `@dazzlabs/dazzui-storybook` | A gallery of both: every token, and every component with the tokens it is drawn from.                            |
| `examples/workbench` | `@dazzlabs/dazzui-example-workbench` | A Git-rooted agent workbench — project tabs, nine pages, a cross-project task queue — built entirely out of the React components. |
| `examples/canvas`    | `@dazzlabs/dazzui-example-canvas` | A design canvas, where the drawing itself is painted out of the tint recipes. |

Further platform packages sit beside `ui_react` as they land — `ui_flutter`,
`ui_swiftui`, `ui_gpui` — each implementing the same tokens for its own
platform. Only
`packages/ds` decides what a value is, and it hands each platform its own
generated sources rather than being installed alongside them: a platform
package builds, ships and is consumed on its own.

`examples/` is where the system is used rather than documented. A gallery shows
every component on its own; an example has to make them agree with each other,
which is the only way to find out whether the vocabulary is enough to build an
app out of. The two ask different questions of it: the workbench is a
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
pnpm workbench
```

And into a design canvas, at <http://localhost:5181>:

```bash
pnpm canvas
```

Either one in a native window packaged with
[Deno Desktop](https://docs.deno.com/runtime/desktop/):

```bash
pnpm workbench:desktop
```

```bash
pnpm canvas:desktop
```

The Rust package is invisible to pnpm, the way the Flutter one is. Its
playground opens in a native window:

```bash
cargo run -p dazzui_playground
```

from `packages/ui_gpui`.

The WinUI 3 playground runs on Windows with the .NET 8 SDK or later:

```powershell
pnpm winui3
```

`pnpm winui3:build` builds it, and `pnpm winui3:test` runs the real-window
smoke tests. It can also build without pnpm; see
[`packages/ui_winui3`](packages/ui_winui3/README.md).

## Scripts

| Script                 | What it does                                            |
| ---------------------- | ------------------------------------------------------- |
| `pnpm tokens`          | Build `packages/ds`, generating into the platform packages |
| `pnpm components`      | Build `packages/ui_react` into its `dist/`              |
| `pnpm packages`        | Both of the above, in order                             |
| `pnpm storybook`       | Build both packages, then start Storybook on port 6006  |
| `pnpm storybook:build` | Build a static Storybook into `apps/storybook/dist`     |
| `pnpm workbench`       | Build both packages, then serve the example on port 5180 |
| `pnpm workbench:desktop` | Build the example, then open it as a native window     |
| `pnpm workbench:desktop:build` | Package the window into `dist-desktop/`          |
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
