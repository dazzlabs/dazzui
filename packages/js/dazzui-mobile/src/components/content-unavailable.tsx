import type { ComponentProps, ReactNode } from 'react'

import { cx } from '../lib/cx'
import { usePlatform } from '../lib/theme'

export interface ContentUnavailableProps extends Omit<
  ComponentProps<'div'>,
  'title'
> {
  /** The glyph above the title — a host glyph, sized by the view. */
  icon?: ReactNode
  /** What is missing: "No Messages", "No Results for …". */
  title: ReactNode
  /** Why, or what to do about it. */
  description?: ReactNode
  /** The buttons beneath; `children` land here too, after them. */
  actions?: ReactNode
  /** Grows to its container's height and centres in it, as a whole screen. */
  fill?: boolean
}

/**
 * The screen a list shows when it has nothing in it. SwiftUI's
 * `ContentUnavailableView` on iOS: a large glyph in the secondary label,
 * the title in title2 bold, the description beneath in the secondary
 * label, then the actions. Material names no such component; Android
 * draws the empty state Google's own apps draw — the glyph in the surface
 * variant ink, a headline-small title, body-medium beneath, the actions
 * below.
 */
export function ContentUnavailable({
  icon,
  title,
  description,
  actions,
  fill = false,
  className,
  children,
  ...props
}: ContentUnavailableProps) {
  const platform = usePlatform()
  const hasActions = Boolean(actions) || Boolean(children)
  return (
    <div
      data-platform={platform}
      data-fill={fill || undefined}
      className={cx('dzm-content-unavailable', className)}
      {...props}
    >
      {icon ? (
        <span aria-hidden className="dzm-content-unavailable__icon">
          {icon}
        </span>
      ) : null}
      <div className="dzm-content-unavailable__text">
        <div className="dzm-content-unavailable__title">{title}</div>
        {description ? (
          <div className="dzm-content-unavailable__description">
            {description}
          </div>
        ) : null}
      </div>
      {hasActions ? (
        <div className="dzm-content-unavailable__actions">
          {actions}
          {children}
        </div>
      ) : null}
    </div>
  )
}
