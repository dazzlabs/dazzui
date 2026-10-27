export interface DazzMenuChoice { id: string; label: string; disabled?: boolean }

export interface DazzMenubarChoice { label: string; disabled?: boolean }
export interface DazzMenubarRect { x: number; y: number; width: number; height: number }
export function insideMenubar(rect: DazzMenubarRect | undefined, x: number, y: number): boolean {
  return rect !== undefined && x >= rect.x && y >= rect.y && x < rect.x + rect.width && y < rect.y + rect.height
}

/** The bar is one composite; disabled labels never become arrow destinations. */
export function menubarDestination(items: DazzMenubarChoice[], current: number,
  key: number, vertical: boolean = false): number {
  if (key === 2081) return items.findIndex((item: DazzMenubarChoice): boolean => !item.disabled)
  if (key === 2082) {
    for (let index = items.length - 1; index >= 0; index--) if (!items[index].disabled) return index
    return -1
  }
  const step = key === (vertical ? 2013 : 2015) ? 1 : key === (vertical ? 2012 : 2014) ? -1 : 0
  if (!step) return -1
  const start = current < 0 && step < 0 ? 0 : current
  for (let count = 1; count <= items.length; count++) {
    const index = (start + count * step + items.length) % items.length
    if (!items[index].disabled) return index
  }
  return -1
}

/** Replaced as a scoped snapshot, so forwarded triggers observe every change. */
export class DazzMenubarContext {
  activeIndex: number = -1
  focusedIndex: number = 0
  focusEdge: number = 0
  scope: string = ''
  modal: boolean = true
  enabled: boolean[] = []
  onOpen: (index: number, open: boolean) => void = () => {}
  onHover: (index: number) => void = () => {}
  onFocus: (index: number) => void = () => {}
  onClick: (index: number) => void = () => {}
  onKey: (index: number, event: KeyEvent, popup: boolean) => boolean = () => false
  returnFocus: (index: number) => void = () => {}
  onClosed: (index: number) => void = () => {}
  bounds: () => DazzMenubarRect | undefined = () => undefined
  entryAt: (x: number, y: number) => number = () => -1
}

/** Match the reference toolkit's prefix search without changing selection. */
export class DazzMenuTypeahead {
  private prefix: string = ''
  private typedAt: number = -Infinity
  private previous: number = -1
  private matched: number = -1
  private resetAfter: number
  // Base UI's Select uses useTypeahead's 750ms default; Menu supplies 500ms.
  constructor(resetAfter: number = 750) { this.resetAfter = resetAfter }
  reset(): void { this.prefix = ''; this.typedAt = -Infinity; this.previous = -1; this.matched = -1 }
  typing(now: number): boolean { return this.prefix.length > 0 && now - this.typedAt < this.resetAfter }
  match(items: DazzMenuChoice[], character: string, active: string, now: number): string | undefined {
    if (!this.typing(now)) this.prefix = ''
    const key = character.toLowerCase()
    if (!key || key === ' ' && !this.prefix) return undefined
    const startSession = !this.prefix
    const activeIndex = items.findIndex((item: DazzMenuChoice): boolean => item.id === active)
    if (startSession) this.previous = activeIndex
    const cycle = items.every((item: DazzMenuChoice): boolean => {
      const label = item.label.toLowerCase()
      return item.disabled || !label || label[0] !== label[1]
    })
    if (cycle && this.prefix === key) { this.prefix = ''; this.previous = this.matched }
    this.prefix += key
    this.typedAt = now
    const start = startSession ? activeIndex : this.previous
    for (let offset = 1; offset <= items.length; offset++) {
      const index = (start + offset + items.length) % items.length
      const item = items[index]
      if (!item.disabled && item.label.toLowerCase().startsWith(this.prefix)) {
        this.matched = index
        return item.id
      }
    }
    if (key !== ' ') this.prefix = ''
    return undefined
  }
}

export function menuCharacter(event: KeyEvent): string {
  if (event.getModifierKeyState?.(['Ctrl']) || event.getModifierKeyState?.(['Alt']) || (event.metaKey ?? 0) !== 0) return ''
  const code = event.unicode ?? 0
  if (code >= 0x20 && code <= 0x10ffff) return String.fromCodePoint(code)
  if (event.keyText?.length === 1 && event.keyText.charCodeAt(0) >= 0x20) return event.keyText
  // Native/synthetic key events can omit Unicode. These are ArkUI's stable
  // letter/digit/space key codes; prefer the layout-resolved character above.
  if (event.keyCode >= 2017 && event.keyCode <= 2042) return String.fromCharCode(97 + event.keyCode - 2017)
  if (event.keyCode >= 2000 && event.keyCode <= 2009) return String.fromCharCode(48 + event.keyCode - 2000)
  return event.keyCode === 2050 ? ' ' : ''
}
