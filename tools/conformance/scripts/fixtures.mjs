/*
 * A made-up run for the viewer to be developed against. The scenes and their
 * addresses are real — the contract codegen writes, `scenes.json` — and
 * everything measured is invented, deterministically, so the viewer has
 * something to show for every platform whether or not its collector exists.
 * Nothing here says anything about any platform.
 */
import { mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

const here = fileURLToPath(new URL('.', import.meta.url))
const root = join(here, '..')
const out = join(root, 'fixtures')

/* A small deterministic generator, seeded per address so a rerun is stable. */
function rng(seed) {
  let h = 2166136261
  for (const c of seed) h = Math.imul(h ^ c.charCodeAt(0), 16777619)
  return () => {
    h += 0x6d2b79f5
    let t = h
    t = Math.imul(t ^ (t >>> 15), t | 1)
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61)
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296
  }
}

const pick = (r, list) => list[Math.floor(r() * list.length)]

const THEMES = {
  'bright-light': {
    surface: '#fbfaf7',
    column: '#f4f3ee',
    ink: '#111c2e',
    border: '#e5e3db',
    accent: '#111c2e',
    onAccent: '#d6ff3f',
  },
  'bright-dark': {
    surface: '#0c141e',
    column: '#060b12',
    ink: '#f2f4ef',
    border: '#1c2734',
    accent: '#d6ff3f',
    onAccent: '#111c2e',
  },
  'studio-light': {
    surface: '#ffffff',
    column: '#f4f4f6',
    ink: '#1c1c1e',
    border: '#d8d8dc',
    accent: '#2f6df6',
    onAccent: '#ffffff',
  },
  'studio-dark': {
    surface: '#1e1e21',
    column: '#17171a',
    ink: '#f2f2f4',
    border: '#37373c',
    accent: '#4f86ff',
    onAccent: '#ffffff',
  },
  'ember-light': {
    surface: '#fffaf6',
    column: '#f7efe8',
    ink: '#2a1d16',
    border: '#e5d6cb',
    accent: '#d9531e',
    onAccent: '#ffffff',
  },
  'ember-dark': {
    surface: '#1f1714',
    column: '#19120f',
    ink: '#f6ece6',
    border: '#3f302a',
    accent: '#f0703a',
    onAccent: '#1f1714',
  },
}

function referenceReading(specimen, theme, r) {
  const t = THEMES[theme]
  const size = { tiny: 20, small: 24, medium: 28, large: 32 }[
    specimen.axes.size ?? pick(r, ['small', 'medium'])
  ]
  const filled =
    specimen.axes.variant === 'filled' || (!specimen.axes.variant && r() > 0.7)
  const outlined = specimen.axes.variant === 'outlined'
  return {
    box: { width: 40 + Math.round(r() * 60), height: size },
    values: {
      corner: size > 24 ? 8 : 6,
      stroke: outlined ? 1 : 0,
      strokeColor: outlined ? t.border : null,
      fill: filled
        ? t.accent
        : outlined || specimen.axes.variant === 'plain'
          ? t.surface
          : t.column,
      ink: filled ? t.onAccent : t.ink,
      fontSize: size > 24 ? 13 : size > 20 ? 12 : 11,
      fontWeight: 500,
      lineHeight: size > 24 ? 18 : 16,
      paddingX: size > 24 ? 12 : 10,
      paddingY: Math.round((size - (size > 24 ? 18 : 16)) / 2),
      shadow: specimen.axes.variant === 'normal' ? 'raised' : null,
    },
  }
}

/* An implementation reads what React read, then drifts a little — and some
   platforms cannot read some things at all. */
const PLATFORMS = {
  flutter: {
    title: 'Flutter',
    font: 'Inter',
    unread: [],
    drift: 0.12,
    miss: 0.02,
  },
  swiftui: {
    title: 'SwiftUI',
    font: 'Inter',
    unread: ['fill', 'ink', 'strokeColor', 'shadow'],
    drift: 0.1,
    miss: 0.04,
  },
  winui: {
    title: 'WinUI',
    font: null,
    unread: ['shadow'],
    drift: 0.2,
    miss: 0.08,
  },
}

function implementationReading(reference, platform, r) {
  const spec = PLATFORMS[platform]
  if (r() < spec.miss) return undefined
  const values = {}
  for (const [key, value] of Object.entries(reference.values)) {
    if (spec.unread.includes(key)) values[key] = null
    else if (value === null) values[key] = null
    else if (r() < spec.drift) {
      if (typeof value === 'number')
        values[key] = value + pick(r, [0.5, 1, -1, 2, 0.25])
      else if (key === 'shadow') values[key] = null
      else values[key] = shade(value, pick(r, [4, 12, 24]))
    } else values[key] = value
  }
  const box = { ...reference.box }
  if (r() < spec.drift) box.width += pick(r, [0.5, 1, -1, 3])
  if (r() < spec.drift / 2) box.height += pick(r, [0.5, 1])
  return { box, values }
}

