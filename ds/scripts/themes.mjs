/**
 * Themes read off another design system rather than drawn here.
 *
 * The six hand-drawn families write every role for every theme, because each
 * one was read off an artboard. An imported theme has no artboard, only the
 * few colours its source decides everything else from — Omarchy's
 * `colors.toml`, a desktop's window background, label colour and system
 * accent — so those are all a file under `themes/` holds, and this derives
 * the rest into the roles this graph already has:
 *
 *   surface   the background, exactly — the paper is the theme's own
 *   ladder    the other surfaces stepped off the paper towards the
 *             foreground (or, for a dark column, under it) by the CIE L*
 *             the design's own boards step by, which `plugins/contrast.mjs`
 *             then holds them to like any other theme
 *   ink       the foreground, and its quieter steps mixed back into the
 *             paper, each pushed up until it clears the contrast floor
 *   hairlines the foreground at the source's separator alpha when it
 *             states one, else at the 13% / 16% every family shares, the
 *             floating edge a rung above; and the control outline at 38%
 *             of it — Omarchy's own, and close to what the desktops draw
 *             their unticked boxes in
 *   on-accent what the source prints on its accent when it says so, else
 *             the paper, else the ink — whichever clears 3:1 first
 *   primary   a ramp through the accent in OKLCH, with the accent itself
 *             at the step the `filled` recipe picks — 600 in a light
 *             theme, 500 in a dark one — so a filled button is the theme's
 *             accent to the digit
 *
 * A file also names its *metrics*: the geometry a family shares — Hyprland's
 * square corner, AppKit's capsule, Fluent's 4px control corner — as token
 * values applied under a family mode, beside `bright` and `graphite`. Several
 * themes can share one: Debian, Fedora and Ubuntu all wear Adwaita's.
 *
 * Like `recipes.mjs`, this runs on the token documents before Terrazzo parses
 * them and writes nothing back. Every theme's derived values are a mode named
 * after the theme itself, added to the token that already carries the role;
 * every metrics group is a mode named after the group.
 */

import fs from 'node:fs'
import YAML from 'yaml'

const DIR = new URL('../themes/', import.meta.url)
const SOURCES = ['omarchy.yaml', 'desktops.yaml', 'applications.yaml']
const REQUIRED = ['title', 'scheme', 'background', 'foreground', 'accent']

/**
 * `{ file, metrics, themes }` per source, in `SOURCES` order. A theme is
 * `{ name, title, scheme, background, foreground, accent, onAccent?,
 * metrics? }`, its name being its `data-theme` value.
 */
export const IMPORTED = SOURCES.map(file => {
  const source = YAML.parse(fs.readFileSync(new URL(file, DIR), 'utf8'))
  const metrics = source.metrics ?? {}
  const themes = Object.entries(source.themes ?? {}).map(([name, spec]) => {
    const { 'on-accent': onAccent, ...rest } = spec
    const theme = { metrics: source.defaults?.metrics, ...rest, onAccent, name }
    for (const key of REQUIRED) {
      if (!theme[key]) {
        throw new Error(`themes/${file}: \`${name}\` is missing \`${key}\`.`)
      }
    }
    if (theme.scheme !== 'light' && theme.scheme !== 'dark') {
      throw new Error(
        `themes/${file}: \`${name}\`'s scheme is \`light\` or \`dark\`.`
      )
    }
    if (theme.metrics && !metrics[theme.metrics]) {
      throw new Error(
        `themes/${file}: \`${name}\` wears metrics \`${theme.metrics}\`, ` +
          'which the file does not define.'
      )
    }
    return theme
  })
  return {
    file,
    metrics,
    themes: source.defaults?.opposite ? themes.flatMap(bothModes) : themes,
  }
})

/**
 * A theme that ships in one appearance, as the pair `<name>-light` and
 * `<name>-dark`: the one it ships in, exactly, and the other derived from
 * its own three colours the way dazzle's prototype does it — the foreground
 * becomes the paper (lifted towards white, or pressed towards black), the
 * paper becomes the ink, and the accent is pressed or lifted so a label
 * printed in it still reads. The derived half is this system's, not the
 * source's; the shipped half is the source's to the digit.
 */
