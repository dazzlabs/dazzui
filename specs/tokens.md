# Tokens

How a value is decided: the token pipeline, the three layers, and the colour
system's washes, grades and recipes.

## The pipeline

1. `scripts/build.mjs` reads the files listed in `terrazzo.config.mjs` and
   hands them to Terrazzo's own `parse` and `build` — the two calls
   `tz build` makes — with one step of its own in between: the recipe table
   under `control.color` writes only its decisions, and
   `scripts/recipes.mjs` fills in the derived cells before the graph is
   parsed (see Conventions below). The plugins then stage
   `ds/dist/`:
   - `style.css` — CSS custom properties (the contract consumers use)
   - `control.css` — the ramp bindings and the resolved interactive color
     recipes, from `plugins/control.mjs`
   - `runtime.ts` — theme-invariant durations needed before a React API has a
     DOM node, from `plugins/runtime.mjs`. The build rejects a per-theme value
     here; scoped visual values remain CSS variables.
   - `flutter/colors.dart` + `flutter/theme_variables.dart` — the Dart token
     sources, from `plugins/flutter.mjs`
   - `swiftui/Colors.swift` + `swiftui/ThemeVariables.swift` — the Swift token
     sources, from `plugins/swiftui.mjs`
   - `gpui/theme_tokens.rs` — every theme as a Rust `const`, from
     `plugins/gpui.mjs`

   `plugins/contrast.mjs` runs alongside and emits nothing: it holds the ink
   ladder to WCAG contrast in every theme and fails the build when a
   re-pointed role lands below the floor. (Terrazzo's own lint rule cannot
   do this — 0.10 lints each token file with only that file's tokens.)
2. `distribute()` in `scripts/build.mjs` then copies each platform's files out
   of that staging directory into the platform package itself — the two
   stylesheets and runtime module into `packages/js/dazzui/src/generated/`, the two Dart files
   into `packages/dart/dazzui/lib/src/generated/`, and so on for Swift and
   Rust. The `TARGETS` list is where a new platform package is added. The
   Dart copies are then run through `dart format` (PATH first, fvm's stable
   as fallback), and the Rust through `rustfmt`, because the committed files
   are formatted and a rebuild must not dirty them.

React's Toast adapter imports its notice/action durations from the committed
runtime module. Its transition still reads scoped CSS variables. HarmonyOS's
queue receives the same resolved durations from its theme tokens; neither
package imports ds at runtime.

Tooltip's native hover-rest and close defaults live in
`tokens/component/tooltip.yaml`. The runtime tests compare the generated values
in every palette with the installed Base UI Tooltip defaults; React retains
Base UI's trigger and provider behavior. These delays are independent of system
font sizing. A Sidebar scope overrides hover rest to zero, matching React.

The date-range picker's trigger/time minimum widths and key tracking live in
`tokens/component/date-range-picker.yaml`, beside its side-column width. React
and HarmonyOS read the same roles. System sizing scales the lengths and applies
tracking relative to the resolved native font size; the tracking ratio itself
does not scale.

Nothing writes a token file: every source, `_base.yaml` included, is
hand-authored. (The Tailwind sync that used to write `_base.yaml` is gone —
the design is drawn to BeyondTranslate, so there is no upstream to track, and
what Tailwind contributed is frozen in `_base.yaml` at the values it had.)
The recipe expansion is the one transformation between a file and the graph,
and it lives only in the build process — the expanded document is never
written anywhere.

A new token file is only built once it is added to `terrazzo.config.mjs`.
`dist/` is gitignored and is staging only — the distributed copies are what a
platform package reads, and they are committed.

## Conventions

Three layers, each referencing the one above it:

- **Base primitives** (`tokens/global/_base.yaml`) — the one place in the
  graph a raw *palette* value exists, all of it hand-authored: eleven colour
  ramps (the three brand ladders, the five semantic hues frozen from
  Tailwind, and the three dark companions that lift the status hues on a
  dark canvas — whole ramps, because a platform that carries a ramp as one
  swatch can only move a ramp whole), the font weights, and the AppKit
  metrics the design is drawn to (`base.corner`, `base.type`,
  `base.leading`, `base.control-height`, `base.font`). One scale per axis —
  there is no second radius or type scale to alias by accident. Components
  must never reference `base.*` directly. The underscore marks the layer,
  not a generator: nothing writes this file but a person. (The semantic
  layer holds raw values of its own kind — each theme's paper, ink and
  shadow are decided there — but never a hue's ladder.)
- **Semantic globals** (`color`, `radius`, `spacing`, `typography`, `effect`,
  and the `control.*` size profiles) — named roles that alias primitives, so a
  theme can re-map a whole ramp by changing one reference. (`spacing` is the
  one exception that holds values directly: its semantic names *are* the grid
  steps, and a base twin under the same names would only be duplication.)
- Themes are **not** a layer or a directory — they are the modes carried
  inline on whichever token varies, and that is forced rather than chosen:
  Terrazzo merges token files by replacing a whole token, so a per-theme file
  adding a mode to `color.surface` would replace the base declaration
  outright, and a file carrying only `$extensions` is dropped with no error.
- **Component tokens** (`tokens/component/*.yaml`) — only for values that are
  genuinely the component's own: a part no shared role covers (`checkbox.box`,
  `switch.thumb`, `dialog.width`) or a `color`+`alpha` wash (`dialog.scrim`).
  A token that would merely forward a shared role (`{control.small.size}`,
  `{radius.medium}`, `{labelSmall}`) must not exist — the component's CSS
  reads the shared variable directly, so most components have no token file
  at all.

The colour system reads in washes and grades:

- **Washes are alphas of the tint, not opaque ramp steps.** A tinted chip is
  the tint at 12%, a plain hover the tint at 8%, a banner's border the tint
  at 25% — so one wash sits correctly on paper, on a card, and on a dark
  canvas without per-theme values.
- **Accent text is a grade darker than accent fill**: content recipes read
  700 where surfaces fill 600, deepening to 800 on hover (300/200 on a dark
  canvas). A label set in the fill colour is the tell of a port.
- **Every surface is a fill an artboard draws, and the ladder is guarded
  at the design's own steps.** Studio's greys are 4a and 4c of the
  BeyondTranslate design, Bright's are 4b and 4e, and the four families with
  no artboard keep Studio's intervals. Three roles come straight off those
  boards and are easy to conflate: `surface-column` is the nav, aside or
  dock — a hair off the paper on 4c, *under* it on 4a and on both Bright
  boards; `surface-muted` is the card, one step off the paper in whichever
  direction the family leans (down in Studio, up to white in Bright); and
  `surface-overlay` is the floating sheet — the paper in a light theme, the
  titlebar's step in a dark one, behind a `.09` edge and a deep shadow. The
  hover wash (`surface-subtle`, what the `normal` recipe hovers to) is an
  alpha of the ink — 6% over paper, 7% over a dark ground — rather than an
  opaque rung, so it moves paper, card and column by the same amount.
  `plugins/contrast.mjs` holds the adjacent rungs to a minimum L\* step read
  off the same boards, so a re-pointed theme cannot collapse two of them or
  trade their places, the way Bright Light's hover once landed on its card.
- **Disabled flattens to the groove** — `surface-sunken` fill,
  `content-faint` text, no border — one dead look for every variant, never a
  translucent ghost of the live one.
- **Selection shifts weight as well as colour.** Rows rest at `labelQuiet`
  (500) and the selected entry takes the label weight (600); a pill tab's
  active label and an option card's title go to `labelStrong` (700, display
  cut). `stroke.control` (1.5px) draws control outlines; `stroke.hairline`
  (1px) draws separators, on every platform and at every display scale — it
  used to halve on Retina to land on one device pixel, and against the
  mockups the system is drawn to a halved rule read as too thin to be a
  decision. `@dazzlabs/dazzui`'s `src/styles/base.css` carries the box-sizing and
  reduced-motion ground rules.
