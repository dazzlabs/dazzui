import assert from 'node:assert/strict'
import { test } from 'node:test'
import { fileURLToPath } from 'node:url'
import { SUPPORT, unsupported } from '../flutter.mjs'
import * as swiftui from '../swiftui.mjs'
import * as winui from '../winui.mjs'
import { scenesJson } from '../scenes.mjs'
import { readStories, StoryCompiler } from '../stories.mjs'

const root = fileURLToPath(new URL('../../../', import.meta.url))
const context = { kitValues: { TINTS: ['primary', 'danger'] } }

/** Compiles one story file's source. */
const compile = body =>
  new StoryCompiler(
    'stories/probe.tsx',
    `import { Button, Switch, Tag, TINTS, useToast } from '@dazzlabs/dazzui'
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
  assert.deepEqual(winui.unsupported(stories), [])
  for (const { id, name, examples } of stories) {
    assert.match(id, /^[a-z][a-z0-9-]*$/)
    assert.ok(name && examples.length, id)
  }
})

test('theme swatches compile only for the native ports that opt in', async () => {
  const shared = await readStories(root)
  const preferences = stories =>
    stories.find(story => story.id === 'preferences')
  const original = preferences(shared)
  assert.ok(
    !original.examples.some(example => example.title === 'Choosing a theme')
  )
  for (const [platform, check] of [
    ['flutter', unsupported],
    ['swiftui', swiftui.unsupported],
    ['winui', winui.unsupported],
  ]) {
    const stories = await readStories(root, { platform })
    assert.deepEqual(check(stories), [])
    const native = preferences(stories)
    assert.equal(native.examples.length, original.examples.length + 1)
    const scene = native.examples.at(-1).scene
    const swatch = scene.content.children[0].children[0].children[0].children[0]
    assert.equal(swatch.$el, 'PreferenceThemeSwatches')
    assert.equal(swatch.props.options.length, 6)
    assert.equal(swatch.props.options[0].theme, '')
    assert.equal(swatch.props.defaultValue, 'bright-light')
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

test('a date is constructed ahead of time and written as local text', () => {
  const { preview } = compile(`
const TODAY = new Date(2026, 9, 16)
const at = (day: number, hours: number, minutes = 0) => new Date(2026, 9, day, hours, minutes)
function Pick() {
  const [day, setDay] = useState<Date | null>(at(20, 9, 30))
  return <Tag selected={day === TODAY} onSelectedChange={setDay}>{\`\${TODAY}\`}</Tag>
}
${story('Pick')}`)
  assert.deepEqual(preview.state, { s0: '2026-10-20T09:30' })
  assert.deepEqual(preview.content.props.selected, {
    $eq: [{ $state: 's0' }, '2026-10-16T00:00'],
  })
  assert.throws(
    () => compile(story(`() => <Tag>{\`\${new Date()}\`}</Tag>`)),
    /each a number known ahead of time/
  )
  assert.throws(
    () => compile(story(`() => <Tag>{\`\${new Map()}\`}</Tag>`)),
    /Only new Date/
  )
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

test('WinUI accepts composed table props and rejects unknown row props', () => {
  const row = {
    $el: 'TableRow',
    props: { active: true, interactive: true, onClick: { $do: [] } },
    children: [
      { $el: 'TableCell', props: { align: 'end' }, children: ['Ready'] },
    ],
  }
  const stories = [
    {
      id: 'table',
      preview: {
        content: {
          $el: 'Table',
          props: { tint: 'danger', 'aria-label': 'Deploys' },
          children: [
            {
              $el: 'TableHead',
              children: [
                {
                  $el: 'TableCell',
                  props: { head: true },
                  children: ['State'],
                },
              ],
            },
            row,
          ],
        },
      },
      examples: [],
    },
  ]
  assert.deepEqual(winui.unsupported(stories), [])
  row.props.wobble = true
  assert.deepEqual(winui.unsupported(stories), [
    'table preview: <TableRow wobble> is not drawn by WinUI',
  ])
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

test('a WinUI story is a lazy member whose JSON sits in a raw string', () => {
  const story = {
    id: 'icon-button',
    name: 'IconButton',
    summary: 'Says """quoted""" things.',
    preview: { content: 'x' },
    examples: [],
  }
  const cs = winui.storyCs(story)
  assert.match(
    cs,
    /public static Story IconButton => iconButtonStory \?\?= Story\.Parse\(/
  )
  // JSON escapes every quote in a string, so the shortest fence holds.
  assert.match(cs, /\n {8}"""\n/)
  assert.ok(cs.includes('\\"\\"\\"quoted'))
  assert.equal(winui.storyFile('icon-button'), 'IconButton.cs')
  assert.match(winui.storiesIndexCs([story]), / {8}IconButton,/)
  const glyphs = winui.glyphsCs([
    { preview: { content: { $icon: 'TextBold20Filled' } } },
  ])
  assert.match(
    glyphs,
    /\["TextBold20Filled"\] = \(Glyph\.TextBold, IconVariant\.Filled\),/
  )
  assert.throws(() => winui.glyphOf('Settings24Regular'), /20px grid/)
})

test('scenes are addressed pre-order, at their initial state', () => {
  const compiled = compile(`
const story: Story = {
  id: 'probe',
  name: 'Probe',
  summary: 'A probe.',
  Preview: () => {
    const [on, setOn] = useState(false)
    return (
      <Row>
        <Button tint="danger" shortcut={<Tag>S</Tag>}>
          <Settings20Regular />
          Save
        </Button>
        {on ? <Tag>On</Tag> : <Switch checked={on} onCheckedChange={setOn} />}
      </Row>
    )
  },
  examples: [],
}`)
  const [scene] = JSON.parse(scenesJson([compiled])).stories[0].scenes
  assert.equal(scene.id, 'probe/preview')
  assert.deepEqual(scene.specimens, [
    {
      address: 'probe/preview/0',
      component: 'button',
      axes: { tint: 'danger' },
      text: 'Save',
      container: true,
    },
    { address: 'probe/preview/1', component: 'tag', axes: {}, text: 'S' },
    { address: 'probe/preview/2', component: 'switch', axes: {} },
  ])
})
