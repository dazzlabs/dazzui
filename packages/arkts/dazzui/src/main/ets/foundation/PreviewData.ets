export interface DazzPreviewRect {
  left: number
  top: number
  right: number
  bottom: number
}
export interface DazzPreviewPoint { x: number; y: number }

/** A virtual inline target can forward events without replacing native spans. */
export class DazzPreviewAnchor {
  hover: (entered: boolean) => void = () => {}
  focus: (focused: boolean) => void = () => {}
  dismiss: () => void = () => {}
}

export interface DazzPreviewClock {
  schedule: (action: () => void, delay: number) => number
  cancel: (id: number) => void
}

/** Controlled proposals, hover intent and keyboard focus share one clock. */
export class DazzPreviewInteraction {
  private triggerHovered: boolean = false
  private popupHovered: boolean = false
  private triggerFocused: boolean = false
  private popupFocused: boolean = false
  private touchedInside: boolean = false
  private blocked: boolean = false
  private travelling: boolean = false
  private opening: number = -1
  private closing: number = -1
  private clock: DazzPreviewClock
  current: () => boolean = () => false
  change: (open: boolean) => void = () => {}
  openDelay: () => number = () => 0
  closeDelay: () => number = () => 0

  constructor(clock: DazzPreviewClock = {
    schedule: (action: () => void, delay: number): number => setTimeout(action, delay),
    cancel: (id: number): void => clearTimeout(id),
  }) { this.clock = clock }

  private cancelOpen(): void { this.clock.cancel(this.opening); this.opening = -1 }
  private cancelClose(): void { this.clock.cancel(this.closing); this.closing = -1 }
  private active(): boolean {
    return this.triggerHovered || this.popupHovered || this.triggerFocused || this.popupFocused || this.touchedInside
  }
  private enter(): void {
    this.cancelClose()
    if (this.current() || this.opening !== -1 || this.blocked) return
    this.opening = this.clock.schedule(() => {
      this.opening = -1
      if (this.active() && !this.blocked && !this.current()) this.change(true)
    }, this.openDelay())
  }
  private leave(delay: number): void {
    if (this.active() || this.blocked) return
    this.cancelOpen()
    if (!this.current() || this.closing !== -1) return
    this.closing = this.clock.schedule(() => {
      this.closing = -1
      if (!this.active() && !this.blocked && this.current()) this.change(false)
    }, delay)
  }
  hover(popup: boolean, entered: boolean): void {
    if (popup) this.popupHovered = entered
    else this.triggerHovered = entered
    if (entered) { this.travelling = false; this.blocked = false; this.enter() }
    else {
      this.travelling = this.current()
      if (!this.current()) this.blocked = false
      this.leave(this.closeDelay())
    }
  }
  focus(popup: boolean, focused: boolean): void {
    this.travelling = false
    const wasPopupFocused = this.popupFocused
    if (popup) this.popupFocused = focused
    else this.triggerFocused = focused
    if (focused) this.enter()
    else {
      if (popup && wasPopupFocused) this.touchedInside = false
      if (!this.current()) this.blocked = false
      this.leave(0)
    }
  }
  touchInside(active: boolean): void {
    this.touchedInside = active
    if (active) this.cancelClose()
    else this.leave(this.closeDelay())
  }
  transit(inside: boolean): void {
    if (!this.travelling || !this.current() || this.active()) return
    if (inside) this.cancelClose()
    else this.leave(this.closeDelay())
  }
  finishTransit(): void {
    this.travelling = false
    this.leave(this.closeDelay())
  }
  dismiss(): void {
    this.touchedInside = false
    this.travelling = false
    this.cancelOpen()
    this.cancelClose()
    this.blocked = true
    if (this.current()) this.change(false)
  }
  dispose(): void {
    this.touchedInside = false
    this.travelling = false
    this.cancelOpen()
    this.cancelClose()
    this.popupHovered = false
    this.popupFocused = false
  }
}
