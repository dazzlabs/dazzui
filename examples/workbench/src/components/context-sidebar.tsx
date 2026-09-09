import {
  Badge,
  Icon,
  IconButton,
  type IconGlyph,
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupAction,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuBadge,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarRail,
  SidebarSeparator,
} from '@dazzlabs/dazzui-react'
import type { ReactNode } from 'react'

import {
  BranchGlyph,
  ChatGlyph,
  CheckCircleGlyph,
  CloudGlyph,
  DiffGlyph,
  DocGlyph,
  FlaskGlyph,
  GearGlyph,
  GithubGlyph,
  IssueGlyph,
  PlusGlyph,
  PullGlyph,
  RocketShipGlyph,
  TrayGlyph,
} from '../data/icons'
import {
  type Attention,
  checksStatus,
  type GithubFace,
  INBOX_FILTERS,
  type InboxFilter,
  needsYou,
  openIssues,
  openPulls,
  type Project,
  type Session,
  SESSION_GROUPS,
  type Task,
  type View,
  type WorkingChange,
} from '../lib/project'
import { RunLight } from './pull-view'
import { STATUS_LABEL, StatusDot } from './status-dot'

/** The seven rows, in the order ⌘1–7 binds them. */
const NAV: { view: View; label: string; glyph: IconGlyph }[] = [
  { view: 'chat', label: 'Chat', glyph: ChatGlyph },
  { view: 'inbox', label: 'Inbox', glyph: TrayGlyph },
  { view: 'review', label: 'Review changes', glyph: DiffGlyph },
  { view: 'git', label: 'Git', glyph: BranchGlyph },
  { view: 'tests', label: 'Tests', glyph: FlaskGlyph },
  { view: 'github', label: 'GitHub', glyph: GithubGlyph },
  { view: 'release', label: 'Release', glyph: RocketShipGlyph },
]

function SessionRow({
  session,
  active,
  onClick,
}: {
  session: Session
  active: boolean
  onClick: () => void
}) {
  return (
    <SidebarMenuItem>
      <SidebarMenuButton
        active={active}
        className="wb-session"
        size="large"
        variant="tinted"
        title={`${session.title} — ${session.when}`}
        onClick={onClick}
      >
        <span className="wb-session__body">
          <span className="wb-session__line">
            <StatusDot size="small" status={session.status} />
            <span className="wb-truncate">{session.title}</span>
          </span>
          <span
            data-tint={session.status === 'waiting' ? 'warning' : undefined}
            className="wb-session__meta"
          >
            {session.status === 'running'
              ? `Running · ${session.tools} tool calls`
              : session.status === 'waiting'
                ? `Waiting · ${session.blockedOn ?? 'approval'}`
                : session.status === 'watching' && session.pull
                  ? `Watching the checks on #${session.pull}`
                  : `${session.when} · ${STATUS_LABEL[session.status]}`}
          </span>
          {session.status === 'watching' ? null : (
            <span className="wb-session__where">
              {session.worktree ?? 'main checkout'}
            </span>
          )}
        </span>
      </SidebarMenuButton>
    </SidebarMenuItem>
  )
}

/** A row that is a filter rather than a page: it names a slice of a list. */
function FilterRow({
  label,
  count,
  active,
  onClick,
}: {
  label: string
  count?: number
  active?: boolean
  onClick?: () => void
}) {
  return (
    <SidebarMenuItem>
      <SidebarMenuButton
        active={active}
        size="small"
        variant="recessed"
        onClick={onClick}
      >
        <span className="wb-truncate">
          {label}
          {count === undefined ? '' : ` · ${count}`}
        </span>
      </SidebarMenuButton>
    </SidebarMenuItem>
  )
}

export interface ContextSidebarProps {
  project: Project
  view: View
  /** The sessions with their live states — the open one's is the player's. */
  sessions: readonly Session[]
  /** Every open project's sessions, which the inbox's counts are of. */
  tasks: readonly Task[]
  /** What the window has noticed across the open projects. */
  inbox: readonly Attention[]
  inboxFilter: InboxFilter
  onInboxFilter: (filter: InboxFilter) => void
  openSession: string
  /** The working tree as it stands, the run's own writes included. */
  changes: readonly WorkingChange[]
  connected: boolean
  /** Which face of the GitHub page is up: the issues or the pull requests. */
  face: GithubFace
  onFace: (face: GithubFace) => void
  /** Which slice of the issue list is shown. */
  issueFilter: string
  onIssueFilter: (filter: string) => void
  /** True while the settings sheet is over the window. */
  settingsOpen: boolean
  onOpenView: (view: View) => void
  onOpenSession: (id: string) => void
  onNewSession: () => void
  onOpenSettings: () => void
}

