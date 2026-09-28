/**
 * The twelve themes the token graph carries, as six families in two modes —
 * which is how they are chosen, too. A family is a look: what the accent is,
 * how the corners are cut, what the paper is made of. The mode is only
 * whether that look is drawn on paper or on a dark ground. One list of
 * twelve makes the reader pick both at once, and pick again when the light
 * changes; two controls make the second choice one click.
 *
 * Studio Light is what `:root` holds, so selecting it *removes* `data-theme`
 * rather than writing `studio-light` — writing it would leave the page looking
 * right for the wrong reason, since none of the theme blocks would match.
 */
export const FAMILIES = [
  { value: 'studio', label: 'Studio', hint: 'The system’s own default.' },
  {
    value: 'bright',
    label: 'Bright',
    hint: 'Ink navy and acid washes; the hover lifts rather than darkens.',
  },
  {
    value: 'frost',
    label: 'Frost',
    hint: 'A teal that reads the same on either ground.',
  },
  {
    value: 'graphite',
    label: 'Graphite',
    hint: 'Monochrome, with the corners cut one rung crisper.',
  },
  {
    value: 'ember',
    label: 'Ember',
    hint: 'A warm accent that inverts its ramp in the dark.',
  },
  {
    value: 'nocturne',
    label: 'Nocturne',
    hint: 'What this window is drawn in: a blurple on blue-grey, from the Tiller mockups.',
  },
] as const

export type Family = (typeof FAMILIES)[number]['value']

/** Light, dark, or whichever the system is set to. */
export const APPEARANCES = [
  { value: 'light', label: 'Light' },
  { value: 'dark', label: 'Dark' },
  { value: 'system', label: 'System' },
] as const

export type Appearance = (typeof APPEARANCES)[number]['value']

export type Theme = `${Family}-light` | `${Family}-dark`

const QUERY = '(prefers-color-scheme: dark)'

/** True where the system is currently set to dark. */
export function systemPrefersDark(): boolean {
  return typeof window !== 'undefined' && window.matchMedia(QUERY).matches
}

/** The one of the twelve a family and an appearance come to, right now. */
export function themeName(family: Family, appearance: Appearance): Theme {
  const dark =
    appearance === 'dark' || (appearance === 'system' && systemPrefersDark())
  return `${family}-${dark ? 'dark' : 'light'}`
}

export function applyTheme(family: Family, appearance: Appearance) {
  const root = document.documentElement
  const theme = themeName(family, appearance)
  if (theme === 'studio-light') root.removeAttribute('data-theme')
  else root.setAttribute('data-theme', theme)
}

/** Calls back when the system's own light changes; returns the way to stop. */
export function watchSystem(onChange: () => void): () => void {
  const media = window.matchMedia(QUERY)
  media.addEventListener('change', onChange)
  return () => media.removeEventListener('change', onChange)
}
