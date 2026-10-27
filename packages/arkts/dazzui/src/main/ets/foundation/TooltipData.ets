export class DazzTooltipDefaults {
  delay: number | undefined = undefined
  closeDelay: number | undefined = undefined
  constructor(delay?: number) { this.delay = delay }
}

/** The innermost tooltip owns a control, including IconButton's built-in hint. */
export class DazzTooltipScope {
  register: (id: number) => void = () => {}
  unregister: (id: number) => void = () => {}
}

export interface DazzTooltipClock {
  schedule: (action: () => void, delay: number) => number
  cancel: (id: number) => void
}

/** Hover waits for rest; keyboard focus is immediate and activation cancels it. */
export class DazzTooltipInteraction {
  private hovered: boolean = false
  private focused: boolean = false
  private blocked: boolean = false
  private dead: boolean = false
  private opening: number = -1
  private closing: number = -1
  private x?: number
  private y?: number
  private clock: DazzTooltipClock
  current: () => boolean = () => false
  change: (visible: boolean) => void = () => {}
  disabled: () => boolean = () => false
  openDelay: () => number = () => 0
  closeDelay: () => number = () => 0
  constructor(clock: DazzTooltipClock = {
    schedule: (action: () => void, delay: number): number => setTimeout(action, delay),
    cancel: (id: number): void => clearTimeout(id),
  }) { this.clock = clock }
  private cancelOpen(): void { this.clock.cancel(this.opening); this.opening = -1 }
  private cancelClose(): void { this.clock.cancel(this.closing); this.closing = -1 }
  private open(): void {
    this.cancelOpen()
    if (this.dead || this.disabled() || this.blocked || this.current()) return
    const delay = Math.max(0, this.openDelay())
    if (delay === 0) { this.change(true); return }
    this.opening = this.clock.schedule(() => {
      this.opening = -1
      if (!this.dead && !this.disabled() && !this.blocked && this.hovered) this.change(true)
    }, delay)
  }
  hover(entered: boolean): void {
    if (this.dead) return
    if (entered) {
      if (!this.hovered) this.blocked = false
      this.hovered = true
      this.cancelClose()
      if (this.opening === -1) this.open()
    } else {
      this.hovered = false; this.blocked = false; this.x = undefined; this.y = undefined
      this.cancelOpen(); this.cancelClose()
      if (!this.current()) return
      const delay = Math.max(0, this.closeDelay())
      if (delay === 0) { this.change(false); return }
      this.closing = this.clock.schedule(() => { this.closing = -1; if (!this.dead && !this.hovered) this.change(false) }, delay)
    }
  }
  move(x: number, y: number): void {
    const moved = this.x === undefined || this.y === undefined || (x - this.x) ** 2 + (y - this.y) ** 2 >= 2
    this.x = x; this.y = y
    if (moved && this.hovered && !this.current() && !this.focused) this.open()
  }
  focus(focused: boolean): void {
    if (this.dead || (focused && this.focused)) return
    this.focused = focused
    this.cancelOpen(); this.cancelClose()
    if (focused) {
      if (!this.disabled()) this.change(true)
    } else { this.blocked = this.hovered; this.change(false) }
  }
  dismiss(): void { this.cancelOpen(); this.cancelClose(); this.blocked = true; this.change(false) }
  configure(): void { if (this.disabled()) this.dismiss(); else if (this.hovered && !this.blocked && !this.current()) this.open() }
  dispose(): void { this.dead = true; this.cancelOpen(); this.cancelClose(); this.change = () => {} }
}
