import type { ComponentPropsWithoutRef } from 'react'

import { cx } from '../lib/cx'

export type KbdSize = 'small' | 'medium' | 'large'

/**
 * `hint` is bare faint text — the default, because most keys shown in an
 * interface are asides. `strong` darkens it a step. `key` alone draws the
 * recessed cap.
 */
export type KbdVariant = 'hint' | 'strong' | 'key'

export interface KbdProps extends ComponentPropsWithoutRef<'kbd'> {
  size?: KbdSize
  variant?: KbdVariant
}

/** One key hint or cap. A chord is several of these, composed by the caller. */
export function Kbd({
  size = 'medium',
  variant = 'hint',
  className,
  ...props
}: KbdProps) {
  return (
    <kbd
      data-size={size}
      data-variant={variant}
      className={cx('dz-kbd', className)}
      {...props}
    />
  )
}
