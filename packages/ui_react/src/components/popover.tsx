import { Popover as Base } from '@base-ui/react/popover'
import type { ReactElement, ReactNode } from 'react'

import { cx } from '../lib/cx'

export interface PopoverProps extends Pick<
  Base.Root.Props,
  'open' | 'defaultOpen' | 'modal'
> {
  /**
   * What opens it. Base UI wires the press, the keyboard and the `aria-*`,
   * so a plain `Button` or `IconButton` is enough.
   */
  trigger: ReactElement<Record<string, unknown>>
  /** Names the panel to a screen reader as well as heading it. */
  title?: ReactNode
  /** A line under the title, and the panel's accessible description. */
  description?: ReactNode
  side?: 'top' | 'bottom' | 'left' | 'right'
  align?: 'start' | 'center' | 'end'
  /** Overrides the panel's own width, which is otherwise its content's. */
  width?: number | string
  /** Where the panel is rendered. The viewport by default. */
  container?: Base.Portal.Props['container']
  onOpenChange?: (open: boolean) => void
  /** Applied to the panel. */
  className?: string
  children?: ReactNode
}

/**
 * A panel of anything, anchored to what opened it.
 *
 * It is the sibling of `Menu` rather than of `Dialog`: it wears the same
 * floating sheet (popup.css) and is positioned against its trigger, but it
 * holds content rather than rows — a form, a filter, a short explanation —
 * and so it is the one popup that does not roll its focus through items.
 *
 * Reach for a `Dialog` instead when the reader has to finish before doing
 * anything else. A popover closes on an outside press, which makes it the
 * wrong home for something half-typed.
 */
export function Popover({
  trigger,
  title,
  description,
  side = 'bottom',
  align = 'center',
  width,
  container,
  open,
  defaultOpen,
  modal,
  onOpenChange,
  className,
  children,
}: PopoverProps) {
  return (
    <Base.Root
      open={open}
      defaultOpen={defaultOpen}
      modal={modal}
      onOpenChange={onOpenChange ? next => onOpenChange(next) : undefined}
    >
      <Base.Trigger render={trigger} />
      <Base.Portal container={container}>
        <Base.Positioner
          side={side}
          align={align}
          className="dz-popup-positioner"
        >
          <Base.Popup
            data-tint="neutral"
            className={cx('dz-popup', 'dz-popover', className)}
            style={width === undefined ? undefined : { width }}
          >
            {title ? (
              <Base.Title className="dz-popover__title">{title}</Base.Title>
            ) : null}
            {description ? (
              <Base.Description className="dz-popover__description">
                {description}
              </Base.Description>
            ) : null}
            {children}
          </Base.Popup>
        </Base.Positioner>
      </Base.Portal>
    </Base.Root>
  )
}

/** Wraps a control so pressing it closes the popover it sits in. */
export const PopoverClose = Base.Close
