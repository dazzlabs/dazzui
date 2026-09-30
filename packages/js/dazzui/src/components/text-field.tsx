import { Input } from '@base-ui/react/input'
import { Select as Base } from '@base-ui/react/select'
import { ChevronDown12Regular, Checkmark12Filled } from '@fluentui/react-icons'
import type { ComponentPropsWithoutRef, ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'

export type TextFieldSize = 'small' | 'medium' | 'large'

export type TextFieldState = 'default' | 'error'

/**
 * An erroring control is re-tinted rather than given a second palette, so its
 * border, wash and focus ring all move together.
 */
export function tintFor(state: TextFieldState, tint: Tint): Tint {
  return state === 'error' ? 'danger' : tint
}

type ControlBase = {
  size?: TextFieldSize
  state?: TextFieldState
  tint?: Tint
  /** Set the mono face — for a key, an ID, or a URL. */
  mono?: boolean
  className?: string
}

/**
 * One field, two elements. `multiline` is the axis rather than a second
 * component, because the box, the states and the tinting are the same design
 * either way — only the corner, the leading and how the height is arrived at
 * differ, and those are three declarations rather than a component's worth.
 *
 * The prop discriminates the type as well as the element, so `rows` is
 * available only on a multiline field and `type` only on a single-line one,
 * exactly as the two DOM elements have it.
 */
export type TextFieldProps =
  | (ControlBase & {
      multiline?: false
    } & Omit<Input.Props, 'size' | 'className' | 'render'>)
  | (ControlBase & {
      multiline: true
    } & Omit<ComponentPropsWithoutRef<'textarea'>, 'size' | 'className'>)

/**
 * Base UI's `Input` under both elements, so a field inside a `FormField` is
 * labelled and described by it. The textarea is the same control rendered
 * onto a different element: its own props (`rows`, a wider `onChange`) are
 * not in the input's type, which is what the cast is for — at runtime Base UI
 * merges them onto the element it renders.
 */
export function TextField({
  size = 'medium',
  state = 'default',
  tint = 'primary',
  mono = false,
  multiline,
  className,
  ...props
}: TextFieldProps) {
  const shared = {
    'aria-invalid': state === 'error' || undefined,
    'data-mono': mono,
    'data-size': size,
    'data-tint': tintFor(state, tint),
    className: cx('dz-text-field', className),
  }

  return multiline ? (
    <Input
      data-multiline=""
      render={<textarea />}
      {...shared}
      {...(props as Input.Props)}
    />
  ) : (
    <Input {...shared} {...(props as Input.Props)} />
  )
}

export interface SelectOption<T extends string = string> {
  value: T
  label: ReactNode
  disabled?: boolean
}

export interface SelectGroup<T extends string = string> {
  label: ReactNode
  options: readonly SelectOption<T>[]
}

export interface SelectProps<T extends string = string> extends ControlBase {
  /** Flat, or grouped under a heading the way `<optgroup>` groups. */
  options: readonly (SelectOption<T> | SelectGroup<T>)[]
  value?: T
  defaultValue?: T
  onValueChange?: (value: T) => void
  /** What the trigger prints while nothing is chosen. */
  placeholder?: ReactNode
  name?: string
  id?: string
  disabled?: boolean
  required?: boolean
  readOnly?: boolean
  'aria-label'?: string
  'aria-labelledby'?: string
}

function isGroup<T extends string>(
  entry: SelectOption<T> | SelectGroup<T>
): entry is SelectGroup<T> {
  return 'options' in entry
}

/**
 * A choice out of a list too long to lay out.
 *
 * It was a native `<select>` until the popups moved to Base UI; now the list
 * is the same panel a `Menu` opens, so the two match, and it opens over its
 * trigger with the chosen item under the pointer, the way a desktop pop-up
 * button does. The trigger reads the TextField namespace rather than owning
 * one: looking exactly like an input is the design, and a second set of
 * numbers could only drift from it.
 */
export function Select<T extends string = string>({
  options,
  value,
  defaultValue,
  onValueChange,
  placeholder = 'Choose…',
  size = 'medium',
  state = 'default',
  tint = 'primary',
  mono = false,
  className,
  name,
  id,
  disabled,
  required,
  readOnly,
  'aria-label': ariaLabel,
  'aria-labelledby': ariaLabelledBy,
}: SelectProps<T>) {
  const flat = options.flatMap(entry =>
    isGroup(entry) ? entry.options : entry
  )
  const labels = new Map(flat.map(option => [option.value, option.label]))

  const renderOption = (option: SelectOption<T>) => (
    <Base.Item
      key={option.value}
      value={option.value}
      disabled={option.disabled}
      className="dz-popup__item"
    >
      <span aria-hidden className="dz-popup__check">
        <Checkmark12Filled />
      </span>
      <Base.ItemText className="dz-popup__label">{option.label}</Base.ItemText>
    </Base.Item>
  )

  return (
    <Base.Root<T>
      items={flat}
      value={value}
      defaultValue={defaultValue}
      onValueChange={next => {
        if (next !== null) onValueChange?.(next)
      }}
      name={name}
      id={id}
      disabled={disabled}
      required={required}
      readOnly={readOnly}
    >
      <Base.Trigger
        aria-label={ariaLabel}
        aria-labelledby={ariaLabelledBy}
        aria-invalid={state === 'error' || undefined}
        data-mono={mono}
        data-size={size}
        data-tint={tintFor(state, tint)}
        className={cx('dz-select', className)}
      >
        <Base.Value className="dz-select__value">
          {(current: T | null) =>
            current === null ? placeholder : (labels.get(current) ?? current)
          }
        </Base.Value>
        <Base.Icon className="dz-select__chevron">
          <ChevronDown12Regular />
        </Base.Icon>
      </Base.Trigger>
      <Base.Portal>
        <Base.Positioner data-tint={tint} className="dz-popup-positioner">
          <Base.Popup className="dz-popup dz-select__popup">
            <Base.List className="dz-popup__list">
              {options.map((entry, index) =>
                isGroup(entry) ? (
                  <Base.Group key={index} className="dz-popup__group">
                    <Base.GroupLabel className="dz-popup__group-label">
                      {entry.label}
                    </Base.GroupLabel>
                    {entry.options.map(renderOption)}
                  </Base.Group>
                ) : (
                  renderOption(entry)
                )
              )}
            </Base.List>
          </Base.Popup>
        </Base.Positioner>
      </Base.Portal>
    </Base.Root>
  )
}

export interface FieldValueProps
  extends ComponentPropsWithoutRef<'div'>, Pick<ControlBase, 'size' | 'mono'> {}

/**
 * A value in the shape of a field, for a row that reports rather than edits.
 * Not a disabled field: nothing here was ever editable, so nothing is greyed
 * out to say it stopped being.
 */
export function FieldValue({
  size = 'medium',
  mono = false,
  className,
  ...props
}: FieldValueProps) {
  return (
    <div
      data-mono={mono}
      data-size={size}
      className={cx('dz-field-value', className)}
      {...props}
    />
  )
}
