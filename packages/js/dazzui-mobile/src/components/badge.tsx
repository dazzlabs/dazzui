import type { ComponentProps } from 'react'

import { cx } from '../lib/cx'
import { usePlatform } from '../lib/theme'

export interface BadgeProps extends ComponentProps<'span'> {
  /** The count; over `max` it shows as `max+`. */
  count?: number
  max?: number
  /** A dot with no number — something is new, not how much. */
  dot?: boolean
  /** Material draws the badge in its error colour; `tint` uses the accent. */
  tone?: 'error' | 'tint'
}

/**
 * The count a tab bar item carries: iOS's red capsule in the footnote
 * weight, Material's 16dp badge in label-small. Both read the danger ramp
 * — UIKit's `badgeColor` default is system red, Material's is error.
 */
export function Badge({
  count,
  max = 99,
  dot = false,
  tone = 'error',
  className,
  children,
  ...props
}: BadgeProps) {
  const platform = usePlatform()
  const label =
    children ?? (count === undefined ? null : count > max ? `${max}+` : count)
  return (
    <span
      data-platform={platform}
      data-dot={dot || undefined}
      data-tone={tone}
      className={cx('dzm-badge', className)}
      {...props}
    >
      {dot ? null : label}
    </span>
  )
}
