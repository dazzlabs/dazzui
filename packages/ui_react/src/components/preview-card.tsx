import { PreviewCard as Base } from '@base-ui/react/preview-card'
import type { ReactElement, ReactNode } from 'react'

import { cx } from '../lib/cx'

export interface PreviewCardProps extends Pick<
  Base.Root.Props,
  'open' | 'defaultOpen'
> {
  /**
   * What it previews — a link, usually. Base UI listens to it for hover and
   * for focus, and keeps the card open while the pointer travels to it.
   */
  trigger: ReactElement<Record<string, unknown>>
  side?: 'top' | 'bottom' | 'left' | 'right'
  align?: 'start' | 'center' | 'end'
  /** How long the pointer has to rest before it opens. */
  delay?: number
  /** How long it stays after the pointer leaves. */
  closeDelay?: number
  width?: number | string
  container?: Base.Portal.Props['container']
  onOpenChange?: (open: boolean) => void
  /** Applied to the card. */
  className?: string
  children?: ReactNode
}

/**
 * What is behind a link, shown without following it — a repository, a person,
 * an issue.
 *
 * It is a `Tooltip` that holds a card rather than a label, and the difference
 * is what may go in it: a tooltip must carry nothing a reader needs, because
 * a pointer never reaches it; a preview card stays open while the pointer
 * travels, so it can hold a link of its own. It is never the only way to
 * reach what it shows.
 */
export function PreviewCard({
  trigger,
  side = 'bottom',
  align = 'center',
  delay,
  closeDelay,
  width,
  container,
  open,
  defaultOpen,
  onOpenChange,
  className,
  children,
}: PreviewCardProps) {
  return (
    <Base.Root
      open={open}
      defaultOpen={defaultOpen}
      onOpenChange={onOpenChange ? next => onOpenChange(next) : undefined}
    >
      <Base.Trigger render={trigger} delay={delay} closeDelay={closeDelay} />
      <Base.Portal container={container}>
        <Base.Positioner
          side={side}
          align={align}
          className="dz-popup-positioner"
        >
          <Base.Popup
            data-tint="neutral"
            className={cx(
              'dz-popup',
              'dz-popover',
              'dz-preview-card',
              className
            )}
            style={width === undefined ? undefined : { width }}
          >
            {children}
          </Base.Popup>
        </Base.Positioner>
      </Base.Portal>
    </Base.Root>
  )
}
