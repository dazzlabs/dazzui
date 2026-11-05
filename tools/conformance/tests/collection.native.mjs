import assert from 'node:assert/strict'
import { spawnSync } from 'node:child_process'
import { createHash } from 'node:crypto'
import {
  existsSync,
  mkdtempSync,
  readFileSync,
  readdirSync,
  rmSync,
} from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import sharp from 'sharp'

const collector = fileURLToPath(
  new URL('../scripts/collect.mjs', import.meta.url)
)
const json = path => JSON.parse(readFileSync(path, 'utf8'))
const hash = path =>
  createHash('sha256').update(readFileSync(path)).digest('hex')

// Explicit opt-in: this test opens real desktop windows through the NSH CLI.
test('NSH native collection publishes crops, merges selected scenes and retains a failed run', async t => {
  const out = mkdtempSync(join(tmpdir(), 'dazzui-native-'))
  t.after(() => rmSync(out, { recursive: true, force: true }))
  const collect = (...args) =>
    spawnSync(
      process.execPath,
      [collector, '--out', out, '--platforms', 'react,nsh', ...args],
      { encoding: 'utf8', timeout: 180000 }
    )
  const initial = collect(
    '--themes',
    'bright-light,studio-dark',
    '--scenes',
    'button/preview,avatar/preview'
  )
  assert.equal(initial.status, 0, initial.stdout + initial.stderr)
  const path = join(out, 'nsh/readings.json')
  const before = json(path)
  assert.equal(before.platform, 'nsh')
  assert.equal(before.font, null)
  const specimen = before.themes['bright-light']['button/preview/0']
  assert.equal(specimen.box.height, 26)
  assert.deepEqual(specimen.values, {})
  const image = await sharp(join(out, 'nsh', specimen.image)).metadata()
  assert.equal(image.width, Math.round(specimen.box.width * before.scale))
  assert.equal(image.height, Math.round(specimen.box.height * before.scale))
  const scene = await sharp(
    join(out, 'nsh', before.scenes['bright-light']['button/preview'])
  ).metadata()
  assert.equal(scene.width, 640 * before.scale)
  const retained = join(
    out,
    'nsh',
    before.scenes['studio-dark']['avatar/preview']
  )
  assert(existsSync(retained))
  const retainedHash = hash(retained)
  const updated = collect(
    '--incremental',
    '--themes',
    'bright-light',
    '--scenes',
    'button/preview',
    '--no-images'
  )
  assert.equal(updated.status, 0, updated.stdout + updated.stderr)
  const after = json(path)
  assert.deepEqual(after.themes['studio-dark'], before.themes['studio-dark'])
  assert.deepEqual(
    after.themes['bright-light']['avatar/preview/0'],
    before.themes['bright-light']['avatar/preview/0']
  )
  assert.equal(
    after.themes['bright-light']['button/preview/0'].image,
    undefined
  )
  assert.equal(after.scenes['bright-light']['button/preview'], undefined)
  assert.equal(hash(retained), retainedHash)
  const runHash = hash(join(out, 'run.json')),
    readingsHash = hash(path)
  const failed = collect(
    '--incremental',
    '--themes',
    'does-not-exist',
    '--scenes',
    'button/preview'
  )
  assert.notEqual(failed.status, 0)
  assert.equal(hash(join(out, 'run.json')), runHash)
  assert.equal(hash(path), readingsHash)
  assert(
    !readdirSync(out).some(
      name => name.startsWith('.pending-') || name === '.collect.lock'
    )
  )
})
