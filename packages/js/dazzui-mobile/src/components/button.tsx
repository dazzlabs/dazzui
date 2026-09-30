import type { ComponentProps, ReactNode } from 'react'

import { cx } from '../lib/cx'
import { usePlatform } from '../lib/theme'

/**
 * The button styles the two platforms share, named for what they do rather
 * than what either calls them:
 *
 * - `filled`   — the one action a screen points at. SwiftUI's
 *                `.glassProminent`; Material's filled button.
 * - `tonal`    — a quieter action on a tint wash. `.bordered`; the tonal
 *                button.
 * - `outlined` — a stroke and no fill. iOS has no such style and draws a
 *                tinted hairline capsule; Material's outlined button.
 * - `plain`    — the label alone, in the tint. `.borderless`; the text
 *                button.
 * - `glass`    — the Liquid Glass capsule, the ink on the material. On
 *                Android the elevated button, which is the same idea: a
 *                surface lifted off the page rather than a colour on it.
 */
export type ButtonVariant = 'filled' | 'tonal' | 'outlined' | 'plain' | 'glass'

/**
 * `small` is a control size — iOS's `.small`, Material's 40dp default —
 * `medium` the regular row button, and `large` the full-width call to
 * action at the foot of a screen.
 */
export type ButtonSize = 'small' | 'medium' | 'large'

export interface ButtonProps extends ComponentProps<'button'> {
  variant?: ButtonVariant
  size?: ButtonSize
  /** `destructive` reads the danger accent — iOS's role, Material's error. */
  tone?: 'default' | 'destructive'
  /**
   * Material Expressive's shape family: `round` is the capsule and `square`
   * the rounded rectangle, and a pressed round button morphs toward square.
   * iOS is always a capsule.
   */
  shape?: 'round' | 'square'
  fullWidth?: boolean
  /** A leading glyph, sized by the button. */
  icon?: ReactNode
}

export function Button({
  variant = 'tonal',
  size = 'medium',
  tone = 'default',
  shape = 'round',
  fullWidth = false,
  icon,
  className,
  children,
  type = 'button',
  ...props
}: ButtonProps) {
  const platform = usePlatform()
  return (
    <button
      type={type}
      data-platform={platform}
      data-variant={variant}
      data-size={size}
      data-tone={tone}
      data-shape={shape}
      data-full-width={fullWidth || undefined}
      className={cx(
        'dzm-button',
        variant === 'glass' && 'dzm-glass',
        'dzm-m3-layer',
        className
      )}
      {...props}
    >
      {icon ? <span className="dzm-button__icon">{icon}</span> : null}
      {children}
    </button>
  )
}
