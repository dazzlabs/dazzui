import {
  Badge,
  Button,
  EmptyState,
  Icon,
  PillTabs,
  Spinner,
  Table,
  TableCell,
  TableHead,
  TableRow,
} from '@dazzlabs/dazzui-react'
import { useState } from 'react'

import { BranchGlyph, CheckGlyph } from '../data/icons'
import type { Checks, Project, Pull, PullState } from '../lib/project'

type Filter = 'active' | 'merged'

/**
 * A pull request's checks are the one thing on the row a reader is actually
 * scanning for, so they get the colour and everything else stays grey.
 */
function ChecksBadge({ checks }: { checks: Checks }) {
  if (checks === 'running') {
    return (
      <span className="ca-pulls__checks">
        <Spinner size="small" />
        checks running
      </span>
    )
  }

  return (
    <Badge size="small" tint={checks === 'passing' ? 'success' : 'danger'}>
      {checks === 'passing' ? <Icon icon={CheckGlyph} size={11} /> : null}
      {checks === 'passing' ? 'checks passed' : 'checks failed'}
    </Badge>
  )
}

const STATE_TINT: Record<PullState, 'info' | 'neutral' | 'primary'> = {
  open: 'info',
  draft: 'neutral',
  merged: 'primary',
}

export interface PullsViewProps {
  project: Project
  /** Whether the project has a provider to list from. */
  connected: boolean
  selected: number | null
  /** Binds the project to a provider, from the empty state's own button. */
  onConnect: () => void
  onSelect: (pull: Pull) => void
}

export function PullsView({
  project,
  connected,
  selected,
  onConnect,
  onSelect,
}: PullsViewProps) {
  const [filter, setFilter] = useState<Filter>('active')

  if (!connected) {
    return (
      <div className="ca-view">
        <header className="ca-view__head">
          <div className="ca-view__titles">
            <h1 className="ca-view__title">Pull requests</h1>
            <p className="ca-view__subtitle">{project.path}</p>
          </div>
        </header>
        <div className="ca-view__body" data-inset="true">
          <EmptyState
            action={
              <Button variant="filled" onClick={onConnect}>
                Connect GitHub
              </Button>
            }
            title="This project has no remote, so there is nothing to list."
          />
        </div>
      </div>
    )
  }

  const active = project.pulls.filter(pull => pull.state !== 'merged')
  const merged = project.pulls.filter(pull => pull.state === 'merged')
  const shown = filter === 'active' ? active : merged

  return (
    <div className="ca-view">
      <header className="ca-view__head">
        <div className="ca-view__titles">
          <h1 className="ca-view__title">Pull requests</h1>
          <p className="ca-view__subtitle">{project.name}</p>
        </div>
        <PillTabs
          items={[
            { value: 'active' as const, label: 'Active', count: active.length },
            { value: 'merged' as const, label: 'Merged', count: merged.length },
          ]}
          value={filter}
          onValueChange={setFilter}
        />
      </header>

      <div className="ca-view__body">
        <Table className="ca-list">
          <TableHead>
            <TableCell head className="ca-list__lead">
              #
            </TableCell>
            <TableCell head className="ca-list__main">
              Pull request
            </TableCell>
            <TableCell head className="ca-list__trail">
              Opened
            </TableCell>
          </TableHead>

          {shown.map(pull => (
            <TableRow
              key={pull.number}
              interactive
              active={pull.number === selected}
              onClick={() => onSelect(pull)}
            >
              <TableCell className="ca-list__lead">{pull.number}</TableCell>
              <TableCell className="ca-list__main">
                <span className="ca-list__title">{pull.title}</span>
                <span className="ca-list__meta">
                  <Badge size="small" tint={STATE_TINT[pull.state]}>
                    {pull.state}
                  </Badge>
                  {pull.state === 'merged' ? null : (
                    <ChecksBadge checks={pull.checks} />
                  )}
                  <span className="ca-list__branch">
                    <Icon icon={BranchGlyph} size={11} />
                    {pull.branch}
                  </span>
                  <span className="ca-list__author">
                    {pull.files} files · +{pull.added} -{pull.removed}
                  </span>
                </span>
              </TableCell>
              <TableCell className="ca-list__trail">{pull.when}</TableCell>
            </TableRow>
          ))}
        </Table>

        {shown.length === 0 ? (
          <div className="ca-view__body" data-inset="true">
            <EmptyState title="Nothing open. Every branch is either merged or unstarted." />
          </div>
        ) : null}
      </div>
    </div>
  )
}
