/*
 * The React collector: draws every scene in scenes.json in headless
 * Chromium, at 2x with motion off, in every theme asked for, and writes the
 * reference readings — `readings.json`, the scene images and a crop per
 * specimen. See specs/conformance.md.
 *
 *   node scripts/conformance.mjs [--out <dir>] [--themes drawn|all|<id,…>]
 *                                [--scenes <story,…>] [--no-images]
 *
 * The kit must be built (`pnpm packages`); the example is served from source.
 */
import { execFileSync } from 'node:child_process'
import { mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs'
import { join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import { parseArgs } from 'node:util'
import { chromium } from 'playwright'
import { createServer } from 'vite'

const example = fileURLToPath(new URL('..', import.meta.url))
const repo = resolve(example, '../../../..')
const SCALE = 2

const { values: args } = parseArgs({
  options: {
    out: {
      type: 'string',
      default: join(repo, 'tools/conformance/data/react'),
    },
    themes: { type: 'string', default: 'drawn' },
    scenes: { type: 'string' },
    'no-images': { type: 'boolean', default: false },
  },
})
const images = !args['no-images']
const out = resolve(args.out)

const contract = JSON.parse(
  readFileSync(join(repo, 'tools/conformance/scenes.json'), 'utf8')
)
const only = args.scenes?.split(',')
const scenes = contract.stories
  .filter(story => !only || only.includes(story.id))
  .flatMap(story => story.scenes)

const server = await createServer({
  root: example,
  logLevel: 'warn',
  server: { host: '127.0.0.1', port: 0 },
})
await server.listen()
const browser = await chromium.launch()
const failures = []

try {
  const page = await browser.newPage({
    deviceScaleFactor: SCALE,
    viewport: { width: 1024, height: 768 },
    reducedMotion: 'reduce',
  })
  page.on('pageerror', error => failures.push(`page: ${error.message}`))
  await page.goto(
    new URL('conformance.html', server.resolvedUrls.local[0]).href
  )
  await page.waitForFunction(() => window.conformance)
  const known = await page.evaluate(() => ({
    scenes: window.conformance.scenes,
    themes: window.conformance.themes,
    drawn: window.conformance.drawn,
  }))
  const themes =
    args.themes === 'drawn'
      ? known.drawn
      : args.themes === 'all'
        ? known.themes
        : args.themes.split(',')
  for (const theme of themes)
    if (!known.themes.includes(theme)) throw new Error(`No theme ${theme}`)

  rmSync(out, { recursive: true, force: true })
  mkdirSync(out, { recursive: true })
  const readings = {
    platform: 'react',
    title: 'React',
    role: 'reference',
    collected: new Date().toISOString(),
    commit: execFileSync('git', ['rev-parse', '--short', 'HEAD'], {
      cwd: repo,
      encoding: 'utf8',
    }).trim(),
    scale: SCALE,
    // The kit sets type in the system face; no test face is embedded, so
    // no crop is compared as a picture.
    font: null,
    themes: {},
    scenes: {},
  }

  for (const theme of themes) {
    const readingsOf = (readings.themes[theme] = {})
    const scenesOf = (readings.scenes[theme] = {})
    if (images) mkdirSync(join(out, 'images', theme), { recursive: true })
    for (const scene of scenes) {
      if (!known.scenes.includes(scene.id)) {
        failures.push(`${scene.id}: the example has no such scene`)
        continue
      }
      const drawn = await page.evaluate(
        ([id, theme]) => window.conformance.show(id, theme),
        [scene.id, theme]
      )
      // The walk here and codegen's must agree, or every address is wrong.
      const want = scene.specimens.map(s => s.component)
      const got = drawn.specimens.map(s => kebab(s.component))
      if (want.join() !== got.join()) {
        failures.push(
          `${scene.id}: codegen addressed ${want.join(' ') || 'nothing'}, the page drew ${got.join(' ') || 'nothing'}`
        )
        continue
      }
      const file = scene.id.replaceAll('/', '-')
      let crops = []
      if (images) {
        const png = await page.screenshot({ clip: drawn.rect, fullPage: true })
        writeFileSync(join(out, 'images', theme, `${file}.png`), png)
        scenesOf[scene.id] = `images/${theme}/${file}.png`
        crops = await page.evaluate(
          ([png, rects, scale]) => window.conformance.crop(png, rects, scale),
          [
            png.toString('base64'),
            drawn.specimens.map(
              ({ rect }) =>
                rect && {
                  ...rect,
                  x: rect.x - drawn.rect.x,
                  y: rect.y - drawn.rect.y,
                }
            ),
            SCALE,
          ]
        )
      }
      drawn.specimens.forEach(({ reading }, index) => {
        if (!reading) return
        const address = scene.specimens[index].address
        const entry = { ...reading }
        if (crops[index]) {
          const name = `images/${theme}/${file}-${index}.png`
          writeFileSync(join(out, name), Buffer.from(crops[index], 'base64'))
          entry.image = name
        }
        readingsOf[address] = entry
      })
    }
    console.log(`react: ${theme}`)
  }
  writeFileSync(join(out, 'readings.json'), JSON.stringify(readings))
} finally {
  await browser.close()
  await server.close()
}

if (failures.length) {
  console.error(failures.join('\n'))
  process.exitCode = 1
}

function kebab(name) {
  return name.replace(/([a-z0-9])([A-Z])/g, '$1-$2').toLowerCase()
}
