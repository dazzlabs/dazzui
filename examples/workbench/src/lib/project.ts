import type { ChangedFile, DiffLine, Entry, PlanStep } from './transcript'

/** Where a project's issues and pull requests come from, if anywhere. */
export type Provider = 'github' | null

/**
 * What a session is doing, which is the axis the whole window is sorted by:
 * the sidebar groups sessions by it, the inbox lists what needs a person
 * across every project, and the dot beside a project tab is the strongest
 * state among them.
 *
 * Two of these need a person and the rest do not. `waiting` is blocked on an
 * answer — a permission, a question, a conflict — and `review` is finished
 * and holding a diff nobody has read. `watching` is the quiet one: it is
 * keeping an eye on a pull request's checks and holds no worktree. `queued`
 * is waiting for a worktree rather than for the agent.
 */
export type SessionStatus =
  | 'running'
  | 'waiting'
  | 'review'
  | 'watching'
  | 'queued'
  | 'paused'
  | 'done'

/**
 * The states in the order the sidebar groups them: what is moving, then
 * what needs you, then what is only waiting.
 */
export const SESSION_GROUPS: { status: SessionStatus; label: string }[] = [
  { status: 'running', label: 'Active' },
  { status: 'waiting', label: 'Needs you' },
  { status: 'review', label: 'To review' },
  { status: 'watching', label: 'Watching' },
  { status: 'queued', label: 'Queued' },
  { status: 'paused', label: 'Paused' },
  { status: 'done', label: 'Done' },
]

/** True where a session is asking for a person rather than for time. */
export function needsYou(status: SessionStatus): boolean {
  return status === 'waiting' || status === 'review'
}

export interface Session {
  id: string
  title: string
  /** Relative time, the way a column prints it. */
  when: string
  status: SessionStatus
  /** The plan, which the thread shows as a checklist. */
  steps: PlanStep[]
  /** How many tool calls it has made, which is what "busy" looks like. */
  tools: number
  /** The worktree it runs in; absent means the main checkout. */
  worktree?: string
  branch?: string
  /** The issue it was opened from, and the pull request it opened. */
  issue?: number
  pull?: number
  /** The command it is blocked on, on a `waiting` session. */
  blockedOn?: string
  /** What the transcript already holds when the session is opened. */
  seed: Entry[]
}

export type IssueState = 'open' | 'closed'

export interface Issue {
  number: number
  title: string
  state: IssueState
  /** One label; the design prints a single tag per row. */
  label: string
  author: string
  when: string
  comments: number
  /** The first paragraph, which is all a detail column has room for. */
  body: string
  /** True where the issue is small and self-contained enough to hand over. */
  agentReady: boolean
  /** The pull request that closes it, where one exists. */
  pull?: number
}

export type PullState = 'open' | 'draft' | 'merged'

export type RunStatus = 'passed' | 'failed' | 'running' | 'queued' | 'skipped'

/** One check on a pull request, as the provider reports it. */
export interface CheckRun {
  name: string
  status: RunStatus
  /** How long it took, or has been running. */
  took?: string
  /** What went wrong, on a failed run. */
  note?: string
}

/** One entry on a pull request's timeline. */
export interface Event {
  author: string
  /** The initials on the avatar; a prototype has no images to load. */
  initials: string
  /** `opened` carries the subtitle; `comment` is one line. */
  kind: 'opened' | 'comment'
  body: string
  when: string
  /** The session that produced it, on an event the agent caused. */
  session?: string
}

export interface Pull {
  number: number
  title: string
  state: PullState
  /** The head branch, as the provider names it. */
  branch: string
  base: string
  runs: CheckRun[]
  commits: number
  files: number
  added: number
  removed: number
  author: string
  when: string
  body: string
  timeline: Event[]
  /** The session that opened it, and the issue it closes. */
  session?: string
  fixes?: number
}

export type ChangeStatus = 'modified' | 'added' | 'deleted'

/** One file in the working tree that no longer matches the last commit. */
export interface WorkingChange {
  path: string
  status: ChangeStatus
  added: number
  removed: number
  /** The hunk shown when the row is opened. Abridged, as a review pane is. */
  hunk: DiffLine[]
  /** Staged files go in the next commit; the rest are only offered. */
  staged: boolean
}

export interface Commit {
  sha: string
  message: string
  author: string
  when: string
  /** True where the agent wrote it, which the history marks. */
  agent: boolean
}

