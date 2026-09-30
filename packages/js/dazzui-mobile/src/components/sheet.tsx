import { Drawer as Base } from '@base-ui/react/drawer'
import { type ReactElement, type ReactNode, useState } from 'react'

import { cx } from '../lib/cx'
import { Check, Close } from '../lib/glyphs'
import { usePlatform, useThemeContainer } from '../lib/theme'

/**
 * How far up a sheet comes: iOS's medium detent is half the screen and
 * its large one all of it, short of the top edge; `auto` is as tall as the
 * content.
 */
export type SheetDetent = 'auto' | 'medium' | 'large'

export interface SheetProps extends Omit<
  Base.Root.Props,
  'children' | 'snapPoints' | 'snapPoint' | 'defaultSnapPoint'
> {
  /** Rendered in place and wired to open the sheet. */
  trigger?: ReactElement<Record<string, unknown>>
  title?: ReactNode
  /** Where the sheet opens. With `detents`, the one it opens at. */
  detent?: SheetDetent
  /**
   * The detents a drag moves the sheet between — `['medium', 'large']` is
   * the resizable sheet Maps and Find My use. A resizable sheet shows its
   * grabber on iOS, as `prefersGrabberVisible` does.
   */
  detents?: readonly Exclude<SheetDetent, 'auto'>[]
  /** Called when the sheet settles at another detent. */
  onDetentChange?: (detent: Exclude<SheetDetent, 'auto'>) => void
  /** Show the grabber even when the sheet cannot be resized. */
  grabber?: boolean
  /**
   * iOS 26 draws the cancellation action as a glass circle carrying an
   * xmark at the leading edge of the sheet's bar. Material swipes the sheet
   * away instead, so Android draws none.
   */
  closeLabel?: string
  /**
   * The confirmation action: a checkmark in a tinted circle at the
   * trailing edge on iOS, a text button on Android. Pressing it calls this
   * and closes the sheet.
   */
  onConfirm?: () => void
  confirmLabel?: string
  className?: string
  children?: ReactNode
}

const SNAP: Record<Exclude<SheetDetent, 'auto'>, number> = {
  medium: 0.5,
  large: 1,
}

/**
 * A sheet from the bottom edge: UIKit's, and Material's modal bottom
 * sheet. Base UI's drawer is the behaviour — the scrim, the focus trap,
 * the scroll lock, Escape, and the drag that moves it between detents and
 * dismisses it when flung down. It is portalled into the theme so it
 * inherits the screen's colours.
 *
 * iOS 26 floats a partial sheet: inset from the screen's edges, every
 * corner concentric with the device's, in Liquid Glass. Pulled to its
 * large detent it lands on the edges, turns opaque, and the screen behind
 * it recedes. Material's sheet is the low surface container at the 28dp
 * corner with its drag handle, always on the bottom edge.
 */
export function Sheet({
  trigger,
  title,
  detent = 'auto',
  detents,
  onDetentChange,
  grabber,
  closeLabel = 'Close',
  onConfirm,
  confirmLabel = 'Done',
  className,
  children,
  ...root
}: SheetProps) {
  const platform = usePlatform()
  const container = useThemeContainer()
  const resizable = Boolean(detents && detents.length > 1)
  const initial = detent === 'auto' ? (detents?.[0] ?? 'large') : detent
  const [current, setCurrent] = useState<SheetDetent>(
    resizable ? initial : detent
  )
  const snapPoints = resizable ? detents!.map(value => SNAP[value]) : undefined
  const showGrabber = grabber ?? (platform === 'android' || resizable)
  const hasBar = Boolean(title || onConfirm)

  return (
    <Base.Root
      swipeDirection="down"
      {...(snapPoints
        ? {
            snapPoints,
            defaultSnapPoint: SNAP[initial],
            onSnapPointChange: point => {
              const next = point === SNAP.medium ? 'medium' : 'large'
              setCurrent(next)
              onDetentChange?.(next)
            },
          }
        : {})}
      {...root}
      onOpenChange={(open, details) => {
        if (open && resizable) setCurrent(initial)
        root.onOpenChange?.(open, details)
      }}
    >
      {trigger ? <Base.Trigger render={trigger} /> : null}
      <Base.Portal container={container}>
        <Base.Backdrop
          data-platform={platform}
          data-resizable={resizable || undefined}
          className="dzm-scrim dzm-sheet-scrim"
        />
        <Base.Viewport
          data-platform={platform}
          className="dzm-overlay dzm-sheet-viewport"
        >
          <Base.Popup
            data-platform={platform}
            data-detent={current}
            data-resizable={resizable || undefined}
            className={cx(
              'dzm-sheet',
              platform === 'ios' && current !== 'large' && 'dzm-glass',
              className
            )}
          >
            {showGrabber ? (
              <span className="dzm-sheet__grabber" aria-hidden />
            ) : null}
            {hasBar ? (
              <div className="dzm-sheet__bar">
                {platform === 'ios' ? (
                  <Base.Close
                    aria-label={closeLabel}
                    className="dzm-sheet__action dzm-glass"
                  >
                    <Close />
                  </Base.Close>
                ) : null}
                {title ? (
                  <Base.Title className="dzm-sheet__title">{title}</Base.Title>
                ) : (
                  <span />
                )}
                {onConfirm ? (
                  <Base.Close
                    aria-label={platform === 'ios' ? confirmLabel : undefined}
                    className={cx(
                      'dzm-sheet__action dzm-m3-layer',
                      platform === 'ios' && 'dzm-glass'
                    )}
                    data-confirm=""
                    onClick={onConfirm}
                  >
                    {platform === 'ios' ? <Check /> : confirmLabel}
                  </Base.Close>
                ) : null}
              </div>
            ) : null}
            <Base.Content className="dzm-sheet__body">{children}</Base.Content>
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
