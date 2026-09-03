import { Tabs } from '@base-ui/react/tabs'
import type { ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export interface PillTabItem<T extends string = string> {
  value: T
  label: ReactNode
  /** Trailing count, set inside the pill. */
  count?: number
  disabled?: boolean
}

export interface PillTabsProps<T extends string = string> extends Omit<
  Tabs.Root.Props,
  | 'className'
  | 'render'
  | 'children'
  | 'value'
  | 'defaultValue'
  | 'onValueChange'
> {
  items: readonly PillTabItem<T>[]
  value?: T
  defaultValue?: T
  onValueChange?: (value: T) => void
  tint?: Tint
  className?: string
}

/**
 * The tab list alone. A caller that has panels to switch owns them: the
 * pills say which is showing, and the content is whatever the caller renders
 * for `value`.
 */
export function PillTabs<T extends string = string>({
  items,
  onValueChange,
  tint = 'primary',
  className,
  ...props
}: PillTabsProps<T>) {
  return (
    <Tabs.Root
      data-tint={tint}
      className={cx('dz-pill-tabs', className)}
      onValueChange={
        onValueChange ? value => onValueChange(value as T) : undefined
      }
      {...props}
    >
      <Tabs.List className="dz-pill-tabs__list">
        {items.map(item => (
          <Tabs.Tab
            key={item.value}
            value={item.value}
            disabled={item.disabled}
            className="dz-pill-tabs__tab"
          >
            {item.label}
            {item.count === undefined ? null : (
              <span className="dz-pill-tabs__count">{item.count}</span>
            )}
          </Tabs.Tab>
        ))}
      </Tabs.List>
    </Tabs.Root>
  )
}
