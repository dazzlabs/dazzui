import assert from 'node:assert/strict'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { readReactStories, unsupported, storyRs } from '../gpui.mjs'
import { scenesJson } from '../scenes.mjs'

const root = fileURLToPath(new URL('../../../', import.meta.url))
const find = (value, name) => {
  if (!value || typeof value !== 'object') return undefined
  if (value.$el === name) return value
  for (const child of Object.values(value)) {
    const found = find(child, name)
    if (found) return found
  }
}
test('GPUI opts into the actual Tabs source and its document removal action', () => {
  const stories = readReactStories(root)
  assert.deepEqual(
    stories.map(story => story.id),
    ['tabs']
  )
  assert.deepEqual(unsupported(stories), [])
  const story = stories[0]
  const documents = story.examples.at(-1).scene
  const tabs = find(documents.content, 'Tabs')
  assert.equal(documents.state[tabs.props.items.$state].length, 4)
  assert.deepEqual(tabs.props.onClose.$do, [
    { remove: tabs.props.items.$state, key: 'value', match: { $arg: 0 } },
  ])
  const contract = JSON.parse(scenesJson(stories, { stage: ['ChromeBar'] }))
  assert.equal(contract.stories[0].scenes.length, 5)
  assert.equal(contract.stories[0].scenes[0].specimens[0].component, 'tabs')
  assert.ok(
    !contract.stories[0].scenes[0].specimens.some(
      s => s.component === 'chrome-bar'
    )
  )
  assert.match(storyRs(story), /stories\/react\/tabs\.tsx/)
})
test('GPUI refuses new tab item shapes rather than dropping their properties', () => {
  const stories = readReactStories(root)
  const tabs = find(stories[0].preview.content, 'Tabs')
  tabs.props.items[0].unmapped = true
  assert.ok(
    unsupported(stories).some(gap => gap.includes('unmapped Tabs items'))
  )
})
