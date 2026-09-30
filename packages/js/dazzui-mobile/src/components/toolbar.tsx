import { Toolbar as Base } from '@base-ui/react/toolbar'
import type { ComponentProps, ReactNode } from 'react'

import { cx } from '../lib/cx'
import { usePlatform } from '../lib/theme'
import { IconButton, type IconButtonProps } from './icon-button'

export interface ToolbarProps {
  /** `ToolbarGroup`s, with `ToolbarSpacer`s between them. */
  children: ReactNode
  /**
   * Material's two floating toolbar colours: the surface container, or the
   * primary container. iOS ignores it.
   */
  variant?: 'standard' | 'vibrant'
  /**
   * A `Fab` beside the toolbar, as Material pairs them — the secondary
   * container beside a standard toolbar, the tertiary beside a vibrant
   * one. iOS ignores it; its toolbar carries the primary action itself.
   */
  fab?: ReactNode
  /**
   * `sticky` floats it over the foot of a `Screen`, passed as its
   * `tabBar`; `static` lays it in the flow where it is written.
   */
  position?: 'sticky' | 'static'
  'aria-label'?: string
  className?: string
}

/**
 * The bar of actions at the foot of a screen that has no tab bar. On iOS
 * it is UIToolbar as Liquid Glass floats it: its items gathered into glass
 * capsules, a flexible space between them, 16pt off the edges. On Android
 * it is Material Expressive's floating toolbar: one capsule of icon
 * buttons centred above the foot, a FAB beside it if there is one. Base
 * UI's toolbar roves the focus across every item.
 */
export function Toolbar({
  variant = 'standard',
  fab,
  position = 'sticky',
  className,
  children,
  ...props
}: ToolbarProps) {
  const platform = usePlatform()
  return (
    <div
      data-platform={platform}
      data-variant={variant}
      data-position={position}
      className={cx('dzm-toolbar', className)}
    >
      <Base.Root aria-label={props['aria-label']} className="dzm-toolbar__bar">
        {children}
      </Base.Root>
      {platform === 'android' && fab ? (
        <div className="dzm-toolbar__fab">{fab}</div>
      ) : null}
    </div>
  )
}

export interface ToolbarGroupProps extends ComponentProps<'div'> {
  children: ReactNode
}

/**
 * Items that belong together. On iOS a group is one glass capsule; on
 * Android the groups flow on inside the toolbar's single capsule.
 */
export function ToolbarGroup({
  className,
  children,
  ...props
}: ToolbarGroupProps) {
  const platform = usePlatform()
  return (
    <Base.Group
      data-platform={platform}
      className={cx(
        'dzm-toolbar__group',
        platform === 'ios' && 'dzm-glass',
        className
      )}
      {...props}
    >
      {children}
    </Base.Group>
  )
}

/**
 * The space between groups: UIKit's flexible space, pushing what follows
 * to the trailing edge, on iOS; a small gap inside the capsule on Android.
 */
export function ToolbarSpacer({ className }: { className?: string }) {
  const platform = usePlatform()
  return (
    <div
      aria-hidden
      data-platform={platform}
      className={cx('dzm-toolbar__spacer', className)}
    />
  )
}

export type ToolbarButtonProps = IconButtonProps

/**
 * One action: an `IconButton`, `plain` by default — a bare glyph in the
 * toolbar's ink — and at the toolbar's own size, 44pt on iOS and 40dp on
 * Android. A `filled` one is the prominent action.
 */
export function ToolbarButton({
  variant = 'plain',
  size,
  disabled,
  children,
  ...props
}: ToolbarButtonProps) {
  const platform = usePlatform()
  return (
    <Base.Button
      disabled={disabled}
      focusableWhenDisabled={false}
      render={
        <IconButton
          variant={variant}
          size={size ?? (platform === 'ios' ? 'medium' : 'small')}
          {...props}
        >
          {children}
        </IconButton>
      }
    />
  )
}
