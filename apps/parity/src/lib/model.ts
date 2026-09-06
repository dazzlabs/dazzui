/*
 * The shape of what the viewer reads, and the one judgement it makes.
 *
 * The data is assembled at dev time by `parity-data.ts` — the React reference
 * folded into specimens, and what each platform measured — and the browser
 * only decides, per cell, whether a value matches. That decision is the same
 * one the platform tests make, written here a second time so a cell the
 * viewer paints red is a line the test would print.
 */

export type ThemeId =
  | 'studio-light'
  | 'studio-dark'
  | 'bright-light'
  | 'bright-dark'

export const THEMES: readonly { id: ThemeId; label: string }[] = [
  { id: 'studio-light', label: 'Studio Light' },
  { id: 'studio-dark', label: 'Studio Dark' },
  { id: 'bright-light', label: 'Bright Light' },
  { id: 'bright-dark', label: 'Bright Dark' },
]

export type PlatformId = 'flutter' | 'swiftui' | 'gpui'

/** The fields a check can be about, in the order the tables list them. */
export const FIELDS = [
  'width',
  'height',
  'background',
  'border-color',
  'border-width',
  'radius',
  'color',
  'font-size',
  'font-weight',
] as const
export type Field = (typeof FIELDS)[number]

export type Value = number | string | null

export interface Rect {
  x: number
  y: number
  width: number
  height: number
}

/** One distinct thing React drew: a class and its axes. */
export interface Specimen {
  key: string
  className: string
  data: Record<string, string>
  text: string
  /** The first story the specimen was seen in, and where it sat in it. */
  sample: { story: string; rects: Partial<Record<ThemeId, Rect>> }
  /** What React measured, per theme; null where the appearances disagreed. */
  expected: Partial<Record<ThemeId, Partial<Record<Field, Value>>>>
}

export interface Platform {
  id: PlatformId
  label: string
  /** Where the platform keeps its parity data, relative to the repo. */
  dir: string
  /** The checks its drift list names, as `<key> · <field>`. */
  drift: string[]
  /** What its reference test measured, or null if the test has not run. */
  measured: Record<
    string,
    Partial<Record<ThemeId, Partial<Record<Field, Value>>>>
  > | null
}

export interface ParityData {
  viewport: { width: number; height: number; scale: number }
  screenshots: boolean
  specimens: Specimen[]
  platforms: Platform[]
}

// ---------------------------------------------------------------------------
// The judgement.

function alpha(value: string): number {
  return value.length === 9 ? Number.parseInt(value.slice(7), 16) : 255
}

/** Equal to the eye: the same channels, with one step of slack on alpha. */
export function sameColour(a: string, b: string): boolean {
  if (alpha(a) === 0 && alpha(b) === 0) return true
  return a.slice(0, 7) === b.slice(0, 7) && Math.abs(alpha(a) - alpha(b)) <= 1
}

export function isColourField(field: Field): boolean {
  return field === 'background' || field === 'border-color' || field === 'color'
}

/**
 * Whether a platform's value matches React's for one field. `null` where
 * there is nothing to compare — one side has no value, or the field is one
 * the tests skip for this specimen (a width that depends on text).
 */
export function verdict(
  specimen: Specimen,
  field: Field,
  expected: Value | undefined,
  measured: Value | undefined
): 'match' | 'drift' | null {
  if (expected == null || measured == null) return null
  if (field === 'width' && specimen.text !== '') return null
  if (isColourField(field)) {
    return sameColour(String(expected), String(measured)) ? 'match' : 'drift'
  }
  if (typeof expected === 'number' && typeof measured === 'number') {
    if (field === 'font-weight')
      return expected === measured ? 'match' : 'drift'
    return Math.abs(expected - measured) <= 0.5 ? 'match' : 'drift'
  }
  return expected === measured ? 'match' : 'drift'
}

/** The check a cell belongs to, spelled the way drift.txt spells it. */
export function checkName(specimen: Specimen, field: Field): string {
  return `${specimen.key} · ${field}`
}

/** Every drifting check for a specimen on a platform, across all themes. */
export function driftingFields(
  specimen: Specimen,
  platform: Platform
): Set<Field> {
  const out = new Set<Field>()
  if (!platform.measured) return out
  const rows = platform.measured[specimen.key]
  if (!rows) return out
  for (const theme of THEMES) {
    const expected = specimen.expected[theme.id]
    const measured = rows[theme.id]
    if (!expected || !measured) continue
    for (const field of FIELDS) {
      if (
        verdict(specimen, field, expected[field], measured[field]) === 'drift'
      ) {
        out.add(field)
      }
    }
  }
  return out
}

/** A short, readable form of a measured value. */
export function format(field: Field, value: Value | undefined): string {
  if (value == null) return '—'
  if (typeof value === 'number') {
    if (field === 'font-weight') return String(value)
    const rounded = Math.round(value * 100) / 100
    return `${rounded}px`
  }
  return value
}
