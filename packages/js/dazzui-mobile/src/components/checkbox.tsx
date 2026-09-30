import { Checkbox as Base } from '@base-ui/react/checkbox'
import type { ReactNode } from 'react'

import { cx } from '../lib/cx'
import { Check, Minus } from '../lib/glyphs'
import { usePlatform } from '../lib/theme'

export interface CheckboxProps extends Omit<
  Base.Root.Props,
  'className' | 'render' | 'children'
> {
  /** The label beside the mark; the whole row is the target. */
  children?: ReactNode
  className?: string
}

/**
 * Material's checkbox, and the circled check iOS uses for the same job —
 * the mark a list row takes in edit mode, and Reminders' completion
 * circle. iOS has no square checkbox: that control is macOS's.
 */
export function Checkbox({ className, children, ...props }: CheckboxProps) {
  const platform = usePlatform()
  const mark = (
    <Base.Root
      data-platform={platform}
      className={cx('dzm-checkbox', !children && className)}
      {...props}
    >
      <Base.Indicator className="dzm-checkbox__indicator" keepMounted>
        <Check className="dzm-checkbox__check" />
        <Minus className="dzm-checkbox__dash" />
      </Base.Indicator>
    </Base.Root>
  )
  if (!children) return mark
  return (
    <label
      data-platform={platform}
      className={cx('dzm-checkbox-row', className)}
    >
      {mark}
      <span className="dzm-checkbox-row__label">{children}</span>
    </label>
  )
}
