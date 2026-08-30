import type { ComponentPropsWithoutRef } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type CardSize = 'small' | 'medium' | 'large'

export type CardVariant = 'raised' | 'sunken' | 'outlined' | 'tinted'

export interface CardProps extends ComponentPropsWithoutRef<'div'> {
  size?: CardSize
  variant?: CardVariant
  /** Which ramp `tinted` resolves against; ignored by the other variants. */
  tint?: Tint
}

/** The filled or bordered box the panels in a layout are built from. */
export function Card({
  size = 'medium',
  variant = 'raised',
  tint = 'primary',
  className,
  ...props
}: CardProps) {
  return (
    <div
      data-size={size}
      data-tint={tint}
      data-variant={variant}
      className={cx('dz-card', className)}
      {...props}
    />
  )
}
