import { useSidebar } from '@dazzlabs/dazzui'
import { type PointerEvent, useRef, useState } from 'react'

export interface Column {
  /** Where the width is kept between visits. */
  key: string
  initial: number
  min: number
  max: number
}

const clamp = (value: number, { min, max }: Column) =>
  Math.min(Math.max(Math.round(value), min), max)

/** A column's width in pixels, remembered by the browser it was set in. */
export function useColumnWidth(column: Column) {
  const [width, setWidth] = useState(() => {
    try {
      const stored = Number(localStorage.getItem(column.key))
      return stored ? clamp(stored, column) : column.initial
    } catch {
      return column.initial
    }
  })
  const set = (next: number) => {
    const clamped = clamp(next, column)
    setWidth(clamped)
    try {
      localStorage.setItem(column.key, String(clamped))
    } catch {
      /* A browser that keeps nothing still resizes; it only forgets. */
    }
  }
  return [width, set] as const
}

const STEP = 8
const COARSE_STEP = 32

interface ResizerProps {
  /** Which column this is the edge of: it grows away from that side. */
  side: 'left' | 'right'
  column: Column
  width: number
  setWidth: (width: number) => void
  label: string
}

/**
 * The edge of a column, to be dragged. The width is in pixels rather than a
 * share of the window, because a source list is as wide as its names and
 * does not want to grow because the window did.
 */
export function Resizer({
  side,
  column,
  width,
  setWidth,
  label,
}: ResizerProps) {
  const { open } = useSidebar()
  const drag = useRef<{ start: number; width: number } | undefined>(undefined)
  const outward = side === 'left' ? 1 : -1

  /* The left column can be put away, and an edge with no column is nothing
     to take hold of. */
  if (side === 'left' && !open) return null

  const end = (event: PointerEvent<HTMLDivElement>) => {
    if (!drag.current) return
    drag.current = undefined
    event.currentTarget.releasePointerCapture(event.pointerId)
    delete document.documentElement.dataset.resizing
  }

  return (
    <div
      role="separator"
      tabIndex={0}
      aria-label={label}
      aria-orientation="vertical"
      aria-valuenow={width}
      aria-valuemin={column.min}
      aria-valuemax={column.max}
      className="gk-resizer"
      data-side={side}
      onPointerDown={event => {
        /* Before anything else: a drag that began as a text selection
           carries the selection along with it. */
        event.preventDefault()
        drag.current = { start: event.clientX, width }
        event.currentTarget.setPointerCapture(event.pointerId)
        document.documentElement.dataset.resizing = ''
      }}
      onPointerMove={event => {
        const state = drag.current
        if (!state) return
        setWidth(state.width + (event.clientX - state.start) * outward)
      }}
      onPointerUp={end}
      onPointerCancel={end}
      onDoubleClick={() => setWidth(column.initial)}
      onKeyDown={event => {
        const step = event.shiftKey ? COARSE_STEP : STEP
        if (event.key === 'ArrowRight') setWidth(width + step * outward)
        else if (event.key === 'ArrowLeft') setWidth(width - step * outward)
        else if (event.key === 'Home') setWidth(column.min)
        else if (event.key === 'End') setWidth(column.max)
        else return
        event.preventDefault()
      }}
    />
  )
}
