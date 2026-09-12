# DazzUI for Inset

An independent Rust workspace implementing DazzUI controls with
[Inset](https://github.com/inset-rs/inset). The Flutter package is the implementation
reference; all twelve themes come from `packages/ds`. The gallery has 54 component
pages, including editable fields, selection controls, navigation, overlays, menus,
notifications, tables, and split panes.

This is an initial implementation with broad component coverage. It is **not yet
an exact behavioral or pixel-parity port**. Read [PORTING.md](PORTING.md) before
using it as a replacement for the Flutter package.

## Run

The framework is a sibling checkout, shared by the library and example:

```text
dazzlabs/
  inset/
  ui/
    packages/ui_inset/
```

From the UI repository, if the sibling does not already exist:

```sh
git clone https://github.com/inset-rs/inset ../inset
git -C ../inset checkout 2b849ce3616a6421c9821496c27eb92e63562e95
```

That is the framework revision this package was tested against. Path dependencies
allow local framework development; Cargo.lock does not pin their revision. Do not
switch an existing sibling checkout with local work just to run the example.

```sh
pnpm inset        # native playground
pnpm inset:test   # library interactions and all 54 pages in all 12 themes
pnpm inset:check  # clippy, including example and tests
```

Or from this directory:

```sh
cargo run -p dazzui_inset_playground
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
```

`rust-toolchain.toml` selects `nightly-2026-09-02`, as required by the tested
framework revision. The first build compiles Inset's renderer and can take several
minutes. The native window and text entry were tested on macOS. Other Inset
platforms have not been validated here; see the framework's platform setup for
required native dependencies. This package does not depend on a platform-specific
DazzUI implementation.

## Use

```toml
[dependencies]
dazzui_inset = { path = "../ui/packages/ui_inset" }
```

```rust
use dazzui_inset::{Button, ControlVariant, Theme, STUDIO_LIGHT};

let save = Theme::new(
    &STUDIO_LIGHT,
    Button::new("Save")
        .variant(ControlVariant::Filled)
        .on_pressed(|app| {
            // Update your application model here.
        }),
);
```

Mount the widget under your Inset `WidgetsApp` and `View`, just as Flutter hosts
this design system under `WidgetsApp`. Supply `DefaultCupertinoLocalizations`
for the native editable-text plumbing, and a `Navigator` for dialogs and drawers.
The complete host is in `example/src/lib.rs`; application state belongs to Inset
entities or stateful widgets. Selection controls expose callbacks and accept their
current values from the host. A button without `on_pressed` is disabled.

`Theme::new` installs the palette and ambient text style; `Theme::of` resolves it
from a build context. `THEMES` exposes the twelve generated instances. Controls use
`Tint`, `ControlVariant`, and `ControlSize`. Text and icon content inherit the host
font and the control's content color. No Material component library is involved;
`CupertinoTextField` supplies Inset's editable-text behavior with DazzUI styling.

## Sources and verification

- `src/foundation/`: token types, native color recipes, and control axes.
- `src/generated/`: committed ds output; regenerate with `pnpm tokens` at repo root.
- `src/widgets/`: library implementations, exported from the crate root.
- `example/src/stories/`: one module per component page.
- `tests/`: actual widget mounting, pointer activation, token invariants, slider
  bounds, splitter constraints, and toast expiration.
- `example/tests/`: mounts, lays out, pumps animation frames, and unmounts every
  gallery page in every theme (648 combinations).

The Inset generator and GPUI generator share `ds/plugins/rust.mjs` so they use the
same mode composition and recipe expansion, then emit their native types. The
existing GPUI generated sources remain unchanged. Inset is not yet a target of
`pnpm parity`; the gallery tests check successful rendering, not screenshot
identity or full React story coverage.
