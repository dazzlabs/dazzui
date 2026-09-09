import { NumberField as Base } from '@base-ui/react/number-field'
import { ChevronDown12Regular, ChevronUp12Regular } from '@fluentui/react-icons'
import type { ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'
import { type TextFieldSize, type TextFieldState, tintFor } from './text-field'

export interface NumberFieldProps extends Omit<
  Base.Root.Props,
  'className' | 'render' | 'children'
> {
  size?: TextFieldSize
  state?: TextFieldState
  tint?: Tint
  /** What the field prints while it is empty. */
  placeholder?: string
  /** A trailing unit inside the box — `px`, `%`, `ms`. */
  unit?: ReactNode
  className?: string
}

/**
 * A number, with the two things a plain `type="number"` never gets right: a
 * stepper you can hold down, and a value that stays a number.
 *
 * Base UI owns the parsing and the formatting — the locale's own decimal
 * separator, the percent and currency formats, the clamp to `min`/`max`, the
 * arrow keys and their large step, and the scrub. What is drawn here is the
 * box, which is the `TextField`'s so a form's rows do not step, and the
 * stepper at its trailing edge.
 */
export function NumberField({
  size = 'medium',
  state = 'default',
  tint = 'primary',
  placeholder,
  unit,
  className,
  ...props
}: NumberFieldProps) {
  return (
    <Base.Root
      /* Base UI's number field takes its invalid state from the `Field` it
         sits in rather than from a prop, so a bare one is marked here — the
         same attribute the field would have set, which is what the stylesheet
         reads either way. */
      data-invalid={state === 'error' ? '' : undefined}
      data-size={size}
      data-tint={tintFor(state, tint)}
      className={cx('dz-number-field', className)}
      {...props}
    >
      <Base.Group className="dz-number-field__group">
        <Base.Input
          placeholder={placeholder}
          aria-invalid={state === 'error' || undefined}
          className="dz-number-field__input"
        />
        {unit ? <span className="dz-number-field__unit">{unit}</span> : null}
        {/* Stacked rather than a minus and a plus on either side: the field
            is read left to right and a value with a control on both sides of
            it reads as three things. The pair is one AppKit stepper. */}
        <span className="dz-number-field__stepper">
          <Base.Increment
            aria-label="Increase"
            className="dz-number-field__step"
          >
            <ChevronUp12Regular />
          </Base.Increment>
          <Base.Decrement
            aria-label="Decrease"
            className="dz-number-field__step"
          >
            <ChevronDown12Regular />
          </Base.Decrement>
        </span>
      </Base.Group>
    </Base.Root>
  )
}
