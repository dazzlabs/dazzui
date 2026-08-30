import type { ComponentPropsWithoutRef } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type ToggleSize = 'small' | 'medium' | 'large'

export interface ToggleProps extends Omit<
  ComponentPropsWithoutRef<'button'>,
  'children' | 'onChange'
> {
  checked: boolean
  onCheckedChange?: (checked: boolean) => void
  size?: ToggleSize
  tint?: Tint
}

export function Toggle({
  checked,
  onCheckedChange,
  size = 'medium',
  tint = 'primary',
  className,
  onClick,
  type = 'button',
  ...props
}: ToggleProps) {
  return (
    <button
      type={type}
      role="switch"
      aria-checked={checked}
      data-size={size}
      data-tint={tint}
      className={cx('dz-toggle', className)}
      onClick={event => {
        onClick?.(event)
        if (!event.defaultPrevented) onCheckedChange?.(!checked)
      }}
      {...props}
    >
      <span aria-hidden className="dz-toggle__thumb" />
    </button>
  )
}
