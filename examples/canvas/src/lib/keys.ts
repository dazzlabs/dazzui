/**
 * Whether the keystroke belongs to something being typed into.
 *
 * The canvas binds bare letters — `R` draws a rectangle — so every keyboard
 * handler in the app has to step aside for a field, a text area, or a shape
 * being edited in place. One predicate, so the three of them agree.
 */
export function isTyping(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) return false
  if (target.isContentEditable) return true
  return ['INPUT', 'TEXTAREA', 'SELECT'].includes(target.tagName)
}
