import test from 'node:test'
import assert from 'node:assert/strict'
import { androidSupports, androidOutputs } from '../android.mjs'

test('Android excludes future content and interactions from generated coverage', () => {
  const scene = {
    state: { text: 'Ada' },
    content: {
      component: 'Button',
      on: { press: { set: 'text', value: 'Grace' } },
      children: ['Save'],
    },
  }
  assert.equal(androidSupports(scene), true)
  scene.content.on.longPress = { set: 'text', value: 'Other' }
  assert.equal(androidSupports(scene), false)
  const output = androidOutputs(
    [{ id: 'unknown-event', title: 'Components/Button', name: 'Example' }],
    new Map([['unknown-event', scene]])
  )
  assert.deepEqual(
    JSON.parse(
      output.get(
        'packages/mobile_ui_android/example/src/main/assets/scenes.json'
      )
    ),
    []
  )
  delete scene.content.on.longPress
  scene.content.props = { href: 'https://example.test/' }
  assert.equal(androidSupports(scene), false)
})

test('Android capability checking traverses slots and expression branches', () => {
  const scene = {
    state: { name: '' },
    content: {
      component: 'EmptyState',
      props: { title: 'Empty' },
      slots: { action: { component: 'TextField', bind: { value: 'name' } } },
    },
  }
  assert.equal(androidSupports(scene), true)
  scene.content.slots.action = {
    component: 'Value',
    // Shared expression data uses `then` as a branch, not a Promise callback.
    // oxlint-disable-next-line unicorn/no-thenable
    value: { if: true, then: 'Ready', else: { uppercase: { state: 'name' } } },
  }
  assert.equal(androidSupports(scene), false)
  scene.content.slots.action = { component: 'NativeOnlyFixture' }
  assert.equal(androidSupports(scene), false)
})
