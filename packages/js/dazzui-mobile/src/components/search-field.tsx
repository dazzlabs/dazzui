import { Input } from '@base-ui/react/input'
import { type ReactNode, useState } from 'react'

import { cx } from '../lib/cx'
import { ClearCircle, Search } from '../lib/glyphs'
import { usePlatform } from '../lib/theme'

export interface SearchFieldProps extends Omit<
  Input.Props,
  'className' | 'render' | 'value' | 'defaultValue' | 'onValueChange' | 'type'
> {
  value?: string
  defaultValue?: string
  onValueChange?: (value: string) => void
  /** Material's trailing slot — an avatar, a microphone. */
  trailing?: ReactNode
  className?: string
}

/**
 * UISearchBar's field, as the glass capsule iOS 26 made it, and Material's
 * search bar: a magnifier, the query, and a way to clear it.
 */
export function SearchField({
  value,
  defaultValue,
  onValueChange,
  trailing,
  className,
  placeholder = 'Search',
  ...props
}: SearchFieldProps) {
  const platform = usePlatform()
  const [inner, setInner] = useState(defaultValue ?? '')
  const text = value ?? inner
  const set = (next: string) => {
    if (value === undefined) setInner(next)
    onValueChange?.(next)
  }
  return (
    <div
      data-platform={platform}
      data-filled={text ? '' : undefined}
      className={cx('dzm-search', platform === 'ios' && 'dzm-glass', className)}
    >
      <span className="dzm-search__glyph">
        <Search />
      </span>
      <Input
        type="search"
        className="dzm-search__input"
        value={text}
        onValueChange={set}
        placeholder={placeholder}
        {...props}
      />
      {text ? (
        <button
          type="button"
          aria-label="Clear search"
          className="dzm-search__clear"
          onClick={() => set('')}
        >
          <ClearCircle />
        </button>
      ) : null}
      {trailing ? (
        <span className="dzm-search__trailing">{trailing}</span>
      ) : null}
    </div>
  )
}
