import test from 'node:test'
import assert from 'node:assert/strict'
import { requireComplete, PLATFORMS } from '../completeness.mjs'

test('native catalog coverage cannot disguise missing shared declarations', () => {
  assert.throws(
    () =>
      requireComplete([{ id: 'a' }], new Map(), {
        platforms: { flutter: { generated: ['a'] } },
      }),
    /React-only/
  )
})
test('a missing platform renderer fails even when all declarations exist', () => {
  assert.throws(
    () =>
      requireComplete([{ id: 'a' }], new Map([['a', {}]]), {
        platforms: { flutter: { generated: [] } },
      }),
    /flutter: 1 missing/
  )
})
test('all declarations and all target renderers are required', () => {
  assert.doesNotThrow(() =>
    requireComplete([{ id: 'a' }], new Map([['a', {}]]), {
      platforms: Object.fromEntries(
        PLATFORMS.map(p => [p, { generated: ['a'] }])
      ),
    })
  )
})
test('omitting a target from the report cannot pass completion', () => {
  assert.throws(
    () =>
      requireComplete([{ id: 'a' }], new Map([['a', {}]]), { platforms: {} }),
    /android: 1 missing/
  )
})
