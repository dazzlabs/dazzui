import assert from 'node:assert/strict'
import test from 'node:test'
import { sceneGaps, storyData, unsupported } from '../nsh.mjs'

test('Native Shell support gate includes previews and examples', () => {
  assert.equal(
    unsupported([
      {
        id: 'test',
        preview: { content: { $el: 'UnknownPreview' } },
        examples: [
          { title: 'Next', scene: { content: { $el: 'UnknownExample' } } },
        ],
      },
    ]).length,
    2
  )
})

test('Native Shell preserves addresses and reactive handlers', () => {
  const preview = {
    state: { s0: false },
    content: {
      $el: 'Checkbox',
      $at: 7,
      props: {
        checked: { $state: 's0' },
        onCheckedChange: { $do: [{ set: 's0', to: { $arg: 0 } }] },
      },
      children: ['Accept'],
    },
  }
  const data = storyData({
    id: 'checkbox',
    name: 'Checkbox',
    summary: '',
    preview,
    examples: [],
  })
  assert.equal(data.preview.id, 'checkbox/preview')
  assert.deepEqual(data.preview.content, preview.content)
  assert.deepEqual(data.preview.gaps, [])
})

test('Native Shell fails closed on unmapped behavior and nested data', () => {
  const cases = [
    [{ $el: 'Button', props: { loading: true } }, /loading/],
    [
      {
        $el: 'Select',
        props: {
          options: [
            {
              label: 'Group',
              extra: true,
              options: [{ value: 'x', label: 'X' }],
            },
          ],
        },
      },
      /select/,
    ],
    [
      {
        $el: 'Menu',
        props: {
          items: [
            {
              label: 'Submenu',
              items: [{ label: 'Nested', unsupported: true }],
            },
          ],
        },
      },
      /menu/,
    ],
    [
      { $el: 'Button', props: { onClick: { $do: [{ alert: 'Hello' }] } } },
      /action/,
    ],
  ]
  for (const [content, expected] of cases)
    assert.match(sceneGaps('test/0', { content }).join('\n'), expected)
})

test('Native Shell maps compound controls and nested native items', () => {
  for (const content of [
    { $el: 'TextField', props: { multiline: true, required: true } },
    {
      $el: 'Select',
      props: {
        options: [
          { label: 'Native', options: [{ value: 'rs', label: 'Rust' }] },
        ],
      },
    },
    {
      $el: 'Menu',
      props: {
        items: [
          { label: 'View', items: [{ label: 'Sidebar', checked: true }] },
        ],
      },
    },
    { $el: 'Progress' },
    {
      $el: 'Button',
      props: { onClick: { $do: [{ toast: { title: 'Saved' } }] } },
    },
  ])
    assert.deepEqual(sceneGaps('test/0', { content }), [])
})
