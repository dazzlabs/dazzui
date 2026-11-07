# DazzUI · Native Shell

The Rust desktop implementation for [Native Shell](https://nsh.dev/), using
NSH's native views and `nsh-base` interaction primitives. This is a separate
Cargo workspace, outside pnpm. It does not depend on the GPUI implementation,
a WebView, JavaScript, Node or ds at runtime.

Requires Rust with edition 2024 support and Native Shell **0.4.0**. The
library uses `nsh-core`, `nsh-base` and `chrono`; an application chooses its
platform backend. The example selects the macOS, Windows or Linux backend
at compile time. Native launch has been verified on macOS, Windows x64
(`windows-laptop`) and Linux x64 with GTK 4.24.1 / Hyprland (`omarchy`).
Full native interaction and layout collection have only been verified on macOS.
See [NSH's documentation](https://nsh.dev/) for the
host toolchain and the `nsh` CLI used by native tests.

```sh
# From this directory; Node and pnpm are not needed
cargo run -p dazzui-nsh-example
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings

# Equivalent commands from the repository root
pnpm nsh
pnpm nsh:build
pnpm nsh:test
pnpm nsh:test:native     # requires nsh 0.4.0 and a desktop display
```

For a Windows SSH session, build with `cargo build --locked -j 1 -p
dazzui-nsh-example`, then run `powershell -NoProfile -ExecutionPolicy Bypass
-File example\tool\desktop.ps1`. The launcher uses a temporary scheduled task
to open (or reuse) the gallery in the signed-in user's desktop. It verifies
the window from that session and returns its title, process ID and window
handle; session 0 cannot enumerate desktop windows. New launches write logs
under `target/desktop.*.log`. Serial compilation avoids peak-memory failures
on a laptop; use `cargo test --workspace -j 1` there as well.

For Linux SSH, run the built executable through the graphical user's
systemd manager, which must already have the Wayland/X11 session environment:

```sh
systemd-run --user --collect --unit=dazzui-nsh \
  --working-directory="$PWD" "$PWD/target/debug/dazzui-nsh-example"
```

## Web gallery

Live gallery: [dazzui-nsh-example.lijy91.workers.dev](https://dazzui-nsh-example.lijy91.workers.dev).

The same Rust library and gallery compile to NSH's HTML/CSS web backend.
The browser runs the Rust engine and application in WebAssembly. Chrono's
`wasmbind` feature supplies the browser clock/timezone for calendars and date
range presets; a bare `std::time` clock is unavailable on this target.

```sh
pnpm nsh:web          # NSH web development server
pnpm nsh:web:build    # release static site, including the Rust Wasm runtime
pnpm nsh:web:deploy   # build, then publish the separate Cloudflare Worker
pnpm nsh:web:test https://your-deployment.workers.dev
```

The static package is `target/web/DazzUI — Native Shell/` inside this Rust
workspace. `example/wrangler.jsonc` serves it through Workers Static Assets
as `dazzui-nsh-example`, with SPA fallback and no server-side application.
Wrangler uses the current authenticated Cloudflare account; no credentials
are written into the package. Run `pnpm install` at the repository root for
Wrangler and the existing React example's Playwright browser test harness.
If NSH reports a wasm-bindgen version mismatch, install the version it names
locally: `cargo install wasm-bindgen-cli --version 0.2.130 --locked --root
target/web-tools` from this Rust workspace. The build script checks that local
`bin` directory before the host PATH; the CLI and Cargo-resolved crate must match.
The web smoke test visits all 53 pages and operates tabs, multi-selection,
menus, dialogs, toasts and theme switching, failing on browser errors.

Add `dazzui-nsh` as a path dependency until it is published. Supply the theme
inside an NSH component, then compose its builders with NSH's ordinary views,
text, signals and slots:

```rust,no_run
use dazzui_nsh::prelude::*;

#[component]
fn Settings() -> impl IntoElement {
    let theme = Theme::new("studio-light");
    provide_context(theme);
    let name = signal(SharedString::from("My project"));
    view().gap(px(16.0))
        .child(form_field().label("Project name").hint("Shown in the sidebar")
            .child(text_field().value(move || Some(name.get()))
                .on_change(move |next: &SharedString| name.set(next.clone()))))
        .child(switch().accessibility_label("Notifications").default_checked(true))
        .child(button().variant(Variant::Filled).child(text("Save"))
            .on_press(move || println!("{}", name.get())))
        .child(button().child(text("Dark appearance"))
            .on_press(move || { theme.set("studio-dark"); }))
}
```

`Theme::all()` contains all 72 ds themes. Themes are scoped to the owning
component, not process globals. `theme_scope().name(...)` overrides a subtree.
A change updates bound styles without reconstructing inputs, resetting state
or losing focus. Unknown names passed to `Theme::set` return `false` and leave
the active theme unchanged. A missing scope uses Bright Light.

`size`, `tint` and `variant` read the same profiles and recipes as React.
Controlled properties use `Some(value)` and report changes to the caller;
`None` uses local state initialized by the corresponding `default_*` property.
Select uses NSH's `SelectValue` to distinguish an explicitly empty selection
from an uncontrolled value. `radio_group` and `segmented_control` take native
`SelectItem`s; the native group owns arrow keys, disabled-item skipping and
Tab focus. `FormField` supplies the name and hint of its TextField or Select.
NSH 0.4.0 does not expose a native invalid accessibility state: errors have
the DazzUI drawing and hint, without a separate native invalid flag.

The library covers the gallery's 53 component pages, including compound
controls: accordions, tables, trees, splitters, sidebars, forms, code blocks,
number/search/OTP fields, comboboxes and autocomplete, grouped choices,
calendars and date ranges, dialogs, drawers, menus and toasts. Spinner,
indeterminate progress and skeleton motion use native animation. Loading
motion and track opacity come from ds; reduced motion stops the skeleton
and slows the spinner, as in React.

The generated [coverage report](example/coverage.json) maps **189 of 189 shared
scenes**. This is scene coverage, not a claim of full geometry or behavior
conformance. Unknown components, props, item data and actions still fail closed
as explicit gaps. `icon` embeds React's Fluent paths, such as
`Checkmark12Filled`; applications can also supply SVGs through NSH directly.

`DateRangePicker` keeps an isolated draft and commits only on Done. Cancel,
Escape and outside dismissal discard it; moving the start time preserves the
duration. `ToastProvider` supplies `ToastHandle::current()`; place a
`ToastViewport` under the same provider. `Form` validates registered text
editors before Submit and restores their defaults on Reset. A drawer's footer
is a renderer (`.footer(|_open: &bool| dialog_close().content(...))`) so close
controls are constructed within the drawer's scope.

The calendar draws six complete weeks and keeps a roving date focus. Arrow keys,
Home/End, Page Up/Down and Shift+Page Up/Down move across days, weeks, months and
years without selecting until Enter/Space. Bounds constrain both pointer and
keyboard actions. `today` seeds an empty calendar as well as its marker. Range
selection reports the first pick, accepts either direction, previews the pending
span and exposes the whole committed range to accessibility. NSH owns native
focus and key dispatch. The shortcut recorder cannot intercept OS-reserved
shortcuts or show modifiers held without a key.

On macOS, a narrow [NSH renderer patch](vendor/nsh-platform-macos/PATCHES.md)
preserves the numeric `wght` axis of the system variable font. AppKit's named
semibold is 590, while Chromium's CSS 600 is 600. The patch changes real painting,
editing and measurement together. Single-line labels retain fractional font
advances; wrapping paragraphs use the actual native line count with ds leading.

## Generated sources

- `pnpm tokens` emits `src/generated/theme_tokens.rs` through
  `ds/plugins/nsh.mjs`. The Rust representation is shared with the GPUI token
  emitter; toolkit color, shadow and font adapters live in this crate.
- `pnpm codegen` emits `src/generated/glyphs.rs`, the gallery's JSON stories,
  registry, theme families and coverage from React's declarations.
- Commit these outputs. Do not edit them. A fresh Cargo build needs neither
  generator and no other platform package.

The gallery's interpreter binds portable expressions to signals. Theme and
control changes update the current view. Each node is instantiated in its
mounted scope, so compound controls inherit their provider. Unsupported props,
data shapes and actions produce explicit coverage gaps.

## Verification and collection

`cargo test --workspace` exercises controlled and uncontrolled state, theme
changes, nested scopes, disabled choices, keyboard navigation, popup dismissal
and labels. It also checks form validation/reset, OTP paste, numeric bounds,
range commit/cancel, overlay close buttons, menu switching, accordion/sidebar
state retention, tree expansion, splitter bounds, shortcut recording and toast
actions/timeouts. It renders every mapped scene in Bright Light and Studio Dark,
cycles through every scene to exercise disposal, and checks the four button
heights and specimen addresses in all 72 themes. The suite passes on macOS,
Windows x64 and Linux x64.
These tests run the real NSH engine with the build host's desktop environment
and a headless text measurer. The two `macos_headless_*` keyboard tests
explicitly use the macOS simulation on every host: `nsh-testing` 0.4.0 only
models view focus for macOS and web. They do not verify Windows/Linux native
keyboard focus, and headless tests do not replace native rendering checks.

`pnpm nsh:test:native` opens a real desktop window, navigates the gallery,
switches theme and appearance, operates controls, and saves PNGs under
`target/`. Native UI tests need a graphical session.

```sh
# All scenes in the twelve drawn themes, with native screenshots and crops
pnpm nsh:conformance
# Focused reference/implementation run, published through shared storage
pnpm conformance --platforms react,nsh --themes bright-light,studio-dark --scenes sidebar,button/preview
pnpm conformance:check
# Update only one scene; keep unrelated snapshots intact
pnpm conformance --incremental --platforms react,nsh --themes bright-light --scenes sidebar/preview --no-images
# Exercise collection, crops, incremental image removal and failed-run retention
pnpm --filter @dazzlabs/dazzui-conformance test:nsh
```

NSH participates in default collection and checks. `tools/conformance/expectations/nsh.txt`
has no geometry exceptions after the 188-scene, twelve-theme macOS run;
it follows the same only-shrinks rule as the other implementations. Codegen rejects new unmapped
NSH scenes or props. Passing the gate means no new measured regressions, not
that every visual property matches React.

The collector uses an intrinsic-height, 640-point stage with the same padding,
body typography and centered content as React. It opens a real desktop host at
1024 × 768, enlarging it for taller scenes. Component refs attach specimen
addresses without a layout wrapper. `ElementRef::measure()` records actual
logical-point boxes; screenshots and crops preserve native pixels at the
host's display scale. The CLI freezes time and reduces motion. Controls in
unopened overlays have no rendered box and are not measured. The empty toast
viewport stays mounted as an actual native overlay, rather than a fragment
whose reference would resolve to its parent.

The collector accepts `--out`, `--themes drawn|all|name,name`, `--scenes`
(story IDs or exact scene IDs) and `--no-images`. Runtime errors fail collection
even if navigation succeeds. Shared orchestration publishes only successful
runs, retains unrelated snapshots on incremental updates, and removes stale
image references when recollecting without images. Each stage owns its theme
so switching specimens cannot schedule old scene bindings during disposal.
With `--no-images`, the host advances after each completed batch of native
measurements, avoiding per-scene screenshots and test-driver round trips.

Decoration and text metrics remain unread: `values` is empty and `font` is
null rather than derived from tokens. Geometry is verified on the primary
Retina display at its actual 2× scale. The exception file only shrinks; no
tolerance or measurement normalization is applied to make the port pass.
Native layout and interaction collection are verified on macOS. Windows x64 and Omarchy Linux
x64 are checked with workspace tests, builds and visible desktop launch; they
are not claimed to share the macOS visual baseline.
