import assert from 'node:assert/strict'
import fs from 'node:fs'
import test from 'node:test'
import { color, dimension, generate, snapshot, themeData } from './harmony.mjs'

const snapshots = JSON.parse(
  fs.readFileSync(
    new URL(
      '../../packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json',
      import.meta.url
    )
  )
)
const generated = fs.readFileSync(
  new URL(
    '../../packages/arkts/dazzui/src/main/ets/generated/ThemeData.ets',
    import.meta.url
  ),
  'utf8'
)

test('HarmonyOS output is reproducible from the shared resolved graph', () => {
  assert.equal(generated, generate(snapshots))
})

test('every theme override reconstructs its complete native snapshot', () => {
  const { base, overrides } = themeData(snapshots)
  for (const [name, values] of Object.entries(snapshots)) {
    const expected = snapshot(values)
    for (const group of Object.keys(base))
      assert.deepEqual(
        { ...base[group], ...overrides[name][group] },
        expected[group],
        `${name}/${group}`
      )
  }
  assert.ok(Object.keys(overrides).length >= 12)
})

test('ARGB conversion multiplies source alpha and dimensions preserve logical pixels', () => {
  assert.equal(color([128, 1, 2, 3], 0.5), '#40010203')
  assert.equal(color([0, 0, 0, 0]), '#00000000')
  assert.equal(dimension({ unit: 'rem', value: 0.09375 }), 1.5)
  assert.equal(dimension({ unit: 'ms', value: 150 }), 150)
  assert.throws(
    () => dimension({ unit: 'em', value: 1 }),
    /Unsupported ArkUI dimension/
  )
  assert.throws(() => color([1, 2, 3]), /four ARGB/)
})

test('all recipes keep tint open and preserve the graph alpha, including dark shades', () => {
  for (const values of Object.values(snapshots)) {
    const output = snapshot(values)
    for (const [key, recipe] of Object.entries(output.recipes)) {
      const id = `control.color.${key}`
      assert.equal(recipe.shade, values[`${id}.shade`] ?? 0)
      assert.equal(recipe.alpha, values[`${id}.alpha`] ?? 1)
      if (!recipe.shade)
        assert.equal(recipe.color, color(values[`${id}.color`]))
      for (const tint of [
        'primary',
        'neutral',
        'info',
        'success',
        'warning',
        'danger',
      ]) {
        const bytes = recipe.shade
          ? values[`color.${tint}.${recipe.shade}`]
          : values[`${id}.color`]
        assert.match(color(bytes, recipe.alpha), /^#[0-9A-F]{8}$/)
      }
    }
  }
})
