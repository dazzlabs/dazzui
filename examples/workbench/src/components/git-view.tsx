import {
  Badge,
  Button,
  Checkbox,
  Icon,
  Table,
  TableCell,
  TableHead,
  TableRow,
  TextField,
} from '@dazzlabs/dazzui-react'

import {
  ArrowRightGlyph,
  BranchGlyph,
  DownloadGlyph,
  PlusGlyph,
  SendGlyph,
  SparkleGlyph,
  TreeGlyph,
} from '../data/icons'
import {
  type Project,
  type WorkingChange,
  type Worktree,
  worktreeStatus,
} from '../lib/project'
import { STATUS_LABEL, StatusDot } from './status-dot'

export interface GitViewProps {
  project: Project
  changes: readonly WorkingChange[]
  isStaged: (change: WorkingChange) => boolean
  onStage: (change: WorkingChange, staged: boolean) => void
  message: string
  onMessageChange: (message: string) => void
  onWriteMessage: () => void
  /** The issue the open session came from, which the commit closes. */
  issue: number | null
  onCommit: () => void
  onCommitPush: () => void
  onPull: () => void
  onPush: () => void
  onOpenSession: (id: string) => void
  onNewWorktree: () => void
  onCleanWorktree: (tree: Worktree) => void
}

/**
 * Git, in three columns the way the Tiller canvas draws its Git face: the
 * sidebar holds the branches and the remote, the column in the middle is
 * the working tree with the commit box at its foot, and the pane on the
 * right is the history and, under it, every worktree on disk.
 *
 * The middle column is the whole of a commit — what goes in, and what it
 * says — with nothing else on it, so staging a file and writing the line
 * never leave each other's sight. The history and the worktrees are what
 * the commit lands in and where it is made, which is why they stand
 * beside it rather than above or below.
 */