function shade(hex, by) {
  const n = parseInt(hex.slice(1), 16)
  const c = v =>
    Math.max(0, Math.min(255, v + by))
      .toString(16)
      .padStart(2, '0')
  return `#${c(n >> 16)}${c((n >> 8) & 255)}${c(n & 255)}`
}

/* A scene, drawn as its specimens' boxes in a row: enough to see. */
function sceneSvg(theme, readings) {
  const t = THEMES[theme]
  const gap = 8
  const pad = 16
  let x = pad
  let tallest = 0
  const rects = readings.map(reading => {
    if (!reading) {
      x += 48 + gap
      return ''
    }
    const { width, height } = reading.box
    const v = reading.values
    tallest = Math.max(tallest, height)
    const s =
      `<rect x="${x}" y="${pad}" width="${width}" height="${height}" rx="${v.corner ?? 6}" fill="${v.fill ?? t.column}" stroke="${v.strokeColor ?? 'none'}" stroke-width="${v.stroke ?? 0}"/>` +
      `<rect x="${x + (v.paddingX ?? 10)}" y="${pad + height / 2 - 1.5}" width="${Math.max(8, width - 2 * (v.paddingX ?? 10))}" height="3" rx="1.5" fill="${v.ink ?? t.ink}" opacity="0.85"/>`
    x += width + gap
    return s
  })
  const w = x - gap + pad
  const h = tallest + pad * 2
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${w * 2}" height="${h * 2}" viewBox="0 0 ${w} ${h}"><rect width="${w}" height="${h}" fill="${t.surface}"/>${rects.join('')}</svg>`
}

rmSync(out, { recursive: true, force: true })
mkdirSync(out, { recursive: true })

const scenes = JSON.parse(readFileSync(join(root, 'scenes.json'), 'utf8'))
writeFileSync(join(out, 'scenes.json'), JSON.stringify(scenes, null, 2))

const collected = '2026-09-30T09:41:00Z'
const commit = '3f5f202'
writeFileSync(
  join(out, 'run.json'),
  JSON.stringify(
    { collected, commit, platforms: ['react', ...Object.keys(PLATFORMS)] },
    null,
    2
  )
)
writeFileSync(
  join(out, 'tolerances.json'),
  readFileSync(join(root, 'tolerances.json'))
)

const reference = {
  platform: 'react',
  title: 'React',
  role: 'reference',
  collected,
  commit,
  scale: 2,
  font: 'Inter',
  themes: {},
  scenes: {},
}
const impls = Object.fromEntries(
  Object.entries(PLATFORMS).map(([id, spec]) => [
    id,
    {
      platform: id,
      title: spec.title,
      role: 'implementation',
      collected,
      commit,
      scale: 2,
      font: spec.font,
      themes: {},
      scenes: {},
    },
  ])
)

for (const theme of Object.keys(THEMES)) {
  reference.themes[theme] = {}
  reference.scenes[theme] = {}
  for (const impl of Object.values(impls)) {
    impl.themes[theme] = {}
    impl.scenes[theme] = {}
  }
  for (const story of scenes.stories) {
    for (const scene of story.scenes) {
      const refs = []
      const got = Object.fromEntries(Object.keys(impls).map(id => [id, []]))
      for (const specimen of scene.specimens) {
        const r = rng(`${theme}:${specimen.address}`)
        const ref = referenceReading(specimen, theme, r)
        reference.themes[theme][specimen.address] = ref
        refs.push(ref)
        for (const [id, impl] of Object.entries(impls)) {
          const reading = implementationReading(
            ref,
            id,
            rng(`${id}:${theme}:${specimen.address}`)
          )
          if (reading) impl.themes[theme][specimen.address] = reading
          got[id].push(reading)
        }
      }
      const file = `images/${theme}/${scene.id.replaceAll('/', '-')}.svg`
      const write = (platform, readings) => {
        mkdirSync(join(out, platform, 'images', theme), { recursive: true })
        writeFileSync(join(out, platform, file), sceneSvg(theme, readings))
      }
      write('react', refs)
      reference.scenes[theme][scene.id] = file
      for (const [id, impl] of Object.entries(impls)) {
        write(id, got[id])
        impl.scenes[theme][scene.id] = file
      }
    }
  }
}

writeFileSync(join(out, 'react', 'readings.json'), JSON.stringify(reference))
for (const [id, impl] of Object.entries(impls)) {
  writeFileSync(join(out, id, 'readings.json'), JSON.stringify(impl))
}

console.log(
  `fixtures: ${scenes.stories.length} stories, ${scenes.stories.reduce((n, s) => n + s.scenes.length, 0)} scenes, ${Object.keys(THEMES).length} themes`
)
