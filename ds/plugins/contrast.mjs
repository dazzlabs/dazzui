/**
 * Terrazzo plugin that holds the ink ladder to WCAG 2 contrast, and the
 * surface ladder to a minimum step, in every theme.
 *
 * Terrazzo's own `a11y/min-contrast` rule cannot do this job here: 0.10
 * lints each token file with only that file's tokens, and it reads default
 * values only. This plugin runs on the merged graph after every mode is
 * parsed, so one table of pairs is checked four times — once per theme —
 * and a re-pointed role that lands below the floor stops the build instead
 * of shipping.
 *
 * What is deliberately not listed: `content-subtle` and `content-faint`,
 * the caption and disabled weights, quiet below AA by design. The filled
 * control's label (`on-accent` over the accent fill) is held to 3:1 rather
 * than 4.5:1 — the UI-component floor — because Studio Dark's violet fill
 * measures 4.35 under a white label; known, accepted, and guarded from
 * getting worse.
 *
 * The second table is about the surfaces themselves. Two fills that sit on
 * each other — a card on the paper, a hover on the card, a table header on
 * its body — have to differ by enough to be seen, and WCAG has no number
 * for that: a ratio of 1.1:1 is what every one of those pairs measures. So
 * the surface ladder is held to a CIE L* difference instead, the closest
 * thing to a perceptual step. The floors are not an ideal: they are the
 * steps the BeyondTranslate artboards (4a, 4c, 4b, 4e) draw between those
 * fills, rounded down a little, so that a theme can be re-pointed without
 * two rungs collapsing onto each other or trading places — which is what
 * had happened to Bright Light's hover before this table existed — while a
 * value the design itself sets a hair apart is never failed for being what
 * the design drew.
 */

/** { fg, bg, min } — token ids, and the ratio the pair must clear. */
const PAIRS = [
  { fg: 'color.content', bg: 'color.surface', min: 4.5 },
  { fg: 'color.content', bg: 'color.canvas', min: 4.5 },
  { fg: 'color.content', bg: 'color.surface-muted', min: 4.5 },
  { fg: 'color.content', bg: 'color.surface-chrome', min: 4.5 },
  { fg: 'color.content-secondary', bg: 'color.surface', min: 4.5 },
  { fg: 'color.content-nav', bg: 'color.surface', min: 4.5 },
  { fg: 'color.content-muted', bg: 'color.surface', min: 4.5 },
]

/**
 * { a, b, light, dark } — two surface roles that sit on each other, and the
 * least L* between them under a light and a dark theme. A translucent role
 * (`surface-subtle`, the hover wash) is composited over the other first.
 */
const LADDER = [
  { a: 'color.surface', b: 'color.surface-muted', light: 1.5, dark: 1.5 },
  { a: 'color.surface', b: 'color.surface-column', light: 0.8, dark: 0.8 },
  { a: 'color.surface', b: 'color.surface-chrome', light: 1.5, dark: 1.5 },
  { a: 'color.surface', b: 'color.surface-subtle', light: 4, dark: 5 },
  {
    a: 'color.surface-muted',
    b: 'color.surface-subtle',
    light: 3.5,
    dark: 4.5,
  },
  {
    a: 'color.surface-column',
    b: 'color.surface-subtle',
    light: 3.5,
    dark: 4.5,
  },
  {
    a: 'color.surface-overlay',
    b: 'color.surface-subtle',
    light: 4,
    dark: 4.5,
  },
  { a: 'color.surface', b: 'color.surface-inset', light: 3.5, dark: 5 },
  { a: 'color.surface-muted', b: 'color.surface-inset', light: 1, dark: 1 },
  { a: 'color.surface', b: 'color.surface-sunken', light: 7, dark: 8 },
  { a: 'color.surface-inset', b: 'color.surface-raised', light: 4.5, dark: 9 },
  { a: 'color.surface-sunken', b: 'color.surface-raised', light: 8, dark: 6 },
  { a: 'color.canvas', b: 'color.surface', light: 5, dark: 1 },
  { a: 'color.surface', b: 'color.surface-overlay', light: 0, dark: 1.5 },
]

class ContrastPlugin {
  constructor(options = {}) {
    this.modeSelectors = options.modeSelectors ?? []
  }

  get name() {
    return '@dazzlabs/dazzui-ds-plugin-contrast'
  }

