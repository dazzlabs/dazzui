import {
  Badge,
  Button,
  EmptyState,
  PillTabs,
  Table,
  TableCell,
  TableHead,
  TableRow,
} from '@dazzlabs/dazzui-react'
import { useState } from 'react'

import type { Issue, IssueState, Project } from '../lib/project'

type Filter = IssueState | 'all'

export interface IssuesViewProps {
  project: Project
  /** Whether the project has a provider to list from. */
  connected: boolean
  selected: number | null
  /** Binds the project to a provider, from the empty state's own button. */
  onConnect: () => void
  onSelect: (issue: Issue) => void
}

/** The project's issues, as the list you pick the next session's subject from. */
export function IssuesView({
  project,
  connected,
  selected,
  onConnect,
  onSelect,
}: IssuesViewProps) {
  const [filter, setFilter] = useState<Filter>('open')

  if (!connected) {
    return (
      <div className="ca-view">
        <header className="ca-view__head">
          <div className="ca-view__titles">
            <h1 className="ca-view__title">Issues</h1>
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

  const open = project.issues.filter(issue => issue.state === 'open')
  const closed = project.issues.filter(issue => issue.state === 'closed')
  const shown =
    filter === 'all' ? project.issues : filter === 'open' ? open : closed

  return (
    <div className="ca-view">
      <header className="ca-view__head">
        <div className="ca-view__titles">
          <h1 className="ca-view__title">Issues</h1>
          <p className="ca-view__subtitle">{project.name}</p>
        </div>
        <PillTabs
          items={[
            { value: 'open' as const, label: 'Open', count: open.length },
            { value: 'closed' as const, label: 'Closed', count: closed.length },
            { value: 'all' as const, label: 'All' },
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
              Issue
            </TableCell>
            <TableCell head className="ca-list__trail">
              Opened
            </TableCell>
          </TableHead>

          {shown.map(issue => (
            <TableRow
              key={issue.number}
              interactive
              active={issue.number === selected}
              onClick={() => onSelect(issue)}
            >
              <TableCell className="ca-list__lead">{issue.number}</TableCell>
              <TableCell className="ca-list__main">
                <span className="ca-list__title">{issue.title}</span>
                <span className="ca-list__meta">
                  {issue.state === 'closed' ? (
                    <Badge size="small" tint="neutral">
                      closed
                    </Badge>
                  ) : null}
                  {issue.labels.map(label => (
                    <Badge
                      key={label}
                      size="small"
                      tint="info"
                      variant="outlined"
                    >
                      {label}
                    </Badge>
                  ))}
                  <span className="ca-list__author">
                    {issue.author} · {issue.comments} comments
                  </span>
                </span>
              </TableCell>
              <TableCell className="ca-list__trail">{issue.when}</TableCell>
            </TableRow>
          ))}
        </Table>

        {shown.length === 0 ? (
          <div className="ca-view__body" data-inset="true">
            <EmptyState title="Nothing here — every issue in this filter is dealt with." />
          </div>
        ) : null}
      </div>
    </div>
  )
}
