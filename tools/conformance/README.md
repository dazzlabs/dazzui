# Conformance

How closely each implementation draws what React draws. The rules are in
[`specs/conformance.md`](../../specs/conformance.md); this package is the
contract, the run, the viewer, the shared verdict module, and the fixtures
the viewer is developed against.

```sh
pnpm conformance         # from the repository root: a run, into data/
pnpm conformance:check   # hold the run to expectations/
pnpm conformance:view    # the viewer on :5183
pnpm --filter @dazzlabs/dazzui-conformance fixtures   # regenerate fixtures/
```

## Layout

```
src/
  components/   the window: the list, the document, the lens
  lib/          the run's shape, how it is loaded, how a verdict is derived
  styles/       the viewer's own ground
scenes.json     the contract — written by `pnpm codegen`, never by hand
scripts/        the run (collect.mjs), the check (check.ts), the fixture generator
tests/          the check's own tests — `pnpm --filter @dazzlabs/dazzui-conformance test`
fixtures/       a made-up run, served when data/ has none
expectations/   what each platform fails today — committed, only shrinks
data/           a real run — gitignored
```

The window is the React example's, put to a different use: the same three
columns, drawn from the kit alone, with `shell.css` only placing things.

## Incremental collection

```sh
pnpm conformance --incremental --platforms react,winui --themes studio-light --scenes table/preview
```

`--incremental` updates only the requested platforms, themes and scenes.
`--platforms` selects collectors; `--scenes` accepts story IDs (`table`) or
exact scene IDs (`table/preview`, `button/0`). Both accept comma-separated
lists. Omit `--incremental` for a replacement run. GTK4 is available explicitly
with `--platforms react,gtk4`; its mapping gaps remain outside the default run.

A collector writes into isolation. If any selected collector fails or cannot
produce readings, the previous run stays published. A successful update
replaces the whole selected scene, including specimens that stopped drawing,
while retaining other themes, scenes and platforms. Screenshot URLs follow
their contents, so unchanged images are reused and new images cannot be
confused with browser-cached old captures. `--no-images` clears outdated
screenshots in the updated scope. Each scene records its own collection time
and commit; retained scenes keep their original provenance.

The open viewer notices a published update, reloads changed platforms and
recomputes the differences without losing the selected page or lens. Hover
a scene caption to see when that scene was collected. Verdicts are still
derived from readings, never persisted. Expectations and tolerances are not
changed by collection.

Incremental merging requires the same scale and shared font. If the specimen
contract changes, include every affected scene and retained platform, or make
a full run. Concurrent collections to the same output are rejected to avoid
lost updates. `--out <directory>` chooses a separate run; set
`DAZZUI_CONFORMANCE_DATA=<directory>` when starting the viewer to inspect it.

`pnpm --filter @dazzlabs/dazzui-conformance test:collect` exercises real React
screenshots, scoped updates and capture-failure recovery. It uses the example's
Playwright browser; `DAZZUI_CHROMIUM_EXECUTABLE` can select an installed Chrome.
