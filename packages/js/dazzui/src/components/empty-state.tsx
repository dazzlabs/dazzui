import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'

export interface EmptyStateProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'title' | 'children'
> {
  /** Optional leading glyph or illustration above the title. */
  icon?: ReactNode
  title: ReactNode
  /** Optional supporting copy beneath the title. */
  description?: ReactNode
  /** The way out of the empty state — usually one button. */
  action?: ReactNode
}

export function EmptyState({
  icon,
  title,
  description,
  action,
  className,
  ...props
}: EmptyStateProps) {
  return (
    <div className={cx('dz-empty-state', className)} {...props}>
      {icon ? (
        <span aria-hidden className="dz-empty-state__icon">
          {icon}
        </span>
      ) : null}
      <span className="dz-empty-state__title">{title}</span>
      {description ? (
        <span className="dz-empty-state__description">{description}</span>
      ) : null}
      {action ? <div className="dz-empty-state__action">{action}</div> : null}
    </div>
  )
}