- **A separator is ink at 13% over paper, 16% over a dark ground** —
  `color.border`, the same two numbers in every family, since the ink each
  one substitutes already carries the difference. `border-strong` keeps its
  rung above (17% / 20%) so a popover's edge still separates from what it
  floats over, which lands it beside the opaque `border-muted` a checkbox or
  a radio is outlined in rather than under it.
- **The focus ring is the system accent, always**: `--focus-ring` on
  `:root` — 3px of primary at a per-theme alpha, offset 0, hugging the
  control the way macOS draws it. It never follows `data-tint`.

Four things that surprise people:

- Interactive colors are stored as *recipes*, not finished colors: a `shade` to
  pick off a ramp plus an `alpha` to apply. Each platform resolves them against
  whichever ramp the control is tinted with.
- The recipe table writes only its decisions. `control.color` is six variants
  by three parts by four states, and most of those cells are one rule
  applied, so a variant writes what it decides and `scripts/recipes.mjs`
  fills in the rest before the graph is parsed: `hovered` unwritten is
  `normal`, `pressed` unwritten is `hovered` dimmed by `$defaults.pressed`,
  `disabled` unwritten is `$defaults.disabled` for that part, a part left
  out entirely is `$defaults.undrawn` in every state, and a shade with no
  alpha is opaque. The names the rules fill in with live under
  `control.color.$defaults`, consumed by the expansion and never a token.
  Every plugin reads the full table, so no platform generator carries a
  rule — and a cell that is only the rule applied must not be written, since
  writing it hides which cells are decisions. A misspelt part or state
  fails the build rather than falling through to a default.
