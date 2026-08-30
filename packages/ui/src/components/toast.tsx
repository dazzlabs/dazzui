import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export interface ToastProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'children'
> {
  tint?: Tint
  /** Leading node — a glyph, or a `Spinner` for work still in flight. */
  icon?: ReactNode
  children?: ReactNode
  /** Trailing affordance — undo, retry. */
  action?: ReactNode
  /** Renders the dismiss button; wire it to remove the toast. */
  onDismiss?: () => void
  /** Accessible name of the dismiss button. */
  dismissLabel?: string
}

/**
 * Lifetime belongs to the host — the system stays stateless — but the contract
 * worth keeping is: a few seconds for a plain notice, longer when it carries
 * an action, and a failure stays until it is dismissed.
 */
export function Toast({
  tint = 'neutral',
  icon,
  action,
  onDismiss,
  dismissLabel = 'Dismiss',
  className,
  children,
  ...props
}: ToastProps) {
  return (
    <div
      role="status"
      data-tint={tint}
      className={cx('dz-toast', className)}
      {...props}
    >
      {icon ? <span className="dz-toast__icon">{icon}</span> : null}
      <span className="dz-toast__message">{children}</span>
      {action ? <span className="dz-toast__action">{action}</span> : null}
      {onDismiss ? (
        <button
          type="button"
          aria-label={dismissLabel}
          className="dz-toast__dismiss"
          onClick={onDismiss}
        >
          ✕
        </button>
      ) : null}
    </div>
  )
}

export interface ToastViewportProps extends ComponentPropsWithoutRef<'div'> {
  /** Which edge the stack hangs from. */
  placement?: 'bottom' | 'top'
}

export function ToastViewport({
  placement = 'bottom',
  className,
  ...props
}: ToastViewportProps) {
  return (
    <div
      data-placement={placement}
      className={cx('dz-toast-viewport', className)}
      {...props}
    />
  )
}
