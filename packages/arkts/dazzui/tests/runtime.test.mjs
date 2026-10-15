import assert from 'node:assert/strict'
import fs from 'node:fs'
import path from 'node:path'
import vm from 'node:vm'
import { createRequire } from 'node:module'
import { fileURLToPath } from 'node:url'
import test from 'node:test'

const require = createRequire(
  new URL('../../../../tools/codegen/package.json', import.meta.url)
)
const ts = require('typescript')
const root = fileURLToPath(new URL('../', import.meta.url))
const loaded = new Map()

// Execute the actual non-UI ArkTS modules. This tests their logic; it is not
// an ArkUI compiler, SDK validation or a substitute for a device test.
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
const { DazzTokens, THEME_NAMES, withAlpha } = load(
  'src/main/ets/foundation/Tokens.ets'
)
const { SceneState, text, booleanValue, stringValue, numberValue } = load(
  'example/entry/src/main/ets/lib/SceneState.ets'
)
const snapshots = JSON.parse(
  fs.readFileSync(
    new URL(
      '../../../../packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json',
      import.meta.url
    )
  )
)

test('native runtime resolves all theme/tint/variant/state recipes to the shared bytes', () => {
  for (const name of THEME_NAMES) {
    const tokens = new DazzTokens(name)
    const source = snapshots[name]
    for (const tint of [
      'primary',
      'neutral',
      'info',
      'success',
      'warning',
      'danger',
    ])
      for (const variant of [
        'filled',
        'tinted',
        'outlined',
        'normal',
        'recessed',
        'plain',
      ])
        for (const part of ['surface', 'content', 'border'])
          for (const state of ['normal', 'hovered', 'pressed', 'disabled']) {
            const id = `control.color.${variant}.${part}.${state}`
            const shade = source[`${id}.shade`]
            const bytes =
              shade === undefined
                ? source[`${id}.color`]
                : source[`color.${tint}.${shade}`]
            const alpha = source[`${id}.alpha`] ?? 1
            const expected =
              '#' +
              [Math.round(bytes[0] * alpha), ...bytes.slice(1)]
                .map(n => n.toString(16).padStart(2, '0'))
                .join('')
                .toUpperCase()
            assert.equal(
              tokens.paint(variant, part, state, tint),
              expected,
              `${name}/${tint}/${id}`
            )
          }
  }
})

test('theme instances are isolated, unknown names and roles fail early', () => {
  const light = new DazzTokens()
  const dark = new DazzTokens('studio-dark')
  assert.notEqual(light.color('surface'), dark.color('surface'))
  assert.equal(light.metric('control.small.size'), 26)
  assert.equal(light.type('labelSmall').size, 11)
  const type = light.type('labelSmall')
  type.size = 999
  assert.equal(light.type('labelSmall').size, 11)
  assert.equal(new DazzTokens().type('labelSmall').size, 11)
  const shadows = light.shadow('shadow.lg')
  shadows[0].radius = 999
  assert.notEqual(light.shadow('shadow.lg')[0].radius, 999)
  assert.throws(() => new DazzTokens('missing'), /Unknown DazzUI theme/)
  assert.throws(() => light.color('missing'), /Unknown DazzUI color/)
  assert.throws(() => light.metric('missing'), /Unknown DazzUI metric/)
  assert.equal(withAlpha('#80010203', 0.5), '#40010203')
})

test('scene setters, arguments and comparisons use the real interpreter and do not leak across scenes', () => {
  const initial = { s0: stringValue('free') }
  const one = new SceneState(initial)
  const two = new SceneState(initial)
  const reference = { kind: 'state', text: 's0' }
  const eq = { kind: 'eq', items: [reference, stringValue('pro')] }
  const action = {
    kind: 'actions',
    items: [
      {
        kind: 'object',
        fields: {
          set: stringValue('s0'),
          to: { kind: 'argument', number: 0 },
        },
      },
    ],
  }
  assert.equal(one.evaluate(eq).boolean, false)
  one.run(action, stringValue('pro'))
  assert.equal(one.evaluate(eq).boolean, true)
  assert.equal(text(two.evaluate(reference)), 'free')
  assert.equal(text(initial.s0), 'free')
  assert.equal(text(booleanValue(false)), 'false')
  assert.equal(text(numberValue(0)), '0')
  assert.throws(
    () =>
      one.run({
        kind: 'actions',
        items: [{ kind: 'object', fields: { set: stringValue('unknown') } }],
      }),
    /Unknown scene state/
  )
})

test('native package is standalone and declares the PC device type', () => {
  const module = JSON.parse(
    fs.readFileSync(path.join(root, 'src/main/module.json5'))
  )
  assert.equal(module.module.type, 'har')
  assert.deepEqual(module.module.deviceTypes, ['2in1'])
  assert.equal(
    JSON.parse(fs.readFileSync(path.join(root, 'oh-package.json5'))).main,
    'Index.ets'
  )
})
