import { type ReactNode, useEffect, useRef } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'
import type { InputSize } from './input'

export interface SearchFieldProps {
  value: string
  onValueChange: (value: string) => void
  placeholder?: string
  /** Trailing hint shown while the field is empty — usually a `Kbd`. */
  hint?: ReactNode
  size?: InputSize
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
      <span aria-hidden className="dz-search-field__glyph">
        <svg viewBox="0 0 14 14" width="13" height="13">
          <circle
            cx="6"
            cy="6"
            r="4.25"
            fill="none"
            stroke="currentColor"
            strokeWidth="1.5"
          />
          <path
            d="M9.2 9.2 12.5 12.5"
            fill="none"
            stroke="currentColor"
            strokeWidth="1.5"
            strokeLinecap="round"
          />
        </svg>
      </span>
      <input
        ref={input}
        type="search"
        value={value}
        aria-label={ariaLabel ?? placeholder}
        placeholder={placeholder}
        className="dz-search-field__input"
        onChange={event => onValueChange(event.target.value)}
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
          ✕
        </button>
      ) : (
        hint
      )}
    </div>
  )
}
