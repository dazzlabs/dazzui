import { OTPField as Base } from '@base-ui/react/otp-field'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'
import { type TextFieldSize, type TextFieldState, tintFor } from './text-field'

export interface OtpFieldProps extends Omit<
  Base.Root.Props,
  'className' | 'render' | 'children'
> {
  size?: TextFieldSize
  state?: TextFieldState
  tint?: Tint
  /**
   * Draws a rule between two runs of cells — `3` on a six-digit code makes it
   * two groups of three, which is how a code is read aloud.
   */
  groupAfter?: number
  className?: string
}

/**
 * The one-time code a reader is copying out of a message.
 *
 * It is a run of cells rather than a single field because that is what makes
 * it fast: Base UI moves the caret between them, takes a paste of the whole
 * code into all of them at once, hands the browser the autofill hint that
 * pulls the code out of an SMS, and can submit the form the moment the last
 * cell fills.
 *
 * Each cell is the text box the rest of the form is set in, at one character
 * wide — a code entered in a box of its own design would be the only control
 * on the page that was.
 */
export function OtpField({
  size = 'medium',
  state = 'default',
  tint = 'primary',
  groupAfter,
  length,
  className,
  ...props
}: OtpFieldProps) {
  return (
    <Base.Root
      length={length}
      data-size={size}
      data-tint={tintFor(state, tint)}
      className={cx('dz-otp-field', className)}
      {...props}
    >
      {Array.from({ length }, (_, index) => (
        <Base.Input
          key={index}
          data-group-end={
            groupAfter !== undefined &&
            index > 0 &&
            index % groupAfter === 0 &&
            index !== length - 1
              ? ''
              : undefined
          }
          className="dz-otp-field__cell"
        />
      ))}
    </Base.Root>
  )
}
