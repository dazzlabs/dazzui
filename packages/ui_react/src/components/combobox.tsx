import { Autocomplete as ABase } from '@base-ui/react/autocomplete'
import { Combobox as Base } from '@base-ui/react/combobox'
import {
  Checkmark12Filled,
  ChevronDown12Regular,
  Dismiss12Regular,
} from '@fluentui/react-icons'
import type { ReactNode } from 'react'

import { cx } from '../lib/cx'
import type { Tint } from '../lib/tint'
import { type TextFieldSize, type TextFieldState, tintFor } from './text-field'

export interface ComboboxOption<T extends string = string> {
  value: T
  /** What is shown, and what the query is matched against. */
  label: string
  disabled?: boolean
}

export interface ComboboxOptionGroup<T extends string = string> {
  label: string
  options: readonly ComboboxOption<T>[]
}

/**
 * Grouped or flat, never both: Base UI reads the whole list one way or the
 * other, so a mixed array would silently lose either the groups or the loose
 * options. The first entry decides.
 */
function isGroup<T extends string>(
  entry: ComboboxOption<T> | ComboboxOptionGroup<T>
): entry is ComboboxOptionGroup<T> {
  return 'options' in entry
}

/** Base UI's own group shape: a label under `value`, and the items under it. */
type BaseGroup<T extends string> = {
  value: string
  items: readonly ComboboxOption<T>[]
}

export interface ComboboxProps<T extends string = string> {
  /** Flat, or grouped under a heading the way `<optgroup>` groups. */
  options: readonly (ComboboxOption<T> | ComboboxOptionGroup<T>)[]
  value?: T | null
  defaultValue?: T
  onValueChange?: (value: T | null) => void
  /** What the field prints while nothing is chosen. */
  placeholder?: string
  /** What the list says when the query matches nothing. */
  empty?: ReactNode
  /** Offers a control that clears the chosen option. */
  clearable?: boolean
  size?: TextFieldSize
  state?: TextFieldState
  tint?: Tint
  name?: string
  id?: string
  disabled?: boolean
  required?: boolean
  readOnly?: boolean
  className?: string
}

/**
 * A choice out of a list too long to scroll: the reader types to narrow it,
 * and picks one of what is left.
 *
 * It is a `Select` with a query in front of it, and that is the only question
 * worth asking when choosing between them — a list of six is a `Select`, a
 * list of six hundred is this. Both end in one value; neither lets a reader
 * enter something that is not on the list, which is what separates them from
 * an `Autocomplete`.
 *
 * The filtering, the highlight the arrow keys move, and the announcement of
 * how many matches are left are Base UI's. The box is the `TextField`'s and
 * the list is the shared popup sheet, so a combobox in a form sits on the
 * same rhythm as everything around it.
 */
