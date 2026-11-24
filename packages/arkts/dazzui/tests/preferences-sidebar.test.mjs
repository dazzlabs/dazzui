import assert from 'node:assert/strict'
import fs from 'node:fs'
import path from 'node:path'
import vm from 'node:vm'
import { createRequire } from 'node:module'
import { fileURLToPath } from 'node:url'
import test from 'node:test'

import { glyphSvg } from '../../../../tools/codegen/gpui.mjs'

const require = createRequire(
  new URL('../../../../tools/codegen/package.json', import.meta.url)
)
const ts = require('typescript')
const root = fileURLToPath(new URL('../', import.meta.url))
const repoRoot = path.resolve(root, '../../..')
const loaded = new Map()

function load(file) {
  const absolute = path.resolve(root, file)
  if (loaded.has(absolute)) return loaded.get(absolute)
  const source = fs.readFileSync(absolute, 'utf8')
  const result = ts.transpileModule(source, {
    compilerOptions: {
      target: ts.ScriptTarget.ES2022,
      module: ts.ModuleKind.CommonJS,
    },
    reportDiagnostics: true,
  })
  assert.deepEqual(result.diagnostics, [], absolute)
  const exports = {}
  loaded.set(absolute, exports)
  const wrapper = vm.runInThisContext(
    `(function(exports, require) { ${result.outputText}\n})`,
    { filename: absolute }
  )
  wrapper(exports, relative =>
    load(path.resolve(path.dirname(absolute), relative + '.ets'))
  )
  return exports
}

const { DazzTokens } = load('src/main/ets/foundation/Tokens.ets')
const {
  THEME_FAMILIES,
  DRAWN_THEME_FAMILIES,
  resolveThemeName,
  resolveAppearanceDark,
  resolveThemeForMode,
  parseThemeFamily,
  parseAppearanceMode,
  isThemeName,
  isThemeFamily,
  isAppearanceMode,
  isDarkTheme,
} = load('src/main/ets/foundation/ThemeAppearance.ets')
const { SWATCH_CORNER_RADIUS, resolveSwatchThemeName, resolveSwatchPalette } =
  load('src/main/ets/foundation/PreferenceSwatchesData.ets')
const {
  clampSidebarWidth,
  resolveSidebarBounds,
  resolveSidebarWidth,
  dragSidebarWidth,
  resetSidebarWidth,
  stepSidebarWidth,
} = load('src/main/ets/foundation/SidebarData.ets')

test('paired 20px directional chevron icons exist and match Fluent SVGs', () => {
  const iconsDir = path.join(root, 'src/main/resources/rawfile/dazzui/icons')
  for (const name of [
    'ChevronLeft20Regular',
    'ChevronRight20Regular',
    'ChevronUp20Regular',
    'ChevronDown20Regular',
  ]) {
    const file = path.join(iconsDir, `${name}.svg`)
    assert.ok(fs.existsSync(file), `missing ${name}.svg`)
    const actual = fs.readFileSync(file, 'utf8')
    const expected =
      glyphSvg(repoRoot, name).replaceAll('currentColor', '#000000') + '\n'
    assert.equal(actual, expected, `${name}.svg matches Fluent source`)
  }
})

