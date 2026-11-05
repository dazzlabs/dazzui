#!/usr/bin/env node
// Native boxes and pictures, using the same stage and selectors as React.
import { readFileSync, mkdirSync, writeFileSync, rmSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, resolve, join } from 'node:path'
import { spawn, execFileSync } from 'node:child_process'
import { parseArgs, stripVTControlCharacters } from 'node:util'
import { selectScenes } from '../../../../../tools/conformance/scripts/storage.mjs'
import {
  cropNativeImage,
  nativeImageScale,
} from '../../../../../tools/conformance/scripts/native-images.mjs'

const example = resolve(dirname(fileURLToPath(import.meta.url)), '..')
const root = resolve(example, '../../../..')
const platform =
  process.platform === 'darwin'
    ? 'macos'
    : process.platform === 'win32'
      ? 'windows'
      : 'linux'
const { values: options } = parseArgs({
  options: {
    out: {
      type: 'string',
      default: resolve(root, 'tools/conformance/data/nsh'),
    },
    themes: { type: 'string', default: 'drawn' },
    scenes: { type: 'string' },
    'no-images': { type: 'boolean', default: false },
  },
})
const groups = JSON.parse(
  readFileSync(resolve(example, 'src/generated/themes.json'), 'utf8')
)
const names = families =>
  families.flatMap(f => [`${f.value}-light`, `${f.value}-dark`])
const known = names(groups.flatMap(g => g.families))
const themes =
  options.themes === 'drawn'
    ? names(groups[0].families)
    : options.themes === 'all'
      ? known
      : options.themes.split(',')
for (const name of themes)
  if (!known.includes(name)) throw new Error(`Unknown theme: ${name}`)
const contract = JSON.parse(
  readFileSync(resolve(root, 'tools/conformance/scenes.json'), 'utf8')
)
const scenes = selectScenes(contract, options.scenes)
const coverage = JSON.parse(
  readFileSync(resolve(example, 'coverage.json'), 'utf8')
)
for (const scene of scenes)
  if (!coverage.find(s => s.id === scene)?.supported)
    throw new Error(`Unmapped scene: ${scene}`)
