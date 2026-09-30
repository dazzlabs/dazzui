import { Menu as Base } from '@base-ui/react/menu'

import type { MenuItem } from '../components/menu'
import { Check } from './glyphs'

/** Whether any item carries a tick, so every row keeps the check column. */
export function hasChecks(items: readonly (MenuItem | null)[]): boolean {
  return items.some(item => item?.checked !== undefined)
}

/**
 * The rows of a menu, written once: the pull-down `Menu` and the
 * long-press `ContextMenu` both draw these into a `.dzm-menu` popup. A
 * context menu's items are Base UI's menu items, so the same parts serve
 * both roots.
 */
export function MenuItems({
  items,
  checks,
}: {
  items: readonly (MenuItem | null)[]
  checks: boolean
}) {
  return items.map((item, index) =>
    item === null ? (
      <Base.Separator key={index} className="dzm-menu__separator" />
    ) : (
      <Base.Item
        key={index}
        disabled={item.disabled}
        data-role={item.role ?? 'default'}
        className="dzm-menu__item"
        onClick={item.onSelect}
      >
        {checks ? (
          <span
            className="dzm-menu__check"
            data-checked={item.checked || undefined}
          >
            <Check />
          </span>
        ) : null}
        <span className="dzm-menu__label">{item.label}</span>
        {item.icon ? <span className="dzm-menu__icon">{item.icon}</span> : null}
      </Base.Item>
    )
  )
}
