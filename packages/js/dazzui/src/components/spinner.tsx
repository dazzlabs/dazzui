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
  /** White ring for a spinner sitting on a filled control's own tint. */
  onAccent?: boolean
  /** Announced to assistive tech; pass your own copy to localise it. */
  label?: string
}

export function Spinner({
  size = 'medium',
  tint = 'primary',
  onAccent = false,
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
      data-on-accent={onAccent || undefined}
      className={cx('dz-spinner', className)}
      {...props}
    />
  )
}
