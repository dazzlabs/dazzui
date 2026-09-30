import { ContextMenu as Base } from '@base-ui/react/context-menu'
import {
  cloneElement,
  type ReactElement,
  type ReactNode,
  type RefObject,
  useLayoutEffect,
  useRef,
  useState,
} from 'react'

import { cx } from '../lib/cx'
import { hasChecks, MenuItems } from '../lib/menu-items'
import { usePlatform, useThemeContainer } from '../lib/theme'
import type { MenuItem } from './menu'

export interface ContextMenuProps {
  /**
   * The element a long press opens the menu from. It is rendered in place
   * and wired as the trigger, so it must pass its props and ref through.
   */
  children: ReactElement<Record<string, unknown>>
  /** Items, with `null` between two for a separator — the `Menu`'s shape. */
  items: readonly (MenuItem | null)[]
  /**
   * iOS only: what lifts in place of the pressed element, on a platter at
   * the menu corner — `UIContextMenuConfiguration`'s preview provider.
   */
  preview?: ReactNode
  onOpenChange?: (open: boolean) => void
  className?: string
}

/**
 * The menu a long press opens; a right click opens it too. On iOS it is
 * `UIContextMenuInteraction`: the pressed element lifts out of the screen
 * over a blurred, dimmed backdrop — or its `preview` does, on a platter —
 * and the glass menu hangs beside it, below or, near the bottom edge,
 * above. On Android it is Material's dropdown menu at the press point, over
 * nothing. The rows are the `Menu`'s, drawn by the same stylesheet; the
 * long press, the focus and the dismissal are Base UI's.
 *
 * The web has no snapshot of a view, so the lifted copy is `children`
 * rendered a second time over the backdrop, at the original's box, while
 * the original steps aside. It keeps its own classes but not its
 * ancestors, so an element that is drawn by a parent's selector should
 * bring a `preview`.
 */
export function ContextMenu({
  children,
  items,
  preview,
  onOpenChange,
  className,
}: ContextMenuProps) {
  const platform = usePlatform()
  const container = useThemeContainer()
  const source = useRef<HTMLDivElement>(null)
  const lift = useRef<HTMLDivElement>(null)
  /*
   * UIKit hangs the menu from the lifted element's leading edge when it
   * sits in the leading half of the screen, and from its trailing edge
   * otherwise.
   */
  const [align, setAlign] = useState<'start' | 'end'>('start')
  const ios = platform === 'ios'
  const checks = hasChecks(items)

  function handleOpenChange(open: boolean) {
    const element = source.current
    if (open && element) {
      const rect = element.getBoundingClientRect()
      const frame = (
        container?.current ?? document.documentElement
      ).getBoundingClientRect()
      setAlign(
        rect.left + rect.width / 2 > frame.left + frame.width / 2
          ? 'end'
          : 'start'
      )
    }
    onOpenChange?.(open)
  }

  return (
    <Base.Root onOpenChange={handleOpenChange}>
      <Base.Trigger
        ref={source}
        render={children}
        data-platform={platform}
        data-preview={preview ? '' : undefined}
        className="dzm-context-menu-trigger"
      />
      <Base.Portal container={container}>
        {ios ? (
          <Base.Backdrop
            data-platform={platform}
            className="dzm-scrim dzm-context-menu-backdrop"
          >
            <Lift source={source} target={lift} preview={Boolean(preview)}>
              {preview || cloneElement(children, { ref: null })}
            </Lift>
          </Base.Backdrop>
        ) : null}
        <Base.Positioner
          anchor={ios ? lift : undefined}
          side={ios ? 'bottom' : undefined}
          align={ios ? align : undefined}
          sideOffset={ios ? 8 : undefined}
          data-platform={platform}
          className="dzm-menu-positioner"
        >
          <Base.Popup
            data-platform={platform}
            data-checks={checks || undefined}
            className={cx(
              'dzm-menu',
              ios && 'dzm-glass dzm-glass--thick',
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

/**
 * What lifts over the backdrop. Without a preview it is the pressed
 * element's copy, laid exactly over the original; with one it is the
 * platter, centred across the screen and level with the original, kept
 * clear of the bars. Measured once as it mounts, in the backdrop's own
 * units, so a theme drawn scaled — a device frame in a gallery — lines up.
 */
function Lift({
  source,
  target,
  preview,
  children,
}: {
  source: RefObject<HTMLElement | null>
  target: RefObject<HTMLDivElement | null>
  preview: boolean
  children: ReactNode
}) {
  useLayoutEffect(() => {
    const element = target.current
    const frame = element?.parentElement
    const from = source.current
    if (!element || !frame || !from || !frame.offsetWidth) return
    const box = frame.getBoundingClientRect()
    const scale = box.width / frame.offsetWidth || 1
    const rect = from.getBoundingClientRect()
    const x = (rect.left - box.left) / scale
    const y = (rect.top - box.top) / scale
    const height = rect.height / scale
    if (!preview) {
      element.style.left = `${x}px`
      element.style.top = `${y}px`
      // Rounded up: a copy a hair narrower than its original wraps.
      element.style.width = `${Math.ceil(rect.width / scale + 0.5)}px`
      element.style.height = `${height}px`
      return
    }
    const style = getComputedStyle(frame)
    const length = (name: string) =>
      parseFloat(style.getPropertyValue(name)) || 0
    const margin = length('--dzm-context-menu-margin')
    const top = length('--dzm-inset-top') + margin
    const bottom =
      frame.offsetHeight -
      length('--dzm-inset-bottom') -
      margin -
      element.offsetHeight
    const centred = y + height / 2 - element.offsetHeight / 2
    element.style.left = `${(frame.offsetWidth - element.offsetWidth) / 2}px`
    element.style.top = `${Math.max(top, Math.min(bottom, centred))}px`
  }, [source, target, preview])

  return (
    <div
      ref={target}
      aria-hidden={preview ? undefined : true}
      data-preview={preview ? '' : undefined}
      className="dzm-context-menu__lift"
    >
      {children}
    </div>
  )
}
