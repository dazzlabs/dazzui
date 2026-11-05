import assert from 'node:assert/strict'
import fs from 'node:fs'
import test from 'node:test'
import YAML from 'yaml'
import {
  color,
  paint,
  fontStack,
  generateCSS,
  TINTS,
  VARIANTS,
  STATES,
} from './gtk4.mjs'

const snapshots = JSON.parse(
  fs.readFileSync(
    new URL(
      '../../packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json',
      import.meta.url
    )
  )
)
const codeFont = YAML.parse(
  fs.readFileSync(
    new URL('../tokens/global/_base.yaml', import.meta.url),
    'utf8'
  )
).base.font.code.$value
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

test('translates the web system font generic before the CJK fallback', () => {
  const generated = generateCSS(
    { 'bright-light': snapshots['bright-light'] },
    codeFont
  )
  assert.match(
    generated,
    /--body-medium-font-family: ".AppleSystemUIFont", "SF Pro Text", sans-serif, "PingFang SC"/
  )
  assert.ok(!generated.includes('"system-ui"'))
  assert.match(
    generated,
    /--base-font-code: monospace, "SF Mono", "Menlo", monospace/
  )
})

test('recipe alpha multiplies the source alpha and missing paints fail', () => {
  assert.equal(
    paint({ 'x.color': [128, 1, 2, 3], 'x.alpha': 0.5 }, 'x'),
    'rgba(1, 2, 3, 0.25098)'
  )
  assert.throws(() => paint({}, 'missing'), /Missing GTK paint/)
})

test('symbol fallback follows the primary UI generic and complete CJK stack', () => {
  assert.equal(
    fontStack(['system-ui', 'Noto Sans CJK SC', 'sans-serif']),
    'sans-serif, "Noto Sans CJK SC", monospace, sans-serif'
  )
  assert.equal(
    fontStack(['Custom UI', 'sans-serif']),
    '"Custom UI", sans-serif'
  )
  assert.equal(
    fontStack(['ui-monospace', 'SF Mono', 'monospace']),
    'monospace, "SF Mono", monospace'
  )
})

test('native menu search uses the React primitive search-session duration', () => {
  const source = fs.readFileSync(
    new URL(
      '../../packages/js/dazzui/node_modules/@base-ui/react/internals/constants.mjs',
      import.meta.url
    ),
    'utf8'
  )
  const milliseconds = Number(source.match(/TYPEAHEAD_RESET_MS = (\d+)/)[1])
  for (const theme of Object.values(snapshots))
    assert.deepEqual(theme['menu.typeahead-reset-duration'], {
      value: milliseconds,
      unit: 'ms',
    })
})

test('native slider shadow resolves the shared lift, including flat Omarchy knobs', () => {
  for (const [name, snapshot] of Object.entries(snapshots)) {
    const block = css.split(`.dz-theme-${name} {\n`)[1].split('\n}')[0]
    const lift = snapshot['slider.thumb.lift']
    assert.ok(block.includes('--slider-thumb-shadow: '))
    if (lift === 0)
      assert.ok(block.includes('--slider-thumb-shadow: none;'), name)
    else {
      const shadow = block.split('--shadow-xs: ')[1].split(';')[0]
      assert.ok(block.includes(`--slider-thumb-shadow: ${shadow};`), name)
    }
  }
})