function bothModes(theme) {
  const other = theme.scheme === 'light' ? 'dark' : 'light'
  const derived =
    other === 'light'
      ? {
          background: mixHex(theme.foreground, '#ffffff', 0.78),
          foreground: theme.background,
          accent: mixHex(theme.accent, '#000000', 0.38),
        }
      : {
          background: mixHex(theme.foreground, '#000000', 0.55),
          foreground: theme.background,
          accent: mixHex(theme.accent, '#ffffff', 0.38),
        }
  return [
    { ...theme, name: `${theme.name}-${theme.scheme}` },
    {
      ...theme,
      ...derived,
      colors: undefined,
      onAccent: undefined,
      scheme: other,
      name: `${theme.name}-${other}`,
    },
  ].sort((a, b) => (a.scheme === 'light' ? -1 : b.scheme === 'light' ? 1 : 0))
}

/** `a` mixed `t` of the way towards `b`, in sRGB, as hex. */
function mixHex(a, b, t) {
  const channels = value =>
    [1, 3, 5].map(i => parseInt(value.slice(i, i + 2), 16))
  const [from, to] = [channels(a), channels(b)]
  return `#${from
    .map((v, i) =>
      Math.round(v + (to[i] - v) * t)
        .toString(16)
        .padStart(2, '0')
    )
    .join('')}`
}

const THEMES = IMPORTED.flatMap(source => source.themes)
const selector = theme => `[data-theme='${theme.name}']`

/**
 * The imported themes' mode selectors, in the three places the config's
 * order needs them: the metrics modes beside the other families, the dark
 * themes beside the other dark ones, and one mode per theme among the
 * corners, last, since it carries what is that theme's alone.
 */
export const importedSelectors = {
  families: IMPORTED.flatMap(source =>
    Object.keys(source.metrics).map(mode => ({
      mode,
      selectors: source.themes
        .filter(theme => theme.metrics === mode)
        .map(selector),
      scheme: 'light',
    }))
  ),
  dark: THEMES.filter(theme => theme.scheme === 'dark').map(selector),
  corners: THEMES.map(theme => ({
    mode: theme.name,
    selectors: [selector(theme)],
    scheme: theme.scheme,
  })),
}

const STEPS = [
  '50',
  '100',
  '200',
  '300',
  '400',
  '500',
  '600',
  '700',
  '800',
  '900',
  '950',
]

/**
 * The token document with every imported theme's derived values, and every
 * metrics group's geometry, added as modes. A document that holds none of
 * those tokens comes back as it went in.
 */
export function expandThemes(source) {
  const document = structuredClone(source)
  let touched = false
  const put = (path, mode, value) => {
    const token = at(document, path)
    if (!token) return
    token.$extensions ??= {}
    token.$extensions.mode ??= {}
    token.$extensions.mode[mode] = value
    touched = true
  }

  for (const { metrics } of IMPORTED) {
    for (const [mode, values] of Object.entries(metrics)) {
      for (const [path, value] of Object.entries(values)) {
        if (path === 'type') continue
        put(path, mode, value)
      }
      if (values.type) {
        for (const [face, value] of typeFaces(document, values.type)) {
          put(face, mode, value)
        }
      }
    }
  }

  for (const theme of THEMES) {
    const derived = derive(theme)
    const base = document?.base?.color
    if (base && base.brand) {
      base[theme.name] = Object.fromEntries(
        STEPS.map(step => [step, { $value: derived.ramp[step] }])
      )
      touched = true
    }
    for (const [path, value] of Object.entries(derived.roles)) {
      put(path, theme.name, value)
    }
  }
  return touched ? document : source
}

/**
 * A metrics group's `type` scale, spread over the faces that read it.
 *
 * The scale is the platform's own, one entry per step of `base.type` a
 * face is set at — `caption`, `small`, `body`, `emphasis`, `title` — as
 * `[size, leading]` in px, and `label`: the size the platform sets a
 * control's text in. Each face keeps its family and weight and moves to its
 * step's size; a face whose leading is its own size (the `label` faces,
 * which never wrap) keeps that, and the `small` step's label faces — what a
 * control's text is set in — take `label`. The headline steps are not the
 * chrome's and are left alone.
 */
function typeFaces(document, scale) {
  const faces = []
  for (const [name, token] of Object.entries(document ?? {})) {
    if (token?.$type !== 'typography') continue
    const value = token.$value
    const step = /^\{base\.type\.([a-z0-9]+)\}$/.exec(value.fontSize)?.[1]
    if (!step || !scale[step]) continue
    const tight = value.lineHeight === value.fontSize
    const size =
      tight && step === 'small' && scale.label ? scale.label : scale[step][0]
    faces.push([
      name,
      {
        ...value,
        fontSize: `${size}px`,
        lineHeight: `${tight ? size : scale[step][1]}px`,
      },
    ])
  }
  return faces
}

