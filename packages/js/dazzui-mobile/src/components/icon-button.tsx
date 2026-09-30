import type { ComponentProps, ReactNode } from 'react'

import { cx } from '../lib/cx'
import { usePlatform } from '../lib/theme'

/**
 * `glass` is the circle a navigation bar's actions sit in on iOS; on
 * Android it is the standard icon button, which floats on the surface the
 * same way. `plain` is the bare tinted glyph. `filled`, `tonal` and
 * `outlined` are Material's three others, and iOS draws them as the button
 * styles of the same names in a circle.
 */
export type IconButtonVariant =
  | 'glass'
  | 'plain'
  | 'filled'
  | 'tonal'
  | 'outlined'

export type IconButtonSize = 'small' | 'medium' | 'large'

export interface IconButtonProps extends Omit<
  ComponentProps<'button'>,
  'children'
> {
  /** The glyph. */
  children: ReactNode
  /** Its accessible name — the glyph has none. */
  label: string
  variant?: IconButtonVariant
  size?: IconButtonSize
  tone?: 'default' | 'destructive'
  /** Material Expressive's shape family; iOS is always a circle. */
  shape?: 'round' | 'square'
}

export function IconButton({
  label,
  variant = 'glass',
  size = 'medium',
  tone = 'default',
  shape = 'round',
  className,
  children,
  type = 'button',
  ...props
}: IconButtonProps) {
  const platform = usePlatform()
  return (
    <button
      type={type}
      aria-label={label}
      data-platform={platform}
      data-variant={variant}
      data-size={size}
      data-tone={tone}
      data-shape={shape}
      className={cx(
        'dzm-icon-button',
        variant === 'glass' && 'dzm-glass',
        'dzm-m3-layer',
        className
      )}
      {...props}
    >
      {children}
    </button>
  )
}
