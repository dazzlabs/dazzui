import {
  Badge,
  Button,
  Icon,
  Menu,
  SidebarProvider,
  SidebarTrigger,
} from '@dazzlabs/dazzui-react'
import type { ReactNode } from 'react'

import { BranchGlyph, SidebarGlyph } from '../data/icons'
import { THEMES, type Theme } from '../lib/theme'

export interface WindowFrameProps {
  project: string
  branch: string
  ahead: number
  behind: number
  theme: Theme
  onThemeChange: (theme: Theme) => void
  sidebarOpen: boolean
  onSidebarOpenChange: (open: boolean) => void
  /** The full-height leading column, drawn beside the toolbar, not under it. */
  sidebar?: ReactNode
  /** The columns under the toolbar: the rail, the content, the aside. */
  children: ReactNode
}

/**
 * The window's contents.
 *
 * It draws no window of its own — no corner, no shadow, no controls. The real
 * one supplies those: `deno desktop` opens it with the titlebar hidden, so the
 * toolbar below runs to the top edge and the system's own controls float in
 * the leading corner. In a browser tab there is no chrome to imitate either.
 *
 * The sidebar runs the full height and the toolbar spans only the pane beside
 * it — the shape a macOS app takes once its sidebar is the thing the window is
 * organised around. That is what the frame tokens are drawn for: the sidebar's
 * header strip is `--frame-titlebar-size` tall, so the two line up along one
 * edge without either being told about the other.
 *
 * The window is the `SidebarProvider`: it holds whether the sidebar is open,
 * and ⌘B and the toolbar's trigger both go through it.
 */
export function WindowFrame({
  project,
  branch,
  ahead,
  behind,
  theme,
  onThemeChange,
  sidebarOpen,
  onSidebarOpenChange,
  sidebar,
  children,
}: WindowFrameProps) {
  const current = THEMES.find(entry => entry.value === theme)

  return (
    <SidebarProvider
      data-sidebar={sidebarOpen}
      className="ca-window"
      open={sidebarOpen}
      onOpenChange={onSidebarOpenChange}
    >
      {sidebar}

      <div className="ca-pane">
        <header className="ca-toolbar">
          <SidebarTrigger
            active={sidebarOpen}
            label={sidebarOpen ? 'Hide projects' : 'Show projects'}
          >
            <Icon icon={SidebarGlyph} size={16} />
          </SidebarTrigger>

          <div className="ca-toolbar__title">
            <span className="ca-toolbar__project">{project}</span>
            <Badge size="small" tint="neutral" variant="outlined">
              <Icon icon={BranchGlyph} size={11} />
              {branch}
            </Badge>
            {ahead || behind ? (
              <Badge
                size="small"
                tint={behind ? 'warning' : 'neutral'}
                title={`${ahead} ahead, ${behind} behind upstream`}
              >
                ↑{ahead} ↓{behind}
              </Badge>
            ) : null}
            <Badge size="small" tint="info">
              Prototype
            </Badge>
          </div>

          <span className="ca-toolbar__gap" />

          <Menu
            align="end"
            items={THEMES.map(entry => ({
              label: entry.label,
              checked: entry.value === theme,
              onSelect: () => onThemeChange(entry.value),
            }))}
            trigger={
              <Button size="tiny" variant="plain">
                {current?.label ?? 'Theme'}
              </Button>
            }
          />
        </header>

        <div className="ca-pane__body">{children}</div>
      </div>
    </SidebarProvider>
  )
}
