# SwiftUI example

The DazzUI example for SwiftUI, the counterpart of
`packages/dart/dazzui/example`.

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

**`swift build` and `swift run` do not work here**, and the reason is the icon
dependency rather than anything about this app: Fluent's Swift package never
declares its asset catalog as a resource, so SwiftPM alone leaves
`Bundle.module` unsynthesised and fails to compile it. Xcode's build system
processes the catalog, which is why the project exists. The package README has
the detail.

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
  ReferenceTests.swift  measures each component against what React drew
Parity/             what the parity extractor distributed, and what the suite
                    drew (neither is committed)
```

A story is changed in the React example and regenerated. A component or
prop a story uses that SwiftUI does not map fails generation; map it in
`Lib/Mapping.swift` and list it in `tools/codegen/swiftui.mjs`.

`Parity/` sits beside the sources rather than inside them because Xcode's
synchronised folder takes everything under `Tests/` as a resource to copy
into the bundle, and every theme's drawings share their file names.

The tests run from the same scheme:

```bash
xcodebuild -project Example.xcodeproj -scheme Example -destination 'platform=macOS' -derivedDataPath .build/xcode test
```

## Why it is a separate package

The library must not carry an entry point, a window, or a dependency that only
the example needs — the same reason the Flutter example is its own app
rather than a target of `dazzui`.

The build produces a plain executable rather than an app bundle, which
launches without a dock presence and behind whatever window has focus. The
`AppDelegate` asks for the regular activation policy, which is what brings it
forward.