export function GitView({
  project,
  changes,
  isStaged,
  onStage,
  message,
  onMessageChange,
  onWriteMessage,
  issue,
  onCommit,
  onCommitPush,
  onPull,
  onPush,
  onOpenSession,
  onNewWorktree,
  onCleanWorktree,
}: GitViewProps) {
  const staged = changes.filter(isStaged)
  const rest = changes.filter(change => !isStaged(change))
  const ready = staged.length > 0 && message.trim().length > 0

  const row = (change: WorkingChange, on: boolean) => (
    <li key={change.path} data-staged={on} className="wb-git__row">
      <Checkbox
        aria-label={on ? `Unstage ${change.path}` : `Stage ${change.path}`}
        checked={on}
        size="small"
        onCheckedChange={next => onStage(change, next)}
      />
      <span className="wb-truncate wb-mono wb-git__path">{change.path}</span>
      <span className="wb-counts">
        <span data-tint="success" className="wb-count">
          +{change.added}
        </span>
        <span data-tint="danger" className="wb-count">
          −{change.removed}
        </span>
      </span>
    </li>
  )

  return (
    <div className="wb-view">
      <header className="wb-view__head">
        <h1 className="wb-view__title">Git</h1>
        <Badge size="small" variant="tinted">
          <Icon icon={BranchGlyph} size={11} />
          {project.branch}
        </Badge>
        <span className="wb-view__gap" />
        <Button size="small" tint="neutral" variant="plain" onClick={onPull}>
          <Icon icon={DownloadGlyph} size={14} />
          Pull
        </Button>
        <Button disabled={project.ahead === 0} size="small" onClick={onPush}>
          <Icon icon={SendGlyph} size={14} />
          Push {project.ahead} {project.ahead === 1 ? 'commit' : 'commits'}
        </Button>
      </header>

      <div className="wb-git">
        {/* The working tree, and the commit it becomes. */}
        <section className="wb-git__changes">
          <span className="wb-git__label">Changes · {project.branch}</span>

          <div className="wb-git__rows">
            {changes.length === 0 ? (
              <span className="wb-card__quiet wb-git__none">
                Nothing to commit — the working tree is clean.
              </span>
            ) : null}
            {staged.length > 0 ? (
              <>
                <span className="wb-git__sub">Staged · {staged.length}</span>
                <ul className="wb-git__list">
                  {staged.map(change => row(change, true))}
                </ul>
              </>
            ) : null}
            {rest.length > 0 ? (
              <>
                <span className="wb-git__sub">Unstaged · {rest.length}</span>
                <ul className="wb-git__list">
                  {rest.map(change => row(change, false))}
                </ul>
              </>
            ) : null}
          </div>

          <div className="wb-git__commit">
            <TextField
              aria-label="Commit message"
              className="wb-mono"
              placeholder="Say what changed, and why"
              size="small"
              value={message}
              onChange={event => onMessageChange(event.target.value)}
            />
            <span className="wb-git__commit-row">
              {issue ? (
                <Badge size="small" variant="tinted">
                  Fixes #{issue}
                </Badge>
              ) : null}
              <Button
                size="tiny"
                tint="neutral"
                variant="plain"
                onClick={onWriteMessage}
              >
                <Icon icon={SparkleGlyph} size={12} />
                Agent writes it
              </Button>
              <span className="wb-view__gap" />
              <Button
                disabled={!ready}
                size="small"
                tint="neutral"
                variant="outlined"
                onClick={onCommit}
              >
                Commit
              </Button>
              <Button
                disabled={!ready}
                size="small"
                variant="filled"
                onClick={onCommitPush}
              >
                Commit + Push
              </Button>
            </span>
          </div>
        </section>

        {/* What the commit lands in, and where it is made. */}
        <div className="wb-git__pane">
          <section className="wb-git__section">
            <span className="wb-git__label">History</span>
            <ul className="wb-git__history">
              {project.history.map((commit, index) => (
                <li key={commit.sha} className="wb-git__commit-line">
                  <span data-agent={commit.agent} className="wb-commit__dot" />
                  <code className="wb-mono wb-git__sha">{commit.sha}</code>
                  <span className="wb-truncate wb-git__message">
                    {commit.message}
                  </span>
                  {index === 0 ? (
                    <Badge size="small" variant="outlined">
                      HEAD
                    </Badge>
                  ) : null}
                  <span className="wb-git__who">
                    {commit.agent ? 'agent' : commit.author} · {commit.when}
                  </span>
                </li>
              ))}
            </ul>
          </section>

          <section className="wb-git__section">
            <span className="wb-card__head">
              <span className="wb-git__label">Worktrees · worktrees/</span>
              <span className="wb-view__gap" />
              <Button
                size="tiny"
                tint="neutral"
                variant="plain"
                onClick={onNewWorktree}
              >
                <Icon icon={PlusGlyph} size={12} />
                New
              </Button>
            </span>

            <Table className="wb-trees">
              <TableHead>
                <TableCell head className="wb-trees__path">
                  Path
                </TableCell>
                <TableCell head className="wb-trees__branch">
                  Branch
                </TableCell>
                <TableCell head className="wb-trees__session">
                  Session
                </TableCell>
                <TableCell head className="wb-trees__state">
                  State
                </TableCell>
                <TableCell head align="end" className="wb-trees__action" />
              </TableHead>

              <TableRow>
                <TableCell className="wb-trees__path">
                  <Icon icon={TreeGlyph} size={13} />
                  <span className="wb-truncate wb-mono">(main checkout)</span>
                </TableCell>
                <TableCell className="wb-trees__branch">
                  <span className="wb-truncate wb-mono">{project.branch}</span>
                </TableCell>
                <TableCell className="wb-trees__session">
                  <span className="wb-card__quiet">—</span>
                </TableCell>
                <TableCell className="wb-trees__state">
                  {changes.length === 0 ? 'Clean' : `±${changes.length}`}
                </TableCell>
                <TableCell align="end" className="wb-trees__action" />
              </TableRow>

              {project.worktrees.map(tree => {
                const state = worktreeStatus(project, tree)
                return (
                  <TableRow key={tree.path} data-archived={tree.archived}>
                    <TableCell className="wb-trees__path">
                      <Icon icon={TreeGlyph} size={13} />
                      <span className="wb-truncate wb-mono">{tree.path}</span>
                    </TableCell>
                    <TableCell className="wb-trees__branch">
                      <span className="wb-truncate wb-mono">{tree.branch}</span>
                    </TableCell>
                    <TableCell className="wb-trees__session">
                      {state.status ? (
                        <>
                          <StatusDot size="small" status={state.status} />
                          <span className="wb-truncate">{state.label}</span>
                        </>
                      ) : (
                        <span className="wb-card__quiet">
                          {tree.archived ? 'Archived' : '—'}
                        </span>
                      )}
                    </TableCell>
                    <TableCell className="wb-trees__state">
                      {state.status ? STATUS_LABEL[state.status] : state.label}
                    </TableCell>
                    <TableCell align="end" className="wb-trees__action">
                      {tree.archived ? (
                        <Button
                          size="tiny"
                          tint="neutral"
                          variant="outlined"
                          onClick={() => onCleanWorktree(tree)}
                        >
                          Clean up
                        </Button>
                      ) : tree.session ? (
                        <Button
                          size="tiny"
                          tint="neutral"
                          variant="plain"
                          onClick={() => onOpenSession(tree.session ?? '')}
                        >
                          Open
                          <Icon icon={ArrowRightGlyph} size={11} />
                        </Button>
                      ) : null}
                    </TableCell>
                  </TableRow>
                )
              })}
            </Table>
          </section>
        </div>
      </div>
    </div>
  )
}
