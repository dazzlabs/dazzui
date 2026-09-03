import { Checkbox as Base } from '@base-ui/react/checkbox'
import { Checkmark12Filled } from '@fluentui/react-icons'
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
        <Base.Indicator className="dz-checkbox__mark">
          <Checkmark12Filled />
        </Base.Indicator>
      </span>
      <span className="dz-checkbox__label">
        {children}
        {note ? <span className="dz-checkbox__note"> {note}</span> : null}
      </span>
    </Base.Root>
  )
}
