# Conformance

How closely each implementation draws what React draws. The rules are in
[`specs/conformance.md`](../../specs/conformance.md); this package is the
viewer, the shared verdict module, and the fixtures the viewer is developed
against.

```sh
pnpm conformance:view    # from the repository root: the viewer on :5183
pnpm --filter @dazzlabs/dazzui-conformance fixtures   # regenerate fixtures/
```

## Layout

```
src/
  components/   the window: the list, the document, the lens
  lib/          the run's shape, how it is loaded, how a verdict is derived
  styles/       the viewer's own ground
scripts/        the fixture generator
fixtures/       a made-up run, served when data/ has none
expectations/   what each platform fails today — committed, only shrinks
data/           a real run — gitignored
```

The window is the React example's, put to a different use: the same three
columns, drawn from the kit alone, with `shell.css` only placing things.
