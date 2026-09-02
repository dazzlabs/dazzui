# Playground

The `dazzui` playground, the GPUI counterpart of
`packages/ui_flutter/example` and `packages/ui_swiftui/Example`.

```bash
cargo run -p dazzui_playground
```

From either this directory or the one above it — the two crates share a
workspace, so the binary lands in one `target/`. On a Mac without the Metal
toolchain, add `--features runtime-shaders`; the package README says why.

The app resolves `dazzui` by path, so it always builds the working copy
beside it rather than a published version.

## What is in it

Six pages, reached from a sidebar built out of the crate's own `Sidebar` and
`NavItem` — the playground is drawn with the design system it documents, so a
theme switch carries the furniture with it too.

| Page | What it shows |
| --- | --- |
| Controls | Every recipe, size and tint; icon buttons, badges, keys, tabs, segments |
| Forms | A form rather than a shelf of specimens — the field chrome, the label and hint, the error state |
| Data | Cards, a table with selection, progress, spinners, a step list |
| Feedback | Callouts, a dialog behind its scrim, a toast, a menu, an empty state |
| Settings | A preferences screen: group over section over rows |
| Themes | **All four themes at once** |

The Themes page is the one worth opening first. A theme here is a value scoped
to a subtree with `themed(…)` rather than a process-wide mode, so the four
cards on that page are the same view rendered four times, each resolving
against its own variables. The ramp rows under them are a reminder that a ramp
is a role ladder rather than a lightness ramp — Bright's ink ladder puts a
lighter colour at 700 than at 600.

The fields on the Forms page are real: type into them, select with the mouse
or with shift and the arrow keys, and paste. The recorder on the Settings page
listens for a chord and prints it the way a menu would.

## Why it is a separate package

The library must not carry an entry point, a window, or a dependency that only
the playground needs — the same reason the Flutter and SwiftUI examples are
their own apps rather than targets of the library. It is a workspace member
rather than a detached crate so the two share one lockfile and one `target/`.

## Where the state lives

One view holds all of it. The pages are specimens rather than screens: a page
has no state a neighbouring page could not read, and threading six entities
through the window would say otherwise. Each page is a function of that state
and the entity to update it — `pages/controls.rs` and friends — which keeps
the page files about what the components look like rather than about how GPUI
routes a callback.
