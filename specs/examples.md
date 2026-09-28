# Examples

What an app in `examples/` may and may not do.

`examples/*` are apps, not packages: private, never published, and each one
built and served on its own port. An example is added to `pnpm-workspace.yaml`
by the `examples/*` glob and given a root script beside the playground's.

The rules an example does keep are the ones that make it worth having:

- **Not a literal in the stylesheets.** Every colour, radius, gap, face and
  height is a token read from `@dazzlabs/dazzui-react/style.css`. An example
  that reached for a hex would be proving the opposite of its point. A layout
  value with no token behind it — a breakpoint, a preview's maximum height — is
  derived from one (`calc(var(--spacing-20) * 3)`) or stated once as a named
  constant with the reason beside it.
- **Interactive colour comes off the recipes.** A part that is tinted carries
  `data-tint` and reads `--control-color-<variant>-<part>-<state>`, exactly as a
  component in `dazzui-react` does. Writing a ramp step into an example's CSS is
  the same mistake there as it is in the library, and the theme menu is what
  makes it visible.
- **The same shape as the library.** A component that owns styles is a
  `.tsx`/`.css` pair, and the sheet is only shipped once it is named in the
  registry (`src/styles/app.css`); parts that are one shape wearing different
  data share a sheet rather than duplicating it (`views.css` dresses every
  page's head, its cards and its rows). Variants are `data-*` attributes, and
  the app takes a class prefix of its own — `wb-` for the workbench — so
  nothing collides with `ff-`.
- **Override by source order, not by force.** Where an example has to reshape a
  library component, it does it with a class of its own at one selector deeper
  (`.wb-composer .wb-composer__input`), which is the extension point `cx()`
  documents. It never edits `dazzui-react`, and it never reaches for `!important`.
  A second navigation column is the same move: the workbench's sidebar is
  `Sidebar collapsible="none"` with `--sidebar-width` re-pointed on the class,
  which is what that component is drawn for. Inside a `Dialog` it is not —
  `Sidebar` reads the window's own context and a sheet renders in a portal
  outside it, so the settings sheet's section list is `Button`s in a column.
- **Fixtures are a sample world, drawn to real shapes.** The workbench's
  projects belong to a sample organisation (`acme`) and a sample person, so
  nothing in it reads as anyone's real repository — but the paths, registries,
  commands and failures are the kind such a project actually has, since an
  invented *shape* is the first thing that reads as fake.
- **An example draws no window.** The corner, the shadow and the controls
  belong to the real one — `deno desktop` opens it with the titlebar hidden —
  and a browser tab has no chrome to imitate. Where the two genuinely differ,
  the server marks the document (`data-shell="desktop"`) and the stylesheet
  reads the attribute; a second build of the app would be the wrong answer.
