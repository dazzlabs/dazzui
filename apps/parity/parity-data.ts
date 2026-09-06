/*
 * The dev-time half of the viewer: read the parity data off the disk and
 * hand it to the page as one module.
 *
 * Three things are read, all of them the outputs of other tools:
 *
 *   - the React reference — `stories.json` and `reference/*.json`, from the
 *     Storybook's staging directory when the extractor has run here, or
 *     from the copy distributed into the Flutter playground otherwise (the
 *     three copies are identical);
 *   - each platform's `drift.txt`, the checks its reference test names as
 *     failing today;
 *   - each platform's `measured.json`, what that test measured on its side,
 *     which only exists once the test has run.
 *
 * The reference is folded into specimens here, the same way the platform
 * tests fold it — a class and its axes, text left out of the key, a value
 * kept only where every appearance agrees — so the viewer and the tests are
 * talking about the same things.
 */

import fs from 'node:fs'
import path from 'node:path'
import type { Plugin } from 'vite'

import {
  FIELDS,
  type Field,
  type ParityData,
  type Platform,
  type PlatformId,
  type Rect,
  type Specimen,
  type ThemeId,
  type Value,
} from './src/lib/model'

const ROOT = path.resolve(import.meta.dirname, '..', '..')
const STAGING = path.join(ROOT, 'apps', 'storybook', 'parity')

const PLATFORMS: { id: PlatformId; label: string; dir: string }[] = [
  {
    id: 'flutter',
    label: 'Flutter',
    dir: 'packages/ui_flutter/example/test/parity',
  },
  {
    id: 'swiftui',
    label: 'SwiftUI',
    dir: 'packages/ui_swiftui/Example/Tests/PlaygroundTests/Parity',
  },
  { id: 'gpui', label: 'GPUI', dir: 'packages/ui_gpui/tests/parity' },
]

/** The computed style the extractor recorded, by the field it answers. */
const STYLE_KEYS: Record<Field, string | null> = {
  width: null,
  height: null,
  background: 'backgroundColor',
  'border-color': 'borderTopColor',
  'border-width': 'borderTopWidth',
  radius: 'borderTopLeftRadius',
  color: 'color',
  'font-size': 'fontSize',
  'font-weight': 'fontWeight',
}

interface RawProbe {
  class: string
  data: Record<string, string>
  text: string
  rect: Rect
  style: Record<string, number | string>
}

function readJson<T>(file: string): T {
  return JSON.parse(fs.readFileSync(file, 'utf8')) as T
}

function readList(file: string): string[] {
  if (!fs.existsSync(file)) return []
  return fs
    .readFileSync(file, 'utf8')
    .split('\n')
    .map(line => line.trim())
    .filter(line => line !== '' && !line.startsWith('#'))
}

function valueOf(probe: RawProbe, field: Field): Value {
  if (field === 'width') return probe.rect.width
  if (field === 'height') return probe.rect.height
  const raw = probe.style[STYLE_KEYS[field]!]
  if (raw === undefined) return null
  if (field === 'font-weight') {
    const weight = Number.parseInt(String(raw), 10)
    return Number.isNaN(weight) ? null : weight
  }
  return raw
}

/** The value every appearance agrees on, or null. */
function agreed(probes: RawProbe[], field: Field): Value {
  const first = valueOf(probes[0], field)
  if (first == null) return null
  for (const probe of probes.slice(1)) {
    const other = valueOf(probe, field)
    if (typeof first === 'number' && typeof other === 'number') {
      if (Math.abs(first - other) > 0.5) return null
    } else if (other !== first) {
      return null
    }
  }
  return first
}

function keyOf(probe: RawProbe): string {
  const axes = Object.keys(probe.data)
    .sort()
    .map(axis => `${axis}=${probe.data[axis]}`)
  return axes.length === 0 ? probe.class : `${probe.class} ${axes.join(' ')}`
}

