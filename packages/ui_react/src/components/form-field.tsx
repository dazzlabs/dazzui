import { Field } from '@base-ui/react/field'
import type { ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'
import { type TextFieldState, tintFor } from './text-field'

export interface FormFieldProps extends Omit<
  Field.Root.Props,
  'className' | 'render' | 'title'
> {
  label: ReactNode
  /** Help text under the control; carries the error tone when erroring. */
  hint?: ReactNode
  state?: TextFieldState
  tint?: Tint
  className?: string
}

/**
 * The label, the hint, and whatever control sits between them.
 *
 * Base UI's `Field` does the wiring: the label is `for` the control, the hint
 * is in its `aria-describedby`, and `state="error"` marks it invalid — as
 * long as the control is one of this library's, since they all register with
 * the field they sit in.
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
    <Field.Root
      invalid={state === 'error'}
      data-tint={tintFor(state, tint)}
      className={cx('dz-form-field', className)}
      {...props}
    >
      <Field.Label className="dz-form-field__label">{label}</Field.Label>
      {children}
      {hint ? (
        <Field.Description className="dz-form-field__hint">
          {hint}
        </Field.Description>
      ) : null}
    </Field.Root>
  )
}
