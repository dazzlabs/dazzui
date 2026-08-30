import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type PillTabsSize = 'small' | 'medium' | 'large'

export interface PillTabItem<T extends string = string> {
  value: T
  label: ReactNode
  /** Trailing count, set inside the pill. */
  count?: number
  disabled?: boolean
}

export interface PillTabsProps<T extends string = string> extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'onChange' | 'children'
> {
  items: readonly PillTabItem<T>[]
  value: T
  onValueChange?: (value: T) => void
  size?: PillTabsSize
  tint?: Tint
}

export function PillTabs<T extends string = string>({
  items,
  value,
  onValueChange,
  size = 'medium',
  tint = 'primary',
  className,
  ...props
}: PillTabsProps<T>) {
  return (
    <div
      role="tablist"
      data-size={size}
      data-tint={tint}
      className={cx('dz-pill-tabs', className)}
      {...props}
    >
      {items.map(item => (
        <button
          key={item.value}
          type="button"
          role="tab"
          aria-selected={item.value === value}
          disabled={item.disabled}
          className="dz-pill-tabs__tab"
          onClick={() => onValueChange?.(item.value)}
        >
          {item.label}
          {item.count === undefined ? null : (
            <span className="dz-pill-tabs__count">{item.count}</span>
          )}
        </button>
      ))}
    </div>
  )
}