export interface Branch {
  name: string
  /** `↑2`, `↓5`, `synced` — how it stands against its upstream. */
  meta: string
}

/** A checkout beside the main one, where a session does its work. */
export interface Worktree {
  path: string
  branch: string
  /** The session bound to it, where one is. */
  session?: string
  /** True once the pull request merged: the session is gone and the
   * directory is only waiting to be removed. */
  archived?: boolean
}

/**
 * Something the window noticed on the reader's behalf and has not been told
 * about yet. Parallel sessions raise the kind of thing nobody is looking at
 * — two of them writing the same file, a merged pull request leaving a
 * worktree behind — and the inbox is where that lands.
 */
export type NoticeKind = 'conflict' | 'archived' | 'checks'

export interface Notice {
  id: string
  kind: NoticeKind
  title: string
  body: string
  /** The file at issue, on a conflict. */
  path?: string
  /** The pull request at issue, on a merge or a failed check. */
  pull?: number
}

/* Tests ------------------------------------------------------------------ */

export interface TestSuite {
  path: string
  passed: number
  failed: number
}

/** A failing case, with enough of the output to act on it. */
export interface TestFailure {
  suite: string
  name: string
  /** The assertion, as the runner printed it. */
  output: string[]
  /** Where it broke, and the agent's reading of why. */
  analysis: string
  /** The pull request whose review said the same thing, where there is one. */
  pull?: number
}

export interface TestRun {
  label: string
  status: 'passed' | 'failed'
  when: string
  took: string
}

export interface Tests {
  passed: number
  failed: number
  skipped: number
  /** Line coverage, as a percentage. */
  coverage: number
  suites: TestSuite[]
  failures: TestFailure[]
  runs: TestRun[]
}

/* Release ---------------------------------------------------------------- */

/** A job either builds an artifact or ships one; nothing else is a job. */
export type JobKind = 'package' | 'publish'

export interface ReleaseJob {
  kind: JobKind
  name: string
  /** The target as the config names it: `dmg`, `aab`, `app-store`. */
  target: string
  status: RunStatus
  took?: string
}

/** A named block in the distribute config, and what it comes to. */
export interface ReleaseConfig {
  name: string
  jobs: number
  note: string
}

/**
 * A channel a build goes out through. The actions a channel offers are the
 * component's to decide from the id rather than the fixture's to list — a
 * store's verbs belong to what it is, not to this data.
 */
export type ChannelId =
  | 'app-store'
  | 'play-store'
  | 'github'
  | 'firebase'
  | 'pgyer'
  | 'npm'
  | 'pub'
  | 'crates'

export interface Channel {
  id: ChannelId
  name: string
  /** The badge: `In review`, `20% rollout`, `Not configured`. */
  state: string
  /** Whether it is live, moving, or off — which ramp the badge takes. */
  tone: 'live' | 'moving' | 'idle'
  /** What is out there now, and the line under it. */
  live: string
  note: string
}

export interface Artifact {
  format: string
  name: string
  size: string
}

export interface Release {
  version: string
  /** The config block the pipeline is running from. */
  active: string
  configs: ReleaseConfig[]
  /** The file the configs are read out of, which the sidebar names. */
  configFile: string
  /** The command the pipeline is, printed where a reader can copy it. */
  command: string
  jobs: ReleaseJob[]
  /** The tail of the running job's output. */
  log: string[]
  channels: Channel[]
  artifacts: Artifact[]
}

/* The project ------------------------------------------------------------ */

export interface Project {
  id: string
  /** `owner/repo` where there is a remote, and the folder name where not. */
  name: string
  /** Where it lives on disk, as the sidebar's head prints it. */
  path: string
  branch: string
  /** How far the branch has drifted from its upstream. */
  ahead: number
  behind: number
  provider: Provider
  account?: string
  branches: Branch[]
  sessions: Session[]
  issues: Issue[]
  pulls: Pull[]
  changes: WorkingChange[]
  history: Commit[]
  worktrees: Worktree[]
  /** What the window has noticed and not yet been told about. */
  inbox: Notice[]
  tests: Tests | null
  release: Release | null
  /** When the window last had it open, as the welcome screen prints it. */
  seen: string
}

/** One session, and the project it belongs to: a row on any cross-project list. */
export interface Task {
  session: Session
  project: Project
}

