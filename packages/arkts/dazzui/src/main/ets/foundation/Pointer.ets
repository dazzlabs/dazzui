/** Scoped pointer observation without consuming a child's native event. */
export class DazzPointerContext {
  private listeners: ((x: number, y: number) => void)[] = []
  private moves: ((x: number, y: number) => void)[] = []
  private releases: (() => void)[] = []
  private touchHoverResets: (() => void)[] = []
  private lastX: number = 0
  private lastY: number = 0
  private lastTimestamp: number | undefined = undefined
  private moveX: number = 0
  private moveY: number = 0
  private moveTimestamp: number | undefined = undefined

  onRelease(listener: () => void): void {
    if (!this.releases.includes(listener)) this.releases.push(listener)
  }
  offRelease(listener: () => void): void {
    this.releases = this.releases.filter(item => item !== listener)
  }
  release(): void { this.releases.slice().forEach(listener => listener()) }
  onTouchHoverReset(listener: () => void): void {
    if (!this.touchHoverResets.includes(listener)) this.touchHoverResets.push(listener)
  }
  offTouchHoverReset(listener: () => void): void {
    this.touchHoverResets = this.touchHoverResets.filter(item => item !== listener)
  }
  /** A detached menu receives a new touch without a main-window coordinate. */
  resetTouchHover(): void { this.touchHoverResets.slice().forEach(listener => listener()) }

  onMove(listener: (x: number, y: number) => void): void {
    if (!this.moves.includes(listener)) this.moves.push(listener)
  }
  offMove(listener: (x: number, y: number) => void): void {
    this.moves = this.moves.filter(item => item !== listener)
  }
  move(x: number, y: number, timestamp?: number): void {
    // A popup and its theme scope may observe the same bubbling mouse event.
    // Feeding that zero-distance sample twice would misread cursor velocity.
    if (timestamp !== undefined && timestamp === this.moveTimestamp && x === this.moveX && y === this.moveY) return
    this.moveTimestamp = timestamp
    this.moveX = x
    this.moveY = y
    this.moves.slice().forEach(listener => listener(x, y))
  }

  on(listener: (x: number, y: number) => void): void {
    if (!this.listeners.includes(listener)) this.listeners.push(listener)
  }

  off(listener: (x: number, y: number) => void): void {
    this.listeners = this.listeners.filter(item => item !== listener)
  }

  press(x: number, y: number, timestamp?: number, inside?: (x: number, y: number) => void): void {
    // A detached surface publishes to the other window observers explicitly;
    // its own outside listener must not mistake that press for an outside one.
    // Mouse input can also synthesize a touch sample for the same press.
    if (timestamp !== undefined && this.lastTimestamp === timestamp && this.lastX === x && this.lastY === y) return
    this.lastTimestamp = timestamp
    this.lastX = x
    this.lastY = y
    // A listener may unsubscribe itself while handling this press.
    this.listeners.slice().forEach(listener => { if (listener !== inside) listener(x, y) })
  }
}
