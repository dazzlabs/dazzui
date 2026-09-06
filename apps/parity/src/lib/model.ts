/*
 * The shape of what the viewer reads, and the one judgement it makes.
 *
 * The data is assembled at dev time by `parity-data.ts` — the React
 * reference folded into specimens, and what each platform measured.
 *
 * **What is comparable is the suite's decision, not this page's.** A
 * platform's `measured.json` carries only the fields its reference test
 * actually compared: a field it skips — a callout's inherited type, a
 * spinner's two-coloured ring, the height of a box its content sets — is
 * absent rather than merely ignored. So the rule here is only ever *do both
 * sides have a value*, and a cell this page paints red is a line that suite
 * would print. Keeping a copy of the policy here is what let the two drift
 * apart once already.
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
  /**
   * Whether what it holds is built out of elements rather than being one run
   * of text — a card's heading over a paragraph rather than a badge's label.
   * A port's adapter is given the axes and the text and nothing else, so it
   * cannot reproduce the second, and a difference between the two pictures
   * would be the story's content rather than the component's drawing.
   */
  composed: boolean
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
  /** Whether its suite has drawn every specimen it can build. */
  renders: boolean
  /** What its reference test measured, or null if the test has not run. */
  measured: Record<
    string,
    Partial<Record<ThemeId, Partial<Record<Field, Value>>>>
  > | null
}

export interface ParityData {
  viewport: { width: number; height: number; scale: number }
  screenshots: boolean
  /** Whether the extractor has drawn every specimen on its own. */
  renders: boolean
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

/**
 * The slack a length is allowed, and what it is for.
 *
 * Two text systems do not agree to the pixel, so a length passes while it is
 * *inside* half a pixel. Half a pixel exactly does not: on a 2x display that
 * is one device pixel, the difference between a rule drawn at `spacing.px`
 * and one drawn at `stroke.hairline` — a decision about the design rather
 * than noise from a text engine.
 */
export const HALF_A_PIXEL = 0.5

export function isColourField(field: Field): boolean {
  return field === 'background' || field === 'border-color' || field === 'color'
}

/**
 * Whether a platform's value matches React's for one field. `null` where
 * there is nothing to compare: one side has no value, which for a platform
 * means its own suite did not compare that field.
 */
export function verdict(
  _specimen: Specimen,
  field: Field,
  expected: Value | undefined,
  measured: Value | undefined
): 'match' | 'drift' | null {
  if (expected == null || measured == null) return null
  if (isColourField(field)) {
    return sameColour(String(expected), String(measured)) ? 'match' : 'drift'
  }
  if (typeof expected === 'number' && typeof measured === 'number') {
    if (field === 'font-weight')
      return expected === measured ? 'match' : 'drift'
    return Math.abs(expected - measured) < HALF_A_PIXEL ? 'match' : 'drift'
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

/** A specimen's key as a file name — the derivation every platform shares. */
export function renderName(key: string): string {
  return key
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '')
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
