import type { ComponentPropsWithoutRef } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type SwitchSize = 'small' | 'medium' | 'large'

export interface SwitchProps extends Omit<
  ComponentPropsWithoutRef<'button'>,
  'children' | 'onChange'
> {
  checked: boolean
  onCheckedChange?: (checked: boolean) => void
  size?: SwitchSize
  tint?: Tint
}

export function Switch({
  checked,
  onCheckedChange,
  size = 'medium',
  tint = 'primary',
  className,
  onClick,
  type = 'button',
  ...props
}: SwitchProps) {
  return (
    <button
      type={type}
      role="switch"
      aria-checked={checked}
      data-size={size}
      data-tint={tint}
      className={cx('dz-switch', className)}
      onClick={event => {
        onClick?.(event)
        if (!event.defaultPrevented) onCheckedChange?.(!checked)
      }}
      {...props}
    >
      <span aria-hidden className="dz-switch__thumb" />
    </button>
  )
}
