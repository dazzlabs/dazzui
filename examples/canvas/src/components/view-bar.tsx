import { Button, Divider, Icon, IconButton, Menu } from '@dazzlabs/dazzui-react'
import {
  GridDots20Regular,
  Ruler20Regular,
  ScaleFit20Regular,
  ZoomIn20Regular,
  ZoomOut20Regular,
} from '@fluentui/react-icons'

import { ZOOM_STEPS } from '../lib/viewport'

export interface ViewBarProps {
  zoom: number
  onZoomTo: (zoom: number) => void
  onZoomBy: (factor: number) => void
  onFit: () => void
  grid: boolean
  onGridChange: (grid: boolean) => void
  snapping: boolean
  onSnappingChange: (snapping: boolean) => void
  selected: number
  total: number
}

/**
 * The strip under the canvas: how the drawing is being looked at, and what is
 * picked out of it.
 *
 * Nothing here edits a shape. That is the line between this bar and the
 * inspector — one is about the eye, the other about the thing — and it is why
 * the grid switches live down here rather than in a settings sheet: they
 * change what you see, not what you have.
 */
export function ViewBar({
  zoom,
  onZoomTo,
  onZoomBy,
  onFit,
  grid,
  onGridChange,
  snapping,
  onSnappingChange,
  selected,
  total,
}: ViewBarProps) {
  return (
    <footer className="cv-viewbar">
      <IconButton
        label="Zoom out"
        size="small"
        variant="plain"
        onClick={() => onZoomBy(1 / 1.25)}
      >
        <Icon icon={ZoomOut20Regular} size={15} />
      </IconButton>

      <Menu
        align="start"
        className="cv-viewbar__menu"
        items={[
          ...ZOOM_STEPS.map(step => ({
            label: `${Math.round(step * 100)}%`,
            checked: Math.abs(step - zoom) < 0.005,
            onSelect: () => onZoomTo(step),
          })),
          { label: 'Zoom to fit', shortcut: '⇧1', onSelect: onFit },
        ]}
        trigger={({ onClick }) => (
          <Button
            className="cv-viewbar__zoom"
            size="tiny"
            variant="plain"
            onClick={onClick}
          >
            {Math.round(zoom * 100)}%
          </Button>
        )}
      />

      <IconButton
        label="Zoom in"
        size="small"
        variant="plain"
        onClick={() => onZoomBy(1.25)}
      >
        <Icon icon={ZoomIn20Regular} size={15} />
      </IconButton>

      <IconButton
        label="Zoom to fit"
        size="small"
        variant="plain"
        onClick={onFit}
      >
        <Icon icon={ScaleFit20Regular} size={15} />
      </IconButton>

      <Divider orientation="vertical" className="cv-viewbar__rule" />

      <IconButton
        active={grid}
        label={grid ? 'Hide the grid' : 'Show the grid'}
        size="small"
        variant="plain"
        onClick={() => onGridChange(!grid)}
      >
        <Icon icon={GridDots20Regular} size={15} />
      </IconButton>

      <IconButton
        active={snapping}
        label={snapping ? 'Stop snapping to the grid' : 'Snap to the grid'}
        size="small"
        variant="plain"
        onClick={() => onSnappingChange(!snapping)}
      >
        <Icon icon={Ruler20Regular} size={15} />
      </IconButton>

      <span className="cv-viewbar__gap" />

      <span className="cv-viewbar__count">
        {selected > 0
          ? `${selected} of ${total} selected`
          : `${total} ${total === 1 ? 'shape' : 'shapes'}`}
      </span>
    </footer>
  )
}
