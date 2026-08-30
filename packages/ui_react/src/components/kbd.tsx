import type { ComponentPropsWithoutRef } from 'react'

import { cx } from '../lib/cx'

export type KbdSize = 'small' | 'medium' | 'large'

export interface KbdProps extends ComponentPropsWithoutRef<'kbd'> {
  size?: KbdSize
}

/** One key cap. A chord is several of these, composed by the caller. */
export function Kbd({ size = 'medium', className, ...props }: KbdProps) {
  return <kbd data-size={size} className={cx('dz-kbd', className)} {...props} />
}
