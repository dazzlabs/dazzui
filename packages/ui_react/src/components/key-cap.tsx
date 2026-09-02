import type { ComponentPropsWithoutRef } from 'react'

import { cx } from '../lib/cx'

export type KeyCapSize = 'small' | 'medium' | 'large'

/**
 * `hint` is bare faint text — the default, because most keys shown in an
 * interface are asides. `strong` darkens it a step. `key` alone draws the
 * recessed cap.
 */
export type KeyCapVariant = 'hint' | 'strong' | 'key'

export interface KeyCapProps extends ComponentPropsWithoutRef<'kbd'> {
  size?: KeyCapSize
  variant?: KeyCapVariant
}

/** One key hint or cap. A chord is several of these, composed by the caller. */
export function KeyCap({
  size = 'medium',
  variant = 'hint',
  className,
  ...props
}: KeyCapProps) {
  return (
    <kbd
      data-size={size}
      data-variant={variant}
      className={cx('dz-key-cap', className)}
      {...props}
    />
  )
}
