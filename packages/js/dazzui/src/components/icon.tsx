import type { ComponentType, SVGProps } from 'react'

import { cx } from '../lib/cx'

/**
 * Any component that renders an `<svg>` — the Fluent set the beyondtranslate
 * apps use satisfies this, and so does every other icon library. The system
 * deliberately does not depend on a specific set: an icon is the one thing a
 * product replaces wholesale, and a design system that hard-codes a set makes
 * that a fork rather than a prop.
 */
export type IconGlyph = ComponentType<SVGProps<SVGSVGElement>>

export interface IconProps extends Omit<
  SVGProps<SVGSVGElement>,
  'width' | 'height' | 'ref'
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
