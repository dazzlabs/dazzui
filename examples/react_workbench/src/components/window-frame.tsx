import {
  Icon,
  IconButton,
  KeyCap,
  PillTabs,
  SidebarProvider,
  SidebarTrigger,
} from '@dazzlabs/dazzui-react'
import type { ReactNode } from 'react'

import { CompassGlyph, PlusGlyph, SearchGlyph } from '../data/icons'
import { type Project, projectStatus, shortName } from '../lib/project'
import { StatusDot } from './status-dot'

export interface WindowFrameProps {
  /** The projects that are open, in tab order. */
  projects: readonly Project[]
  /** The one the window is showing, or null on the welcome screen. */
  current: Project | null
  onSelect: (id: string) => void
  /** Opens the welcome screen, which is where a project is opened from. */
  onOpen: () => void
  onPalette: () => void
  /** The full-height leading column, absent while Welcome has the window. */
  sidebar?: ReactNode
  /** The line across the foot, absent while Welcome has the window. */
  statusBar?: ReactNode
  children: ReactNode
}

/**
 * The window: a titlebar across the top, the sidebar and pane under it, and
 * a status bar across the foot.
 *
 * The titlebar is where the *projects* are, as tabs — this window runs
 * several at once, each with its own sessions and worktrees, and a tab is
 * what makes that visible without a column spent on it. Each tab carries one
 * dot: the most demanding thing happening inside that project, so a project
 * that needs a person says so from the other side of the window.
 *
 * Nothing here draws a window. The corner, the shadow and the controls belong
 * to the real one — `deno desktop` opens it with the titlebar hidden — and a
 * browser tab has no chrome to imitate.
 */
export function WindowFrame({
  projects,
  current,
  onSelect,
  onOpen,
  onPalette,
  sidebar,
  statusBar,
  children,
}: WindowFrameProps) {
  return (
    <SidebarProvider className="wb-window">
      <header className="wb-titlebar">
        {/* The provider wraps the bar as well as the body, so the trigger
            up here reaches the column below it. */}
        {sidebar ? (
          <SidebarTrigger
            label="Hide or show the sidebar · ⌘B"
            size="small"
            tint="neutral"
            variant="plain"
          />
        ) : null}
        <Icon icon={CompassGlyph} size={17} className="wb-titlebar__mark" />
        <span className="wb-titlebar__product">Workbench</span>

        <PillTabs
          aria-label="Open projects"
          className="wb-tabs"
          items={projects.map(project => ({
            value: project.id,
            label: (
              <>
                <StatusDot status={projectStatus(project)} size="small" />
                {shortName(project.name)}
              </>
            ),
          }))}
          value={current?.id ?? ''}
          onValueChange={onSelect}
        />

        <IconButton
          label="Open a project · ⌘P"
          size="small"
          tint="neutral"
          variant="plain"
          onClick={onOpen}
        >
          <Icon icon={PlusGlyph} size={15} />
        </IconButton>

        <span className="wb-titlebar__gap" />

        <button className="wb-palette" type="button" onClick={onPalette}>
          <Icon icon={SearchGlyph} size={13} />
          Command palette
          <KeyCap size="small" variant="hint">
            ⌘K
          </KeyCap>
        </button>
      </header>

      <div className="wb-body">
        {sidebar}
        <div className="wb-pane">{children}</div>
      </div>

      {statusBar}
    </SidebarProvider>
  )
}
