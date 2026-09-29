/* oxlint-disable unicorn/no-thenable -- "then" is declarative JSON branch data. */
import { test } from 'node:test'
import assert from 'node:assert/strict'
import { validateScene } from '../schema/scene.mjs'

test('typography only accepts named sizes and heights must be positive', () => {
  assert.throws(
    () =>
      validateScene({
        content: {
          component: 'Inline',
          props: { fontSize: 22 },
          children: ['Count'],
        },
      }),
    /invalid fontSize/
  )
  assert.throws(
    () =>
      validateScene({ content: { component: 'Box', props: { height: -1 } } }),
    /positive logical pixels/
  )
  assert.doesNotThrow(() =>
    validateScene({
      content: {
        component: 'Col',
        props: { gap: 'spacing.2' },
        children: [
          { component: 'Inline', props: { mono: true }, children: ['POST'] },
        ],
      },
    })
  )
})

test('grid column minima must be positive finite logical pixels', () => {
  for (const min of [0, -1, '18rem', Infinity])
    assert.throws(
      () => validateScene({ content: { component: 'Grid', props: { min } } }),
      /scalar|positive logical pixels/
    )
  assert.doesNotThrow(() =>
    validateScene({ content: { component: 'Grid', props: { min: 288 } } })
  )
})

test('named slots share the scene state and checkpoint target namespace', () => {
  const scene = {
    state: { query: 'Ada' },
    content: {
      component: 'EmptyState',
      props: { title: 'No results' },
      slots: {
        action: {
          component: 'Col',
          children: [
            { component: 'TextField', id: 'query', bind: { value: 'query' } },
            {
              component: 'Button',
              id: 'clear',
              on: { press: { set: 'query', value: '' } },
              children: ['Clear'],
            },
          ],
        },
      },
    },
    checkpoints: [
      {
        id: 'cleared',
        steps: [{ action: 'click', target: 'clear' }],
        expect: { query: '' },
      },
    ],
  }
  assert.doesNotThrow(() => validateScene(scene))
  const duplicate = structuredClone(scene)
  duplicate.content.slots.action.children[1].id = 'query'
  assert.throws(() => validateScene(duplicate), /duplicate node id/)
  const unknown = structuredClone(scene)
  unknown.content.slots.footer = { component: 'Button' }
  assert.throws(() => validateScene(unknown), /unknown field footer/)
  const missing = structuredClone(scene)
  missing.content.slots.action.children[0].bind.value = 'missing'
  assert.throws(() => validateScene(missing), /invalid state binding/)
})
test('steps require supported status branches and textual content', () => {
  const scene = {
    state: { stage: 0 },
    content: {
      component: 'Step',
      props: { label: 'Upload' },
      derive: {
        status: {
          if: { gte: [{ state: 'stage' }, 1] },
          then: 'done',
          else: 'active',
        },
      },
    },
  }
  assert.doesNotThrow(() => validateScene(scene))
  scene.content.derive.status.else = 'unknown'
  assert.throws(() => validateScene(scene), /valid field states/)
  assert.throws(
    () =>
      validateScene({
        content: {
          component: 'Step',
          props: { label: 'Upload', status: 'done', meta: 42 },
        },
      }),
    /meta must be text/
  )
  assert.throws(
    () =>
      validateScene({
        content: { component: 'Step', props: { label: 'Upload' } },
      }),
    /requires a label and status/
  )
})
test('progress accepts finite percentages and rejects invalid values and children', () => {
  const progress = props => ({ content: { component: 'Progress', props } })
  for (const value of [-12, 0, 43.5, 150])
    assert.doesNotThrow(() => validateScene(progress({ value })))
  assert.doesNotThrow(() => validateScene(progress({ gradient: true })))
  for (const value of [Infinity, NaN, '43'])
    assert.throws(
      () => validateScene(progress({ value })),
      /scalar|finite number/
    )
  assert.throws(() => validateScene(progress({ gradient: 'true' })), /boolean/)
  assert.throws(
    () =>
      validateScene({ content: { component: 'Progress', children: ['43%'] } }),
    /no children/
  )
})
const scene = () => ({
  state: { text: '' },
  content: { component: 'TextField', id: 'field', bind: { value: 'text' } },
  checkpoints: [
    {
      id: 'edited',
      steps: [{ action: 'fill', target: 'field', value: 'Ada' }],
      expect: { text: 'Ada' },
    },
  ],
})
test('validates a shared editable scene', () =>
  assert.doesNotThrow(() => validateScene(scene())))
