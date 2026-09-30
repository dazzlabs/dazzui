import { Select as Base } from '@base-ui/react/select'
import type { ReactNode } from 'react'

import { cx } from '../lib/cx'
import { Check, ChevronUpDown, DropDown } from '../lib/glyphs'
import { usePlatform, useThemeContainer } from '../lib/theme'

export interface SelectOption {
  value: string
  label: ReactNode
  disabled?: boolean
}

export interface SelectProps {
  options: readonly SelectOption[]
  value?: string | null
  defaultValue?: string | null
  onValueChange?: (value: string) => void
  /** Material's floating label; iOS shows it in the row beside the value. */
  label?: ReactNode
  placeholder?: ReactNode
  disabled?: boolean
  name?: string
  'aria-label'?: string
  className?: string
}

/**
 * SwiftUI's menu picker — the value in a tinted capsule with the
 * up-and-down chevron, opening the platform menu — and Material's
 * exposed dropdown, an outlined field that opens the platform menu. The
 * list is the menu's own drawing; Base UI's select keeps the value, the
 * type-ahead and the hidden input a form needs.
 */
export function Select({
  options,
  value,
  defaultValue,
  onValueChange,
  label,
  placeholder = 'Select',
  disabled,
  name,
  className,
  ...props
}: SelectProps) {
  const platform = usePlatform()
  const container = useThemeContainer()
  return (
    <Base.Root
      value={value}
      defaultValue={defaultValue}
      onValueChange={next => {
        if (next !== null) onValueChange?.(next)
      }}
      disabled={disabled}
      name={name}
      items={options.map(option => ({
        value: option.value,
        label: option.label,
      }))}
    >
      <Base.Trigger
        data-platform={platform}
        aria-label={props['aria-label']}
        className={cx('dzm-select', 'dzm-m3-layer', className)}
      >
        {label ? <span className="dzm-select__label">{label}</span> : null}
        <Base.Value className="dzm-select__value" placeholder={placeholder} />
        <Base.Icon className="dzm-select__chevron">
          {platform === 'ios' ? <ChevronUpDown /> : <DropDown />}
        </Base.Icon>
      </Base.Trigger>
      <Base.Portal container={container}>
        <Base.Positioner
          data-platform={platform}
          sideOffset={platform === 'ios' ? 8 : 4}
          alignItemWithTrigger={false}
          className="dzm-menu-positioner"
        >
          <Base.Popup
            data-platform={platform}
            data-checks=""
            className={cx(
              'dzm-menu',
              platform === 'ios' && 'dzm-glass dzm-glass--thick'
            )}
          >
            <Base.List>
              {options.map(option => (
                <Base.Item
                  key={option.value}
                  value={option.value}
                  disabled={option.disabled}
                  className="dzm-menu__item"
                >
                  <Base.ItemIndicator
                    className="dzm-menu__check"
                    data-checked=""
                    keepMounted
                  >
                    <Check />
                  </Base.ItemIndicator>
                  <Base.ItemText className="dzm-menu__label">
                    {option.label}
                  </Base.ItemText>
                </Base.Item>
              ))}
            </Base.List>
          </Base.Popup>
        </Base.Positioner>
      </Base.Portal>
    </Base.Root>
  )
}
