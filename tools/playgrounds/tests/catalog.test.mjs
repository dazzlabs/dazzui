import { test } from 'node:test'
import assert from 'node:assert/strict'
import { parseStories, sortStories } from '../catalog.mjs'

const meta =
  "const meta = { title: 'Components/TextField', parameters: { layout: 'centered' } }; export default meta;"
test('IDs use export names independently of display names', () => {
  const [story] = parseStories(
    `${meta} export const ErrorState = { name: 'Error', render: () => null }`,
    'text-field.stories.tsx'
  )
  assert.equal(story.id, 'components-textfield--error-state')
  assert.equal(story.name, 'Error')
  assert.equal(story.layout, 'centered')
})
test('source order is retained and imported scene metadata is read without executing TSX', () => {
  const source = `import scenes from './text-field.scene.json'; ${meta}
    export const Overview = { parameters: { layout: 'fullscreen' }, render: () => { throw new Error('must not execute') } };
    export const Default = defineStory(scenes.Default);
    export const ErrorState = defineStory(scenes.ErrorState, { name: 'Error' });`
  const stories = sortStories(parseStories(source, 'text-field.stories.tsx'))
  assert.deepEqual(
    stories.map(s => s.exportName),
    ['Overview', 'Default', 'ErrorState']
  )
  assert.deepEqual(stories[1].scene, {
    file: 'text-field.scene.json',
    key: 'Default',
  })
  assert.equal(stories[2].name, 'Error')
})
test('duplicate IDs and unrecognized exports fail generation', () => {
  const stories = parseStories(
    `${meta} export const Default = { render: () => null }`,
    'a.stories.tsx'
  )
  assert.throws(() => sortStories([...stories, ...stories]), /Duplicate/)
  assert.throws(
    () =>
      parseStories(
        `${meta} export const Default = unknownFactory()`,
        'a.stories.tsx'
      ),
    /must be a story object/
  )
})
