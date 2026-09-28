import { Button, Card, Icon, KeyCap } from '@dazzlabs/dazzui-react'

import { CloudGlyph, FolderOpenGlyph } from '../data/icons'
import { needsYou, type Project, projectStatus } from '../lib/project'
import { StatusDot } from './status-dot'

export interface WelcomeViewProps {
  projects: readonly Project[]
  /** Which are already open as tabs, so the card says "open" rather than a time. */
  open: readonly string[]
  onOpen: (id: string) => void
}

/**
 * What the window shows with no project in front of it.
 *
 * It is the only page drawn without the sidebar, because the sidebar is a
 * project's own furniture and there is no project yet. Everything on it is a
 * way in: the two buttons that make a new one, and the recents that reopen
 * the ones this window already knows.
 */
export function WelcomeView({ projects, open, onOpen }: WelcomeViewProps) {
  return (
    <div className="wb-welcome">
      <h1 className="wb-welcome__title">Start the day</h1>
      <p className="wb-welcome__lede">
        Open a project, or hand a GitHub issue straight to the agent.
      </p>

      <span className="wb-welcome__actions">
        <Button size="large" variant="filled">
          <Icon icon={FolderOpenGlyph} size={16} />
          Open a local folder
        </Button>
        <Button size="large">
          <Icon icon={CloudGlyph} size={16} />
          Clone from GitHub
        </Button>
      </span>

      <span className="wb-label wb-welcome__label">Recent projects</span>

      <div className="wb-grid" data-columns="3">
        {projects.map(project => {
          const running = project.sessions.filter(
            session => session.status === 'running'
          ).length
          const asking = project.sessions.filter(session =>
            needsYou(session.status)
          ).length

          return (
            <Card
              key={project.id}
              className="wb-recent"
              size="large"
              variant="raised"
              onClick={() => onOpen(project.id)}
            >
              <span className="wb-card__title">
                {project.name.split('/').pop()}
              </span>
              <span className="wb-mono wb-card__quiet">{project.path}</span>

              <span className="wb-recent__tags">
                {running > 0 ? (
                  <span className="wb-recent__tag">
                    <StatusDot size="small" status="running" />
                    {running} running
                  </span>
                ) : null}
                {asking > 0 ? (
                  <span data-tint="warning" className="wb-recent__tag">
                    <StatusDot size="small" status={projectStatus(project)} />
                    {asking} {asking === 1 ? 'needs' : 'need'} you
                  </span>
                ) : null}
                <span className="wb-recent__tag wb-mono">{project.branch}</span>
              </span>

              <span className="wb-card__quiet">
                {open.includes(project.id) ? 'Open in a tab' : project.seen}
              </span>
            </Card>
          )
        })}
      </div>

      <span className="wb-welcome__keys">
        <span>
          <KeyCap size="small" variant="key">
            ⌘K
          </KeyCap>{' '}
          Command palette
        </span>
        <span>
          <KeyCap size="small" variant="key">
            ⌘P
          </KeyCap>{' '}
          Switch project
        </span>
        <span>
          <KeyCap size="small" variant="key">
            ⌘N
          </KeyCap>{' '}
          New session
        </span>
      </span>
    </div>
  )
}