/** A notice, and the project it was noticed in. */
export interface Attention {
  notice: Notice
  project: Project
}

/**
 * The pages, in the order the sidebar lists them — which is also the order
 * ⌘1–7 binds them in, so the list is the shortcut table.
 *
 * Inbox is second because it is what the reader opens the window for: every
 * agent workbench that runs sessions in parallel has grown one — a review
 * queue, a triage list, an inbox — since a session that needs a person is
 * the one thing the sessions cannot notice for themselves.
 *
 * GitHub is one page with two faces — the issues and the pull requests —
 * because they are one thing seen from two ends: an issue is where a
 * session comes from and a pull request is where it goes, and the sidebar
 * under the page is where the reader moves between them.
 *
 * `welcome` is outside that order (⌘0): it is what the window shows with no
 * project open, and the only page drawn without the sidebar beside it.
 * Settings is not here at all — it is a sheet over the window rather than a
 * page inside it, because it belongs to the window rather than to whichever
 * project is open.
 */
export type View =
  | 'chat'
  | 'inbox'
  | 'review'
  | 'git'
  | 'tests'
  | 'github'
  | 'release'
  | 'welcome'

/** The seven pages the number keys reach, in that order. */
export const VIEWS: readonly View[] = [
  'chat',
  'inbox',
  'review',
  'git',
  'tests',
  'github',
  'release',
]

/** The three faces of the GitHub page: the two things, and the light over them. */
export type GithubFace = 'issues' | 'pulls' | 'checks'

/** The slices the inbox is read in. */
export const INBOX_FILTERS = [
  { id: 'all', label: 'Everything' },
  { id: 'waiting', label: 'Needs an answer' },
  { id: 'review', label: 'Ready to review' },
  { id: 'notices', label: 'Notices' },
] as const

export type InboxFilter = (typeof INBOX_FILTERS)[number]['id']

export function openIssues(project: Project): Issue[] {
  return project.issues.filter(issue => issue.state === 'open')
}

export function openPulls(project: Project): Pull[] {
  return project.pulls.filter(pull => pull.state !== 'merged')
}

/** The one overall light a pull request's checks add up to. */
export function checksStatus(pull: Pull): RunStatus {
  if (pull.runs.some(run => run.status === 'failed')) return 'failed'
  if (pull.runs.some(run => run.status === 'running')) return 'running'
  if (pull.runs.some(run => run.status === 'queued')) return 'queued'
  return 'passed'
}

/**
 * The state a project's tab shows: the most demanding thing happening inside
 * it. A tab is one dot, and the dot should be the reason to look.
 */
export function projectStatus(project: Project): SessionStatus {
  const order: SessionStatus[] = [
    'waiting',
    'review',
    'running',
    'queued',
    'watching',
    'paused',
  ]
  for (const status of order) {
    if (project.sessions.some(session => session.status === status)) {
      return status
    }
  }
  return 'done'
}

/**
 * What a worktree is doing, read off the session that holds it. A worktree
 * with no session is either clean or, once its pull request merged, waiting
 * to be removed.
 */
export function worktreeStatus(
  project: Project,
  tree: Worktree
): { label: string; status: SessionStatus | null } {
  if (tree.archived) return { label: 'To clean up', status: null }
  const session = project.sessions.find(entry => entry.id === tree.session)
  if (!session) return { label: 'Clean', status: null }
  return { label: session.title, status: session.status }
}

/** `owner/` is the part that repeats; the repo is the part that names it. */
export function shortName(name: string): string {
  return name.split('/').pop() ?? name
}

/**
 * The working tree as it stands: what was already uncommitted when the window
 * opened, plus whatever the open session has written since.
 *
 * The second half is read back out of the transcript rather than kept beside
 * it. The tool rows already carry every diff, so a second list would only be a
 * chance for the two to disagree.
 */
export function workingTree(
  project: Project,
  written: readonly ChangedFile[]
): WorkingChange[] {
  const listed = new Set(project.changes.map(change => change.path))

  const fresh: WorkingChange[] = written
    .filter(file => !listed.has(file.path))
    .map(file => ({
      path: file.path,
      status: file.created ? 'added' : 'modified',
      added: file.added,
      removed: file.removed,
      hunk: file.lines,
      // A file the agent has only just written is not staged: staging it is
      // the reader's way of saying they have looked at it.
      staged: false,
    }))

  return [...fresh, ...project.changes]
}