test('theme swatch resolution isolates each swatch palette while keeping selection ring on host theme', () => {
  assert.equal(
    resolveSwatchThemeName({ value: 'bright', label: 'Bright', theme: '' }),
    'bright-light'
  )
  assert.equal(
    resolveSwatchThemeName({
      value: 'ember',
      label: 'Ember',
      theme: 'ember-dark',
    }),
    'ember-dark'
  )
  assert.equal(
    resolveSwatchThemeName({ value: 'frost-light', label: 'Frost' }),
    'frost-light'
  )
  assert.equal(
    resolveSwatchThemeName(
      { value: 'unknown', label: 'Unknown' },
      'graphite-dark'
    ),
    'graphite-dark'
  )
  assert.equal(
    resolveSwatchThemeName({ value: 'unknown', label: 'Unknown' }, 'invalid'),
    'bright-light'
  )

  const outer = new DazzTokens('studio-dark')
  const brightLight = new DazzTokens('bright-light')
  const palette = resolveSwatchPalette(
    { value: 'bright', label: 'Bright', theme: 'bright-light' },
    outer
  )
  assert.equal(palette.themeName, 'bright-light')
  assert.equal(palette.surface, brightLight.color('surface'))
  assert.equal(palette.column, brightLight.color('surface-column'))
  assert.equal(palette.border, brightLight.color('border-strong'))
  assert.equal(palette.ink, brightLight.color('content'))
  assert.equal(palette.accent, brightLight.color('primary.500'))
  assert.equal(palette.radius, SWATCH_CORNER_RADIUS)
  assert.equal(palette.swatchHeight, brightLight.size('small'))
  assert.equal(palette.swatchWidth, brightLight.size('small') * 1.5)
  assert.equal(palette.ringGap, outer.metric('stroke.hairline'))
  assert.equal(palette.ringGapColor, outer.color('surface'))
  assert.equal(palette.ringStroke, outer.metric('stroke.control'))
  assert.equal(
    palette.ringColor,
    outer.paint('outlined', 'border', 'normal', 'primary')
  )
})

test('resizable sidebar width helpers clamp, reverse right-side drag, reset and step by keyboard', () => {
  assert.equal(clampSidebarWidth(150, 180, 400), 180)
  assert.equal(clampSidebarWidth(450, 180, 400), 400)
  assert.equal(clampSidebarWidth(240, 400, 180), 240)
  assert.equal(clampSidebarWidth(Number.NaN, 180, 400), 180)

  assert.deepEqual(resolveSidebarBounds(172, undefined, 180, 400), {
    min: 180,
    max: 400,
  })
  assert.deepEqual(resolveSidebarBounds(172, 200, undefined, undefined), {
    min: 120,
    max: 480,
  })

  assert.equal(resolveSidebarWidth(172, 260, 220, 200, 180, 400), 260)
  assert.equal(resolveSidebarWidth(172, undefined, 220, 200, 180, 400), 220)
  assert.equal(
    resolveSidebarWidth(172, undefined, undefined, 200, 180, 400),
    200
  )
  assert.equal(
    resolveSidebarWidth(172, undefined, undefined, undefined, 140, 400),
    172
  )

  assert.equal(dragSidebarWidth(200, 45, 'left', 180, 400), 245)
  assert.equal(dragSidebarWidth(200, -50, 'left', 180, 400), 180)
  assert.equal(dragSidebarWidth(200, -45, 'right', 180, 400), 245)
  assert.equal(dragSidebarWidth(200, 50, 'right', 180, 400), 180)

  assert.equal(resetSidebarWidth(172, 200, 180, 400), 200)
  assert.equal(resetSidebarWidth(172, undefined, 180, 400), 180)

  assert.equal(stepSidebarWidth(200, 2014, 'left', false, 180, 400), 199)
  assert.equal(stepSidebarWidth(200, 2015, 'left', false, 180, 400), 201)
  assert.equal(stepSidebarWidth(200, 2014, 'left', true, 180, 400), 190)
  assert.equal(stepSidebarWidth(200, 2015, 'left', true, 180, 400), 210)
  assert.equal(stepSidebarWidth(200, 2014, 'right', true, 180, 400), 210)
  assert.equal(stepSidebarWidth(200, 2015, 'right', true, 180, 400), 190)
  assert.equal(stepSidebarWidth(200, 2081, 'left', false, 180, 400), 180)
  assert.equal(stepSidebarWidth(200, 2082, 'left', false, 180, 400), 400)
  assert.equal(stepSidebarWidth(200, 2054, 'left', false, 180, 400), undefined)
})

