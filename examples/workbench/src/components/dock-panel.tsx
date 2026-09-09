import {
  Badge,
  Button,
  EmptyState,
  Icon,
  PillTabs,
} from '@dazzlabs/dazzui-react'
import { useState } from 'react'

import {
  ArrowRightGlyph,
  BranchGlyph,
  DocGlyph,
  ExternalGlyph,
  TerminalGlyph,
} from '../data/icons'
import { checksStatus, type Pull, type WorkingChange } from '../lib/project'
import type { Entry, ToolEntry } from '../lib/transcript'
import { DiffView } from './diff-view'
import { RunLight } from './pull-view'

export type DockTab = 'changes' | 'terminal' | 'pull'

export interface DockPanelProps {
  changes: readonly WorkingChange[]
  /** The transcript, which is where the terminal's history is read from. */
  entries: readonly Entry[]
  /** The pull request the open session is working towards, if any. */
  pull: Pull | null
  onReview: () => void
  onOpenPull: () => void
}

/**
 * The dock beside the thread: what the session is *doing to the repository*,
 * in three views the reader can switch between without leaving the
 * conversation.
 *
 * Changes is a summary rather than a review — what the agent has touched and
 * the head of the newest diff, so the reader can see the shape of it; the
 * review page is where it is decided. Terminal is every command the run has
 * issued, in order, as one scrollback rather than a card per call. And the
 * pull request tab is the one that closes the loop: the checks on the branch
 * this session is pushing to, drawn where the pushing happens.
 */
export function DockPanel({
  changes,
  entries,
  pull,
  onReview,
  onOpenPull,
}: DockPanelProps) {
  const [chosen, setChosen] = useState<DockTab>('changes')
  /* A tab for a pull request that is not there falls back rather than
     showing an empty pane the reader did not ask for. */
  const tab: DockTab = chosen === 'pull' && !pull ? 'changes' : chosen

  const commands = entries.filter(
    (entry): entry is ToolEntry =>
      entry.kind === 'tool' && entry.tool === 'Bash'
  )
  const newest = changes[0]

  const tabs = [
    { value: 'changes' as const, label: 'Changes', count: changes.length },
    { value: 'terminal' as const, label: 'Terminal', count: commands.length },
    ...(pull ? [{ value: 'pull' as const, label: `PR #${pull.number}` }] : []),
  ]

  return (
    <aside className="wb-dock">
      <header className="wb-dock__head">
        <PillTabs
          aria-label="Dock"
          className="wb-dock__tabs"
          items={tabs}
          value={tab}
          onValueChange={setChosen}
        />
      </header>

      {tab === 'changes' ? (
        <div className="wb-dock__body">
          {changes.length === 0 ? (
            <EmptyState title="Nothing has been written yet. Files appear here as the agent works." />
          ) : null}

          {changes.map(change => (
            <div key={change.path} className="wb-dock__row">
              <Icon icon={DocGlyph} size={14} />
              <span className="wb-truncate wb-mono">{change.path}</span>
              <span className="wb-counts">
                <span data-tint="success" className="wb-count">
                  +{change.added}
                </span>
                <span data-tint="danger" className="wb-count">
                  −{change.removed}
                </span>
              </span>
            </div>
          ))}

          {newest ? (
            <div className="wb-dock__preview">
              <span className="wb-dock__file wb-mono">{newest.path}</span>
              <DiffView lines={newest.hunk} />
            </div>
          ) : null}
        </div>
      ) : null}

      {tab === 'terminal' ? (
        <div className="wb-dock__terminal">
          {commands.length === 0 ? (
            <div className="wb-dock__prompt">
              <span className="wb-terminal__command">❯</span> No command has run
              in this session yet.
            </div>
          ) : null}
          {commands.map(entry => (
            <div key={entry.id} className="wb-dock__run">
              <div className="wb-terminal__command">
                ❯ {entry.target}
                {entry.status === 'running' ? (
                  <span className="wb-dock__running"> · running</span>
                ) : entry.status === 'stopped' ? (
                  <span className="wb-dock__running"> · interrupted</span>
                ) : null}
              </div>
              {entry.output?.kind === 'lines'
                ? entry.output.lines.map((line, index) => (
                    <div key={index}>{line}</div>
                  ))
                : null}
            </div>
          ))}
        </div>
      ) : null}

      {tab === 'pull' && pull ? (
        <div className="wb-dock__body">
          <div className="wb-dock__pull">
            <span className="wb-dock__title">
              <RunLight status={checksStatus(pull)} />
              <span className="wb-truncate">
                #{pull.number} {pull.title}
              </span>
              <Badge
                size="small"
                tint={pull.state === 'merged' ? 'primary' : 'info'}
                variant="tinted"
              >
                {pull.state}
              </Badge>
            </span>
            <span className="wb-facts wb-mono">
              <Icon icon={BranchGlyph} size={11} />
              {pull.branch} → {pull.base}
              {pull.fixes ? ` · fixes #${pull.fixes}` : ''}
            </span>
          </div>

          <ul className="wb-rows wb-dock__checks">
            {pull.runs.map(run => (
              <li key={run.name}>
                <RunLight status={run.status} />
                <span className="wb-truncate wb-mono">{run.name}</span>
                <span className="wb-rows__meta">
                  {run.status === 'queued' ? 'waiting' : run.took}
                </span>
              </li>
            ))}
          </ul>

          <span className="wb-dock__actions">
            <Button size="small" onClick={onOpenPull}>
              Open the pull request
              <Icon icon={ArrowRightGlyph} size={12} />
            </Button>
            <Button size="small" tint="neutral" variant="plain">
              GitHub
              <Icon icon={ExternalGlyph} size={12} />
            </Button>
          </span>
        </div>
      ) : null}

      {tab === 'changes' ? (
        <footer className="wb-dock__foot">
          <span className="wb-truncate wb-card__quiet">
            {changes.length === 0
              ? 'The review page opens once there is something to read.'
              : `${changes.length} ${changes.length === 1 ? 'file' : 'files'} to read`}
          </span>
          <Button
            disabled={changes.length === 0}
            size="tiny"
            variant="plain"
            onClick={onReview}
          >
            Full review
            <Icon icon={ArrowRightGlyph} size={12} />
          </Button>
        </footer>
      ) : null}

      {tab === 'terminal' ? (
        <footer className="wb-dock__foot">
          <span className="wb-truncate wb-card__quiet">
            <Icon icon={TerminalGlyph} size={12} /> Every command the run has
            issued, oldest first.
          </span>
        </footer>
      ) : null}
    </aside>
  )
}
