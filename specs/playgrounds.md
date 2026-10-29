# Playgrounds

How a story is written once, in the React example, and compiled into the
other platforms' examples.

- **The React example is the source.** Every story is a file in
  `packages/js/dazzui/example/src/stories/`, named as the kit names the
  component (`icon-button.tsx`), whose default export is one `Story`: an
  `id` matching the file, a `name`, a `summary`, a `Preview` — the one scene
  that stands for the component on the overview — and `examples`, each a
  `title`, an optional `note` and a `Demo`. React renders the file as it is.
- **Codegen compiles it; it never runs it.** `tools/codegen/stories.mjs`
  reads each file with the TypeScript compiler and evaluates it ahead of
  time: module constants, `.map` over a constant list, a template string, a
  helper component called with props — all of it is worked out, so what
  comes out is the finished tree the React page renders. What stays live is
  kept as a reference: a `useState` is a slot, a setter handed to an event is
  an action that writes it, a comparison against a slot is an expression,
  `useToast().add` is an action that raises one.
- **A story is written in the portable subset**, and codegen refuses
  anything outside it with the file and line:
  - values come only from the kit, `@fluentui/react-icons`, the stage
    (`../components/stage`) and `react`'s `useState`; the one constructor
    is `new Date(year, month, day[, hours[, minutes]])` with parts known
    ahead of time, written into the data as local `YYYY-MM-DDTHH:MM`, and
    a platform writes a date a calendar picks back the same way;
  - elements are kit components, glyphs and the four stage components —
    `Row`, `Col`, `Frame`, `Text` — never HTML, never `style` or
    `className`;
  - an event handler only calls a setter, raises a toast, or does nothing.
- **Every platform draws what the story asks for, or generation fails.**
  `tools/codegen/flutter.mjs`, `swiftui.mjs`, `winui.mjs` and `gpui.mjs` each list
  the components
  and props their example maps; a story that sets one missing from a list
  fails `pnpm codegen`, naming it and the platform. The list and the
  mapping change together.
- **A story outside the portable pipeline lives in `stories/react/`.**
  The portable compiler reads the top level of `stories/` alone, so a story
  there is not a shared conformance scene,
  and is not held to the portable subset. Tabs and WindowFrame live there.
  A platform may implement one of these components separately or opt into
  reading a story with an explicit platform-specific grammar and separate
  coverage. HarmonyOS does so for Tabs and WindowFrame, and reads the
  portable Calendar and DateRangePicker from their own files with that
  grammar, because its wire format carries a date as integer parts rather
  than local civil time; this does not expand the portable subset. The
  WindowFrame story draws presentation windows inside the gallery's real OS
  window. Moving a story up out of `react/` is the claim that every platform
  now maps it.
  A file there may instead export a `StoryExtension` — an `id` and
  `examples` alone — under a compiled story's id, and its examples join that
  page on React only: the Preferences page's theme row is one, since a
  swatch takes its theme by nesting a `data-theme`, which no port can do
  inside one subtree yet.
- **A platform's example mirrors the React one's layout.** On Flutter:
  `lib/stories/` holds a generated file per story and `index.dart`;
  `lib/lib/` holds the story type, the scene interpreter and its component
  mapping, the generated themes and glyphs; `lib/widgets/` is the window —
  React's `components/`. On SwiftUI the same under `Sources/Example/`:
  `Stories/`, `Lib/`, and `Views/` for the window. On WinUI the same under
  `example/`: `Stories/`, `Lib/`, and `Controls/` for the window. On GPUI the
  same under `example/src/`: `stories/`, `lib/` — whose `mapping/` is one
  file per family of components — and `window/`. Every platform names a
  glyph by the React name the story wrote: Flutter from
  `fluentui_system_icons`, SwiftUI and WinUI from `FluentIcons`, and GPUI
  from SVG codegen renders out of `@fluentui/react-icons` itself, so its
  outline is the web's to the path. The window is written by hand to the same columns,
  widths and tokens as React's; only the stories are generated. A story is
  JSON inside the generated file — on SwiftUI and WinUI a raw string parsed
  at launch, which the compiler never has to read; on GPUI a raw string too.
