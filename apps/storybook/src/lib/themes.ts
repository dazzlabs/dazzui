/**
 * The four themes the token graph carries, in the order the toolbar lists
 * them. Studio Light is the default and is what `:root` holds, so it sets no
 * attribute at all — selecting it has to *remove* `data-theme` rather than
 * write `studio-light`, or the Bright and dark blocks would simply not match
 * and the page would look right for the wrong reason.
 *
 * Both the preview and the parity extractor read this list, which is what
 * keeps the toolbar and the reference data on the same four themes.
 */
export const THEMES = [
  { value: 'studio-light', title: 'Studio Light' },
  { value: 'studio-dark', title: 'Studio Dark' },
  { value: 'bright-light', title: 'Bright Light' },
  { value: 'bright-dark', title: 'Bright Dark' },
] as const

export type ThemeValue = (typeof THEMES)[number]['value']

/** The default theme: the one that clears `data-theme` instead of setting it. */
export const DEFAULT_THEME: ThemeValue = 'studio-light'

/** Point the document at a theme the way the token stylesheet expects. */
export function applyTheme(root: HTMLElement, theme: string) {
  if (theme === DEFAULT_THEME) root.removeAttribute('data-theme')
  else root.setAttribute('data-theme', theme)
}
