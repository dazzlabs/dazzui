import { AlertDialog as Alert } from '@base-ui/react/alert-dialog'
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
   * Whether Escape and a press outside close the sheet. `false` makes it an
   * alert dialog: the only way out is a control inside it.
   *
   * It is an axis rather than a second component because everything a reader
   * sees is identical — the scrim, the sheet, the three rows — and the one
   * thing that differs is whether the sheet can be dismissed without
   * answering. Reach for it where dismissing would be an answer: a
   * destructive confirmation, or an error the app cannot continue past.
   */
  dismissible?: boolean
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
  /**
   * Confines the scrim and viewport to the nearest positioned container
   * (`position: absolute` instead of `position: fixed`), for dialogs mounted
   * inside a `WindowFrame` or preview container.
   */
  contained?: boolean
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
  dismissible = true,
  trigger,
  container,
  contained = false,
  initialFocus,
  finalFocus,
  className,
  style,
  modal,
  disablePointerDismissal,
  children,
  ...root
}: DialogProps) {
  /*
   * Only the root and the trigger differ between the two. Base UI's alert
   * dialog re-exports the dialog's own portal, backdrop, viewport, popup,
   * title and description — they are the same components reading the same
   * context — so the sheet below is written once and mounted under whichever
   * root the caller asked for.
   */
  const body = (
    <>
      {trigger ? (
        dismissible ? (
          <Base.Trigger render={trigger} />
        ) : (
          <Alert.Trigger render={trigger} />
        )
      ) : null}
      <Base.Portal container={container}>
        <Base.Backdrop
          data-contained={contained || undefined}
          className="dz-dialog-scrim"
        />
        <Base.Viewport
          data-contained={contained || undefined}
          className="dz-dialog-viewport"
        >
          <Base.Popup
            /* Stated rather than left to the primitive: a sheet with no role
               announces as a plain group. `danger` is an alert because it is
               reporting as much as it is asking, and so is a sheet with no
               way out but an answer. */
            role={tone === 'danger' || !dismissible ? 'alertdialog' : 'dialog'}
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
    </>
  )

  return dismissible ? (
    <Base.Root
      modal={modal}
      disablePointerDismissal={disablePointerDismissal}
      {...root}
    >
      {body}
    </Base.Root>
  ) : (
    /* The alert root takes neither `modal` nor `disablePointerDismissal` —
       it is always modal and never dismissed by a press — so they are
       destructured out above rather than passed and ignored. The cast is
       over the event-detail type on `onOpenChange`, which the two roots
       declare separately and which carries the same reasons. */
    <Alert.Root {...(root as Alert.Root.Props)}>{body}</Alert.Root>
  )
}

export interface DialogHeaderProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'title'
> {
  title: ReactNode
  subtitle?: ReactNode
  /** Leading visual — a thumbnail, icon, or avatar aligned beside the title and subtitle. */
  leading?: ReactNode
  /** Trailing control — a close button or action aligned to the trailing edge. */
  trailing?: ReactNode
  /** Set the subtitle in the monospace code face — for an address, ID, or path. */
  monoSubtitle?: boolean
}

/** The title names the sheet and the subtitle describes it, to a screen reader as well. */
export function DialogHeader({
  title,
  subtitle,
  leading,
  trailing,
  monoSubtitle = false,
  className,
  children,
  ...props
}: DialogHeaderProps) {
  return (
    <div
      data-has-leading={leading ? '' : undefined}
      data-has-trailing={trailing ? '' : undefined}
      className={cx('dz-dialog__header', className)}
      {...props}
    >
      {leading ? <div className="dz-dialog__leading">{leading}</div> : null}
      <Base.Title className="dz-dialog__title">{title}</Base.Title>
      {subtitle ? (
        <Base.Description
          data-mono={monoSubtitle || undefined}
          className="dz-dialog__subtitle"
        >
          {subtitle}
        </Base.Description>
      ) : null}
      {trailing ? <div className="dz-dialog__trailing">{trailing}</div> : null}
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
