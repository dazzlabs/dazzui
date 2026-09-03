import type { ComponentProps, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'
import type { ButtonSize, ButtonVariant } from './button'
import { Tooltip } from './tooltip'

export interface IconButtonProps extends Omit<
  ComponentProps<'button'>,
  'children'
> {
  /** Accessible name, also surfaced as the tooltip. */
  label: string
  size?: ButtonSize
  variant?: ButtonVariant
  tint?: Tint
  /** A persistent on-state — a pin held down, a panel kept open. */
  active?: boolean
  children: ReactNode
}

/**
 * A `Tooltip` rather than a `title`: the native one is late, unstyled and
 * absent from every keyboard. The button itself is a plain element, which is
 * what lets a `Menu` or a `Dialog` take it as a trigger.
 */
export function IconButton({
  label,
  size = 'tiny',
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
    <Tooltip label={label}>
      <button
        type={type}
        aria-label={label}
        aria-pressed={active || undefined}
        data-control=""
        data-size={size}
        data-tint={tint}
        data-variant={variant}
        className={cx('dz-icon-button', className)}
        {...props}
      >
        {children}
      </button>
    </Tooltip>
  )
}
