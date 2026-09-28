import {
  Badge,
  Button,
  Card,
  EmptyState,
  Icon,
  KeyCap,
  SearchField,
} from '@dazzlabs/dazzui-react'
import { useEffect, useMemo, useState } from 'react'

import {
  ArchiveGlyph,
  ArrowRightGlyph,
  IssueGlyph,
  PlusGlyph,
  PullGlyph,
  RocketGlyph,
  SparkleGlyph,
} from '../data/icons'
import {
  type CheckRun,
  checksStatus,
  type GithubFace,
  type Issue,
  type Pull,
  type Session,
} from '../lib/project'
import { type Mode, MODES } from '../lib/settings'
import {
  PullDetail,
  type PullDetailProps,
  ProviderEmpty,
  RunLight,
} from './pull-view'
import { slugOf } from './start-session-dialog'
import { STATUS_LABEL, StatusDot } from './status-dot'

/** A check run, and the pull request it ran on. */
export interface CheckRef {
  pull: Pull
  run: CheckRun
}

export function checkKey(pull: Pull, run: CheckRun): string {
  return `${pull.number}:${run.name}`
}

/** True where the key was pressed in a field, and belongs to it. */
function inField(target: EventTarget | null): boolean {
  return (
    target instanceof HTMLElement &&
    (target.tagName === 'INPUT' ||
      target.tagName === 'TEXTAREA' ||
      target.isContentEditable)
  )
}

export interface GithubViewProps extends Omit<PullDetailProps, 'pull'> {
  face: GithubFace
  connected: boolean
  onConnect: () => void
  /** The project's sessions with their live states, to say which issue has one. */
  sessions: readonly Session[]
  onOpenSession: (id: string) => void
  /** Which slice of the issue list the sidebar has selected. */
  issueFilter: string
  issue: Issue | null
  onSelectIssue: (number: number) => void
  pull: Pull | null
  onSelectPull: (number: number) => void
  check: CheckRef | null
  onSelectCheck: (key: string) => void
  /** The mode the window is set to, which a session from an issue starts in. */
  mode: Mode
  /** Start a session from the issue with the defaults, at once. */
  onStart: (issue: Issue) => void
  /** Open the sheet to change the defaults first. */
  onPlan: (issue: Issue) => void
}

/**
 * GitHub, in three columns the way the Tiller canvas draws it: the sidebar
 * names the face, the list in the middle scans it, and the pane on the
 * right reads one row of it.
 *
 * The issues face is where a session comes from, and its pane says so: the
 * issue, then a card that opens a session from it with every default
 * already filled — the worktree, the branch, what goes in as the first
 * context — so Return is enough. Under that is the monitor: every open pull
 * request and the checks on it, with the one control a red check needs.
 * The pull requests face reads one at a time, and the checks face is the
 * same monitor turned round, one run at a time with what it printed.
 */
