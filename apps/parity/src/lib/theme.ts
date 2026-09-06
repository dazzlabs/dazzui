import type { ThemeId } from './model'

/**
 * Point the document at a theme the way the token stylesheet expects:
 * Studio Light is `:root` and sets no attribute, so selecting it removes
 * `data-theme` rather than writing `studio-light`.
 */
export function applyTheme(theme: ThemeId) {
  const root = document.documentElement
  if (theme === 'studio-light') root.removeAttribute('data-theme')
  else root.setAttribute('data-theme', theme)
}
