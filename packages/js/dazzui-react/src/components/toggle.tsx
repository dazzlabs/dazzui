import { Toggle as Base } from '@base-ui/react/toggle'
import { ToggleGroup as BaseGroup } from '@base-ui/react/toggle-group'
import type { ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'
import type { ButtonSize, ButtonVariant } from './button'
import { Tooltip } from './tooltip'

export interface ToggleProps extends Omit<Base.Props, 'className' | 'render'> {
  size?: ButtonSize
  variant?: ButtonVariant
  tint?: Tint
  /**
   * Accessible name and tooltip, for a toggle whose label is a glyph — which
   * is most of them, since these live in toolbars. A toggle with a word in it
   * needs none.
   */
  label?: string
  /**
   * Set by `ToggleGroup` on every member of a run. It is an axis rather than
   * a class because a grouped toggle really is drawn differently — square
   * corners on the inside of the run, and its edge collapsed into its
   * neighbour's — and the DOM is where this system says that.
   */
  grouped?: boolean
  className?: string
}

/**
 * A button that stays down.
 *
 * The distinction it draws against everything nearby is worth stating,
 * because three components in this package are one press away from each
 * other. A `Switch` commits immediately and reads as a setting. A
 * `SegmentedControl` is a choice *between* its segments and always has one.
 * This is neither: it is a single property that is on or off — bold, wrap,
 * show whitespace — and the row it sits in may have none of them on.
 *
 * Pressed is drawn as the system draws every selection: the tinted wash, and
 * the label a weight heavier. Colour alone would be one grade in a toolbar
 * of glyphs, which is exactly where it is hardest to see.
 */
export function Toggle({
  size = 'tiny',
  variant = 'plain',
  // Neutral, for the same reason `IconButton` is: a toolbar of these is
  // chrome, and a row of accented glyphs reads as a row of actions.
  tint = 'neutral',
  label,
  grouped = false,
  className,
  children,
  ...props
}: ToggleProps) {
  const toggle = (
    <Base
      aria-label={label}
      data-control=""
      data-grouped={grouped || undefined}
      data-size={size}
      data-tint={tint}
      data-variant={variant}
      className={cx('dz-toggle', className)}
      {...props}
    >
      {children}
    </Base>
  )

  return label ? <Tooltip label={label}>{toggle}</Tooltip> : toggle
}

export interface ToggleItem<T extends string = string> {
  value: T
  label: ReactNode
  /** Accessible name and tooltip, where the label is a glyph. */
  hint?: string
  disabled?: boolean
}

export interface ToggleGroupProps<T extends string = string> extends Omit<
  BaseGroup.Props<T>,
  'className' | 'render' | 'children' | 'value' | 'onValueChange'
> {
  items: readonly ToggleItem<T>[]
  value: readonly T[]
  onValueChange?: (value: T[]) => void
  size?: ButtonSize
  variant?: ButtonVariant
  tint?: Tint
  className?: string
}

/**
 * Several toggles that belong together, joined into one control.
 *
 * Joined rather than merely adjacent: the corners are rounded at the ends of
 * the run only and the shared edges collapse to one, which is what says the
 * group is one thing with several states rather than three buttons that
 * happen to touch. Base UI owns the part that is easy to get wrong — the
 * group is one tab stop and the arrow keys move inside it.
 *
 * `multiple` is where it parts from `SegmentedControl`: several on at once,
 * or none. A group that must always have exactly one is that component, and
 * drawing it with this one would let the reader clear a choice that cannot
 * be cleared.
 */
export function ToggleGroup<T extends string = string>({
  items,
  value,
  onValueChange,
  size = 'small',
  variant = 'recessed',
  tint = 'primary',
  multiple = true,
  className,
  ...props
}: ToggleGroupProps<T>) {
  return (
    <BaseGroup
      value={value}
      onValueChange={next => onValueChange?.(next)}
      multiple={multiple}
      className={cx('dz-toggle-group', className)}
      {...props}
    >
      {items.map(item => (
        <Toggle
          key={item.value}
          value={item.value}
          label={item.hint}
          grouped
          disabled={item.disabled}
          size={size}
          variant={variant}
          tint={tint}
        >
          {item.label}
        </Toggle>
      ))}
    </BaseGroup>
  )
}
