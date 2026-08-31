import { Checkmark12Filled } from '@fluentui/react-icons'
import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type CheckboxSize = 'small' | 'medium' | 'large'

export interface CheckboxProps extends Omit<
  ComponentPropsWithoutRef<'button'>,
  'onChange'
> {
  checked: boolean
  onCheckedChange?: (checked: boolean) => void
  size?: CheckboxSize
  tint?: Tint
  /** A de-emphasised aside on the label — a count, a size. */
  note?: ReactNode
}

export function Checkbox({
  checked,
  onCheckedChange,
  size = 'medium',
  tint = 'primary',
  note,
  className,
  children,
  onClick,
  type = 'button',
  ...props
}: CheckboxProps) {
  return (
    <button
      type={type}
      role="checkbox"
      aria-checked={checked}
      data-size={size}
      data-tint={tint}
      className={cx('dz-checkbox', className)}
      onClick={event => {
        onClick?.(event)
        if (!event.defaultPrevented) onCheckedChange?.(!checked)
      }}
      {...props}
    >
      <span aria-hidden className="dz-checkbox__box">
        {checked ? <Checkmark12Filled /> : null}
      </span>
      <span className="dz-checkbox__label">
        {children}
        {note ? <span className="dz-checkbox__note"> {note}</span> : null}
      </span>
    </button>
  )
}
