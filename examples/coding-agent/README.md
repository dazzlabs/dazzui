# Coding agent

A prototype of a coding agent's desktop interface, built entirely out of
`@dazzlabs/dazzui-react`. It is a design prototype, not an agent: nothing
reaches the network, no file is written, and every message replays the same
scripted run.

It exists to answer a question a component gallery cannot: **is the vocabulary
enough to build an app out of?** Every value here — every colour, radius,
gap, type face and control height — is a token read from the design system.
There is not a literal in the stylesheets.

In a browser tab, on <http://localhost:5180>:

```bash
pnpm coding-agent
```

As a real desktop window:

```bash
pnpm coding-agent:desktop
```

## What it shows

A project is the thing the window is organised around, so the columns go
outside-in: the projects, then what is inside one, then the thing itself.

| Column                | What it is                                                                 |
| --------------------- | -------------------------------------------------------------------------- |
| `Sidebar` (full height) | The projects. It runs the whole height of the window, and its header strip is where the system's own controls land. |
| `Rail`                | What is inside the open project: its issues, its pull requests, its working tree, and its sessions. In settings it takes over and lists the settings pages instead. |
| Content               | A session's transcript and composer, one of the three project panes, or the settings. |
| `Aside`               | Whatever the content column is showing: the run's context, the selected issue or pull request, the commit, or the note that settings are not persisted. |

The three project panes are where the agent meets the repository:

- **Issues** and **Pull requests** — the provider's lists, as `Table`s with a
  detail column. Picking one hands its subject to a new session, pre-addressed,
  which is where a real session's subject comes from. A project with no remote
  (`forge-cli` in the fixtures) has both rows disabled and an empty state
  behind them.
- **Changes** — the working tree, staged and not, each file folding open to its
  diff. It is *live*: the files a run writes are read back out of the
  transcript's own tool rows and appear here as it goes, so the count in the
  rail and the card in the sidebar move while the agent works.

**Settings** belong to the window rather than to any one project, so they take
the rail over instead of sitting in it — one navigation column that changes
what it lists, rather than a second one that appears. Five pages: General,
Agent, Permissions, Integrations, Shortcuts.

They are not decoration. The theme applies as you pick it and the toolbar's
menu follows; the model and mode are the ones the composer sends under;
Integrations is what fills the Issues and Pull requests panes, so connecting
`forge-cli` there turns on two rail rows that were disabled; and turning off
*ask before writing* takes the permission sheet out of the run rather than
answering it for you. Two switches wearing one name would be two settings, so
the pair the aside offers mid-run edit the same values this page does.

The session view carries the rest: a plan that completes as the run walks it,
tool calls that fold open onto diffs, command output and match lists, a
permission sheet in front of the first write, and a composer that streams the
reply back.

## The desktop window

`deno desktop` packages the built app into a native window: the Deno runtime,
the platform's own WebView and the Vite output in one bundle, ~66 MB on macOS.
Deno is a devDependency of this package rather than something to install, so
`pnpm install` is all the setup there is.

```bash
pnpm coding-agent:desktop        # build, then open the window and watch
pnpm coding-agent:desktop:build  # package dist-desktop/Forge.app and stop
```

The app draws no window of its own — no corner, no shadow, no imitation
traffic lights. `desktop.ts` opens the real one with `transparentTitlebar`, so
the titlebar is gone, the toolbar runs to the top edge and the system's
controls float in the leading corner. While the sidebar is open they land in
its header strip, which holds nothing else; with it closed they land in the
toolbar, which starts after them. (For a window with no controls at all,
`frameless: true` is the one-word change — and then nothing can close or drag
it but the keyboard.)

`desktop.ts` is a static file server and nothing more. Two details in it are
not obvious:

- The `dist` path is resolved against `import.meta.dirname`, not the working
  directory: in a packaged app those files live in the binary's virtual file
  system, and the working directory is wherever the user launched from. The
  directory has to be passed with `--include dist` for the same reason — a
  path built by string concatenation is not something the bundler can see.
- It serves `index.html` with a `data-shell="desktop"` attribute added. That
  is the whole mechanism behind the paragraph above: one attribute lets the
  stylesheet tell a window from a browser tab, which beats shipping a second
  build of the app.

`--hmr` runs the window and watches this package, which is the loop for
`desktop.ts` itself. Iterating on the *interface* is faster in a browser
tab — the window serves the built `dist/`, so a change to `src/` needs a
build to show up.

## The parts worth stealing

**Interactive colour is never written down here.** A diff's added line is
`data-tint="success"` plus `--control-color-tinted-surface-normal`, and its
`+` is that recipe's content grade. Both are resolved by the token build
against whichever ramp the line is tinted with, which is why the diff is
legible on the paper canvas and on the near-black one without a per-theme
value anywhere in this package. Try the theme menu in the toolbar.

**The frame metrics do the aligning.** The sidebar's header strip and the
toolbar beside it are both `--frame-titlebar-size` tall, and neither is told
about the other. Same for the column widths: the rail is narrow because
`--frame-rail-width` says so, which is why a label like "Pull requests" is
shortened here rather than the column being widened.

**A consumer overrides by source order, not by force.** The composer is one
box made of a `Textarea` and a row of controls; the field gives up its own
border to the wrapper through a rule two selectors deep. That is the extension
point the library documents, used the way it is meant to be.

## Layout

```
deno.json              the desktop app's name, identifier and output path
desktop.ts             the window, and the static server behind it
src/
  app.tsx              the composition root: which project, which view, which session
  components/          the app's own parts, one .tsx/.css pair each
  data/                the fixtures — projects, issues, pulls, the scripted run, the glyphs
  lib/                 the model: the project shape, the transcript, the run player, the settings
  styles/app.css       the stylesheet registry, the way packages/ui_react has one
```

`src/lib/player.ts` is the interesting one. A run is a list of beats the player
walks with a plain `for await`, rather than a chain of timers — because a run
has to be able to stop in the middle, on a denied permission or on the Stop
button, and every await checks the generation it started in so a superseded
run unwinds instead of writing into the transcript that replaced it.

The reply is typed out from the clock rather than a character per tick: a timer
the browser has throttled then reveals the text in fewer, larger steps instead
of at a fraction of the speed.

## Fixtures

`src/data/projects.ts` holds three projects. The first is this repository, and
every path, registry and command in it is real — a prototype that invents a
project has to invent its file paths too, and invented paths are the first
thing that reads as fake. The scripted run adds a `Tooltip` component to
`packages/ui_react`, which is a change this workspace would actually accept.

Switching sessions or projects resets the transcript, and with it the files
that session had written. The working tree falls back to what the fixture says
was already uncommitted.

Settings live in memory for as long as the window is open — there is nothing
behind them to write to, and a prototype that quietly persisted state would be
harder to demonstrate, not easier. Reloading puts every one of them back.
