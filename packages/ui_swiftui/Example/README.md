# Playground

The DazzUI playground, the SwiftUI counterpart of
`packages/ui_flutter/example`.

Open it in Xcode and press ⌘R:

```bash
open Playground.xcodeproj
```

That is the intended way in. The project builds a real `.app` bundle, so the
playground runs, debugs and profiles like any other Mac app.

From the command line, the same scheme:

```bash
xcodebuild -project Playground.xcodeproj -scheme Playground -destination 'platform=macOS' -derivedDataPath .build/xcode build
open .build/xcode/Build/Products/Debug/Playground.app
```

Sources are picked up from `Sources/Playground` by a synchronised folder
reference, so a new file in that directory joins the target with no project
edit.

The app resolves `DazzUI` by path, so it always builds the working copy
beside it rather than a published version.

**`swift build` and `swift run` do not work here**, and the reason is the icon
dependency rather than anything about this app: Fluent's Swift package never
declares its asset catalog as a resource, so SwiftPM alone leaves
`Bundle.module` unsynthesised and fails to compile it. Xcode's build system
processes the catalog, which is why the project exists. The package README has
the detail.

## What is in it

Six pages, reached from a sidebar built out of the package's own `Sidebar` and
`NavItem` — the playground is drawn with the design system it documents, so a
theme switch carries the furniture with it too.

| Page | What it shows |
| --- | --- |
| Controls | Every recipe, size and tint; icon buttons, badges, keys, tabs, segments |
| Forms | A form rather than a shelf of specimens — the field chrome, the label and hint, the error state |
| Data | Cards, a table with selection, progress, spinners, a step list |
| Feedback | Banners, a dialog behind its scrim, a toast, a menu, an empty state |
| Settings | A preferences screen: group over section over rows |
| Themes | **All four themes at once** |

The Themes page is the one worth opening first. A theme here is a value scoped
to a subtree with `.theme(_:)` rather than a process-wide mode, so the four
cards on that page are the same view rendered four times, each resolving
against its own variables. The ramp rows under them are a reminder that a ramp
is a role ladder rather than a lightness ramp — Bright's ink ladder puts a
lighter colour at 700 than at 600.

## Why it is a separate package

The library must not carry an entry point, a window, or a dependency that only
the playground needs — the same reason the Flutter example is its own app
rather than a target of `dazzui`.

The build produces a plain executable rather than an app bundle, which
launches without a dock presence and behind whatever window has focus. The
`AppDelegate` asks for the regular activation policy, which is what brings it
forward.
