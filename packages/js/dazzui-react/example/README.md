# React example

Every React component, live, in every theme the token graph carries — the
drawn families and the imported ones — in the window of a desktop
application. A plain Vite application over `@dazzlabs/dazzui-react`.

```sh
pnpm example          # from the repository root: build the packages, serve :6006
pnpm example:build    # production build
```

## Layout

```
src/
  stories/      what is shown: one story per component, in a file named as the kit names it
  components/   the example's own: the window, its theme panel, the stage a story is set on
  lib/          what both read: the story shape, the themes, the address
  styles/       the example's own ground
```

Adding a component is a file in `src/stories/` — `button.tsx` beside the
kit's `button.tsx` — whose default export is one `Story`: a name, a summary,
a `Preview` and its examples. Nothing else changes; the file is picked up
by being there.

## Stories are compiled

Codegen compiles every story into the other platforms' examples, so a story
is written in the portable subset: values from the kit, the icons, the stage
and `useState`; elements that are kit components, glyphs or the stage's
`Row`, `Col`, `Frame` and `Text` — no HTML, no `style`; event handlers that
only set state or raise a toast. Anything else fails `pnpm codegen` with the
file and line. See [`specs/playgrounds.md`](../../../../specs/playgrounds.md).

The address is the state: `?theme=ember-dark`, and `#/button` for the page.

## The window

Three full-height columns: the source list on the left, the document under
its toolbar, and the theme panel on the right. The
shell is drawn from the kit alone and `shell.css` only places things, so a
native port builds the same window out of its own components.
