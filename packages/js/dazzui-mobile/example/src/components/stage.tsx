import { Text } from '@dazzlabs/dazzui-mobile'
import type { ReactNode } from 'react'

/**
 * What a story arranges its specimens with: a padded column, rows, and a
 * caption per group. The kit's own `Screen` and bars wrap a story that
 * is a whole screen; these are for the ones that are a set of controls.
 */

export function Stage({ children }: { children: ReactNode }) {
  return <div className="stage">{children}</div>
}

export function Group({
  title,
  children,
}: {
  title: string
  children: ReactNode
}) {
  return (
    <section className="stage__group">
      <Text role="footnote" tone="secondary" as="h2" className="stage__caption">
        {title}
      </Text>
      {children}
    </section>
  )
}

export function Row({
  children,
  between = false,
}: {
  children: ReactNode
  between?: boolean
}) {
  return (
    <div className="stage__row" data-between={between || undefined}>
      {children}
    </div>
  )
}

export function Col({ children }: { children: ReactNode }) {
  return <div className="stage__col">{children}</div>
}
