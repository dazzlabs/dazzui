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
| `packages/js/dazzui`    | `@dazzlabs/dazzui`            | The React implementation, and the standard the others are held to. Plain CSS over the tokens.               |
| `packages/dart/dazzui`        | `dazzui`                            | The Flutter implementation.                                                                                 |
| `packages/swift/DazzUI`       | `DazzUI`                            | The macOS SwiftUI implementation.                                                                           |
| `packages/swift/DazzMobileUI` | `DazzMobileUI`                      | iOS: system SwiftUI controls with ds colours.                                                               |
| `packages/csharp/DazzUI`      | `DazzUI`                            | The WinUI 3 implementation. See its [README](packages/csharp/DazzUI/README.md).                              |
| `packages/kotlin/dazzui-mobile` | `dazzui-mobile`                   | Android: stock Compose Material 3 with ds colours. See its [README](packages/kotlin/dazzui-mobile/README.md). |
| `packages/arkts/dazzui-mobile` | `@dazzlabs/dazzui-mobile` | HarmonyOS mobile: native ArkUI controls with ds colours. See its [README](packages/arkts/dazzui-mobile/README.md). |
| `packages/rust/dazzui`        | `dazzui`                            | The GPUI implementation. See its [README](packages/rust/dazzui/README.md).                                   |
| `tools/codegen`               | `@dazzlabs/dazzui-codegen`          | Generates every playground's stories from the React declarations.                                           |
| `tools/conformance`           | `@dazzlabs/dazzui-conformance`      | The conformance viewer: how closely each implementation draws what React draws.                             |

Only `ds` decides what a value is, and it hands each package its own
generated sources rather than being installed alongside them: a package
builds, ships and is consumed on its own.

The Inset port, a Rust port of the Flutter package, is maintained on the
`platform/gpui` branch rather than here, with the `ds` plugins it needs.

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
| `pnpm components`      | Build `packages/js/dazzui` into its `dist/`              |
| `pnpm packages`        | Both of the above, in order                             |
| `pnpm playground`       | Build both packages, then start Storybook on port 6006  |
| `pnpm playground:build` | Build a static Storybook into `packages/js/dazzui/example/dist`     |
| `pnpm build`           | Build everything and the static Storybook               |
| `pnpm typecheck`       | Run `tsc` across the workspace                          |
| `pnpm lint`            | Lint the workspace with [oxlint](https://oxc.rs/)       |
| `pnpm format`          | Format the workspace with [oxfmt](https://oxc.rs/)      |

The Storybook reads the generated `style.css` rather than the token sources, so
it always shows what a consumer actually gets — including how each token
resolves once its `var()` chain is followed.

## Native Shell

`packages/rust/dazzui-nsh` is the Rust desktop implementation for
[Native Shell](https://nsh.dev/). Run `pnpm nsh` for its native gallery,
`pnpm nsh:test` for engine tests, or `pnpm nsh:test:native` for real-window
checks. `pnpm nsh:web:build` compiles the same Rust gallery for browsers;
`pnpm nsh:web:deploy` publishes its independent Cloudflare Worker.
It embeds all ds themes and maps all 188 shared scenes across 53 pages.
It participates in the default conformance check and codegen support gate.
The native collector records actual boxes and screenshots; remaining differences
and measurement limits are documented in its [README](packages/rust/dazzui-nsh/README.md).

## GTK4

The Rust/gtk4-rs implementation lives in
[`packages/rust/dazzui-gtk4`](packages/rust/dazzui-gtk4/README.md).
It embeds the shared ds themes and returns native GTK controls. Requires
GTK4 >= 4.16. Run `pnpm gtk4` for its gallery, `pnpm gtk4:build` to build,
and `pnpm gtk4:test` for the native tests. The initial port's supported
story mappings and remaining gaps are documented in the package README.

## HarmonyOS desktop

The native ArkTS/ArkUI HAR and PC gallery live in
[`packages/arkts/dazzui`](packages/arkts/dazzui/README.md). The package receives
committed ds themes and generated React stories, independently of pnpm's
workspace. Open its `example/` in DevEco Studio with HarmonyOS SDK 6.0.0
(API 20) and a PC/2in1 target. `pnpm harmony:build` builds the example,
`pnpm harmony:har` builds the library, and `pnpm harmony:test` checks the
generators and non-UI runtime logic. The initial port's scene coverage and
native compilation/conformance limitations are documented in its README.

## HarmonyOS mobile

[`packages/arkts/dazzui-mobile`](packages/arkts/dazzui-mobile/README.md) supplies
color-only `DazzMobileTheme` scopes for native ArkUI controls on phones and
tablets. All paired ds families support light, dark and system appearance.
`pnpm harmony:mobile` builds, installs and opens the mobile gallery;
`pnpm harmony:mobile:har` builds the standalone HAR. Use
`pnpm harmony:mobile:test` for generator/runtime checks and
`pnpm harmony:mobile:test:device` for the installed gallery's native tests.

## License

MIT
