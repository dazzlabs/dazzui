# DazzUI Mobile — HarmonyOS

A native ArkTS/ArkUI HAR for HarmonyOS phones and tablets. The library supplies
**colors only** through `DazzMobileTheme` and ArkUI `WithTheme`. Use system
`Button`, `TextInput`, `Checkbox`, `Radio`, `Toggle`, `Slider`, `Select`,
`Progress`, menus, pickers, dialogs and sheets directly.

ArkUI keeps its typography, shapes, touch targets, layout, animation,
accessibility and behavior. This package does not depend on the desktop HAR,
wrap individual controls, draw custom controls, or apply desktop recipes to
control geometry. Desktop pixel conformance does not apply.

## Use

Requires the Stage model and HarmonyOS API 20 or newer (`CustomTheme.darkColors`).
Add `"@dazzlabs/dazzui-mobile": "file:path/to/packages/arkts/dazzui-mobile"` to
your module's `oh-package.json5` and register the `dazzui_mobile` HAR module in
your project build profile. The example provides a complete configuration.

```ts
import { DazzMobileTheme, DazzThemeFamily } from '@dazzlabs/dazzui-mobile'

@Entry
@Component
struct Index {
  @State count: number = 0

  @LocalBuilder
  content() {
    Column() {
      Text('Count: ' + this.count)
      Button('Continue').onClick(() => { this.count++ })
    }
  }

  build() {
    Column() {
      DazzMobileTheme({
        family: DazzThemeFamily.Bright,
        content: this.content
      })
    }
  }
}
```

Studio is the default. All 36 paired ds families are available in
`DazzThemeFamily` and `DAZZ_THEME_FAMILIES`. `appearance` defaults to
`ThemeColorMode.SYSTEM`; set `LIGHT` or `DARK` for a local override. This does
not change the application's or device's color-mode policy. Nested scopes can
use different families and appearances. Use `@LocalBuilder` when forwarding a
slot so callbacks retain the state owner's `this`. The content must build one
root container, as required by `WithTheme`.

Hosts that already use `WithTheme` can pass `dazzTheme(family)` directly. It
returns a fresh `CustomTheme` with `colors` and `darkColors`, so ArkUI chooses
the appropriate half and scopes never share mutable color objects. For custom
page backgrounds, use `onWillApplyTheme(theme: Theme)` to read the scope's
`theme.colors.backgroundPrimary`; built-in controls inherit the colors directly.
Use `DazzMobileTheme` for these callbacks: on the API 26 emulator, raw
`WithTheme` reports the light `colors` to the callback even while native controls
use `darkColors`. The scope observes configuration changes, resolves the same
effective palette for both paths, and removes its observer when unmounted.

## Generated colors

`pnpm tokens` runs `ds/plugins/harmony-mobile.mjs` and distributes the committed
`src/main/ets/generated/DazzThemes.ets`. Never edit that output. The HAR builds
independently of Node, ds, and the desktop package.

The generator reads the same resolved graph as desktop, but emits only ArkUI
color roles. Brand, emphasized surfaces and selected controls use the primary
fill recipe; emphasized text uses the primary content recipe. Semantic status
colors use warning, danger and success. Font/icon roles, surfaces and dividers
read ds semantic colors; tinted backgrounds preserve the recipe alpha in
ArkUI's ARGB format. Native state opacity, disabled styling and motion remain
ArkUI's defaults. No typography, geometry or motion tokens are exported.

The underlying APIs are documented in
[WithTheme](https://developer.huawei.com/consumer/en/doc/harmonyos-references/ts-container-with-theme)
and [ArkUI themes](https://developer.huawei.com/consumer/en/doc/harmonyos-references/js-apis-arkui-theme).

## Build and run

Open `example/` in DevEco Studio. Use its bundled HarmonyOS SDK; compatibility
and target are API 20. The Stage gallery supports `phone` and `tablet`.
From the repository root:

```sh
pnpm harmony:mobile:setup
pnpm harmony:mobile:build
pnpm harmony:mobile:har
pnpm harmony:mobile
```

The run command builds, installs and starts the gallery. It selects a connected
phone/tablet automatically when there is exactly one. Set `HARMONY_DEVICE` for
multiple mobile devices. Tool overrides are `DEVECO_HOME`, `OHPM`, `HVIGOR`,
`DEVECO_NODE` and `HDC`. A physical device requires your own signing configuration
in DevEco Studio; the repository contains no signing identities. The local phone
emulator accepts the unsigned debug HAP.

The gallery has native family/appearance menus and Actions, Inputs and Explore
tabs. It is authored for mobile; it does not import the desktop stories or
rewrite the frozen iOS/Android galleries. Want parameters `family` and
`appearance` (`system`, `light`, `dark`) open a chosen theme for automation.
The host sets native tab labels/indicators, select arrows and system bar colors
from the effective palette, since these do not all inherit `WithTheme` on the
tested SDK. The HAR itself does not change window settings.

`.ohpmignore` excludes the nested gallery, scripts, tests and caches from the
published HAR. Its rooted directory patterns omit a trailing slash, matching
Hvigor's directory-name filtering.

## Verification

```sh
pnpm harmony:mobile:test          # generator and actual non-UI ArkTS runtime
pnpm harmony:mobile              # build and install the current gallery
pnpm harmony:mobile:test:device  # installed gallery, actual native UI
```

Node tests check reproducibility against all shared ds snapshots, paired-family
coverage, alpha, the public API and mutable scope isolation. They do not compile
ArkUI. Device verification checks all 45 supplied roles through the real
`onWillApplyTheme` callback in every family and appearance, nested scope isolation,
and system-mode changes. It then drives native buttons, disabled state, dialogs,
sheets, text input, switches, checkboxes and theme menus through `uitest` and
asserts the native accessibility tree. The native probe page is test-only;
it is not part of the gallery's navigation.

Verified on 2026-10-17 on `macbook-pro`, using the bundled API 26 SDK and a
HarmonyOS API 26 phone emulator: HAP compilation/install/start, standalone HAR
compilation and archive inspection, all 74 native theme checks (45 roles in
outer, nested and sibling scopes), and the native interaction suite passed.
Runtime verification on API 20 and physical phones/tablets has not been performed.
