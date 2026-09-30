import { Meter as Base } from '@base-ui/react/meter'
import type { ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type MeterSize = 'small' | 'medium' | 'large'

export interface MeterProps extends Omit<
  Base.Root.Props,
  'className' | 'render' | 'children'
> {
  size?: MeterSize
  tint?: Tint
  /** Names the reading. It is the meter's accessible label as well. */
  label?: ReactNode
  /** Prints the formatted value beside the label. */
  showValue?: boolean
  className?: string
}

/**
 * A reading inside a known range — disk used of disk there, spend against a
 * budget, an API quota.
 *
 * It is not a `Progress`, and the difference is not how they look. Progress
 * reports on something running: it moves on its own, it can be indeterminate,
 * and it ends. A meter is a measurement of something that already has a
 * value, so it never animates and never has none — which is why it prints
 * its number, and progress does not.
 */
export function Meter({
  size = 'medium',
  tint = 'primary',
  label,
  showValue = false,
  className,
  ...props
}: MeterProps) {
  return (
    <Base.Root
      data-size={size}
      data-tint={tint}
      className={cx('dz-meter', className)}
      {...props}
    >
      {label || showValue ? (
        <div className="dz-meter__head">
          {label ? (
            <Base.Label className="dz-meter__label">{label}</Base.Label>
          ) : null}
          {showValue ? <Base.Value className="dz-meter__value" /> : null}
        </div>
      ) : null}
      <Base.Track className="dz-meter__track">
        <Base.Indicator className="dz-meter__fill" />
      </Base.Track>
    </Base.Root>
  )
}