test('two-axis theme appearance helpers resolve every family across light, dark and system modes', () => {
  assert.deepEqual(DRAWN_THEME_FAMILIES, [
    'bright',
    'studio',
    'frost',
    'graphite',
    'ember',
    'nocturne',
  ])
  assert.equal(THEME_FAMILIES.length, 36)

  for (const family of THEME_FAMILIES) {
    assert.equal(isThemeFamily(family), true)
    const light = resolveThemeName(family, false)
    const dark = resolveThemeName(family, true)
    assert.equal(light, `${family}-light`)
    assert.equal(dark, `${family}-dark`)
    assert.equal(isThemeName(light), true)
    assert.equal(isThemeName(dark), true)
    assert.equal(isDarkTheme(light), false)
    assert.equal(isDarkTheme(dark), true)
    assert.equal(parseThemeFamily(light), family)
    assert.equal(parseThemeFamily(dark), family)
    assert.equal(parseThemeFamily(family), family)
    assert.equal(resolveThemeForMode(family, 'light', true), light)
    assert.equal(resolveThemeForMode(family, 'dark', false), dark)
    assert.equal(resolveThemeForMode(family, 'system', false), light)
    assert.equal(resolveThemeForMode(family, 'system', true), dark)
  }

  assert.equal(
    parseThemeFamily('omarchy-flexoki-light-dark'),
    'omarchy-flexoki-light'
  )
  assert.equal(
    parseThemeFamily('omarchy-tokyo-night-light'),
    'omarchy-tokyo-night'
  )
  assert.equal(parseThemeFamily('unknown-theme'), 'bright')
  assert.equal(parseThemeFamily('unknown-theme', 'studio'), 'studio')

  assert.equal(isThemeName('not-a-theme'), false)
  assert.equal(isThemeFamily('not-a-family'), false)

  assert.equal(isAppearanceMode('light'), true)
  assert.equal(isAppearanceMode('dark'), true)
  assert.equal(isAppearanceMode('system'), true)
  assert.equal(isAppearanceMode('auto'), false)
  assert.equal(parseAppearanceMode('dark'), 'dark')
  assert.equal(parseAppearanceMode('auto'), 'system')
  assert.equal(parseAppearanceMode('auto', 'light'), 'light')

  assert.equal(resolveAppearanceDark('light', true), false)
  assert.equal(resolveAppearanceDark('dark', false), true)
  assert.equal(resolveAppearanceDark('system', false), false)
  assert.equal(resolveAppearanceDark('system', true), true)
})

test('Index.ets exports new theme, swatch and sidebar symbols without ArkUI reserved prop collisions', () => {
  const index = fs.readFileSync(path.join(root, 'Index.ets'), 'utf8')
  for (const symbol of [
    'AppearanceMode',
    'DazzAppearanceMode',
    'DazzThemeName',
    'DazzThemeFamily',
    'DazzThemeGroup',
    'THEME_FAMILIES',
    'DRAWN_THEME_FAMILIES',
    'resolveThemeName',
    'resolveAppearanceDark',
    'resolveThemeForMode',
    'parseThemeFamily',
    'parseAppearanceMode',
    'DazzPreferenceThemeSwatches',
    'DazzPreferenceThemeOption',
    'DazzSwatchPalette',
    'DazzSidebarBounds',
  ]) {
    assert.ok(index.includes(symbol), `Index.ets exports ${symbol}`)
  }

  const reserved = new Set(['width', 'height', 'size', 'align', 'enabled'])
  for (const file of [
    'src/main/ets/components/Preferences.ets',
    'src/main/ets/components/Sidebar.ets',
  ]) {
    const source = fs.readFileSync(path.join(root, file), 'utf8')
    const props = [
      ...source.matchAll(/@Prop(?:\s+@\w+\([^)]*\))?\s+(\w+)/g),
    ].map(m => m[1])
    for (const prop of props) {
      assert.equal(
        reserved.has(prop),
        false,
        `${file} must not declare reserved ArkUI @Prop "${prop}"`
      )
    }
  }
})
