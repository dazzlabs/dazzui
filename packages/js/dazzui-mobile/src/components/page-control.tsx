import { type KeyboardEvent, useRef, useState } from 'react'

import { cx } from '../lib/cx'
import { usePlatform } from '../lib/theme'

export interface PageControlProps {
  /** How many pages there are. */
  count: number
  value?: number
  defaultValue?: number
  onValueChange?: (value: number) => void
  /**
   * UIPageControl's prominent background: the dots in a glass capsule of
   * their own, for a control laid over content. Android ignores it.
   */
  background?: boolean
  'aria-label'?: string
  className?: string
}

/**
 * UIPageControl — a row of dots, the current page's in the label colour —
 * and the row Google's own apps draw where Material has no component: the
 * current page stretched into a pill in the primary colour, the others
 * dots in the outline variant. Each dot is a button; the current one
 * holds the focus, and the arrow keys move the page.
 */
export function PageControl({
  count,
  value,
  defaultValue = 0,
  onValueChange,
  background = false,
  className,
  ...props
}: PageControlProps) {
  const platform = usePlatform()
  const [own, setOwn] = useState(defaultValue)
  const current = clamp(value ?? own, count)
  const dots = useRef<(HTMLButtonElement | null)[]>([])

  const select = (next: number, focus = false) => {
    const page = clamp(next, count)
    if (focus) dots.current[page]?.focus()
    if (page === current) return
    if (value === undefined) setOwn(page)
    onValueChange?.(page)
  }

  const onKeyDown = (event: KeyboardEvent) => {
    const step =
      event.key === 'ArrowRight' || event.key === 'ArrowDown'
        ? 1
        : event.key === 'ArrowLeft' || event.key === 'ArrowUp'
          ? -1
          : 0
    const target =
      event.key === 'Home'
        ? 0
        : event.key === 'End'
          ? count - 1
          : step
            ? current + step
            : null
    if (target === null) return
    event.preventDefault()
    select(target, true)
  }

  return (
    <div
      role="group"
      aria-label={props['aria-label'] ?? 'Page'}
      data-platform={platform}
      data-background={background ? '' : undefined}
      className={cx(
        'dzm-page-control',
        background && platform === 'ios' && 'dzm-glass',
        className
      )}
      onKeyDown={onKeyDown}
    >
      {Array.from({ length: count }, (_, page) => (
        <button
          key={page}
          ref={element => {
            dots.current[page] = element
          }}
          type="button"
          tabIndex={page === current ? 0 : -1}
          aria-label={`Page ${page + 1} of ${count}`}
          aria-current={page === current ? 'true' : undefined}
          data-current={page === current ? '' : undefined}
          className="dzm-page-control__dot"
          onClick={() => select(page)}
        />
      ))}
    </div>
  )
}

function clamp(page: number, count: number) {
  return Math.max(0, Math.min(count - 1, Math.round(page)))
}
