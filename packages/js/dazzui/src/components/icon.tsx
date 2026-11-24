import type { ComponentType, SVGAttributes } from 'react'

import { cx } from '../lib/cx'

/**
 * Any component that renders an `<svg>` — Fluent icons (`SVGAttributes<SVGElement>`)
 * and icon libraries typed against `SVGSVGElement` both satisfy this. The system
 * deliberately does not depend on a specific set: an icon is the one thing a
 * product replaces wholesale, and a design system that hard-codes a set makes
 * that a fork rather than a prop.
 */
export type IconGlyph = ComponentType<SVGAttributes<SVGElement>>

export interface IconProps extends Omit<
  SVGAttributes<SVGElement>,
  'width' | 'height'
> {
  icon: IconGlyph
  /** Rendered edge length. Defaults to the enclosing component's size or the theme's medium icon. */
  size?: number | string
}

/** Draws a glyph at a given size, inheriting the surrounding text color. */
export function Icon({
  icon: Glyph,
  size = 'var(--dz-icon-size, var(--icon-medium))',
  className,
  ...props
}: IconProps) {
  return (
    <Glyph
      aria-hidden
      focusable="false"
      {...props}
      className={cx('dz-icon', className)}
      style={{ width: size, height: size, ...props.style }}
    />
  )
}
