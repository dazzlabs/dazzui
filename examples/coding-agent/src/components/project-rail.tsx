import {
  Icon,
  NavAction,
  NavItem,
  Rail,
  RailGroup,
} from '@dazzlabs/dazzui-react'

import {
  BranchGlyph,
  ChatGlyph,
  DiffGlyph,
  IssueGlyph,
  PlusGlyph,
} from '../data/icons'
import {
  openIssues,
  openPulls,
  type Project,
  type ProjectPane,
  type View,
} from '../lib/project'

export interface ProjectRailProps {
  project: Project
  /** How many files the working tree holds, the run's own writes included. */
  changed: number
  /** Whether the project has a provider to list issues and pulls from. */
  connected: boolean
  view: View
  onOpenPane: (pane: ProjectPane) => void
  onOpenSession: (id: string) => void
  onNewSession: () => void
}

/**
 * The second navigation column: what is inside the selected project.
 *
 * The first run goes unlabelled — it is what the pane is already called — and
 * carries the project's own views. The sessions get the label, because a gap
 * alone would leave the reader guessing what the rows below have in common.
 */
export function ProjectRail({
  project,
  changed,
  connected,
  view,
  onOpenPane,
  onOpenSession,
  onNewSession,
}: ProjectRailProps) {
  const pane = view.kind === 'pane' ? view.pane : null

  return (
    <Rail className="ca-rail">
      <RailGroup>
        <NavItem
          active={pane === 'issues'}
          // A project with no remote has no issues to list, and a row that
          // opens an empty state is worse than a row that says it is off.
          disabled={!connected}
          icon={<Icon icon={IssueGlyph} size={14} />}
          size="small"
          title={connected ? undefined : 'No remote connected'}
          onClick={() => onOpenPane('issues')}
        >
          <span className="ca-truncate">Issues</span>
          {connected ? (
            <span className="ca-rail__count">{openIssues(project).length}</span>
          ) : null}
        </NavItem>

        <NavItem
          active={pane === 'pulls'}
          disabled={!connected}
          icon={<Icon icon={BranchGlyph} size={14} />}
          size="small"
          title={connected ? 'Pull requests' : 'No remote connected'}
          onClick={() => onOpenPane('pulls')}
        >
          {/* The column is 150px by token and "Pull requests" does not fit
              beside a glyph and a count. The pane's own heading says it in
              full; here the tooltip does. */}
          <span className="ca-truncate">Pulls</span>
          {connected ? (
            <span className="ca-rail__count">{openPulls(project).length}</span>
          ) : null}
        </NavItem>

        <NavItem
          active={pane === 'changes'}
          icon={<Icon icon={DiffGlyph} size={14} />}
          size="small"
          onClick={() => onOpenPane('changes')}
        >
          <span className="ca-truncate">Changes</span>
          <span className="ca-rail__count">{changed}</span>
        </NavItem>
      </RailGroup>

      <RailGroup label="Sessions">
        {project.sessions.map(session => (
          <NavItem
            key={session.id}
            active={view.kind === 'session' && view.id === session.id}
            icon={<Icon icon={ChatGlyph} size={14} />}
            size="small"
            title={`${session.title} — ${session.when}`}
            onClick={() => onOpenSession(session.id)}
          >
            <span className="ca-truncate">{session.title}</span>
          </NavItem>
        ))}

        <NavAction className="ca-rail__new" size="small" onClick={onNewSession}>
          <Icon icon={PlusGlyph} size={14} />
          New session
        </NavAction>
      </RailGroup>
    </Rail>
  )
}
