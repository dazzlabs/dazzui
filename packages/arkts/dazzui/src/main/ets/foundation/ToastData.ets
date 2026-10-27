import { DazzTint } from './TokenTypes'

export interface DazzToastAction {
  label: string
  onAction: () => void
}

export interface DazzToastOptions {
  id?: string
  title: string
  tint?: DazzTint
  icon?: string
  action?: DazzToastAction
  dismissable?: boolean
  dismissLabel?: string
  timeout?: number
  priority?: 'low' | 'high'
  onClose?: () => void
}

export interface DazzToastItem {
  id: string
  title: string
  tint: DazzTint
  icon?: string
  action?: DazzToastAction
  dismissable: boolean
  dismissLabel: string
  timeout: number
  priority: 'low' | 'high'
  onClose?: () => void
  ending: boolean
  limited: boolean
  revision: number
}

export interface DazzToastTimings {
  notice: number
  action: number
  transition: number
}

export interface DazzToastClock {
  now: () => number
  schedule: (action: () => void, delay: number) => number
  cancel: (id: number) => void
}

interface DazzToastTimer {
  handle: number
  started: number
  remaining: number
  delay: number
}

let toastSequence: number = 0

/** One provider's queue; consuming a missing provider fails at the action. */
export class DazzToastContext {
  providerID: string = ''
  queue?: DazzToastQueue
  add(options: DazzToastOptions): string {
    if (!this.queue) throw new Error('A ToastProvider is required')
    return this.queue.add(options)
  }
  close(id?: string): void { this.queue?.close(id) }
}

/** Queue state only. Native view/focus ownership belongs to the viewport. */
export class DazzToastQueue {
  private entries: DazzToastItem[] = []
  private listeners: (() => void)[] = []
  private timers: Map<string, DazzToastTimer> = new Map()
  private removals: Map<string, number> = new Map()
  private reasons: Set<string> = new Set()
  private disposed: boolean = false
  private revision: number = 0
  private timings: DazzToastTimings
  private clock: DazzToastClock
  private limit: number

  constructor(timings: DazzToastTimings, limit: number = 3, clock?: DazzToastClock) {
    this.timings = { notice: timings.notice, action: timings.action, transition: timings.transition }
    this.limit = limit
    this.clock = clock ?? {
      now: (): number => Date.now(),
      schedule: (action: () => void, delay: number): number => setTimeout(action, delay),
      cancel: (id: number): void => clearTimeout(id),
    }
  }

  snapshot(): DazzToastItem[] {
    return this.entries.map((item: DazzToastItem): DazzToastItem => ({
      id: item.id, title: item.title, tint: item.tint, icon: item.icon,
      action: item.action ? { label: item.action.label, onAction: item.action.onAction } : undefined,
      dismissable: item.dismissable, dismissLabel: item.dismissLabel,
      timeout: item.timeout, priority: item.priority, onClose: item.onClose,
      ending: item.ending, limited: item.limited, revision: item.revision,
    }))
  }

  subscribe(listener: () => void): () => void {
    if (this.disposed) return () => {}
    this.listeners.push(listener)
    return () => { this.listeners = this.listeners.filter((entry: () => void): boolean => entry !== listener) }
  }

  private changed(): void {
    let index = 0
    for (const item of this.entries) {
      if (!item.ending) item.limited = index++ >= this.limit
    }
    for (const listener of this.listeners.slice()) listener()
  }

  setLimit(limit: number): void {
    if (this.disposed || this.limit === limit) return
    this.limit = limit
    this.changed()
  }

