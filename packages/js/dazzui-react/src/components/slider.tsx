import { Slider as Base } from '@base-ui/react/slider'
import type { ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type SliderSize = 'small' | 'medium' | 'large'

export interface SliderProps extends Omit<
  Base.Root.Props<number | readonly number[]>,
  'className' | 'render' | 'children'
> {
  size?: SliderSize
  tint?: Tint
  /**
   * Prints the current value at the trailing edge of the row. A slider whose
   * value the reader has to be able to say — a font size, a threshold — needs
   * it; a slider they only have to feel — a volume — does not.
   */
  showValue?: boolean
  /** Replaces the printed value, for a unit the format options cannot give. */
  formatValue?: (values: readonly number[]) => ReactNode
  className?: string
}

/**
 * A value picked along a range.
 *
 * Base UI owns everything that makes one usable rather than merely draggable:
 * the arrow keys and their large step, the press on the track that jumps the
 * nearest thumb, the hidden inputs a form reads, and — for a range — which
 * thumb the pointer actually grabbed when the two are on top of each other.
 *
 * A range is the same component: pass an array and it draws a thumb per
 * value. There is no second component, because everything but the number of
 * thumbs is identical.
 */
export function Slider({
  size = 'medium',
  tint = 'primary',
  showValue = false,
  formatValue,
  className,
  value,
  defaultValue,
  ...props
}: SliderProps) {
  const current = value ?? defaultValue
  const thumbs = Array.isArray(current) ? current.length : 1

  return (
    <Base.Root
      value={value}
      defaultValue={defaultValue}
      data-size={size}
      data-tint={tint}
      className={cx('dz-slider', className)}
      {...props}
    >
      <div className="dz-slider__row">
        <Base.Control className="dz-slider__control">
          <Base.Track className="dz-slider__track">
            <Base.Indicator className="dz-slider__indicator" />
            {Array.from({ length: thumbs }, (_, index) => (
              <Base.Thumb key={index} className="dz-slider__thumb" />
            ))}
          </Base.Track>
        </Base.Control>
        {showValue ? (
          <Base.Value className="dz-slider__value">
            {formatValue
              ? (_formatted: readonly string[], values: readonly number[]) =>
                  formatValue(values)
              : undefined}
          </Base.Value>
        ) : null}
      </div>
    </Base.Root>
  )
}
