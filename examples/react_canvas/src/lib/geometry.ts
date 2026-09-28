/**
 * Rectangle arithmetic, in scene units.
 *
 * A scene unit is the document's own measure: one unit at 100% zoom is one
 * CSS pixel, and nothing here knows about the viewport. Everything the pointer
 * hands in is converted by `viewport.ts` before it reaches this file, so the
 * geometry stays the same whatever the drawing is being looked at through.
 */

export interface Point {
  x: number
  y: number
}

export interface Rect {
  x: number
  y: number
  width: number
  height: number
}

/** The eight grips on a selection, clockwise from the top-left corner. */
export const HANDLES = ['nw', 'n', 'ne', 'e', 'se', 's', 'sw', 'w'] as const

export type Handle = (typeof HANDLES)[number]

/** The cursor each grip takes, so the pointer says which way the edge moves. */
export const CURSORS: Record<Handle, string> = {
  nw: 'nwse-resize',
  n: 'ns-resize',
  ne: 'nesw-resize',
  e: 'ew-resize',
  se: 'nwse-resize',
  s: 'ns-resize',
  sw: 'nesw-resize',
  w: 'ew-resize',
}

/**
 * Nothing may be dragged smaller than this. It is a scene-unit floor rather
 * than a screen one: a shape zoomed out to nothing is still a shape, and
 * clamping by what the pointer can see would resize the drawing differently
 * depending on how close you were standing.
 */
export const MIN_SIZE = 8

/** The rectangle two dragged corners describe, whichever way the drag went. */
export function rectFrom(a: Point, b: Point): Rect {
  return {
    x: Math.min(a.x, b.x),
    y: Math.min(a.y, b.y),
    width: Math.abs(a.x - b.x),
    height: Math.abs(a.y - b.y),
  }
}

export function contains(rect: Rect, point: Point): boolean {
  return (
    point.x >= rect.x &&
    point.x <= rect.x + rect.width &&
    point.y >= rect.y &&
    point.y <= rect.y + rect.height
  )
}

/** Whether two rectangles share any area — what a marquee selects by. */
export function overlaps(a: Rect, b: Rect): boolean {
  return (
    a.x < b.x + b.width &&
    a.x + a.width > b.x &&
    a.y < b.y + b.height &&
    a.y + a.height > b.y
  )
}

/** The one rectangle that holds them all, or nothing for an empty set. */
export function union(rects: readonly Rect[]): Rect | null {
  if (rects.length === 0) return null
  const x = Math.min(...rects.map(rect => rect.x))
  const y = Math.min(...rects.map(rect => rect.y))
  const right = Math.max(...rects.map(rect => rect.x + rect.width))
  const bottom = Math.max(...rects.map(rect => rect.y + rect.height))
  return { x, y, width: right - x, height: bottom - y }
}

/** Where a grip sits on the rectangle it belongs to. */
export function grip(rect: Rect, handle: Handle): Point {
  const x = handle.includes('w')
    ? rect.x
    : handle.includes('e')
      ? rect.x + rect.width
      : rect.x + rect.width / 2
  const y = handle.includes('n')
    ? rect.y
    : handle.includes('s')
      ? rect.y + rect.height
      : rect.y + rect.height / 2
  return { x, y }
}

/**
 * The rectangle after a grip has been dragged by (dx, dy).
 *
 * A grip dragged past the opposite edge stops at `MIN_SIZE` rather than
 * flipping the rectangle inside out. Flipping is what a drawing tool does, and
 * it is exactly wrong here: the grip under the pointer would become a
 * different grip mid-drag, and the shape would keep growing while the hand
 * kept pulling.
 */
export function resize(
  rect: Rect,
  handle: Handle,
  dx: number,
  dy: number
): Rect {
  let { x, y, width, height } = rect

  if (handle.includes('w')) {
    const edge = Math.min(x + dx, x + width - MIN_SIZE)
    width += x - edge
    x = edge
  }
  if (handle.includes('e')) width = Math.max(MIN_SIZE, width + dx)
  if (handle.includes('n')) {
    const edge = Math.min(y + dy, y + height - MIN_SIZE)
    height += y - edge
    y = edge
  }
  if (handle.includes('s')) height = Math.max(MIN_SIZE, height + dy)

  return { x, y, width, height }
}

/** The nearest grid line, for the drags the document's grid is turned on for. */
export function snap(value: number, step: number): number {
  return Math.round(value / step) * step
}
