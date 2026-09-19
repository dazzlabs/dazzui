# Parity

A viewer for where the platform ports drift from the React reference — the
step after the numbers exist, when someone has to decide which of six
hundred lines to fix first.

```bash
pnpm parity:view
```

It reads three things off the disk, none of which it produces:

- **The React reference** — `packages/ui_react/example/parity/` (the extractor's
  staging, when `pnpm parity` has run here; screenshots come from there
  too) or, failing that, the copy distributed into the Flutter playground.
  The reference is folded into *specimens* — a class and its axes — the
  same way the platform tests fold it.
- **Each platform's `drift.txt`** — the checks its reference test names as
  failing today.
- **Each platform's `measured.json`** — what that test *compared* on its
  side. It carries only the fields that suite actually asked about: one it
  skips — a callout's inherited type, a spinner's two-coloured ring, the
  height of a box its content sets — is absent rather than merely ignored.
  It is written by the test, so a platform whose suite has not run shows its
  drift list but no values; run the suite and reload.

The overview counts the drift by platform, by field, and by component. A
component page lists its specimens, drifting ones first, each with what
React drew — cut out of the story's screenshot at the probe's rect — and a
table of every field with React's value beside each platform's, under the
theme the toolbar has selected. A red cell disagrees with React; an
outlined one also is not in `drift.txt` yet, which is the cell that would
fail the suite. A dash is a field that suite did not compare.

**And every side draws.** Under each specimen are the pictures: what React
drew and what each port drew of the same thing, at the same scale, in the
same box. That is the half of the comparison a value table cannot reach —
only one of the three frameworks will say what colour a view painted or
what face it set a label in — so the drawings are where the rest of it
shows. The number beneath one is how much of it differs from React's, and
it is never zero: two rasterisers disagree about the edge of a letter and
the inside of a corner, and a small drawing is mostly edges. Read it for
the *size* of a difference. A specimen whose React element holds other
elements rather than one run of text says `holds the story` instead, since
an adapter is given the axes and the text and nothing else.

The drawings come from `pnpm parity` for React, from the suite itself for
SwiftUI, and from `pnpm parity:flutter` for Flutter — a real window,
because a widget test draws every glyph as a filled rectangle.

**What is comparable is the suite's decision, not this page's.** The viewer
renders what it is given and asks only whether both sides have a value —
which is the whole of its judgement. Keeping a second copy of the policy
here is what let the page once show two hundred disagreements the suites
did not have.

The viewer is built out of `ui_react` and reads only its tokens, like the
examples: a tool that reports drift from the design system had better not
drift from it itself.
