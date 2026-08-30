import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export interface OptionCardProps extends Omit<
  ComponentPropsWithoutRef<'button'>,
  'title' | 'children'
> {
  title: ReactNode
  description?: ReactNode
  selected?: boolean
  tint?: Tint
}

/** One choice in a radio group of cards. Wrap a set in `role="radiogroup"`. */
export function OptionCard({
  title,
  description,
  selected = false,
  tint = 'primary',
  className,
  type = 'button',
  ...props
}: OptionCardProps) {
  return (
    <button
      type={type}
      role="radio"
      aria-checked={selected}
      data-tint={tint}
      className={cx('dz-option-card', className)}
      {...props}
    >
      <span className="dz-option-card__title">{title}</span>
      {description ? (
        <span className="dz-option-card__description">{description}</span>
      ) : null}
    </button>
  )
}
