import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'
import { SectionLabel } from './section-label'

/** The second navigation column, between the sidebar and the content. */
export function Rail({ className, ...props }: ComponentPropsWithoutRef<'div'>) {
  return <div className={cx('dz-rail', className)} {...props} />
}

export interface RailGroupProps extends ComponentPropsWithoutRef<'div'> {
  label?: ReactNode
}

/**
 * A run of rail rows, optionally named. The first run usually goes unlabelled:
 * it is what the pane is already called, and naming it twice says nothing. A
 * second run is the one that needs the label, because a gap alone leaves the
 * reader to guess what the rows below it have in common.
 */
export function RailGroup({
  label,
  className,
  children,
  ...props
}: RailGroupProps) {
  return (
    <div className={cx('dz-rail-group', className)} {...props}>
      {label ? (
        <SectionLabel className="dz-rail-group__label">{label}</SectionLabel>
      ) : null}
      {children}
    </div>
  )
}
