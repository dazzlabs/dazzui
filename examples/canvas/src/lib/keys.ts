/**
 * Whether the keystroke belongs to something other than the canvas.
 *
 * The canvas binds bare letters — `R` draws a rectangle — so every keyboard
 * handler in the app has to step aside for a field, a shape being edited in
 * place, or an open menu. One predicate, so the three of them agree.
 *
 * The second half of that used to come free: a Select was a native
 * `<select>`, and the platform ran its own type-ahead inside it. Now the
 * menus are drawn, so their type-ahead is an ordinary key event on an
 * ordinary element, and `H` would reach for the Hand tool while the reader
 * was only looking for "Hairline". The roles below are what a popup
 * announces itself as, so this stays a statement about the page rather than
 * about whichever library drew it.
 */
const TYPING_TAGS = ['INPUT', 'TEXTAREA', 'SELECT']

const FLOATING_ROLES =
  '[role="combobox"],[role="listbox"],[role="menu"],[role="dialog"],[role="alertdialog"]'

export function isTyping(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) return false
  if (target.isContentEditable) return true
  if (TYPING_TAGS.includes(target.tagName)) return true
  return target.closest(FLOATING_ROLES) !== null
}
