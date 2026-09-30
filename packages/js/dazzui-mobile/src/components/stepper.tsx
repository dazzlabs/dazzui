import { NumberField } from '@base-ui/react/number-field'

import { cx } from '../lib/cx'
import { Minus, Plus } from '../lib/glyphs'
import { usePlatform } from '../lib/theme'

export interface StepperProps extends Omit<
  NumberField.Root.Props,
  'className' | 'render' | 'children'
> {
  /** Shows the value between the two buttons; iOS's stepper shows none. */
  showValue?: boolean
  className?: string
}

/**
 * UIStepper — a minus and a plus in one glass capsule — and, on Android,
 * where Material has no stepper, the same two as a connected pair of
 * outlined icon buttons. Base UI's number field owns the value, the
 * bounds, the step and the hold-to-repeat.
 */
export function Stepper({
  showValue = false,
  className,
  ...props
}: StepperProps) {
  const platform = usePlatform()
  return (
    <NumberField.Root
      data-platform={platform}
      className={cx('dzm-stepper', className)}
      {...props}
    >
      <NumberField.Group className="dzm-stepper__group">
        <NumberField.Decrement className="dzm-stepper__button dzm-m3-layer">
          <Minus />
        </NumberField.Decrement>
        <NumberField.Input
          className="dzm-stepper__input"
          data-shown={showValue || undefined}
          readOnly
          tabIndex={showValue ? undefined : -1}
        />
        <NumberField.Increment className="dzm-stepper__button dzm-m3-layer">
          <Plus />
        </NumberField.Increment>
      </NumberField.Group>
    </NumberField.Root>
  )
}
