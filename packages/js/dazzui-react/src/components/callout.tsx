import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type CalloutSize = 'small' | 'medium' | 'large'

export interface CalloutProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'title'
> {
  size?: CalloutSize
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

export function Callout({
  size = 'medium',
  tint = 'neutral',
  icon,
  title,
  action,
  className,
  children,
  ...props
}: CalloutProps) {
  return (
    <div
      data-size={size}
      data-tint={tint}
      data-has-description={title && children ? true : undefined}
      className={cx('dz-callout', className)}
      {...props}
    >
      {icon ? <span className="dz-callout__icon">{icon}</span> : null}
      <div className="dz-callout__body">
        {title ? <span className="dz-callout__title">{title}</span> : null}
        {children ? (
          <span className="dz-callout__description">{children}</span>
        ) : null}
      </div>
      {action ? <span className="dz-callout__action">{action}</span> : null}
    </div>
  )
}
