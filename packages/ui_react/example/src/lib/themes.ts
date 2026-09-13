/**
 * The twelve themes the token graph carries, in the order the toolbar lists
 * them — the six families, light before dark. Studio Light is the default
 * and is what `:root` holds, so it sets no attribute at all — selecting it
 * has to *remove* `data-theme` rather than write `studio-light`, or none of
 * the theme blocks would match and the page would look right for the wrong
 * reason.
 *
 * Both the preview and the parity extractor read this list, which is what
 * keeps the toolbar and the reference data on the same themes.
 */
export const THEMES = [
  { value: 'studio-light', title: 'Studio Light' },
  { value: 'studio-dark', title: 'Studio Dark' },
  { value: 'bright-light', title: 'Bright Light' },
  { value: 'bright-dark', title: 'Bright Dark' },
  { value: 'frost-light', title: 'Frost Light' },
  { value: 'frost-dark', title: 'Frost Dark' },
  { value: 'graphite-light', title: 'Graphite Light' },
  { value: 'graphite-dark', title: 'Graphite Dark' },
  { value: 'ember-light', title: 'Ember Light' },
  { value: 'ember-dark', title: 'Ember Dark' },
  { value: 'nocturne-light', title: 'Nocturne Light' },
  { value: 'nocturne-dark', title: 'Nocturne Dark' },
] as const

export type ThemeValue = (typeof THEMES)[number]['value']

/** The default theme: the one that clears `data-theme` instead of setting it. */
export const DEFAULT_THEME: ThemeValue = 'studio-light'

/** Point the document at a theme the way the token stylesheet expects. */
export function applyTheme(root: HTMLElement, theme: string) {
  if (theme === DEFAULT_THEME) root.removeAttribute('data-theme')
  else root.setAttribute('data-theme', theme)
}
