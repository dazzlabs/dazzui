import { Checkbox as Base } from '@base-ui/react/checkbox'
import { CheckboxGroup as BaseGroup } from '@base-ui/react/checkbox-group'
import { Checkmark12Filled, Subtract12Filled } from '@fluentui/react-icons'
import type { ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type CheckboxSize = 'small' | 'medium' | 'large'

export interface CheckboxProps extends Omit<
  Base.Root.Props,
  'className' | 'render'
> {
  size?: CheckboxSize
  tint?: Tint
  /** A de-emphasised aside on the label — a count, a size. */
  note?: ReactNode
  className?: string
}

/**
 * The behaviour — the hidden input, the mixed state, the form wiring, the
 * label association inside a `FormField` — is Base UI's. What is drawn here
 * is the box and the row it sits on.
 */
export function Checkbox({
  size = 'medium',
  tint = 'primary',
  note,
  className,
  children,
  ...props
}: CheckboxProps) {
  return (
    <Base.Root
      data-size={size}
      data-tint={tint}
      className={cx('dz-checkbox', className)}
      {...props}
    >
      <span aria-hidden className="dz-checkbox__box">
        {/* Both glyphs, switched in CSS. The component cannot ask whether it
            is mixed — the state lives in Base UI and arrives as an attribute
            on the root — and rendering only the tick is what made a parent
            with two of four children checked claim all four. */}
        <Base.Indicator className="dz-checkbox__mark">
          <Checkmark12Filled className="dz-checkbox__tick" />
          <Subtract12Filled className="dz-checkbox__dash" />
        </Base.Indicator>
      </span>
      <span className="dz-checkbox__label">
        {children}
        {note ? <span className="dz-checkbox__note"> {note}</span> : null}
      </span>
    </Base.Root>
  )
}

export interface CheckboxGroupProps extends Omit<
  BaseGroup.Props,
  'className' | 'render'
> {
  className?: string
}

/**
 * Several checkboxes that are one answer, and the parent that turns them all
 * on or off.
 *
 * The parent is where this earns its place. A checkbox with `parent` set
 * reads the group rather than a value of its own: it is checked when every
 * child is, *mixed* when only some are — the state a lone checkbox can never
 * arrive at by itself — and pressing it sets or clears the lot, restoring
 * what was checked before if it is pressed twice. `allValues` is what tells
 * the group which children exist; without it the parent has nothing to
 * count.
 *
 * The children of a parent are indented to its label, which is what makes the
 * relationship visible rather than merely wired. They are plain siblings of
 * the parent rather than a nested group: a `CheckboxGroup` is a context
 * provider, so a group inside a group would cut them off from the parent
 * that is counting them.
 */
export function CheckboxGroup({ className, ...props }: CheckboxGroupProps) {
  return <BaseGroup className={cx('dz-checkbox-group', className)} {...props} />
}