const output = resolve(options.out)
mkdirSync(output, { recursive: true })
const raw = join(output, '.native-layout.jsonl')
const snapshots = join(output, '.windows')
const images = !options['no-images']
if (images) mkdirSync(snapshots, { recursive: true })
rmSync(raw, { force: true })
const testFile = resolve(example, '.capture.test.ts')
const pairs = themes.flatMap(theme => scenes.map(scene => ({ theme, scene })))
const collected = new Date().toISOString()
writeFileSync(
  testFile,
  `import { open, test } from 'nsh/test'
test('native conformance collection', async () => {
  const app = await open('.', { width: 1024, height: 768, platform: ${JSON.stringify(platform)}, reducedMotion: true, clock: {} })
  ${!images ? `await app.waitUntil(() => app.texts().includes('Complete capture'), { timeout: ${Math.max(120000, pairs.length * 500)} }); return` : ''}
  const pairs = ${JSON.stringify(pairs)}
  let height = 768
  for (let i = 0; i < pairs.length; i++) {
    const { theme, scene } = pairs[i]
    const prefix = 'Ready ' + theme + ' ' + scene + ' '
    if (i > 0) {
      await app.keyboard.press('ctrl+alt+n')
      for (let attempt = 0; ; attempt++) {
        try {
          await app.waitUntil(() => app.texts().some(text => text.startsWith(prefix)))
          break
        } catch (error) {
          // A desktop can lose a synthetic key when focus changes. Retry only
          // if the prior stage is still ready: never skip a preparing stage.
          const previous = pairs[i - 1]
          const stillPrevious = app.texts().some(text => text.startsWith('Ready ' + previous.theme + ' ' + previous.scene + ' '))
          if (!stillPrevious || attempt >= 2) throw error
          await app.press('label:Next capture')
        }
      }
    } else {
      await app.waitUntil(() => app.texts().some(text => text.startsWith(prefix)))
    }
    const stageHeight = Number(app.texts().find(text => text.startsWith(prefix)).slice(prefix.length))
    const nextHeight = Math.max(768, Math.ceil(stageHeight + 48))
    if (height !== nextHeight) { await app.setViewport(1024, nextHeight); height = nextHeight }
    await app.hover('label:Next capture')
    ${images ? `await app.screenshot(${JSON.stringify(snapshots)} + '/' + i + '.png')` : ''}
    if (i + 1 === pairs.length || pairs[i + 1].theme !== theme) console.log('nsh: ' + theme)
  }
}, ${Math.max(120000, pairs.length * 5000)})
`
)
try {
  const run = spawn('nsh', ['test', testFile, '--target', platform], {
    cwd: example,
    stdio: ['inherit', 'pipe', 'pipe'],
    env: {
      ...process.env,
      CARGO_TARGET_DIR: resolve(example, '../target'),
      DAZZUI_NSH_CAPTURE: raw,
      ...(!images ? { DAZZUI_NSH_AUTO_CAPTURE: '1' } : {}),
      DAZZUI_NSH_THEMES: JSON.stringify(themes),
      DAZZUI_NSH_SCENES: JSON.stringify(scenes),
    },
  })
  let log = ''
  for (const [stream, output] of [
    [run.stdout, process.stdout],
    [run.stderr, process.stderr],
  ])
    stream.on('data', chunk => {
      log += chunk.toString()
      output.write(chunk)
    })
  const status = await new Promise((resolve, reject) => {
    run.once('error', reject)
    run.once('close', resolve)
  })
  if (status !== 0) throw new Error(`NSH native capture failed (${status})`)
  if (/^error\b/m.test(stripVTControlCharacters(log)))
    throw new Error('NSH native capture logged a runtime error')
  const rows = readFileSync(raw, 'utf8')
    .trim()
    .split('\n')
    .map(line => JSON.parse(line))
  if (rows.length !== pairs.length)
    throw new Error(`Measured ${rows.length} of ${pairs.length} stages`)
  const scale = rows[0].scale
  if (!Number.isFinite(scale) || scale <= 0)
    throw new Error('Invalid native display scale')
  const readings = {
    platform: 'nsh',
    title: 'Native Shell',
    role: 'implementation',
    collected,
    commit:
      process.env.DAZZUI_CONFORMANCE_COMMIT ??
      execFileSync('git', ['rev-parse', '--short', 'HEAD'], {
        cwd: root,
        encoding: 'utf8',
      }).trim(),
    scale,
    font: null,
    themes: {},
    scenes: {},
  }
  for (let index = 0; index < pairs.length; index++) {
    const { theme, scene } = pairs[index]
    const row = rows[index]
    if (row.scale !== scale)
      throw new Error('Native display scale changed during collection')
    if (row.theme !== theme || row.scene !== scene)
      throw new Error(`Out-of-order native stage ${index}`)
    const entries = (readings.themes[theme] ??= {})
    const sceneImages = (readings.scenes[theme] ??= {})
    const png = join(snapshots, index + '.png')
    if (images) {
      if ((await nativeImageScale(png, 1024)) !== scale)
        throw new Error('Native display scale changed during collection')
      const path = `images/${theme}/${scene.replaceAll('/', '-')}.png`
      if (!(await cropNativeImage(png, join(output, path), row.stage, scale)))
        throw new Error(`Empty native stage ${scene}`)
      sceneImages[scene] = path
    }
    for (const [address, rect] of Object.entries(row.frames)) {
      if (!address.startsWith(scene + '/'))
        throw new Error(`Wrong native address: ${address}`)
      // NSH exposes actual layout; paint and text metrics remain unread.
      const entry = {
        box: { width: rect.width, height: rect.height },
        values: {},
      }
      if (images) {
        const path = `images/${theme}/${address.replaceAll('/', '-')}.png`
        if (await cropNativeImage(png, join(output, path), rect, scale))
          entry.image = path
      }
      entries[address] = entry
    }
  }
  writeFileSync(
    join(output, 'readings.json'),
    JSON.stringify(readings, null, 2) + '\n'
  )
  console.log(
    `nsh: collected ${scenes.length} scenes × ${themes.length} themes`
  )
} finally {
  rmSync(testFile, { force: true })
  rmSync(raw, { force: true })
  rmSync(snapshots, { recursive: true, force: true })
}
