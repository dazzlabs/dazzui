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
pnpm harmony:test:device # installed gallery: every page and native interactions
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

@Component
struct EditorContent {
  @State name: string = ''

  build() {
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
}

@Entry
@Component
struct Editor {
  @LocalBuilder content() { EditorContent() }
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

- Button, IconButton, Toggle and ToggleGroup; Checkbox, Switch, RadioGroup, Slider;
  TextField, SearchField, Select, Combobox/Autocomplete, NumberField, OtpField,
  ShortcutRecorder and SegmentedControl.
- Icon, Badge, Avatar, KeyCap, Card, Divider, Progress, Spinner, Meter,
  Code/CodeBlock, Link, SectionLabel, Skeleton and SkeletonText.
- Form, FormField, Fieldset, Accordion, AvatarGroup, PillTabs, OptionCard, Callout, EmptyState, Preferences and
  its group/section/row parts, ActionBar, Table, Tag, StepList, Tree, Sidebar,
  navigation Tabs, Calendar, DateRangePicker and presentation WindowFrame parts.
- Dialog with Header/Body/Footer/Close, Drawer/DrawerClose, Popover/PopoverClose,
  PreviewCard, Tooltip, Menubar, Menu/context menu,
  Collapsible, ToastProvider/ToastViewport and native Splitter/Panel.

`DazzToastProvider` owns a queue with an optional `limit` (default 3). A child
consumes `DazzToastContext` as `@Consume('dazzui.toast')` and calls `add` or
`close`. Titles, tint, optional Fluent icon, action, dismiss label, priority and
close callbacks follow the React options; actions use `onAction`. Timeout roles
come from ds; `timeout: 0` persists, which is also the default for danger toasts.
`DazzToastViewport` places the queue at the window's top or bottom edge,
independently of where it appears in the content builder.

F6 enters notifications; Tab visits the whole receipt and its buttons, and
returns to the previous control at the boundary. Focus and held touches pause
timers. Body presses select the receipt without activating keyboard focus.
Touch and mouse drags follow React's down/right directions, axis lock, damping,
threshold and reversal cancellation. Button presses remain native actions.
The receipt protects controls below it while viewport padding and gaps remain
transparent. Drag-in-progress replacement, entrance interruption, native
pointer cancellation and multi-touch still need targeted checks.
The non-UI swipe checks execute the installed Base UI pointer handlers as the
reference, covering directional damping, thresholds, reversal, cancelled input
and interrupted entrance transforms. These checks do not replace native tests
of gesture cancellation or input handoff.

`DazzCalendar` accepts local `Date` values in single mode, or a `DazzDateRange`
with nullable `start`/`end` in range mode. It supports controlled `value` and
`month` callbacks or their `defaultValue`/`defaultMonth` counterparts, one or
two months, small/medium `controlSize`, bounds, locale and week start. A first
range press emits its start with a null end; the second orders the endpoints.
Native Buttons handle the cells and chevrons. Arrow keys, Home/End and
Page Up/Down move focus without choosing a day; Shift changes the page jump
to a year, and Enter/Space chooses the focused day. Outside-month days remain
pickable, while bounds block choosing without removing keyboard focus, as in
React. The implementation uses local date arithmetic across daylight-saving
changes, and all lengths and colors come from ds. Under system sizing, day
cells use the resolved native compact or normal control height.

`DazzDateRangePicker` accepts a nullable controlled `DazzDateRangeValue`, or a
`defaultValue`, with local `Date` start/end values and `allDay`. It composes the
calendar, native-menu time lists, duration toggles and footer actions. Editing
does not publish a value until Done; Cancel, Escape and outside dismissal
discard it. Reopening reads the current committed value. Moving the start time
carries the end by the same duration; an end before the start pulls the start
back an hour. All-day mode retains both times. Custom presets take a label and
a `range(today)` callback; durations take a label and minutes. `timeStep` must
be positive and finite, and an off-step current time remains in its list.
Labels, locale, week start, one/two months, field size and error state follow
the React API. The all-day summary retains React's fixed `All day` wording.
Touch opening focuses the popup itself; keyboard and mouse opening focus the
previous-month button. Tab enters its controls, reverse Tab can return to the
trigger while leaving it open, and traversal beyond the first/last stop closes
it and continues in the page. A nested time list or icon hint handles Escape
before the picker does.

Native Button and IconButton also accept glyph names through `icon`, and Button
accepts `trailingIcon`. The library owns their profile size, label shrink,
edge inset and live state color; callers do not compute the Quick pick glyph's
geometry. Existing builder slots remain available. A scoped numeric font
feature gives calendar dates and the picker sheet tabular digits.
IconButton automatically exposes its accessible label as a Tooltip. When a
Tooltip also wraps that button, the inner hint owns its label and placement;
hover and keyboard focus produce one hint.

`DazzDrawer` accepts an edge (`side`), `title`, `description`, `trigger`,
`content` and an optional `footer` builder. `extent` is React's `size`: a
logical length or percentage, applied as width on the sides and height at the
top or bottom. Its default is `drawer.size`. `DazzDrawerClose` wraps a control
through its `content` builder, or supplies a button using its `label`.
Headers, body insets and footer materials reuse the dialog drawing; the
drawer footer aligns its controls to the start. Only inward-facing corners
are rounded.

`DazzTree` accepts typed `DazzTreeNode` items, controlled `selected` and
`expanded` values with their callbacks, or `defaultExpanded` for local expansion.
Each node has a string label, optional Fluent glyph and explicit glyph size,
plain metadata or a `DazzTreeBadge`. Present `children`, including an empty
array, marks a branch. The row selects and toggles; its disclosure column only
toggles. Native button focus scopes keep the tree at one Tab stop while arrows,
Home/End, Enter/Space and visible-row type-ahead follow React's rules. The shared
`tree.typeahead-window` token owns the prefix interval. API 20's key Unicode
field supports basic Latin printables; composed non-Latin type-ahead is not
implemented. The SDK has no tree/tree-item accessibility role: native list roles
carry selection, with level, sibling position, expansion and unavailability in
the row description. Screen-reader hierarchy equivalence remains unverified.

`DazzSidebarProvider` owns controlled `open`/`onOpenChange` or local
`defaultOpen` state. Compose `DazzSidebar`, its Header/Content/Group/Menu parts,
`DazzSidebarInset` and `DazzSidebarTrigger` through builder slots. A column can
slide away, collapse to icons or stay open, with sidebar, floating and inset
surfaces on either side. Menu buttons own native focus and activation; selected
rows and sibling counts share a row snapshot. Supply that snapshot's `active`,
`variant`, `tint`, `disabled` and `hasBadge` options on `DazzSidebarMenuItem`
when composing a button and a badge together. The scene interpreter derives
those options from the React button so callers do not duplicate story state.

Ctrl+B toggles each mounted provider in the same native window, as React's
window listeners do. The theme scope registers the native shortcut so it also
works from text editors; bubbling key handlers cover provider and dialog
surfaces. A recording ShortcutRecorder consumes the chord first. Pointer
triggers affect only their own provider. Collapsed icon labels use a popup
without an input-blocking mask. System sizing expands the icon column as needed
to contain the native-sized button and its side insets.

`DazzPreviewCard` takes `trigger` and `content` builders, controlled
`open`/`onOpenChange` or `defaultOpen`, `side`, `contentAlign`, `contentWidth`
and optional millisecond `delay`/`closeDelay`. Shared ds timing defaults match
Base UI (600/300 ms). It uses ArkUI popup placement and dismissal, with native
hover/focus intent and a cursor-origin safe polygon between the target and
popup. The polygon uses the actual placed side, including a flipped popup,
with Base UI’s velocity threshold and intent timeout. The content
can own interactive controls; the popup opens without taking focus.
`DazzPreviewAnchor` forwards hover/focus/dismissal for a virtual text target.
The gallery retains native spans for inline previews, then measures their
UTF-16 ranges after layout to position a separate keyboard target. Copying the
paragraph includes its link label, not the preview body.

Built-in text/search/query editors report native pre-edit text through the
theme's `dazzui.editing` context, so PreviewCard leaves Escape to an active IME.
Custom native editors can consume `DazzEditingContext`, call `preview(id,
previewText?.value ?? '')` from `onChange`, and `clear(id)` on blur/unmount.
ArkUI cancels mouse hover during a touch. A press inside the card retains the
interaction through focus handoff; real mouse movement, popup blur or dismissal
releases it. Native hide notifications do not repeat an already proposed close
while a controlled host's accepted value is still propagating.

Paragraphs inherit the gallery's body-medium role; the shared prose popup
surface overrides it with body-small. `Text strong` changes only the weight,
as React does. Popover and PreviewCard share padding, hairline, corner and
elevation; a small badge uses React's 1.4 line height. Popup borders contribute
outside their content padding, matching CSS box geometry. Native verification
and remaining interaction gaps are recorded below.
The native popup owns the rounded silhouette and first shadow layer; painting
that shadow on its content would clip it to a square at the popup boundary.

The native modal owns the mask, Escape, focus containment and focus restoration.
Native swipe recognition dismisses toward the drawer's edge. The body scrolls
independently of its header and footer; a vertical scroll hands outward gestures
to the drawer only at the matching content edge. Short bodies do not install an
unnecessary scroll gesture. The sheet's edge transition reads the shared motion
duration and Bezier coordinates generated by ds.

ArkUI owns text editing and IME, native control activation, radio grouping,
slider keyboard operation, tab selection, menu positioning and modal lifecycle.
Drawing reads ds metrics and recipes. Native
radio/checkbox marks, switch rendering and splitter handles still retain
platform details. The port has not established desktop geometry conformance.
Toggle, ToggleGroup, Accordion and Collapsible use native button activation.
Group state stays scoped; forms register their editors for validation and reset.
Popup slots forward native activation explicitly, and dialogs inherit the live
theme scope. Nested providers use ArkUI’s explicit override option.

`DazzShortcutRecorder` is controlled by `value` and `onValueChange`; an optional
`onClear` overrides clearing to the empty string. It records physical key codes
so a keyboard layout's modified text cannot replace the key name. Held modifiers
print in React's order; unmodified typing is rejected, while F1–F20 work alone.
Escape cancels, Backspace/Delete clear, and blur stops listening. The theme scope
observes native touch and mouse presses, including noninteractive backgrounds;
a native click observer also covers controls in modal surfaces. Both cancel a
press outside the recorder without consuming the target's action. Observers are
removed when recording stops or the component unmounts. Native buttons
own focus and activation, including the separate clear button. While recording,
key events are consumed so Tab and Escape do not reach an enclosing surface.
The rejected-key feedback duration lives in ds and is shared with React.

Put stateful modal content in its own `@Component`, as the scene interpreter
and `ModalVerification` do. In the API 26 SDK check, an outer page's
`@LocalBuilder` updated built-in Text after its state changed but did not forward
the new primitive `@Prop` value into a custom child in the dialog. A reference
parameter did not resolve that case. Keeping the state with the modal content
preserves normal controlled callbacks; outer-page builder forwarding remains
a limitation of the verified configuration.
This also applies to a modal form's forwarded content: `OtpEditableSpecimen`
owns its controlled values below the `DazzForm` builder boundary.

`DazzOtpField` supplies numeric code cells with `length`, optional `groupAfter`,
`controlSize`, `value`/`defaultValue` and `onValueChange`. Native text editors own
IME, selection, clipboard menus and API 20's one-time-code input hint. The group
has one Tab stop; arrows and Home/End move within its entered prefix, typing
advances, and Backspace/Delete keep the code contiguous. Accepted edits advance
the logical cursor and native focus together, so IME events already queued for
the previous editor still reach the intended next cell.
Controlled edits also chain pending proposals while ArkUI forwards the host's
next value. That draft expires after rendering; cells always draw the host's
accepted value, and completion waits for the matching update.
`onValueComplete` and `onValueInvalid` report completed codes and rejected text.
`required` and `autoSubmit` integrate with `DazzForm`, including reset.
The public mapping currently supports numeric validation; React's other
validation modes and custom normalizers are not advertised by this port.

Each cell draws its border on a fixed outer box. On the checked SDK, applying a
custom border directly to TextInput enlarged its requested dimensions; a native
editor inside the box preserves both native editing and the shared size profile.
The error state selects the danger focus tint; form validation supplies the
invalid fill and border, matching the current React OTP stylesheet.

## System fonts and sizing

`DazzTheme.sizing` is independent of `name`:

```ts
DazzTheme({ name: 'studio-dark', sizing: 'system', content: this.content })
```

The default, `design`, keeps the generated desktop geometry and type sizes.
`system` reads the current UIAbility configuration and public system typography,
button and text-field resources. Dimension resources are converted from physical
pixels to logical values before native font scaling. It uses text
measurement to resolve font sizes and minimum control heights; spacing, icons
and available space grow with the effective body-font scale. Compact controls
use the compact height and text role. Public button padding is shared between
compact and normal controls because the native compact padding is private.
Corners, strokes, motion and ds color recipes stay with the selected design.
This is a runtime adaptation, not a pixel-conformance claim for native controls.

Font-family selection and fallback remain ArkUI's responsibility, so changing
the system face also reaches inputs; only explicit code text uses monospace.
`readSystemStyle(uiContext)` returns a fresh snapshot with `fontId`,
`fontScale`, `fontWeightScale`, configured generic families and resolved metrics.
The generic-family list describes the system font configuration, not which
family is currently selected. ArkUI applies system weight scaling itself.
`DazzTheme` observes environment changes and disposes its subscription on unmount.
Resolved token fonts are applied in vp, preventing a second native font scale.
For host-authored text using token sizes, use
`.fontSize(tokens.fontSize(tokens.type('bodySmall').size))` too.

The host must enable system font following, separately from its color theme.
In `AppScope/app.json5`, set `app.configuration` to
`"$profile:configuration"`. Create
`AppScope/resources/base/profile/configuration.json`:

```json
{ "configuration": { "fontSizeScale": "followSystem" } }
```

The published HAR excludes the gallery, tests, scripts and build caches through
`.ohpmignore`. The gallery includes this configuration and a **Sizing** selector alongside
its theme selector. A launch Want can select `sizing=system`. Avoid calling
`ApplicationContext.setFontSizeScale` for this option: that API overrides the
app's font policy and stops following system changes.

After installing the gallery, `pnpm harmony:test:system` exercises the native
font test page: system-following startup, scale changes, a fixed design scope nested inside an adapted scope,
and a color-theme switch without a size change. It uses process-local scale
overrides only in the test page, then restarts the normal gallery to restore
system following. Set `HARMONY_DEVICE` when multiple PCs are connected.
Pass an output directory after the command to save 1x, 2x and recolored 2x
native screenshots with their measured report.

## React stories and coverage

The example mirrors React’s three-column shell with native resizable splitters,
a searchable component rail, wrapping Overview previews, component pages with
notes, and a grouped three-column theme panel. Toolbar buttons toggle both
sidebars. Initial sidebar widths and limits follow React’s shell. Its stories are compiled from
`packages/js/dazzui/example/src/stories/` by `tools/codegen/harmony.mjs`.
The typed scene interpreter preserves state setters, event arguments,
comparisons and specimen addresses without dynamic reflection or `any`.
Menu and Select share native MenuItems with a common custom drawing: quiet
labels, compact row padding, check/icon columns, shortcut hints, concentric
highlight corners and the neutral normal recipe. Menu panels measure intrinsic
content; Select panels also match their trigger width. Select placeholders use
faint ink, disabled fields use the normal disabled recipe, and an open field
keeps its focus surface, border and glow even while focus is in the menu.
Its noninteractive painted face sits beside the native Button, outside the
native disabled-opacity effect. The native Button retains its semantic label,
focus, disabled state and menu anchor; the painted face is excluded from
accessibility. A screenshot check verifies the actual disabled fill and text
ink rather than inferring them from the submitted opacity attribute.
Select preserves native option groups, with compact noninteractive captions
that do not enter the keyboard order. Menu checks use the reference radio-group
rule: the first supplied checked item is selected. Menu, Select and context
menus draw `shadow.lg` behind a native foreground Scroll, with a fixed viewport
border. The real Menu and background are siblings, allowing the native
composite host to leave the outer shadow unclipped. Native MenuItems retain
focus and activation; their focus callbacks reveal the row through the
foreground Scroller. The painted silhouette rounds the measured border box
to physical pixels, matching the native foreground.
Pointer focus follows real mouse movement. A stationary pointer does not take
focus back when keyboard scrolling moves another row underneath it.
ArkUI closes its menu host when the window resizes. Ordinary menus and
touch-opened Selects restore their logical open state, scroll offset and
focused row after that native dismissal. Mouse/keyboard Selects follow the
reference's trigger-aligned resize dismissal policy. Queued reopening is
cancelled when the owning component disappears. This establishes settled
interaction state; continuous-resize frame paint still needs verification.
The key adapters add Home/End and prefix search while native items keep arrows,
Enter/Space and dismissal. Open-list search moves focus before confirmation;
a closed Select applies its match directly. Prefix sessions follow the reference
toolkit's timing and repeated-initial cycling, and skip disabled choices.
Closed Selects also open with Up/Down/Enter/Space, restoring the selected item
or focusing the first enabled item when unselected. A native item's confirmation
can run before its bubbling key callback; that same command and its release
must not be interpreted as another trigger activation. Late appearance
callbacks cannot reopen a list that already began closing.
API 26's native target-gap option uses `spacing.1`; older runtime placement
remains native and has not been verified.

Menubar uses a scoped native focus group with compact, intrinsic-width labels,
the shared gap and neutral hover/open wash. Arrows wrap and skip disabled
labels; Home/End select endpoints. Down/Up open at the first/last enabled
action, lateral popup arrows switch menus, Escape returns to the active label,
and Tab leaves the composite after the native popup disappears. Independent
bars keep separate active menus and focus memory. `modal` defaults to `true`;
`false` allows outside controls to act while dismissing the menu.
The native modal host routes pointer movement and presses within its owning
bar through public FrameNode events. Disabled labels and bar gaps resume the
menu after ArkUI's earlier outside-dismiss callback and a rendered frame,
restoring its last focused row. This preserves the final interaction state;
frame-by-frame paint of the dismissal/resume bridge still needs verification.
Nonmodal menus use a native subwindow so outside controls receive the original
press. Disabled-label presses, keyboard and same-label toggle paths in that
mode still need separate checks.

Combobox uses a native editor and an anchored popup, with grouped filtering, clear, Escape and arrow/Enter
selection. Autocomplete accepts text outside its suggestion list. Segmented
controls draw label glyphs, icon-only segments, and raised/filled selection
using the same ds padding, concentric corners and shadow as React. Unsized
story rows and columns use `LayoutPolicy.wrapContent` (API 20), preserving
React’s intrinsic widths and centered stage layout; explicit story widths
remain unchanged. Pill counts use a real token gap and an intrinsic tab strip.
Document cards include their large padding within the 160 vp minimum, as in
React; the minimum is not applied to an inner column and then padded again.
Overview previews keep their 128 vp inner-stage minimum.

Run `pnpm codegen` after story changes, and `pnpm codegen:check` to detect
drift. `example/coverage.json` records every unmapped scene and its exact
component/prop/value gaps. Scenes with any gap display that list in full;
they never draw a misleading subset. The initial port stays outside the
codegen full-support gate, like GTK4.

The current mapping covers all 176 portable scenes. This is source mapping
coverage, not a full-support or fidelity claim. See the generated coverage file for exact
scene coverage; a mapped scene alone does not prove geometry conformance.
Those counts cover only the portable stories. HarmonyOS additionally reads the
actual React-only Tabs, Calendar, DateRangePicker and WindowFrame stories through
`tools/codegen/harmony-react.mjs`. Their previews and twenty-four examples bring the
gallery to 55 pages and 149 examples. `example/react-coverage.json` tracks these
28 scenes separately. The portable grammar and conformance gate do not include
these extra scenes. WindowFrame draws presentation windows inside the gallery;
its controls dispatch callbacks and never operate the host OS window.

Inside `titleContent`, a custom title component can consume
`dazzui.window-title` as a `DazzWindowTitleStyle`. It carries the resolved
logical `size`, `leading`, `weight` and `color` used by a plain title and updates
when the theme or inactive state changes. Use vp for the resolved dimensions:

```ts
@Component
struct ReportTitle {
  @Consume('dazzui.window-title') style: DazzWindowTitleStyle = new DazzWindowTitleStyle()
  build() {
    Text('Report').fontSize(this.style.size + 'vp')
      .lineHeight(this.style.leading + 'vp').fontWeight(this.style.weight)
      .fontColor(this.style.color).maxLines(1)
  }
}
```

The gallery forwards this style through rich title rows and their text/icons,
preserving explicit strong text or color overrides. Windows rich titles use
the 12-vp platform size instead of body sizing; system sizing scales it once.
Title and subtitle retain the inherited body line height and baseline layout.
`window-title` measures 45 title and nine subtitle styles across Studio
light/dark and Bright light; their size, weight, line height and ink match the
actual React story in a browser. It also checks the native image's color and
size in those three palettes. These are attribute/geometry
checks, not full typography paint conformance.
The Windows rich title and its secondary body text also have lossless native
ink probes. Browser-computed ink and surface colors define the allowed channel
ranges; probes require visible glyph contrast and reject pixels outside those
ranges by more than two byte levels. They reject the previous black-span
renderer even when its parent Text reports the correct font color. Ordinary
spans now receive their ink explicitly; links keep their own recipe and inline
Code keeps its own content ink. These probes do not compare glyph shapes or
antialiasing between the two font engines.
The scope belongs only to the heading; leading, trailing and toolbar controls
keep their own typography. A live check changes appearance and family without
restarting the gallery, verifying title/glyph ink and retained Pin state.

```sh
node tools/harmony/gallery-test.mjs --checks window-title
node tools/harmony/window-title-reference.mjs
node tools/harmony/gallery-test.mjs --checks window-title-live
```

Copy native captures from the PC host before running the reference tool where
the React example's Vite, Playwright and Chromium dependencies are installed.

`pnpm harmony:test:device` starts each of the 55 generated component pages,
scrolls through every example and checks the three shell columns. Restored
families must contain no mapping-gap placeholder. It also drives accordion
selection, clipboard copy, popover/dialog opening and dismissal, form required
validation and reset, and sidebar toggles through the native accessibility tree.
The suite maximizes only its own app window before injecting input, so other
desktop windows cannot obscure controls. It waits for restored window geometry
to settle. This suite requires an installed gallery on a PC/2in1 target; set
`HARMONY_DEVICE` to select it. `--interactions-only` on
`tools/harmony/gallery-test.mjs` narrows a diagnostic run to interactions.
Use `--pages select,combobox --renders-only` for a page sweep or
`--checks select,combobox` for selected interactions. The `window` check
restores/maximizes the native window, checks that the sidebars retain their
widths while the content fills the remaining space, verifies that story state
survives, and drags a native splitter.
The `drawer`, `drawer-swipes` and `drawer-keyboard` checks exercise the real
React scenes on every edge, including header/body/footer geometry, switches,
Escape, the mask, keyboard closing and focus restoration. `drawer-overflow`
uses a test-only native page to verify a 40-row body, fixed footer, scroll-edge
handoff, omitted footer and percentage sizing. It is not a gallery story.
`shortcut-background` checks touch and mouse cancellation on plain document
text while preserving an internal press. `shortcut-modal` checks modal keyboard
containment, controlled updates and outside actions.
`otp-field` measures the drawn cell boxes and grouping gaps, then checks native
keyboard editing, rapid typing and one-stop Tab traversal. `otp-modal` checks
real clipboard paste, numeric filtering, controlled callbacks, form submission
and reset, read-only/disabled cells and a host that refuses a proposed value.
`otp-theme` switches the live code to dark appearance and back, checking that
the palette changes while values, cell dimensions and position remain intact.

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

## Verification and remaining gaps

Verification uses `macbook-pro`, DevEco/Hvigor 6.26.8 and the bundled API 26
SDK. The package retains API 20 compatibility. The desktop HAP has built,
installed and run on the PC/2in1 emulator; the mobile HAP also runs on the phone
emulator. These are emulator results, not physical-device or full paint
conformance results. The standalone HAR is checked separately from the HAP.

To build, install and launch from the configured host:

```sh
fastforge --no-version-check host exec macbook-pro -- \
  '"/Applications/DevEco-Studio.app/Contents/tools/node/bin/node" packages/arkts/dazzui/scripts/run.mjs'
```

`pnpm harmony:test` checks the token generator, story compiler and actual
non-UI ArkTS logic. Reference comparisons execute the installed Base UI code
and React helpers; these tests do not compile ArkUI. SDK builds and the native
interaction suite provide separate evidence.

Button, WindowFrame, Dialog, Toast, Sidebar, Menu and Select use ArkUI's native Drawing Gaussian image filter.
A pointer-transparent RenderNode paints behind the foreground, with enough
canvas space for the blur and offset. Its padding is rounded to physical
pixels; sRGB float colors avoid losing precision in tinted shadows. No image
files or web renderer are used by the component.
The filter receives an opaque silhouette, and the shared shadow alpha is
applied when that layer is composited. This preserves low-alpha tint precision
without changing the token values or the pixel tolerance.
The non-touchable canvas clips to its expanded allocation so its overflow does
not extend a receipt's hit region into viewport padding or inter-receipt gaps.

An enabled filled Button draws its lift behind the native foreground, whose
own shadow is cleared. Independent corner radii retain joined group edges.
The layer follows the measured border box while the native Button keeps focus,
activation and layout. Disabled, normal, IconButton and Toggle controls remain
flat. Shared prose popups let their native host cast the shadow beyond its
content clip; this path does not require extra layout or response regions.

The targeted pixel check renders an isolated 300-vp window, captures a lossless
SDK screenshot and measures its actual bounds. Four straight edges are compared
with the Gaussian convolution and colors from the committed shared token graph.
The limit is two channel byte levels at any sample and one RMS byte level.
It rejects the older narrow native shadow-mask profile and a missing shadow.
This is narrower than full screenshot or rounded-corner conformance.

macOS presentation traffic lights also use native Drawing for the circular
fill, inside black ring, and modern gloss and white inset. The native Button
retains activation and accessibility; a separate paint layer avoids applying
its disabled opacity to an already resolved surface. Every glyph and paint
boundary excludes input. The hover group spans only the lights and their gaps.
The paths and viewport share the SVG's physical-pixel coordinate space before
scaling to the logical icon size.

The reference check runs the actual React WindowFrame at the captured density.
It reads the drawing node's actual paint rectangle rather than assuming that
fractional native layout coordinates are its raster origin. It compares the
material outside a 0.75-physical-pixel boundary antialiasing band, excluding the
central glyph under hover, with the same two-byte maximum and one-byte RMS
limits. A separate ink-centre check permits one physical pixel of native
allocation rounding. Complete glyph antialiasing and pressed motion remain
separate checks.

After installing the gallery, run on its connected PC host:

```sh
node tools/harmony/gallery-test.mjs --checks shadows,shadow-lifecycle --paint-output packages/arkts/dazzui/example/entry/build/verification/paint
node tools/harmony/gallery-test.mjs --checks window-control-paint --paint-output packages/arkts/dazzui/example/entry/build/verification/window-controls
node tools/harmony/window-controls-reference.mjs packages/arkts/dazzui/example/entry/build/verification/window-controls
node tools/harmony/gallery-test.mjs --checks dialog,dialog-paint,dialog-overflow,dialog-scenes --paint-output packages/arkts/dazzui/example/entry/build/verification/dialog-paint
node tools/harmony/gallery-test.mjs --checks toast-paint --paint-output packages/arkts/dazzui/example/entry/build/verification/toast-paint
node tools/harmony/gallery-test.mjs --checks sidebar-paint --paint-output packages/arkts/dazzui/example/entry/build/verification/sidebar-paint
node tools/harmony/gallery-test.mjs --checks sidebar-labels,sidebar
node tools/harmony/gallery-test.mjs --checks popup-paint --paint-output packages/arkts/dazzui/example/entry/build/verification/popup-paint
node tools/harmony/gallery-test.mjs --checks menu-paint --paint-output packages/arkts/dazzui/example/entry/build/verification/menu-paint
node tools/harmony/gallery-test.mjs --checks menu-scroll --paint-output packages/arkts/dazzui/example/entry/build/verification/menu-scroll
node tools/harmony/gallery-test.mjs --checks menu-viewport --paint-output packages/arkts/dazzui/example/entry/build/verification/menu-viewport
node tools/harmony/gallery-test.mjs --checks select-opening
node tools/harmony/gallery-test.mjs --checks control-shadow-paint --paint-output packages/arkts/dazzui/example/entry/build/verification/control-shadow
node tools/harmony/paint.mjs packages/arkts/dazzui/example/entry/build/verification/paint
```

`window-control-paint` captures modern and flat macOS chrome in Studio
light/dark and Bright light, with enabled, inactive, disabled and hidden
controls. It checks that moving above the lights leaves glyphs hidden,
hovering reveals only enabled glyphs, and only enabled controls invoke actions.
The React reference tool runs where the React example's Vite, Playwright and
Chromium dependencies are available; copy native captures there when using a
remote PC host.

The API 26 run on `macbook-pro` captured 42 images and compared 108 control
results. All native state/action checks and glyph-centre checks passed;
105 control results met the material limit. Three dark grey close-light
results reached a maximum error of three byte levels, above the unchanged
two-byte limit. That material gate remains failing; it is not full paint
conformance. Window geometry, slots, accessory/workspace interaction and
two same-process shadow unmount/remount cycles passed separately.

The `shadows` check captures Studio light/dark and Bright light, for active and
inactive windows, plus a shadowless Omarchy window. `shadow-lifecycle` navigates
away and back twice in one process and checks that remounted drawing nodes
paint outside the window bounds. `shadow-paint` saves additional diagnostic
captures; `--paint-theme` selects a theme. Captures are measurements, not
baselines.

`dialog-paint` opens real native dialogs over an otherwise solid page. The same
edge/corner check includes the shared dialog scrim in its expected backdrop, in
Studio light/dark and Bright light. `dialog-overflow` scrolls a long body to its
final action, checks that the header and full-height footer stay pinned, and
closes it through that footer. The `dialog` check also keeps short forms at their
natural height. Address wrappers around fixed modal chrome must preserve
`flexShrink(0)`; the generic scene renderer does so.
`dialog-scenes` opens all five generated React dialog examples, checks their
header/body/footer bounds and closes each through its actual footer action.

`toast-paint` checks short receipts and long receipts at the shared width cap in
the same three themes. The rounded-shape convolution is integrated numerically;
independent disk-mass and translation checks validate that calculation. Samples
cover four straight edges and four outer corner diagonals, skipping the tail
clipped by the native viewport and requiring samples on every profile. The
pixel tolerance remains two channel byte levels maximum and one RMS byte level.
These checks do not replace full surface, border, motion or all-theme paint
conformance.

`sidebar-paint` measures floating cards beside a real content pane, expanded,
collapsed to icons and restored, plus the inset pane over the column-colored
ground. It captures four
edge and four outer-corner profiles, records the independently queried native
frame, and exercises foreground controls and input outside the allocated
column. The default covers all four states in Studio light/dark and Bright
light; `--paint-theme`, `--paint-sidebar floating` and `--paint-sidebar inset`
isolate diagnostics.
The composition rejects a shadow hidden behind its peer pane. Main menu items
keep the leading edge, while badges use an independent trailing row and bind
their scene address to the actual painted label.

`popup-paint` captures the four native Popover placements and a real PreviewCard
in Studio light/dark and Bright light. The unchanged Gaussian edge/corner gate
compares the pixels with `shadow.lg`, measures the visual anchor gap, exercises
the foreground button and closes through Escape. The flat native trigger
leaves a uniform backdrop beneath the blur. `--paint-theme` and `--paint-side`
(`top`, `bottom`, `left`, `right` or `preview`) isolate a diagnostic case.
Popover cases also reopen and dismiss through a press in the visible shadow,
which must remain outside the popup's response region.

`control-shadow-paint` checks an actual filled Button at its normal size in
Studio light/dark and Bright light. Its small shadow is sampled pixel by pixel
outside the unfocused border box, using native floating-point geometry rather
than a rounded accessibility rectangle. Every profile still requires at least
eight samples; the maximum and RMS error limits are unchanged. No screenshot
magnification or design-value adjustment is used by this check.

`menu-paint` captures Menu, Select and right-click menus in Studio light/dark
and Bright light, on first open and reopening. It checks the four straight
edges and four outer corners against `shadow.lg`, exercises the foreground
action, dismisses from the visible shadow and rejects residual paint after
closure. Context-menu bounds come from the public native window element dump;
main-window captures use the painted accessibility border box.
`menu-scroll` uses sixty native rows to check clipping and the viewport shadow,
then exercises arrows, Home/End, confirmation and selected-value reopening.
Long-list captures explicitly report profiles clipped by the window viewport
and compare every visible sample at the unchanged error limits. Short-list
captures still require eight samples on each of all eight profiles.
`menu-viewport` exercises real mouse-wheel and touch scrolling without row
activation. A delayed public Window resize changes the real floating-window
constraints while the menu is open. Checks cover shrinking and growing,
viewport-shadow pixels, clipping, retained scrolling and focus, subsequent
Home/End and mouse-opened Select dismissal without changing its value.
`select-opening` checks all four opening keys in unselected and selected lists,
including key release without selection and Escape returning to the trigger.

`sidebar-labels` checks short and long labels at all three row sizes in both
design and system sizing. Labels stay before the trailing count, preserve their
leading edge when updated, and fit inside the button. Counts and labels hide
in icon mode, the glyph still activates the row, and the labels return without
overlap. The same check measures all four generated gallery variants in both
sizing modes. Native text receives the remaining row width; the fixed glyph
and the existing badge reserve do not compete with its intrinsic width.

`tooltip` checks focus without moving focus into the hint, Escape, refocusing,
presses through the label, touch activation and disabled hints retaining the
original control's action. `tooltip-paint` captures four directions in Studio
light/dark and Bright light; `--paint-label multiline` adds a two-line label.
`tools/harmony/tooltip-reference.mjs` compares the captured font, border, padding,
height and gap with the actual React component. The native Text padding includes
the border inset that CSS reports separately; each native text line receives
the shared line height. Tooltip's passive overlay and Gaussian drawing layer
do not receive input. Its shared hover-rest and close delays follow the installed
Base UI defaults, and Sidebar scopes preserve React's zero hover delay.
`tooltip-gallery` exercises all five generated triggers with their actual labels
and sides; Escape dismisses a hovered hint before keyboard focus has been
activated, and a stationary pointer does not immediately reopen it.
`tooltip-nesting` also checks an explicit Tooltip around IconButton: the inner
label/placement wins, one Escape dismisses it, and Enter/touch still activate
the control once. Keyboard activation is observed before native Buttons consume
the key. `tools/harmony/tooltip-nesting-reference.mjs` exercises the actual React
components, including removal/restoration of the inner hint under StrictMode.

`slider-paint` captures all three sizes, both endpoints, a range and a disabled
control in Studio light/dark, Bright light and Omarchy Tokyo Night dark.
`--paint-state rest|hover|focus` selects the interaction state.
`tools/harmony/slider-reference.mjs` renders the actual React Slider with the
example's stylesheet reset and checks the full track and thumb regions without
translating the captured pixels. Best-fit translations diagnose displacement;
they cannot turn a failed raster comparison into a pass. The strict maximum
two-byte / RMS one-byte raster gate still reports edge antialiasing differences.
`slider-pointer` holds one native mouse-input process through each drag and
checks updates before release, capture beyond the track and dragging from the
outer half of an endpoint thumb. Native input remains responsible for gestures,
focus and accessibility; the ds drawing layer has no response region.
React's thumb ring follows its nested range input's `:focus-visible` state;
keyboard Tab and pointer/rest checks exercise the real focus owner.

`tabs-live` shrinks and grows one mounted header, selects hidden tabs, changes
labels, counts and trailing width, and removes/restores a busy item. Surviving
tabs retain their native button identity while their visible content updates.
Explicit, compact-fallback, empty and disabled hover titles use native system
hints. Both implementations reserve trailing margins and measure active labels
at their strong weight. `tools/harmony/tabs-reference.mjs` compares the captured
inputs, visible text, selection, overflow and clipping with the actual React
component. Seven of eight fixtures pass; the 200 vp promoted-tab case retains
one extra native icon tab, so the reference check still fails at that boundary.
The native system font and the browser's system font have different advances.
`--font-file <path>` adds a diagnostic comparison using the matching device font
in the browser. It writes `same-font-reference-results.json` and separate images;
the default Mac-font report is retained. All eight current fixtures pass with
the emulator's HarmonyOS Sans file, including the 200 vp promoted tab. The
original Mac-font comparison remains seven of eight. The report records the
font's SHA-256 and verifies that the visible tabs and rulers use it. Font files
remain in ignored verification output rather than the library or gallery.
`tabs-pulse` samples actual status-dot screenshot pixels through running,
stopped, restarted and remounted states. The lifecycle driver discards stale
animation completions; each half-cycle uses the forward ds easing, as CSS does.
These checks do not establish exact animation trajectories, all-theme paint or
assistive-technology conformance.

`tabs-actions` checks close and overflow hints, their live icon/button colours,
Enter activation, menu return, controlled close refusal, focus after removal,
and forward/reverse Tab with and without close controls. The first Tab after
touch advances in the same press because keyboard mode activates before native
traversal. Removed focused controls clear their focus before ArkUI can choose
another tab; refused removal retains it. The glyph receives an observed
interaction object so it repaints alongside the button background through the
forwarded builder. `tools/harmony/tabs-actions-reference.mjs` runs the same
sequence on the actual React component and checks native screenshot glyph ink
against its computed colours. All 22 states agree on values, selection, focus,
hint labels, menu state, action colours and glyph sizes; all 20 visible-glyph
captures pass the colour-range check with at most two channel levels of error. Touch opening retains the
trigger's hover wash until another target is touched or the mouse moves, as in
the reference. Detached menus reset that hover without fabricating an outside
press for other overlays. These checks leave the 200 vp system-font fitting
failure above unchanged, and do not establish transition trajectories,
all-theme paint or assistive-technology conformance.

`keyboard-actions` separates physical key down, repeated down and key up on
Button, IconButton and Tabs close/overflow controls. It also changes focus while
Space is held. Space arms a control until release and blur cancels that action;
Enter retains down/repeat activation without a persistent pressed face. Ordinary
buttons activate keyboard mode before native Tab traversal, so the first Tab
after touch advances. A native uinput process retains each held key through its
release; independent release processes cannot reproduce that event correctly.
`tools/harmony/keyboard-actions-reference.mjs <capture-directory>` compares
31 checkpoints with actual React controls, including action counts, focus,
menu visibility and settled background colours. This does not establish
transition trajectories, focus-ring raster identity, all-theme paint or
assistive-technology equivalence.

`button-pointer` checks 27 held-mouse states on Button, IconButton and Tabs
close controls. A mouse press focuses the native button immediately; its
pressed paint survives movement outside and back until release. Releasing
outside cancels activation. Native buttons still own pointer clicks. A scoped
release observer clears paint even when release occurs beyond the target, and
unsubscribes on release, blur, disabling or removal. Run
`tools/harmony/keyboard-actions-reference.mjs <capture-directory> --pointer`
to compare counts, focus, menu visibility and settled colours with actual React.
All 27 states agree; the pre-fix capture agreed in 12. The reference reads
straight computed sRGB channels rather than recovering translucent colours
from a premultiplied 8-bit canvas.

`button-focus` captures the actual keyboard-focused button in Studio Light,
Studio Dark and Bright Light. The default system focus box is suppressed so it
does not paint an extra opaque blue ring over the ds outline.
`tools/harmony/button-focus-reference.mjs <capture-directory>` compares four
straight edge profiles with actual React at the captured density and border-box
size, independently of platform font advances. A passive native drawing layer now snaps the outline rectangles to the
reference's CSS layout grid and normalizes their corners before filling the
ring. It caches software-rasterized paint, refreshes it when layout or paint
inputs change, and releases the bitmap on replacement or disposal. The native
button retains input and accessibility. Across the three captured palettes,
RMS falls below one and maximum channel error falls from 80–118 to 5–6.
The strict maximum-two / RMS-one checks still fail at a few boundary pixels.
Paired unfocused captures expose differences in the underlying button edge,
independently of focus paint. Their complete
profiles and screenshots remain recorded; rounded corners, other input states
and complete focus-ring raster equivalence remain open.

`font-metrics` captures the public UI font configuration, resource family,
font ID/scales and pixel density, alongside measured and actual Text widths.
`tools/harmony/font-metrics-reference.mjs <capture-directory> [font-file]` checks
that capture against the matching font file. All eleven native samples agree
with their Text layout. The ten Latin samples also match browser advances after
the observed native physical-pixel ceiling at this emulator's density. The CJK
sample verifies native measurement only; the Latin file does not establish
Chromium's CJK fallback. These diagnostics explain this fit difference without
changing production typography or weakening the original reference check.

`calendar-live` checks one mounted calendar through month changes, one/two-month
Tab traversal, accepted/refused controlled navigation and pending range previews
moving between mouse and keyboard. Every date's native label and Tab index are
read from the running component; focus and selection come from the native UI
tree. `tools/harmony/calendar-reference.mjs` performs the same actions against
the actual React component. All nine captured states agree on month titles,
controlled values/proposals, date labels, selection, focus and Tab indices.
Both implementations now fall back to the first month's first day when today
and the last focused date are outside the visible months. An outside leading
grid cell cannot remove the grid's only Tab stop. Screen-reader announcements
and full paint conformance remain separate checks.

`date-range-live` compares nineteen states with the actual React component using
`tools/harmony/date-range-reference.mjs`: accepted/refused controlled values,
draft cancellation and reopening, touch/mouse/keyboard opening, nested-menu and
tooltip Escape handling, focus return and forward/reverse Tab traversal. Native
control names, selected dates, time labels, summaries and focus owners agree.
The popup's real body receives touch focus; its parent scope forwards boundary
Tab events through the page so native traversal reaches the next control.
Narrow fitting, separator/stretch paint, modal handoff and rapid remount/input
sequences remain separate checks.

The native checks completed during the alignment work include:

| Area | Checked on the API 26 PC emulator |
| --- | --- |
| Gallery | The 51 portable pages and 125 example containers; all overview previews; padded stage dimensions and centering; search binding; both shell panels; splitter widths surviving restore/maximize. The four additional React-only pages have targeted checks below. |
| Button paint | Filled Button shadow pixels match the shared small-shadow recipe at actual scale in Studio light/dark and Bright light. The native foreground has no duplicate shadow; disabled and normal buttons and filled IconButtons stay flat. A text Toggle changes to the strong label weight when latched, restores its profile weight when released, and stays flat. |
| Dialog | Overlay fill, rounded content clipping, short-form sizing, activation and Cancel dismissal in Studio light/dark and Bright light. Real dialog Gaussian shadow pixels pass the straight-edge and outer-corner check over the shared scrim in those three themes. A long body scrolls to its final action while the header and full-height footer stay pinned; the sheet leaves the shared fixed viewport padding. All five generated examples preserve their header/body/footer bounds and dismiss through their footer actions. |
| Toast paint | Native Gaussian shadows pass the four straight-edge and four outer-corner profiles for short and width-capped long receipts in Studio light/dark and Bright light. The capped message preserves both control gaps, its action fires once, and action/dismissal remove the live drawing scope. |
| Forms and choices | Required validation/reset; accordion selection; clipboard copy; Select groups and Home/End; Combobox/Autocomplete filtering, arrows, Enter, clear, empty results, Escape and free text; segmented/pill keyboard selection and one-stop Tab exit. |
| Menu and Select | Three-palette native row heights, typography, shortcuts, neutral pointer highlights, panel insets and intrinsic/anchor widths. Compact group captions preserve pointer selection, Home/End, arrow traversal that skips headings, and Space. Radio checks reserve columns only on checkable rows. Native prefix search, closed Select matching, Enter, right-click actions and trigger focus return have targeted checks. Gaussian edge/corner pixels, reopening, shadow dismissal and paint cleanup cover Menu, Select and context menus in three palettes. Sixty-row lists check clipping, visible viewport-shadow profiles, arrows, endpoints and selected-value reopening. Wheel/touch scrolling, resize shrink/grow, retained scroll/focus and stationary-pointer keyboard scrolling have native checks. Select's four opening keys cover first/selected focus and key-release containment; mouse and keyboard resize dismissal preserve its value. Assistive-technology semantics remain a separate requirement. |
| Menubar | One-stop forward/reverse Tab, remembered trigger, disabled skipping, wrap and endpoints, first/last opening, lateral popup switching, Escape and forward/reverse popup Tab. Modal hover, touch/mouse handoff, disabled labels and same-label toggle; modal outside consumption, nonmodal outside actions and independent bar scopes. Three-palette control heights, typography, gaps and open wash are measured from the source gallery. |
| Link | Forward and reverse Tab traversal through native text focus nodes, pointer activation, Enter activating once and Space not activating; the latter paths also run inside an editable preview popup. |
| Drawer | All four edges, 352 vp default thickness, switches, fixed footer, mask/Escape/swipe dismissal and trigger focus return; long-body scrolling and edge handoff, omitted footer and percentage sizing. |
| ShortcutRecorder | 132×28 vp resting field, modifier lifetime, invalid typing, chords/F5, Escape, deletion, pointer/keyboard clear, disabled/conflicting values, controlled modal edits and outside cancellation without consuming the outside action. |
| OtpField | 26/28/32 vp cells, grouping gaps, rapid typing, overwrite/deletion, native clipboard paste/filtering, one-stop Tab traversal, controlled form submit/reset, read-only/disabled/refused updates and appearance changes retaining values/geometry. Real SMS autofill remains unverified. |
| Tree | 240/320 vp columns and 24/28 vp rows; disclosure-only actions, selection, expansion, parent/child/Home/End/Enter/Space navigation, basic Latin type-ahead, disabled rows, one-stop Tab traversal, stable IDs, empty branches and controlled refusal. |
| Sidebar | Fixed/collapsible/icon/floating/inset geometry, live selection text, 172-to-48 vp collapse, right-side default/controlled/refused state, hover labels, window-wide Ctrl+B from an editor and recorder containment. Floating expanded/icon/restored cards beside a real pane, plus the inset sheet, pass the Gaussian edge/corner check in three themes. Foreground controls stay live and shadow overflow passes input. Main buttons keep the leading edge, badges keep their four-vp trailing inset and vertical centre, and scene addresses bind the painted count. |
| PreviewCard | Native inline text structure, 76 vp person card, 288×85.4 vp repository card, 4 vp gap, 19 vp border-inclusive inset, 21.4 vp small badge, mouse activation/travel, Tab focus and Escape; defaultOpen, accepted controlled open/close and refused opening, touch/mouse actions and input inside the popup, outside touch and Escape containment inside a dialog. Native paragraph copying still needs an end-to-end check. |
| Slider | Scalar/range rendering, native thumb focus, Tab and Shift+Tab, Home/End/arrows/Shift/Page increments, neighbour clamping, pointer pushing, controlled acceptance/refusal and disabled input. Refusal checks read the native Slider values as well as the displayed labels. |
| Toast | 38/40 vp notice/action heights, window-fixed positioning, 16 vp edge padding, actions/dismissal, provider isolation, persistent danger notices and ordinary timeout. F6 region entry, whole-receipt focus, forward/reverse Tab, action-button Enter, Escape, exact button/editor focus return, focus-paused timers, stacked source order, limited-item skipping and multiple-provider dispatch/return. Native touch and held-button mouse drags check direction, thresholds, return to rest, capture beyond the original bounds and action/dismiss button isolation. Held touches pause timers; release and touches in another provider resume them. Covered-control probes distinguish receipt blocking from transparent viewport padding and inter-receipt gaps. A modal probe checks transparent-area input, receipt pointer actions above its mask, window-edge positioning and return to DazzUI and raw native controls. |
| Tabs | Four authored header dimensions, hidden-item selection, a fully visible promoted History tab beside the overflow control, Home, selected-document removal and the source-authored trailing action. Live width/name/count/trailing changes, stable native button identity, native hover titles and actual busy-dot pixels through stop/restart/remount have targeted checks. Twenty-two action states match React, including close/more hints, controlled refusal, focus after removal, Tab in both directions, menu return and touch opening; twenty glyph captures match the source ink. |
| Calendar | 208×230 vp single and 404×218 vp two-month grids; single/range selection; day/week/month keyboard navigation; min/max bounds; Tab and reverse Tab; localized accessible date labels. Nine live states match the actual React component, including one/two-month fallback Tab entry, controlled month acceptance/refusal and mouse-to-keyboard range preview. |
| DateRangePicker | Default 404×276 vp popup and wider two-month layout, draft duration/Cancel/reopening, coordinated start/end times, all-day disabling, controlled Done, Escape/outside cancellation and custom presets. Nineteen live states match React, including controlled refusal, input-dependent opening focus, focus return, nested Escape and forward/reverse Tab traversal. |
| WindowFrame | All seven platform frame sizes and all ten example sections, independent Pin slots, accessory selection, workspace title/sidebar geometry, fixed 384 vp workspace height, and controlled search/clear/Escape without losing the title. Default/minimum widths and maximum-height clipping have native checks. Plain/rich title and subtitle font attributes match React in three palettes; rich glyph size and ink are checked separately. Native Gaussian shadow pixels pass the four-edge check in Studio light/dark and Bright light for active/inactive windows; Omarchy remains shadowless. Modern and flat macOS traffic lights have native material, hover-region, glyph-centre and callback checks. Rounded corners, shadow transitions and all-theme paint remain separate checks. |

System-font checks passed on both PC and phone emulators at 1x and 2x,
including palette independence and a nested fixed design scope. Desktop native
buttons measured 40 vp at 1x and approximately 53.37 vp at 2x, while the design
scope retained its 28 vp controls. Mobile checks additionally covered 74 native
theme scopes and native button/input heights of 40 vp at 1x and approximately
53.67 vp at 2x, independent of the selected family. Interaction checks covered
button, dialog, sheet, input, switch, checkbox, family and
appearance changes. Both galleries were relaunched without test-only scale
overrides. Numeric injected queries avoid changing the host's active pinyin IME.

Mapping coverage does not close the remaining implementation and verification
work:

- PreviewCard: touching a trigger while the mouse remains parked, nested floating-child
  coordination, wrapped multi-line link anchors, controlled closing refusal,
  focus return and accessibility.
  Cursor-origin geometry, velocity and the 40 ms intent timer match the installed
  Base UI handler in 3,872 deterministic comparisons; native integration still
  needs more paths than the basic trigger-to-card traversal. Real IME
  composition requires a targeted native check.
- Toast: opening further overlays above existing notices, resize, theme changes,
  drag-in-progress replacement, entrance interruption, pointer cancellation,
  multi-touch, removed/disabled previous controls,
  announcements and narrow-window truncation need more native checks.
- Slider edge antialiasing, transition trajectories, pointer cancellation,
  mixed-input handoff, live fit changes and all-theme paint still need work.
  Tabs exact overflow boundaries across system fonts, pulse trajectories,
  all-theme paint and accessibility; Calendar grid/month announcements;
  DateRangePicker narrow fitting, separator/stretch geometry, modal handoff and
  rapid remount/input sequences remain open.
- WindowFrame narrow rich-title clipping and typography paint, percentage/intrinsic bounds, control
  handoff, mixed control states, shadow transitions and pressed motion need
  native checks. Traffic-light glyph antialiasing and outer window corners
  remain outside the targeted material check; three dark grey close-light
  results still exceed its two-byte maximum-error limit by one byte level.
- Legacy icon-slot ordering/tint/size, Tree icon conformance addresses and Callout
  geometry need further alignment. All-theme paint,
  typography and screen-reader conformance remain outside the default gate.
  Menubar still needs press-drag selection, vertical opening, dismissal/resume paint and
  assistive-technology checks. Continuous-resize frame paint and
  context-menu resize/touch long-press behavior
  still need native checks. Native
  default materials do not establish ds paint conformance.
  Tooltip still needs provider warm groups, rich labels, aligned-edge collision,
  live resizing, overlay nesting, motion and assistive-technology checks.
  Button, WindowFrame, Dialog, Toast, Sidebar, Menu, Select and Tooltip use the Gaussian drawing path;
  shared prose popups retain the separately verified native host shadow.
  Full control-surface and all-theme raster comparisons remain open.

Targeted checks are rerun after shared changes; an earlier passing result does
not establish every later combination. `example/coverage.json` and
`example/react-coverage.json` report source mapping only.
