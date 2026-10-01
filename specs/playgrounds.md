# Playgrounds

How a story is written once, in the React example, and compiled into the
other platforms' examples.

- **The React example is the source.** Every story is a file in
  `packages/js/dazzui/example/src/stories/`, named as the kit names the
  component (`icon-button.tsx`), whose default export is one `Story`: an
  `id` matching the file, a `name`, a `summary`, a `Preview` — the one scene
  that stands for the component on the overview — and `examples`, each a
  `title`, an optional `note` and a `Demo`. React renders the file as it is.
- **Codegen compiles it; it never runs it.** `tools/codegen/stories.mjs`
  reads each file with the TypeScript compiler and evaluates it ahead of
  time: module constants, `.map` over a constant list, a template string, a
  helper component called with props — all of it is worked out, so what
  comes out is the finished tree the React page renders. What stays live is
  kept as a reference: a `useState` is a slot, a setter handed to an event is
  an action that writes it, a comparison against a slot is an expression,
  `useToast().add` is an action that raises one.
- **A story is written in the portable subset**, and codegen refuses
  anything outside it with the file and line:
  - values come only from the kit, `@fluentui/react-icons`, the stage
    (`../components/stage`) and `react`'s `useState`;
  - elements are kit components, glyphs and the four stage components —
    `Row`, `Col`, `Frame`, `Text` — never HTML, never `style` or
    `className`;
  - an event handler only calls a setter, raises a toast, or does nothing.
- **Every platform draws what the story asks for, or generation fails.**
  `tools/codegen/flutter.mjs`, `swiftui.mjs`, `winui.mjs` and `gpui.mjs` each list
  the components
  and props their example maps; a story that sets one missing from a list
  fails `pnpm codegen`, naming it and the platform. The list and the
  mapping change together.
- **A component only React has keeps its story in `stories/react/`.**
  Codegen reads the top level of `stories/` alone, so a story there is
  listed by the React example and nowhere else, is not a conformance scene,
  and is not held to the portable subset. `WindowFrame` is the one today:
  no other platform draws a window inside its own. Moving a story up out of
  `react/` is the claim that every platform now maps it.
- **A platform's example mirrors the React one's layout.** On Flutter:
  `lib/stories/` holds a generated file per story and `index.dart`;
  `lib/lib/` holds the story type, the scene interpreter and its component
  mapping, the generated themes and glyphs; `lib/widgets/` is the window —
  React's `components/`. On SwiftUI the same under `Sources/Example/`:
  `Stories/`, `Lib/`, and `Views/` for the window. On WinUI the same under
  `example/`: `Stories/`, `Lib/`, and `Controls/` for the window. On GPUI the
  same under `example/src/`: `stories/`, `lib/` — whose `mapping/` is one
  file per family of components — and `window/`. Every platform names a
  glyph by the React name the story wrote: Flutter from
  `fluentui_system_icons`, SwiftUI and WinUI from `FluentIcons`, and GPUI
  from SVG codegen renders out of `@fluentui/react-icons` itself, so its
  outline is the web's to the path. The window is written by hand to the same columns,
  widths and tokens as React's; only the stories are generated. A story is
  JSON inside the generated file — on SwiftUI and WinUI a raw string parsed
  at launch, which the compiler never has to read; on GPUI a raw string too.
- **Never hand-edit a generated file.** `tools/codegen/outputs.json` records
  what codegen owns and the hash it wrote; `pnpm codegen:check` fails on any
  drift, and a file codegen stops writing is deleted only while it still
  matches that hash. Generation needs Dart to format its output.
- **Flutter, SwiftUI, WinUI and GPUI are compiled today.** The iOS and Android
  examples keep the generated sources the previous pipeline wrote, frozen:
  codegen no longer owns or rewrites them. Each is brought back by giving it
  a mapping and a support list of its own.
- Run `pnpm codegen` after changing a story, `pnpm codegen:check` to verify,
  `pnpm codegen:test` for the compiler's own tests. The Flutter example's
  `test/stories_test.dart` and the SwiftUI example's `StoriesTests.swift`
  draw every story's preview and every example in light and dark, and
  drive the interactions the interpreters carry; the WinUI example's
  `--smoke-test` draws every page in every theme in a real window.
