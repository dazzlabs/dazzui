/** A flat choice or a named group; the source order is also keyboard order. */
export interface DazzSelectOption {
  value?: string
  label: string
  disabled?: boolean
  options?: DazzSelectOption[]
}

export function flattenOptions(options: DazzSelectOption[]): DazzSelectOption[] {
  return options.flatMap((option: DazzSelectOption): DazzSelectOption[] =>
    option.options ? flattenOptions(option.options) : [option])
}

export function filterOptions(options: DazzSelectOption[], query: string): DazzSelectOption[] {
  const needle = query.trim().toLocaleLowerCase()
  return options.flatMap((option: DazzSelectOption): DazzSelectOption[] => {
    if (!option.options) return option.label.toLocaleLowerCase().includes(needle) ? [option] : []
    const children = filterOptions(option.options, query)
    return children.length ? [{ label: option.label, options: children }] : []
  })
}

export function optionLabel(options: DazzSelectOption[], value: string): string {
  return flattenOptions(options).find((option: DazzSelectOption) => option.value === value)?.label ?? ''
}

export function nextOption(options: DazzSelectOption[], index: number, offset: number): number {
  const choices = flattenOptions(options)
  const start = index < 0 && offset < 0 ? 0 : index
  for (let step = 1; step <= choices.length; step++) {
    const next = (start + offset * step + choices.length) % choices.length
    if (!choices[next].disabled) return next
  }
  return -1
}
