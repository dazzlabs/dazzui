# DazzUI for HarmonyOS desktop

A native ArkTS/ArkUI desktop implementation, published as the
`@dazzlabs/dazzui` HAR. It targets HarmonyOS 6.0.0 (API 20) and the
`2in1` PC device type. The project uses the Stage application model and
ArkUI's V1 component state model. It is independent of pnpm's workspace.

`src/main/ets/generated/ThemeData.ets` is committed output from
`ds/plugins/harmony.mjs`. The library builds without ds, React or a WebView.
`pnpm tokens` regenerates it; never edit generated sources. All 72 themes,
semantic roles, desktop metrics, typography and six tint ramps come from ds.
Colors are ArkUI ARGB strings. Recipes keep their tint open at runtime, retain
source alpha and pick the same per-theme shades as the other desktop ports.
The UI face is HarmonyOS Sans at ds's sizes and weights; CSS font stacks
cannot install an Apple font on HarmonyOS.

## Build and run

Open **this package's `example/` directory** in DevEco Studio, install the
bundled HarmonyOS SDK (API 20 or newer) and sync the project. Its `entry` module depends on the
sibling `dazzui` HAR through `file:../..`. Select a HarmonyOS PC/2in1 target
and run `entry`. Configure your own signing profile in DevEco for deployment;
there are no signing identities or certificates in this repository.

Once the DevEco tools are available, these root commands are equivalent:

```sh
pnpm harmony             # install dependencies, build, install and start on a device/emulator
pnpm harmony:setup       # ohpm install --all in example/
pnpm harmony:build       # unsigned debug HAP, including the HAR dependency
pnpm harmony:har         # standalone library HAR
pnpm harmony:test        # generator and non-UI runtime tests, no SDK needed
```

`DEVECO_HOME` can point to the DevEco installation's `Contents` directory.
`HVIGOR`, `OHPM` and `DEVECO_NODE` override their executable paths.
`HDC` selects the device tool and `HARMONY_DEVICE` selects a connected target.
The compile SDK follows DevEco’s bundled SDK; compatibility and target stay at API 20. The build
runner also accepts the `example/hvigorw` wrapper created by DevEco. Wrapper
binaries, local SDK paths, build output and ohpm dependencies are ignored.
The SDK build fails with an explicit message when the tools are unavailable.
The Node tests execute the actual non-UI ArkTS logic after TypeScript
transpilation; they do **not** establish ArkUI compiler or device compatibility.

## Use in an application

Declare the HAR in the consuming module's `oh-package.json5`, run ohpm
install, and import from `@dazzlabs/dazzui`. The `Dazz` prefix avoids collisions
with ArkUI's built-in `Button`, `TextInput`, `Select`, `Tabs` and other globals.
Constructor options also avoid inherited ArkUI attributes: use `controlSize`,
`iconSize`, `contentWidth`/`contentHeight`, `contentAlign` and `onAction`.
React story props keep their original names; the interpreter translates them.

```ts
import { DazzTheme, DazzButton, DazzTextField } from '@dazzlabs/dazzui'

@Entry
@Component
struct Editor {
  @State name: string = ''

  @LocalBuilder content() {
    Column({ space: 16 }) {
      DazzTextField({
        value: this.name,
        placeholder: 'Project name',
        accessibilityLabel: 'Project name',
        onValueChange: (value: string) => { this.name = value }
      })
      DazzButton({ label: 'Save', variant: 'filled', onAction: () => {} })
    }
  }

  build() {
    Column() {
      DazzTheme({ name: 'studio-light', content: this.content })
    }.width('100%').height('100%')
  }
}
```

`DazzTheme` provides a fresh `DazzTokens` instance to its subtree. Changing
`name` updates descendants, and nested theme scopes do not mutate another
window's palette. The host chooses a theme explicitly; following OS appearance
is the host's policy, as for the other desktop implementations.

Input and selection controls expose an optional controlled `value`, `checked`
or `pressed`, the corresponding `on…Change` callback and an uncontrolled
`default…` value. Supply a callback when controlling a value. Buttons take
builder slots for icons; pass `DazzIcon` a Fluent glyph's React name, such as
`ArrowDownload20Regular`. Committed SVGs are generated from the same React
Fluent sources by `pnpm codegen`.

