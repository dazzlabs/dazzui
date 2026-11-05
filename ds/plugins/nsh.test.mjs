import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import test from 'node:test'

test('Native Shell carries the complete shared Rust token representation', () => {
  const read = path => readFileSync(new URL(path, import.meta.url), 'utf8')
  const native = read(
    '../../packages/rust/dazzui-nsh/src/generated/theme_tokens.rs'
  )
  const gpui = read('../../packages/rust/dazzui/src/generated/theme_tokens.rs')
  assert.equal(
    native.replace('ds/plugins/nsh.mjs', 'ds/plugins/gpui.mjs'),
    gpui
  )
  assert.doesNotMatch(native, /use gpui/)
})
