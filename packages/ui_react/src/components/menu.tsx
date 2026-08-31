import { Checkmark12Filled } from '@fluentui/react-icons'
import { type ReactNode, useEffect, useRef, useState } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export interface MenuItem {
  label: string
  icon?: ReactNode
  /** Trailing hint — pass a `Kbd`, or the glyphs to set in one. */
  shortcut?: ReactNode
  /**
   * Radio-style check state. Set it on every item of a group, including the
   * unchecked ones, so the leading column stays aligned.
   */
  checked?: boolean
  disabled?: boolean
  onSelect?: () => void
}

export interface MenuProps {
  /** Renders the trigger; receives the open state and a toggle handler. */
  trigger: (state: { open: boolean; onClick: () => void }) => ReactNode
  items: MenuItem[]
  align?: 'start' | 'end'
  tint?: Tint
  className?: string
}

/**
 * A popover menu. It closes on an outside pointer press and on Escape, so it
 * stays operable from the keyboard as well as the mouse.
 */
export function Menu({
  trigger,
  items,
  align = 'end',
  tint = 'neutral',
  className,
}: MenuProps) {
  const [open, setOpen] = useState(false)
  const root = useRef<HTMLDivElement>(null)

  useEffect(() => {
    if (!open) return

    const onPointerDown = (event: PointerEvent) => {
      if (!root.current?.contains(event.target as Node)) setOpen(false)
    }
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') setOpen(false)
    }

    document.addEventListener('pointerdown', onPointerDown)
    document.addEventListener('keydown', onKeyDown)
    return () => {
      document.removeEventListener('pointerdown', onPointerDown)
      document.removeEventListener('keydown', onKeyDown)
    }
  }, [open])

  return (
    <div ref={root} data-tint={tint} className={cx('dz-menu', className)}>
      {trigger({ open, onClick: () => setOpen(value => !value) })}
      {open ? (
        <div role="menu" data-align={align} className="dz-menu__panel">
          {items.map(item => (
            <button
              key={item.label}
              type="button"
              role={item.checked === undefined ? 'menuitem' : 'menuitemradio'}
              aria-checked={item.checked}
              disabled={item.disabled}
              className="dz-menu__item"
              onClick={() => {
                item.onSelect?.()
                setOpen(false)
              }}
            >
              {item.checked === undefined ? null : (
                <span
                  aria-hidden
                  data-checked={item.checked}
                  className="dz-menu__check"
                >
                  <Checkmark12Filled />
                </span>
              )}
              {item.icon ? (
                <span className="dz-menu__icon">{item.icon}</span>
              ) : null}
              <span className="dz-menu__label">{item.label}</span>
              {item.shortcut ? (
                <span className="dz-menu__shortcut">{item.shortcut}</span>
              ) : null}
            </button>
          ))}
        </div>
      ) : null}
    </div>
  )
}
