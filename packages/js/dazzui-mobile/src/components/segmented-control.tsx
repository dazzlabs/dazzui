import { Toggle } from '@base-ui/react/toggle'
import { ToggleGroup } from '@base-ui/react/toggle-group'
import type { ReactNode } from 'react'

import { cx } from '../lib/cx'
import { usePlatform } from '../lib/theme'

export interface Segment {
  value: string
  label: ReactNode
  disabled?: boolean
}

export interface SegmentedControlProps {
  segments: readonly Segment[]
  value?: string
  defaultValue?: string
  onValueChange?: (value: string) => void
  disabled?: boolean
  /** A name for the group; the segments are its options. */
  'aria-label'?: string
  className?: string
}

/**
 * UISegmentedControl, in its glass form, and Material Expressive's
 * connected button group: one of several, always one. Base UI's toggle
 * group roves the focus; the wrapper only refuses to let the last segment
 * be switched off, since a segmented control has no empty state.
 */
export function SegmentedControl({
  segments,
  value,
  defaultValue,
  onValueChange,
  disabled,
  className,
  ...props
}: SegmentedControlProps) {
  const platform = usePlatform()
  const first = segments[0]?.value
  return (
    <ToggleGroup
      data-platform={platform}
      className={cx('dzm-segmented', className)}
      aria-label={props['aria-label']}
      disabled={disabled}
      value={value === undefined ? undefined : [value]}
      defaultValue={
        value === undefined
          ? [defaultValue ?? first].filter((v): v is string => v !== undefined)
          : undefined
      }
      onValueChange={next => {
        const chosen = next[next.length - 1]
        if (chosen !== undefined) onValueChange?.(chosen)
      }}
    >
      {segments.map(segment => (
        <Toggle
          key={segment.value}
          value={segment.value}
          disabled={segment.disabled}
          className="dzm-segmented__segment"
        >
          {segment.label}
        </Toggle>
      ))}
    </ToggleGroup>
  )
}
