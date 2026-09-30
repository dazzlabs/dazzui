import type { ComponentPropsWithoutRef } from 'react'

import { cx } from '../lib/cx'

export interface ActionBarProps extends ComponentPropsWithoutRef<'div'> {
  /** Pushes the row to the trailing edge — a dialog footer's usual shape. */
  align?: 'start' | 'end' | 'between'
}

/** A row of controls kept on one rhythm. */
export function ActionBar({
  align = 'start',
  className,
  ...props
}: ActionBarProps) {
  return (
    <div
      data-align={align}
      className={cx('dz-action-bar', className)}
      {...props}
    />
  )
}
