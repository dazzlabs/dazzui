# SwiftUI example

The DazzUI example for SwiftUI, the counterpart of
`packages/dart/dazzui/example`.

## macOS

Open it in Xcode and press ⌘R:

```bash
open Example.xcodeproj
```

That is the intended way in. The project builds a real `.app` bundle, so the
example runs, debugs and profiles like any other Mac app.

From the command line, the same scheme:

```bash
xcodebuild -project Example.xcodeproj -scheme Example -destination 'platform=macOS' -derivedDataPath .build/xcode build
open .build/xcode/Build/Products/Debug/Example.app
```

Sources are picked up from `Sources/Example` by a synchronised folder
reference, so a new file in that directory joins the target with no project
edit.

The app resolves `DazzUI` by path, so it always builds the working copy
beside it rather than a published version.

The macOS build requires Xcode to process the icon catalog. Fluent's Swift
package never
declares its asset catalog as a resource, so SwiftPM alone leaves
`Bundle.module` unsynthesised and fails to compile it. Xcode's build system
processes the catalog, which is why the project exists. The package README has
the detail.

The gallery starts with system sizing: AppKit's preferred typography and native
control measurements, independent from the selected palette. **Follow system
sizing** in the theme panel switches back to the shared design geometry. Pass
`-design-sizing` to start with design metrics. **Use native menus** in the same
panel switches ordinary menus to AppKit while keeping their triggers and
fields in the gallery's layout. Search suggestions remain custom.

## Linux

The Gallery builds the same `Sources/Example` and DazzUI components using
SwiftUIPort and the bundled Fluent icon resources. It runs in GTK 3 on Wayland
or X11. The Gallery uses DazzUI design sizing and custom menus on Linux.

Install Swift and the nativeapi build dependencies. On Ubuntu the packages are
`pkg-config`, `libgtk-3-dev`, `libx11-dev` and `libxi-dev`; on Arch use
`pkgconf`, `gtk3`, `libx11` and `libxi`. Clone nativeapi with its submodules.
Set the package paths and run these commands in this directory.

```bash
export SWIFTUI_PORT_PATH=/path/to/swiftui-port
export NATIVEAPI_SWIFT_PATH=/path/to/nativeapi/bindings/swift
swift build
swift run -c release Example -page calendar -theme studio-dark
swift test
```

`swift run Example --smoke` opens the window and exits after three seconds.
The SwiftPM tests render all generated scenes, all story pages and every theme,
and exercise search, navigation and theme switching through the shared runtime.

## WebAssembly

Build the SwiftUI gallery with the Swift.org Swift 6.4.2 toolchain, its matching
[WebAssembly SDK](https://www.swift.org/documentation/articles/wasm-getting-started.html),
and Node.js with npm. Use a SwiftUIPort checkout with Web support. The build
script finds the sibling `libnativeapi/swiftui-port` checkout or uses
`SWIFTUI_PORT_PATH` if set.

Run these commands from the Example directory.

```bash
swiftly run ./tool/build-web.sh +6.4.2
python3 -m http.server 8766 --bind 127.0.0.1 --directory .build/web
```

Use `./tool/build-web.sh` directly when the Swift.org toolchain is already
selected. Open [the gallery](http://127.0.0.1:8766/) or start on a specific page
with [`?page=button&theme=ember-dark`](http://127.0.0.1:8766/?page=button&theme=ember-dark).
The static output includes the compiled Swift app, browser runtime and Fluent
icons. It uses design sizing and custom menus.

## How it is organised

The React example's window, drawn with this package: the component list,
the document under its toolbar, the theme panel. Its stories are compiled
from the React example's by `pnpm codegen`; nothing here is written twice.
The layout is the React example's, file for file:

```
Sources/Example/
  Main.swift        the app; `-page button -theme ember-dark` opens a page
  App.swift         the page and the theme, and the window that shows them
  Stories/          generated — one file per React story, and Index.swift
  Views/            the window: shell, theme panel, theme preview, resizer,
                    brand, the stage stories arrange with (React's components/)
  Lib/
    Story.swift     the story type, decoded from what codegen wrote
    Scene.swift     draws a compiled scene: its slots, its events, its tree
    Mapping.swift   how each React component a story uses is drawn here
    Themes.swift    generated — the theme list, as React groups it
    Glyphs.swift    generated — the Fluent glyphs the stories name
    Location.swift  the page and theme the app was launched on
Tests/ExampleTests/
  StoriesTests.swift    every preview and example, in light and dark
```

A story is changed in the React example and regenerated. A component or
prop a story uses that SwiftUI does not map fails generation; map it in
`Lib/Mapping.swift` and list it in `tools/codegen/swiftui.mjs`.

The tests run from the same scheme:

```bash
xcodebuild -project Example.xcodeproj -scheme Example -destination 'platform=macOS' -derivedDataPath .build/xcode test
```

`SystemSizingTests` verifies native measurements, all-theme scaling at 100%,
150% and 225%, unchanged color/recipe/stroke/timing values, and notification-driven
updates preserving the native editor and its selection. It also renders all 51
story previews in light and dark at 225%. These scale fixtures do not modify
system preferences.

`MenuPresentationTests` verifies default and nested scopes, live propagation
through dialogs and floating panels, native item conversion, selection and
cancellation in real AppKit tracking loops, keyboard dispatch, menubar switching,
mode changes and teardown. It also checks that native mode preserves trigger
geometry and leaves Combobox and Autocomplete suggestions custom and editable.
Run just these checks with `-only-testing:ExampleTests/MenuPresentationTests`.

## Why it is a separate package

The library must not carry an entry point, a window, or a dependency that only
the example needs — the same reason the Flutter example is its own app
rather than a target of `dazzui`.

The build produces a plain executable rather than an app bundle, which
launches without a dock presence and behind whatever window has focus. The
`AppDelegate` asks for the regular activation policy, which is what brings it
forward.

## Conformance

`tool/conformance.sh --out <dir>` is the SwiftUI collector `pnpm conformance`
runs: `Tests/ExampleTests/ConformanceCollector.swift` draws every scene of
`tools/conformance/scenes.json` and writes what it can read of them — their
boxes and pictures. See [`specs/conformance.md`](../../../../specs/conformance.md).
Run without it, the test skips itself.
