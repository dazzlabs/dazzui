import { DazzItem } from './TokenTypes'

/** Shared selection rules; callers own focus and controlled state. */
export function toggleSelection(current: string[], value: string, multiple: boolean): string[] {
  if (current.includes(value)) return current.filter((item: string) => item !== value)
  return multiple ? current.concat([value]) : [value]
}

export class DazzAccordionContext {
  values: string[] = []
  headers: string[] = []
  focusPrefix: string = ''
  divided: boolean = false
  toggle: (value: string) => void = () => {}
  register(value: string): void {
    if (!this.headers.includes(value)) this.headers = this.headers.concat([value])
  }
  unregister(value: string): void { this.headers = this.headers.filter((item: string) => item !== value) }
  next(value: string, offset: number): string {
    const count = this.headers.length
    return count ? this.headers[(this.headers.indexOf(value) + offset + count) % count] : value
  }
}

/** Roving focus is separate from selection; disabled choices never receive it. */
export function nextTabFocus(items: DazzItem[], current: number, key: number): number {
  const enabled = items.map((_item: DazzItem, index: number) => index)
    .filter((index: number) => !items[index].disabled)
  if (!enabled.length) return -1
  if (key === 2081) return enabled[0]
  if (key === 2082) return enabled[enabled.length - 1]
  if (key !== 2014 && key !== 2015) return -1
  const offset = key === 2014 ? -1 : 1
  const position = enabled.indexOf(current)
  if (position < 0) return offset < 0 ? enabled[enabled.length - 1] : enabled[0]
  return enabled[(position + offset + enabled.length) % enabled.length]
}
