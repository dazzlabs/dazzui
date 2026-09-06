# dazzui (GPUI)

The DazzUI design system for [GPUI] — the platform sibling of
`packages/ui_react`, `packages/ui_flutter` and `packages/ui_swiftui`. Same
tokens, same four themes, same components.

**`packages/ui_react` is the standard.** Every name, prop and metric here is
read off the React source; where the two disagree, the React file wins.

## Using it

```rust
use dazzui::prelude::*;
use dazzui::{Button, ControlVariant, Theme, ThemeName};
use gpui::{App, Application};

fn main() {
    Application::new()
        .with_assets(dazzui::assets::Assets)
        .run(|cx: &mut App| {
            Theme::set(ThemeName::StudioLight, cx);
            // … open a window whose views build with the components:
            // Button::new("translate", "Translate").variant(ControlVariant::Filled)
        });
}
```

A component reads the theme from the app, and [`themed`] scopes one to a
subtree the way React's `data-theme` attribute does. Themes are per-subtree,
so several can sit on screen at once.

## Naming

Names match the React export **exactly**, with no prefix — `Button` is
`Button`, `Divider` is `Divider`, `Table` is `Table`. The colour axis is
`tint` (`Tint::Primary`), never `kind` or `secondary`: the ds vocabulary is
the API here as much as it is in Flutter and Swift.

A component is a builder: `Button::new(id, label)` then `.variant(…)`,
`.tint(…)`, `.size(…)`, `.on_click(…)`. The id is GPUI's — an element that
answers the pointer needs one to keep its state between frames.

## Tokens

`src/generated/` holds what `packages/ds` wrote and is committed, so the crate
builds and ships on its own — there is no dependency edge to the design system
and so no build order to get wrong. **Never hand-edit it**: a value a
component needs belongs in `packages/ds`, and anything written there is
overwritten on the next `pnpm tokens`.

Two files come out of the generator, beside a hand-written `mod.rs` that is
only two declarations:

- `generated/colors.rs` — the base ramps as `ColorSwatch` constants
- `generated/theme_variables.rs` — the semantic and component layers as one
  struct, plus one `static` per theme

Rust has no cascade, no default arguments and no computed properties, so a
theme cannot be a block of overrides layered on at render time. The generator
splits the token set in two instead: `ThemeSeed` is what a theme may
re-point, and `ThemeVariables::resolve` is a `const fn` deriving everything
written in terms of it. A theme is then

```rust
pub static STUDIO_DARK: ThemeVariables = ThemeVariables::resolve(ThemeSeed {
    color_canvas: Hsla { … },
    ..STUDIO_LIGHT_SEED
});
```

— only what moves is named, and every recipe built on a re-pointed role
follows on its own. It is all `const`, which is what lets each theme be a
`static`: a component holds `&'static ThemeVariables` and nothing is allocated
or recomputed per frame.

| `ThemeName`          | Feel |
| -------------------- | ---- |
| `StudioLight`        | White card, light grey canvas, violet accent. **The baseline** |
| `StudioDark`         | The same skeleton over a near-black canvas |
| `BrightLight`        | Warm paper, ink navy, acid-green marker, pill controls |
| `BrightDark`         | Ink-navy canvas, with acid green as the accent itself |

## Foundation

The hand-written half of the vocabulary — the types the generated file is
written against, and the interaction primitives CSS gets from pseudo-classes:

- `ColorSwatch`, `ColorRecipe`, `ColorDescriptor`, `TypeStyle`, `TokenShadow`,
  `BezierCurve`
- `Tint`, `ControlVariant`, `ControlSize`, `ControlState`, `ControlPart`, and
  the `control_colors(variant, tint, state)` resolution every interactive
  component reads — the Rust half of `data-control` plus `data-variant`
- `pressable`, `latched`, `focus_ring`, `hairline`, `rule`
- `StyledExt`, which puts a face (`type_style`), a shadow stack
  (`token_shadow`) or a resolved recipe (`control_colors`) onto an element

Interactive colours are stored as *recipes* rather than finished colours: a
shade to pick off a ramp plus an alpha to apply, resolved against whichever
ramp the control is tinted with. A component names a state, never a shade.

## Icons

