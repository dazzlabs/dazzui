import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type AlertBannerSize = 'small' | 'medium' | 'large'

export interface AlertBannerProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'title'
> {
  size?: AlertBannerSize
  tint?: Tint
  /** Leading node — an icon or a spinner. */
  icon?: ReactNode
  /** Omit for a one-line notice; the description then carries it alone. */
  title?: ReactNode
  /** Trailing affordance, pushed to the far edge. */
  action?: ReactNode
  /** The description. */
  children?: ReactNode
}

export function AlertBanner({
  size = 'medium',
  tint = 'info',
  icon,
  title,
  action,
  className,
  children,
  ...props
}: AlertBannerProps) {
  return (
    <div
      data-size={size}
      data-tint={tint}
      className={cx('dz-alert-banner', className)}
      {...props}
    >
      {icon ? <span className="dz-alert-banner__icon">{icon}</span> : null}
      <div className="dz-alert-banner__body">
        {title ? <span className="dz-alert-banner__title">{title}</span> : null}
        {children ? (
          <span className="dz-alert-banner__description">{children}</span>
        ) : null}
      </div>
      {action ? (
        <span className="dz-alert-banner__action">{action}</span>
      ) : null}
    </div>
  )
}
