import type { ComponentProps } from 'react'

import { cx } from '../lib/cx'
import { usePlatform } from '../lib/theme'

/**
 * Material's three cards. iOS has one card — the inset group — and draws
 * `elevated` and `filled` as that surface at the 26pt corner, `outlined`
 * as the same with a hairline.
 */
export type CardVariant = 'elevated' | 'filled' | 'outlined'

export interface CardProps extends ComponentProps<'div'> {
  variant?: CardVariant
  /** Removes the padding, for a card that is a list or an image. */
  flush?: boolean
}

export function Card({
  variant = 'elevated',
  flush = false,
  className,
  ...props
}: CardProps) {
  const platform = usePlatform()
  return (
    <div
      data-platform={platform}
      data-variant={variant}
      data-flush={flush || undefined}
      className={cx('dzm-card', className)}
      {...props}
    />
  )
}
