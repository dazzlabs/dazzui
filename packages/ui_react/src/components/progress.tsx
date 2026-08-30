import type { ComponentPropsWithoutRef } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type ProgressSize = 'small' | 'medium' | 'large'

export interface ProgressProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'children'
> {
  /** 0–100. Omit for an indeterminate bar. */
  value?: number
  size?: ProgressSize
  tint?: Tint
  /** Paint the fill with the theme's two-stop accent ramp instead of a flat tint. */
  gradient?: boolean
}

export function Progress({
  value,
  size = 'small',
  tint = 'primary',
  gradient = false,
  className,
  ...props
}: ProgressProps) {
  const indeterminate = value === undefined
  const clamped = indeterminate ? 0 : Math.min(100, Math.max(0, value))

  return (
    <div
      role="progressbar"
      aria-valuemin={indeterminate ? undefined : 0}
      aria-valuemax={indeterminate ? undefined : 100}
      aria-valuenow={indeterminate ? undefined : clamped}
      data-indeterminate={indeterminate}
      data-gradient={gradient || undefined}
      data-size={size}
      data-tint={tint}
      className={cx('dz-progress', className)}
      {...props}
    >
      <div
        className="dz-progress__fill"
        style={indeterminate ? undefined : { width: `${clamped}%` }}
      />
    </div>
  )
}