  async build({ tokens }) {
    const themes = collectThemes(this.modeSelectors)
    const failures = []

    for (const [theme, modes] of themes) {
      for (const { fg, bg, min } of PAIRS) {
        check(tokens, theme, modes, fg, bg, min, failures)
      }

      const scheme = modes.includes('dark') ? 'dark' : 'light'
      for (const pair of LADDER) {
        step(tokens, theme, modes, pair.a, pair.b, pair[scheme], failures)
      }

      // The filled control's label over the fill it is printed on. The fill
      // is a recipe — a shade off the primary ramp, re-picked per theme —
      // so the pair is assembled the way a platform assembles it.
      const shade = valueFor(
        tokens['control.color.filled.surface.normal.shade'],
        modes
      )
      if (shade !== undefined) {
        check(
          tokens,
          theme,
          modes,
          'color.on-accent',
          `color.primary.${shade}`,
          3,
          failures
        )
      }
    }

    if (failures.length > 0) {
      throw new Error(`Contrast guard failed:\n  ${failures.join('\n  ')}`)
    }
  }
}

export default function contrastPlugin(options) {
  return new ContrastPlugin(options)
}

/**
 * The themes as ['name', [modes…]] in config order, the default first. The
 * same collapse every platform plugin performs: a theme selected by several
 * modeSelectors applies them in config order, later winning.
 */
function collectThemes(modeSelectors) {
  const themes = new Map([['default', []]])
  for (const { mode, selectors = [] } of modeSelectors) {
    for (const selector of selectors) {
      if (!themes.has(selector)) themes.set(selector, [])
      themes.get(selector).push(mode)
    }
  }
  return themes
}

/** The token's value under a theme's mode chain, later modes winning. */
function valueFor(token, modes) {
  if (!token) return undefined
  let value = token.$value
  for (const mode of modes) {
    if (token.mode?.[mode]) value = token.mode[mode].$value
  }
  return value
}

function check(tokens, theme, modes, fgId, bgId, min, failures) {
  const fg = toRgba(valueFor(tokens[fgId], modes), fgId)
  const bg = toRgba(valueFor(tokens[bgId], modes), bgId)
  const ratio = contrast(fg, bg)
  if (ratio < min) {
    failures.push(
      `${fgId} on ${bgId} under "${theme}": ${ratio.toFixed(2)}, needs ${min}`
    )
  }
}

/**
 * The L* step between two surfaces, the translucent one (if either is)
 * composited over the other. Fails when the step is under the floor; a
 * floor of 0 is a pair the scheme does not separate at all.
 */
function step(tokens, theme, modes, aId, bId, min, failures) {
  if (min === 0) return
  let a = toRgba(valueFor(tokens[aId], modes), aId)
  let b = toRgba(valueFor(tokens[bId], modes), bId)
  if (a.a < 1) [a, b] = [b, a]
  const over = b.c.map((v, i) => v * b.a + a.c[i] * (1 - b.a))
  const delta = Math.abs(lightness(over) - lightness(a.c))
  if (delta < min) {
    failures.push(
      `${aId} vs ${bId} under "${theme}": ΔL* ${delta.toFixed(1)}, needs ${min}`
    )
  }
}

/** CIE L* of an opaque srgb colour. */
function lightness(rgb) {
  const y = luminance(rgb)
  return y > 0.008856 ? 116 * Math.cbrt(y) - 16 : 903.3 * y
}

/** Normalized color value (hex string or srgb object) -> { c: [r,g,b], a }. */
function toRgba(value, id) {
  if (typeof value === 'string' && value.startsWith('#')) {
    const hex = value.slice(1)
    const c = [0, 2, 4].map(i => parseInt(hex.slice(i, i + 2), 16) / 255)
    const a = hex.length === 8 ? parseInt(hex.slice(6, 8), 16) / 255 : 1
    return { c, a }
  }
  if (value && typeof value === 'object' && Array.isArray(value.components)) {
    if (value.colorSpace && value.colorSpace !== 'srgb') {
      throw new Error(
        `${id} is in ${value.colorSpace}; the contrast guard only reads ` +
          'srgb. Write the token in hex, the precision the design ships at.'
      )
    }
    return { c: value.components.slice(0, 3), a: value.alpha ?? 1 }
  }
  throw new Error(
    `${id} did not resolve to a color the contrast guard can read.`
  )
}

/** WCAG 2 contrast, the translucent ink composited over its ground first. */
function contrast(fg, bg) {
  const over = fg.c.map((v, i) => v * fg.a + bg.c[i] * (1 - fg.a))
  const [hi, lo] = [luminance(over), luminance(bg.c)].sort((a, b) => b - a)
  return (hi + 0.05) / (lo + 0.05)
}

function luminance(rgb) {
  const [r, g, b] = rgb.map(v =>
    v <= 0.03928 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4
  )
  return 0.2126 * r + 0.7152 * g + 0.0722 * b
}
