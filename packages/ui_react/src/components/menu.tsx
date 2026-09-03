import { Menu as Base } from '@base-ui/react/menu'
import { mergeProps } from '@base-ui/react/merge-props'
import { Checkmark12Filled } from '@fluentui/react-icons'
import { cloneElement, type ReactElement, type ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export interface MenuItem {
  label: string
  icon?: ReactNode
  /** Trailing hint — pass a `KeyCap`, or the glyphs to set in one. */
  shortcut?: ReactNode
  /**
   * Radio-style check state. Set it on every item of a group, including the
   * unchecked ones, so the leading column stays aligned.
   */
  checked?: boolean
  disabled?: boolean
  onSelect?: () => void
}

/** A trigger element, or a function of the open state that returns one. */
export type MenuTrigger =
  | ReactElement<Record<string, unknown>>
  | ((state: { open: boolean }) => ReactElement<Record<string, unknown>>)

export interface MenuProps extends Pick<
  Base.Root.Props,
  'open' | 'defaultOpen' | 'modal'
> {
  /**
   * What opens the menu. Base UI wires it — the click, the keyboard, the
   * `aria-*` — so a plain `Button` is enough; pass a function to change its
   * look while the menu is open.
   */
  trigger: MenuTrigger
  items: MenuItem[]
  align?: 'start' | 'center' | 'end'
  side?: 'top' | 'bottom' | 'left' | 'right'
  tint?: Tint
  onOpenChange?: (open: boolean) => void
  /** Applied to the panel. */
  className?: string
}

/**
 * A popover menu: Base UI positions it, traps and roves the focus, closes it
 * on an outside press and on Escape, and types ahead. The panel and its rows
 * are the shared popup sheet, so a `Select`'s list is the same drawing.
 */
export function Menu({
  trigger,
  items,
  align = 'end',
  side = 'bottom',
  tint = 'neutral',
  open,
  defaultOpen,
  modal,
  onOpenChange,
  className,
}: MenuProps) {
  const checkedIndex = items.findIndex(item => item.checked)
  const hasChecks = items.some(item => item.checked !== undefined)

  const rows = items.map((item, index) => {
    const content = (
      <>
        {item.checked === undefined ? null : (
          <span aria-hidden className="dz-popup__check">
            <Checkmark12Filled />
          </span>
        )}
        {item.icon ? <span className="dz-popup__icon">{item.icon}</span> : null}
        <span className="dz-popup__label">{item.label}</span>
        {item.shortcut ? (
          <span className="dz-popup__shortcut">{item.shortcut}</span>
        ) : null}
      </>
    )

    return item.checked === undefined ? (
      <Base.Item
        key={item.label}
        disabled={item.disabled}
        className="dz-popup__item"
        onClick={item.onSelect}
      >
        {content}
      </Base.Item>
    ) : (
      <Base.RadioItem
        key={item.label}
        value={index}
        disabled={item.disabled}
        closeOnClick
        className="dz-popup__item"
      >
        {content}
      </Base.RadioItem>
    )
  })

  return (
    <Base.Root
      open={open}
      defaultOpen={defaultOpen}
      modal={modal}
      onOpenChange={onOpenChange ? next => onOpenChange(next) : undefined}
    >
      <Base.Trigger
        render={
          typeof trigger === 'function'
            ? (props, state) => {
                const element = trigger({ open: state.open })
                return cloneElement(element, mergeProps(props, element.props))
              }
            : trigger
        }
      />
      <Base.Portal>
        <Base.Positioner
          side={side}
          align={align}
          className="dz-popup-positioner"
        >
          <Base.Popup
            data-tint={tint}
            className={cx('dz-popup', 'dz-menu', className)}
          >
            {hasChecks ? (
              <Base.RadioGroup
                value={checkedIndex}
                onValueChange={(next: number) => items[next]?.onSelect?.()}
              >
                {rows}
              </Base.RadioGroup>
            ) : (
              rows
            )}
          </Base.Popup>
        </Base.Positioner>
      </Base.Portal>
    </Base.Root>
  )
}
