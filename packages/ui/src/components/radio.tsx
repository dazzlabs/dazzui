import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type RadioSize = 'small' | 'medium' | 'large'

export interface RadioProps extends ComponentPropsWithoutRef<'button'> {
  checked: boolean
  size?: RadioSize
  tint?: Tint
}

/** One choice. A set of them belongs in a `RadioGroup`. */
export function Radio({
  checked,
  size = 'medium',
  tint = 'primary',
  className,
  children,
  type = 'button',
  ...props
}: RadioProps) {
  return (
    <button
      type={type}
      role="radio"
      aria-checked={checked}
      data-size={size}
      data-tint={tint}
      className={cx('dz-radio', className)}
      {...props}
    >
      <span aria-hidden className="dz-radio__box">
        {checked ? <span className="dz-radio__dot" /> : null}
      </span>
      <span className="dz-radio__label">{children}</span>
    </button>
  )
}

export interface RadioGroupOption<T extends string = string> {
  value: T
  label: ReactNode
  disabled?: boolean
}

export interface RadioGroupProps<T extends string = string> extends Omit<
  ComponentPropsWithoutRef<'div'>,
  'onChange' | 'children'
> {
  options: readonly RadioGroupOption<T>[]
  value: T
  onValueChange?: (value: T) => void
  size?: RadioSize
  tint?: Tint
}

export function RadioGroup<T extends string = string>({
  options,
  value,
  onValueChange,
  size = 'medium',
  tint = 'primary',
  className,
  ...props
}: RadioGroupProps<T>) {
  return (
    <div
      role="radiogroup"
      className={cx('dz-radio-group', className)}
      {...props}
    >
      {options.map(option => (
        <Radio
          key={option.value}
          checked={option.value === value}
          disabled={option.disabled}
          size={size}
          tint={tint}
          onClick={() => onValueChange?.(option.value)}
        >
          {option.label}
        </Radio>
      ))}
    </div>
  )
}
