# dazzui (GPUI)

The DazzUI design system for [GPUI](https://gpui.rs): the same tokens, the
same seventy themes and the same components as `@dazzlabs/dazzui`, drawn to
its geometry.

**`packages/js/dazzui` is the standard.** Every name, prop and length here is
read off the React source; where the two disagree, React wins, and
`pnpm conformance` measures by how much.

## Using it

```rust
use dazzui::{Button, Theme, Variant};
use gpui::{App, Application};

fn main() {
    Application::new()
        .with_assets(dazzui::Assets)
        .run(|cx: &mut App| {
            dazzui::init(cx);
            Theme::set("studio-dark", cx);
            // … open a window whose views build with the components:
            // Button::new("save").variant(Variant::Filled).child("Save")
        });
}
```

`dazzui::init` installs the theme and the key bindings the kit's controls
answer to. `dazzui::Assets` serves the glyphs the kit draws; an app with
glyphs of its own serves them and falls through to it, as the example's
`lib/assets.rs` does.

A component is a builder named as React exports it: `Button::new(id)` then
`.variant(…)`, `.tint(…)`, `.size(…)`, `.on_click(…)`, and `.child(…)` where
React takes children. The id is GPUI's — an element that answers the
pointer needs one to keep its state between frames. A prop React leaves to
the component (`defaultValue`, `defaultChecked`) is held by the component;
`value` with its change handler makes it the caller's.

A date is `chrono`'s: `Calendar` takes and gives back a `NaiveDate`, and
`DateRangePicker` a `DateRangeValue` of two `NaiveDateTime`s, civil and
local, since the kit draws a calendar and does not do calendars. The crate
is re-exported as `dazzui::chrono`, so an app reads the same version.

Controls prepare plain string children with the shared font stack and
display-pixel measurement. For text drawn directly with GPUI, use
`dazzui::foundation::text_child`; rich elements pass through unchanged.

## Themes

`Theme::set(name, cx)` sets the app's theme by the name React selects it with
(`data-theme`); `themed(entry, child)` draws a subtree in another, the way
`data-theme` on an element does. A component reads the theme in force with
`cx.theme()`, a `&'static ThemeTokens` whose fields are the CSS custom
properties by name: `--control-medium-size` is `t.control_medium_size`.

`src/generated/theme_tokens.rs` is what `ds/plugins/gpui.mjs` wrote. Every
theme is a `const` naming only what differs from Bright Light's, over
`..BRIGHT_LIGHT`, so nothing is derived at run time except the tint: a
recipe cell that picks a shade off the control's ramp stays a
`Paint::Shade`, resolved against the ramp where the control is drawn.
`src/glyphs.rs` is written by `pnpm codegen` from the Fluent icons the
React components import. **Never hand-edit either.**

## Building

```bash
cargo build --features runtime-shaders
```

```bash
cargo test --workspace --features runtime-shaders
```

GPUI compiles its Metal shaders in a build script, which needs the Metal
toolchain (`xcodebuild -downloadComponent MetalToolchain`). Without it,
`runtime-shaders` compiles them at launch instead; what is drawn is the
same.

On Windows and Linux, use `cargo build --workspace` and
`cargo test --workspace` without the Metal feature. Input tests use the
platform's own selection, word movement and clipboard shortcuts. Linux's
UI and code fonts follow the host's Fontconfig generic-family settings.

On macOS, `compat/font-kit` keeps GPUI's system UI font instances at the
CSS weights React uses. Core Text's native Medium and Semibold instances
have variable weights 510 and 590; matching their names alone does not
produce CSS 500 and 600. The adapter changes the native fonts used for
both shaping and painting, leaving other families and platforms upstream.
It wraps the exact upstream commit shipped as `0.14.1-zed`, pinned in
`Cargo.lock`. Cargo applies patches
only from the workspace root; an app embedding this crate from another
workspace must include the same patch, with the path adjusted:

```toml
[patch.crates-io]
zed-font-kit = { path = "path/to/dazzui/compat/font-kit" }
gpui = { path = "path/to/dazzui/compat/gpui" }
```

Geometry collection reads native boxes; paint and interaction need separate
verification. Shadows translate CSS blur lengths to GPUI's Gaussian sigma,
and Slider reads the theme's thumb ink, edge, lift and hover scale. Popup
type-ahead, hover delays, feedback timers and text leading use the generated
tokens rather than copies of their current values.

Unhandled Tab and Shift-Tab traverse GPUI's native tab order; a host can
consume them first. Dialog focuses the first eligible control; Drawer
initially focuses its panel. Both contain traversal and restore the previous
focus after the exit finishes, including nested sheets and dropdowns.
Kit portals enqueue nested layers together rather than
nesting SDK deferred draws; detached layout preserves namespaces, providers
and subtree themes. Slider pointer changes push neighbouring thumbs while
keyboard changes stop at them, and a drag retains its initial grab offset.

Button and IconButton follow native web button timing: Enter activates on
keydown, including repeats; Space holds the pressed recipe and activates
once on release. Losing focus, becoming disabled or preventing the key's
default cancels the pending activation. Keyboard clicks carry the actual
control bounds. Dialog and Drawer trigger/close wrappers share that timing,
so releasing an opening Enter cannot click the newly focused close button.
Hover, focus and pressed ink merge with the control's base text properties;
they keep its face, size, weight and leading instead of inheriting the page's
typography and changing the control's width.

Skeleton sweeps, Spinner turns and Progress sweeps share a loading clock and
the CSS cubic Bézier curves. Determinate progress transitions over twice the
theme's motion duration and reverses from its current painted width. Give
progress bars built at the same call site in a loop a distinct `.id(...)`.

`MotionPreference::System` follows macOS Reduce Motion, including changes
while the window is idle. Other hosts can forward their accessibility
setting with `Motion::set_preference(MotionPreference::Reduced, cx)`.
Reduced motion settles Skeleton's highlight, slows Spinner and the
indeterminate bar, and jumps directly to a determinate value.
`Motion::freeze_at(duration, cx)` fixes loading at an exact time and settles
value transitions; `Motion::resume(cx)` resumes the loading clock. The
native and React collectors accept `--motion-ms 400` and optionally
`--reduced-motion` for matching animation snapshots when called directly.

Drawer translates its measured sheet and hitboxes from the selected edge;
the scrim fades independently. Closing sheets stay mounted and retain focus
until the last frame. Reopening during exit preserves the original return
focus. Interrupted transitions use the
[CSS reversal shortening rule](https://www.w3.org/TR/css-transitions-1/#reversing).
An initially open sheet starts at rest. `Motion::seek(time, cx)` pauses a
transition timeline without settling its values.

The committed GPUI compatibility patch gives the Metal renderer isolated
subtree opacity and rounded overflow clipping. Dialog's header, body and
footer compose before fading, and its child backgrounds stay inside the
rounded padding edge. Cached views retain their original paint ranges.
Dialog scales the whole sheet around its centre from `dialog.transition-scale`
to one. Hitboxes and pointer coordinates follow that transform, including
nested scales and cached views; input-method rectangles and character probes
map between the same coordinates. Detached menus anchor to painted trigger
boxes. Other renderer backends still use upstream opacity, rectangular
clipping and unscaled layout. See
[`compat/gpui/DAZZUI-PATCH.md`](compat/gpui/DAZZUI-PATCH.md).

## The example

`example/` is the React example's window, drawn with this package: the
component list, the document, the theme panel. Its stories are compiled from
the React ones by `pnpm codegen` into `example/src/stories/`; see
[`specs/playgrounds.md`](../../../specs/playgrounds.md).

The shell constrains all three columns to the window's height. Each owns its
scroll handle; the theme panel uses SidebarContent's scroll container. A
SidebarContent host may pass `.scroll_handle(handle)` to retain or control a
column's position.

```bash
cargo run -p dazzui-example --features runtime-shaders
```

`--story <id>` opens on a component, `--theme <name>` in a theme.
`example/tool/conformance.sh` is its conformance collector; see
[`specs/conformance.md`](../../../specs/conformance.md).
On Linux and Windows, run the example executable with `--conformance`,
`--out <directory>`, `--contract <repo>/tools/conformance/scenes.json`, and
`--no-images`. It needs the signed-in user's desktop session. The readings
record the actual display scale; screenshot capture currently uses macOS's
`screencapture`.
