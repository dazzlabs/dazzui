import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'
import { type TextFieldState, tintFor } from './text-field'

export interface FormFieldProps extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'title'
> {
  label: ReactNode
  /** Help text under the control; carries the error tone when erroring. */
  hint?: ReactNode
  state?: TextFieldState
  tint?: Tint
}

/**
 * The label, the hint, and whatever control sits between them.
 *
 * It is `FormField` rather than `Field` because in this library the `*Field`
 * suffix already belongs to controls — `SearchField` is one — and a wrapper
 * sharing that suffix reads like their base rather than their container.
 */
export function FormField({
  label,
  hint,
  state = 'default',
  tint = 'primary',
  className,
  children,
  ...props
}: FormFieldProps) {
  return (
    <div
      data-invalid={state === 'error'}
      data-tint={tintFor(state, tint)}
      className={cx('dz-form-field', className)}
      {...props}
    >
      <span className="dz-form-field__label">{label}</span>
      {children}
      {hint ? <span className="dz-form-field__hint">{hint}</span> : null}
    </div>
  )
}
