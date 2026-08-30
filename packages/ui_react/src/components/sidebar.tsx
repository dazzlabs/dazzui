import {
  type ComponentPropsWithoutRef,
  type KeyboardEvent as ReactKeyboardEvent,
  type PointerEvent as ReactPointerEvent,
  type ReactNode,
  useEffect,
  useRef,
  useState,
} from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'
import { SectionLabel } from './section-label'

/**
 * How far the divider may travel. Below the floor the labels stop fitting
 * their rows; above the ceiling the sidebar starts competing with the pane it
 * is meant to serve.
 */
const MIN_WIDTH = 150
const MAX_WIDTH = 320

/**
 * Drag this far past the floor and the sidebar collapses instead of shrinking.
 * A desktop divider does this, and it is the only way to close a sidebar
 * without going back to the toolbar button.
 */
const COLLAPSE_SLOP = 32

/** Arrow keys walk the divider; shift makes the step a coarse one. */
const KEY_STEP = 8
const COARSE_KEY_STEP = 32

export interface SidebarProps extends ComponentPropsWithoutRef<'nav'> {
  /**
   * Content for the strip above the row list, held at exactly the titlebar's
   * height so it lines up with the toolbar in the pane beside it.
   */
  header?: ReactNode
  /**
   * Let the separator on the trailing edge be dragged. Off by default: a
   * sidebar standing on its own has no pane to trade width with, and a handle
   * that leads nowhere is worse than no handle.
   */
  resizable?: boolean
  /** Controlled width in px. Leave it out and the sidebar owns its own. */
  width?: number
  /** Starting width for the uncontrolled case; defaults to the token. */
  defaultWidth?: number
  onWidthChange?: (width: number) => void
  minWidth?: number
  maxWidth?: number
  /**
   * Called when the divider is dragged past the floor. Left out, the drag
   * simply stops at `minWidth` — pass it only where collapsing is a state the
   * window can actually be in.
   */
  onCollapse?: () => void
  /** Accessible name for the divider. */
  resizeLabel?: string
  tint?: Tint
}

