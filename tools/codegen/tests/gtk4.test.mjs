import assert from 'node:assert/strict'
import test from 'node:test'
import {
  sceneGaps,
  storyJSON,
  storyRust,
  storiesIndexRust,
  themesRust,
} from '../gtk4.mjs'

test('reports unsupported props and ranges instead of dropping them', () => {
  assert.match(
    sceneGaps('button/0', {
      content: { $el: 'Button', props: { unsupportedProp: 'x' } },
    })[0],
    /unsupportedProp/
  )
  assert.match(
    sceneGaps('slider/0', {
      content: { $el: 'Slider', props: { defaultValue: [20, 80] } },
    }).join('\n'),
    /range sliders/
  )
  assert.deepEqual(
    sceneGaps('switch/0', {
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

test('keeps scene ids and specimen addresses from shared codegen', () => {
  const story = {
    id: 'button',
    name: 'Button',
    summary: 'Press',
    preview: { content: { $el: 'Button', $at: 7, children: ['Save'] } },
    examples: [],
  }
  const result = JSON.parse(storyJSON(story))
  assert.equal(result.preview.id, 'button/preview')
  assert.equal(result.preview.content.$at, 7)
  assert.deepEqual(result.preview.gaps, [])
})

test('accepts grouped selectors and keeps rich label gaps explicit', () => {
  const props = {
    options: [{ label: 'People', options: [{ value: 'ada', label: 'Ada' }] }],
    placeholder: 'Choose a person',
  }
  assert.deepEqual(
    sceneGaps('select/0', { content: { $el: 'Select', props } }),
    []
  )
  assert.match(
    sceneGaps('select/1', {
      content: {
        $el: 'Select',
        props: {
          ...props,
          options: [
            { value: 'ada', label: { $el: 'Badge', children: ['Ada'] } },
          ],
        },
      },
    }).join('\n'),
    /rich select option labels/
  )
})

test('generates one Rust module per story, a registry and React family groups', () => {
  const story = {
    id: 'icon-button',
    name: 'IconButton',
    summary: 'Press',
    preview: { content: { $el: 'Button' } },
    examples: [],
  }
  assert.match(storyRust(story), /pub const STORY/)
  assert.match(storiesIndexRust([story]), /mod icon_button;/)
  assert.match(storiesIndexRust([story]), /icon_button::STORY/)
  assert.match(
    themesRust([
      { title: 'DazzUI', families: [{ value: 'studio', title: 'Studio' }] },
    ]),
    /Family \{ value: "studio", title: "Studio" \}/
  )
})
