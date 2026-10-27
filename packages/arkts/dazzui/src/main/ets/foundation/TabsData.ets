export interface DazzTabItem {
  value: string
  label: string
  icon?: string
  count?: number
  status?: 'neutral' | 'primary' | 'success' | 'warning' | 'danger'
  busy?: boolean
  detail?: string
  closable?: boolean
  title?: string
  disabled?: boolean
}

export interface DazzTabsFit {
  compact: boolean
  shown: number
  basis: number
  floor: number
}

export interface DazzTabsPulseAnimator {
  reset: () => void
  half: (dimmed: boolean, finished: () => void) => void
}

/** Each CSS keyframe interval runs the easing forward, including the return. */
export class DazzTabsPulse {
  animator: DazzTabsPulseAnimator = { reset: () => {}, half: () => {} }
  private running: boolean = false
  private started: boolean = false
  private dead: boolean = false
  private generation: number = 0
  start(busy: boolean): void {
    if (this.dead || this.started && this.running === busy) return
    this.started = true; this.running = busy
    const generation = ++this.generation
    this.animator.reset()
    if (busy) this.half(generation, true)
  }
  private half(generation: number, dimmed: boolean): void {
    if (this.dead || !this.running || generation !== this.generation) return
    this.animator.half(dimmed, () => this.half(generation, !dimmed))
  }
  dispose(): void { this.dead = true; this.running = false; this.generation++ }
}

/** Widths are measured independently of the visible row to avoid feedback. */
export function fitTabs(whole: number[], small: number[], active: number, width: number, trailing: number, more: number): DazzTabsFit {
  const sum = (values: number[], count: number): number => values.slice(0, count).reduce((total: number, value: number): number => total + value, 0)
  const basis = sum(whole, whole.length) + trailing
  const room = width - trailing
  const floor = Math.min(basis, (whole[Math.max(active, 0)] ?? 0) + (whole.length > 1 ? more : 0) + trailing)
  if (sum(whole, whole.length) <= room) return { compact: false, shown: whole.length, basis, floor }
  const mixed = small.map((value: number, index: number): number => index === active ? (whole[index] ?? value) : value)
  const visibleWidth = (count: number): number => {
    const widths = mixed.slice(0, count)
    if (active >= count && count > 0) widths[count - 1] = whole[active] ?? small[active] ?? 0
    return sum(widths, widths.length)
  }
  let shown = whole.length
  while (shown > 1 && visibleWidth(shown) + (shown < whole.length ? more : 0) > room) shown--
  return { compact: true, shown, basis, floor }
}

/** A hidden active tab takes the last visible place without reordering peers. */
export function tabsOrder(length: number, shown: number, active: number): number[] {
  const order: number[] = []
  for (let index = 0; index < length; index++) order.push(index)
  if (active >= shown && shown > 0) {
    order.splice(active, 1)
    order.splice(shown - 1, 0, active)
  }
  return order
}

export function tabsNavigate(shown: DazzTabItem[], value: string | null, key: string): string | undefined {
  if (!['ArrowLeft', 'ArrowRight', 'Home', 'End'].includes(key)) return undefined
  const enabled = shown.filter((item: DazzTabItem): boolean => !item.disabled)
  const at = enabled.findIndex((item: DazzTabItem): boolean => item.value === value)
  const to = key === 'Home' ? 0 : key === 'End' ? enabled.length - 1
    : (at + (key === 'ArrowRight' ? 1 : -1) + enabled.length) % enabled.length
  return enabled[to]?.value
}