/**
 * Every metrics path, checked once against the merged sources, so a misspelt
 * token path fails the build rather than quietly setting nothing.
 */
export function checkMetrics(documents) {
  for (const { file, metrics } of IMPORTED) {
    for (const [mode, values] of Object.entries(metrics)) {
      for (const path of Object.keys(values)) {
        if (path === 'type') continue
        if (!documents.some(document => at(document, path))) {
          throw new Error(
            `themes/${file}: metrics \`${mode}\` sets \`${path}\`, ` +
              'which no token file declares.'
          )
        }
      }
    }
  }
}

function at(document, path) {
  let node = document
  for (const key of path.split('.')) {
    if (!node || typeof node !== 'object' || !(key in node)) return undefined
    node = node[key]
  }
  return node && typeof node === 'object' && '$value' in node ? node : undefined
}

/**
 * The CIE L* each surface stands off the paper by. They are the steps the
 * BeyondTranslate boards draw between the same pairs, taken a little above
 * the floors `plugins/contrast.mjs` holds every theme to, so a derived theme
 * lands inside the guard rather than on its edge.
 */
const LADDER = {
  light: {
    muted: 2.5,
    column: 1.5,
    inset: 5.5,
    sunken: 10.5,
    canvas: 6,
  },
  dark: {
    muted: 3,
    column: 1.5,
    inset: 6,
    sunken: 10,
    raised: 20,
    canvas: 2,
  },
}

/** Every derived value for one theme: its ramp, and its roles by token path. */
export function derive(theme) {
  const dark = theme.scheme === 'dark'
  const paper = parse(theme.background)
  // A label colour written with an alpha — AppKit's `labelColor` is black
  // at 85% — is what it paints over the paper, so it is composited first.
  const ink = parse(theme.foreground, paper)
  const accent = parse(theme.accent, paper)
  const steps = LADDER[theme.scheme]
  const anchor = dark ? '500' : '600'
  const ramp = rampThrough(accent, anchor, paper, dark)

  const off = dL => stepOff(paper, ink, dL)
  // A dark column recedes *under* the paper; a paper already at black has
  // nowhere under it to go, and lifts instead.
  const under = dL => stepOff(paper, [0, 0, 0], dL) ?? off(dL)

  // A source that publishes a surface ladder can retain those exact roles.
  // Derive the hover wash against the actual surfaces, including overrides.
  const color = (role, fallback) =>
    theme.colors?.[role] ? parse(theme.colors[role], paper) : fallback
  const muted = color('surface-muted', off(steps.muted))
  const column = color(
    'surface-column',
    dark ? under(steps.column) : off(steps.column)
  )
  const inset = color('surface-inset', off(steps.inset))
  const sunken = color('surface-sunken', off(steps.sunken))
  const raised = color('surface-raised', dark ? off(steps.raised) : paper)
  const canvas = color('canvas', dark ? under(steps.canvas) : off(steps.canvas))
  const overlay = color('surface-overlay', dark ? muted : paper)

  // The hover wash: the ink at the least alpha that moves every fill it
  // lands on by the guard's step, and never under the design's own 6% / 7%.
  const washOver = [
    [paper, dark ? 5.5 : 4.5],
    [muted, dark ? 5 : 4],
    [column, dark ? 5 : 4],
    [overlay, dark ? 5 : 4.5],
  ]
  const subtle = Math.max(
    dark ? 0.07 : 0.06,
    ...washOver.map(([ground, dL]) => alphaFor(ink, ground, dL))
  )

  // The quieter inks, mixed back into the paper and pushed up until each
  // clears the floor — in order, so no step lands above the one before it.
  const faint = mix(paper, ink, 0.36)
  const subtleInk = mix(paper, ink, 0.5)
  const mutedT = legible(paper, ink, 0.64, 4.6)
  const navT = Math.max(legible(paper, ink, 0.72, 4.6), mutedT)
  const secondaryT = Math.max(legible(paper, ink, 0.82, 4.6), navT)

  // The source's own separator alpha when it states one (AppKit rules at
  // 7–8%, far lighter than the families here); the shared 13% / 16%
  // otherwise. The floating edge keeps its rung above it.
  const separator = theme.separator ?? (dark ? 0.16 : 0.13)

  const onAccent = theme.onAccent
    ? parse(theme.onAccent)
    : ([paper, ink].find(c => contrast(c, accent) >= 3) ?? paper)

  const roles = {
    canvas: hex(canvas),
    surface: hex(paper),
    'surface-muted': hex(muted),
    'surface-sunken': hex(sunken),
    'surface-subtle': hex(ink, subtle),
    'surface-inset': hex(inset),
    'surface-raised': hex(raised),
    'surface-overlay': hex(overlay),
    'surface-chrome': hex(muted),
    'surface-column': hex(column),
    content: hex(ink),
    'content-secondary': hex(mix(paper, ink, secondaryT)),
    'content-nav': hex(mix(paper, ink, navT)),
    'content-muted': hex(mix(paper, ink, mutedT)),
    'content-subtle': hex(subtleInk),
    'content-faint': hex(faint),
    border: hex(ink, separator),
    'border-strong': hex(ink, separator + (theme.separator ? 0.03 : 0.04)),
    'border-muted': hex(mix(paper, ink, 0.38)),
    'on-accent': hex(onAccent),
  }

  for (const [role, value] of Object.entries(theme.colors ?? {})) {
    if (
      !(role in roles) ||
      role === 'surface' ||
      role === 'content' ||
      role === 'on-accent'
    ) {
      throw new Error(
        `${theme.name}: unknown or palette-owned color role \`${role}\`.`
      )
    }
    roles[role] = hex(parse(value, paper))
  }

  const ref = step => `{base.color.${theme.name}.${step}}`
  return {
    ramp: Object.fromEntries(STEPS.map(step => [step, hex(ramp[step])])),
    roles: {
      ...Object.fromEntries(
        Object.entries(roles).map(([role, value]) => [`color.${role}`, value])
      ),
      ...Object.fromEntries(
        STEPS.map(step => [`color.primary.${step}`, ref(step)])
      ),
      'progress.gradient.from': ref(anchor),
      'progress.gradient.to': ref('300'),
    },
  }
}

