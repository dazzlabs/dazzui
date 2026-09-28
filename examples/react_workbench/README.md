# Workbench

A prototype of a Git-rooted agent workbench, built entirely out of
`@dazzlabs/dazzui-react`. It is a design prototype, not an agent: nothing
reaches the network, no file is written, and every message replays the same
scripted run.

It exists to answer a question a component gallery cannot: **is the vocabulary
enough to build an app out of?** Every value here — every colour, radius, gap,
type face and control height — is a token read from the design system. There
is not a literal in the stylesheets.

In a browser tab, on <http://localhost:5180>:

```bash
pnpm workbench
```

As a real desktop window:

```bash
pnpm workbench:desktop
```

## The shape of it

Several projects at once, each with its own sessions and worktrees, and one
column that changes what it lists.

| Region        | What it is                                                                                                                                                          |
| ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Titlebar      | The projects, as tabs. Each carries one dot: the most demanding thing happening inside it, so a project that needs a person says so from the other side of the window. |
| Sidebar       | The project it is on, then the eight pages, then — under a rule — whatever the open page is *made of*: sessions, changed files, branches, suites, filters, configs. Settings sits at its foot and opens a sheet. |
| Pane          | The page. Chat keeps the changes panel beside it; everything else fills the pane.                                                                                     |

The sidebar's lower half is the whole idea. Seven pages with one static column
would need a second column for every list; here the column is the page's own
index, so the reader never loses the place they navigate from. The column
never names the project — the titlebar's tab does, and the status bar
carries the branch — and it hides on ⌘B, the way an editor's does, for a
review that is worth the window.

**Seven pages**, bound to ⌘1–7 in the order the sidebar lists them:

- **Chat** — the thread and the composer, with the dock beside it: the
  workspace changes, the terminal, and the pull request the session is
  pushing to, as three tabs. The composer carries the mode a message goes out
  in — Ask, Plan, Agent or Auto — because that is decided per message.
- **Inbox** — everything across the open projects that is waiting on a
  person: a session blocked on an answer, a session holding a diff nobody has
  read, and what the window noticed on its own, such as two sessions writing
  the same file. Every parallel-agent workbench has grown one of these under
  one name or another, and for the same reason: which session needs *you* is
  the one thing the sessions cannot say for themselves.
- **Review changes** — every file the agent wrote, accepted or reverted one at
  a time. A file that has a verdict folds shut and fades, so what is left to
  read is always what is still open. A comment on a file goes back to the
  agent as its next instruction; J and K move between files, A, X and C act
  on the one under the cursor; and the foot stages what was accepted and
  opens the commit.
- **Git** — three columns, after the Tiller canvas's Git face: the branches
  and the remote in the sidebar, the working tree in a column of its own with
  the commit box at its foot — what it closes, Commit, Commit + Push — and
  beside it the history and every worktree the project has on disk with the
  session holding each.
- **Tests** — the four numbers, then every failure with the assertion as the
  runner printed it and the agent's reading of why.
- **GitHub** — three columns, the way the Tiller canvas draws its GitHub
  face: the sidebar names the face, a list in the middle scans it, and the
  pane on the right reads one row. *Issues* is where a session comes from:
  the pane shows the issue, then a card that opens a session from it with
  every default already filled — the worktree, the branch, what goes in as
  the first context — so Return is enough and S skips the sheet altogether;
  under that is the monitor, every open pull request with its red checks
  and the one control they need. *Pull requests* reads one at a time, with
  its checks, its timeline and a merge button that says why it is disabled.
  *Checks* is the monitor turned round: one run at a time, with what it
  printed.
- **Release** — one pipeline from a build to a shop window: the jobs, the
  channels they go out through, and the artifacts they produced.

**Settings is a sheet, not a page** (⌘, or the row at the foot of the
sidebar). They belong to the window rather than to whichever project is open,
and they are somewhere you go for a moment and come back from — so they take
the window over briefly instead of becoming an eighth page to navigate away
from. The five sections stay a column inside the sheet, and there is no
Cancel: every control writes as it is touched.

⌘K opens the command palette, ⌘P the welcome screen, ⌘N a session.

## What is worth looking at

**The permission card is in the thread.** A risky operation stops with a card
where it came up rather than a sheet over the window — the reader usually
wants to scroll up and check the plan before answering, and a modal is exactly
the thing that prevents that. The answer stays in the thread once given.

**The terminal is dark in every theme.** It is the one surface that does not
follow the canvas, and it is deliberate: a terminal is dark, and a light one
reads as a picture of a terminal rather than as output. It takes the neutral
ramp's deep end, which no theme re-points.

**What the person said is a bubble; what the agent said is the page.** Only
one of the two can be ordinary prose, and it should be the longer one — so the
agent's turns carry no mark, no bubble and no avatar.

**Changes are live.** The files a run writes are read back out of the
transcript's own tool rows, so the dock, the sidebar's count and the
review page all move while the agent works.

**The mode shapes the run.** Ask stops before the first thing that would
write and says so; Plan stops once the plan is on the page and waits for a
go; Agent is the run as scripted, stopping for the permission card; Auto is
the same run with nothing in it that asks. The permission policy on the
status bar is a second, coarser switch over all four.

**Nothing in the sidebar is ever disabled.** A project with no remote, or
one whose account was disconnected, still reaches the GitHub page — which
says why it is empty and offers the way out. A
greyed row cannot.

**The projects differ on purpose.** They belong to a sample organisation,
`acme`, and a sample person, Mia Chen. `acme/atlas` is a design-system
monorepo that publishes to package registries and has two failing parity
tests; `acme/lingo` is a Flutter app that ships to the stores through
dazzui, with DMG, MSIX, AAB and IPA jobs feeding App Store, Play Store and
GitHub Releases; `notes-cli` has no remote and nothing to release, which is
what puts three pages in their empty state.

## The desktop window

`deno desktop` packages the built app into a native window: the Deno runtime,
the platform's own WebView and the Vite output in one bundle. Deno is a
devDependency of this package rather than something to install, so
`pnpm install` is all the setup there is.

```bash
pnpm workbench:desktop        # build, then open the window and watch
pnpm workbench:desktop:build  # package dist-desktop/Workbench.app and stop
```

The app draws no window of its own — no corner, no shadow, no imitation
traffic lights. `desktop.ts` opens the real one with `transparentTitlebar`, so
the titlebar runs to the top edge and the system's controls float in its
leading corner; the server marks the document with `data-shell="desktop"` and
the stylesheet reserves the room for them only there.
