import type { CSSProperties, ReactNode } from 'react'

/*
 * The arrangements a bare specimen needs.
 *
 * Only a component's `Overview` is a documented page; every other story
 * renders the component and nothing else — no headings, no prose, no labelled
 * rows — so all it ever needs is a row or a column with a gap in it, and a
 * width where the component has none of its own.
 */

type StageProps = {
  gap?: string
  width?: string
  children: ReactNode
}

export function Row({ gap = 'var(--spacing-3)', width, children }: StageProps) {
  const style: CSSProperties = {
    display: 'flex',
    flexWrap: 'wrap',
    alignItems: 'center',
    gap,
    width,
  }
  return <div style={style}>{children}</div>
}

export function Col({ gap = 'var(--spacing-3)', width, children }: StageProps) {
  const style: CSSProperties = {
    display: 'flex',
    flexDirection: 'column',
    gap,
    width,
  }
  return <div style={style}>{children}</div>
}

/** A window, for the columns that only make sense with a pane beside them. */
export function Frame({
  height = '22rem',
  width = '46rem',
  children,
}: {
  height?: string
  width?: string
  children: ReactNode
}) {
  return (
    <div
      style={{
        display: 'flex',
        width,
        height,
        overflow: 'hidden',
        border: 'var(--spacing-px) solid var(--color-border)',
        borderRadius: 'var(--radius-large)',
        background: 'var(--color-surface)',
      }}
    >
      {children}
    </div>
  )
}

/** The empty pane a navigation column is a navigation column to. */
export function Pane({ children }: { children?: ReactNode }) {
  return (
    <div style={{ flex: 1, minWidth: 0, padding: 'var(--spacing-5)' }}>
      {children}
    </div>
  )
}
