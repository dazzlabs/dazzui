import type { CSSProperties, ReactNode } from 'react'

interface StageProps {
  gap?: string
  width?: string
  align?: CSSProperties['alignItems']
  children: ReactNode
}

/** Instances side by side. Draws nothing itself. */
export function Row({
  gap = '3',
  width,
  align = 'center',
  children,
}: StageProps) {
  return (
    <div
      style={{
        display: 'flex',
        flexWrap: 'wrap',
        alignItems: align,
        gap: `var(--spacing-${gap})`,
        width,
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
        alignItems: align,
        gap: `var(--spacing-${gap})`,
        width,
        maxWidth: '100%',
      }}
    >
      {children}
    </div>
  )
}
