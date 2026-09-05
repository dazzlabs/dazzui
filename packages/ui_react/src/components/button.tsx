import type { ComponentProps, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type ButtonSize = 'tiny' | 'small' | 'medium' | 'large'

/**
 * The `control.color.*` recipes, which is the whole set: `recessed` is the
 * workaday grey chip and the default, `filled` the one action a view points
 * at, `normal` the raised white neighbour, `tinted` a quiet accent chip,
 * `outlined` its drawn twin, and `plain` text alone.
 */
export type ButtonVariant =
  | 'recessed'
  | 'normal'
  | 'filled'
  | 'tinted'
  | 'outlined'
  | 'plain'

export interface ButtonProps extends ComponentProps<'button'> {
  size?: ButtonSize
  variant?: ButtonVariant
  /**
   * Which ramp the recipe resolves against. The neutral recipes —
   * `recessed` and `normal` — read the theme's surface and ink roles rather
   * than a ramp, so it does nothing to them.
   */
  tint?: Tint
  fullWidth?: boolean
  /** Trailing hint — the key that also triggers this action. */
  shortcut?: ReactNode
}

export function Button({
  size = 'small',
  variant = 'recessed',
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
