import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type ButtonSize = 'small' | 'medium' | 'large'

/**
 * The `control.color.*` recipes, which is the whole set: `filled` is the one
 * action a view points at, `normal` and `outlined` are its neighbours,
 * `tinted` is a quiet chip, and `plain` is text alone.
 */
export type ButtonVariant =
  | 'normal'
  | 'filled'
  | 'tinted'
  | 'outlined'
  | 'plain'

export interface ButtonProps extends ComponentPropsWithoutRef<'button'> {
  size?: ButtonSize
  variant?: ButtonVariant
  tint?: Tint
  fullWidth?: boolean
  /** Trailing hint — the key that also triggers this action. */
  shortcut?: ReactNode
}

export function Button({
  size = 'medium',
  variant = 'normal',
  tint = 'primary',
  fullWidth = false,
  shortcut,
  className,
  children,
  type = 'button',
  ...props
}: ButtonProps) {
  return (
    <button
      type={type}
      data-control=""
      data-full-width={fullWidth}
      data-size={size}
      data-tint={tint}
      data-variant={variant}
      className={cx('dz-button', className)}
      {...props}
    >
      {children}
      {shortcut ? (
        <span className="dz-button__shortcut">{shortcut}</span>
      ) : null}
    </button>
  )
}
