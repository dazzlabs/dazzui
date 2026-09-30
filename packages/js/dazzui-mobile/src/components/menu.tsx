import { Menu as Base } from '@base-ui/react/menu'
import { mergeProps } from '@base-ui/react/merge-props'
import { cloneElement, type ReactElement, type ReactNode } from 'react'

import { cx } from '../lib/cx'
import { hasChecks, MenuItems } from '../lib/menu-items'
import { usePlatform, useThemeContainer } from '../lib/theme'

export interface MenuItem {
  label: ReactNode
  /** iOS sets a menu's glyphs at the trailing edge; Material leads with them. */
  icon?: ReactNode
  /** A tick beside the label. Set it on every item of a group, so the column stays. */
  checked?: boolean
  role?: 'default' | 'destructive'
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
  trigger: MenuTrigger
  /** Items, with `null` between two for a separator. */
  items: readonly (MenuItem | null)[]
  align?: 'start' | 'center' | 'end'
  side?: 'top' | 'bottom' | 'left' | 'right'
  onOpenChange?: (open: boolean) => void
  className?: string
}

/**
 * UIMenu — the glass panel a button pulls down — and Material's menu on
 * its surface container. Base UI positions it, roves the focus, closes it
 * on an outside press and on Escape, and types ahead.
 */
export function Menu({
  trigger,
  items,
  align = 'end',
  side = 'bottom',
  onOpenChange,
  className,
  ...root
}: MenuProps) {
  const platform = usePlatform()
  const container = useThemeContainer()
  const checks = hasChecks(items)
  return (
    <Base.Root onOpenChange={onOpenChange} {...root}>
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
      <Base.Portal container={container}>
        <Base.Positioner
          align={align}
          side={side}
          sideOffset={platform === 'ios' ? 8 : 4}
          data-platform={platform}
          className="dzm-menu-positioner"
        >
          <Base.Popup
            data-platform={platform}
            data-checks={checks || undefined}
            className={cx(
              'dzm-menu',
              platform === 'ios' && 'dzm-glass dzm-glass--thick',
              className
            )}
          >
            <MenuItems items={items} checks={checks} />
          </Base.Popup>
        </Base.Positioner>
      </Base.Portal>
    </Base.Root>
  )
}
