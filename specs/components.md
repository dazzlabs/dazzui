# Components

How the React package is built and how a component is drawn — the model every
port follows.

## The pipeline

`vite build` bundles `packages/js/dazzui/src/` into that package's own `dist/`:

- `index.js` + `*.d.ts` — the components; React and Base UI stay external
- `components.css` — every component stylesheet, via the `styles.ts` entry

Base UI is external for the reason React is, and one more: it holds the open
popup, the toast queue and the field's wiring in React context, so a second
copy in the bundle would be a second set of contexts and a provider a
consumer's own components could not see.

`src/styles/components.css` is the registry, the way `terrazzo.config.mjs` is
for tokens: a component's stylesheet only ships once it is imported there. It
also `@import`s `../generated/control.css`, so the generated recipes are
bundled in rather than pushed onto consumers as a third stylesheet.

Because `src/generated/` is committed, this package builds without `ds` having
run — there is no dependency edge and so no build order to get wrong. The cost
is that editing a token file does nothing until the generator runs, which is
why `pnpm packages` runs both and the playground scripts call it.

Never hand-edit `src/generated/`. A value a component needs belongs in
`ds`, and anything written there is overwritten on the next build.

## Conventions

- **Behaviour is Base UI's, drawing is ours.** A component that has state a
  reader can get wrong — a popup that has to close, a group the arrow keys
  move through, a control that has to reach a form — is a thin skin over the
  matching Base UI part rather than a re-implementation. Reach for a plain
  element only where there is genuinely nothing to get right (Badge, Card,
  Divider's tone). What this buys is not shorter files: it is the focus trap,
  the type-ahead, the `aria-*` wiring and the collision handling that a
  hand-rolled version never quite finishes.
- **Where Base UI has no part, the behaviour is written out and the reason is
  written down.** Four components own their own: `Tree`, whose keyboard is
  not a list's — `ArrowRight` opens a shut branch and descends into an open
  one, `ArrowLeft` shuts one and climbs out of a leaf, and the whole tree is
  one tab stop, none of which falls out of a `Collapsible` per node;
  `Calendar`, whose grid is one tab stop the arrow keys move through by a
  day and a week and Page Up and Page Down by a month, and whose range mode
  previews the span against the day under the pointer or the keyboard —
  Base UI has no calendar part; `Splitter`, whose seams are a pointer
  gesture with an `aria-valuenow` rather than a state machine; and
  `Skeleton`, which has no behaviour at all.
  Writing one of these is the exception, not a precedent: check
  [base-ui.com](https://base-ui.com) first, and if a part exists, wear it.
- **State comes off Base UI's attributes, not the DOM's pseudo-classes.**
  A checkbox root is a focusable `span` over a hidden input, so `:checked` and
  `:disabled` never match — the CSS reads `[data-checked]` and
  `[data-disabled]`. A popup row reads `[data-highlighted]` rather than
  `:hover`, which is what makes the keyboard and the pointer land on the same
  state. `[data-*]` was already the convention for variants; this is the same
  rule reaching the states.
- One folder-level file pair per component: `src/components/<name>.tsx` and
  `<name>.css`, both listed in their registries (`components/index.ts`,
  `styles/components.css`).
- **A family is named after its root, and the root is a component.** `Dialog`
  holds `DialogHeader`; `Table` holds `TableRow`; `Preferences` — the settings
  column — holds `PreferenceGroup`, `PreferenceSection` and `PreferenceRow`.
  The file, the stylesheet, the story page and the port's file all take the
  root's name, so a name that names nothing (`Preference`, when only its parts
  existed) is the tell that a root is missing rather than a family that has
  none. What the root owns is whatever the parts cannot — the run between
  groups, the column's width — which is usually exactly what the stories and
  the examples were each writing for themselves.
- Components are styled by plain CSS reading the token custom properties —
  there is no Tailwind here. A value a component needs is a shared variable
  read directly (`--spacing-4`, `--radius-medium`, `--label-small-font-size`,
  `--control-small-padding-block`); only a value no shared role covers becomes
  a token in `tokens/component/<name>.yaml`. Never a literal in the CSS.
- A value the design lands between two steps of a scale is composed out of
  them rather than written down: the 18px a dialog body and a preference group
  both pad by is `calc(var(--spacing-4) + var(--spacing-0-5))`. That keeps the
  scale the only source of lengths, and it keeps the value honest — a step and
  a half says what it is, where `18px` only says what it measures.
- **A class on an icon is one class too few for `display`.** A Fluent icon
  brings its own sheet — `display: inline`, `line-height: 0`, the RTL flip —
  and appends it to the head as the first icon renders, which is after every
  stylesheet linked here, so it takes a one-class tie. A rule that sets one of
  those on the icon itself names the parent as well, the way `checkbox.css`
  hides its dash (`.dz-checkbox .dz-checkbox__dash`); every other part puts
  the glyph in a span of its own — `.dz-checkbox__mark`, `.dz-select__chevron`
  — and styles that.
- Icons use the shared `icon.small / medium / large` theme scale. A leading
  slot binds `--dz-icon-size` and sizes its direct SVG children with it, so
  raw icon-library SVGs and `Icon` agree. Sized controls bind it to
  `--control-icon-size`; auxiliary chevrons, checks and dismiss glyphs use
  `--icon-small`. Keep explicit inline sizes as overrides and never size
  arbitrary descendants, avatars or spinners as glyphs.
- A glyph beside a label is not a glyph alone. The profile's icon size fills
  a square button, and next to a label it overtops the type, so Button draws
  its glyphs `control.label-glyph.shrink` under it. The end a glyph sits at
  pads `control.label-glyph.inset` less than `padding.inline`, because the
  glyph's frame carries whitespace of its own. Both are trims on the
  profile rather than lengths, so they follow the desktop metrics that
  re-pick the icon scale and the inline padding. Every port's Button owns
  both; a consumer passes a glyph and gets them. A SegmentedControl item's
  `icon` follows the same two trims beside its label, and an `iconOnly`
  segment draws the glyph alone at the full profile size.
- A glyph alone in the tiny IconButton square takes
  `icon-button.tiny.icon-size`, not the profile's small step: the small
  step is sized for 11px type and reads as a speck alone in 24px.
- Sized controls read the `[data-size]` profile bindings — `--control-size`, `--control-icon-size`,
  `--control-gap`, `--control-padding-*`, `--control-radius`,
  `--control-font-*`, `--control-line-height` — once in their base rule
  instead of restating geometry per size (Button and IconButton are the
  model — neither keeps a per-size block at all, since the profile carries the
  design's own control type). A deviation from the profile keeps its size
  block and overrides the bound variable with another token's variable.
  Non-controls with sizes (Badge, KeyCap, Card) skip the bindings and
  read the profile steps and typography faces directly per size block —
  `badge.css` is that model.
- Variants are `data-*` attributes, not classes, so the CSS selector names the
  axis (`[data-variant='tinted']`) and consumers can see the state in the DOM.
- **The floating sheet is one drawing, in `popup.css`.** A Menu's panel, a
  Select's list and a Tooltip all wear it, and `menu.css` and the select rules
  in `text-field.css` add only what is genuinely theirs — which check
  attribute reveals the tick column. Gaps between a popup and its trigger are
  padding on the positioner rather than an offset passed in code, so the gap
  is a token and the collision logic counts it when the panel flips.
- **Native dropdowns are an application preference on SwiftUI, Flutter and
  GPUI.** All three default to `MenuPresentation.custom`. SwiftUI's
  `.menuPresentation(.native)`, Flutter's `MenuConfiguration` and GPUI's
  `provide_menu_presentation` scope Menu, Select, ContextMenu and Menubar
  independently from the color theme. A nearer scope overrides the root;
  changing it dismisses an open menu. Native mode keeps the trigger's DazzUI
  drawing and lets the desktop OS draw the list. Searchable Combobox and
  Autocomplete suggestions keep their custom panels, as do rich popovers
  such as the date picker. Ordinary menus inside those panels inherit the
  preference. Unsupported platforms or an unavailable native backend fall
  back to the custom panel. Shortcut strings remain hints, rather than
  registering additional accelerators. Flutter depends directly on
  `nativeapi` and follows its Dart SDK floor (3.13); GPUI takes nativeapi's
  Rust binding behind its `native-menus` cargo feature, and its native rows
  carry no icon or key-cap slots, since gpui cannot rasterize an element
  into the bitmap a native row wears.
- `PreferenceThemeSwatches` is also available on Flutter, macOS SwiftUI and
  WinUI 3. Each option has a value, an accessible label and an optional theme
  override; omitting that override uses the value as the theme name. The
  miniature window reads its own generated palette and geometry, while the
  selection ring and the gap between swatches use the surrounding theme.
  Selecting reports a value rather than applying a theme to the app. Native
  previews wrap at the trailing edge of narrow preference rows. Flutter and
  SwiftUI accept controlled or default values; WinUI exposes a bindable
  `Value` and `ValueChanged` over native radio buttons.
- **A text control's family is one file per platform, and the three wear the
  same box.** React keeps `TextField`, `Select` and `FieldValue` in
  `text-field.tsx` because the box, the states and the tinting are one design
  either way; Flutter mirrors that with `field_box.dart` holding the
  decoration every one of them reads, and SwiftUI with `TextField.swift`
  holding all three. A control added to one of those files is a claim that it
  belongs to the family — a `Select` is a field with a chevron, a `Combobox` a
  select with a query — so it shares the height, the corner and the recipe
  rather than resembling them. That corner is `control.field.radius` —
  the regular control corner in every drawn family, and its own role
  because some families draw a field and a button differently (macOS 27's
  capsule buttons and search fields beside rounded-rectangle text fields).
  A search field and a segmented track are controls, not field boxes, and
  keep `radius.medium`. The Flutter port went without a `Select` for a
  long time; it is the reason `pending.txt` exists, and its stories left that
  list the day it landed.
- CSS cannot index a ramp by a variable, so the shade/alpha recipes are
  resolved in the token build instead — `plugins/control.mjs` emits both the
  `[data-tint]` ramp bindings (`--tint-50` … `--tint-950`) and a finished color
  per variant/part/state (`--control-color-filled-surface-hovered`). Component
  CSS names a state, never a shade; a shade written into a stylesheet would
  drift the moment a recipe changed.
- Both halves land on `[data-tint]`, not `:root`, and that is load bearing: a
  custom property holding `var()` is substituted where it is *declared*, so
  `:root { --x: var(--tint-600) }` would resolve against a root with no ramp
  bound and every descendant would inherit the dead value. **Any component
  reading `--control-color-*` must therefore carry `data-tint`.**
- An interactive control opts into the shared state machine with `data-control`
  plus `data-variant`, then reads `--control-surface` / `--control-content` /
  `--control-border` and gets hover, press, and disabled for free. Button and
  IconButton are the model. A component that is *not* interactive (Badge, Card)
  must stay out of it and read the `-normal` recipes directly — `:hover` would
  fire on it just the same, and a label that changes under the pointer reads as
  something you can press.
- **One drawing for "on".** A selection is a tinted wash and a heavier label,
  everywhere it appears — a sidebar row, a tree row, a pill tab, a `Tag`, a
  `Toggle`. So a glyph button that stays down is a `Toggle` and not an
  `IconButton`: that component acts and returns, and a second on-state living
  there would be a second drawing for one state. A `Toggle` whose whole label
  is a lone glyph is square at the control height, which is what lets the two
  sit in one toolbar without a seam. A trigger that opens something is neither
  — a disclosure is `aria-expanded` and a dialog trigger is the dialog.
- Pick the recipe by what the part *is*, not by which colour looks right.
  `normal` is the neutral control — its hover wash is `surface-muted`, which is
  what a menu item, a table row, or a preference row wants. `plain` is the
  quiet *accent* control, so its wash comes off the tinted ramp and would sit
  heavier on a row than a `tinted` selection one step below it.
- `tint` is a prop only where a component genuinely re-tints. Components whose
  colors are already in the token graph (KeyCap, Progress's track) read those
  directly.
- A `color` paired with an `alpha` anywhere else in the graph (`dialog.scrim`)
  is resolved by the same plugin onto `:root`, so no component has to apply an
  alpha in CSS.
- **A hint is presented over the window, not in the layout.** A tooltip is a
  remark about a control, and the layout that holds the control clips it (a
  sidebar's edge, a scrolling pane) or paints a later sibling over it. React
  portals the label to the body; SwiftUI presents it in a panel of its own,
  flipped to the other side and nudged along the edge when the side asked for
  would leave the window; ArkUI uses a passive native overlay. What it must
  never be is a child of the control's layout.
- **Motion is the component's, and it is one token.** A state the reader
  toggles — a column collapsing on either edge, a row becoming the current
  page, a seam asked for a new width by a key rather than a drag — travels
  over `motion` inside the component that draws it:
  `--motion-duration`/`--motion-easing`, `vars.motion` on SwiftUI,
  `ThemeVariables.motionDuration`/`motionEasing` on Flutter. A host names no
  duration and draws no transition of its own, which is how a port lands on
  the same 150ms rather than on an opinion of its own. SwiftUI is the one
  platform where the host still makes the change inside the token's
  transaction (`withAnimation(vars.motion)`): a pane reflows only in the
  transaction its sibling's width was changed in, so a column that swept in a
  transaction of its own would leave the pane already at its new width
  underneath it.
  The trailing column is the leading one's mirror — `Sidebar side="right"` in
  React, `Aside` in Flutter and SwiftUI: the same motion, a clip anchored to
  the window's own edge, and the same leaving of the tree. A drag is the
  exception in the other direction: the width the hand is drawing is applied
  on the spot, so nothing the pointer drives ever settles. And a column that
  has gone is taken out of the accessibility tree and the pointer as well as
  out of the picture — React `aria-hidden`s the container and hides it once
  the slide finishes, SwiftUI `.accessibilityHidden` and
  `.allowsHitTesting(false)`, Flutter `ExcludeSemantics`, `ExcludeFocus` and
  `IgnorePointer` — because a column put away still holding a tab stop is a
  control a reader can reach and not see.

## HarmonyOS desktop

`packages/arkts/dazzui` implements a native ArkTS/ArkUI desktop HAR. ArkUI
owns input/IME, checkbox/radio/switch interaction, slider/tab selection,
menu positioning, modal lifecycle and splitter dragging. Controls draw from
the same ds profiles and recipes as the desktop reference, under a scoped
`DazzTheme` provider. The `Dazz` API prefix avoids collisions with ArkUI's
built-in component names. Native marks and handles retain some
platform geometry; the initial implementation has not established full
geometry conformance. Its unmapped React scenes are recorded by codegen.

Navigation Tabs measure the active label with its strong weight and reserve
the trailing slot's margin as well as its content. The React ruler uses the
same active state and margin reservation. Stable observed item records update
labels, counts, disabled state and busy state without replacing a surviving
native button. Compact labels and explicit titles use native `bindTips` hints;
an explicit empty title suppresses the compact fallback. Busy dots run two
forward eased intervals per cycle, matching CSS keyframes rather than reversing
the easing curve. Stopping or disposing a dot invalidates old completion
callbacks. Native pixel captures check dimming, brightening, stopping,
restarting and remounting. Close and overflow buttons use IconButton's label
Tooltip and quiet chrome. An observed interaction object keeps glyph ink live
through the forwarded builder, rather than retaining its initial child prop.
Tab activates keyboard mode before native traversal, so the first Tab after a
touch advances through close, overflow and trailing controls. Removing a focused
tab clears that native focus before the subtree disappears; otherwise ArkUI can
focus an unrelated peer. Refused controlled removal retains the close button's
focus. A narrow promoted-tab fixture still shows a different
overflow boundary under the native and browser system fonts; exact fit and
all-theme paint conformance remain open.
With the matching device font loaded into the browser, all eight captured fit
states agree, including that promoted tab. A native probe compares the measure
API with actual Text boxes for eleven samples; ten Latin samples also agree
with browser advances after native physical-pixel ceiling at the captured
density. The diagnostic records the font hash and checks the ruler's family.
The default Mac-font comparison remains a separate failing check. This does not
establish CJK fallback identity, all-font fitting or glyph raster equivalence.
Touch-opened overflow controls retain the reference's hover wash until a new
touch target or mouse movement clears it. A detached menu resets that state
through a separate scoped notification, without inventing main-window press
coordinates that could dismiss unrelated overlays. The controls unsubscribe
their press, motion and reset observers on removal. Twenty-two action states
match the actual React component; twenty lossless glyph captures verify ink
including hover and touch-opened menus. These are colour checks, not complete
glyph-shape, animation or all-theme raster conformance.

Button, IconButton and Tabs close/overflow actions arm Space until key release;
repeated down events do not activate them, and blur cancels the pending action.
Enter retains native down/repeat activation without painting a held Space face.
Ordinary buttons activate keyboard mode before native Tab traversal, matching
Tabs actions after a touch. A separate live fixture compares 31 checkpoints
against actual React controls: action counts, focus, menu visibility and settled
background colours. Its native driver keeps down/repeat/up in one uinput
process so the release carries the same pressed key. These checks do not prove
focus-ring raster identity, transition timing or all-theme paint conformance.

A mouse press on Button, IconButton or a Tabs action focuses it immediately,
while the native button still owns click activation. Pressed paint persists
across mouse motion and exit/reentry, and scoped release observation clears it
when release occurs outside. The observer is removed on release, blur,
disabling and disposal. Twenty-seven held-mouse states match actual React for
values, focus, menu visibility and settled backgrounds. The computed-colour
reader retains straight sRGB channels rather than losing low-alpha precision
through an 8-bit canvas.
The Button's ds outline suppresses the additional native system focus box.
A passive layer uses CSS layout-unit truncation and snapped outline rectangles,
normalizes corners, and caches a software-rasterized ring. Layout and theme
updates invalidate the cache; replacement and disposal release its bitmap.
Three-palette straight-edge profiles compare actual React at matching density
and border-box geometry. RMS is now below one; maximum channel error has fallen
from 80–118 to 5–6. The strict maximum-two / RMS-one checks still fail at a few
boundary pixels. Paired unfocused captures also record the underlying button
edge differences. The reports retain those failures; this does not establish
complete focus-ring raster conformance.

A menu's activation boundary remembers its trigger's focus appearance before
opening. Returning from the menu restores that appearance when it was already
keyboard-focused, including when the subsequent press used touch. A fresh touch
opening retains pointer focus appearance. Tooltip and focus restoration can
therefore follow the same trigger rather than the window's most recent input.

Calendar keeps one date Tab stop across one or two visible months. It prefers
the last focused date, then today; when both are outside the visible months it
uses the first month's first day. The first cell of the six-week grid may be
an outside date, so it cannot serve as that fallback. React and HarmonyOS use
the same rule, excluding duplicate outside cells from the Tab sequence.

Slider keeps one native `SliderStyle.NONE` input per value. This native style
has no thumb/shadow gutter, so pointer coordinates cover the entire ds track.
Response regions include the visible half-thumb beyond both endpoints. Native
sliders retain focus, capture and accessibility; their values remain the source
for controlled-proposal and disabled checks. A pointer-transparent Drawing node
paints the groove, indicator, thumbs, Gaussian lift and focus rings from ds.
It tracks the input's window-relative layout phase and uses one coordinate
system for the full track. Hover growth and border transitions run independently
of value movement. Native held-mouse tests keep the original input process alive
while checking updates before release and capture outside both endpoints.
The React raster probe includes the example's own border-box reset. It retains
unshifted comparisons of the full track and each thumb; translated best fits
are diagnostics only. Edge antialiasing differences still fail that strict gate,
so these checks do not establish complete Slider paint conformance.

Menu and Select share a custom row inside the native `MenuItem`. The drawing
follows `popup.css`: quiet labels, optional check and icon columns, trailing
shortcut typography, item padding, concentric corners and the neutral normal
highlight recipe. Native items retain focus, activation and disabled behavior.
The native Menu layout overlays its border rather than measuring it, so its
padding explicitly includes the shared border width. A Select panel measures
its anchor; an action panel measures its labels and columns against the shared
minimum width. Group captions use noninteractive custom rows inside native
`MenuItemGroup`s, avoiding the native builder-header minimum without adding
keyboard stops. A composite native menu host keeps the actual Menu and its
surface background as siblings. Its foreground Scroll clips long lists;
a separate Gaussian drawing layer casts `shadow.lg` outside that viewport,
and a fixed border stays above the scrolling rows. The silhouette uses the
physical painted border box, rounded at the native pixel boundary. No proxy
node attributes or extra invisible menus are needed. The native Menu retains
placement and MenuItem behavior; focused rows reveal themselves through the
foreground Scroller, including endpoint search and selected-value reopening.
Pointer focus follows mouse movement rather than hover entry: scrolling can
move a new row beneath a stationary pointer, which must not steal a keyboard
destination. ArkUI dismisses its menu host on window resize. Ordinary menus
and touch-opened Selects preserve logical open state by recreating that host
after its dismissal frame, restoring scroll offset and the focused row.
The reference's trigger-aligned mouse/keyboard Select closes on resize instead.
Window geometry is read through the public Window API; component disposal
cancels queued reopen work. Continuous-resize frame paint and context-menu
resize behavior remain separate verification requirements.
The adapters add endpoint traversal and the reference toolkit's prefix-search
behavior, including disabled skipping, repeated-initial cycling and closed
Select matching. They leave arrows and confirmation with native MenuItems.
The closed Select trigger opens on Up/Down/Enter/Space and focuses its selected
item or the first enabled option. Native confirmation may precede the bubbling
key callback. Contain that closing command and its release so the trigger does
not reopen from the same key, and never revive closed state from a late
appearance callback.
On API 26 the native target-space option receives `spacing.1`; older runtimes
retain native menu placement until separately verified.
Select keeps its native Button, label, disabled state and focus lifecycle. Its
painted face is a noninteractive sibling outside the Button's disabled-opacity
boundary; the native label remains semantic and transparent. This avoids
applying ArkUI's disabled alpha to an already resolved disabled recipe. The
face is excluded from accessibility, so only the native control supplies the
label and current value. A lossless screenshot checks the disabled background
and text ink; an inspector opacity reading alone does not establish paint.

Menubar is one scoped focus composite. Closed arrows move between enabled
labels; an open popup's lateral arrows switch menus. Home/End, first/last
opening, Escape return and Tab continuation use the native focus tree. Popup
Tab waits for native dismissal before dispatching the original key through the
current root, avoiding a second Tab stop at the trigger. The compact labels,
inter-label gap and neutral open wash follow `menu.css`.
The default modal menu consumes outside presses while its owning bar remains
interactive. Public FrameNode events on its MenuWrapper route hover and presses
to that bar; the inner native scroll clip cannot reach the bar. ArkUI processes
outside-touch dismissal before these callbacks, so disabled labels and gaps
resume the same menu after dismissal and the next rendered frame. The last
focused row is restored. Frame-by-frame paint of that resume, press-drag
selection, vertical opening and assistive-technology behavior remain separate
verification requirements.

Button, IconButton and Toggle share native button mechanics, but retain their
own drawing rules: an enabled filled Button has the `shadow.2xs` lift and
strong label; IconButton stays flat; a latched Toggle stays flat and uses the
strong label until released or disabled. Native shadow options convert logical
lengths and CSS blur semantics at the rendering boundary. An inspector reading
proves the submitted parameters, not the resulting pixels; screenshot paint
comparison remains a separate conformance requirement.

Popover, PreviewCard and DateRangePicker let the native popup host cast its
shadow beyond the content clip. Their body bounds, placement, collision
handling and response regions stay with ArkUI. A pixel check verifies that
path independently; it does not establish the same falloff for a shadow
directly attached to a control.

Button, WindowFrame, Dialog, Toast, Sidebar, Menu, Select and Tooltip draw their outer shadows with a separate native
Drawing RenderNode. It blurs with the CSS Gaussian sigma, preserves sRGB tint
and alpha, and knocks out the interior after filtering. A canvas with the
shadow's bounds sits behind the native foreground; it neither measures content
nor takes input.
The blur receives an opaque silhouette; compositing then applies the shared
shadow alpha. Keeping low alpha out of the intermediate filter surface
preserves the precision of tinted shadows.
The drawing layer accepts four independent corner radii for joined Button
groups. The native Button remains the foreground, including its focus and
activation; its measured border box determines the shadow and an intrinsic
wrapper keeps the control's width and height.
Touchability is disabled through the custom and native node boundaries, and the
canvas clips to its expanded allocation. This keeps its overflow out of gesture
ancestors' hit regions while retaining the visible blur. A Toast receipt's
foreground owns the response region; its surrounding canvas remains transparent
to input between adjacent receipts.

The targeted lossless-screenshot check compares all four straight edges with
the shared shadow source in three palettes. It does not replace full corner,
motion, or all-theme screenshot conformance. Dialog and Toast also have outer
corner probes using the Gaussian convolution of their actual rounded shape.
Menu, Select and right-click menus check those eight profiles on first open
and reopening, plus dismissal from the visible shadow and removal of the
canvas. Long lists compare every available sample and explicitly report
profiles clipped by the window viewport; short-list checks still require all
eight profiles. Pixel-error limits remain the same for both.
This matters for a short receipt: its corners affect even a straight edge's
midpoint. Those probes do not test the painted border curve or interior pixels.

Tooltip uses a passive native overlay rather than a popup host: presses through
the hint must reach the original controls below it. Its Text layout counts
wrapped lines and gives each the shared line height; the ArkUI padding includes
the border inset that CSS accounts for separately. Hover waits for pointer rest
using the shared `tooltip.open-delay`; focus opens immediately, activation and
Escape dismiss, and disabled hints leave their controls enabled. Sidebar scopes
retain React's zero hover delay. Provider warm groups, rich labels, aligned-edge
collision behavior, motion and assistive technology still need native coverage.
IconButton installs a label Tooltip automatically. An explicit Tooltip around
it yields to the inner hint in React, ArkUI and SwiftUI, including keyboard
focus;
otherwise nested Base UI triggers can display two hints and leave one open
after activation. Native key observation runs before the Button consumes
activation, while the control retains its own action. Scope registration is removed
when the inner hint unmounts so an outer control can expose its hint again.

DateRangePicker focuses its real popup body after touch opening and its first
navigation button after keyboard/mouse opening. Its outer keyboard scope keeps
native Tab traversal through the controls and dispatches boundary traversal
through the page after returning focus to the trigger. The body must not use a
negative native Tab index: ArkUI then omits the footer from traversal. Focus
observers are window-scoped and unsubscribe when the popup disappears.

HarmonyOS presentation WindowFrame traffic lights use a separate native Drawing
layer for the platform circle, inside black ring, and modern macOS gloss and
white inset. Their constants follow the React chrome stylesheet, independently
from ds themes. The native Button keeps activation and disabled semantics;
the paint sits outside its disabled-opacity subtree. Glyph paths and their
viewport use physical-pixel coordinates, with the SVG viewport scaled to the
logical icon size. Every native drawing boundary excludes input.
The macOS hover group is the lights' own height, including the gaps between
lights, rather than the titlebar's full height.

The traffic-light screenshot comparison runs the actual React WindowFrame at
the native density. It aligns raster phase using the drawing node's public
paint rectangle: fractional layout coordinates alone do not describe the
pixel-snapped canvas allocation. Material samples omit the outer 0.75-physical-
pixel antialiasing band; hovered samples use the annulus outside the glyph.
The same two-byte maximum and one-byte RMS limits apply. Glyph ink centres
have a separate one-physical-pixel bound, allowing native allocation rounding.
These checks cover material and centring, not complete glyph antialiasing,
outer window corners, press animations or all-theme conformance.

WindowFrame provides `dazzui.window-title` as a `DazzWindowTitleStyle` inside
its heading: resolved logical size, weight, inherited body line height and
active/inactive ink. Its replacement on theme/window changes keeps consumed
values live across forwarded LocalBuilder slots; snapshot arguments do not.
Plain titles use the same style. The gallery applies it through the source
Row into its text and glyph, retaining explicit strong/tone overrides instead
of replacing the title with body typography. The heading aligns its title and
subtitle on their baseline. Native numeric font sizes are applied in vp, so
system-following sizes are not scaled twice. A browser comparison executes the
actual React stories and checks native font attributes in three palettes.
A same-process appearance/family check preserves Pin state while updating
the title and glyph's ink. These checks verify style values; glyph
rasterization and narrow-title clipping remain separate requirements.
Plain spans explicitly receive the paragraph's ink rather than calling native
Span.fontColor with undefined, which resets painted ink to black even when the
parent Text inspector reports a themed color. Link and Code runs keep their own
ink rules. Lossless title/body probes check visible contrast and channel bounds
against React's computed ink/surface colors, allowing two byte levels for
compositing precision. They reject that former black-span rendering without
assuming the two font engines draw identical glyph coverage.

Floating sidebar cards keep their shadow outside the icon-mode content clip.
The inset pane uses the same small shadow on the column-colored ground. Main
menu buttons keep the leading edge while badges align independently to the
trailing edge. A noninteractive row handles badge alignment, and scene addresses
bind the painted label. The button's glyph cannot shrink; its text takes the
remaining row width, so truncation respects the reserved trailing space even
under system typography. Collapsible columns retain React's stacking above the
following pane, through both the component and scene wrappers, so their shadows
stay visible over adjacent content.

The native Dialog keeps its rounded content clipping separate from that shadow.
Its vertical Flex preserves the header and footer while the body owns a native
Scroll viewport. The scene renderer's address wrappers preserve those same
shrink rules. Its maximum size leaves `spacing.6` on each side of the viewport,
matching React's fixed viewport padding rather than a percentage inset.
