import assert from 'node:assert/strict'
import fs from 'node:fs'
import test from 'node:test'
import { generate } from './runtime.mjs'

const themes = JSON.parse(
  fs.readFileSync(
    new URL(
      '../../packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json',
      import.meta.url
    )
  )
)
test('React runtime constants reproduce shared theme-invariant durations', () => {
  const actual = fs.readFileSync(
    new URL(
      '../../packages/js/dazzui/src/generated/runtime.ts',
      import.meta.url
    ),
    'utf8'
  )
  assert.equal(actual, generate(themes))
})

test('runtime constants reject theme-dependent or invalid durations', () => {
  const first = themes['studio-light']
  assert.throws(() => generate({}), /require a theme/)
  assert.throws(
    () => generate({ a: { ...first, 'toast.notice-duration': 5000 } }),
    /Invalid runtime duration/
  )
  assert.throws(
    () =>
      generate({
        a: first,
        b: { ...first, 'toast.action-duration': { value: 2000, unit: 'ms' } },
      }),
    /must not vary by theme/
  )
})

test('runtime steps reject invalid and theme-dependent numbers', () => {
  const first = themes['studio-light']
  assert.throws(
    () => generate({ a: { ...first, 'splitter.key-step': 0 } }),
    /Invalid runtime number/
  )
  assert.throws(
    () => generate({ a: first, b: { ...first, 'splitter.key-step': 2 } }),
    /must not vary by theme/
  )
})
