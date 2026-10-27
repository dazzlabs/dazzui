import assert from 'node:assert/strict'
import fs from 'node:fs'
import vm from 'node:vm'
import { createRequire } from 'node:module'
import test from 'node:test'

const require = createRequire(
  new URL('../../../../tools/codegen/package.json', import.meta.url)
)
const ts = require('typescript')
const root = new URL('../', import.meta.url)

function reader(ui, font) {
  function load(file) {
    const source = fs.readFileSync(new URL(file, root), 'utf8')
    const compiled = ts.transpileModule(source, {
      compilerOptions: { module: ts.ModuleKind.CommonJS },
    })
    const exports = {}
    vm.runInNewContext(
      `(function(exports, require) { ${compiled.outputText}\n})`,
      {
        $r: id => ({ id }),
        FontWeight: { Normal: 400 },
      }
    )(exports, name =>
      name === '@kit.ArkUI'
        ? { font }
        : load('src/main/ets/foundation/SystemStyleData.ets')
    )
    return exports
  }
  return load('src/main/ets/foundation/SystemStyle.ets').readSystemStyle(ui)
}

test('native font resources are resolved once, with engine weights and a scoped font query', () => {
  const config = {
    fontId: 'selected-face',
    fontSizeScale: 2,
    fontWeightScale: 1.4,
  }
  const metrics = {
    ohos_id_button_font_size: 16,
    ohos_id_text_size_body1: 16,
    ohos_id_text_size_body2: 14,
    ohos_id_text_size_button2: 14,
    ohos_id_button_font_weight: 4,
    ohos_id_textfield_font_weight: 10,
    ohos_id_big_font_size_scale: 1.75,
    ohos_id_large_font_size_scale: 2,
    ohos_id_max_font_size_scale: 3.2,
    button_aging_normal_padding: 8,
    button_aging_small_padding: 4,
    ohos_id_button_height: 40,
    control_button_height_small: 28,
    ohos_id_button_horizontal_padding: 16,
    ohos_id_button_vertical_padding: 4,
    ohos_id_textfield_padding_horizontal: 12,
    ohos_id_textfield_padding_vertical: 8,
    ohos_id_textfield_height: 40,
  }
  let scoped = false
  const ui = {
    getHostContext: () => ({
      config,
      resourceManager: {
        getNumber: id => {
          const key = id.replace('sys.float.', '')
          const value = metrics[key] ?? 12
          return /font_weight|font_size_scale/.test(key) ? value : value * 3
        },
        getStringSync: () => 'sans-serif',
      },
    }),
    // Density is 3; font scaling is nonlinear. Do not just multiply by config.
    fp2px: fp => fp * 1.75 * 3,
    px2vp: px => px / 3,
    runScopedTask: action => {
      scoped = true
      action()
      scoped = false
    },
    getMeasureUtils: () => ({
      measureTextSize: ({ fontSize }) => ({
        height: parseFloat(fontSize) * 1.2 * 3,
      }),
    }),
  }
  const font = {
    getUIFontConfig: () => {
      assert.equal(scoped, true)
      return { generic: [{ family: 'sans-serif' }, { family: 'serif' }] }
    },
  }
  const style = reader(ui, font)
  assert.equal(style.fontId, 'selected-face')
  assert.equal(style.fontScale, 2)
  assert.equal(style.fontWeightScale, 1.4)
  assert.equal(style.buttonFontSize, 28)
  assert.equal(style.smallButtonFontSize, 24.5)
  assert.equal(style.buttonFontWeight, 500)
  assert.equal(style.fieldFontWeight, 400)
  assert.equal(style.lengthScale, 1.75)
  assert.equal(style.buttonHeight, 49.6)
  assert.equal(style.fieldHeight, 49.6)
  assert.deepEqual(Array.from(style.fontFamilies), ['sans-serif', 'serif'])
  assert.equal(style.fontFamily, 'sans-serif')
  config.fontId = 'new-face'
  assert.equal(reader(ui, font).fontId, 'new-face')
  assert.equal(style.fontId, 'selected-face')
  assert.throws(
    () => reader({ getHostContext: () => undefined }, font),
    /live UIAbility/
  )
})

test('the desktop host opts into system fonts independently from its palette', () => {
  const app = JSON.parse(
    fs.readFileSync(new URL('example/AppScope/app.json5', root))
  )
  assert.equal(app.app.configuration, '$profile:configuration')
  const profile = JSON.parse(
    fs.readFileSync(
      new URL(
        'example/AppScope/resources/base/profile/configuration.json',
        root
      )
    )
  )
  assert.equal(profile.configuration.fontSizeScale, 'followSystem')
})
