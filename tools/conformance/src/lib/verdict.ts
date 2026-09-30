import type { Reading, Readings, Tolerances, Values } from './model'

/*
 * A verdict is derived, never stored. This module is the one place a
 * comparison is made, shared by the viewer and the command-line check.
 */

export type Verdict = 'match' | 'within' | 'off' | 'unread' | 'missing'

/** The properties a specimen is held to: its box, then the vocabulary. */
export function properties(reference: Reading): string[] {
  return ['width', 'height', ...Object.keys(reference.values)]
}

function valueOf(reading: Reading, property: string) {
  if (property === 'width') return reading.box.width
  if (property === 'height') return reading.box.height
  return reading.values[property] ?? null
}

/** sRGB hex, `#rrggbb` or `#rrggbbaa`, to OKLab and an alpha. */
function oklab(hex: string): [number, number, number, number] | null {
  const m = /^#?([0-9a-f]{6})([0-9a-f]{2})?$/i.exec(hex.trim())
  if (!m) return null
  const n = parseInt(m[1]!, 16)
  const alpha = m[2] ? parseInt(m[2], 16) / 255 : 1
  const lin = (c: number) => {
    const v = c / 255
    return v <= 0.04045 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4
  }
  const r = lin(n >> 16)
  const g = lin((n >> 8) & 255)
  const b = lin(n & 255)
  const l = Math.cbrt(0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b)
  const m_ = Math.cbrt(0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b)
  const s = Math.cbrt(0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b)
  return [
    0.2104542553 * l + 0.793617785 * m_ - 0.0040720468 * s,
    1.9779984951 * l - 2.428592205 * m_ + 0.4505937099 * s,
    0.0259040371 * l + 0.7827717662 * m_ - 0.808675766 * s,
    alpha,
  ]
}

/**
 * How far apart two colours are: the OKLab distance, weighted by how much
 * of each shows, plus the difference in how much shows. Two fully clear
 * colours are the same colour whatever their channels say.
 */
export function colourDistance(a: string, b: string): number | null {
  const x = oklab(a)
  const y = oklab(b)
  if (!x || !y) return null
  const shown = Math.min(x[3], y[3])
  return (
    Math.hypot(x[0] - y[0], x[1] - y[1], x[2] - y[2]) * shown +
    Math.abs(x[3] - y[3])
  )
}

export function compareValues(
  property: string,
  expected: Values[string],
  actual: Values[string],
  tolerances: Tolerances
): Verdict {
  if (actual === null || actual === undefined) return 'unread'
  if (expected === null || expected === undefined) return 'unread'
  if (expected === actual) return 'match'
  const kind = tolerances.kinds[property] ?? 'discrete'
  if (
    kind === 'length' &&
    typeof expected === 'number' &&
    typeof actual === 'number'
  ) {
    return Math.abs(expected - actual) < tolerances.length ? 'within' : 'off'
  }
  if (
    kind === 'colour' &&
    typeof expected === 'string' &&
    typeof actual === 'string'
  ) {
    const d = colourDistance(expected, actual)
    if (d === null) return 'off'
    return d < tolerances.colour ? 'within' : 'off'
  }
  return 'off'
}

export interface Check {
  theme: string
  address: string
  property: string
  expected: Values[string]
  actual: Values[string]
  verdict: Verdict
}

/** Every check for one specimen under one theme, against one implementation. */
export function checks(
  theme: string,
  address: string,
  reference: Readings,
  implementation: Readings,
  tolerances: Tolerances
): Check[] {
  const expected = reference.themes[theme]?.[address]
  if (!expected) return []
  const actual = implementation.themes[theme]?.[address]
  return properties(expected).map(property => {
    const want = valueOf(expected, property)
    if (!actual) {
      return {
        theme,
        address,
        property,
        expected: want,
        actual: null,
        verdict: 'missing',
      }
    }
    const got = valueOf(actual, property)
    return {
      theme,
      address,
      property,
      expected: want,
      actual: got,
      verdict: compareValues(property, want, got, tolerances),
    }
  })
}

export interface Tally {
  match: number
  within: number
  off: number
  unread: number
  missing: number
}

export const EMPTY: Tally = {
  match: 0,
  within: 0,
  off: 0,
  unread: 0,
  missing: 0,
}

export function tally(
  list: readonly Check[],
  into: Tally = { ...EMPTY }
): Tally {
  for (const check of list) into[check.verdict] += 1
  return into
}

export function add(a: Tally, b: Tally): Tally {
  return {
    match: a.match + b.match,
    within: a.within + b.within,
    off: a.off + b.off,
    unread: a.unread + b.unread,
    missing: a.missing + b.missing,
  }
}

/** Matched or within, over everything but unread. `null` when nothing was held to. */
export function score(t: Tally): number | null {
  const held = t.match + t.within + t.off + t.missing
  if (held === 0) return null
  return (t.match + t.within) / held
}

/** What a tally says about a specimen or a scene, in one word. */
export function summarize(t: Tally): Verdict {
  const held = t.match + t.within + t.off + t.missing
  if (held === 0) return 'unread'
  if (t.missing === held) return 'missing'
  if (t.off + t.missing > 0) return 'off'
  if (t.within > 0) return 'within'
  return 'match'
}
