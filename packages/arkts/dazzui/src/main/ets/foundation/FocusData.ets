interface WindowFocus { target: string; scope: string }
const windows: Map<string, WindowFocus> = new Map()
const observers: Map<string, Set<(id: string) => void>> = new Map()
let focusSequence: number = 0

export function focusID(kind: string): string { return 'dazzui-focus-' + kind + '-' + String(++focusSequence) }
function state(window: string): WindowFocus {
  let current = windows.get(window)
  if (!current) { current = { target: '', scope: '' }; windows.set(window, current) }
  return current
}
/** IDs belong to native focus nodes, not custom-component layout wrappers. */
export function rememberFocus(window: string, id: string): void {
  const current = state(window)
  if (current.target === id) return
  current.target = id
  observers.get(window)?.forEach((notify: (id: string) => void) => notify(id))
}
export function forgetFocus(window: string, id: string): void {
  const current = windows.get(window)
  if (id && current?.target === id) {
    current.target = ''
    observers.get(window)?.forEach((notify: (id: string) => void) => notify(''))
  }
}
export function observeFocus(window: string, notify: (id: string) => void): () => void {
  let listeners = observers.get(window)
  if (!listeners) { listeners = new Set(); observers.set(window, listeners) }
  listeners.add(notify)
  const current = listeners
  return () => {
    current.delete(notify)
    if (!current.size && observers.get(window) === current) observers.delete(window)
  }
}
export function rememberFocusScope(window: string, id: string): void { state(window).scope = id }
export function forgetFocusScope(window: string, id: string): void {
  const current = windows.get(window)
  if (current?.scope === id) current.scope = ''
}
export function focusTarget(window: string): string { return windows.get(window)?.target ?? '' }
/** A native scope remembers its last child for host-authored controls too. */
export function captureFocus(window: string): string {
  const current = windows.get(window)
  return current?.target || current?.scope || ''
}
