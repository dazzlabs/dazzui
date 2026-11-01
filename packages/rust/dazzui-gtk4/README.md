# DazzUI · GTK4

The Rust/gtk4-rs desktop implementation. GTK owns input, focus, selection,
keyboard navigation, accessibility and popup positioning; DazzUI draws the
native CSS nodes with the shared ds tokens. This crate is a separate Cargo
workspace beside the GPUI implementation and is invisible to pnpm.

Requires Rust and GTK4 **4.16 or later** with development headers and
`pkg-config` and `glib-compile-resources`. The minimum follows GTK's
introduction of CSS custom properties.
On macOS: `brew install gtk4 pkg-config`. On Linux, install the corresponding
GTK4 development package; Ubuntu 24.04's stock GTK 4.14 needs an upgrade.

```sh
# From this directory
cargo build --workspace
cargo run -p dazzui-gtk4-example
G_DEBUG=fatal-warnings cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Tests open native windows and must run with a display. On a Linux CI machine,
use `xvfb-run -a env G_DEBUG=fatal-warnings cargo test --workspace`. GTK runs
on the main process thread; the native test binaries deliberately do not use
the threaded Rust test harness.

```rust,no_run
use dazzui_gtk4::{gtk, prelude::*, *};

gtk::init().unwrap();
let window = gtk::Window::new();
Theme::new("bright-light").unwrap().apply(&window);
let column = box_(gtk::Orientation::Vertical, "3");
let email = text_field("you@example.com", Size::Medium);
column.append(&form_field("Email", &email, Some("Used for receipts")));
let save = button("Save", Variant::Filled, Tint::Primary, Size::Medium);
save.connect_clicked(|_| println!("saved"));
column.append(&save);
window.set_child(Some(&column));
```

Every constructor returns its actual GTK type. Connect native signals and
read/write state with gtk4-rs, rather than learning another state API. Use
`set_dazz_size`, `set_dazz_tint`, `set_dazz_variant` and `set_dazz_invalid`
for drawing axes. `icon` loads the embedded Fluent glyph by its React export
name. Icon buttons also accept host icon-theme names and require an
accessible label. `glyph_button` puts an icon-theme glyph before or
after a label, sized, spaced and padded as React's Button does;
`glyph_segmented_control` does the same for segments, or draws each glyph
alone with the label as its accessible name. Table and tree expose GTK's selection models and factories.
`combobox` is a searchable model selector; free-form autocomplete is not yet
provided. `tag` is a native selectable toggle.

`Theme::apply` installs the embedded sheets once per display and selects a
complete token snapshot on a subtree. All 70 ds themes are included. Nested
scopes reset every token, so an inner light scope does not inherit a dark
palette. Changing themes preserves the widgets and their state. Modal windows
carry their parent's theme when created; apply a new theme to them separately
when changing the parent while they are open. Application CSS is scoped to
DazzUI classes. The host chooses its theme; GTK's global theme is not replaced.

`src/generated/tokens.css` and `themes.rs` are committed output of
`ds/plugins/gtk4.mjs`. Run `pnpm tokens` at the repository root to regenerate;
never edit these files. `src/generated/icons/` is emitted by `pnpm codegen`
from the same Fluent SVG paths React imports and embedded during the Rust
build. The crate builds without Node or ds installed.

The library includes buttons, toggles, checkboxes, radios, switches, text,
password, multiline, search and number fields, selectors, sliders, progress,
meters, spinners, badges, tags, cards, callouts, typography, code, key caps,
avatars, dividers, links, forms, preferences, collapsibles, splitters, tabs,
segmented controls, option cards, popovers, menus, dialogs, tables, trees and
action bars.

The example follows React's organization:

```text
example/src/
  main.rs          application / collector entry point
  app.rs           window lifecycle
  components/      shell, brand, stage, theme panel and theme preview
  lib/             story model, scene interpreter and generated theme families
  stories/         one generated Rust file per React story, plus registry
  conformance/     native readings and snapshots
```

The three columns are the searchable component list, the document under its
toolbar, and the theme panel. Overview contains Preview cards; a component
document contains its summary and examples (with notes). Native splitters
resize the columns, and toolbar buttons toggle the two side panels.
Theme families and their group headings come from React's `FAMILY_GROUPS`;
the panel switches appearance and displays a miniature of every family.
Use `pnpm gtk4 --story button --theme bright-dark` to open a specific page.

The example's 51 story pages are generated from the React stories by
`pnpm codegen`; it currently renders **103 scenes across 33 component pages**.
Unmapped scenes show their exact missing components or props. Mapped scenes
evaluate shared state expressions and connect native widget signals without
replacing edited fields. `select_options` accepts flat and grouped text
options, placeholders and disabled choices, and `select_values` maps native
positions to domain values. Rich option labels, range sliders and
several advanced families (drawer, toast, OTP, shortcut recorder, skeleton,
sidebar, step list and accordion) still need mapping or implementation. This
is an initial GTK port, not a claim of complete React feature or geometry
conformance. Native GTK text metrics and control internals still need visual
comparison, particularly imported themes.

A collector already reads the generated specimen addresses and writes native
bounds, ink, font size, all four painted corners and 2× scene/specimen images
in the shared format:

```sh
example/tool/conformance.sh --out /tmp/dazzui-gtk4-readings \
  --themes studio-light,bright-dark --scenes button,checkbox,switch
```

Use `--themes drawn|all|<names>`, `--scenes <story ids>`, and `--no-images`.
`gaps.json` records skipped scenes; unread drawing properties are `null`.
The collector uses GTK's actual renderer and the platform's font fallback
(`font: null`), so image comparisons must not claim a shared test face. GTK
has not joined the default conformance run or its shrinking expectation
baselines until the remaining mappings and a measured baseline are reviewed.
