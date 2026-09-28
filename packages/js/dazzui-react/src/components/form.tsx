import { Fieldset as BaseFieldset } from '@base-ui/react/fieldset'
import { Form as BaseForm } from '@base-ui/react/form'
import type { ReactNode } from 'react'

import { cx } from '../lib/cx'

export interface FormProps extends Omit<
  BaseForm.Props,
  'className' | 'render'
> {
  className?: string
}

/**
 * The run of fields, and the one thing above them that knows about all of
 * them: `errors`.
 *
 * That is what it is for. A `<form>` element already submits; what it cannot
 * do is take what the server said back — `{ email: 'Already taken' }` — and
 * put each message under the field it belongs to, move the focus to the first
 * one, and clear it again when that field is edited. Base UI's `Form` does,
 * as long as the fields inside are `FormField`s with names.
 *
 * It also owns the rhythm between the fields, which is the other thing no
 * single field can decide.
 */
export function Form({ className, ...props }: FormProps) {
  return <BaseForm className={cx('dz-form', className)} {...props} />
}

export interface FieldsetProps extends Omit<
  BaseFieldset.Root.Props,
  'className' | 'render'
> {
  /** Names the group. It is the fieldset's accessible name as well. */
  legend?: ReactNode
  className?: string
}

/**
 * A run of fields that are one question between them — a date's three boxes,
 * an address, a set of radios.
 *
 * The legend is the group's accessible name, which is the whole point: a
 * screen reader announces it before each field inside, so "Street" is heard
 * as "Billing address, Street". A heading in a `<p>` above the fields looks
 * the same and says none of that.
 *
 * It is not the way to head a *section* of unrelated fields — that is a
 * `SectionLabel`, or a `PreferenceSection`. A fieldset claims the fields
 * inside it belong together.
 */
export function Fieldset({
  legend,
  className,
  children,
  ...props
}: FieldsetProps) {
  return (
    <BaseFieldset.Root className={cx('dz-fieldset', className)} {...props}>
      {legend ? (
        <BaseFieldset.Legend className="dz-fieldset__legend">
          {legend}
        </BaseFieldset.Legend>
      ) : null}
      {children}
    </BaseFieldset.Root>
  )
}
