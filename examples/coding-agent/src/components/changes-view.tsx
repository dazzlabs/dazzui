import {
  Badge,
  Button,
  EmptyState,
  Icon,
  IconButton,
  SectionLabel,
} from '@dazzlabs/dazzui-react'
import { useState } from 'react'

import { CheckGlyph, PlusGlyph } from '../data/icons'
import type { ChangeStatus, WorkingChange } from '../lib/project'
import { DiffView } from './diff-view'
import { Fold } from './fold'

/**
 * What the letter in the gutter of a git status means, and which ramp it takes.
 * Deleted is danger, added is success, and modified stays neutral — a file
 * that merely changed is the ordinary case and should not read as an event.
 */
const STATUS: Record<
  ChangeStatus,
  { letter: string; tint: 'success' | 'danger' | 'neutral' }
> = {
  added: { letter: 'A', tint: 'success' },
  modified: { letter: 'M', tint: 'neutral' },
  deleted: { letter: 'D', tint: 'danger' },
}

function Row({
  change,
  open,
  staged,
  onToggle,
  onStage,
}: {
  change: WorkingChange
  open: boolean
  staged: boolean
  onToggle: () => void
  onStage: () => void
}) {
  const status = STATUS[change.status]

  return (
    <Fold
      action={
        <IconButton
          label={staged ? `Unstage ${change.path}` : `Stage ${change.path}`}
          tint={staged ? 'neutral' : 'success'}
          onClick={onStage}
        >
          <Icon icon={staged ? PlusGlyph : CheckGlyph} size={14} />
        </IconButton>
      }
      open={open}
      summary={
        <>
          <Badge size="small" tint={status.tint} variant="outlined">
            {status.letter}
          </Badge>
          <span className="ca-change__name">
            {change.path.split('/').pop()}
          </span>
          <span className="ca-change__dir">
            {change.path.split('/').slice(0, -1).join('/')}
          </span>
          {/* The counts take the content grade of their ramp through the
              tinted recipe, rather than a step written here — a fixed step
              cannot be right on both the paper canvas and the near-black one. */}
          <span className="ca-change__counts">
            <span data-tint="success" className="ca-change__count">
              +{change.added}
            </span>
            <span data-tint="danger" className="ca-change__count">
              -{change.removed}
            </span>
          </span>
        </>
      }
      onToggle={onToggle}
    >
      <DiffView lines={change.hunk} />
    </Fold>
  )
}

export interface ChangesViewProps {
  changes: readonly WorkingChange[]
  branch: string
  isStaged: (change: WorkingChange) => boolean
  onStage: (change: WorkingChange, staged: boolean) => void
  onCommit: () => void
}

/** The working tree: what is written but not yet committed. */
export function ChangesView({
  changes,
  branch,
  isStaged,
  onStage,
  onCommit,
}: ChangesViewProps) {
  const [open, setOpen] = useState<string[]>([])

  const staged = changes.filter(isStaged)
  const rest = changes.filter(change => !isStaged(change))
  const added = changes.reduce((sum, file) => sum + file.added, 0)
  const removed = changes.reduce((sum, file) => sum + file.removed, 0)

  const toggle = (path: string) =>
    setOpen(list =>
      list.includes(path) ? list.filter(item => item !== path) : [...list, path]
    )

  const section = (label: string, files: WorkingChange[], action?: string) =>
    files.length === 0 ? null : (
      <section className="ca-group">
        <div className="ca-group__head">
          <SectionLabel faint>
            {label} · {files.length}
          </SectionLabel>
          {action ? (
            <Button
              size="tiny"
              variant="plain"
              onClick={() =>
                files.forEach(file => onStage(file, label !== 'Staged'))
              }
            >
              {action}
            </Button>
          ) : null}
        </div>
        {files.map(change => (
          <Row
            key={change.path}
            change={change}
            open={open.includes(change.path)}
            staged={isStaged(change)}
            onStage={() => onStage(change, !isStaged(change))}
            onToggle={() => toggle(change.path)}
          />
        ))}
      </section>
    )

  return (
    <div className="ca-view">
      <header className="ca-view__head">
        <div className="ca-view__titles">
          <h1 className="ca-view__title">Changes</h1>
          <p className="ca-view__subtitle">
            {changes.length} {changes.length === 1 ? 'file' : 'files'} · +
            {added} -{removed} · on {branch}
          </p>
        </div>
        <Button
          disabled={staged.length === 0}
          variant="filled"
          onClick={onCommit}
        >
          Commit {staged.length} {staged.length === 1 ? 'file' : 'files'}…
        </Button>
      </header>

      <div className="ca-view__body" data-inset="true">
        {changes.length === 0 ? (
          <EmptyState title="The working tree is clean — everything is committed." />
        ) : (
          <>
            {section('Staged', staged, 'Unstage all')}
            {section('Not staged', rest, 'Stage all')}
          </>
        )}
      </div>
    </div>
  )
}
