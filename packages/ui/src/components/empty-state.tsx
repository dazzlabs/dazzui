import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'

export interface EmptyStateProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'title' | 'children'
> {
  title: ReactNode
  /** The way out of the empty state — usually one button. */
  action?: ReactNode
}

export function EmptyState({
  title,
  action,
  className,
  ...props
}: EmptyStateProps) {
  return (
    <div className={cx('dz-empty-state', className)} {...props}>
      <span className="dz-empty-state__title">{title}</span>
      {action ? <div className="dz-empty-state__action">{action}</div> : null}
    </div>
  )
}
