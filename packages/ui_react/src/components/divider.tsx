import type { ComponentPropsWithoutRef } from 'react'

import { cx } from '../lib/cx'

export type DividerTone = 'default' | 'muted'

export type DividerOrientation = 'horizontal' | 'vertical'

export interface DividerProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'children'
> {
  tone?: DividerTone
  orientation?: DividerOrientation
}

export function Divider({
  tone = 'default',
  orientation = 'horizontal',
  className,
  ...props
}: DividerProps) {
  return (
    <div
      role="separator"
      aria-orientation={orientation}
      data-orientation={orientation}
      data-tone={tone}
      className={cx('dz-divider', className)}
      {...props}
    />
  )
}
