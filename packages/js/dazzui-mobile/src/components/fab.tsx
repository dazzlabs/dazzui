import type { ComponentProps, ReactNode } from 'react'

import { cx } from '../lib/cx'
import { usePlatform } from '../lib/theme'

export interface FabProps extends Omit<ComponentProps<'button'>, 'children'> {
  icon: ReactNode
  /** With a label the FAB is extended. */
  children?: ReactNode
  /** Its accessible name, when there is no label. */
  label?: string
  size?: 'small' | 'medium' | 'large'
  /** Material's colour: the primary container by default. */
  tone?: 'primary' | 'secondary' | 'tertiary' | 'surface'
}

/**
 * Material's floating action button. iOS has none; the nearest thing is
 * the glass circle Maps and Photos float over their content, and that is
 * what it draws — a control the two platforms only share on the web.
 */
export function Fab({
  icon,
  label,
  size = 'medium',
  tone = 'primary',
  className,
  children,
  type = 'button',
  ...props
}: FabProps) {
  const platform = usePlatform()
  return (
    <button
      type={type}
      aria-label={children ? undefined : label}
      data-platform={platform}
      data-size={size}
      data-tone={tone}
      data-extended={children ? '' : undefined}
      className={cx(
        'dzm-fab',
        platform === 'ios' && 'dzm-glass',
        'dzm-m3-layer',
        className
      )}
      {...props}
    >
      <span className="dzm-fab__icon">{icon}</span>
      {children ? <span className="dzm-fab__label">{children}</span> : null}
    </button>
  )
}