- A ramp is a **role ladder, not a lightness ramp**. Read `base.color.ink`
  before assuming otherwise: its 700 is *lighter* than its 600, because
  Bright's primary action is ink navy whose hover lifts, and its 50–200 are
  acid-tinted washes rather than pale navy. A step is only ever reached by a
  recipe, so what a step means is the role that recipe gives it. This is what
  lets twelve themes this different share one recipe table untouched — and why a
  shade written into a stylesheet would be a lie rather than merely a
  duplicate.
- A recipe can re-pick its step per theme (`shade` carries a mode), because a
  fixed step cannot be right in both directions: `tinted` wants the palest
  step of the ramp on paper and the darkest on a near-black canvas.
  `plugins/control.mjs` resolves those into a block per theme, since the shade
  is baked into the emitted value and nothing downstream could re-pick it.

## GTK4

GTK4 snapshots are generated by `ds/plugins/gtk4.mjs` into
`packages/rust/dazzui-gtk4/src/generated/`: complete theme-scoped CSS variables
and a Rust theme registry. GTK CSS custom properties require GTK >= 4.16.

## HarmonyOS desktop

`ds/plugins/harmony.mjs` writes ArkTS theme data into
`packages/arkts/dazzui/src/main/ets/generated/ThemeData.ets`. It shares the
resolved theme walk with WinUI/GPUI. Colors use ArkUI ARGB order; px/rem
lengths become logical vp and durations remain milliseconds. A complete
Bright Light snapshot plus per-theme differences carries every semantic
metric, type style, shadow and recipe. `DazzTokens` copies those maps per scope
and resolves recipe shades against the selected tint at runtime, multiplying
the source alpha rather than replacing it. It does not import ds at runtime.

Shadow lengths remain the CSS lengths in logical pixels. Button, WindowFrame, Dialog,
Toast, Sidebar, Menu and Select use `components/ShadowLayer.ets` to draw a native Gaussian image filter
with sigma equal to half the physical blur length. The canvas includes the blur's
outset and offset and keeps its origin aligned to the surface's pixel phase.
Other native controls and popup hosts use `foundation/Shadow.ets` to map offsets
and sigma into native shadow options. The native popup host passes the targeted
Gaussian check; a directly attached small button shadow did not, so Button
uses the separate drawing layer. Menu and Select use `shadow.lg` behind their
clipped foreground viewport; their native host defaults failed the shared
Gaussian profile. The silhouette follows the native painted border box,
rounded to physical pixels, rather than the fractional layout rectangle.
These conversions belong to the renderer rather than the token source.
The current 72 themes each carry seven single-layer outer shadows with zero
spread; there is no second outer layer to reproduce. Inset chrome effects are
separate platform drawings.

## HarmonyOS mobile

`ds/plugins/harmony-mobile.mjs` uses the shared resolved graph and paired-family
filter to generate `packages/arkts/dazzui-mobile/src/main/ets/generated/DazzThemes.ets`.
It maps only colors to ArkUI `CustomColors`, emitted as ARGB with recipe alpha
preserved. `dazzTheme` returns a fresh light/dark `CustomTheme`; no platform
package is a runtime dependency. The mobile HAR does not import desktop
metrics, typography, shapes or motion. Native components own those decisions.