## Components and native behavior

The public entry is `Index.ets`. Implemented families include:

- Button, IconButton and Toggle; Checkbox, Switch, RadioGroup, Slider;
  TextField, SearchField, Select, NumberField and SegmentedControl.
- Icon, Badge, Avatar, KeyCap, Card, Divider, Progress, Spinner, Meter,
  Code, SectionLabel, Skeleton and SkeletonText.
- FormField, Fieldset, OptionCard, Callout, EmptyState, Preferences and
  its group/section/row parts, ActionBar, Table, Tag and StepList.
- Dialog and DialogClose, Popover, Tooltip, Menu/context menu,
  Collapsible and native Splitter/Panel.

ArkUI owns text editing and IME, native control activation, radio grouping,
slider keyboard operation, tab selection, menu positioning and modal lifecycle.
Drawing reads ds metrics and recipes. Native Select/menu popup row geometry,
radio/checkbox marks, switch rendering and splitter handles still retain
platform details. The port has not established desktop geometry conformance.
Toggle and Collapsible use native button activation with local selection/open
state; their richer group behavior is not yet mapped.

## React stories and coverage

The example has a searchable component rail, Overview previews, component
pages with notes, and a grouped theme panel. Its stories are compiled from
`packages/js/dazzui/example/src/stories/` by `tools/codegen/harmony.mjs`.
The typed scene interpreter preserves state setters, event arguments,
comparisons and specimen addresses without dynamic reflection or `any`.

Run `pnpm codegen` after story changes, and `pnpm codegen:check` to detect
drift. `example/coverage.json` records every unmapped scene and its exact
component/prop/value gaps. Scenes with any gap display that list in full;
they never draw a misleading subset. The initial port stays outside the
codegen full-support gate, like GTK4.

Remaining families include Accordion, Combobox/Autocomplete, Drawer, Link,
OtpField, PillTabs, PreviewCard, ShortcutRecorder, Sidebar, Toast, ToggleGroup,
and Tree. Additional gaps include mixed checkboxes, grouped/rich Select
options, range sliders, segmented icons, required/form submission, rich inline
text and optional CodeBlock controls. See the generated coverage file for the
current list instead of treating this first implementation as a full port.

## Native capture

Build, sign and install the gallery on a connected PC, then launch a scene:

```sh
pnpm harmony:conformance button/preview studio-light
```

This uses `hdc` (`HDC` overrides its path) to open the native capture page.
Use **Capture geometry and image** after the scene is laid out. The page writes
JSON geometry and a PNG into the app's `filesDir`, and displays their filename;
export them through DevEco's app-file tooling. The collector reads actual
ArkUI rectangles in physical pixels, converts them to logical vp, and keeps
unmounted popup nodes separate from unsupported scenes. The PNG uses the
native component snapshot API with a 2x capture scale.

These captures are diagnostic artifacts, not fabricated baselines. They do
not yet emit all decoration/type readings or participate in the default
`pnpm conformance:check` run. Native compilation, all-theme rendering,
keyboard/focus, IME and popup tests still require DevEco and a HarmonyOS PC.

The platform contracts follow the official [module configuration](https://github.com/openharmony/docs/blob/master/en/application-dev/quick-start/module-configuration-file.md),
[ArkUI state scopes](https://github.com/openharmony/docs/blob/master/en/application-dev/ui/state-management/arkts-provide-and-consume.md)
and [component snapshot API](https://github.com/openharmony/docs/blob/master/en/application-dev/reference/apis-arkui/arkts-apis-uicontext-componentsnapshot.md).

## Verified run

On 2026-10-15 the HAP compiled with DevEco/Hvigor 6.26.8 and its bundled API 26
SDK on the `macbook-pro` host, installed on the running PC/2in1 emulator, and
launched the gallery. This verifies compilation and an emulator startup; it does
not establish all-theme rendering, every interaction or physical-PC conformance.

From the repository root, repeat that run on the configured host with:

```sh
fastforge --no-version-check host exec macbook-pro -- node packages/arkts/dazzui/scripts/run.mjs
```
