import type { CSSProperties } from 'react'

import { DEFAULT_THEME, type Theme, themeId } from '../lib/themes'

/** What a preview draws with: the papers, the rule, the ink and the accent. */
const DRAWN = [
  '--color-surface',
  '--color-surface-column',
  '--color-border-strong',
  '--color-content',
  '--color-primary-500',
] as const

/**
 * Studio Light's values, read off the document before any theme is applied.
 * Every other theme can be asked for by nesting its `data-theme`; this one
 * lives on `:root`, so inside another theme there is no selector to ask.
 */
const studioLight = (() => {
  const root = document.documentElement
  const applied = root.getAttribute('data-theme')
  root.removeAttribute('data-theme')
  const computed = getComputedStyle(root)
  const values = Object.fromEntries(
    DRAWN.map(name => [name, computed.getPropertyValue(name)])
  )
  if (applied) root.setAttribute('data-theme', applied)
  return values as CSSProperties
})()

/**
 * A theme in miniature: a window of its own — the column, the paper, two
 * lines of its ink and a button in its accent.
 */
export function ThemePreview({ theme }: { theme: Theme }) {
  const id = themeId(theme)
  const isDefault = id === themeId(DEFAULT_THEME)
  return (
    <span
      className="g-preview"
      data-theme={isDefault ? undefined : id}
      style={isDefault ? studioLight : undefined}
      aria-hidden
    >
      <span className="g-preview__column" />
      <span className="g-preview__paper">
        <span className="g-preview__ink" />
        <span className="g-preview__ink" />
        <span className="g-preview__accent" />
      </span>
    </span>
  )
}
