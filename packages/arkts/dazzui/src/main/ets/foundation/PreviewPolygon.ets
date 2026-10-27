/*
 * Adapted from @base-ui/react 1.7.0 floating-ui-react/safePolygon.js.
 * MIT License
 * Copyright (c) 2019 Material-UI SAS
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
import { DazzPreviewClock, DazzPreviewPoint, DazzPreviewRect } from './PreviewData'

function inRect(point: DazzPreviewPoint, left: number, top: number, right: number, bottom: number): boolean {
  return point.x >= Math.min(left, right) && point.x <= Math.max(left, right) &&
    point.y >= Math.min(top, bottom) && point.y <= Math.max(top, bottom)
}

function inPolygon(point: DazzPreviewPoint, vertices: DazzPreviewPoint[]): boolean {
  let inside = false
  for (let i = 0; i < vertices.length; i++) {
    const a = vertices[i]
    const b = vertices[(i + 1) % vertices.length]
    if ((a.y >= point.y) !== (b.y >= point.y) &&
      point.x <= (b.x - a.x) * (point.y - a.y) / (b.y - a.y) + a.x) inside = !inside
  }
  return inside
}

/** One cursor-origin polygon, from a native hover exit until re-entry/close. */
export class DazzPreviewPolygon {
  private origin: DazzPreviewPoint = { x: 0, y: 0 }
  private side: string = 'bottom'
  private landed: boolean = false
  private last: DazzPreviewPoint | undefined = undefined
  private lastTime: number = 0
  private timeout: number = -1
  private clock: DazzPreviewClock
  private now: () => number
  active: boolean = false
  onClose: () => void = () => {}
  hasOpenChild: () => boolean = () => false

  constructor(clock: DazzPreviewClock = {
    schedule: (action: () => void, delay: number): number => setTimeout(action, delay),
    cancel: (id: number): void => clearTimeout(id),
  }, now: () => number = () => Date.now()) { this.clock = clock; this.now = now }

  begin(origin: DazzPreviewPoint, side: string): void {
    this.dispose()
    this.origin = origin
    this.side = side
    this.landed = false
    this.last = undefined
    this.lastTime = this.now()
    this.active = true
  }
  dispose(): void {
    this.clock.cancel(this.timeout)
    this.timeout = -1
    this.active = false
  }
  private close(): void {
    if (this.hasOpenChild()) return
    this.dispose()
    this.onClose()
  }
  private slow(point: DazzPreviewPoint): boolean {
    const time = this.now()
    const elapsed = time - this.lastTime
    const previous = this.last
    this.last = point
    this.lastTime = time
    if (!previous || elapsed === 0) return false
    const dx = point.x - previous.x
    const dy = point.y - previous.y
    return dx * dx + dy * dy < elapsed * elapsed * 0.01
  }
  move(point: DazzPreviewPoint, reference: DazzPreviewRect, popup: DazzPreviewRect,
    target: string = 'outside', leave: boolean = false, relatedPopup: boolean = false): void {
    if (!this.active) return
    this.clock.cancel(this.timeout)
    this.timeout = -1
    if (!Number.isFinite(point.x) || !Number.isFinite(point.y)) { this.close(); return }
    if (target === 'popup') { this.landed = true; if (!leave) return }
    if (target === 'reference') {
      this.landed = false
      if (!leave) { this.landed = true; return }
    }
    if ((leave && relatedPopup) || this.hasOpenChild()) return
    const x = this.origin.x
    const y = this.origin.y
    const side = this.side
    const wider = popup.right - popup.left > reference.right - reference.left
    const taller = popup.bottom - popup.top > reference.bottom - reference.top
    const fromRight = x > (popup.left + popup.right) / 2
    const fromBottom = y > (popup.top + popup.bottom) / 2
    const left = (wider ? reference : popup).left
    const right = (wider ? reference : popup).right
    const top = (taller ? reference : popup).top
    const bottom = (taller ? reference : popup).bottom
    if ((side === 'top' && y >= reference.bottom - 1) ||
      (side === 'bottom' && y <= reference.top + 1) ||
      (side === 'left' && x >= reference.right - 1) ||
      (side === 'right' && x <= reference.left + 1)) { this.close(); return }
    const trough = side === 'top' ? inRect(point, left, reference.top + 1, right, popup.bottom - 1)
      : side === 'bottom' ? inRect(point, left, popup.top + 1, right, reference.bottom - 1)
      : side === 'left' ? inRect(point, popup.right - 1, bottom, reference.left + 1, top)
      : side === 'right' ? inRect(point, reference.right - 1, bottom, popup.left + 1, top) : false
    if (trough) return
    if ((this.landed && !inRect(point, reference.left, reference.top, reference.right, reference.bottom)) ||
      (!leave && this.slow(point))) { this.close(); return }
    const buffer = 0.5
    let vertices: DazzPreviewPoint[] = []
    if (side === 'top' || side === 'bottom') {
      const offset = wider ? buffer / 2 : buffer * 4
      const firstX = wider || fromRight ? x + offset : x - offset
      const secondX = wider ? x - offset : fromRight ? x + offset : x - offset
      const cursorY = side === 'top' ? y + buffer + 1 : y - buffer
      const near = side === 'top' ? popup.bottom - buffer : popup.top + buffer
      const far = side === 'top' ? popup.top : popup.bottom
      const leftY = fromRight || wider ? near : far
      const rightY = !fromRight || wider ? near : far
      vertices = [{ x: firstX, y: cursorY }, { x: secondX, y: cursorY },
        { x: popup.left, y: leftY }, { x: popup.right, y: rightY }]
    } else if (side === 'left' || side === 'right') {
      const offset = taller ? buffer / 2 : buffer * 4
      const firstY = taller || fromBottom ? y + offset : y - offset
      const secondY = taller ? y - offset : fromBottom ? y + offset : y - offset
      const cursorX = side === 'left' ? x + buffer + 1 : x - buffer
      const near = side === 'left' ? popup.right - buffer : popup.left + buffer
      const far = side === 'left' ? popup.left : popup.right
      const topX = fromBottom || taller ? near : far
      const bottomX = !fromBottom || taller ? near : far
      vertices = side === 'left' ? [{ x: topX, y: popup.top }, { x: bottomX, y: popup.bottom },
        { x: cursorX, y: firstY }, { x: cursorX, y: secondY }]
        : [{ x: cursorX, y: firstY }, { x: cursorX, y: secondY },
          { x: topX, y: popup.top }, { x: bottomX, y: popup.bottom }]
    }
    if (!inPolygon(point, vertices)) this.close()
    else if (!this.landed) this.timeout = this.clock.schedule(() => this.close(), 40)
  }
}
