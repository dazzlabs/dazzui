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
  written down.** Three components own their own: `Tree`, whose keyboard is
  not a list's — `ArrowRight` opens a shut branch and descends into an open
  one, `ArrowLeft` shuts one and climbs out of a leaf, and the whole tree is
  one tab stop, none of which falls out of a `Collapsible` per node;
  `Splitter`, whose seams are a pointer gesture with an `aria-valuenow`
  rather than a state machine; and `Skeleton`, which has no behaviour at all.
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