export function Sidebar({
  header,
  resizable = false,
  width: widthProp,
  defaultWidth,
  onWidthChange,
  minWidth = MIN_WIDTH,
  maxWidth = MAX_WIDTH,
  onCollapse,
  resizeLabel = 'Resize sidebar',
  tint = 'primary',
  className,
  children,
  style,
  ...props
}: SidebarProps) {
  const nav = useRef<HTMLElement>(null)
  const origin = useRef<{ x: number; width: number } | null>(null)
  /** What the sidebar measured before anyone dragged it — double-click home. */
  const natural = useRef(defaultWidth)
  const [ownWidth, setOwnWidth] = useState(defaultWidth)
  const [dragging, setDragging] = useState(false)

  const width = widthProp ?? ownWidth
  const clamp = (value: number) =>
    Math.round(Math.min(maxWidth, Math.max(minWidth, value)))

  const commit = (next: number) => {
    if (widthProp === undefined) setOwnWidth(next)
    onWidthChange?.(next)
  }

  // Until the first drag the width is the token's, and the divider needs a
  // number: to report as aria-valuenow, to start the keyboard from, and to
  // send a double-click home to.
  //
  // Measure it rather than parsing `--frame-sidebar-width`. What unit the token
  // carries is the design system's business, and reading it here means
  // agreeing with that choice forever — parsing a `10.75rem` sidebar as a
  // number yields 10.75, which is a perfectly finite way to be 11px wide.
  // The rendered width is the resolved value, in px, whatever the token said.
  useEffect(() => {
    if (!resizable) return
    const element = nav.current
    if (!element) return

    if (natural.current === undefined) {
      const measured = Math.round(element.getBoundingClientRect().width)
      // Not laid out yet — leave the CSS width alone rather than commit a 0.
      if (measured === 0) return
      natural.current = measured
    }
    if (width === undefined) setOwnWidth(natural.current)
  }, [resizable, width])

  // The pointer is captured for the length of the drag, so the events keep
  // coming — but the cursor still belongs to whatever the pointer is over. Pin
  // it, and stop the pane beside it from selecting text on the way past.
  useEffect(() => {
    if (!dragging) return

    const { style: body } = document.body
    const cursor = body.cursor
    const select = body.userSelect
    body.cursor = 'col-resize'
    body.userSelect = 'none'
    return () => {
      body.cursor = cursor
      body.userSelect = select
    }
  }, [dragging])

  const endDrag = (event: ReactPointerEvent<HTMLDivElement>) => {
    origin.current = null
    setDragging(false)
    if (event.currentTarget.hasPointerCapture(event.pointerId)) {
      event.currentTarget.releasePointerCapture(event.pointerId)
    }
  }

  const onPointerDown = (event: ReactPointerEvent<HTMLDivElement>) => {
    const element = nav.current
    if (event.button !== 0 || !element) return

    event.preventDefault()
    event.currentTarget.setPointerCapture(event.pointerId)
    origin.current = {
      x: event.clientX,
      width: element.getBoundingClientRect().width,
    }
    setDragging(true)
  }

  const onPointerMove = (event: ReactPointerEvent<HTMLDivElement>) => {
    const from = origin.current
    if (!from) return

    const raw = from.width + (event.clientX - from.x)
    if (onCollapse && raw < minWidth - COLLAPSE_SLOP) {
      // Collapsed is not a width. Hand back the one the drag started from, so
      // re-opening the sidebar does not inherit a half-dragged number.
      commit(clamp(from.width))
      endDrag(event)
      onCollapse()
      return
    }
    commit(clamp(raw))
  }

  const onKeyDown = (event: ReactKeyboardEvent<HTMLDivElement>) => {
    if (event.key !== 'ArrowLeft' && event.key !== 'ArrowRight') return

    const step = event.shiftKey ? COARSE_KEY_STEP : KEY_STEP
    const current =
      width ?? nav.current?.getBoundingClientRect().width ?? minWidth
    event.preventDefault()
    commit(clamp(current + (event.key === 'ArrowRight' ? step : -step)))
  }

  return (
    <nav
      ref={nav}
      data-tint={tint}
      className={cx('dz-sidebar', className)}
      style={width === undefined ? style : { width, ...style }}
      {...props}
    >
      {header ? <div className="dz-sidebar__header">{header}</div> : null}
      <div className="dz-sidebar__body">{children}</div>

      {resizable ? (
        <div
          role="separator"
          aria-orientation="vertical"
          aria-label={resizeLabel}
          aria-valuenow={width}
          aria-valuemin={minWidth}
          aria-valuemax={maxWidth}
          tabIndex={0}
          data-dragging={dragging}
          className="dz-sidebar__resizer"
          onDoubleClick={() => {
            // Double-clicking a divider puts it back where it started, the
            // same thing every split view does.
            if (natural.current !== undefined) commit(clamp(natural.current))
          }}
          onKeyDown={onKeyDown}
          onPointerCancel={endDrag}
          onPointerDown={onPointerDown}
          onPointerMove={onPointerMove}
          onPointerUp={endDrag}
        />
      ) : null}
    </nav>
  )
}

export interface SidebarGroupProps extends ComponentPropsWithoutRef<'div'> {
  label?: ReactNode
}

/**
 * A labelled run of rows. A sidebar breaks its rows into groups rather than
 * stacking them; the gap between groups does most of the work and the label
 * only names it.
 */
export function SidebarGroup({
  label,
  className,
  children,
  ...props
}: SidebarGroupProps) {
  return (
    <div className={cx('dz-sidebar-group', className)} {...props}>
      {label ? (
        <SectionLabel className="dz-sidebar-group__label">{label}</SectionLabel>
      ) : null}
      {children}
    </div>
  )
}

export interface SidebarCardProps extends ComponentPropsWithoutRef<'div'> {
  label?: ReactNode
}

/** The card pinned to the foot of a sidebar — a count, a version, a queue. */
export function SidebarCard({
  label,
  className,
  children,
  ...props
}: SidebarCardProps) {
  return (
    <div className={cx('dz-sidebar-card', className)} {...props}>
      {label ? <SectionLabel>{label}</SectionLabel> : null}
      {children}
    </div>
  )
}
