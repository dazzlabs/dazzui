import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'
import type { ButtonSize, ButtonVariant } from './button'

export interface IconButtonProps extends Omit<
  ComponentPropsWithoutRef<'button'>,
  'children'
> {
  /** Accessible name, also surfaced as the native tooltip. */
  label: string
  size?: ButtonSize
  variant?: ButtonVariant
  tint?: Tint
  /** A persistent on-state — a pin held down, a panel kept open. */
  active?: boolean
  children: ReactNode
}

export function IconButton({
  label,
  size = 'medium',
  variant = 'plain',
  // Neutral by default: a toolbar of these should read as chrome, not as a row
  // of accented actions. Pass a tint where one button genuinely leads.
  tint = 'neutral',
  active,
  className,
  children,
  type = 'button',
  ...props
}: IconButtonProps) {
  return (
    <button
      type={type}
      aria-label={label}
      title={label}
      aria-pressed={active}
      data-control=""
      data-size={size}
      data-tint={tint}
      data-variant={variant}
      className={cx('dz-icon-button', className)}
      {...props}
    >
      {children}
    </button>
  )
}
