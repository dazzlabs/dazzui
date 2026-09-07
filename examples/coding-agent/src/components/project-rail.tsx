import {
  Icon,
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
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
 * The second navigation column: what is inside the selected project. It is
 * the same `Sidebar` as the projects column at the rail's own width, and it
 * never collapses — a column that comes and goes moves the content under
 * the pointer.
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
    <Sidebar collapsible="none" className="ca-rail">
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  active={pane === 'issues'}
                  // A project with no remote has no issues to list, and a row
                  // that opens an empty state is worse than one that says it
                  // is off.
                  disabled={!connected}
                  icon={<Icon icon={IssueGlyph} size={14} />}
                  size="small"
                  title={connected ? undefined : 'No remote connected'}
                  onClick={() => onOpenPane('issues')}
                >
                  <span className="ca-truncate">Issues</span>
                  {connected ? (
                    <span className="ca-rail__count">
                      {openIssues(project).length}
                    </span>
                  ) : null}
                </SidebarMenuButton>
              </SidebarMenuItem>

              <SidebarMenuItem>
                <SidebarMenuButton
                  active={pane === 'pulls'}
                  disabled={!connected}
                  icon={<Icon icon={BranchGlyph} size={14} />}
                  size="small"
                  title={connected ? 'Pull requests' : 'No remote connected'}
                  onClick={() => onOpenPane('pulls')}
                >
                  {/* The column is 150px by token and "Pull requests" does
                      not fit beside a glyph and a count. The pane's own
                      heading says it in full; here the tooltip does. */}
                  <span className="ca-truncate">Pulls</span>
                  {connected ? (
                    <span className="ca-rail__count">
                      {openPulls(project).length}
                    </span>
                  ) : null}
                </SidebarMenuButton>
              </SidebarMenuItem>

              <SidebarMenuItem>
                <SidebarMenuButton
                  active={pane === 'changes'}
                  icon={<Icon icon={DiffGlyph} size={14} />}
                  size="small"
                  onClick={() => onOpenPane('changes')}
                >
                  <span className="ca-truncate">Changes</span>
                  <span className="ca-rail__count">{changed}</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarGroup>
          <SidebarGroupLabel>Sessions</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {project.sessions.map(session => (
                <SidebarMenuItem key={session.id}>
                  <SidebarMenuButton
                    active={view.kind === 'session' && view.id === session.id}
                    icon={<Icon icon={ChatGlyph} size={14} />}
                    size="small"
                    title={`${session.title} — ${session.when}`}
                    onClick={() => onOpenSession(session.id)}
                  >
                    <span>{session.title}</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}

              <SidebarMenuItem className="ca-rail__new">
                <SidebarMenuButton
                  size="small"
                  variant="plain"
                  onClick={onNewSession}
                >
                  <Icon icon={PlusGlyph} size={14} />
                  <span>New session</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  )
}
