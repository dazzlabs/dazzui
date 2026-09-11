import assert from 'node:assert/strict'
import test from 'node:test'
import winUI3Plugin from './winui3.mjs'

function generate(tokens, modeSelectors = []) {
  let result
  winUI3Plugin({ modeSelectors }).build({
    tokens,
    outputFile(name, contents) {
      assert.equal(name, 'winui3/tokens.json')
      result = JSON.parse(contents)
    },
  })
  return result
}

test('theme aliases follow the last applicable mode, including referenced roles', () => {
  const themes = generate(
    {
      'base.size': { $value: { value: 8, unit: 'px' } },
      'radius.small': {
        aliasOf: 'base.size',
        mode: {
          family: { $value: { value: 7, unit: 'px' } },
          dark: { $value: { value: 6, unit: 'px' } },
          corner: { $value: { value: 5, unit: 'px' } },
        },
      },
      'control.small.radius': { aliasOf: 'radius.small' },
    },
    ['family', 'dark', 'corner'].map(mode => ({
      mode,
      selectors: ["[data-theme='family-dark']"],
    }))
  )
  assert.deepEqual(themes['studio-light']['control.small.radius'], {
    value: 8,
    unit: 'px',
  })
  assert.equal(themes['family-dark']['control.small.radius'].value, 5)
  assert.equal('base.size' in themes['studio-light'], false)
})

test('exports sRGB alpha and converts OKLCH without treating it as sRGB', () => {
  const themes = generate({
    'color.surface': {
      $value: { colorSpace: 'srgb', components: [1, 0.5, 0], alpha: 0.5 },
    },
    'color.content': {
      $value: { colorSpace: 'oklch', components: [0.5, 0, 0], alpha: 1 },
    },
  })
  assert.deepEqual(themes['studio-light']['color.surface'], [128, 255, 128, 0])
  assert.deepEqual(themes['studio-light']['color.content'], [255, 99, 99, 99])
})

test('fails on missing aliases and cycles instead of silently emitting a value', () => {
  assert.throws(() => generate({ a: { aliasOf: 'missing' } }), /Missing WinUI/)
  assert.throws(
    () => generate({ a: { aliasOf: 'b' }, b: { aliasOf: 'a' } }),
    /Circular WinUI/
  )
})
