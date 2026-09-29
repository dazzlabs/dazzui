import assert from 'node:assert/strict'
import { test } from 'node:test'
import { fileURLToPath } from 'node:url'
import { SUPPORT, unsupported } from '../flutter.mjs'
import * as swiftui from '../swiftui.mjs'
import { readStories, StoryCompiler } from '../stories.mjs'

const root = fileURLToPath(new URL('../../../', import.meta.url))
const context = { kitValues: { TINTS: ['primary', 'danger'] } }

/** Compiles one story file's source. */
const compile = body =>
  new StoryCompiler(
    'stories/probe.tsx',
    `import { Button, Switch, Tag, TINTS, useToast } from '@dazzlabs/dazzui-react'
import { Settings20Regular } from '@fluentui/react-icons'
import { useState } from 'react'
import { Col, Row, Text } from '../components/stage'
import type { Story } from '../lib/story'
${body}
export default story
`,
    context
  ).story()

const story = preview => `const story: Story = {
  id: 'probe',
  name: 'Probe',
  summary: 'A probe.',
  Preview: ${preview},
  examples: [],
}`

test('every React story compiles, and Flutter draws all of it', async () => {
  const stories = await readStories(root)
  assert.ok(stories.length > 0)
  assert.deepEqual(unsupported(stories), [])
  assert.deepEqual(swiftui.unsupported(stories), [])
  for (const { id, name, examples } of stories) {
    assert.match(id, /^[a-z][a-z0-9-]*$/)
    assert.ok(name && examples.length, id)
  }
})

test('constant lists are unrolled ahead of time', () => {
  const { preview } = compile(
    story(
      `() => <Row>{TINTS.map(tint => <Button key={tint} tint={tint}>{\`A \${tint}\`}</Button>)}</Row>`
    )
  )
  assert.deepEqual(preview.content, {
    $el: 'Row',
    children: [
      { $el: 'Button', props: { tint: 'primary' }, children: ['A primary'] },
      { $el: 'Button', props: { tint: 'danger' }, children: ['A danger'] },
    ],
  })
})

test('a helper component is called with its props, each call its own slot', () => {
  const { preview } = compile(`
function Filter({ name, on = false }: { name: string; on?: boolean }) {
  const [selected, setSelected] = useState(on)
  return <Tag selected={selected} onSelectedChange={setSelected}>{name}</Tag>
}
${story(`() => <Row><Filter name="A" on /><Filter name="B" /></Row>`)}`)
  assert.deepEqual(preview.state, { s0: true, s1: false })
  assert.deepEqual(preview.content.children[1], {
    $el: 'Tag',
    props: {
      selected: { $state: 's1' },
      onSelectedChange: { $do: [{ set: 's1', to: { $arg: 0 } }] },
    },
    children: ['B'],
  })
})

test('a comparison against a slot stays live', () => {
  const { preview } = compile(`
function Pick() {
  const [plan, setPlan] = useState('a')
  return <Button variant={plan === 'b' ? 'filled' : 'plain'} onClick={() => setPlan('b')}>B</Button>
}
${story('Pick')}`)
  assert.deepEqual(preview.content.props, {
    variant: { $if: [{ $eq: [{ $state: 's0' }, 'b'] }, 'filled', 'plain'] },
    onClick: { $do: [{ set: 's0', to: 'b' }] },
  })
})

test('a toast is an action, and a glyph is named', () => {
  const { preview } = compile(`
function Save() {
  const toast = useToast()
  return <Button onClick={() => toast.add({ title: 'Saved', tint: 'success' })}><Settings20Regular /></Button>
}
${story('Save')}`)
  assert.deepEqual(preview.content.props.onClick, {
    $do: [{ toast: { title: 'Saved', tint: 'success' } }],
  })
  assert.deepEqual(preview.content.children, [{ $icon: 'Settings20Regular' }])
})

test('JSX text follows the JSX whitespace rule', () => {
  const { preview } = compile(
    story(`() => (
    <Text>
      Run the
      build   first.
    </Text>
  )`)
  )
  assert.deepEqual(preview.content.children, ['Run the build   first.'])
})

test('HTML and inline style are refused, naming the line', () => {
  assert.throws(
    () => compile(story('() => <div>no</div>')),
    /stories\/probe\.tsx:\d+:\d+: <div> is not portable/
  )
  assert.throws(
    () => compile(story(`() => <Button style={{ width: 1 }}>no</Button>`)),
    /style is not portable/
  )
})

test('a handler that does more than set or toast is refused', () => {
  assert.throws(
    () =>
      compile(`
function Bad() {
  const [n, setN] = useState(0)
  return <Button onClick={() => { console.log(n) }}>x</Button>
}
${story('Bad')}`),
    /An event handler may only set state or raise a toast|Unknown name: console/
  )
})

test('an import from outside the kit, the icons or the stage is refused', () => {
  assert.throws(
    () =>
      new StoryCompiler(
        'stories/probe.tsx',
        `import { x } from 'lodash'\nexport default {}`,
        context
      ).story(),
    /Stories may not import from lodash/
  )
})

test('Flutter reports a prop it does not draw', () => {
  const gaps = unsupported([
    {
      id: 'probe',
      preview: { content: { $el: 'Button', props: { wobble: true } } },
      examples: [],
    },
  ])
  assert.deepEqual(gaps, [
    'probe preview: <Button wobble> is not drawn by Flutter',
  ])
  assert.ok(SUPPORT.Button.includes('onClick'))
})

test('a SwiftUI story member that is a keyword is fenced, and its JSON is raw', () => {
  const story = {
    id: 'switch',
    name: 'Switch',
    summary: 'A "quoted" $ummary \\ with a backslash.',
    preview: { content: 'x' },
    examples: [],
  }
  const swift = swiftui.storySwift(story)
  assert.match(swift, /static let `switch` = Story\(/)
  assert.match(swift, /#"""\n/)
  assert.ok(
    swift.includes('"summary": "A \\"quoted\\" $ummary \\\\ with a backslash."')
  )
  assert.equal(swiftui.storyFile('icon-button'), 'IconButton.swift')
  assert.match(swiftui.storiesIndexSwift([story]), /    `switch`,/)
})
