import assert from 'node:assert/strict'
import fs from 'node:fs'
import test from 'node:test'
import {
  SUPPORT,
  sceneGaps,
  storiesData,
  storiesETS,
  value,
} from '../harmony.mjs'

const renderer = fs.readFileSync(
  new URL(
    '../../../packages/arkts/dazzui/example/entry/src/main/ets/lib/SceneNode.ets',
    import.meta.url
  ),
  'utf8'
)

test('every promised component has a native interpreter branch', () => {
  for (const name of Object.keys(SUPPORT))
    assert.ok(renderer.includes(`this.node.name === '${name}'`), name)
})

test('unsupported controls, props, ranges and actions stay explicit', () => {
  for (const content of [
    { $el: 'Tree' },
    { $el: 'Button', props: { madeUp: true } },
    { $el: 'Slider', props: { defaultValue: [20, 80] } },
    {
      $el: 'Button',
      props: { onClick: { $do: [{ toast: { title: 'Saved' } }] } },
    },
    { $el: 'Select', props: { options: [{ label: 'People', options: [] }] } },
  ])
    assert.ok(sceneGaps('test', { content }).length)
  assert.deepEqual(
    sceneGaps('live', {
      state: { s0: true },
      content: {
        $el: 'Switch',
        props: {
          checked: { $state: 's0' },
          onCheckedChange: { $do: [{ set: 's0', to: { $arg: 0 } }] },
        },
      },
    }),
    []
  )
})

test('the typed wire format preserves live state, expressions, actions and specimen addresses', () => {
  const node = value({
    $el: 'Switch',
    $at: 7,
    props: {
      checked: { $eq: [{ $state: 's0' }, 'on'] },
      onCheckedChange: { $do: [{ set: 's0', to: { $arg: 0 } }] },
    },
  })
  assert.equal(node.address, 7)
  assert.equal(node.fields.checked.items[0].kind, 'state')
  assert.equal(node.fields.onCheckedChange.items[0].fields.to.kind, 'argument')
  assert.deepEqual(value(null), { kind: 'null' })
})

test('generated stories keep previews separate, scene ids, notes and initial state', () => {
  const story = {
    id: 'button',
    name: 'Button',
    summary: 'Press',
    preview: { content: { $el: 'Button', children: ['Save'] } },
    examples: [
      {
        title: 'Live',
        note: 'A note',
        scene: {
          state: { s0: false },
          content: { $el: 'Switch', props: { checked: { $state: 's0' } } },
        },
      },
    ],
  }
  const [output] = storiesData([story])
  assert.equal(output.preview.id, 'button/preview')
  assert.equal(output.examples[0].id, 'button/0')
  assert.equal(output.examples[0].note, 'A note')
  assert.equal(output.examples[0].initial.s0.boolean, false)
  assert.deepEqual(output.examples[0].gaps, [])
  assert.match(storiesETS([story]), /export const STORIES: Story\[\]/)
})