- **Never hand-edit a generated file.** `tools/codegen/outputs.json` records
  what codegen owns and the hash it wrote; `pnpm codegen:check` fails on any
  drift, and a file codegen stops writing is deleted only while it still
  matches that hash. Generation needs Dart to format its output.
- **Flutter, SwiftUI, WinUI and GPUI are compiled today.** The iOS and Android
  examples keep the generated sources the previous pipeline wrote, frozen:
  codegen no longer owns or rewrites them. Each is brought back by giving it
  a mapping and a support list of its own.
- Run `pnpm codegen` after changing a story, `pnpm codegen:check` to verify,
  `pnpm codegen:test` for the compiler's own tests. The Flutter example's
  `test/stories_test.dart` and the SwiftUI example's `StoriesTests.swift`
  draw every story's preview and every example in light and dark, and
  drive the interactions the interpreters carry; the WinUI example's
  `--smoke-test` draws every page in every theme in a real window.

## GTK4

The GTK4 example mirrors React under `example/src/`: `stories/` holds one
Rust module per component and its generated registry; `lib/` holds the story
model, scene interpreter and generated theme families; `components/` holds
the shell, brand, stage, theme panel and theme preview. `app.rs` owns the
window lifecycle and `main.rs` dispatches the application or `conformance/`.
Stories keep Preview separate from examples: previews appear on the searchable
Overview; documents display the summary and examples with their optional notes.
The shell has the same component list, document toolbar and grouped theme
panel as React, with native GTK splitters between its columns.
`tools/codegen/gtk4.mjs` records each unmapped component, prop or live scene
in `gaps`; the gallery shows those gaps instead of drawing a partial scene.
The initial port does not yet participate in the full-support gate.

## HarmonyOS desktop

