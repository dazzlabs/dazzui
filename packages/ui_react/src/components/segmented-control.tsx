import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type SegmentedControlSize = 'small' | 'medium' | 'large'

export interface SegmentedItem<T extends string = string> {
  value: T
  label: ReactNode
  disabled?: boolean
}

export interface SegmentedControlProps<T extends string = string> extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'onChange' | 'children'
> {
  items: readonly SegmentedItem<T>[]
  value: T
  onValueChange?: (value: T) => void
  size?: SegmentedControlSize
  tint?: Tint
  /**
   * `raised` lifts the active segment off the track, the way a desktop
   * segmented control draws it. `filled` paints it with the tint — louder, and
   * worth reserving for a choice that has to shout.
   */
  activeStyle?: 'raised' | 'filled'
  /** Fill the available width, splitting it evenly between the segments. */
  stretch?: boolean
}

export function SegmentedControl<T extends string = string>({
  items,
  value,
  onValueChange,
  size = 'medium',
  tint = 'primary',
  activeStyle = 'raised',
  stretch = false,
  className,
  ...props
}: SegmentedControlProps<T>) {
  return (
    <div
      role="radiogroup"
      data-active-style={activeStyle}
      data-size={size}
      data-stretch={stretch}
      data-tint={tint}
      className={cx('dz-segmented-control', className)}
      {...props}
    >
      {items.map(item => (
        <button
          key={item.value}
          type="button"
          role="radio"
          aria-checked={item.value === value}
          disabled={item.disabled}
          className="dz-segmented-control__segment"
          onClick={() => onValueChange?.(item.value)}
        >
          {item.label}
        </button>
      ))}
    </div>
  )
}