The set is Fluent — Microsoft's, the same set `packages/ui_react` pulls from
`@fluentui/react-icons` and `packages/ui_swiftui` takes from the official
Swift package. That parity is the point: the React components name specific
glyphs, so a port drawing its own checkmark would be a different mark at a
different weight in the same box.

| React component | Glyph |
| --- | --- |
| `checkbox.tsx`, `menu.tsx`, `step-list.tsx` | `Checkmark12Filled` |
| `search-field.tsx`, `toast.tsx` | `Dismiss12Regular` |
| `search-field.tsx` | `Search16Regular` |
| `input.tsx` | `ChevronDown12Regular` |
| `shortcut-recorder.tsx` | `DismissCircle12Filled` |

`Icon::new(path)` takes any path the app's asset source can resolve; the
glyphs this crate carries are in `assets::icons`, embedded in the binary and
served by `assets::Assets`. A host with an asset source of its own chains
`Assets::bytes` into it rather than replacing it. GPUI paints an SVG as a mask
in the element's own text colour, so an icon with no colour of its own reads
the ambient one — the same thing `fill: currentColor` does on the web.

## Known divergences

Everything not listed here is a value-for-value port.

| What | Why |
| --- | --- |
| A per-theme *shade* on a recipe naming a semantic role | Such a recipe is derived from this theme's roles, so it has no seed field to re-point. The same limit the Flutter and SwiftUI ports carry. |
| Hover and press are read from the previous frame | GPUI resolves its own `hover()` and `active()` refinements a phase *after* a text run's colour is baked, so a recipe whose content darkens under the pointer would move its surface and leave its label behind. `pressable` decides the state before the subtree is built instead, which is what keeps every part of a recipe together. It reads the pointer against the last frame's bounds, so a control under a scrim still lights up. |
| No colour transition | `motion.duration` and `motion.easing` reach Rust, but GPUI has no transition on a style: a colour changes on the frame the state does. `BezierCurve::ease` is there for the animations that are hand-driven. |
| Keyboard focus is passed in | A focus handle belongs to the view that owns it, so a component draws the ring for a handle it is given (`.focus_handle(&handle)`) rather than keeping one. The two controls that must own focus — `TextBuffer` and `ShortcutRecorder` — do. |
| `TextBuffer` has no React counterpart | The web hands a component the DOM's own editing; GPUI hands it nothing. `TextBuffer` is the buffer, the selection and the focus a `TextField` and a `SearchField` are drawn around. |
| A `Button` takes a label and an optional icon | React takes children. A button's label is a word, and an element tree in the place of one is a composition the caller can build out of a `div`. |
| `Select` opens a `Menu` | The web has a native select; GPUI has none, so the control wears the text box and opens the system's own popover under it. |
| The mono face resolves rather than being named | `text-field.css` reaches for `--base-font-code`, whose stack leads with `ui-monospace` — a request for the platform's monospaced face rather than a family to look up. GPUI names the system UI face (`.SystemUIFont`) but not that one, so the stack's next real family is named instead. (`base.font.code` has no semantic token in front of it; a component reaching a `base.*` primitive is the graph's own rule being bent, and it is a `packages/ds` decision to unbend.) |
| A menu closes on an outside press, not on Escape | Escape is a key the window routes to whatever has focus, and a menu here takes none. |

## Building

```bash
cargo build
```

```bash
cargo test
```

Both from this directory. The tests are what a design system can be checked
for without a screen: the recipes resolve the way the design states them, a
control is the height its size profile says, every component lays out and
paints in all four themes, and the pointer and the keyboard reach the things
they are supposed to — a hover, a click, typing into a field, a chord into the
recorder, a menu opening and choosing. On a Mac without the Metal toolchain installed,
GPUI's build script cannot compile its shaders — the error names the fix
(`xcodebuild -downloadComponent MetalToolchain`). Until it is installed, build
with the feature that switches GPUI to compiling them at launch instead:

```bash
cargo build --features runtime-shaders
```

It changes nothing about what is drawn.

## Playground

`example/` holds the playground — one page per component, organised the way
`apps/storybook` is, with a Tokens group beside it and a Themes page that
renders the same view under all four themes at once:

```bash
cargo run -p dazzui_playground
```

`cargo test -p dazzui_playground` draws every story in every theme
headlessly, which is where a story nobody has opened lately fails. See its own
README for the conventions the pages are written to.

[GPUI]: https://gpui.rs
[`themed`]: src/theme.rs
