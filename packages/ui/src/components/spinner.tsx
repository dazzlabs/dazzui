import type { ComponentPropsWithoutRef } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type SpinnerSize = 'small' | 'medium' | 'large'

export interface SpinnerProps extends Omit<
  ComponentPropsWithoutRef<'span'>,
  'children'
> {
  size?: SpinnerSize
  tint?: Tint
  /** Announced to assistive tech; pass your own copy to localise it. */
  label?: string
}

export function Spinner({
  size = 'medium',
  tint = 'primary',
  label = 'Loading',
  className,
  ...props
}: SpinnerProps) {
  return (
    <span
      role="status"
      aria-label={label}
      data-size={size}
      data-tint={tint}
      className={cx('dz-spinner', className)}
      {...props}
    />
  )
}
