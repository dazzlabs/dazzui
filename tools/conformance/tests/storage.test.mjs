import assert from 'node:assert/strict'
import {
  mkdtempSync,
  mkdirSync,
  readFileSync,
  rmSync,
  statSync,
  writeFileSync,
} from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { test } from 'node:test'
import {
  lockRun,
  mergeGaps,
  mergeReadings,
  publishImages,
  selectScenes,
  validateContract,
} from '../scripts/storage.mjs'

const contract = {
  stories: [
    { id: 'a', scenes: [{ id: 'a/preview' }, { id: 'a/0' }] },
    { id: 'ab', scenes: [{ id: 'ab/0' }] },
  ],
}
const reading = value => ({
  box: { width: value, height: 24 },
  values: { corner: 6 },
})
function sample() {
  return {
    platform: 'react',
    title: 'React',
    role: 'reference',
    collected: 'old',
    commit: 'old',
    scale: 2,
    font: null,
    themes: {
      light: {
        'a/preview/0': reading(10),
        'a/preview/1': reading(20),
        'a/0/0': reading(30),
        'ab/0/0': reading(40),
      },
      dark: { 'a/preview/0': reading(50) },
    },
    scenes: {
      light: { 'a/preview': 'old.png', 'a/0': 'keep.png' },
      dark: { 'a/preview': 'dark.png' },
    },
  }
}
const patch = () => ({
  ...sample(),
  collected: 'new',
  commit: 'new',
  themes: { light: { 'a/preview/0': reading(99) } },
  scenes: { light: { 'a/preview': 'new.png' } },
})

test('selectors accept stories and exact scenes without prefix collisions', () => {
  assert.deepEqual(selectScenes(contract, 'a'), ['a/preview', 'a/0'])
  assert.deepEqual(selectScenes(contract, 'a/0,ab/0,a/0'), ['a/0', 'ab/0'])
  assert.throws(() => selectScenes(contract, 'a/nope'), /Unknown scene/)
  assert.throws(() => selectScenes(contract, ''), /Unknown scene/)
})

test('replace a complete scene, retain unrelated themes, scenes and provenance', () => {
  const old = sample()
  const next = mergeReadings(old, patch(), ['a/preview'])
  assert.equal(next.themes.light['a/preview/0'].box.width, 99)
  assert.equal(next.themes.light['a/preview/1'], undefined)
  assert.deepEqual(next.themes.light['a/0/0'], old.themes.light['a/0/0'])
  assert.deepEqual(next.themes.dark, old.themes.dark)
  assert.equal(next.scenes.light['a/0'], 'keep.png')
  assert.deepEqual(next.snapshots.light['a/0'], {
    collected: 'old',
    commit: 'old',
  })
  assert.deepEqual(next.snapshots.light['a/preview'], {
    collected: 'new',
    commit: 'new',
  })
  assert.equal(old.themes.light['a/preview/1'].box.width, 20)
})

test('missing specimens and a no-images update invalidate old selected screenshots', () => {
  const fresh = patch()
  fresh.themes.light = {}
  fresh.scenes.light = {}
  const next = mergeReadings(sample(), fresh, ['a/preview'])
  assert.equal(next.themes.light['a/preview/0'], undefined)
  assert.equal(next.scenes.light['a/preview'], undefined)
  assert.equal(next.scenes.light['a/0'], 'keep.png')
})

test('incompatible hosts and collector scope leakage cannot be merged', () => {
  for (const field of ['platform', 'role', 'scale', 'font']) {
    const fresh = patch()
    fresh[field] = 'different'
    assert.throws(
      () => mergeReadings(sample(), fresh, ['a/preview']),
      /changed/
    )
  }
  assert.throws(
    () => mergeReadings(sample(), sample(), ['a/preview']),
    /unrequested/
  )
})

test('image contents get immutable URLs; identical captures are not rewritten', t => {
  const root = mkdtempSync(join(tmpdir(), 'dazzui-captures-'))
  t.after(() => rmSync(root, { recursive: true, force: true }))
  const staging = join(root, 'staging'),
    destination = join(root, 'data')
  mkdirSync(staging)
  writeFileSync(join(staging, 'new.png'), 'first image')
  const fresh = patch()
  fresh.themes.light['a/preview/0'].image = 'new.png'
  const first = publishImages(fresh, staging, destination)
  const path = first.scenes.light['a/preview']
  assert.equal(first.themes.light['a/preview/0'].image, path)
  const mtime = statSync(join(destination, path)).mtimeMs
  assert.equal(
    publishImages(fresh, staging, destination).scenes.light['a/preview'],
    path
  )
  assert.equal(statSync(join(destination, path)).mtimeMs, mtime)
  writeFileSync(join(staging, 'new.png'), 'updated image')
  const changed = publishImages(fresh, staging, destination)
  assert.notEqual(changed.scenes.light['a/preview'], path)
  assert.equal(readFileSync(join(destination, path), 'utf8'), 'first image')
  fresh.scenes.light['a/preview'] = '../outside.png'
  assert.throws(
    () => publishImages(fresh, staging, destination),
    /outside collector/
  )
})

test('contract changes require all affected scenes and retained platforms', () => {
  const next = structuredClone(contract)
  next.stories[0].scenes[0].specimens = [{ address: 'a/preview/0' }]
  assert.throws(
    () => validateContract(contract, next, ['a/0'], ['react'], ['react']),
    /outside/
  )
  assert.throws(
    () =>
      validateContract(
        contract,
        next,
        ['a/preview'],
        ['react', 'winui'],
        ['react']
      ),
    /also update winui/
  )
  validateContract(
    contract,
    next,
    ['a/preview'],
    ['react', 'winui'],
    ['react', 'winui']
  )
})

test('related verdicts follow updated readings and removed specimens', async () => {
  const { measure } = await import('../src/lib/expectations.ts')
  const reference = sample()
  const implementation = {
    ...sample(),
    platform: 'probe',
    role: 'implementation',
  }
  const tolerances = {
    length: 0.5,
    colour: 0.02,
    kinds: { width: 'length', height: 'length', corner: 'length' },
  }
  assert.equal(measure(reference, implementation, tolerances).failing.size, 0)
  const fresh = { ...patch(), platform: 'probe', role: 'implementation' }
  const next = mergeReadings(implementation, fresh, ['a/preview'])
  assert.deepEqual([...measure(reference, next, tolerances).failing].sort(), [
    'light a/preview/0 width',
    'light a/preview/1 *',
  ])
})

test('concurrent collectors cannot overwrite one output', t => {
  const root = mkdtempSync(join(tmpdir(), 'dazzui-lock-'))
  t.after(() => rmSync(root, { recursive: true, force: true }))
  const release = lockRun(root)
  assert.throws(() => lockRun(root), /Another collection/)
  release()
  lockRun(root)()
})

test('GTK mapping gaps replace only updated scenes', () => {
  assert.deepEqual(
    mergeGaps({ 'a/0': ['old'], 'a/1': ['keep'] }, { 'a/0': ['new'] }, ['a/0']),
    { 'a/0': ['new'], 'a/1': ['keep'] }
  )
  assert.deepEqual(
    mergeGaps({ 'a/0': ['fixed'], 'a/1': ['keep'] }, {}, ['a/0']),
    { 'a/1': ['keep'] }
  )
  assert.throws(() => mergeGaps({}, { 'a/1': [] }, ['a/0']), /Unrequested/)
})
