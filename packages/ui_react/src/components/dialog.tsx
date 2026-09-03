import { Dialog as Base } from '@base-ui/react/dialog'
import type {
  ComponentPropsWithoutRef,
  CSSProperties,
  ReactElement,
  ReactNode,
} from 'react'

import { cx } from '../lib/cx'

export type DialogTone = 'default' | 'danger'

export interface DialogProps
  extends
    Omit<Base.Root.Props, 'children'>,
    Pick<Base.Popup.Props, 'initialFocus' | 'finalFocus'> {
  /** Overrides the token width for a sheet that needs more or less room. */
  width?: number | string
  /** `danger` is what a destructive confirmation wears; it also announces as an alert. */
  tone?: DialogTone
  /**
   * Rendered in place and wired to open the sheet — a `Button`, usually. A
   * caller driving `open` itself needs none.
   */
  trigger?: ReactElement<Record<string, unknown>>
  /**
   * Where the sheet is rendered. It covers the viewport by default; a shell
   * that wants it over one of its windows passes that window.
   */
  container?: Base.Portal.Props['container']
  className?: string
  style?: CSSProperties
  children?: ReactNode
}

/**
 * The sheet a modal form or a confirmation is built in — the scrim, the focus
 * trap, the scroll lock and the Escape are Base UI's; the header, the body
 * and the footer are separate components rather than props precisely so a
 * sheet can leave one out.
 */
export function Dialog({
  width,
  tone = 'default',
  trigger,
  container,
  initialFocus,
  finalFocus,
  className,
  style,
  children,
  ...root
}: DialogProps) {
  return (
    <Base.Root {...root}>
      {trigger ? <Base.Trigger render={trigger} /> : null}
      <Base.Portal container={container}>
        <Base.Backdrop className="dz-dialog-scrim" />
        <Base.Viewport className="dz-dialog-viewport">
          <Base.Popup
            /* Stated rather than left to the primitive: a sheet with no role
               announces as a plain group, and `danger` is an alert because it
               is reporting as much as it is asking. */
            role={tone === 'danger' ? 'alertdialog' : 'dialog'}
            data-tone={tone}
            initialFocus={initialFocus}
            finalFocus={finalFocus}
            className={cx('dz-dialog', className)}
            style={width === undefined ? style : { width, ...style }}
          >
            {children}
          </Base.Popup>
        </Base.Viewport>
      </Base.Portal>
    </Base.Root>
  )
}

export interface DialogHeaderProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'title'
> {
  title: ReactNode
  subtitle?: ReactNode
}

/** The title names the sheet and the subtitle describes it, to a screen reader as well. */
export function DialogHeader({
  title,
  subtitle,
  className,
  children,
  ...props
}: DialogHeaderProps) {
  return (
    <div className={cx('dz-dialog__header', className)} {...props}>
      <Base.Title className="dz-dialog__title">{title}</Base.Title>
      {subtitle ? (
        <Base.Description className="dz-dialog__subtitle">
          {subtitle}
        </Base.Description>
      ) : null}
      {children}
    </div>
  )
}

export function DialogBody({
  className,
  ...props
}: ComponentPropsWithoutRef<'div'>) {
  return <div className={cx('dz-dialog__body', className)} {...props} />
}

export function DialogFooter({
  className,
  ...props
}: ComponentPropsWithoutRef<'div'>) {
  return <div className={cx('dz-dialog__footer', className)} {...props} />
}

export interface DialogCloseProps extends Omit<
  Base.Close.Props,
  'className' | 'render'
> {
  /** The control to render — a `Button`, usually. */
  render: ReactElement<Record<string, unknown>>
}

/** Wraps a control so pressing it closes the sheet it sits in. */
export function DialogClose(props: DialogCloseProps) {
  return <Base.Close {...props} />
}
