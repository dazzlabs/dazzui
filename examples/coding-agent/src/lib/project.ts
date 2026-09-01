import type { ChangedFile, DiffLine, Entry } from './transcript'

/** Where a project's issues and pull requests come from, if anywhere. */
export type Provider = 'github' | null

export interface Session {
  id: string
  title: string
  /** Relative time, the way a column prints it. */
  when: string
  /** What the transcript already holds when the session is opened. */
  seed: Entry[]
}

export type IssueState = 'open' | 'closed'

export interface Issue {
  number: number
  title: string
  state: IssueState
  labels: string[]
  author: string
  when: string
  comments: number
  /** The first paragraph, which is all a detail column has room for. */
  body: string
}

export type PullState = 'open' | 'draft' | 'merged'
export type Checks = 'passing' | 'failing' | 'running'

export interface Pull {
  number: number
  title: string
  state: PullState
  /** The head branch, as the provider names it. */
  branch: string
  checks: Checks
  files: number
  added: number
  removed: number
  author: string
  when: string
  body: string
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

export interface Project {
  id: string
  /** `owner/repo` where there is a remote, and the folder name where not. */
  name: string
  /** Where it lives on disk, as the titlebar prints it. */
  path: string
  branch: string
  /** How far the branch has drifted from its upstream. */
  ahead: number
  behind: number
  provider: Provider
  sessions: Session[]
  issues: Issue[]
  pulls: Pull[]
  changes: WorkingChange[]
}

/** The panes a project has, beside its sessions. */
export type ProjectPane = 'issues' | 'pulls' | 'changes'

/** The pages of the settings view, in the order its column lists them. */
export type SettingsSection =
  | 'general'
  | 'agent'
  | 'permissions'
  | 'integrations'
  | 'shortcuts'

/**
 * What the main column is showing: a session, one of the project panes, or
 * the settings, which belong to the window rather than to any one project.
 */
export type View =
  | { kind: 'session'; id: string }
  | { kind: 'pane'; pane: ProjectPane }
  | { kind: 'settings'; section: SettingsSection }

export function openIssues(project: Project): Issue[] {
  return project.issues.filter(issue => issue.state === 'open')
}

export function openPulls(project: Project): Pull[] {
  return project.pulls.filter(pull => pull.state !== 'merged')
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
