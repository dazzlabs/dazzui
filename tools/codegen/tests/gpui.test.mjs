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
    ['tabs', 'window-frame']
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

test('GPUI sidebar extensions retain shared addresses and compile nested row actions', async () => {
  const { readStories } = await import('../stories.mjs')
  const { extensionScenes } = await import('../gpui.mjs')
  const shared = await readStories(root)
  const optedIn = await readStories(root, { platform: 'gpui' })
  const sidebar = optedIn.find(story => story.id === 'sidebar')
  const original = shared.find(story => story.id === 'sidebar')
  assert.equal(sidebar.examples.length, original.examples.length + 4)
  const scene = sidebar.examples.find(
    example => example.title === 'Nested navigation'
  ).scene
  const row = find(scene.content, 'SidebarMenuSubButton')
  assert.ok(row.props.onClick.$do.length)
  assert.deepEqual(unsupported(optedIn), [])
  const contract = JSON.parse(
    extensionScenes([...optedIn, ...readReactStories(root)], scenesJson(shared))
  )
  assert.deepEqual(
    contract.stories.map(story => [
      story.id,
      story.scenes.map(scene => scene.id),
    ]),
    [
      ['menu', ['menu/3', 'menu/4']],
      [
        'number-field',
        [
          'number-field/1',
          'number-field/2',
          'number-field/3',
          'number-field/4',
          'number-field/5',
        ],
      ],
      ['preferences', ['preferences/3']],
      ['sidebar', ['sidebar/3', 'sidebar/4', 'sidebar/5', 'sidebar/6']],
      ['tabs', ['tabs/preview', 'tabs/0', 'tabs/1', 'tabs/2', 'tabs/3']],
      [
        'window-frame',
        [
          'window-frame/preview',
          ...Array.from({ length: 10 }, (_, index) => `window-frame/${index}`),
        ],
      ],
    ]
  )
  const specimens = contract.stories.find(story => story.id === 'sidebar')
    .scenes[0].specimens
  assert.equal(
    specimens.filter(
      specimen => specimen.component === 'sidebar-menu-sub-button'
    ).length,
    3
  )
  assert.equal(
    specimens.find(specimen => specimen.axes.size === 'medium').axes.tint,
    'success'
  )
})

test('GPUI compiles the menu submenu extension with its nested items', async () => {
  const { readStories } = await import('../stories.mjs')
  const stories = await readStories(root, { platform: 'gpui' })
  const story = stories.find(story => story.id === 'menu')
  assert.equal(story.examples.length, 5)
  const menu = find(story.examples.at(-2).scene.content, 'Menu')
  const branch = menu.props.items.find(item => item.label === 'Move to')
  assert.equal(branch.items.length, 2)
  const nested = branch.items.find(item => item.label === 'Projects')
  assert.deepEqual(
    nested.items.map(item => item.label),
    ['DazzUI', 'Native shell']
  )
  assert.ok(menu.props.items.at(-1).disabled)
  assert.deepEqual(unsupported([story]), [])
})

test('GPUI compiles the existing theme swatch example and rejects unknown option fields', async () => {
  const { readStories } = await import('../stories.mjs')
  const stories = await readStories(root, { platform: 'gpui' })
  const example = stories
    .find(story => story.id === 'preferences')
    .examples.at(-1)
  const swatches = find(example.scene.content, 'PreferenceThemeSwatches')
  assert.equal(swatches.props.options.length, 6)
  assert.equal(swatches.props.options[0].theme, '')
  assert.deepEqual(unsupported(stories), [])
  swatches.props.options[0].unmapped = true
  assert.ok(
    unsupported(stories).some(gap =>
      gap.includes('unmapped PreferenceThemeSwatches options')
    )
  )
})

test('GPUI compiles all WindowFrame examples and refuses unsupported chrome data', () => {
  const story = readReactStories(root).find(
    story => story.id === 'window-frame'
  )
  assert.equal(story.examples.length, 10)
  assert.deepEqual(unsupported([story]), [])
  const platforms = story.examples[0].scene.content.children.map(
    node => node.props.platform
  )
  assert.deepEqual(platforms, [
    'macos',
    'macos15',
    'windows',
    'gnome',
    'ubuntu',
    'kde',
    'omarchy',
  ])
  for (const [key, value, kind] of [
    ['platform', 'unknown', 'platform'],
    ['controls', { close: 'sometimes' }, 'controls'],
    ['titlebar', { unknown: true }, 'titlebar'],
    ['minWidth', 'calc(100% - 1px)', 'minWidth'],
  ]) {
    const invalid = structuredClone(story)
    find(invalid.preview.content, 'WindowFrame').props[key] = value
    assert.ok(
      unsupported([invalid]).some(gap =>
        gap.includes(`unmapped window ${kind}`)
      )
    )
  }
})

test('GPUI keeps NumberField modifiers and controlled nullable values in its examples', async () => {
  const { readStories } = await import('../stories.mjs')
  const stories = await readStories(root, { platform: 'gpui' })
  const story = stories.find(story => story.id === 'number-field')
  assert.equal(story.examples.length, 6)
  const controlled = find(story.examples[2].scene.content, 'NumberField')
  assert.equal(controlled.props.smallStep, 0.25)
  assert.equal(controlled.props.largeStep, 20)
  assert.ok(controlled.props.value.$state)
  assert.ok(controlled.props.onValueChange.$do.length)
  const wheel = find(story.examples[4].scene.content, 'NumberField')
  assert.equal(wheel.props.allowWheelScrub, true)
  const committed = find(story.examples[5].scene.content, 'NumberField')
  assert.equal(committed.props.allowOutOfRange, true)
  assert.ok(committed.props.onValueCommitted.$do.length)
  assert.deepEqual(unsupported([story]), [])
})
