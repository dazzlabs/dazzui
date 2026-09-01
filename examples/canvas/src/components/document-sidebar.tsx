import {
  Icon,
  IconButton,
  NavAction,
  NavItem,
  Sidebar,
  SidebarGroup,
} from '@dazzlabs/dazzui-react'
import {
  Add20Regular,
  Document20Regular,
  Eye20Regular,
  EyeOff20Regular,
  LockClosed20Regular,
  LockOpen20Regular,
} from '@fluentui/react-icons'
import type { MouseEvent } from 'react'

import { toolOf } from '../data/tools'
import type { Page, Shape } from '../lib/scene'

export interface DocumentSidebarProps {
  pages: readonly Page[]
  pageId: string
  onOpenPage: (id: string) => void
  onAddPage: () => void
  /** The open page, whose layers the column lists below its pages. */
  page: Page
  selection: readonly string[]
  onSelect: (id: string, additive: boolean) => void
  onSetHidden: (id: string, hidden: boolean) => void
  onSetLocked: (id: string, locked: boolean) => void
}

/**
 * The leading column: the document. Its pages, and the layers on the open one.
 *
 * The layers are listed front to back — the shape on top of the drawing is the
 * row at the top of the list — which is the one place this column deliberately
 * disagrees with the model underneath it, where the last shape is the one in
 * front. Every canvas does it this way, because the list is read as a stack
 * seen from above.
 */
export function DocumentSidebar({
  pages,
  pageId,
  onOpenPage,
  onAddPage,
  page,
  selection,
  onSelect,
  onSetHidden,
  onSetLocked,
}: DocumentSidebarProps) {
  const layers = [...page.shapes].reverse()

  return (
    <Sidebar
      className="cv-sidebar"
      // The strip carries nothing of its own, and still has to be here: it is
      // held at the titlebar's height, which is what keeps this column's rows
      // level with the toolbar beside them — and with the titlebar hidden it
      // is where the window's own controls land.
      header={<span aria-hidden />}
    >
      <SidebarGroup label="Pages">
        {pages.map(entry => (
          <NavItem
            key={entry.id}
            active={entry.id === pageId}
            icon={<Icon icon={Document20Regular} size={14} />}
            size="small"
            onClick={() => onOpenPage(entry.id)}
          >
            <span className="cv-truncate">{entry.name}</span>
            <span className="cv-count">{entry.shapes.length}</span>
          </NavItem>
        ))}

        {/* A nav action takes no icon slot — it adds rather than names a page,
            so its glyph belongs in the label with it. */}
        <NavAction className="cv-sidebar__new" size="small" onClick={onAddPage}>
          <Icon icon={Add20Regular} size={14} />
          New page
        </NavAction>
      </SidebarGroup>

      <SidebarGroup label="Layers">
        {layers.map(shape => (
          <Layer
            key={shape.id}
            selected={selection.includes(shape.id)}
            shape={shape}
            onSelect={onSelect}
            onSetHidden={onSetHidden}
            onSetLocked={onSetLocked}
          />
        ))}

        {layers.length === 0 ? (
          <p className="cv-sidebar__empty">Nothing drawn yet.</p>
        ) : null}
      </SidebarGroup>
    </Sidebar>
  )
}

interface LayerProps {
  shape: Shape
  selected: boolean
  onSelect: (id: string, additive: boolean) => void
  onSetHidden: (id: string, hidden: boolean) => void
  onSetLocked: (id: string, locked: boolean) => void
}

/**
 * One layer.
 *
 * The row's two switches are siblings of the row rather than children of it: a
 * `NavItem` is a button, and a button inside a button is not a thing a browser
 * will build. They are laid over its trailing edge instead, and they appear on
 * hover — except where they are already doing something, since a hidden layer
 * that looks like every other row is a layer you will spend a minute looking
 * for on the canvas.
 */
function Layer({
  shape,
  selected,
  onSelect,
  onSetHidden,
  onSetLocked,
}: LayerProps) {
  const marked = shape.hidden || shape.locked

  return (
    <div data-marked={marked} className="cv-layer">
      <NavItem
        active={selected}
        icon={<Icon icon={toolOf(shape.kind).icon} size={14} />}
        size="small"
        title={`${shape.name} — ${Math.round(shape.width)} × ${Math.round(shape.height)}`}
        onClick={(event: MouseEvent) =>
          onSelect(shape.id, event.metaKey || event.shiftKey)
        }
      >
        <span className="cv-truncate" data-dim={shape.hidden}>
          {shape.name}
        </span>
      </NavItem>

      <span className="cv-layer__switches">
        <IconButton
          label={shape.hidden ? `Show ${shape.name}` : `Hide ${shape.name}`}
          size="tiny"
          variant="plain"
          onClick={() => onSetHidden(shape.id, !shape.hidden)}
        >
          <Icon
            icon={shape.hidden ? EyeOff20Regular : Eye20Regular}
            size={13}
          />
        </IconButton>
        <IconButton
          label={shape.locked ? `Unlock ${shape.name}` : `Lock ${shape.name}`}
          size="tiny"
          variant="plain"
          onClick={() => onSetLocked(shape.id, !shape.locked)}
        >
          <Icon
            icon={shape.locked ? LockClosed20Regular : LockOpen20Regular}
            size={13}
          />
        </IconButton>
      </span>
    </div>
  )
}
