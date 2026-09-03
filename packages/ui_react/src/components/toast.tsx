import { Toast as Base } from '@base-ui/react/toast'
import { Dismiss12Regular } from '@fluentui/react-icons'
import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'
import { Button } from './button'

/**
 * How long a toast stays up, in milliseconds. A plain notice reads in a few
 * seconds; one carrying an action waits for the hand to reach it; a failure
 * stays until it is dismissed.
 */
const NOTICE_MS = 5000
const ACTION_MS = 8000

export interface ToastAction {
  label: ReactNode
  onClick: () => void
}

interface ToastData {
  tint: Tint
  icon?: ReactNode
  action?: ToastAction
  dismissable: boolean
  dismissLabel: string
}

export interface ToastOptions {
  /** Replaces the toast already showing under this id, if there is one. */
  id?: string
  /** The message. One line, no wrapping — anything longer is a Callout. */
  title: ReactNode
  tint?: Tint
  /** Leading node — a glyph, or a `Spinner` for work still in flight. */
  icon?: ReactNode
  /** Trailing affordance — undo, retry. Pressing it closes the toast. */
  action?: ToastAction
  /** Whether to render the close button. On by default. */
  dismissable?: boolean
  /** Accessible name of the close button. */
  dismissLabel?: string
  /** Milliseconds before it goes on its own; `0` keeps it until dismissed. */
  timeout?: number
  /** `high` interrupts the screen reader; keep it for a failure. */
  priority?: 'low' | 'high'
  onClose?: () => void
}

function toBase(options: ToastOptions) {
  const {
    id,
    title,
    tint = 'neutral',
    icon,
    action,
    dismissable = true,
    dismissLabel = 'Dismiss',
    timeout,
    priority,
    onClose,
  } = options

  const data: ToastData = { tint, icon, action, dismissable, dismissLabel }

  return {
    id,
    title,
    type: tint,
    timeout:
      timeout ?? (tint === 'danger' ? 0 : action ? ACTION_MS : NOTICE_MS),
    priority: priority ?? (tint === 'danger' ? 'high' : 'low'),
    onClose,
    data,
  }
}

/**
 * The queue. Call `add` from anywhere under a `ToastProvider`; the viewport
 * draws what is in it, and Base UI keeps the timers, the limit, the pause on
 * hover and the announcement.
 */
export function useToast() {
  const manager = Base.useToastManager()
  return {
    add: (options: ToastOptions) => manager.add(toBase(options)),
    close: manager.close,
  }
}

export interface ToastProviderProps extends Base.Provider.Props {}

/** Holds the queue for the tree beneath it. One per window. */
export function ToastProvider(props: ToastProviderProps) {
  return <Base.Provider {...props} />
}

export interface ToastProps {
  toast: Base.Root.ToastObject<ToastData>
}

/**
 * One toast, as the viewport draws it. It is exported for a host that lays
 * its own viewport out; most render `ToastViewport` and never see it.
 */
export function Toast({ toast }: ToastProps) {
  const { close } = Base.useToastManager()
  const { tint, icon, action, dismissable, dismissLabel } = toast.data ?? {
    tint: 'neutral',
    dismissable: true,
    dismissLabel: 'Dismiss',
  }

  return (
    <Base.Root toast={toast} data-tint={tint} className="dz-toast">
      {icon ? <span className="dz-toast__icon">{icon}</span> : null}
      <Base.Title render={<span />} className="dz-toast__message" />
      {action ? (
        <Base.Action
          className="dz-toast__action"
          render={<Button size="small" tint={tint} variant="plain" />}
          onClick={() => {
            action.onClick()
            close(toast.id)
          }}
        >
          {action.label}
        </Base.Action>
      ) : null}
      {dismissable ? (
        <Base.Close aria-label={dismissLabel} className="dz-toast__dismiss">
          <Dismiss12Regular />
        </Base.Close>
      ) : null}
    </Base.Root>
  )
}

export interface ToastViewportProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'children'
> {
  /** Which edge the stack hangs from. */
  placement?: 'bottom' | 'top'
}

/**
 * Where a window's toasts land. Transparent to the pointer between them, so
 * the page underneath stays usable while one is on screen.
 */
export function ToastViewport({
  placement = 'bottom',
  className,
  ...props
}: ToastViewportProps) {
  const { toasts } = Base.useToastManager()

  return (
    <Base.Viewport
      data-placement={placement}
      className={cx('dz-toast-viewport', className)}
      {...props}
    >
      {toasts.map(toast => (
        <Toast key={toast.id} toast={toast} />
      ))}
    </Base.Viewport>
  )
}
