import {
  Badge,
  Button,
  Divider,
  Icon,
  IconButton,
  Menu,
} from '@dazzlabs/dazzui-react'
import {
  ArrowExport20Regular,
  ArrowRedo20Regular,
  ArrowUndo20Regular,
  PanelLeft20Regular,
} from '@fluentui/react-icons'
import type { ReactNode } from 'react'

import { THEMES, type Theme } from '../lib/theme'

export interface WindowFrameProps {
  /** The document, as a titlebar names a file. */
  name: string
  page: string
  theme: Theme
  onThemeChange: (theme: Theme) => void
  sidebarOpen: boolean
  onSidebarToggle: () => void
  canUndo: boolean
  canRedo: boolean
  onUndo: () => void
  onRedo: () => void
  onExport: () => void
  /** The full-height leading column, drawn beside the toolbar, not under it. */
  sidebar?: ReactNode
  /** The columns under the toolbar: the tools, the canvas, the inspector. */
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
 * The toolbar carries what belongs to the document rather than to the drawing:
 * the history, the theme, and the way out. Everything that acts on shapes is
 * in the two columns that flank the canvas, where the shapes are.
 */
export function WindowFrame({
  name,
  page,
  theme,
  onThemeChange,
  sidebarOpen,
  onSidebarToggle,
  canUndo,
  canRedo,
  onUndo,
  onRedo,
  onExport,
  sidebar,
  children,
}: WindowFrameProps) {
  const current = THEMES.find(entry => entry.value === theme)

  return (
    <div data-sidebar={sidebarOpen} className="cv-window">
      {sidebar}

      <div className="cv-pane">
        <header className="cv-toolbar">
          <IconButton
            active={sidebarOpen}
            label={sidebarOpen ? 'Hide the document' : 'Show the document'}
            onClick={onSidebarToggle}
          >
            <Icon icon={PanelLeft20Regular} size={16} />
          </IconButton>

          <Divider orientation="vertical" className="cv-toolbar__rule" />

          <IconButton
            disabled={!canUndo}
            label="Undo"
            title="Undo ⌘Z"
            onClick={onUndo}
          >
            <Icon icon={ArrowUndo20Regular} size={16} />
          </IconButton>
          <IconButton
            disabled={!canRedo}
            label="Redo"
            title="Redo ⇧⌘Z"
            onClick={onRedo}
          >
            <Icon icon={ArrowRedo20Regular} size={16} />
          </IconButton>

          <div className="cv-toolbar__title">
            <span className="cv-toolbar__name">{name}</span>
            <Badge size="small" tint="neutral" variant="outlined">
              {page}
            </Badge>
            <Badge size="small" tint="info">
              Prototype
            </Badge>
          </div>

          <span className="cv-toolbar__gap" />

          <Menu
            align="end"
            items={THEMES.map(entry => ({
              label: entry.label,
              checked: entry.value === theme,
              onSelect: () => onThemeChange(entry.value),
            }))}
            trigger={({ onClick }) => (
              <Button size="tiny" variant="plain" onClick={onClick}>
                {current?.label ?? 'Theme'}
              </Button>
            )}
          />

          <Button size="small" variant="filled" onClick={onExport}>
            <Icon icon={ArrowExport20Regular} size={14} />
            Export
          </Button>
        </header>

        <div className="cv-pane__body">{children}</div>
      </div>
    </div>
  )
}
