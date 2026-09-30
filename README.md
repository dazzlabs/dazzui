# DazzUI

The DazzUI design system: one set of design tokens, a package per
platform that implements them, and a playground in each platform package.

## Workspace

The layout follows [nativeapi](https://github.com/libnativeapi/nativeapi)'s
`core/` + `bindings/<language>/<package>`: the design system at the top, and
an implementation per UI toolkit under `packages/<language>/`, named as it is
published. Each package carries its own playground in `example/`.

| Path                          | Package                             | What it is                                                                                                  |
| ----------------------------- | ----------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `ds`                          | `@dazzlabs/dazzui-ds`               | The design system itself: tokens built with [Terrazzo](https://terrazzo.app/). A generator, not a dependency. |
| `packages/js/dazzui-react`    | `@dazzlabs/dazzui-react`            | The React implementation, and the standard the others are held to. Plain CSS over the tokens.               |
| `packages/dart/dazzui`        | `dazzui`                            | The Flutter implementation.                                                                                 |
| `packages/swift/DazzUI`       | `DazzUI`                            | The macOS SwiftUI implementation.                                                                           |
| `packages/swift/DazzMobileUI` | `DazzMobileUI`                      | iOS: system SwiftUI controls with ds colours.                                                               |
| `packages/csharp/DazzUI`      | `DazzUI`                            | The WinUI 3 implementation. See its [README](packages/csharp/DazzUI/README.md).                              |
| `packages/kotlin/dazzui-mobile` | `dazzui-mobile`                   | Android: stock Compose Material 3 with ds colours. See its [README](packages/kotlin/dazzui-mobile/README.md). |
| `tools/codegen`               | `@dazzlabs/dazzui-codegen`          | Generates every playground's stories from the React declarations.                                           |
| `tools/conformance`           | `@dazzlabs/dazzui-conformance`      | The conformance viewer: how closely each implementation draws what React draws.                             |

Only `ds` decides what a value is, and it hands each package its own
generated sources rather than being installed alongside them: a package
builds, ships and is consumed on its own.

The two Rust ports — `packages/rust/dazzui` and `packages/rust/dazzui_inset` — are
maintained on the `platform/gpui` branch rather than here. Everything they
need is on it: their own generated sources, the `ds` plugins that write them,
and the playground adapters that compile the shared scenes into Rust.

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
pnpm playground
```

The WinUI 3 example runs on Windows with the .NET 8 SDK or later:

```powershell
pnpm winui3
```

`pnpm winui3:build` builds it, and `pnpm winui3:test` runs the real-window
smoke tests. It can also build without pnpm; see
[`packages/csharp/DazzUI`](packages/csharp/DazzUI/README.md).

## Scripts

| Script                 | What it does                                            |
| ---------------------- | ------------------------------------------------------- |
| `pnpm tokens`          | Build `ds`, generating into the platform packages |
| `pnpm components`      | Build `packages/js/dazzui-react` into its `dist/`              |
| `pnpm packages`        | Both of the above, in order                             |
| `pnpm playground`       | Build both packages, then start Storybook on port 6006  |
| `pnpm playground:build` | Build a static Storybook into `packages/js/dazzui-react/example/dist`     |
| `pnpm build`           | Build everything and the static Storybook               |
| `pnpm typecheck`       | Run `tsc` across the workspace                          |
| `pnpm lint`            | Lint the workspace with [oxlint](https://oxc.rs/)       |
| `pnpm format`          | Format the workspace with [oxfmt](https://oxc.rs/)      |

The Storybook reads the generated `style.css` rather than the token sources, so
it always shows what a consumer actually gets — including how each token
resolves once its `var()` chain is followed.

## License

MIT
