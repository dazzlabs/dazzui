import tokensCss from '@dazzlabs/dazzui/style.css?raw'

export type Token = {
  /** Custom property name without the leading dashes, e.g. `color-primary-500`. */
  name: string
  /** The custom property itself, e.g. `--color-primary-500`. */
  property: string
  /** Value as authored in `style.css`, often a `var()` reference. */
  value: string
}

/**
 * `style.css` starts with the canonical `:root` block, followed by
 * `@media (color-gamut: …)` blocks that re-declare a subset of the colors.
 * Only the first block is the full token list, so anchor on column 0.
 */
const ROOT_BLOCK = /^:root\s*\{([\s\S]*?)^\}/m
const DECLARATION = /--([\w.-]+)\s*:\s*([^;]+);/g

function parse(css: string): Token[] {
  const block = ROOT_BLOCK.exec(css)?.[1]
  if (!block) {
    throw new Error(
      'Could not find the ":root" block in @dazzlabs/dazzui/style.css. ' +
        'Run `pnpm tokens` to rebuild the tokens.'
    )
  }

  const parsed: Token[] = []
  for (const [, name, value] of block.matchAll(DECLARATION)) {
    parsed.push({
      name,
      property: `--${name}`,
      value: value.replace(/\s+/g, ' ').trim(),
    })
  }
  return parsed
}

/** Every token declared by `@dazzlabs/dazzui`, in source order. */
export const tokens = parse(tokensCss)

const tokensByName = new Map(tokens.map(token => [token.name, token]))

/** Look up a single token by name, e.g. `getToken('radius-medium')`. */
export function getToken(name: string): Token | undefined {
  return tokensByName.get(name)
}

/**
 * Named sizes, ordered small to large. Covers both the DazzUI scale
 * (`tiny` … `big`) and the Tailwind-derived base scale (`2xs` … `7xl`).
 */
const NAMED_SIZES = [
  'none',
  'tiny',
  '4xs',
  '3xs',
  '2xs',
  'xs',
  'sm',
  'small',
  'base',
  'md',
  'medium',
  'lg',
  'large',
  'big',
  'xl',
  '2xl',
  '3xl',
  '4xl',
  '5xl',
  '6xl',
  '7xl',
  'full',
]

/** `0-5` -> 0.5, `12` -> 12, anything else -> null. */
function numericKey(key: string): number | null {
  // `px` is a literal 1px step, which sits between `0` and `0.5` on the scale.
  if (key === 'px') return 0.0625

  const numeric = Number(key.replace('-', '.'))
  return Number.isFinite(numeric) ? numeric : null
}

/** Order scale keys the way a designer reads them: smallest first. */
export function compareKeys(a: string, b: string): number {
  if (a === b) return 0

  const numericA = numericKey(a)
  const numericB = numericKey(b)
  if (numericA !== null && numericB !== null) return numericA - numericB
  if (numericA !== null) return -1
  if (numericB !== null) return 1

  const indexA = NAMED_SIZES.indexOf(a)
  const indexB = NAMED_SIZES.indexOf(b)
  if (indexA !== -1 && indexB !== -1) return indexA - indexB
  if (indexA !== -1) return -1
  if (indexB !== -1) return 1

  return a.localeCompare(b)
}

export type PrefixOptions = {
  /** Drop tokens whose remaining name still contains a `-`. */
  leafOnly?: boolean
  /** Sort by the remaining name using {@link compareKeys}. */
  sorted?: boolean
}

/** Tokens under a namespace, e.g. `byPrefix('radius-')`. */
export function byPrefix(prefix: string, options: PrefixOptions = {}): Token[] {
  const matched = tokens.filter(token => {
    if (!token.name.startsWith(prefix)) return false
    return !options.leafOnly || !token.name.slice(prefix.length).includes('-')
  })

  if (!options.sorted) return matched
  return [...matched].sort((a, b) =>
    compareKeys(a.name.slice(prefix.length), b.name.slice(prefix.length))
  )
}

/** The shared control sizing scale, smallest first. */
const CONTROL_SIZES = ['tiny', 'small', 'medium', 'large', 'big']

/**
 * One component's tokens: the parts shared across sizes first, then a group
 * per size, so the table reads in the order the scale does.
 */
export function componentTokens(component: string): Token[] {
  const prefix = `${component}-`
  const rank = (token: Token) =>
    CONTROL_SIZES.indexOf(token.name.slice(prefix.length).split('-')[0] ?? '')

  return byPrefix(prefix).sort(
    (a, b) => rank(a) - rank(b) || a.name.localeCompare(b.name)
  )
}

export type TokenScale = {
  /** The shared part of the name, e.g. `primary` for `color-primary-*`. */
  key: string
  steps: { step: string; token: Token }[]
}

/**
 * Split `<prefix><key>-<step>` tokens into one scale per key, e.g.
 * `groupScales('color-')` returns primary, neutral, info, … each with 50…950.
 */
export function groupScales(
  prefix: string,
  isStep: (step: string) => boolean = () => true
): TokenScale[] {
  const scales = new Map<string, TokenScale>()

  for (const token of byPrefix(prefix)) {
    const rest = token.name.slice(prefix.length)
    const separator = rest.lastIndexOf('-')
    if (separator === -1) continue

    const key = rest.slice(0, separator)
    const step = rest.slice(separator + 1)
    if (!isStep(step)) continue

    let scale = scales.get(key)
    if (!scale) {
      scale = { key, steps: [] }
      scales.set(key, scale)
    }
    scale.steps.push({ step, token })
  }

  for (const scale of scales.values()) {
    scale.steps.sort((a, b) => compareKeys(a.step, b.step))
  }
  return [...scales.values()]
}

/** True for the numeric shades of a color ramp (`50` … `950`). */
export function isColorShade(step: string): boolean {
  return /^\d+$/.test(step)
}

/** Resolve a custom property against the live document. */
export function resolve(property: string): string {
  if (typeof document === 'undefined') return ''
  return getComputedStyle(document.documentElement)
    .getPropertyValue(property)
    .trim()
}
