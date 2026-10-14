import assert from 'node:assert/strict'
import { spawnSync } from 'node:child_process'
import { createHash } from 'node:crypto'
import {
  existsSync,
  mkdtempSync,
  readFileSync,
  readdirSync,
  rmSync,
  statSync,
} from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'

const collector = fileURLToPath(
  new URL('../scripts/collect.mjs', import.meta.url)
)
const json = path => JSON.parse(readFileSync(path, 'utf8'))
const hash = path =>
  createHash('sha256').update(readFileSync(path)).digest('hex')

test('real screenshots update one scene/theme and a failed capture retains the run', t => {
  const out = mkdtempSync(join(tmpdir(), 'dazzui-incremental-'))
  t.after(() => rmSync(out, { recursive: true, force: true }))
  const collect = (...args) =>
    spawnSync(
      process.execPath,
      [collector, '--out', out, '--platforms', 'react', ...args],
      { encoding: 'utf8', timeout: 120000 }
    )
  const initial = collect(
    '--themes',
    'studio-light,bright-dark',
    '--scenes',
    'table/preview,badge/preview'
  )
  assert.equal(initial.status, 0, initial.stderr)
  const path = join(out, 'react/readings.json')
  const before = json(path),
    oldRun = json(join(out, 'run.json'))
  const retained = before.scenes['bright-dark']['badge/preview']
  const file = join(out, 'react', retained)
  assert(existsSync(file))
  const bytes = hash(file),
    mtime = statSync(file).mtimeMs
  const updated = collect(
    '--incremental',
    '--themes',
    'studio-light',
    '--scenes',
    'table/preview'
  )
  assert.equal(updated.status, 0, updated.stderr)
  const after = json(path)
  assert.deepEqual(after.themes['bright-dark'], before.themes['bright-dark'])
  assert.deepEqual(after.scenes['bright-dark'], before.scenes['bright-dark'])
  for (const [address, value] of Object.entries(before.themes['studio-light']))
    if (address.startsWith('badge/'))
      assert.deepEqual(after.themes['studio-light'][address], value)
  assert.equal(hash(file), bytes)
  assert.equal(statSync(file).mtimeMs, mtime)
  assert.deepEqual(
    after.snapshots['bright-dark']['badge/preview'],
    before.snapshots['bright-dark']['badge/preview']
  )
  assert.notEqual(
    after.snapshots['studio-light']['table/preview'].collected,
    before.snapshots['studio-light']['table/preview'].collected
  )
  assert.notEqual(json(join(out, 'run.json')).revision, oldRun.revision)
  const runHash = hash(join(out, 'run.json')),
    readingsHash = hash(path)
  const failed = collect(
    '--incremental',
    '--themes',
    'does-not-exist',
    '--scenes',
    'table/preview'
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
