import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type NavItemSize = 'small' | 'medium'

export interface NavItemProps extends ComponentPropsWithoutRef<'button'> {
  /** Marks this row as the page or pane currently shown. */
  active?: boolean
  /** Leading glyph. It follows the row's color, so it tints with selection. */
  icon?: ReactNode
  /** `medium` in a sidebar, `small` in the narrower rail. */
  size?: NavItemSize
  tint?: Tint
}

/** One row of a navigation column. */
export function NavItem({
  active = false,
  icon,
  size = 'medium',
  tint = 'primary',
  className,
  children,
  type = 'button',
  ...props
}: NavItemProps) {
  return (
    <button
      type={type}
      aria-current={active ? 'page' : undefined}
      data-size={size}
      data-tint={tint}
      className={cx('dz-nav-item', className)}
      {...props}
    >
      {icon ? (
        <span aria-hidden className="dz-nav-item__icon">
          {icon}
        </span>
      ) : null}
      {children}
    </button>
  )
}

export interface NavActionProps extends Omit<NavItemProps, 'active' | 'icon'> {}

/**
 * The trailing action pinned to the foot of a navigation column — "New", "Add
 * a provider". Shaped like a `NavItem` so the column reads as one list, but
 * printed in the ramp rather than filled with it, because it adds rather than
 * selects.
 */
export function NavAction({ className, ...props }: NavActionProps) {
  return <NavItem className={cx('dz-nav-action', className)} {...props} />
}
