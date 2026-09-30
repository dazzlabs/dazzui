import type { Readings, Tolerances } from './model'
import { checks } from './verdict.ts'

/*
 * The checks a platform fails today, as `expectations/<platform>.txt` lists
 * them, and how a run is held to that list. See specs/conformance.md.
 *
 * A line is `theme address property`. Two shorthands keep the file an
 * account a person can read: `*` for the theme is every theme the run
 * measured — which is what nearly every structural difference is — and `*`
 * for the property is a specimen missing as a whole.
 */

export const EVERY = '*'

export interface Line {
  theme: string
  address: string
  property: string
}

const key = ({ theme, address, property }: Line) =>
  `${theme} ${address} ${property}`

export function parse(text: string): Line[] {
  const lines: Line[] = []
  for (const raw of text.split('\n')) {
    const line = raw.replace(/#.*/, '').trim()
    if (!line) continue
    const [theme, address, property, ...rest] = line.split(/\s+/)
    if (!theme || !address || !property || rest.length)
      throw new Error(`Not an expectation: ${raw}`)
    lines.push({ theme, address, property })
  }
  return lines
}

/** What a run measured of one implementation, and every check it failed. */
export interface Measured {
  themes: readonly string[]
  /** Addresses the reference read, by theme: what could be held to. */
  held: ReadonlyMap<string, ReadonlySet<string>>
  /** `theme address property` for every off check, `theme address *` for every missing specimen. */
  failing: ReadonlySet<string>
}

export function measure(
  reference: Readings,
  implementation: Readings,
  tolerances: Tolerances
): Measured {
  const themes = Object.keys(implementation.themes).filter(
    theme => reference.themes[theme]
  )
  const held = new Map<string, Set<string>>()
  const failing = new Set<string>()
  for (const theme of themes) {
    const addresses = Object.keys(reference.themes[theme]!)
    held.set(theme, new Set(addresses))
    for (const address of addresses) {
      const list = checks(theme, address, reference, implementation, tolerances)
      if (list[0]?.verdict === 'missing') {
        failing.add(key({ theme, address, property: EVERY }))
        continue
      }
      for (const check of list)
        if (check.verdict === 'off')
          failing.add(key({ theme, address, property: check.property }))
    }
  }
  return { themes, held, failing }
}

export interface Held {
  /** Failing checks no line lists. */
  unexpected: string[]
  /** Lines that no longer fail, with what now passes. */
  stale: { line: Line; passes: string[] }[]
}

/** A run held to a platform's lines: what fails unlisted, and what is listed and passes. */
export function hold(lines: readonly Line[], measured: Measured): Held {
  const listed = new Set(lines.map(key))
  const unexpected = [...measured.failing].filter(failure => {
    const [, address, property] = failure.split(' ')
    return (
      !listed.has(failure) &&
      !listed.has(key({ theme: EVERY, address: address!, property: property! }))
    )
  })
  const stale: Held['stale'] = []
  for (const line of lines) {
    // A line about something this run did not measure says nothing either way.
    const themes = (
      line.theme === EVERY ? measured.themes : [line.theme]
    ).filter(theme => measured.held.get(theme)?.has(line.address))
    const passes = themes.filter(
      theme => !measured.failing.has(key({ ...line, theme }))
    )
    if (passes.length) stale.push({ line, passes })
  }
  return { unexpected: unexpected.sort(), stale }
}

/** Lines for every failing check, `*` where a check fails in every measured theme. */
export function baseline(measured: Measured): Line[] {
  const themesOf = new Map<string, string[]>()
  for (const failure of measured.failing) {
    const [theme, address, property] = failure.split(' ')
    const at = `${address} ${property}`
    themesOf.set(at, [...(themesOf.get(at) ?? []), theme!])
  }
  const lines: Line[] = []
  for (const [at, themes] of themesOf) {
    const [address, property] = at.split(' ') as [string, string]
    if (measured.themes.length > 1 && themes.length === measured.themes.length)
      lines.push({ theme: EVERY, address, property })
    else for (const theme of themes) lines.push({ theme, address, property })
  }
  return sort(lines)
}

/** The lines left once what now passes is taken out: a `*` line that passes somewhere keeps the themes it still fails in. */
export function prune(lines: readonly Line[], measured: Measured): Line[] {
  const { stale } = hold(lines, measured)
  const passing = new Map(stale.map(({ line, passes }) => [key(line), passes]))
  return lines.flatMap(line => {
    const passes = passing.get(key(line))
    if (!passes) return [line]
    if (line.theme !== EVERY) return []
    return measured.themes
      .filter(theme => !passes.includes(theme))
      .map(theme => ({ ...line, theme }))
  })
}

/** Addresses in the order a story reads — `button/2/10` after `button/2/9`. */
function sort(lines: Line[]): Line[] {
  const collator = new Intl.Collator('en', { numeric: true })
  return lines.sort(
    (a, b) =>
      collator.compare(a.address, b.address) ||
      (a.property === EVERY ? -1 : b.property === EVERY ? 1 : 0) ||
      a.property.localeCompare(b.property) ||
      (a.theme === EVERY ? -1 : b.theme === EVERY ? 1 : 0) ||
      a.theme.localeCompare(b.theme)
  )
}

/** A platform's file: its header, then the lines, aligned. */
export function format(header: readonly string[], lines: Line[]): string {
  const width = Math.max(0, ...lines.map(line => line.theme.length))
  const addressWidth = Math.max(0, ...lines.map(line => line.address.length))
  const body = sort([...lines]).map(
    line =>
      `${line.theme.padEnd(width)}  ${line.address.padEnd(addressWidth)}  ${line.property}`
  )
  return [...header, ...(body.length ? ['', ...body] : [])].join('\n') + '\n'
}