export function Combobox<T extends string = string>({
  options,
  value,
  defaultValue,
  onValueChange,
  placeholder = 'Search…',
  empty = 'No matches',
  clearable = true,
  size = 'medium',
  state = 'default',
  tint = 'primary',
  name,
  id,
  disabled,
  required,
  readOnly,
  className,
}: ComboboxProps<T>) {
  const grouped = options.length > 0 && isGroup(options[0]!)
  const flat = options.flatMap(entry =>
    isGroup(entry) ? [...entry.options] : [entry]
  )
  const items = grouped
    ? options.map(entry =>
        isGroup(entry)
          ? { value: entry.label, items: entry.options }
          : { value: entry.label, items: [entry] }
      )
    : flat

  const optionFor = (next: T | null | undefined) =>
    next === undefined ? undefined : (flat.find(o => o.value === next) ?? null)

  const renderItem = (item: ComboboxOption<T>) => (
    <Base.Item
      key={item.value}
      value={item}
      disabled={item.disabled}
      className="dz-popup__item"
    >
      <span aria-hidden className="dz-popup__check">
        <Checkmark12Filled />
      </span>
      <span className="dz-popup__label">{item.label}</span>
    </Base.Item>
  )

  return (
    <Base.Root<ComboboxOption<T>>
      items={items}
      value={optionFor(value)}
      defaultValue={optionFor(defaultValue)}
      onValueChange={next => onValueChange?.(next?.value ?? null)}
      isItemEqualToValue={(a, b) => a?.value === b?.value}
      name={name}
      disabled={disabled}
      required={required}
      readOnly={readOnly}
    >
      <Base.InputGroup
        data-size={size}
        data-tint={tintFor(state, tint)}
        className={cx('dz-combobox__group', className)}
      >
        <Base.Input
          id={id}
          placeholder={placeholder}
          aria-invalid={state === 'error' || undefined}
          className="dz-combobox__input"
        />
        <span className="dz-combobox__actions">
          {clearable ? (
            <Base.Clear aria-label="Clear" className="dz-combobox__action">
              <Dismiss12Regular />
            </Base.Clear>
          ) : null}
          <Base.Trigger aria-label="Open" className="dz-combobox__action">
            <Base.Icon>
              <ChevronDown12Regular />
            </Base.Icon>
          </Base.Trigger>
        </span>
      </Base.InputGroup>
      <Base.Portal>
        <Base.Positioner data-tint={tint} className="dz-popup-positioner">
          <Base.Popup className="dz-popup dz-combobox__popup">
            <Base.Empty className="dz-popup__empty">{empty}</Base.Empty>
            <Base.List className="dz-popup__list">
              {grouped
                ? (group: BaseGroup<T>) => (
                    <Base.Group
                      key={group.value}
                      items={group.items}
                      className="dz-popup__group"
                    >
                      <Base.GroupLabel className="dz-popup__group-label">
                        {group.value}
                      </Base.GroupLabel>
                      <Base.Collection>{renderItem}</Base.Collection>
                    </Base.Group>
                  )
                : renderItem}
            </Base.List>
          </Base.Popup>
        </Base.Positioner>
      </Base.Portal>
    </Base.Root>
  )
}

export interface AutocompleteProps {
  /** The suggestions offered as the reader types. */
  items: readonly string[]
  value?: string
  defaultValue?: string
  onValueChange?: (value: string) => void
  placeholder?: string
  /** What the list says when the query matches nothing. */
  empty?: ReactNode
  size?: TextFieldSize
  state?: TextFieldState
  tint?: Tint
  /** Set the mono face — for a path, a branch name, a key. */
  mono?: boolean
  name?: string
  id?: string
  disabled?: boolean
  required?: boolean
  readOnly?: boolean
  className?: string
}

/**
 * A text field that suggests as you type.
 *
 * The difference from a `Combobox` is what the value is. A combobox ends in
 * one of its options and nothing else; an autocomplete's value is whatever
 * was typed, and the list is a shortcut to a likely answer rather than the
 * set of allowed ones. A branch name, a search query, a tag that may not
 * exist yet.
 *
 * So there is no chevron and no clear: nothing here is opened or chosen, and
 * the field is emptied the way any text field is.
 */
export function Autocomplete({
  items,
  value,
  defaultValue,
  onValueChange,
  placeholder,
  empty = 'No matches',
  size = 'medium',
  state = 'default',
  tint = 'primary',
  mono = false,
  name,
  id,
  disabled,
  required,
  readOnly,
  className,
}: AutocompleteProps) {
  return (
    <ABase.Root
      items={items}
      value={value}
      defaultValue={defaultValue}
      onValueChange={next => onValueChange?.(next)}
      name={name}
      disabled={disabled}
      required={required}
      readOnly={readOnly}
    >
      <ABase.Input
        id={id}
        placeholder={placeholder}
        aria-invalid={state === 'error' || undefined}
        data-mono={mono}
        data-size={size}
        data-tint={tintFor(state, tint)}
        className={cx('dz-text-field', className)}
      />
      <ABase.Portal>
        <ABase.Positioner data-tint={tint} className="dz-popup-positioner">
          <ABase.Popup className="dz-popup dz-combobox__popup">
            <ABase.Empty className="dz-popup__empty">{empty}</ABase.Empty>
            <ABase.List className="dz-popup__list">
              {(item: string) => (
                <ABase.Item key={item} value={item} className="dz-popup__item">
                  <span className="dz-popup__label">{item}</span>
                </ABase.Item>
              )}
            </ABase.List>
          </ABase.Popup>
        </ABase.Positioner>
      </ABase.Portal>
    </ABase.Root>
  )
}