  add(options: DazzToastOptions): string {
    if (this.disposed) throw new Error('Toast queue is disposed')
    const id = options.id || 'dazzui-toast-' + String(++toastSequence)
    const previous = this.entries.find((entry: DazzToastItem): boolean => entry.id === id)
    this.clearTimer(id)
    const removal = this.removals.get(id)
    if (removal !== undefined) this.clock.cancel(removal)
    this.removals.delete(id)
    const tint = options.tint ?? 'neutral'
    const action = options.action
    const item: DazzToastItem = {
      id, title: options.title, tint, icon: options.icon,
      action: action ? { label: action.label, onAction: action.onAction } : undefined,
      dismissable: options.dismissable ?? true,
      dismissLabel: options.dismissLabel ?? 'Dismiss',
      timeout: options.timeout ?? (tint === 'danger' ? 0 : action ? this.timings.action : this.timings.notice),
      priority: options.priority ?? (tint === 'danger' ? 'high' : 'low'),
      onClose: options.onClose, ending: false, limited: false,
      revision: ++this.revision,
    }
    if (previous && !previous.ending) {
      this.entries = this.entries.map((entry: DazzToastItem): DazzToastItem => entry.id === id ? item : entry)
    } else {
      this.entries = [item, ...this.entries.filter((entry: DazzToastItem): boolean => entry.id !== id)]
    }
    if (item.timeout > 0) {
      const timer: DazzToastTimer = { handle: -1, started: this.clock.now(), remaining: item.timeout, delay: item.timeout }
      this.timers.set(id, timer)
      if (this.reasons.size === 0) this.startTimer(id, timer)
    }
    this.changed()
    return id
  }

  private clearTimer(id: string): void {
    const timer = this.timers.get(id)
    if (timer && timer.handle !== -1) this.clock.cancel(timer.handle)
    this.timers.delete(id)
  }

  private startTimer(id: string, timer: DazzToastTimer): void {
    timer.started = this.clock.now()
    timer.handle = this.clock.schedule(() => {
      // A replaced toast may have the same id after a stale native callback.
      if (this.disposed || this.timers.get(id) !== timer) return
      this.timers.delete(id)
      this.close(id)
    }, timer.remaining)
  }

  /** Reasons compose: leaving hover must not resume a blurred window. */
  pause(reason: string, paused: boolean): void {
    if (this.disposed) return
    const wasPaused = this.reasons.size > 0
    if (paused) this.reasons.add(reason)
    else this.reasons.delete(reason)
    const isPaused = this.reasons.size > 0
    if (wasPaused === isPaused) return
    this.timers.forEach((timer: DazzToastTimer, id: string) => {
      if (isPaused) {
        if (timer.handle !== -1) {
          this.clock.cancel(timer.handle)
          timer.handle = -1
          timer.remaining = Math.max(0, timer.remaining - (this.clock.now() - timer.started))
        }
      } else {
        if (timer.remaining <= 0) timer.remaining = timer.delay
        this.startTimer(id, timer)
      }
    })
  }

  close(id?: string): void {
    if (this.disposed) return
    const closing = this.entries.filter((item: DazzToastItem): boolean => !item.ending && (id === undefined || item.id === id))
    for (const item of closing) {
      this.clearTimer(item.id)
      item.ending = true
      const revision = item.revision
      this.removals.set(item.id, this.clock.schedule(() => {
        const current = this.entries.find((entry: DazzToastItem): boolean => entry.id === item.id)
        if (this.disposed || !current?.ending || current.revision !== revision) return
        this.removals.delete(item.id)
        this.entries = this.entries.filter((entry: DazzToastItem): boolean => entry !== current)
        this.changed()
      }, this.timings.transition))
    }
    if (!closing.length) return
    if (this.entries.every((item: DazzToastItem): boolean => item.ending)) {
      this.reasons.delete('hover')
      this.reasons.delete('focus')
    }
    this.changed()
    for (const item of closing) item.onClose?.()
  }

  activate(id: string): void {
    const item = this.entries.find((entry: DazzToastItem): boolean => entry.id === id)
    if (this.disposed || !item || item.ending || item.limited || !item.action) return
    item.action.onAction()
    this.close(id)
  }

  dispose(): void {
    if (this.disposed) return
    this.disposed = true
    for (const id of this.timers.keys()) this.clearTimer(id)
    for (const handle of this.removals.values()) this.clock.cancel(handle)
    this.removals.clear()
    this.reasons.clear()
    this.entries = []
    this.listeners = []
  }
}
