import { Tabs } from '@base-ui/react/tabs'
import type { ReactNode } from 'react'

import { cx } from '../lib/cx'
import { Search } from '../lib/glyphs'
import { usePlatform } from '../lib/theme'
import { Badge } from './badge'

export interface TabBarItem {
  value: string
  label: ReactNode
  icon: ReactNode
  /** A count on the item; `true` for a dot. */
  badge?: number | boolean
}

export interface TabBarProps {
  items: readonly TabBarItem[]
  value?: string
  defaultValue?: string
  onValueChange?: (value: string) => void
  /**
   * iOS 26 moved search out of the tabs into a circle of its own beside
   * the bar; pass a handler to draw it. Android ignores it.
   */
  onSearch?: () => void
  'aria-label'?: string
  className?: string
}

/**
 * UITabBar as Liquid Glass floats it — a capsule above the content with a
 * lens over the chosen tab — and Material's navigation bar, an 80dp
 * surface container with a pill behind the chosen icon. Base UI's tabs
 * rove the focus and carry the selection.
 */
export function TabBar({
  items,
  value,
  defaultValue,
  onValueChange,
  onSearch,
  className,
  ...props
}: TabBarProps) {
  const platform = usePlatform()
  return (
    <Tabs.Root
      value={value}
      defaultValue={defaultValue ?? items[0]?.value}
      onValueChange={next => onValueChange?.(String(next))}
      data-platform={platform}
      className={cx('dzm-tab-bar', className)}
    >
      <Tabs.List
        aria-label={props['aria-label']}
        className={cx('dzm-tab-bar__list', platform === 'ios' && 'dzm-glass')}
      >
        <Tabs.Indicator className="dzm-tab-bar__lens" />
        {items.map(item => (
          <Tabs.Tab
            key={item.value}
            value={item.value}
            className="dzm-tab-bar__tab dzm-m3-layer"
          >
            <span className="dzm-tab-bar__icon">
              {item.icon}
              {item.badge ? (
                <Badge
                  className="dzm-tab-bar__badge"
                  dot={item.badge === true}
                  count={
                    typeof item.badge === 'number' ? item.badge : undefined
                  }
                />
              ) : null}
            </span>
            <span className="dzm-tab-bar__label">{item.label}</span>
          </Tabs.Tab>
        ))}
      </Tabs.List>
      {platform === 'ios' && onSearch ? (
        <button
          type="button"
          aria-label="Search"
          className="dzm-tab-bar__search dzm-glass"
          onClick={onSearch}
        >
          <Search />
        </button>
      ) : null}
    </Tabs.Root>
  )
}
