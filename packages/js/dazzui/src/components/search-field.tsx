import { Input } from '@base-ui/react/input'
import { Dismiss12Regular, Search16Regular } from '@fluentui/react-icons'
import { type ReactNode, useEffect, useRef } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'
import type { TextFieldSize } from './text-field'

export interface SearchFieldProps {
  value: string
  onValueChange: (value: string) => void
  placeholder?: string
  /** Trailing hint shown while the field is empty — usually a `KeyCap`. */
  hint?: ReactNode
  size?: TextFieldSize
  tint?: Tint
  /** Focus on mount, for a field opened by a shortcut. */
  autoFocus?: boolean
  /** Called on Escape once the field is already empty. */
  onDismiss?: () => void
  /** Accessible name of the clear button. */
  clearLabel?: string
  'aria-label'?: string
  className?: string
}

/** Inline search for a list or a table. */
export function SearchField({
  value,
  onValueChange,
  placeholder = 'Search',
  hint,
  size = 'medium',
  tint = 'primary',
  autoFocus = false,
  onDismiss,
  clearLabel = 'Clear search',
  'aria-label': ariaLabel,
  className,
}: SearchFieldProps) {
  const input = useRef<HTMLInputElement>(null)

  useEffect(() => {
    if (autoFocus) input.current?.focus()
  }, [autoFocus])

  return (
    <div
      data-size={size}
      data-tint={tint}
      className={cx('dz-search-field', className)}
    >
      <Search16Regular aria-hidden className="dz-search-field__glyph" />
      <Input
        ref={input}
        type="search"
        value={value}
        aria-label={ariaLabel ?? placeholder}
        placeholder={placeholder}
        className="dz-search-field__input"
        onValueChange={onValueChange}
        onKeyDown={event => {
          if (event.key !== 'Escape') return
          event.preventDefault()
          // Escape clears first and dismisses only once there is nothing left,
          // so a stray press never closes a field mid-search.
          if (value) onValueChange('')
          else onDismiss?.()
        }}
      />
      {value ? (
        <button
          type="button"
          aria-label={clearLabel}
          className="dz-search-field__clear"
          onClick={() => onValueChange('')}
        >
          <Dismiss12Regular />
        </button>
      ) : (
        hint
      )}
    </div>
  )
}
