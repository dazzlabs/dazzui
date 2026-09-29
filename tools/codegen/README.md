# Codegen

Compiles the React example's stories into the Flutter and SwiftUI examples.
The rules are in [`specs/playgrounds.md`](../../specs/playgrounds.md).

- `stories.mjs` reads `packages/js/dazzui-react/example/src/stories/*.tsx`
  and evaluates each ahead of time into data: a tree of components, the
  slots it keeps live, and the actions its events make. It also reads the
  React example's theme list.
- `flutter.mjs` is the Flutter side: `SUPPORT`, every component and prop
  the Flutter example draws, and the emitters for its generated sources —
  a file per story, the index, the themes, the glyphs.
- `swiftui.mjs` is the SwiftUI side, the same four emitters and its own
  `SUPPORT`.
- `support.mjs` finds what a story sets that a platform's `SUPPORT` does not
  name.
- `generate.mjs` writes the outputs, or with `--check` fails on drift, and
  keeps `outputs.json`, the files it owns and their hashes.
- `format.mjs` runs `dart format` before comparing.

```sh
pnpm codegen          # from the repository root
pnpm codegen:check
pnpm codegen:test
```
