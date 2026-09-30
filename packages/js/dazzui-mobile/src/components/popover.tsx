import { mergeProps } from '@base-ui/react/merge-props'
import { Popover as Base } from '@base-ui/react/popover'
import { cloneElement, type ReactElement, type ReactNode } from 'react'

import { cx } from '../lib/cx'
import { usePlatform, useThemeContainer } from '../lib/theme'
import type { MenuTrigger } from './menu'

export interface PopoverProps extends Pick<
  Base.Root.Props,
  'open' | 'defaultOpen'
> {
  /** A trigger element, or a function of the open state that returns one. */
  trigger: MenuTrigger
  /** iOS sets it in the headline weight; Material as the subhead. */
  title?: ReactNode
  /** The supporting text, or whatever small content the panel carries. */
  children?: ReactNode
  /**
   * Beneath the content: text buttons, as Material's rich tooltip and a
   * TipKit tip set theirs. Wrap one in `PopoverClose` to dismiss.
   */
  actions?: ReactNode
  /** Below the trigger on iOS, above it on Android, unless it is set. */
  side?: 'top' | 'bottom' | 'left' | 'right'
  align?: 'start' | 'center' | 'end'
  onOpenChange?: (open: boolean) => void
  className?: string
}

/**
 * A small panel anchored to its trigger. On iOS it is
 * `UIPopoverPresentationController` as iOS 26 draws it on iPhone, kept a
 * popover rather than adapted into a sheet: a Liquid Glass panel at the
 * menu corner with no arrow — the content styled as a TipKit popover tip.
 * Material has no popover; its nearest is the rich tooltip, which is what
 * Android draws: the surface container at the 12dp corner, a subhead, the
 * supporting text and optional text-button actions. Base UI positions it,
 * flips it off an edge and closes it on an outside press and on Escape.
 */
export function Popover({
  trigger,
  title,
  children,
  actions,
  side,
  align = 'center',
  onOpenChange,
  className,
  ...root
}: PopoverProps) {
  const platform = usePlatform()
  const container = useThemeContainer()
  const ios = platform === 'ios'
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
        {/* Material's tooltip stands 4dp off its anchor; the glass panel
            hangs 8pt, as the menu does. */}
        <Base.Positioner
          side={side ?? (ios ? 'bottom' : 'top')}
          align={align}
          sideOffset={ios ? 8 : 4}
          data-platform={platform}
          className="dzm-popover-positioner"
        >
          <Base.Popup
            data-platform={platform}
            className={cx(
              'dzm-popover',
              ios && 'dzm-glass dzm-glass--thick',
              className
            )}
          >
            {title || children ? (
              <div className="dzm-popover__content">
                {title ? (
                  <Base.Title className="dzm-popover__title">
                    {title}
                  </Base.Title>
                ) : null}
                {children ? (
                  <Base.Description
                    render={<div />}
                    className="dzm-popover__body"
                  >
                    {children}
                  </Base.Description>
                ) : null}
              </div>
            ) : null}
            {actions ? (
              <div className="dzm-popover__actions">{actions}</div>
            ) : null}
          </Base.Popup>
        </Base.Positioner>
      </Base.Portal>
    </Base.Root>
  )
}

export interface PopoverCloseProps extends Omit<
  Base.Close.Props,
  'className' | 'render'
> {
  render: ReactElement<Record<string, unknown>>
}

/** Wraps a control so pressing it closes the popover it sits in. */
export function PopoverClose(props: PopoverCloseProps) {
  return <Base.Close {...props} />
}
