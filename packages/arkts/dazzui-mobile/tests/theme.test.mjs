import assert from 'node:assert/strict'
import fs from 'node:fs'
import vm from 'node:vm'
import { createRequire } from 'node:module'
import test from 'node:test'
import { mobileColors } from '../../../../ds/plugins/harmony-mobile.mjs'

const require = createRequire(
  new URL('../../../../tools/codegen/package.json', import.meta.url)
)
const ts = require('typescript')
const root = new URL('../', import.meta.url)
const source = fs.readFileSync(
  new URL('src/main/ets/generated/DazzThemes.ets', root),
  'utf8'
)
const result = ts.transpileModule(source, {
  compilerOptions: {
    target: ts.ScriptTarget.ES2022,
    module: ts.ModuleKind.CommonJS,
  },
  reportDiagnostics: true,
})
assert.deepEqual(result.diagnostics, [])
const exports = {}
vm.runInThisContext(`(function(exports) { ${result.outputText}\n})`)(exports)
const { dazzTheme, DazzThemeFamily, DAZZ_THEME_FAMILIES } = exports
const snapshots = JSON.parse(
  fs.readFileSync(
    new URL(
      '../../../../packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json',
      import.meta.url
    )
  )
)

test('the actual ArkTS family API returns both complete native appearances', () => {
  assert.equal(DAZZ_THEME_FAMILIES.length, Object.keys(DazzThemeFamily).length)
  for (const family of DAZZ_THEME_FAMILIES) {
    const theme = dazzTheme(family)
    assert.deepEqual(theme.colors, mobileColors(snapshots[`${family}-light`]))
    assert.deepEqual(
      theme.darkColors,
      mobileColors(snapshots[`${family}-dark`])
    )
    assert.deepEqual(Object.keys(theme), ['colors', 'darkColors'])
  }
  assert.deepEqual(dazzTheme(), dazzTheme(DazzThemeFamily.Studio))
  assert.throws(() => dazzTheme('missing'), /Unknown/)
})

test('separate scopes and appearances cannot mutate one another', () => {
  const first = dazzTheme()
  const second = dazzTheme()
  first.colors.brand = '#FF123456'
  assert.notEqual(second.colors.brand, first.colors.brand)
  assert.notEqual(first.darkColors.brand, first.colors.brand)
  assert.deepEqual(dazzTheme(), second)
})

test('effective palettes keep native callbacks and controls on the chosen appearance', () => {
  const source = fs.readFileSync(
    new URL('src/main/ets/ResolvedTheme.ets', root),
    'utf8'
  )
  const compiled = ts.transpileModule(source, {
    compilerOptions: { module: ts.ModuleKind.CommonJS },
  })
  const effective = {}
  vm.runInThisContext(
    `(function(exports, require) { ${compiled.outputText}\n})`
  )(effective, () => exports)
  for (const family of DAZZ_THEME_FAMILIES) {
    for (const dark of [false, true]) {
      const resolved = effective.resolvedTheme(family, dark)
      const expected = dazzTheme(family)[dark ? 'darkColors' : 'colors']
      assert.deepEqual(resolved.colors, expected)
      assert.deepEqual(resolved.darkColors, expected)
    }
  }
})

test('the HAR is standalone, color-only, and defaults to system appearance', () => {
  const metadata = JSON.parse(
    fs.readFileSync(new URL('oh-package.json5', root))
  )
  assert.deepEqual(metadata.dependencies, {})
  const module = JSON.parse(
    fs.readFileSync(new URL('src/main/module.json5', root))
  )
  assert.deepEqual(module.module.deviceTypes, ['phone', 'tablet'])
  const scope = fs.readFileSync(
    new URL('src/main/ets/DazzMobileTheme.ets', root),
    'utf8'
  )
  assert.match(scope, /appearance: ThemeColorMode = ThemeColorMode.SYSTEM/)
  assert.match(
    scope,
    /WithTheme\(\{ theme: this.effectiveTheme\(\), colorMode: this.appearance \}\)/
  )
  assert.ok(
    !/setDefaultTheme|setColorMode|backgroundColor|fontSize|borderRadius/.test(
      scope
    )
  )
})
