/**
 * The four themes the token graph carries, as three modes over one default.
 *
 * Studio Light is what `:root` holds, so selecting it *removes* `data-theme`
 * rather than writing `studio-light` — writing it would leave the page looking
 * right for the wrong reason, since none of the three theme blocks would match.
 */
export const THEMES = [
  { value: 'studio-light', label: 'Studio Light' },
  { value: 'studio-dark', label: 'Studio Dark' },
  { value: 'bright-light', label: 'Bright Light' },
  { value: 'bright-dark', label: 'Bright Dark' },
] as const

export type Theme = (typeof THEMES)[number]['value']

export function applyTheme(theme: Theme) {
  const root = document.documentElement
  if (theme === 'studio-light') root.removeAttribute('data-theme')
  else root.setAttribute('data-theme', theme)
}
