import { useCallback, useState } from 'react'

import type { Point, Rect } from './geometry'

/**
 * What the drawing is being looked at through: where the scene's origin lands
 * on the stage, in screen pixels, and how far away the eye is.
 *
 * The viewport is not part of the document. Panning is not an edit, and undo
 * putting the drawing back where it was standing would undo the wrong thing.
 */
export interface Viewport {
  x: number
  y: number
  zoom: number
}

export interface Size {
  width: number
  height: number
}

const ZOOM_MIN = 0.1
const ZOOM_MAX = 4

/** The stops the zoom menu offers, and the ones ⌘+ and ⌘− walk between. */
export const ZOOM_STEPS = [0.25, 0.5, 1, 2, 4]

/**
 * How much room a fitted drawing keeps around itself, in screen pixels.
 *
 * Not a token: this is the distance the eye stands back at, which is the
 * viewport's business rather than the design system's — nothing on screen is
 * spaced by it.
 */
const FIT_PADDING = 56

const clamp = (zoom: number) => Math.min(ZOOM_MAX, Math.max(ZOOM_MIN, zoom))

/** A point on the stage, in the scene's own units. */
export function toScene(viewport: Viewport, point: Point): Point {
  return {
    x: (point.x - viewport.x) / viewport.zoom,
    y: (point.y - viewport.y) / viewport.zoom,
  }
}

/** The next stop above or below the current zoom, for the keyboard. */
export function nextZoom(zoom: number, direction: 1 | -1): number {
  const stops = direction === 1 ? ZOOM_STEPS : [...ZOOM_STEPS].reverse()
  // A hair of slack, so a zoom sitting exactly on a stop moves off it rather
  // than finding itself.
  const found = stops.find(stop =>
    direction === 1 ? stop > zoom * 1.01 : stop < zoom * 0.99
  )
  return clamp(found ?? zoom)
}

export interface ViewportStore {
  viewport: Viewport
  /** The stage's own size, as its resize observer reports it. */
  size: Size
  measure: (size: Size) => void
  panBy: (dx: number, dy: number) => void
  /** Zoom about a point on the stage — the pointer, or the stage's middle. */
  zoomTo: (zoom: number, pivot?: Point) => void
  zoomBy: (factor: number, pivot?: Point) => void
  /** Frame a rectangle. Nothing to frame puts the eye back at 100%. */
  fit: (rect: Rect | null) => void
}

export function useViewport(): ViewportStore {
  const [viewport, setViewport] = useState<Viewport>({ x: 0, y: 0, zoom: 1 })
  const [size, setSize] = useState<Size>({ width: 0, height: 0 })

  const measure = useCallback((next: Size) => {
    setSize(current =>
      current.width === next.width && current.height === next.height
        ? current
        : next
    )
  }, [])

  const panBy = useCallback((dx: number, dy: number) => {
    setViewport(current => ({
      ...current,
      x: current.x + dx,
      y: current.y + dy,
    }))
  }, [])

  /**
   * Zooming holds one point still: whatever was under the pivot stays under
   * it. Without that the drawing slides out from under the pointer, and a
   * wheel zoom becomes a game of chasing what you were looking at.
   */
  const zoomTo = useCallback(
    (zoom: number, pivot?: Point) => {
      setViewport(current => {
        const next = clamp(zoom)
        const at = pivot ?? { x: size.width / 2, y: size.height / 2 }
        const scene = toScene(current, at)
        return {
          zoom: next,
          x: at.x - scene.x * next,
          y: at.y - scene.y * next,
        }
      })
    },
    [size.height, size.width]
  )

  const zoomBy = useCallback(
    (factor: number, pivot?: Point) => {
      setViewport(current => {
        const next = clamp(current.zoom * factor)
        const at = pivot ?? { x: size.width / 2, y: size.height / 2 }
        const scene = toScene(current, at)
        return {
          zoom: next,
          x: at.x - scene.x * next,
          y: at.y - scene.y * next,
        }
      })
    },
    [size.height, size.width]
  )

  const fit = useCallback(
    (rect: Rect | null) => {
      if (size.width === 0 || size.height === 0) return
      if (!rect || rect.width === 0 || rect.height === 0) {
        setViewport({ x: size.width / 2, y: size.height / 2, zoom: 1 })
        return
      }

      // Never zoom past 100% to fill the stage: a drawing blown up because it
      // happened to be small reads as a different drawing.
      const zoom = clamp(
        Math.min(
          1,
          (size.width - FIT_PADDING * 2) / rect.width,
          (size.height - FIT_PADDING * 2) / rect.height
        )
      )

      setViewport({
        zoom,
        x: size.width / 2 - (rect.x + rect.width / 2) * zoom,
        y: size.height / 2 - (rect.y + rect.height / 2) * zoom,
      })
    },
    [size.height, size.width]
  )

  return { viewport, size, measure, panBy, zoomTo, zoomBy, fit }
}
