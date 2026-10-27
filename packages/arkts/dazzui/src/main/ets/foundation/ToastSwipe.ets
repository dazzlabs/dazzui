/*
 * Adapted from Base UI ToastRoot's pointer handlers.
 * MIT License
 *
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

export interface DazzToastTransform { x: number; y: number; scale: number }

// These are Base UI's interaction thresholds, rather than visual token values.
const SWIPE_THRESHOLD: number = 40
const REVERSE_CANCEL_THRESHOLD: number = 10
const MIN_DRAG_THRESHOLD: number = 1

/** The DazzUI Toast wrapper uses Base UI's default down/right directions. */
export class DazzToastSwipe {
  active: boolean = false
  real: boolean = false
  locked: string = ''
  direction: string = ''
  cancelled: boolean = false
  transform: DazzToastTransform = { x: 0, y: 0, scale: 1 }
  private pointer: number = -1
  private startX: number = 0
  private startY: number = 0
  private baselineX: number = 0
  private baselineY: number = 0
  private maximum: number = 0
  private first: boolean = false
  private initial: DazzToastTransform = { x: 0, y: 0, scale: 1 }

  begin(pointer: number, x: number, y: number, initial: DazzToastTransform,
    button: number = 0, interactive: boolean = false): boolean {
    if (button !== 0 || interactive) return false
    this.pointer = pointer
    this.startX = x; this.startY = y
    this.baselineX = x; this.baselineY = y
    this.initial = { x: initial.x, y: initial.y, scale: initial.scale }
    this.transform = { x: initial.x, y: initial.y, scale: initial.scale }
    this.maximum = 0
    this.cancelled = false; this.direction = ''; this.locked = ''
    this.active = true; this.real = false; this.first = true
    return true
  }

  move(pointer: number, x: number, y: number, movementX: number, movementY: number): boolean {
    if (!this.active || pointer !== this.pointer) return false
    if (this.first) { this.startX = x; this.startY = y; this.first = false }
    if ((movementY < 0 && y > this.baselineY) || (movementY > 0 && y < this.baselineY)) this.baselineY = y
    if ((movementX < 0 && x > this.baselineX) || (movementX > 0 && x < this.baselineX)) this.baselineX = x
    const dx = x - this.startX
    const dy = y - this.startY
    if (!this.real && Math.sqrt(dx * dx + dy * dy) >= MIN_DRAG_THRESHOLD) {
      this.real = true
      this.locked = Math.abs(dx) > Math.abs(dy) ? 'horizontal' : 'vertical'
    }
    if (!this.direction) {
      const candidate = this.locked === 'vertical' ? (dy > 0 ? 'down' : dy < 0 ? 'up' : '') :
        this.locked === 'horizontal' ? (dx > 0 ? 'right' : dx < 0 ? 'left' : '') :
        Math.abs(dx) >= Math.abs(dy) ? (dx > 0 ? 'right' : 'left') : (dy > 0 ? 'down' : 'up')
      if (candidate === 'down' || candidate === 'right') {
        this.direction = candidate
        this.maximum = candidate === 'down' ? dy : dx
      }
    } else {
      const displacement = this.direction === 'down' ? y - this.baselineY : x - this.baselineX
      if (displacement > SWIPE_THRESHOLD) this.cancelled = false
      else if (this.maximum - displacement >= REVERSE_CANCEL_THRESHOLD) this.cancelled = true
    }
    this.transform = {
      x: this.initial.x + (this.locked === 'vertical' ? 0 : dx < 0 ? -Math.sqrt(-dx) : dx),
      y: this.initial.y + (this.locked === 'horizontal' ? 0 : dy < 0 ? -Math.sqrt(-dy) : dy),
      scale: this.initial.scale,
    }
    return true
  }

  end(pointer: number, cancel: boolean = false): boolean {
    if (!this.active || pointer !== this.pointer) return false
    this.pointer = -1; this.active = false; this.real = false; this.locked = ''
    if (!cancel && !this.cancelled) {
      const dx = this.transform.x - this.initial.x
      const dy = this.transform.y - this.initial.y
      if (dy > SWIPE_THRESHOLD || dx > SWIPE_THRESHOLD) {
        this.direction = dy > SWIPE_THRESHOLD ? 'down' : 'right'
        return true
      }
    }
    this.transform = { x: this.initial.x, y: this.initial.y, scale: this.initial.scale }
    this.direction = ''
    return false
  }
}
