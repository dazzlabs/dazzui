import { Drawer as Base } from '@base-ui/react/drawer'
import type { CSSProperties, ReactElement, ReactNode } from 'react'

import { cx } from '../lib/cx'

export type DrawerSide = 'left' | 'right' | 'top' | 'bottom'

/**
 * Which way it is pushed to dismiss it, which is the way it came in. Base UI
 * names the gesture rather than the edge, and the edge is what a caller
 * thinks in.
 */
const SWIPE: Record<DrawerSide, 'left' | 'right' | 'up' | 'down'> = {
  left: 'left',
  right: 'right',
  top: 'up',
  bottom: 'down',
}

export interface DrawerProps extends Omit<
  Base.Root.Props,
  'children' | 'swipeDirection'
> {
  /** The edge it comes in from. */
  side?: DrawerSide
  /** Overrides the token width — or height, on a top or bottom drawer. */
  size?: number | string
  /** Names the sheet to a screen reader as well as heading it. */
  title?: ReactNode
  /** A line under the title, and the sheet's accessible description. */
  description?: ReactNode
  /** A row of controls pinned to the far edge, clear of the scrolling body. */
  footer?: ReactNode
  /** Rendered in place and wired to open the drawer. */
  trigger?: ReactElement<Record<string, unknown>>
  container?: Base.Portal.Props['container']
  className?: string
  style?: CSSProperties
  children?: ReactNode
}

/**
 * A sheet that comes in from an edge and can be pushed back out.
 *
 * It is the `Dialog`'s sibling, and the choice between them is about where
 * the reader's attention has to go rather than about the screen it is on: a
 * dialog takes the middle of the view and asks a question; a drawer takes a
 * side of it and holds a panel the reader dips into — filters, a detail
 * pane — while what they were reading stays where it was.
 *
 * The header, body and footer are props here rather than the separate
 * components a `Dialog` has, because those read a dialog's own context: a
 * `DialogHeader` inside a drawer would find no title to register. One family
 * with a second set of parts would be two drawings of the same three rows.
 */
export function Drawer({
  side = 'right',
  size,
  title,
  description,
  footer,
  trigger,
  container,
  className,
  style,
  children,
  ...root
}: DrawerProps) {
  const sizing =
    size === undefined
      ? undefined
      : side === 'top' || side === 'bottom'
        ? { height: size }
        : { width: size }

  return (
    <Base.Root swipeDirection={SWIPE[side]} {...root}>
      {trigger ? <Base.Trigger render={trigger} /> : null}
      <Base.Portal container={container}>
        <Base.Backdrop className="dz-drawer-scrim" />
        <Base.Viewport className="dz-drawer-viewport">
          <Base.Popup
            data-side={side}
            className={cx('dz-drawer', className)}
            style={{ ...sizing, ...style }}
          >
            {/* The grabber, on the two edges a thumb actually pushes. It is
                decoration over Base UI's own swipe handling rather than the
                control itself — which is why it is hidden rather than
                labelled. */}
            {side === 'top' || side === 'bottom' ? (
              <span aria-hidden className="dz-drawer__grabber" />
            ) : null}
            {title || description ? (
              <div className="dz-drawer__header">
                {title ? (
                  <Base.Title className="dz-drawer__title">{title}</Base.Title>
                ) : null}
                {description ? (
                  <Base.Description className="dz-drawer__subtitle">
                    {description}
                  </Base.Description>
                ) : null}
              </div>
            ) : null}
            <div className="dz-drawer__body">{children}</div>
            {footer ? <div className="dz-drawer__footer">{footer}</div> : null}
          </Base.Popup>
        </Base.Viewport>
      </Base.Portal>
    </Base.Root>
  )
}

/** Wraps a control so pressing it closes the drawer it sits in. */
export const DrawerClose = Base.Close
