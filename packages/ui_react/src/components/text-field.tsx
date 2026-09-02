import { ChevronDown12Regular } from '@fluentui/react-icons'
import type { ComponentPropsWithoutRef } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type TextFieldSize = 'small' | 'medium' | 'large'

export type TextFieldState = 'default' | 'error'

/**
 * An erroring control is re-tinted rather than given a second palette, so its
 * border, wash and focus ring all move together.
 */
export function tintFor(state: TextFieldState, tint: Tint): Tint {
  return state === 'error' ? 'danger' : tint
}

type ControlBase = {
  size?: TextFieldSize
  state?: TextFieldState
  tint?: Tint
  /** Set the mono face — for a key, an ID, or a URL. */
  mono?: boolean
}

export interface TextFieldProps
  extends Omit<ComponentPropsWithoutRef<'input'>, 'size'>, ControlBase {}

export function TextField({
  size = 'medium',
  state = 'default',
  tint = 'primary',
  mono = false,
  className,
  ...props
}: TextFieldProps) {
  return (
    <input
      aria-invalid={state === 'error' || undefined}
      data-mono={mono}
      data-size={size}
      data-tint={tintFor(state, tint)}
      className={cx('dz-text-field', className)}
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
      <ChevronDown12Regular aria-hidden className="dz-select__chevron" />
    </span>
  )
}

export interface FieldValueProps
  extends ComponentPropsWithoutRef<'div'>, Pick<ControlBase, 'size' | 'mono'> {}

/**
 * A value in the shape of an input, for a row that reports rather than edits.
 * Not a disabled input: nothing here was ever editable, so nothing is greyed
 * out to say it stopped being.
 */
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
