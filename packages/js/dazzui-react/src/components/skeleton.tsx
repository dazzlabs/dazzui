import type { ComponentPropsWithoutRef, CSSProperties } from 'react'

import { cx } from '../lib/cx'

/**
 * `text` is a bar on the line box of the type it stands in for; `circle` is
 * an avatar or a glyph; `rect` is everything with a box of its own — a
 * thumbnail, a card, a chart.
 */
export type SkeletonShape = 'text' | 'circle' | 'rect'

export interface SkeletonProps extends ComponentPropsWithoutRef<'span'> {
  shape?: SkeletonShape
  /** Any CSS length. A `text` bar with no width takes the whole line. */
  width?: string | number
  /** Any CSS length. `text` and `circle` derive theirs and ignore it. */
  height?: string | number
}

/**
 * The shape of something that has not arrived.
 *
 * A `Spinner` says the app is busy; a skeleton says *what* is coming, and
 * where it will be — which is why it belongs on the first load of a view
 * whose layout is already known, and a spinner belongs on an action whose
 * result is not. Drawing one for content that then lands somewhere else is
 * worse than drawing nothing.
 *
 * It carries no text and announces nothing: the region it fills should be
 * `aria-busy` on the container, so a reader hears one "loading" rather than
 * one per bar.
 */
export function Skeleton({
  shape = 'text',
  width,
  height,
  className,
  style,
  ...props
}: SkeletonProps) {
  return (
    <span
      aria-hidden
      data-shape={shape}
      className={cx('dz-skeleton', className)}
      style={{ width, height, ...style } as CSSProperties}
      {...props}
    />
  )
}

export interface SkeletonTextProps extends ComponentPropsWithoutRef<'span'> {
  /** How many bars. The last one is short, the way a paragraph ends. */
  lines?: number
}

/**
 * A paragraph's worth of bars. The last one stops short because a full-width
 * final line reads as a table rather than as prose, and the ragged edge is
 * most of what makes a block of bars legible as text at all.
 */
export function SkeletonText({
  lines = 3,
  className,
  ...props
}: SkeletonTextProps) {
  return (
    <span className={cx('dz-skeleton-text', className)} {...props}>
      {Array.from({ length: lines }, (_, index) => (
        <Skeleton key={index} data-last={index === lines - 1} />
      ))}
    </span>
  )
}
