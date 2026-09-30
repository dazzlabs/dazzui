import { Field } from '@base-ui/react/field'
import { Input } from '@base-ui/react/input'
import { type ReactNode, useId, useState } from 'react'

import { cx } from '../lib/cx'
import { ClearCircle } from '../lib/glyphs'
import { usePlatform } from '../lib/theme'

export interface TextFieldProps extends Omit<
  Input.Props,
  'className' | 'render' | 'value' | 'defaultValue' | 'onValueChange'
> {
  /**
   * The name of the field. Material floats it from the box to its top
   * edge; iOS sets it above the field in the footnote style, since a
   * UITextField has no label of its own.
   */
  label?: ReactNode
  /** Under the field: a hint, or the error when `error` is set. */
  supporting?: ReactNode
  /** Puts the field in its error colour. */
  error?: boolean
  /** Material's two boxes; iOS draws one field either way. */
  variant?: 'outlined' | 'filled'
  /** Glyphs inside the box. */
  leading?: ReactNode
  trailing?: ReactNode
  /** iOS's clear button, shown while the field has text. */
  clearable?: boolean
  value?: string
  defaultValue?: string
  onValueChange?: (value: string) => void
  className?: string
}

/**
 * UITextField and Material's text field. The box is what differs: iOS
 * draws a rounded rectangle on the surface and keeps its label outside;
 * Material draws an outline or a filled box and floats the label into it.
 * Base UI's field wires the label, the description and the error state.
 */
export function TextField({
  label,
  supporting,
  error = false,
  variant = 'outlined',
  leading,
  trailing,
  clearable = true,
  value,
  defaultValue,
  onValueChange,
  className,
  disabled,
  placeholder,
  ...props
}: TextFieldProps) {
  const platform = usePlatform()
  const [inner, setInner] = useState(defaultValue ?? '')
  const text = value ?? inner
  const set = (next: string) => {
    if (value === undefined) setInner(next)
    onValueChange?.(next)
  }
  const id = useId()

  return (
    <Field.Root
      data-platform={platform}
      data-variant={variant}
      data-filled={text ? '' : undefined}
      data-leading={leading ? '' : undefined}
      invalid={error}
      disabled={disabled}
      className={cx('dzm-field', className)}
    >
      {platform === 'ios' && label ? (
        <Field.Label className="dzm-field__label">{label}</Field.Label>
      ) : null}
      <div className="dzm-field__box">
        {leading ? <span className="dzm-field__leading">{leading}</span> : null}
        <Input
          id={id}
          className="dzm-field__input"
          value={text}
          onValueChange={set}
          placeholder={platform === 'android' && label ? ' ' : placeholder}
          {...props}
        />
        {platform === 'android' && label ? (
          <Field.Label className="dzm-field__label">{label}</Field.Label>
        ) : null}
        {platform === 'ios' && clearable && text && !disabled ? (
          <button
            type="button"
            aria-label="Clear text"
            className="dzm-field__clear"
            onClick={() => set('')}
          >
            <ClearCircle />
          </button>
        ) : null}
        {trailing ? (
          <span className="dzm-field__trailing">{trailing}</span>
        ) : null}
      </div>
      {supporting ? (
        error ? (
          <Field.Error className="dzm-field__supporting" match>
            {supporting}
          </Field.Error>
        ) : (
          <Field.Description className="dzm-field__supporting">
            {supporting}
          </Field.Description>
        )
      ) : null}
    </Field.Root>
  )
}
