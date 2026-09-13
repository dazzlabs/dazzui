import test from 'node:test'
import assert from 'node:assert/strict'
import { gpuiSupports, gpuiDefinition } from '../gpui-scene.mjs'

test('GPUI traverses slot contents for coverage and input buffers', () => {
  const scene = {
    state: { query: 'Ada' },
    content: {
      component: 'EmptyState',
      props: { title: 'Search' },
      slots: {
        action: { component: 'TextField', bind: { value: 'query' } },
      },
    },
  }
  assert.equal(gpuiSupports(scene), true)
  assert.match(gpuiDefinition(scene, 'slot-test'), /query/)
  assert.match(gpuiDefinition(scene, 'slot-test'), /slot-action/)
  scene.content.slots.action = { component: 'Unsupported' }
  assert.equal(gpuiSupports(scene), false)
})

test('GPUI coverage admits bindings and actions but rejects unsupported content', () => {
  const scene = {
    state: { enabled: true },
    content: { component: 'Switch', bind: { checked: 'enabled' } },
  }
  assert.equal(gpuiSupports(scene), true)
  assert.equal(gpuiSupports({ ...scene, state: { enabled: 'yes' } }), false)
  assert.equal(
    gpuiSupports({
      ...scene,
      content: {
        component: 'Button',
        on: { press: { set: 'enabled', value: false } },
      },
    }),
    true
  )
  assert.equal(
    gpuiSupports({
      ...scene,
      content: {
        component: 'Checkbox',
        bind: { checked: 'enabled' },
        children: [
          { component: 'Icon', props: { name: 'ArrowDownload20Regular' } },
        ],
      },
    }),
    false
  )
})
