import assert from 'node:assert/strict'
import { test } from 'node:test'
import { load } from '../src/lib/data.ts'
import type { Readings, Run } from '../src/lib/model.ts'

const reading = (platform: string): Readings => ({
  platform,
  title: platform,
  role: platform === 'react' ? 'reference' : 'implementation',
  collected: 'before',
  commit: 'old',
  scale: 2,
  font: null,
  themes: {},
  scenes: {},
})

test('an update reloads changed platforms while retaining unchanged platform data', async t => {
  const calls: string[] = []
  const run: Run = {
    collected: 'before',
    commit: 'old',
    revision: '1',
    platforms: ['react', 'probe'],
    snapshots: {
      react: { collected: 'before', commit: 'old' },
      probe: { collected: 'before', commit: 'old' },
    },
  }
  const files: Record<string, unknown> = {
    '/run.json': run,
    '/scenes.json': { stories: [] },
    '/tolerances.json': {},
    '/react/readings.json': reading('react'),
    '/probe/readings.json': reading('probe'),
  }
  t.mock.method(globalThis, 'fetch', async (path: string) => {
    calls.push(path)
    return new Response(JSON.stringify(files[path]))
  })
  const before = await load()
  files['/run.json'] = {
    ...run,
    revision: '2',
    snapshots: {
      ...run.snapshots,
      probe: { collected: 'after', commit: 'new' },
    },
  }
  files['/probe/readings.json'] = { ...reading('probe'), collected: 'after' }
  calls.length = 0
  const after = await load(before)
  assert.equal(after.readings.react, before.readings.react)
  assert.notEqual(after.readings.probe, before.readings.probe)
  assert(!calls.includes('/react/readings.json'))
  assert(calls.includes('/probe/readings.json'))
})

test('a publication during loading cannot produce a mixed run', async t => {
  let revision = 0
  t.mock.method(
    globalThis,
    'fetch',
    async (path: string) =>
      new Response(
        JSON.stringify(
          path === '/run.json'
            ? { collected: '', platforms: [], revision: String(++revision) }
            : {}
        )
      )
  )
  await assert.rejects(load(), /changed while loading/)
})
