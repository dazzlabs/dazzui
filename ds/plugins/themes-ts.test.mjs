import assert from 'node:assert/strict'
import fs from 'node:fs'
import test from 'node:test'
import { modeSelectors } from '../terrazzo.config.mjs'
import { buildCatalog } from './catalog.mjs'
import { generateThemesTS } from './themes-ts.mjs'

const generated = fs.readFileSync(
  new URL('../../packages/js/dazzui/src/generated/themes.ts', import.meta.url),
  'utf8'
)

test('generated themes.ts is reproducible from modeSelectors', () => {
  assert.equal(generated, generateThemesTS(modeSelectors))
})

test('catalog covers all drawn and imported themes in light/dark pairs', () => {
  const { groups, families, themes } = buildCatalog(modeSelectors)
  assert.equal(groups.length, 4)
  assert.deepEqual(
    groups.map(group => group.id),
    ['drawn', 'desktops', 'applications', 'omarchy']
  )
  assert.equal(families.length * 2, themes.length)
  assert.equal(families[0].family, 'bright')
  assert.equal(themes[0].name, 'bright-light')
  assert.equal(themes[0].isDefault, true)
  for (const theme of themes.slice(1)) {
    assert.equal(theme.isDefault, false)
  }
})
