/* oxlint-disable unicorn/no-thenable -- "then" is validated JSON branch data, never a function. */
import test from 'node:test'
import assert from 'node:assert/strict'
import { valueType, evaluateValue } from '../schema/value.mjs'
import { validateScene } from '../schema/scene.mjs'

test('conditional content follows shared state without mutating it', () => {
  const expression = { if: { state: 'enabled' }, then: 'On', else: 'Off' }
  const state = { enabled: true, other: false }
  assert.equal(valueType(expression, state), 'string')
  assert.equal(evaluateValue(expression, state), 'On')
  assert.equal(evaluateValue(expression, { ...state, enabled: false }), 'Off')
  assert.deepEqual(state, { enabled: true, other: false })
})
test('expressions reject unknown state, executable escapes and mixed branches', () => {
  for (const value of [
    { state: 'missing' },
    { code: 'doSomething()' },
    { if: 'truthy', then: 'a', else: 'b' },
    { if: true, then: 1, else: '1' },
  ])
    assert.throws(() => valueType(value, {}))
})
test('checked bindings require boolean state and matching checkpoint targets', () => {
  const scene = {
    state: { enabled: true },
    content: {
      component: 'Switch',
      id: 'switch',
      bind: { checked: 'enabled' },
    },
    checkpoints: [
      {
        id: 'off',
        steps: [{ action: 'click', target: 'switch' }],
        expect: { enabled: false },
      },
    ],
  }
  assert.doesNotThrow(() => validateScene(scene))
  scene.state.enabled = 'yes'
  assert.throws(() => validateScene(scene), /invalid state binding/)
})

test('derived form expressions trim ECMAScript whitespace and compose strings', () => {
  const value = { concat: ['Saved as ', { trim: { state: 'name' } }, '.docx'] }
  assert.equal(valueType(value, { name: '' }), 'string')
  assert.equal(
    evaluateValue(value, { name: '\ufeff\u00a0  Aurora \u3000' }),
    'Saved as Aurora.docx'
  )
  assert.equal(evaluateValue({ trim: '\u0085x\u0085' }, {}), '\u0085x\u0085')
  assert.equal(
    evaluateValue(
      { eq: [{ trim: { state: 'name' } }, ''] },
      { name: ' \t\ufeff' }
    ),
    true
  )
  assert.throws(() => valueType({ eq: [true, 'true'] }, {}), /same type/)
  assert.throws(() => valueType({ trim: 12 }, {}), /requires text/)
  assert.throws(() => valueType({ concat: [1, 'file'] }, {}), /text operands/)
})

test('periodic numeric state reaches its endpoint and wraps without coercion', () => {
  const expression = {
    if: { gte: [{ state: 'value' }, 100] },
    then: 0,
    else: { add: [{ state: 'value' }, 4] },
  }
  const state = { value: 8 }
  assert.equal(valueType(expression, state), 'number')
  for (let i = 0; i < 23; i++) state.value = evaluateValue(expression, state)
  assert.equal(state.value, 100)
  assert.equal(evaluateValue(expression, state), 0)
  assert.throws(() => valueType({ add: [1, '4'] }, {}), /numeric operands/)
  assert.throws(() => valueType({ gte: [1] }, {}), /numeric operands/)
})
test('timer definitions reject invalid intervals, targets and result types', () => {
  const scene = {
    state: { value: 8 },
    content: { component: 'Progress', derive: { value: { state: 'value' } } },
    timers: [
      { every: 400, set: 'value', value: { add: [{ state: 'value' }, 4] } },
    ],
  }
  assert.doesNotThrow(() => validateScene(scene))
  for (const every of [0, -1, 0.5, 2147483648])
    assert.throws(
      () =>
        validateScene({ ...scene, timers: [{ ...scene.timers[0], every }] }),
      /interval/
    )
  assert.throws(
    () =>
      validateScene({
        ...scene,
        timers: [{ ...scene.timers[0], set: 'missing' }],
      }),
    /matching declared state/
  )
  assert.throws(
    () =>
      validateScene({ ...scene, timers: [{ ...scene.timers[0], value: '9' }] }),
    /matching declared state/
  )
})
