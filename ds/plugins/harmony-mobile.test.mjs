import assert from 'node:assert/strict'
import fs from 'node:fs'
import test from 'node:test'
import { color } from './harmony.mjs'
import {
  generateMobile,
  mobileColors,
  mobileFamilies,
} from './harmony-mobile.mjs'

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
    '../../packages/arkts/dazzui-mobile/src/main/ets/generated/DazzThemes.ets',
    import.meta.url
  ),
  'utf8'
)

test('mobile ArkUI output is reproducible from the shared resolved graph', () => {
  assert.equal(generated, generateMobile(snapshots))
})

test('every paired theme gets only native color roles, with ds recipe alpha intact', () => {
  const families = mobileFamilies(snapshots)
  assert.equal(families.length * 2, Object.keys(snapshots).length)
  for (const family of families) {
    for (const mode of ['light', 'dark']) {
      const values = snapshots[`${family}-${mode}`]
      const result = mobileColors(values)
      assert.equal(Object.keys(result).length, 45)
      for (const value of Object.values(result))
        assert.match(value, /^#[\dA-F]{8}$/)
      const fill = 'control.color.filled.surface.normal'
      assert.equal(
        result.brand,
        color(
          values[`color.primary.${values[`${fill}.shade`]}`],
          values[`${fill}.alpha`]
        )
      )
      assert.equal(result.backgroundEmphasize, result.brand)
      assert.equal(result.compBackgroundEmphasize, result.brand)
      for (const [native, ds] of Object.entries({
        fontPrimary: 'content',
        fontSecondary: 'content-secondary',
        fontTertiary: 'content-muted',
        fontFourth: 'content-faint',
        fontOnPrimary: 'on-accent',
        backgroundPrimary: 'surface',
        backgroundSecondary: 'surface-column',
        compDivider: 'border',
      }))
        assert.equal(
          result[native],
          color(values[`color.${ds}`]),
          `${family}-${mode}/${native}`
        )
      const wash = 'control.color.tinted.surface.normal'
      assert.equal(
        result.compEmphasizeSecondary,
        color(
          values[`color.primary.${values[`${wash}.shade`]}`],
          values[`${wash}.alpha`]
        )
      )
      assert.equal(result.compBackgroundPrimaryTran.slice(1, 3), '00')
      assert.equal(
        result.interactivePressed,
        undefined,
        'ArkUI keeps state treatment'
      )
    }
  }
  assert.ok(!/metrics|typography|radius|duration/.test(generated))
})

test('single-appearance imports stay outside the mobile family registry', () => {
  assert.deepEqual(
    mobileFamilies({
      'studio-light': {},
      'studio-dark': {},
      'solo-light': {},
      odd: {},
    }),
    ['studio']
  )
})
