import type { ComponentPropsWithoutRef } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type BadgeSize = 'small' | 'medium' | 'large'

/** The `control.color.*` recipes that read as a label rather than a control. */
export type BadgeVariant = 'filled' | 'tinted' | 'outlined' | 'raised'

export interface BadgeProps extends ComponentPropsWithoutRef<'span'> {
  size?: BadgeSize
  variant?: BadgeVariant
  /** Which color ramp the variant resolves against. */
  tint?: Tint
}

export function Badge({
  size = 'medium',
  variant = 'tinted',
  tint = 'primary',
  className,
  ...props
}: BadgeProps) {
  return (
    <span
      data-size={size}
      data-tint={tint}
      data-variant={variant}
      className={cx('dz-badge', className)}
      {...props}
    />
  )
}
