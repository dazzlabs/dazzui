import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type FieldSize = 'small' | 'medium' | 'large'

export type FieldState = 'default' | 'error'

/**
 * An erroring control is re-tinted rather than given a second palette, so its
 * border, wash, and focus ring all move together.
 */
function tintFor(state: FieldState, tint: Tint): Tint {
  return state === 'error' ? 'danger' : tint
}

type ControlBase = {
  size?: FieldSize
  state?: FieldState
  tint?: Tint
  /** Set the mono face — for a key, an ID, or a URL. */
  mono?: boolean
}

export interface InputProps
  extends Omit<ComponentPropsWithoutRef<'input'>, 'size'>, ControlBase {}

export function Input({
  size = 'medium',
  state = 'default',
  tint = 'primary',
  mono = false,
  className,
  ...props
}: InputProps) {
  return (
    <input
      aria-invalid={state === 'error' || undefined}
      data-mono={mono}
      data-size={size}
      data-tint={tintFor(state, tint)}
      className={cx('dz-input', className)}
      {...props}
    />
  )
}

export interface TextareaProps
  extends ComponentPropsWithoutRef<'textarea'>, ControlBase {}

export function Textarea({
  size = 'medium',
  state = 'default',
  tint = 'primary',
  mono = false,
  className,
  ...props
}: TextareaProps) {
  return (
    <textarea
      aria-invalid={state === 'error' || undefined}
      data-mono={mono}
      data-size={size}
      data-tint={tintFor(state, tint)}
      className={cx('dz-textarea', className)}
      {...props}
    />
  )
}

// `size` on a native select is the number of visible rows, so the control's
// own size axis has to displace it — same as on an input.
export interface SelectProps
  extends Omit<ComponentPropsWithoutRef<'select'>, 'size'>, ControlBase {}

export function Select({
  size = 'medium',
  state = 'default',
  tint = 'primary',
  className,
  ...props
}: SelectProps) {
  return (
    <span
      data-size={size}
      data-tint={tintFor(state, tint)}
      className={cx('dz-select', className)}
    >
      <select
        aria-invalid={state === 'error' || undefined}
        className="dz-select__control"
        {...props}
      />
      <svg
        aria-hidden
        viewBox="0 0 12 12"
        width="12"
        height="12"
        className="dz-select__chevron"
      >
        <path
          d="M2.5 4.5 6 8l3.5-3.5"
          fill="none"
          stroke="currentColor"
          strokeWidth="1.5"
          strokeLinecap="round"
          strokeLinejoin="round"
        />
      </svg>
    </span>
  )
}

export interface FieldValueProps
  extends ComponentPropsWithoutRef<'div'>, Pick<ControlBase, 'size' | 'mono'> {}

/** A value in the shape of an input, for a row that reports rather than edits. */
export function FieldValue({
  size = 'medium',
  mono = false,
  className,
  ...props
}: FieldValueProps) {
  return (
    <div
      data-mono={mono}
      data-size={size}
      className={cx('dz-field-value', className)}
      {...props}
    />
  )
}

export interface FieldProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'title'
> {
  label: ReactNode
  /** Help text under the control; carries the error tone when erroring. */
  hint?: ReactNode
  state?: FieldState
  tint?: Tint
}

/** Label, control, hint — the shape of a form row. */
export function Field({
  label,
  hint,
  state = 'default',
  tint = 'primary',
  className,
  children,
  ...props
}: FieldProps) {
  return (
    <div
      data-invalid={state === 'error'}
      data-tint={tintFor(state, tint)}
      className={cx('dz-field', className)}
      {...props}
    >
      <span className="dz-field__label">{label}</span>
      {children}
      {hint ? <span className="dz-field__hint">{hint}</span> : null}
    </div>
  )
}
