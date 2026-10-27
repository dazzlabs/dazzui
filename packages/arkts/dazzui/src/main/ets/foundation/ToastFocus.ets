import { DazzToastItem } from './ToastData'
import { focusTarget } from './FocusData'

interface ToastShortcut { window: string; viewport: string; provider: string; enter: () => void }
let shortcuts: ToastShortcut[] = []
let observers: (() => void)[] = []
export function observeToastShortcuts(observer: () => void): void { if (!observers.includes(observer)) observers.push(observer) }
export function unobserveToastShortcuts(observer: () => void): void { observers = observers.filter(item => item !== observer) }
export function setToastShortcut(window: string, viewport: string, provider: string, available: boolean, enter: () => void): void {
  const existing = shortcuts.some(item => item.viewport === viewport)
  if (existing === available) return
  shortcuts = shortcuts.filter(item => item.viewport !== viewport)
  if (available) shortcuts.push({ window, viewport, provider, enter })
  observers.slice().forEach(observer => observer())
}
export function toastShortcutProvider(window: string): string {
  return shortcuts.filter(item => item.window === window).slice(-1)[0]?.provider ?? ''
}
/** Base UI attaches a window listener for each nonempty viewport, in this order. */
export function enterWindowToasts(window: string): void {
  const original = focusTarget(window)
  shortcuts.filter(item => item.window === window && item.viewport + '-focus' !== original).slice().forEach(item => item.enter())
}

export function toastRootFocus(viewport: string, id: string): string { return 'dazzui-toast-root-focus/' + viewport + '/' + id }
export function toastActionFocus(viewport: string, id: string): string { return 'dazzui-toast-action/' + viewport + '/' + id }
export function toastDismissFocus(viewport: string, id: string): string { return 'dazzui-toast-dismiss/' + viewport + '/' + id }
/** Keyboard order follows source DOM order, including when top placement reverses the drawing. */
export function toastFocusStops(viewport: string, items: DazzToastItem[]): string[] {
  const result: string[] = []
  items.filter(item => !item.ending && !item.limited).forEach(item => {
    result.push(toastRootFocus(viewport, item.id))
    if (item.action) result.push(toastActionFocus(viewport, item.id))
    if (item.dismissable) result.push(toastDismissFocus(viewport, item.id))
  })
  return result
}
