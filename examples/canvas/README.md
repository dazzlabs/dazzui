# Canvas

A small design canvas, built entirely out of `@dazzlabs/dazzui-react`. It is a
prototype of the interface, not a design tool: nothing reaches the network,
nothing is written to disk, and reloading puts the fixture back.

It exists to ask the design system a question the coding-agent example cannot.
That one is a reading app — columns of rows, a transcript, a form. This one has
a *drawing* in the middle of it, and a drawing is the hardest thing to hand a
token graph, because it is the one region of the window whose colours are the
user's rather than the designer's.

The answer here is that they are not: **a shape carries a `tint` and a
`variant`, exactly as a `Badge` does**, and the token build resolves the pair
against whichever ramp the tint names. There is not a colour written down in
this package. Switch the theme in the toolbar and the drawing follows it into
Bright, into the dark, and back.

In a browser tab, on <http://localhost:5181>:

```bash
pnpm canvas
```

As a real desktop window:

```bash
pnpm canvas:desktop
```

## What it shows

The columns go outside-in, the way the coding agent's do: the document, then
what you can do to it, then the thing itself, then what is selected.

| Column                  | What it is                                                                                  |
| ----------------------- | ------------------------------------------------------------------------------------------- |
| `Sidebar` (full height) | The document: its pages, and the layers on the open one. Its header strip is where the system's own window controls land. |
| `Rail`                  | The tools. Two that only look — Select and Hand — and five that draw.                        |
| Canvas                  | The drawing, and under it the strip that says how it is being looked at.                     |
| `Aside`                 | The selection: its box, how it is arranged, how it is painted, and what it says. With nothing selected it describes the page and lists the keys. |

The canvas itself does what a canvas has to do before any of the above is worth
looking at: draw by dragging a tool out, pick by clicking, add to a selection
with shift, sweep one out with a marquee, move it, resize it from any of eight
grips, snap to the grid, pan with space or the hand, zoom on the pointer,
double-click a shape to type into it, and undo the lot.

Five kinds of shape, and the differences between them are decisions rather than
settings:

- **Frame** is the paper the drawing sits on, so it takes the surface roles
  instead of a tint, and its edge and its label are divided by the zoom — they
  are chrome, and chrome does not scale with the drawing.
- **Text** has a colour rather than a fill, so it reads its tint's *content*
  grade whatever variant it carries, and the inspector does not offer it one.
- **Rectangle**, **Ellipse** and **Note** carry the full `tint` × `variant`
  pair. `tinted` deliberately takes the chip's own 12% rather than the surface
  wash a `Card` swaps to at that size — the card's own reasoning is that the
  chip fill *fills* a card instead of tinting it, which is exactly what a shape
  on a canvas wants.

## The parts worth stealing

**The colour model is the design system's, without translation.** The six
swatches in the inspector are the six ramps; the three fills are the three
recipes. A canvas that had invented a palette would have had to invent a dark
version of it too, and then a Bright one.

**One gesture is one undo.** The store takes page-to-page recipes rather than
verbs, and a drag opens a step, drafts a few dozen frames into it, and records
it when the hand comes off (`begin` / `draft` / `settle` in
`src/lib/scene.ts`). Typing into a shape uses the same three calls, which is
why a name typed a letter at a time is one thing that happened rather than
eleven.

**Screen units and scene units are kept apart.** `src/lib/geometry.ts` is pure
rectangle arithmetic in the drawing's own units and knows nothing about the
viewport; `src/lib/viewport.ts` owns the conversion, and the selection overlay
is drawn in screen units so a grip is the same size to the hand at any zoom.
The two places that cross the line do it in CSS, by dividing a token by
`--cv-zoom`.

**A consumer overrides by source order, not by force.** Where this app reshapes
a library component it does it with a class of its own at one selector deeper,
and it takes the `cv-` prefix so nothing collides with `ff-`.

## Layout

```
deno.json              the desktop app's name, identifier and output path
desktop.ts             the window, and the static server behind it
src/
  app.tsx              the composition root: the document, the viewport, the keys
  components/          the app's own parts, one .tsx/.css pair each
  data/                the fixture drawing, the tools, the key list
  lib/                 geometry, the viewport, the document and its history
  styles/app.css       the stylesheet registry, the way packages/ui_react has one
```

`src/lib/scene.ts` is the interesting one. The document is a reducer over
`{ past, present, future, held }`, and `held` is what makes a drag one step
instead of forty. Selection and the open page sit *outside* the history on
purpose: neither is a fact about the drawing, and an undo that moved you
somewhere else to show you what it did would be undoing something you never
asked for.

## Fixtures

`src/data/document.ts` holds three pages. The first draws the token pipeline
this repository actually has, step for step; the second is a cover for the
design system the canvas is built out of; the third is empty, because the empty
state is worth being able to see without deleting the other two.

The two frames are locked, so a drag that starts on the paper sweeps a marquee
rather than moving the world. Unlock either one in the layer list.

Nothing persists. Reloading brings the fixture back, and the pages you added go
with it.
