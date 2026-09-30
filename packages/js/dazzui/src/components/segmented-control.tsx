import { Toggle } from '@base-ui/react/toggle'
import { ToggleGroup } from '@base-ui/react/toggle-group'
import type { ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type SegmentedControlSize = 'small' | 'medium' | 'large'

export interface SegmentedItem<T extends string = string> {
  value: T
  label: ReactNode
  disabled?: boolean
}

export interface SegmentedControlProps<T extends string = string> extends Omit<
  ToggleGroup.Props,
  | 'className'
  | 'render'
  | 'children'
  | 'value'
  | 'defaultValue'
  | 'onValueChange'
  | 'multiple'
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
  className?: string
}

/**
 * A single-select toggle group that always has a selection: pressing the
 * active segment again is ignored rather than clearing it, because a
 * segmented control is a choice between its segments, not a switch.
 */
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
    <ToggleGroup
      value={[value]}
      onValueChange={next => {
        const [selected] = next as T[]
        if (selected !== undefined) onValueChange?.(selected)
      }}
      data-active-style={activeStyle}
      data-size={size}
      data-stretch={stretch}
      data-tint={tint}
      className={cx('dz-segmented-control', className)}
      {...props}
    >
      {items.map(item => (
        <Toggle
          key={item.value}
          value={item.value}
          disabled={item.disabled}
          className="dz-segmented-control__segment"
        >
          {item.label}
        </Toggle>
      ))}
    </ToggleGroup>
  )
}