`tools/codegen/harmony.mjs` writes typed scene data and theme groups for the
ArkUI gallery, and shares the Fluent SVG emitter with GPUI. The wire format
keeps state slots, setters, arguments, comparisons and specimen addresses;
the native interpreter uses typed records instead of reflection. Overview
uses a wrapping preview grid; document pages use examples and notes. Native
resizable splitters separate React’s component rail, document column and grouped
three-column theme picker; toolbar controls toggle either sidebar. Unsized
story rows and columns shrink to their contents with API 20’s
`LayoutPolicy.wrapContent`, while explicit scene widths stay literal. The shell
feeds host-area dimensions to RowSplit so native resize caches refresh without
unmounting controls or losing story state.
It saves a dragged sidebar's final measured width after the release frame has
rendered, so a later window restore/maximize keeps the completed drag position.
Document cards apply their 160 vp minimum to the complete padded card, matching
React's border-box stage. Overview previews keep their separate 128 vp inner
stage minimum. The device stage check measures both axes of centering and the
document card's total minimum height.
Inside dialog bodies and drawers, unsized stage rows and columns inherit the
container's column stretching; the centered gallery stage keeps intrinsic
widths. Drawer scenes use a native modal with four edge alignments, shared
dialog rows and a body scroll view that hands outward edge gestures to the
sheet. A test-only modal page verifies overflow and optional slots independently
of the generated stories.
ShortcutRecorder scenes use physical native key events, preserving React's
printed chord format and controlled callbacks. A modal probe checks that
recording consumes Escape before native dialog dismissal and that blur cancels.
OtpField scenes use native editors inside fixed cell boxes; the native input's
own border does not determine the profile size. A logical cursor preserves fast
IME input while focus moves between cells. Native probes also check clipboard
distribution, controlled values and form callbacks below the modal form's
builder boundary.
Tree scenes preserve the recursive item data, empty branches, glyphs and badge
metadata. Native focus groups separate keyboard navigation from selection and
keep one Tab stop; a separate modal probe exercises controlled expansion, changed
labels and refused updates. Basic Latin type-ahead uses the same generated prefix
interval as React. Native list roles describe hierarchy because the SDK does not
expose a tree role; this is not a screen-reader conformance claim.
Sidebar scenes preserve the provider, column, menu and inset composition,
including the fixed preview rail and icon, floating and inset examples. The
interpreter forwards flexible layout to content and inset children explicitly.
Row state also reaches sibling counts; a stable native paragraph reads its
current scene run after selection changes. Window-scoped shortcuts toggle all
mounted providers, including the gallery's own component rail, while recorder
input can consume the chord before native shortcut dispatch.
PreviewCard scenes keep inline link triggers in native paragraph spans. The
paragraph's layout manager supplies UTF-16 text rectangles for a virtual popup
anchor and keyboard target, without replacing the sentence with a wrapping row.
Popup content stays out of the paragraph's text and copy buffer. Shared prose
scopes distinguish body-medium gallery text from body-small popup content.
The hover/focus clock and cursor-origin polygon have non-UI checks against
the installed Base UI implementation. SDK and native checks cover inline
keyboard focus, basic pointer traversal, popup dimensions and Escape; nested
floating children, mixed input and wrapped anchors remain verification gaps.
Slider scenes retain numeric arrays and their controlled callbacks. The same
native component handles scalar values and ranges; overlapping native sliders
share a token-drawn indicator. Pointer changes push neighbouring values, whereas
keyboard changes stop at the neighbour. Non-UI tests compare both paths with the
installed Base UI implementation, including its actual keyboard handler. The
range mapping compiles with the SDK. Native checks cover pointer pushing,
controlled refusal, native values and forward/reverse Tab order; track paint,
handle insets and hover/shadow geometry still need comparison.
Toast actions travel through the scene interpreter to the consumed provider;
their action and close callbacks retain the originating scene state. The native
viewport mounts an ordered native overlay with stable provider order and forwards
the live theme into its detached content. Both the builder boundary and its
layout pass pointer input through transparent areas. Native full-window layout
places the padded receipt stack at either edge, including when raised inside a
dialog. Its first frame is excluded from focus so displaying a notice preserves
the source control. F6 enters the notification region; Tab visits each eligible
receipt, its action and dismiss button, and restores the previous control at
the boundary. Focus and held touches pause the queue's timers. Native pan
recognition retains the drag beyond a receipt's original bounds; raw pointer
samples preserve Base UI's first-move handling, axis lock, directional damping,
threshold and reversal cancellation. Non-UI traces execute the actual Base UI
pointer handlers as their reference. Receipts block input to covered controls;
their protected presses still reach other scoped pointer observers without
treating their own viewport as an outside target. Source mappings cover all
four portable Toast scenes. Screen-reader equivalence and full paint
conformance remain gaps, independent of the mapping count.
`tools/codegen/harmony-react.mjs` also reads the actual React-only Tabs and
WindowFrame pages, and the portable Calendar and DateRangePicker ones, whose
dates it writes as integer parts for the native interpreter.
Its narrow grammar accepts the story's token-based fixed header and functional
array removal, while refusing other HTML styles and arbitrary callbacks with
source locations. The generated `react-scenes.json` and `react-coverage.json`
keep those scenes separate from portable conformance, and the portable list
drops the pages it read natively so the gallery carries each once. Their data
joins only the HarmonyOS gallery registry and glyph set. WindowFrame
additionally accepts only its source Pane's token padding, retaining typed
titlebar slots, rich titles, scoped Pin/segmented state and the five sidebar
workspaces. Native
Buttons dispatch window-control callbacks; macOS keeps disabled lights while
Windows and Linux omit unavailable controls. The native component transcribes
platform chrome metrics from the React stylesheet, using ds for theme colours,
spacing, toolbar height and shadows. Native checks cover all seven platform
frame sizes, Pin/accessory slots, workspace state/geometry, default/minimum
width and maximum-height constraints. Rich titles consume the same resolved
platform style as plain titles from a live heading scope, applied through the
authored Row, Text and Icon. Native title/subtitle font attributes and the rich glyph's
size/color match actual React computed styles in three palettes. Narrow title
clipping, percentage/intrinsic bounds, control handoff, motion and complete
paint remain separate checks. Native Tabs uses independent measurements,
compact items, an overflow menu and controlled selection/close callbacks;
non-UI tests execute React's actual layout and keyboard functions as references.
Native checks cover authored header sizes, overflow promotion, Home, selected
document removal and the trailing action; paint and accessibility remain open.
A separate action fixture compares 22 live states with actual React Tabs:
close/more hints and colours, keyboard activation, controlled close refusal,
focus after removal, forward/reverse Tab with and without close controls,
menu return and touch opening. Twenty native glyph captures also pass the
source-ink range check. The 200 vp promoted-tab system-font boundary remains
a failing fit fixture under the default Mac font. A separate comparison using
the captured emulator's font passes all eight fit states and records its hash
and computed ruler/tab families. The original report remains separate; full
paint, motion and accessibility remain open.
A keyboard action fixture compares Button, IconButton and Tabs actions with
their actual React components, separating down, repeated down and release.
Its 31 states include cancelling a held Space action by moving focus, Enter
repeat and opening the overflow menu only on Space release. Counts, focus,
menu visibility and settled backgrounds are compared independently of the
larger paint and accessibility gates.
The same action specimen also checks 27 held-mouse states against React:
press, movement inside/outside, reentry, release and outside cancellation.
Native clicks remain authoritative; the adapter retains paint and focuses on
press. Separate three-palette keyboard-focus captures suppress the duplicate
system focus box. The outline adapter follows the CSS layout grid and caches passive paint.
Its three straight-edge raster comparisons now have RMS below one but still
exceed the maximum-two limit at a few pixels. Paired unfocused captures record
underlying button-edge differences. These failures remain outside the
full-support gate.
Calendar dates are local integer parts in the wire format; they become native
Dates on the device rather than timestamps from the generator's timezone.
Conditional captions read scoped date/range state and format in the native
locale. Calendar's native day Buttons preserve single/range selection, pending
hover/focus preview, bounds, one or two six-week grids and roving keyboard
focus. Date and interaction tests execute the actual React functions as
references, including a daylight-saving timezone. Native checks cover grid
dimensions, controlled day/range picking, keyboard movement, bounds, two months
and localized labels. A live fixture also compares nine states with the actual
React component: fallback Tab entry in one/two months, accepted/refused month
proposals, and pending range preview moving between pointer and keyboard.
Native labels and Tab indices are read from the running buttons, with focus
and selection from the UI tree. Full paint and accessibility remain open.
The date-range reader preserves local Date parts, live formatted captions,
product durations and presets. `harmony-dates.mjs` translates only a
sealed set of date getters, constructors, numeric operations and known local
date helpers, rather than executing arbitrary callbacks; a preset that is data
— a pair of whole-day offsets from today — becomes the same sealed formula,
and the component's own preset metadata comes from the actual React component
module. Formula evaluation stays on the device so a timestamp-based sprint
crossing a daylight-saving boundary behaves as the original source does. The
native picker keeps its draft/session beneath the forwarded popup builder and
publishes only on Done. Native checks cover dimensions, initial focus,
draft/Cancel/Done/reopen, time coordination, all-day disabling,
Escape/outside cancellation and custom presets. Nineteen live states
also agree with the actual React component on controlled acceptance/refusal,
draft reopening, input-dependent opening focus, nested Escape, focus return and
forward/reverse Tab traversal. All five picker examples render; narrow fitting,
separator/stretch paint, modal handoff and rapid remount/input sequences remain
open. This is not full geometry-conformance evidence.
`pnpm harmony:test:device` scrolls every page and exercises the mapped native
interactions. `example/coverage.json`
records unsupported components, props and values. Any gap replaces the whole
scene with that list, and the initial port stays outside the full-support gate.

## HarmonyOS mobile

`packages/arkts/dazzui-mobile/example` is a Stage phone/tablet gallery built
directly from native ArkUI controls, with family and appearance selectors.
Its Actions, Inputs and Explore tabs use mobile layouts and native behavior.
It is authored independently of desktop stories and does not change the frozen
iOS/Android outputs. Its test-only Verification page checks all generated
families in both modes, system appearance changes and nested scopes through
native theme callbacks; `scripts/test-device.mjs` also drives the live gallery
and asserts the OS accessibility tree. Desktop pixel conformance does not apply.
