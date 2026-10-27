import { UIContext, FrameCallback, window } from '@kit.ArkUI'

class AfterResizeFrame extends FrameCallback {
  private action: () => void
  constructor(action: () => void) { super(); this.action = action }
  onIdle(_timeLeftInNano: number): void { this.action() }
}

/** ArkUI dismisses menus on resize; retain the reference's logical open state. */
export class DazzMenuWindow {
  private openingSize: string = ''
  private pending: boolean = false
  private generation: number = 0
  private size(ui: UIContext): string {
    const name = ui.getWindowName()
    if (!name) return ''
    try {
      const rect = window.findWindow(name).getWindowProperties().windowRect
      return String(rect.width) + ':' + String(rect.height)
    } catch (_) { return '' }
  }
  opened(ui: UIContext): void {
    this.cancel()
    this.openingSize = this.size(ui)
  }
  closing(ui: UIContext, wanted: boolean): boolean {
    const size = this.size(ui)
    this.pending = wanted && !!size && !!this.openingSize && size !== this.openingSize
    return this.pending
  }
  afterClosed(ui: UIContext, reopen: () => void): void {
    if (!this.pending) return
    this.pending = false
    const generation = ++this.generation
    ui.postFrameCallback(new AfterResizeFrame(() => {
      if (generation === this.generation) reopen()
    }))
  }
  cancel(): void { this.pending = false; this.generation++ }
}