/* The ramp ----------------------------------------------------------------- */

/**
 * OKLCH lightness and relative chroma along a ramp, the shape a hand-drawn
 * ramp in `_base.yaml` has: pale and nearly grey at 50, most saturated
 * through the middle, deep and a little quieter at 950.
 */
const SHAPE = {
  50: [0.975, 0.12],
  100: [0.945, 0.25],
  200: [0.89, 0.45],
  300: [0.82, 0.7],
  400: [0.72, 0.9],
  500: [0.64, 1],
  600: [0.56, 1],
  700: [0.49, 0.92],
  800: [0.42, 0.82],
  900: [0.36, 0.7],
  950: [0.27, 0.55],
}

/**
 * A ramp with `accent` exactly at `anchor`. Every other step keeps the
 * accent's hue, and its lightness is the shape's, stretched or squeezed so
 * the two ends stay where the shape puts them whatever the accent's own
 * lightness — an accent already near white still leaves 50 the palest step
 * and 950 the deepest. Chroma follows the shape and is pulled in until the
 * step is inside sRGB.
 *
 * The steps a recipe sets text in — 700 and 800 on paper, 300 and 200 on a
 * dark ground — are then pushed further from the paper until they clear
 * 4.5:1 on it, since an accent Omarchy chose to fill with is not always one
 * that can be read as a label a grade off it.
 */
function rampThrough(accent, anchor, paper, dark) {
  const [L, C, h] = oklch(accent)
  const [aL, aC] = SHAPE[anchor]
  const light = SHAPE['50'][0]
  const deep = SHAPE['950'][0]
  const lightnessOf = step => {
    const sL = SHAPE[step][0]
    return sL > aL
      ? L + ((Math.max(light, L) - L) * (sL - aL)) / (light - aL)
      : L - ((L - Math.min(deep, L)) * (aL - sL)) / (aL - deep)
  }
  const ramp = {}
  for (const step of STEPS) {
    ramp[step] =
      step === anchor
        ? accent
        : inGamut(lightnessOf(step), (C * SHAPE[step][1]) / aC, h)
  }
  for (const step of dark ? ['300', '200'] : ['700', '800']) {
    let sL = oklch(ramp[step])[0]
    const chroma = (C * SHAPE[step][1]) / aC
    while (contrast(ramp[step], paper) < 4.5 && sL > 0.02 && sL < 0.99) {
      sL += dark ? 0.01 : -0.01
      ramp[step] = inGamut(sL, chroma, h)
    }
  }
  return ramp
}

function inGamut(L, C, h) {
  let c = C
  for (let i = 0; i < 40; i++) {
    const rgb = fromOklch([L, c, h])
    if (rgb.every(v => v >= -1e-4 && v <= 1 + 1e-4)) return rgb.map(clamp01)
    c *= 0.9
  }
  return fromOklch([L, 0, h]).map(clamp01)
}

