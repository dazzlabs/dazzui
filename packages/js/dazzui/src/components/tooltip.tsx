import { Tooltip as Base } from '@base-ui/react/tooltip'
import type { ReactElement, ReactNode } from 'react'

import { cx } from '../lib/cx'

export interface TooltipProps extends Pick<
  Base.Trigger.Props,
  'delay' | 'closeDelay'
> {
  /** What it says — a short label, not a sentence. */
  label: ReactNode
  /** The control it describes. Base UI listens to it for hover and focus. */
  children: ReactElement<Record<string, unknown>>
  side?: 'top' | 'bottom' | 'left' | 'right'
  align?: 'start' | 'center' | 'end'
  disabled?: boolean
  /** Applied to the panel. */
  className?: string
}

/**
 * The label a control surfaces on hover. It never carries anything a reader
 * has to have: a tooltip is a reminder of what an icon means, and the
 * accessible name is on the control itself.
 */
export function Tooltip({
  label,
  children,
  side = 'top',
  align = 'center',
  delay,
  closeDelay,
  disabled,
  className,
}: TooltipProps) {
  return (
    <Base.Root disabled={disabled}>
      <Base.Trigger render={children} delay={delay} closeDelay={closeDelay} />
      <Base.Portal>
        <Base.Positioner
          side={side}
          align={align}
          className="dz-popup-positioner"
        >
          <Base.Popup className={cx('dz-tooltip', className)}>
            {label}
          </Base.Popup>
        </Base.Positioner>
      </Base.Portal>
    </Base.Root>
  )
}

/**
 * Shares one delay across the tooltips inside it, so moving along a toolbar
 * opens each one instantly once the first has opened.
 */
export const TooltipProvider = Base.Provider
