import { Radio as Base } from '@base-ui/react/radio'
import { RadioGroup as BaseGroup } from '@base-ui/react/radio-group'
import type { ReactNode } from 'react'

import { cx } from '../lib/cx'
import { Check } from '../lib/glyphs'
import { usePlatform } from '../lib/theme'

export interface RadioGroupProps extends Omit<
  BaseGroup.Props,
  'className' | 'render'
> {
  className?: string
}

/**
 * The group the arrow keys move through. On iOS a group of choices is a
 * list whose chosen row carries a check, so the rows draw themselves that
 * way; on Android each row is a radio button and its label.
 */
export function RadioGroup({ className, ...props }: RadioGroupProps) {
  const platform = usePlatform()
  return (
    <BaseGroup
      data-platform={platform}
      className={cx('dzm-radio-group', className)}
      {...props}
    />
  )
}

export interface RadioProps extends Omit<
  Base.Root.Props,
  'className' | 'render' | 'children'
> {
  children?: ReactNode
  className?: string
}

export function Radio({ className, children, ...props }: RadioProps) {
  const platform = usePlatform()
  return (
    <label data-platform={platform} className={cx('dzm-radio', className)}>
      <Base.Root
        data-platform={platform}
        className="dzm-radio__control"
        {...props}
      >
        <Base.Indicator className="dzm-radio__indicator" keepMounted>
          {platform === 'ios' ? <Check className="dzm-radio__check" /> : null}
        </Base.Indicator>
      </Base.Root>
      <span className="dzm-radio__label">{children}</span>
    </label>
  )
}