/* The ladders -------------------------------------------------------------- */

/**
 * `from` mixed towards `to` until it stands `dL` of CIE L* off `from`, or
 * undefined when even `to` itself does not.
 */
function stepOff(from, to, dL) {
  const L0 = lightness(from)
  if (Math.abs(lightness(to) - L0) < dL) return undefined
  let lo = 0
  let hi = 1
  for (let i = 0; i < 40; i++) {
    const t = (lo + hi) / 2
    if (Math.abs(lightness(mix(from, to, t)) - L0) < dL) lo = t
    else hi = t
  }
  return mix(from, to, hi)
}

/** The least alpha of `ink` over `ground` that moves it `dL` of L*. */
function alphaFor(ink, ground, dL) {
  const L0 = lightness(ground)
  let lo = 0
  let hi = 1
  for (let i = 0; i < 40; i++) {
    const a = (lo + hi) / 2
    if (Math.abs(lightness(mix(ground, ink, a)) - L0) < dL) lo = a
    else hi = a
  }
  // Round up to the step a hex alpha can carry, so the emitted value is
  // never the one just short of the floor.
  return Math.ceil(hi * 255) / 255
}

/** The mix of `ink` into `paper`, from `t` up, that clears `min` contrast. */
function legible(paper, ink, t, min) {
  let u = t
  while (u < 1 && contrast(mix(paper, ink, u), paper) < min) u += 0.01
  return Math.min(u, 1)
}

/* Colour ------------------------------------------------------------------- */

/** A hex colour; one with an alpha is composited over `ground` first. */
function parse(value, ground = [1, 1, 1]) {
  const h = value.replace('#', '')
  const rgb = [0, 2, 4].map(i => parseInt(h.slice(i, i + 2), 16) / 255)
  const alpha = h.length === 8 ? parseInt(h.slice(6, 8), 16) / 255 : 1
  return rgb.map((v, i) => v * alpha + ground[i] * (1 - alpha))
}

function hex(rgb, alpha = 1) {
  const byte = v =>
    Math.round(clamp01(v) * 255)
      .toString(16)
      .padStart(2, '0')
  return `#${rgb.map(byte).join('')}${alpha < 1 ? byte(alpha) : ''}`
}

/** Hex precision is what ships, so the ladder is measured at it too. */
function mix(a, b, t) {
  return a.map((v, i) => Math.round((v + (b[i] - v) * t) * 255) / 255)
}

const clamp01 = v => Math.min(1, Math.max(0, v))

const linear = v => (v <= 0.04045 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4)
const gamma = v =>
  v <= 0.0031308
    ? 12.92 * v
    : 1.055 * Math.sign(v) * Math.abs(v) ** (1 / 2.4) - 0.055

function luminance(rgb) {
  const [r, g, b] = rgb.map(linear)
  return 0.2126 * r + 0.7152 * g + 0.0722 * b
}

/** CIE L*, the measure `plugins/contrast.mjs` holds the ladder to. */
function lightness(rgb) {
  const y = luminance(rgb)
  return y > 0.008856 ? 116 * Math.cbrt(y) - 16 : 903.3 * y
}

export function contrast(a, b) {
  const [hi, lo] = [luminance(a), luminance(b)].sort((x, y) => y - x)
  return (hi + 0.05) / (lo + 0.05)
}

function oklch(rgb) {
  const [r, g, b] = rgb.map(linear)
  const l = Math.cbrt(0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b)
  const m = Math.cbrt(0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b)
  const s = Math.cbrt(0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b)
  const L = 0.2104542553 * l + 0.793617785 * m - 0.0040720468 * s
  const A = 1.9779984951 * l - 2.428592205 * m + 0.4505937099 * s
  const B = 0.0259040371 * l + 0.7827717662 * m - 0.808675766 * s
  return [L, Math.hypot(A, B), Math.atan2(B, A)]
}

function fromOklch([L, C, h]) {
  const A = C * Math.cos(h)
  const B = C * Math.sin(h)
  const l = (L + 0.3963377774 * A + 0.2158037573 * B) ** 3
  const m = (L - 0.1055613458 * A - 0.0638541728 * B) ** 3
  const s = (L - 0.0894841775 * A - 1.291485548 * B) ** 3
  return [
    4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
    -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
    -0.0041960863 * l - 0.7034186147 * m + 1.707614701 * s,
  ].map(gamma)
}
