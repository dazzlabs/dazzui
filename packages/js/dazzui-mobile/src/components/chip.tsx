import { Toggle } from '@base-ui/react/toggle'
import type { ComponentProps, ReactNode } from 'react'

import { cx } from '../lib/cx'
import { Check, Close } from '../lib/glyphs'
import { usePlatform } from '../lib/theme'

export interface ChipProps extends Omit<ComponentProps<'button'>, 'onSelect'> {
  /** A leading glyph. */
  icon?: ReactNode
  /** Makes it a filter: pressing toggles `selected`. */
  selected?: boolean
  defaultSelected?: boolean
  onSelectedChange?: (selected: boolean) => void
  /** Makes it an input chip: a dismiss glyph at the trailing edge. */
  onRemove?: () => void
  /** Material's elevated chip; iOS's chips are glass already. */
  elevated?: boolean
}

/**
 * The capsule a filter or a tag sits in. On iOS it is the glass capsule
 * Maps and Photos filter with, filled in the tint when chosen; on Android
 * Material's chip — assist, filter or input by which props it is given,
 * the chosen filter on the secondary container with a leading check.
 */
export function Chip({
  icon,
  selected,
  defaultSelected,
  onSelectedChange,
  onRemove,
  elevated = false,
  className,
  children,
  type = 'button',
  ...props
}: ChipProps) {
  const platform = usePlatform()
  const selectable =
    selected !== undefined ||
    defaultSelected !== undefined ||
    Boolean(onSelectedChange)
  const shared = {
    'data-platform': platform,
    'data-elevated': elevated || undefined,
    'data-removable': onRemove ? '' : undefined,
    className: cx(
      'dzm-chip',
      platform === 'ios' && 'dzm-glass',
      'dzm-m3-layer',
      className
    ),
  }
  const content = (
    <>
      {selectable ? (
        <span className="dzm-chip__check">
          <Check />
        </span>
      ) : null}
      {icon ? <span className="dzm-chip__icon">{icon}</span> : null}
      <span className="dzm-chip__label">{children}</span>
      {onRemove ? (
        <span
          role="button"
          tabIndex={0}
          aria-label="Remove"
          className="dzm-chip__remove"
          onClick={event => {
            event.stopPropagation()
            onRemove()
          }}
          onKeyDown={event => {
            if (event.key === 'Enter' || event.key === ' ') {
              event.preventDefault()
              event.stopPropagation()
              onRemove()
            }
          }}
        >
          <Close />
        </span>
      ) : null}
    </>
  )
  if (selectable) {
    return (
      <Toggle
        pressed={selected}
        defaultPressed={defaultSelected}
        onPressedChange={onSelectedChange}
        {...shared}
        {...(props as Omit<Toggle.Props, 'className'>)}
      >
        {content}
      </Toggle>
    )
  }
  return (
    <button type={type} {...shared} {...props}>
      {content}
    </button>
  )
}
