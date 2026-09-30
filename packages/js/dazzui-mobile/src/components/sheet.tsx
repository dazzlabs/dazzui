import { Dialog as Base } from '@base-ui/react/dialog'
import type { ReactElement, ReactNode } from 'react'

import { cx } from '../lib/cx'
import { Close } from '../lib/glyphs'
import { usePlatform, useThemeContainer } from '../lib/theme'

export interface SheetProps extends Omit<Base.Root.Props, 'children'> {
  /** Rendered in place and wired to open the sheet. */
  trigger?: ReactElement<Record<string, unknown>>
  title?: ReactNode
  /**
   * How far up the sheet comes: iOS's medium detent is half the screen,
   * large all of it; `auto` is as tall as its content.
   */
  detent?: 'auto' | 'medium' | 'large'
  /** iOS draws a close button in the bar when there is a title. */
  closeLabel?: string
  className?: string
  children?: ReactNode
}

/**
 * A sheet from the bottom edge: UIKit's with its grabber and the big
 * concentric corner, Material's modal bottom sheet with its drag handle.
 * The scrim, the focus trap, the scroll lock and the Escape are Base UI's,
 * and it is portalled into the theme so it inherits the screen's colours.
 */
export function Sheet({
  trigger,
  title,
  detent = 'auto',
  closeLabel = 'Close',
  className,
  children,
  ...root
}: SheetProps) {
  const platform = usePlatform()
  const container = useThemeContainer()
  return (
    <Base.Root {...root}>
      {trigger ? <Base.Trigger render={trigger} /> : null}
      <Base.Portal container={container}>
        <Base.Backdrop
          data-platform={platform}
          className="dzm-scrim dzm-sheet-scrim"
        />
        <Base.Viewport
          data-platform={platform}
          className="dzm-overlay dzm-sheet-viewport"
        >
          <Base.Popup
            data-platform={platform}
            data-detent={detent}
            className={cx('dzm-sheet', className)}
          >
            <span className="dzm-sheet__grabber" aria-hidden />
            {title ? (
              <div className="dzm-sheet__bar">
                <Base.Title className="dzm-sheet__title">{title}</Base.Title>
                {platform === 'ios' ? (
                  <Base.Close
                    aria-label={closeLabel}
                    className="dzm-sheet__close"
                  >
                    <Close />
                  </Base.Close>
                ) : null}
              </div>
            ) : null}
            <div className="dzm-sheet__body">{children}</div>
          </Base.Popup>
        </Base.Viewport>
      </Base.Portal>
    </Base.Root>
  )
}

export interface SheetCloseProps extends Omit<
  Base.Close.Props,
  'className' | 'render'
> {
  render: ReactElement<Record<string, unknown>>
}

/** Wraps a control so pressing it closes the sheet it sits in. */
export function SheetClose(props: SheetCloseProps) {
  return <Base.Close {...props} />
}
