import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type CardSize = 'small' | 'medium' | 'large'

export type CardVariant = 'raised' | 'sunken' | 'outlined' | 'tinted'

export type CardPadding = 'default' | 'none'

export interface CardProps extends ComponentPropsWithoutRef<'div'> {
  size?: CardSize
  variant?: CardVariant
  /** Which ramp `tinted` resolves against; ignored by the other variants. */
  tint?: Tint
  /** Set to `'none'` for flush content such as tables or edge-to-edge media. */
  padding?: CardPadding
  /** Highlights the border under the pointer for selectable or clickable cards. */
  interactive?: boolean
}

/** The filled or bordered box the panels in a layout are built from. */
export function Card({
  size = 'medium',
  variant = 'raised',
  tint = 'primary',
  padding = 'default',
  interactive = false,
  className,
  ...props
}: CardProps) {
  return (
    <div
      data-size={size}
      data-tint={tint}
      data-variant={variant}
      data-padding={padding === 'none' ? 'none' : undefined}
      data-interactive={interactive || undefined}
      className={cx('dz-card', className)}
      {...props}
    />
  )
}

export interface CardHeaderProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'title'
> {
  title?: ReactNode
  description?: ReactNode
  /** Trailing actions aligned to the header's opposite edge. */
  actions?: ReactNode
}

/** Optional heading band for a `Card`, with title, description, and trailing actions. */
export function CardHeader({
  title,
  description,
  actions,
  className,
  children,
  ...props
}: CardHeaderProps) {
  return (
    <div className={cx('dz-card__header', className)} {...props}>
      {title || description ? (
        <div className="dz-card__heading">
          {title ? <div className="dz-card__title">{title}</div> : null}
          {description ? (
            <div className="dz-card__description">{description}</div>
          ) : null}
        </div>
      ) : null}
      {children}
      {actions ? <div className="dz-card__actions">{actions}</div> : null}
    </div>
  )
}
