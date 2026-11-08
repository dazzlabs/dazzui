/*
 * The React collector: draws every scene in scenes.json in headless
 * Chromium, at 2x with motion off, in every theme asked for, and writes the
 * reference readings — `readings.json`, the scene images and a crop per
 * specimen. See specs/conformance.md.
 *
 *   node scripts/conformance.mjs [--out <dir>] [--themes drawn|all|<id,…>]
 *                                [--scenes <story,…>] [--no-images]
 *                                [--container-values]
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
    contract: {
      type: 'string',
      default: join(repo, 'tools/conformance/scenes.json'),
    },
    'no-images': { type: 'boolean', default: false },
    'container-values': { type: 'boolean', default: false },
    // Accepted for older callers; positions are part of ordinary collection.
    positions: { type: 'boolean' },
    'motion-ms': { type: 'string' },
    'reduced-motion': { type: 'boolean', default: false },
    modal: { type: 'string' },
    'modal-index': { type: 'string', default: '0' },
  },
})
const images = !args['no-images']
const motionTime =
  args['motion-ms'] === undefined ? 0 : Number(args['motion-ms'])
if (!Number.isFinite(motionTime) || motionTime < 0)
  throw new Error('--motion-ms must be nonnegative and finite')
if (args.modal && !['open', 'close'].includes(args.modal))
  throw new Error('--modal must be open or close')
const modalIndex = Number(args['modal-index'])
if (!Number.isInteger(modalIndex) || modalIndex < 0)
  throw new Error('--modal-index must be an unsigned integer')
const reducedMotion =
  (args['motion-ms'] === undefined && !args.modal) || args['reduced-motion']
const out = resolve(args.out)

const contract = JSON.parse(readFileSync(resolve(args.contract), 'utf8'))
const only = args.scenes?.split(',')
const scenes = contract.stories
  .flatMap(story => story.scenes)
  .filter(
    scene =>
      !only || only.some(id => scene.id === id || scene.id.startsWith(`${id}/`))
  )

const server = await createServer({
  root: example,
  logLevel: 'warn',
  server: { host: '127.0.0.1', port: 0 },
})
await server.listen()
const browser = await chromium.launch({
  executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE || undefined,
})
const failures = []

try {
  const page = await browser.newPage({
    // Scene labels and native example defaults are English. Keep intrinsic
    // date widths independent of the machine's browser language.
    locale: 'en-US',
    deviceScaleFactor: SCALE,
    viewport: { width: 1024, height: 768 },
    reducedMotion: reducedMotion ? 'reduce' : 'no-preference',
  })
  page.on('pageerror', error => failures.push(`page: ${error.message}`))
  await page.goto(
    new URL('conformance.html', server.resolvedUrls.local[0]).href
  )
  await page.waitForFunction(() => window.conformance)
  if (args.modal) {
    await page.addStyleTag({
      content: `
      .dz-dialog,.dz-dialog-scrim,.dz-drawer,.dz-drawer-scrim {
        transition: opacity var(--motion-duration) var(--motion-easing),
          transform var(--motion-duration) var(--motion-easing) !important;
      }
    `,
    })
  }
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
    commit: process.env.DAZZUI_CONFORMANCE_COMMIT ?? commitOf(repo),
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
      let drawn = await page.evaluate(
        ([id, theme]) => window.conformance.show(id, theme),
        [scene.id, theme]
      )
      if (args.modal) {
        const target = page.locator('[aria-haspopup="dialog"]').nth(modalIndex)
        await target.click()
        if (!reducedMotion) {
          await page.waitForFunction(() =>
            document
              .getAnimations()
              .some(
                animation =>
                  animation instanceof CSSTransition &&
                  animation.effect.target.matches('.dz-dialog,.dz-drawer')
              )
          )
        }
        if (args.modal === 'close') {
          await page.evaluate(() => {
            for (const animation of document.getAnimations()) {
              if (
                animation instanceof CSSTransition &&
                animation.effect.target.matches(
                  '.dz-dialog,.dz-dialog-scrim,.dz-drawer,.dz-drawer-scrim'
                )
              )
                animation.finish()
            }
          })
          await page.keyboard.press('Escape')
          if (!reducedMotion) {
            await page.waitForFunction(() =>
              document
                .getAnimations()
                .some(
                  animation =>
                    animation instanceof CSSTransition &&
                    animation.playState !== 'finished' &&
                    animation.effect.target.matches('.dz-dialog,.dz-drawer')
                )
            )
          }
        }
        await page.evaluate(time => {
          for (const animation of document.getAnimations()) {
            animation.pause()
            animation.currentTime = time
          }
        }, motionTime)
        await page.evaluate(
          () => new Promise(resolve => requestAnimationFrame(resolve))
        )
        drawn = await page.evaluate(() => window.conformance.read())
        drawn.rect = { x: 0, y: 0, width: 1024, height: 768 }
      }
      await page.evaluate(time => {
        for (const animation of document.getAnimations()) {
          animation.pause()
          animation.currentTime = time
        }
      }, motionTime)
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
        // The contract compares a container's box only; its children are
        // independent specimens with their own decoration and typography.
        if (scene.specimens[index].container && !args['container-values'])
          entry.values = {}
        const rect = drawn.specimens[index].rect
        if (rect) {
          entry.values = {
            ...entry.values,
            left: Math.round((rect.x - drawn.rect.x) * 100) / 100,
            top: Math.round((rect.y - drawn.rect.y) * 100) / 100,
          }
        }
        if (typeof entry.values.baseline === 'number')
          entry.values.baseline =
            Math.round((entry.values.baseline - drawn.rect.y) * 100) / 100
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

function commitOf(repo) {
  try {
    return execFileSync('git', ['rev-parse', '--short', 'HEAD'], {
      cwd: repo,
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'ignore'],
    }).trim()
  } catch {
    return 'unknown'
  }
}
