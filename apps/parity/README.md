# Parity

A viewer for where the platform ports drift from the React reference — the
step after the numbers exist, when someone has to decide which of six
hundred lines to fix first.

```bash
pnpm parity:view
```

It reads three things off the disk, none of which it produces:

- **The React reference** — `apps/storybook/parity/` (the extractor's
  staging, when `pnpm parity` has run here; screenshots come from there
  too) or, failing that, the copy distributed into the Flutter playground.
  The reference is folded into *specimens* — a class and its axes — the
  same way the platform tests fold it.
- **Each platform's `drift.txt`** — the checks its reference test names as
  failing today.
- **Each platform's `measured.json`** — what that test measured on its side.
  It is written by the test, so a platform whose suite has not run shows
  its drift list but no values; run the suite and reload.

The overview counts the drift by platform, by field, and by component. A
component page lists its specimens, drifting ones first, each with what
React drew — cut out of the story's screenshot at the probe's rect — and a
table of every field with React's value beside each platform's, under the
theme the toolbar has selected. A red cell disagrees with React by the
tests' own rule; an outlined one also is not in `drift.txt` yet, which is
the cell that would fail the suite.

The viewer is built out of `ui_react` and reads only its tokens, like the
examples: a tool that reports drift from the design system had better not
drift from it itself.
