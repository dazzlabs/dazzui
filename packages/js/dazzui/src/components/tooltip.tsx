import { Tooltip as Base } from '@base-ui/react/tooltip'
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useId,
  useState,
  type ReactElement,
  type ReactNode,
} from 'react'

import { cx } from '../lib/cx'

const TooltipScope = createContext<
  ((id: string, mounted: boolean) => void) | null
>(null)

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
  const parent = useContext(TooltipScope)
  const id = useId()
  const [nested, setNested] = useState<Set<string>>(() => new Set())
  const register = useCallback((id: string, mounted: boolean) => {
    setNested(previous => {
      if (previous.has(id) === mounted) return previous
      const next = new Set(previous)
      if (mounted) next.add(id)
      else next.delete(id)
      return next
    })
  }, [])
  useEffect(() => {
    parent?.(id, true)
    return () => parent?.(id, false)
  }, [parent, id])

  return (
    // A control such as IconButton already owns its hint. Scope only the
    // trigger; tooltips in a rich label must not disable their popup owner.
    <Base.Root disabled={disabled || nested.size > 0}>
      <TooltipScope.Provider value={register}>
        <Base.Trigger render={children} delay={delay} closeDelay={closeDelay} />
      </TooltipScope.Provider>
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