test('rejects unknown props, nodes, bindings and checkpoint targets', () => {
  let value = scene()
  value.content.props = { colour: '#fff' }
  assert.throws(() => validateScene(value), /unknown field colour/)
  value = scene()
  value.content.component = 'Missing'
  assert.throws(() => validateScene(value), /unsupported component/)
  value = scene()
  value.content.bind.value = 'missing'
  assert.throws(() => validateScene(value), /invalid state binding/)
  value = scene()
  value.checkpoints[0].steps[0].target = 'missing'
  assert.throws(() => validateScene(value), /unknown checkpoint target/)
})
test('rejects invalid enums, duplicate IDs and wrong state types', () => {
  let value = scene()
  value.content.props = { size: 'huge' }
  assert.throws(() => validateScene(value), /invalid size/)
  value = scene()
  value.content = { component: 'Row', children: [value.content, value.content] }
  assert.throws(() => validateScene(value), /duplicate node id/)
  value = scene()
  value.checkpoints[0].expect.text = false
  assert.throws(() => validateScene(value), /invalid expected state/)
})

test('documentation and glyph content cannot be omitted or replaced by adapter defaults', () => {
  for (const component of ['Page', 'Section', 'Specimen', 'Icon', 'KeyCap'])
    assert.throws(() => validateScene({ content: { component } }), /requires/)
})

test('badge and container axes retain their own vocabulary', () => {
  assert.doesNotThrow(() =>
    validateScene({
      content: {
        component: 'Badge',
        props: { variant: 'raised' },
        children: ['Draft'],
      },
    })
  )
  assert.throws(
    () =>
      validateScene({
        content: { component: 'Badge', props: { variant: 'plain' } },
      }),
    /invalid variant/
  )
  assert.doesNotThrow(() =>
    validateScene({
      content: {
        component: 'Icon',
        props: { name: 'Circle20Filled', size: 8 },
      },
    })
  )
  assert.throws(
    () =>
      validateScene({
        content: { component: 'Box', props: { surface: '#fff' } },
      }),
    /invalid surface/
  )
  assert.throws(
    () =>
      validateScene({
        content: { component: 'TableCell', props: { align: 'between' } },
      }),
    /invalid align/
  )
})

test('derived props reject invalid enums, duplicates and non-button disabled assertions', () => {
  const content = {
    component: 'Button',
    id: 'save',
    derive: { disabled: { eq: ['a', 'b'] } },
    children: ['Save'],
  }
  assert.doesNotThrow(() =>
    validateScene({
      content,
      checkpoints: [
        { id: 'initial', steps: [], expectDisabled: { save: false } },
      ],
    })
  )
  assert.throws(
    () =>
      validateScene({ content: { ...content, props: { disabled: false } } }),
    /duplicate static and derived/
  )
  assert.throws(
    () =>
      validateScene({
        state: { name: 'error' },
        content: {
          component: 'FormField',
          props: { label: 'Name' },
          derive: { state: { state: 'name' } },
        },
      }),
    /select valid field states/
  )
  assert.throws(
    () =>
      validateScene({
        content: { component: 'Inline', id: 'save', children: ['Save'] },
        checkpoints: [
          { id: 'initial', steps: [], expectDisabled: { save: false } },
        ],
      }),
    /button id/
  )
})
