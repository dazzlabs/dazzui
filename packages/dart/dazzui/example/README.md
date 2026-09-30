# Flutter example

Every Flutter widget, live, in every theme the token graph carries, in the
same window the React example draws — the component list, the document under
its toolbar, the theme panel. Its stories are compiled from the React
example's by `pnpm codegen`; nothing here is written twice.

```bash
flutter run            # macOS
flutter run -d chrome  # the web build; `?theme=ember-dark#/button` opens a page
```

```bash
flutter analyze
flutter test
```

## Layout

The React example's, file for file:

```
lib/
  app.dart       the page and the theme, and the window that shows them
  main.dart
  stories/       generated — one file per React story, and index.dart
  widgets/       the window: shell, theme panel, theme preview, resizer,
                 brand, the stage stories arrange with (React's components/)
  lib/
    story.dart   the story type
    scene.dart   draws a compiled scene: its slots, its events, its tree
    mapping.dart how each React component a story uses is drawn here
    themes.dart  generated — the theme list, as React groups it
    glyphs.dart  generated — the Fluent glyphs the stories name
    location.dart the page and theme an address names
```

A story is changed in the React example and regenerated. A component or
prop a story uses that Flutter does not map fails generation; map it in
`lib/lib/mapping.dart` and list it in `tools/codegen/flutter.mjs`.

`test/stories_test.dart` draws every story's preview and every example in
light and dark, and drives the interactions the interpreter carries — a
dialog's trigger, a toast, a slot two components share.
