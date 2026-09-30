import type { ReactNode } from 'react'

/*
 * What a story arranges its components with. These are the only elements a
 * story may use beside the kit's own: no HTML, no inline style. Codegen
 * compiles every story for the other platforms, and each of them draws these
 * four from its own stage, so a story written with them lays out the same
 * everywhere.
 *
 * Lengths are written as the web writes them — a spacing step for a gap
 * (`'3'`), rem or a percentage for a width — and every port reads them the
 * same way.
 */

type Align = 'start' | 'center' | 'end' | 'stretch'

interface StageProps {
  /** A spacing step: `'3'` is `--spacing-3`. */
  gap?: string
  /** `'20rem'`, or `'100%'` to fill. */
  width?: string
  align?: Align
  children: ReactNode
}

const FLEX = { start: 'flex-start', center: 'center', end: 'flex-end' }
const flex = (align: Align) => (align === 'stretch' ? align : FLEX[align])

/**
 * Instances side by side, wrapping when they run out of room. `between`
 * pushes the first and the last to the two edges instead, which is how a
 * label and its control share a row.
 */
export function Row({
  gap = '3',
  width,
  align = 'center',
  justify = 'start',
  children,
}: StageProps & { justify?: 'start' | 'between' }) {
  return (
    <div
      style={{
        display: 'flex',
        flexWrap: justify === 'between' ? 'nowrap' : 'wrap',
        alignItems: flex(align),
        justifyContent: justify === 'between' ? 'space-between' : undefined,
        gap: `var(--spacing-${gap})`,
        width,
        maxWidth: '100%',
      }}
    >
      {children}
    </div>
  )
}

/** Instances stacked, at the width the component needs. */
export function Col({
  gap = '3',
  width,
  align = 'stretch',
  children,
}: StageProps) {
  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: flex(align),
        gap: `var(--spacing-${gap})`,
        width,
        maxWidth: '100%',
      }}
    >
      {children}
    </div>
  )
}

/**
 * A window, for what only makes sense inside one: a sidebar, a splitter. It
 * draws the edge a window has and gives its contents a size to fill.
 */
export function Frame({
  width = '36rem',
  height = '16rem',
  children,
}: {
  width?: string
  height?: string
  children: ReactNode
}) {
  return (
    <div
      style={{
        display: 'flex',
        overflow: 'hidden',
        width,
        maxWidth: '100%',
        height,
        border: 'var(--stroke-hairline) solid var(--color-border)',
        borderRadius: 'var(--radius-large)',
        background: 'var(--color-surface)',
      }}
    >
      {children}
    </div>
  )
}

/**
 * A run of text, which may hold components inline — a `Code`, a `Link`.
 * `secondary` is the quieter ink a description is set in; `strong` is the
 * weight a name is; a tint colours it, and a glyph inside takes the colour.
 */
export function Text({
  tone = 'default',
  strong = false,
  tint,
  children,
}: {
  tone?: 'default' | 'secondary'
  strong?: boolean
  tint?: 'primary' | 'info' | 'success' | 'warning' | 'danger'
  children: ReactNode
}) {
  return (
    <span
      style={{
        color: tint
          ? `var(--color-${tint}-500)`
          : tone === 'secondary'
            ? 'var(--color-content-secondary)'
            : undefined,
        fontWeight: strong ? 'var(--label-strong-font-weight)' : undefined,
      }}
    >
      {children}
    </span>
  )
}
