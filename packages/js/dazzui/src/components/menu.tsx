import { ContextMenu as ContextBase } from '@base-ui/react/context-menu'
import { Menu as Base } from '@base-ui/react/menu'
import { Menubar as BaseMenubar } from '@base-ui/react/menubar'
import { mergeProps } from '@base-ui/react/merge-props'
import { Checkmark12Filled, ChevronRight12Regular } from '@fluentui/react-icons'
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
   * unchecked ones, so the leading column stays aligned. Checks are a
   * flat-list feature: a list holding a submenu renders plain rows.
   */
  checked?: boolean
  disabled?: boolean
  onSelect?: () => void
  /**
   * What a right arrow opens: the row becomes a branch wearing the chevron,
   * and its list hangs beside it, itself holding branches — a menu nests as
   * deep as its data does.
   */
  items?: MenuItem[]
}

/** A trigger element, or a function of the open state that returns one. */
export type MenuTrigger =
  | ReactElement<Record<string, unknown>>
  | ((state: { open: boolean }) => ReactElement<Record<string, unknown>>)

/**
 * The rows, and the radio group they need when any of them is checkable.
 *
 * It is a function rather than a component so the three menus can each put it
 * inside their own popup: a `ContextMenu`'s parts *are* the menu's — Base UI
 * re-exports them — so one row renderer serves all three, and a right-click
 * menu cannot drift from the one a button opens.
 *
 * A row holding `items` is a branch: a `SubmenuTrigger` whose list hangs
 * beside it, itself drawn by this function — Base UI does the opening on
 * hover, the arrows across levels, and the Escape that closes one level.
 */
function menuRows(
  items: MenuItem[],
  tint: Tint,
  className?: string
): ReactNode {
  const checkedIndex = items.findIndex(item => item.checked)
  const hasChecks = items.some(item => item.checked !== undefined)
  const hasBranches = items.some(item => item.items?.length)

  const rows = items.map((item, index) => {
    const branch = item.items?.length
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
        {branch ? (
          <span aria-hidden className="dz-popup__chevron">
            <ChevronRight12Regular />
          </span>
        ) : null}
      </>
    )

    if (branch) {
      return (
        <Base.SubmenuRoot key={item.label}>
          <Base.SubmenuTrigger
            disabled={item.disabled}
            className="dz-popup__item"
          >
            {content}
          </Base.SubmenuTrigger>
          <Base.Portal>
            <Base.Positioner
              side="inline-end"
              align="start"
              className="dz-popup-positioner"
            >
              <Base.Popup
                data-tint={tint}
                className={cx('dz-popup', 'dz-menu', className)}
              >
                {menuRows(item.items!, tint, className)}
              </Base.Popup>
            </Base.Positioner>
          </Base.Portal>
        </Base.SubmenuRoot>
      )
    }

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

  // The radio group is a flat list's shape: a `SubmenuTrigger` is not a radio
  // choice, and a list holding one renders plain rows.
  return hasChecks && !hasBranches ? (
    <Base.RadioGroup
      value={checkedIndex}
      onValueChange={(next: number) => items[next]?.onSelect?.()}
    >
      {rows}
    </Base.RadioGroup>
  ) : (
    rows
  )
}

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
            {menuRows(items, tint, className)}
          </Base.Popup>
        </Base.Positioner>
      </Base.Portal>
    </Base.Root>
  )
}

export interface ContextMenuProps {
  items: MenuItem[]
  tint?: Tint
  /**
   * The region that opens the menu. It is rendered as given — the trigger is
   * a listener on it, not a wrapper around it — so a row, a card or a canvas
   * keeps its own layout.
   */
  children: ReactElement<Record<string, unknown>>
  /** Applied to the panel. */
  className?: string
}

/**
 * The menu a right-click opens, over the thing that was clicked.
 *
 * It is the same menu — Base UI re-exports the popup, the positioner and
 * every row type from `Menu`, so this shares the drawing and the row renderer
 * rather than resembling them. What is its own is where it opens: at the
 * pointer, not against a trigger's edge, and on a long press as well as a
 * right-click so it is reachable on a touch screen.
 */
export function ContextMenu({
  items,
  tint = 'neutral',
  children,
  className,
}: ContextMenuProps) {
  return (
    <ContextBase.Root>
      <ContextBase.Trigger render={children} />
      <ContextBase.Portal>
        <ContextBase.Positioner className="dz-popup-positioner">
          <ContextBase.Popup
            data-tint={tint}
            className={cx('dz-popup', 'dz-menu', className)}
          >
            {menuRows(items, tint, className)}
          </ContextBase.Popup>
        </ContextBase.Positioner>
      </ContextBase.Portal>
    </ContextBase.Root>
  )
}

export interface MenubarMenu {
  /** The name on the bar — File, Edit, View. */
  label: string
  items: MenuItem[]
  disabled?: boolean
}

export interface MenubarProps extends Pick<
  BaseMenubar.Props,
  'modal' | 'disabled' | 'orientation'
> {
  menus: MenubarMenu[]
  tint?: Tint
  className?: string
}

/**
 * The application menu bar: several menus that behave as one.
 *
 * That is the whole reason it exists rather than a row of `Menu`s. Once one
 * of them is open, moving the pointer along the bar opens the next without a
 * second click, and the arrow keys walk the bar as well as the rows — which
 * is the behaviour every desktop menu bar has and none of them documents.
 *
 * Each menu is a `Menu`, so the panel, the rows and the checkmarks are the
 * same drawing as everywhere else; the bar adds only its triggers.
 */
export function Menubar({
  menus,
  tint = 'neutral',
  modal,
  disabled,
  orientation,
  className,
}: MenubarProps) {
  return (
    <BaseMenubar
      modal={modal}
      disabled={disabled}
      orientation={orientation}
      /* The triggers hover on the `normal` recipe, and every
         `--control-color-*` is declared inside `[data-tint]`. */
      data-tint="neutral"
      className={cx('dz-menubar', className)}
    >
      {menus.map(menu => (
        <Menu
          key={menu.label}
          align="start"
          items={menu.items}
          tint={tint}
          trigger={
            <button
              type="button"
              disabled={menu.disabled}
              className="dz-menubar__item"
            >
              {menu.label}
            </button>
          }
        />
      ))}
    </BaseMenubar>
  )
}