export function GithubView({
  project,
  face,
  connected,
  onConnect,
  sessions,
  onOpenSession,
  issueFilter,
  issue,
  onSelectIssue,
  pull,
  onSelectPull,
  check,
  onSelectCheck,
  mode,
  onStart,
  onPlan,
  mergeMethod,
  onMergeMethod,
  onMerge,
  onAgentReply,
  onAutoFix,
  onLog,
}: GithubViewProps) {
  const [query, setQuery] = useState('')

  const issues = useMemo(() => {
    const needle = query.trim().toLowerCase()
    return project.issues
      .filter(entry => {
        switch (issueFilter) {
          case 'closed':
            return entry.state === 'closed'
          case 'mine':
            return entry.state === 'open' && entry.author === 'mia'
          case 'agent':
            return entry.state === 'open' && entry.agentReady
          default:
            return entry.state === 'open'
        }
      })
      .filter(
        entry =>
          !needle ||
          `#${entry.number} ${entry.title} ${entry.label}`
            .toLowerCase()
            .includes(needle)
      )
  }, [project.issues, issueFilter, query])

  const open = project.pulls.filter(entry => entry.state !== 'merged')
  const merged = project.pulls.filter(entry => entry.state === 'merged')
  const runs: CheckRef[] = open.flatMap(entry =>
    entry.runs.map(run => ({ pull: entry, run }))
  )

  const sessionFor = (entry: Issue) =>
    sessions.find(session => session.issue === entry.number)

  /* ↑ and ↓ walk the issue list; Return opens the sheet on the one under
     the cursor and S starts it with the defaults. The list is the thing the
     keys are for — an issue is read once and then handed over. */
  useEffect(() => {
    if (face !== 'issues') return
    const onKey = (event: KeyboardEvent) => {
      if (event.metaKey || event.ctrlKey || event.altKey) return
      if (inField(event.target)) return
      const at = issues.findIndex(entry => entry.number === issue?.number)
      switch (event.key) {
        case 'ArrowDown': {
          event.preventDefault()
          const next = issues[Math.min(issues.length - 1, at + 1)]
          if (next) onSelectIssue(next.number)
          return
        }
        case 'ArrowUp': {
          event.preventDefault()
          const next = issues[Math.max(0, at - 1)]
          if (next) onSelectIssue(next.number)
          return
        }
        case 'Enter':
          if (issue?.agentReady) {
            event.preventDefault()
            onPlan(issue)
          }
          return
        case 's':
          if (issue?.agentReady) {
            event.preventDefault()
            onStart(issue)
          }
          return
      }
    }
    document.addEventListener('keydown', onKey)
    return () => document.removeEventListener('keydown', onKey)
  }, [face, issues, issue, onSelectIssue, onPlan, onStart])

  const title =
    face === 'issues' ? 'Issues' : face === 'pulls' ? 'Pull requests' : 'Checks'

  if (!connected) {
    return (
      <div className="wb-view">
        <header className="wb-view__head">
          <h1 className="wb-view__title">{title}</h1>
        </header>
        <div className="wb-view__body">
          <ProviderEmpty
            project={project}
            what={title.toLowerCase()}
            onConnect={onConnect}
          />
        </div>
      </div>
    )
  }

  /* The monitor: every open pull request, its red checks with their verbs,
     and the merged ones as a line each. */
  const monitor = (
    <>
      <span className="wb-label">Pull requests · checks</span>
      {open.length === 0 && merged.length === 0 ? (
        <span className="wb-card__quiet">No pull request is open.</span>
      ) : null}
      {open.map(entry => {
        const failed = entry.runs.filter(run => run.status === 'failed')
        const rest = entry.runs.filter(run => run.status !== 'failed')
        const session = project.sessions.find(item => item.id === entry.session)
        return (
          <div key={entry.number} className="wb-gh__pull">
            <button
              className="wb-gh__pull-row wb-gh__pull-head"
              data-head
              type="button"
              onClick={() => onSelectPull(entry.number)}
            >
              <RunLight status={checksStatus(entry)} />
              <span className="wb-truncate">
                #{entry.number} {entry.title}
              </span>
              <span className="wb-view__gap" />
              {session ? (
                <span className="wb-card__quiet">Session: {session.title}</span>
              ) : null}
            </button>
            {failed.map(run => (
              <div key={run.name} className="wb-gh__pull-row">
                <RunLight status="failed" />
                <span className="wb-mono">{run.name}</span>
                {run.note ? (
                  <span className="wb-card__quiet">— {run.note}</span>
                ) : null}
                <span className="wb-view__gap" />
                <span className="wb-check__actions">
                  <Button size="tiny" onClick={() => onAutoFix(entry, run)}>
                    <Icon icon={SparkleGlyph} size={12} />
                    Auto-fix
                  </Button>
                  <Button
                    size="tiny"
                    tint="neutral"
                    variant="plain"
                    onClick={() => onLog(entry, run)}
                  >
                    Log
                  </Button>
                </span>
              </div>
            ))}
            {rest.length > 0 ? (
              <div className="wb-gh__pull-row">
                <RunLight
                  status={
                    rest.some(run => run.status === 'running')
                      ? 'running'
                      : rest.some(run => run.status === 'queued')
                        ? 'queued'
                        : 'passed'
                  }
                />
                <span className="wb-mono">
                  {rest.map(run => run.name).join(' · ')}
                </span>
                <span className="wb-view__gap" />
                <span className="wb-card__quiet">
                  {rest.every(run => run.status === 'passed')
                    ? 'passed'
                    : rest.some(run => run.status === 'running')
                      ? 'running…'
                      : 'queued'}
                </span>
              </div>
            ) : null}
          </div>
        )
      })}
      {merged.map(entry => (
        <div key={entry.number} className="wb-gh__pull">
          <div className="wb-gh__pull-row">
            <span data-tint="primary" className="wb-mark">
              <Icon icon={ArchiveGlyph} size={14} />
            </span>
            <span className="wb-truncate">
              #{entry.number} {entry.title}
            </span>
            <Badge size="small" tint="primary" variant="tinted">
              merged
            </Badge>
            <span className="wb-view__gap" />
            <span className="wb-card__quiet">
              Session archived · worktree cleaned up
            </span>
          </div>
        </div>
      ))}
    </>
  )

  const list = (() => {
    switch (face) {
      case 'issues':
        return (
          <>
            <div className="wb-gh__rows">
              {issues.length === 0 ? (
                <EmptyState title="Nothing in this filter." />
              ) : null}
              {issues.map(entry => {
                const session = sessionFor(entry)
                return (
                  <button
                    key={entry.number}
                    aria-current={entry.number === issue?.number || undefined}
                    data-tint="primary"
                    className="wb-gh__row"
                    type="button"
                    onClick={() => onSelectIssue(entry.number)}
                  >
                    <span className="wb-gh__line">
                      <span
                        data-tint={
                          entry.state === 'open' ? 'success' : undefined
                        }
                        className="wb-mark"
                      >
                        <Icon icon={IssueGlyph} size={15} />
                      </span>
                      <span className="wb-gh__row-title">{entry.title}</span>
                      <span className="wb-gh__number">#{entry.number}</span>
                    </span>
                    <span
                      data-tint={session ? 'primary' : undefined}
                      className="wb-gh__meta"
                    >
                      <Badge size="small" variant="outlined">
                        {entry.label}
                      </Badge>
                      {session ? (
                        <span className="wb-gh__live">
                          <StatusDot size="small" status={session.status} />
                          Session {STATUS_LABEL[session.status].toLowerCase()}
                        </span>
                      ) : (
                        <span>
                          {entry.when} · no session
                          {entry.pull ? ` · #${entry.pull}` : ''}
                        </span>
                      )}
                    </span>
                  </button>
                )
              })}
            </div>
            <span className="wb-gh__keys">
              <KeyCap size="small" variant="key">
                ↑↓
              </KeyCap>{' '}
              select ·{' '}
              <KeyCap size="small" variant="key">
                ⏎
              </KeyCap>{' '}
              open a session ·{' '}
              <KeyCap size="small" variant="key">
                S
              </KeyCap>{' '}
              start with the defaults
            </span>
          </>
        )

      case 'pulls':
        return (
          <div className="wb-gh__rows">
            {project.pulls.length === 0 ? (
              <EmptyState title="No pull request is open on this project yet." />
            ) : null}
            {project.pulls.map(entry => (
              <button
                key={entry.number}
                aria-current={entry.number === pull?.number || undefined}
                data-tint="primary"
                className="wb-gh__row"
                type="button"
                onClick={() => onSelectPull(entry.number)}
              >
                <span className="wb-gh__line">
                  <span
                    data-tint={entry.state === 'merged' ? 'primary' : 'success'}
                    className="wb-mark"
                  >
                    <Icon icon={PullGlyph} size={15} />
                  </span>
                  <span className="wb-gh__row-title">{entry.title}</span>
                  <span className="wb-gh__number">#{entry.number}</span>
                </span>
                <span className="wb-gh__meta">
                  <Badge
                    size="small"
                    tint={entry.state === 'merged' ? 'primary' : 'info'}
                    variant="tinted"
                  >
                    {entry.state}
                  </Badge>
                  <span>
                    {entry.when} · {entry.branch}
                  </span>
                  {entry.state !== 'merged' ? (
                    <RunLight status={checksStatus(entry)} />
                  ) : null}
                </span>
              </button>
            ))}
          </div>
        )

      case 'checks':
        return (
          <div className="wb-gh__rows">
            {runs.length === 0 ? (
              <EmptyState title="No check is running on an open pull request." />
            ) : null}
            {runs.map(entry => (
              <button
                key={checkKey(entry.pull, entry.run)}
                aria-current={
                  (check &&
                    checkKey(check.pull, check.run) ===
                      checkKey(entry.pull, entry.run)) ||
                  undefined
                }
                data-tint="primary"
                className="wb-gh__row"
                type="button"
                onClick={() => onSelectCheck(checkKey(entry.pull, entry.run))}
              >
                <span className="wb-gh__line">
                  <RunLight status={entry.run.status} />
                  <span className="wb-gh__row-title wb-mono">
                    {entry.run.name}
                  </span>
                  <span className="wb-gh__number">#{entry.pull.number}</span>
                </span>
                <span className="wb-gh__meta">
                  <span>
                    {entry.run.status === 'queued'
                      ? 'waiting for a runner'
                      : entry.run.took}
                    {entry.run.note ? ` · ${entry.run.note}` : ''}
                  </span>
                </span>
              </button>
            ))}
          </div>
        )
    }
  })()

  const pane = (() => {
    switch (face) {
      case 'issues': {
        if (!issue) {
          return <EmptyState title="Pick an issue to read it." />
        }
        const session = sessionFor(issue)
        const slug = `${issue.number}-${slugOf(issue.title)}`
        const chosen = MODES.find(entry => entry.value === mode)
        return (
          <>
            <header className="wb-gh__head">
              <span
                data-tint={issue.state === 'open' ? 'success' : undefined}
                className="wb-mark"
              >
                <Icon icon={IssueGlyph} size={18} />
              </span>
              <h2 className="wb-gh__title wb-truncate">{issue.title}</h2>
              <span className="wb-gh__number">#{issue.number}</span>
            </header>
            <span className="wb-facts">
              <Badge size="small" variant="outlined">
                {issue.label}
              </Badge>
              <span>
                {issue.author} opened this {issue.when}
                {issue.comments > 0 ? ` · ${issue.comments} comments` : ''}
                {issue.pull ? ` · linked to #${issue.pull}` : ''}
              </span>
            </span>

            <p className="wb-gh__body">{issue.body}</p>

            {session ? (
              <Card size="large" variant="raised">
                <span className="wb-card__head">
                  <StatusDot size="small" status={session.status} />
                  <span className="wb-card__title">
                    A session has this issue
                  </span>
                </span>
                <span className="wb-card__note">
                  “{session.title}” · {STATUS_LABEL[session.status]}
                  {session.worktree ? ` · ${session.worktree}` : ''}
                </span>
                <span className="wb-card__head">
                  <Button
                    size="small"
                    onClick={() => onOpenSession(session.id)}
                  >
                    Open the session
                    <Icon icon={ArrowRightGlyph} size={13} />
                  </Button>
                </span>
              </Card>
            ) : (
              <Card size="large" variant="raised">
                <span className="wb-card__head">
                  <span data-tint="primary" className="wb-mark">
                    <Icon icon={RocketGlyph} size={16} />
                  </span>
                  <span className="wb-card__title">
                    Open a session from this issue
                  </span>
                </span>
                <dl className="wb-gh__facts">
                  <dt>worktree</dt>
                  <dd className="wb-mono">worktrees/{slug}</dd>
                  <dt>branch</dt>
                  <dd className="wb-mono">
                    feat/{slug} ← {project.branch}
                  </dd>
                  <dt>first context</dt>
                  <dd>
                    The title, the body and {issue.comments}{' '}
                    {issue.comments === 1 ? 'comment' : 'comments'}, injected
                  </dd>
                </dl>
                <span className="wb-card__head">
                  <Button
                    disabled={!issue.agentReady}
                    size="small"
                    variant="filled"
                    onClick={() => onStart(issue)}
                  >
                    Open session
                  </Button>
                  <Button
                    disabled={!issue.agentReady}
                    size="small"
                    tint="neutral"
                    variant="plain"
                    onClick={() => onPlan(issue)}
                  >
                    Change the defaults first…
                  </Button>
                  <span className="wb-view__gap" />
                  <span className="wb-card__quiet">
                    {issue.agentReady
                      ? `Mode: ${chosen?.label ?? mode}`
                      : 'Too broad to hand over as it stands'}
                  </span>
                </span>
              </Card>
            )}

            <div className="wb-rule" />
            {monitor}
          </>
        )
      }

      case 'pulls':
        return pull ? (
          <PullDetail
            mergeMethod={mergeMethod}
            project={project}
            pull={pull}
            onAgentReply={onAgentReply}
            onAutoFix={onAutoFix}
            onLog={onLog}
            onMerge={onMerge}
            onMergeMethod={onMergeMethod}
          />
        ) : (
          <EmptyState title="Pick a pull request to read it." />
        )

      case 'checks':
        return check ? (
          <>
            <header className="wb-gh__head">
              <RunLight status={check.run.status} />
              <h2 className="wb-gh__title wb-truncate wb-mono">
                {check.run.name}
              </h2>
              <Badge
                size="small"
                tint={
                  check.run.status === 'failed'
                    ? 'danger'
                    : check.run.status === 'passed'
                      ? 'success'
                      : 'warning'
                }
                variant="tinted"
              >
                {check.run.status}
              </Badge>
              <span className="wb-view__gap" />
              <Button
                size="small"
                tint="neutral"
                variant="plain"
                onClick={() => onSelectPull(check.pull.number)}
              >
                #{check.pull.number} {check.pull.title}
                <Icon icon={ArrowRightGlyph} size={13} />
              </Button>
            </header>
            <span className="wb-facts">
              <span>
                on {check.pull.branch} ·{' '}
                {check.run.status === 'queued'
                  ? 'waiting for a runner'
                  : check.run.took}
              </span>
            </span>
            <div className="wb-terminal__body wb-terminal__body--flat">
              <div className="wb-terminal__command">
                $ gh run view --job "{check.run.name}" --log
                {check.run.status === 'failed' ? '-failed' : ''}
              </div>
              <div>
                {check.run.status === 'failed' ? '✗' : '✓'} {check.run.name} ·{' '}
                {check.run.took ?? ''}
              </div>
              <div>
                {check.run.note ??
                  (check.run.status === 'running'
                    ? '  still running…'
                    : '  no findings')}
              </div>
            </div>
            {check.run.status === 'failed' ? (
              <span className="wb-card__head">
                <Button
                  size="small"
                  variant="filled"
                  onClick={() => onAutoFix(check.pull, check.run)}
                >
                  <Icon icon={SparkleGlyph} size={13} />
                  Auto-fix
                </Button>
                <Button
                  size="small"
                  tint="neutral"
                  variant="plain"
                  onClick={() => onLog(check.pull, check.run)}
                >
                  Full log
                </Button>
              </span>
            ) : null}
          </>
        ) : (
          <EmptyState title="Pick a check to read what it printed." />
        )
    }
  })()

  return (
    <div className="wb-view">
      <header className="wb-view__head">
        <h1 className="wb-view__title">{title}</h1>
        <Badge size="small" tint="neutral">
          {project.name}
        </Badge>
        <span className="wb-view__gap" />
        {face === 'issues' ? (
          <>
            <SearchField
              className="wb-gh__search"
              placeholder="Search issues…"
              size="small"
              value={query}
              onValueChange={setQuery}
            />
            <Button size="small" variant="filled">
              <Icon icon={PlusGlyph} size={14} />
              New issue
            </Button>
          </>
        ) : null}
      </header>

      <div className="wb-gh">
        <div className="wb-gh__list">{list}</div>
        <div className="wb-gh__pane">{pane}</div>
      </div>
    </div>
  )
}