/**
 * The sidebar: the seven pages, and — under a rule — whatever the open page
 * is *made of*. It does not name the project: the titlebar's tab already
 * does, and the status bar carries the branch.
 *
 * That lower half is the whole idea. A window with seven pages and one static
 * column would need a second column for every list; here the column is the
 * page's own index, so Chat lists sessions, Inbox lists what is waiting on
 * you, Git lists branches, Tests lists suites, GitHub lists its two faces and
 * what each holds, and Release lists the configs and the channels. One
 * column, seven jobs, and the reader never loses the place they navigate
 * from. It hides on ⌘B, the way every editor's does, because a review or a
 * long thread is sometimes worth the whole window.
 *
 * The sessions are grouped by state rather than by age, with the group that
 * needs a person marked: a column of parallel sessions is read for "which of
 * these is waiting on me", and a flat list makes the reader work that out
 * from the dots. Nothing in the nav is ever disabled — a page with nothing
 * in it says why, which a greyed row cannot.
 */
export function ContextSidebar({
  project,
  view,
  sessions,
  tasks,
  inbox,
  inboxFilter,
  onInboxFilter,
  openSession,
  changes,
  connected,
  face,
  onFace,
  issueFilter,
  onIssueFilter,
  settingsOpen,
  onOpenView,
  onOpenSession,
  onNewSession,
  onOpenSettings,
}: ContextSidebarProps) {
  const issues = openIssues(project)
  const pulls = openPulls(project)
  const failing = project.tests?.failed ?? 0
  /** Every session across the open projects that is asking for a person. */
  const asking = tasks.filter(task => needsYou(task.session.status))

  /** What each nav row carries on its trailing edge, where anything does. */
  const badge = (target: View): ReactNode => {
    switch (target) {
      case 'review':
        return changes.length > 0 ? (
          <Badge size="small">{changes.length}</Badge>
        ) : null
      case 'tests':
        return failing > 0 ? (
          <Badge size="small" tint="danger">
            {failing} failed
          </Badge>
        ) : null
      case 'github':
        return connected ? issues.length + pulls.length : null
      case 'release':
        return project.release ? (
          <Badge size="small">{project.release.version}</Badge>
        ) : null
      case 'inbox': {
        const attention = asking.length + inbox.length
        return attention > 0 ? (
          <Badge size="small" tint="warning">
            {attention}
          </Badge>
        ) : null
      }
      default:
        return null
    }
  }

  const context = (() => {
    switch (view) {
      case 'chat':
        return (
          <SidebarGroup>
            <SidebarGroupLabel>Sessions</SidebarGroupLabel>
            <SidebarGroupAction
              label="New session"
              size="tiny"
              variant="plain"
              onClick={onNewSession}
            >
              <Icon icon={PlusGlyph} size={14} />
            </SidebarGroupAction>
            <SidebarGroupContent>
              <SidebarMenu>
                {SESSION_GROUPS.map(group => {
                  const rows = sessions.filter(
                    session => session.status === group.status
                  )
                  if (rows.length === 0) return null
                  return (
                    <li key={group.status} className="wb-sidebar__group">
                      <span
                        data-tint={
                          needsYou(group.status) ? 'warning' : undefined
                        }
                        className="wb-sidebar__sub"
                      >
                        {group.label} · {rows.length}
                      </span>
                      <ul className="wb-sidebar__rows">
                        {rows.map(session => (
                          <SessionRow
                            key={session.id}
                            active={session.id === openSession}
                            session={session}
                            onClick={() => onOpenSession(session.id)}
                          />
                        ))}
                      </ul>
                    </li>
                  )
                })}
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        )

      case 'inbox':
        return (
          <SidebarGroup>
            <SidebarGroupLabel>Waiting on you</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                {INBOX_FILTERS.map(entry => (
                  <FilterRow
                    key={entry.id}
                    active={inboxFilter === entry.id}
                    count={
                      entry.id === 'all'
                        ? asking.length + inbox.length
                        : entry.id === 'notices'
                          ? inbox.length
                          : asking.filter(
                              task => task.session.status === entry.id
                            ).length
                    }
                    label={entry.label}
                    onClick={() => onInboxFilter(entry.id)}
                  />
                ))}
              </SidebarMenu>
              <span className="wb-sidebar__note">
                Every open project, in one list.
              </span>
            </SidebarGroupContent>
          </SidebarGroup>
        )

      case 'review':
        return (
          <SidebarGroup>
            <SidebarGroupLabel>
              Changed files · {changes.length}
            </SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                {changes.map(change => (
                  <SidebarMenuItem key={change.path}>
                    <SidebarMenuButton
                      icon={<Icon icon={DocGlyph} size={14} />}
                      size="small"
                      title={change.path}
                      variant="recessed"
                    >
                      <span className="wb-truncate wb-mono">
                        {change.path.split('/').pop()}
                      </span>
                      <span className="wb-counts">
                        <span data-tint="success" className="wb-count">
                          +{change.added}
                        </span>
                        <span data-tint="danger" className="wb-count">
                          −{change.removed}
                        </span>
                      </span>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                ))}
              </SidebarMenu>
              <span className="wb-sidebar__note">
                J / K move between files · A accepts · X reverts · C comments
              </span>
            </SidebarGroupContent>
          </SidebarGroup>
        )

      case 'git':
        return (
          <>
            <SidebarGroup>
              <SidebarGroupLabel>Branches</SidebarGroupLabel>
              <SidebarGroupContent>
                <SidebarMenu>
                  {project.branches.map(branch => (
                    <SidebarMenuItem key={branch.name}>
                      <SidebarMenuButton
                        active={branch.name === project.branch}
                        icon={<Icon icon={BranchGlyph} size={14} />}
                        size="small"
                        variant="tinted"
                      >
                        <span className="wb-truncate wb-mono">
                          {branch.name}
                        </span>
                      </SidebarMenuButton>
                      <SidebarMenuBadge>{branch.meta}</SidebarMenuBadge>
                    </SidebarMenuItem>
                  ))}
                </SidebarMenu>
              </SidebarGroupContent>
            </SidebarGroup>
            <SidebarGroup>
              <SidebarGroupLabel>Remote</SidebarGroupLabel>
              <SidebarGroupContent>
                <SidebarMenu>
                  {project.provider ? (
                    <SidebarMenuItem>
                      <SidebarMenuButton
                        icon={<Icon icon={CloudGlyph} size={14} />}
                        size="small"
                      >
                        <span className="wb-truncate">origin · github.com</span>
                      </SidebarMenuButton>
                      <SidebarMenuBadge>
                        {project.behind > 0 ? `↓${project.behind}` : 'synced'}
                      </SidebarMenuBadge>
                    </SidebarMenuItem>
                  ) : (
                    <span className="wb-sidebar__note">
                      No remote. Commits stay on this machine.
                    </span>
                  )}
                </SidebarMenu>
              </SidebarGroupContent>
            </SidebarGroup>
          </>
        )

      case 'tests':
        return (
          <SidebarGroup>
            <SidebarGroupLabel>Suites</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                {(project.tests?.suites ?? []).map(suite => (
                  <SidebarMenuItem key={suite.path}>
                    <SidebarMenuButton
                      size="small"
                      title={suite.path}
                      variant="recessed"
                    >
                      <StatusDot
                        size="small"
                        status={suite.failed > 0 ? 'waiting' : 'done'}
                      />
                      <span className="wb-truncate wb-mono">{suite.path}</span>
                    </SidebarMenuButton>
                    <SidebarMenuBadge>
                      {suite.failed > 0
                        ? `${suite.failed} failed`
                        : `${suite.passed} passed`}
                    </SidebarMenuBadge>
                  </SidebarMenuItem>
                ))}
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        )

      case 'github':
        return (
          <>
            {/* The three faces, as rows the way Tiller's GitHub face lists
                them: the two things, and the light over them. The page's own
                list column holds the rows; this column only says which. */}
            <SidebarGroup>
              <SidebarGroupLabel>GitHub · {project.name}</SidebarGroupLabel>
              <SidebarGroupContent>
                <SidebarMenu>
                  <SidebarMenuItem>
                    <SidebarMenuButton
                      active={face === 'issues'}
                      icon={<Icon icon={IssueGlyph} size={14} />}
                      size="small"
                      variant="tinted"
                      onClick={() => onFace('issues')}
                    >
                      <span className="wb-truncate">Issues</span>
                    </SidebarMenuButton>
                    {connected ? (
                      <SidebarMenuBadge>{issues.length}</SidebarMenuBadge>
                    ) : null}
                  </SidebarMenuItem>
                  <SidebarMenuItem>
                    <SidebarMenuButton
                      active={face === 'pulls'}
                      icon={<Icon icon={PullGlyph} size={14} />}
                      size="small"
                      variant="tinted"
                      onClick={() => onFace('pulls')}
                    >
                      <span className="wb-truncate">Pull requests</span>
                    </SidebarMenuButton>
                    {connected ? (
                      <SidebarMenuBadge>{pulls.length}</SidebarMenuBadge>
                    ) : null}
                  </SidebarMenuItem>
                  <SidebarMenuItem>
                    <SidebarMenuButton
                      active={face === 'checks'}
                      icon={<Icon icon={CheckCircleGlyph} size={14} />}
                      size="small"
                      variant="tinted"
                      onClick={() => onFace('checks')}
                    >
                      <span className="wb-truncate">Checks</span>
                    </SidebarMenuButton>
                    {connected ? (
                      <SidebarMenuBadge>
                        <RunLight
                          status={
                            pulls.some(pull => checksStatus(pull) === 'failed')
                              ? 'failed'
                              : pulls.some(
                                    pull => checksStatus(pull) === 'running'
                                  )
                                ? 'running'
                                : 'passed'
                          }
                        />
                      </SidebarMenuBadge>
                    ) : null}
                  </SidebarMenuItem>
                </SidebarMenu>
              </SidebarGroupContent>
            </SidebarGroup>

            {connected && face === 'issues' ? (
              <SidebarGroup>
                <SidebarGroupLabel>Filters</SidebarGroupLabel>
                <SidebarGroupContent>
                  <SidebarMenu>
                    <FilterRow
                      active={issueFilter === 'open'}
                      count={issues.length}
                      label="Open issues"
                      onClick={() => onIssueFilter('open')}
                    />
                    <FilterRow
                      active={issueFilter === 'mine'}
                      count={
                        issues.filter(issue => issue.author === 'mia').length
                      }
                      label="Assigned to me"
                      onClick={() => onIssueFilter('mine')}
                    />
                    <FilterRow
                      active={issueFilter === 'agent'}
                      count={issues.filter(issue => issue.agentReady).length}
                      label="Agent can take"
                      onClick={() => onIssueFilter('agent')}
                    />
                    <FilterRow
                      active={issueFilter === 'closed'}
                      count={
                        project.issues.filter(issue => issue.state === 'closed')
                          .length
                      }
                      label="Closed"
                      onClick={() => onIssueFilter('closed')}
                    />
                  </SidebarMenu>
                </SidebarGroupContent>
              </SidebarGroup>
            ) : null}
          </>
        )

      case 'release':
        return project.release ? (
          <>
            <SidebarGroup>
              <SidebarGroupLabel>Release configs</SidebarGroupLabel>
              <SidebarGroupContent>
                <SidebarMenu>
                  {project.release.configs.map(config => (
                    <SidebarMenuItem key={config.name}>
                      <SidebarMenuButton
                        active={config.name === project.release?.active}
                        className="wb-session"
                        size="large"
                        variant="tinted"
                      >
                        <span className="wb-session__body">
                          <span className="wb-truncate wb-mono">
                            {config.name}
                          </span>
                          <span className="wb-session__meta">
                            {config.note}
                          </span>
                        </span>
                      </SidebarMenuButton>
                    </SidebarMenuItem>
                  ))}
                </SidebarMenu>
                <span className="wb-sidebar__note wb-mono">
                  {project.release.configFile}
                </span>
              </SidebarGroupContent>
            </SidebarGroup>
            <SidebarGroup>
              <SidebarGroupLabel>Channels</SidebarGroupLabel>
              <SidebarGroupContent>
                <SidebarMenu>
                  {project.release.channels.map(channel => (
                    <SidebarMenuItem key={channel.id}>
                      <SidebarMenuButton size="small">
                        <span
                          data-tone={channel.tone}
                          className="wb-channel-dot"
                        />
                        <span className="wb-truncate">{channel.name}</span>
                      </SidebarMenuButton>
                      <SidebarMenuBadge>{channel.state}</SidebarMenuBadge>
                    </SidebarMenuItem>
                  ))}
                </SidebarMenu>
              </SidebarGroupContent>
            </SidebarGroup>
          </>
        ) : null

      default:
        return null
    }
  })()

  return (
    <Sidebar className="wb-sidebar">
      <SidebarContent className="wb-sidebar__content">
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              {NAV.map(row => (
                <SidebarMenuItem key={row.view}>
                  <SidebarMenuButton
                    active={view === row.view}
                    icon={<Icon icon={row.glyph} size={15} />}
                    onClick={() => onOpenView(row.view)}
                  >
                    <span className="wb-truncate">{row.label}</span>
                  </SidebarMenuButton>
                  {badge(row.view) ? (
                    <SidebarMenuBadge>{badge(row.view)}</SidebarMenuBadge>
                  ) : null}
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarSeparator />

        {context}
      </SidebarContent>

      {/* The foot is who is at the window, and the one thing that is about
          the window rather than the project: its settings. The agent's own
          state is on the sessions above and the status bar below, so
          nothing here repeats it. */}
      <SidebarFooter className="wb-sidebar__foot">
        <span aria-hidden data-tint="primary" className="wb-foot__avatar">
          MC
        </span>
        <span className="wb-truncate wb-foot__name">Mia Chen</span>
        <span className="wb-foot__gap" />
        <IconButton
          active={settingsOpen}
          label="Settings · ⌘,"
          size="small"
          tint="neutral"
          variant="plain"
          onClick={onOpenSettings}
        >
          <Icon icon={GearGlyph} size={17} />
        </IconButton>
      </SidebarFooter>

      {/* The edge is the way back: ⌘B hides the column, and the strip along
          its edge — or the trigger in the titlebar — brings it back. */}
      <SidebarRail />
    </Sidebar>
  )
}
