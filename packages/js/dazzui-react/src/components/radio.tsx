import { Radio as Base } from '@base-ui/react/radio'
import { RadioGroup as BaseGroup } from '@base-ui/react/radio-group'
import type { ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type RadioSize = 'small' | 'medium' | 'large'

export interface RadioProps extends Omit<
  Base.Root.Props,
  'className' | 'render'
> {
  size?: RadioSize
  tint?: Tint
  className?: string
}

/**
 * One choice. It only means something inside a `RadioGroup`, which is where
 * the selection lives — a radio outside one is never checked.
 */
export function Radio({
  size = 'medium',
  tint = 'primary',
  className,
  children,
  ...props
}: RadioProps) {
  return (
    <Base.Root
      data-size={size}
      data-tint={tint}
      className={cx('dz-radio', className)}
      {...props}
    >
      <span aria-hidden className="dz-radio__box">
        <Base.Indicator className="dz-radio__dot" />
      </span>
      <span className="dz-radio__label">{children}</span>
    </Base.Root>
  )
}

export interface RadioGroupOption<T extends string = string> {
  value: T
  label: ReactNode
  disabled?: boolean
}

export interface RadioGroupProps<T extends string = string> extends Omit<
  BaseGroup.Props,
  | 'className'
  | 'render'
  | 'children'
  | 'value'
  | 'defaultValue'
  | 'onValueChange'
> {
  options: readonly RadioGroupOption<T>[]
  value?: T
  defaultValue?: T
  onValueChange?: (value: T) => void
  size?: RadioSize
  tint?: Tint
  className?: string
}

export function RadioGroup<T extends string = string>({
  options,
  onValueChange,
  size = 'medium',
  tint = 'primary',
  className,
  ...props
}: RadioGroupProps<T>) {
  return (
    <BaseGroup
      className={cx('dz-radio-group', className)}
      onValueChange={
        onValueChange ? value => onValueChange(value as T) : undefined
      }
      {...props}
    >
      {options.map(option => (
        <Radio
          key={option.value}
          value={option.value}
          disabled={option.disabled}
          size={size}
          tint={tint}
        >
          {option.label}
        </Radio>
      ))}
    </BaseGroup>
  )
}