export function loadParity(): ParityData {
  const referenceDir = fs.existsSync(path.join(STAGING, 'reference'))
    ? STAGING
    : path.join(ROOT, PLATFORMS[0].dir)

  const stories = readJson<{ viewport: ParityData['viewport'] }>(
    path.join(referenceDir, 'stories.json')
  )

  const seen = new Map<
    string,
    {
      probe: RawProbe
      story: string
      rects: Partial<Record<ThemeId, Rect>>
      themes: Partial<Record<ThemeId, RawProbe[]>>
    }
  >()

  const pages = fs
    .readdirSync(path.join(referenceDir, 'reference'))
    .filter(file => file.endsWith('.json'))
    .sort()
  for (const file of pages) {
    const page = readJson<{
      stories: Record<
        string,
        { themes: Record<ThemeId, { probes: RawProbe[] }> }
      >
    }>(path.join(referenceDir, 'reference', file))
    for (const [storyId, story] of Object.entries(page.stories)) {
      for (const [theme, drawn] of Object.entries(story.themes) as [
        ThemeId,
        { probes: RawProbe[] },
      ][]) {
        for (const probe of drawn.probes) {
          const key = keyOf(probe)
          let entry = seen.get(key)
          if (!entry) {
            entry = { probe, story: storyId, rects: {}, themes: {} }
            seen.set(key, entry)
          }
          if (entry.story === storyId && !entry.rects[theme]) {
            entry.rects[theme] = probe.rect
          }
          ;(entry.themes[theme] ??= []).push(probe)
        }
      }
    }
  }

  const specimens: Specimen[] = [...seen.entries()]
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([key, entry]) => {
      const expected: Specimen['expected'] = {}
      for (const [theme, probes] of Object.entries(entry.themes) as [
        ThemeId,
        RawProbe[],
      ][]) {
        const values: Partial<Record<Field, Value>> = {}
        for (const field of FIELDS) values[field] = agreed(probes, field)
        expected[theme] = values
      }
      return {
        key,
        className: entry.probe.class,
        data: entry.probe.data,
        text: entry.probe.text,
        sample: { story: entry.story, rects: entry.rects },
        expected,
      }
    })

  const platforms: Platform[] = PLATFORMS.map(platform => {
    const dir = path.join(ROOT, platform.dir)
    const measuredFile = path.join(dir, 'measured.json')
    return {
      ...platform,
      drift: readList(path.join(dir, 'drift.txt')),
      measured: fs.existsSync(measuredFile)
        ? readJson<{ specimens: Platform['measured'] }>(measuredFile).specimens
        : null,
    }
  })

  return {
    viewport: stories.viewport,
    screenshots: fs.existsSync(path.join(STAGING, 'screenshots')),
    specimens,
    platforms,
  }
}

const MODULE = 'virtual:parity'
const RESOLVED = '\0virtual:parity'

/**
 * `virtual:parity` is the data as a module, and `/screenshots/<theme>/<id>.png`
 * is the extractor's staging directory, served as it is. Both are read when
 * asked for, so rerunning a platform test and reloading the page is enough.
 */
export function parityData(): Plugin {
  return {
    name: 'parity-data',
    resolveId(id) {
      return id === MODULE ? RESOLVED : undefined
    },
    load(id) {
      if (id !== RESOLVED) return undefined
      return `export default ${JSON.stringify(loadParity())}`
    },
    configureServer(server) {
      // The data is another tool's output: a platform test rewriting its
      // measured.json, or a drift list edited by hand, is a reason to read
      // everything again and reload the page.
      const watched = [
        path.join(STAGING, 'stories.json'),
        path.join(STAGING, 'reference'),
        ...PLATFORMS.flatMap(platform => [
          path.join(ROOT, platform.dir, 'drift.txt'),
          path.join(ROOT, platform.dir, 'measured.json'),
          path.join(ROOT, platform.dir, 'reference'),
        ]),
      ]
      server.watcher.add(watched)
      const refresh = (file: string) => {
        if (
          !watched.some(w => file === w || file.startsWith(`${w}${path.sep}`))
        )
          return
        const module = server.moduleGraph.getModuleById(RESOLVED)
        if (module) server.moduleGraph.invalidateModule(module)
        server.ws.send({ type: 'full-reload' })
      }
      server.watcher.on('change', refresh)
      server.watcher.on('add', refresh)
      server.watcher.on('unlink', refresh)

      server.middlewares.use('/screenshots', (req, res, next) => {
        const file = path.join(
          STAGING,
          'screenshots',
          decodeURIComponent(req.url ?? '')
        )
        if (
          !file.startsWith(path.join(STAGING, 'screenshots')) ||
          !fs.existsSync(file)
        ) {
          next()
          return
        }
        res.setHeader('content-type', 'image/png')
        fs.createReadStream(file).pipe(res)
      })
    },
  }
}
