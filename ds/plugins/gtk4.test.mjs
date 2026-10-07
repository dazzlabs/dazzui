import assert from 'node:assert/strict'
import fs from 'node:fs'
import test from 'node:test'
import { color, paint, TINTS, VARIANTS, STATES } from './gtk4.mjs'

const snapshots = JSON.parse(
  fs.readFileSync(
    new URL(
      '../../packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json',
      import.meta.url
    )
  )
)
const css = fs.readFileSync(
  new URL(
    '../../packages/rust/dazzui-gtk4/src/generated/tokens.css',
    import.meta.url
  ),
  'utf8'
)
const components = fs.readFileSync(
  new URL(
    '../../packages/rust/dazzui-gtk4/src/components.css',
    import.meta.url
  ),
  'utf8'
)

test('every committed theme carries the same resolved recipes as the native token snapshot', () => {
  for (const [name, snapshot] of Object.entries(snapshots)) {
    const block = css.split(`.dz-theme-${name} {\n`)[1]?.split('\n}')[0]
    assert.ok(block, name)
    for (const tint of TINTS)
      for (const variant of VARIANTS)
        for (const part of ['surface', 'content', 'border'])
          for (const state of STATES) {
            assert.ok(
              block.includes(
                `--recipe-${tint}-${variant}-${part}-${state}: ${paint(snapshot, `control.color.${variant}.${part}.${state}`, tint)};`
              ),
              `${name}/${tint}/${variant}/${part}/${state}`
            )
          }
    assert.ok(
      block.includes(`--color-surface: ${color(snapshot['color.surface'])};`)
    )
  }
})

test('every variable in the GTK sheets has a declaration', () => {
  const sheets = css + components
  const declarations = new Set(
    [...sheets.matchAll(/(--[a-z0-9-]+):/g)].map(m => m[1])
  )
  for (const [, reference] of sheets.matchAll(/var\((--[a-z0-9-]+)\)/g))
    assert.ok(declarations.has(reference), reference)
})

test('recipe alpha multiplies the source alpha and missing paints fail', () => {
  assert.equal(
    paint({ 'x.color': [128, 1, 2, 3], 'x.alpha': 0.5 }, 'x'),
    'rgba(1, 2, 3, 0.25098)'
  )
  assert.throws(() => paint({}, 'missing'), /Missing GTK paint/)
})
