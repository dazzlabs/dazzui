import { Dismiss12Regular } from '@fluentui/react-icons'
import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'
import type { BadgeSize, BadgeVariant } from './badge'

export type TagSize = BadgeSize
export type TagVariant = BadgeVariant

export interface TagProps extends ComponentPropsWithoutRef<'span'> {
  size?: TagSize
  variant?: TagVariant
  tint?: Tint
  /** A glyph before the label — a status dot, an author's avatar. */
  icon?: ReactNode
  /**
   * Latched. Passing it at all is what makes the label pressable, and a
   * selected tag is drawn with the `tinted` recipe and the tint's edge
   * whatever `variant` says — a filter that is on has to read as on from
   * across the row, not as one grade of the same chip.
   */
  selected?: boolean
  onSelectedChange?: (selected: boolean) => void
  /** Draws the dismiss button, and is what it does. */
  onRemove?: () => void
  /** The dismiss button's accessible name — say what is being removed. */
  removeLabel?: string
  disabled?: boolean
}

/**
 * A `Badge` you can act on.
 *
 * The two are one drawing and two components rather than one component with
 * an `interactive` prop, because the difference is not cosmetic: a badge
 * reads its recipes at rest and deliberately stays out of the shared state
 * machine, since a label that moves under the pointer reads as something you
 * can press. A tag *is* something you can press, so it takes the machine —
 * and taking it is the whole reason the component exists.
 *
 * The dismiss button is its own control rather than a second job for the
 * label: a chip that removes itself when clicked has no way to be selected,
 * and one that does both from the same target removes things by accident.
 */
export function Tag({
  size = 'medium',
  variant = 'tinted',
  tint = 'primary',
  icon,
  selected,
  onSelectedChange,
  onRemove,
  removeLabel = 'Remove',
  disabled = false,
  className,
  children,
  ...props
}: TagProps) {
  const pressable = selected !== undefined

  return (
    <span
      /* Only a pressable tag opts into the state machine. One that merely
         carries a dismiss button is still a label, and a wash that answered
         the pointer anywhere on the capsule would promise a target that is
         not there. */
      data-control={pressable ? '' : undefined}
      data-disabled={disabled || undefined}
      data-removable={onRemove !== undefined}
      data-selected={pressable ? selected : undefined}
      data-size={size}
      data-tint={tint}
      data-variant={pressable && selected ? 'tinted' : variant}
      aria-disabled={pressable && disabled ? true : undefined}
      className={cx('dz-tag', className)}
      {...props}
    >
      {icon ? (
        <span aria-hidden className="dz-tag__icon">
          {icon}
        </span>
      ) : null}
      {pressable ? (
        <button
          type="button"
          aria-pressed={selected}
          disabled={disabled}
          onClick={() => onSelectedChange?.(!selected)}
          className="dz-tag__label"
        >
          {children}
        </button>
      ) : (
        <span className="dz-tag__label">{children}</span>
      )}
      {onRemove ? (
        <button
          type="button"
          aria-label={removeLabel}
          disabled={disabled}
          onClick={onRemove}
          className="dz-tag__remove"
        >
          <Dismiss12Regular />
        </button>
      ) : null}
    </span>
  )
}
