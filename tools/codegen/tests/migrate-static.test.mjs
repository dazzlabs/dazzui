import test from 'node:test'
import assert from 'node:assert/strict'
import { extractStaticStories } from '../migrate-static.mjs'

test('pure templates expand explicit props and reject effects and recursive content', () => {
  const source = `function Line({ label, value = 'Idle' }) { return <Row><span>{label}</span><Badge>{value}</Badge></Row> }
    export const A = { render: () => <Col><Line label="DeepL" value="Connected" /><Line label="Google" /></Col> }`
  const { scenes } = extractStaticStories(source, 'line.tsx')
  assert.deepEqual(
    scenes.A.content.children.map(n => n.children[1].children),
    [['Connected'], ['Idle']]
  )
  assert.throws(
    () =>
      extractStaticStories(
        source.replace('return <Row>', "console.log('effect'); return <Row>"),
        'line.tsx'
      ),
    /single pure JSX return/
  )
  assert.throws(
    () =>
      extractStaticStories(
        `function Again({label}) { return <Again label={label} /> }; export const A = {render: () => <Again label="A"/>}`,
        'recursive.tsx'
      ),
    /recursive template/
  )
  assert.throws(
    () =>
      extractStaticStories(
        source.replace(
          '<Line label="Google" />',
          '<Line label="Google" extra="unrepresented" />'
        ),
        'line.tsx'
      ),
    /unconsumed template prop/
  )
})

test('inline typography and container height keep tokens and logical pixels', () => {
  const { scenes } = extractStaticStories(
    `export const A = {render: () => <div style={{height: '1.5rem', display: 'flex', gap: 'var(--spacing-3)'}}><span style={{fontSize: 'var(--title-large-font-size)', fontFamily: 'var(--base-font-code)'}}>392</span></div>}`,
    'type.tsx'
  )
  assert.equal(scenes.A.content.props.height, 24)
  assert.deepEqual(scenes.A.content.children[0], {
    component: 'Inline',
    props: { fontSize: 'title.large', mono: true },
    children: ['392'],
  })
})

test('grid minimum column width retains rem geometry as logical pixels', () => {
  const { scenes } = extractStaticStories(
    'export const A = { render: () => <Grid min="18rem"><Card /></Grid> }',
    'grid.tsx'
  )
  assert.deepEqual(scenes.A.content, {
    component: 'Grid',
    props: { min: 288 },
    children: [{ component: 'Card' }],
  })
})

test('named slots preserve fixture content and lift nested editable fields', () => {
  const { scenes } = extractStaticStories(
    'const fixture = { title: "Search", initial: "Ada" }; export const A = { render: () => <EmptyState title={`${fixture.title} results`} action={<TextField defaultValue={fixture.initial} />} /> }',
    'slots.stories.tsx'
  )
  assert.equal(scenes.A.content.props.title, 'Search results')
  assert.deepEqual(scenes.A.state, { 'field-slot-action': 'Ada' })
  assert.deepEqual(scenes.A.content.slots.action.bind, {
    value: 'field-slot-action',
  })
})

test('inline chords preserve separate flex items and token spacing', () => {
  const { scenes } = extractStaticStories(
    `export const Chord = { render: () => <Row gap="var(--spacing-5)"><span style={{ display: 'inline-flex', alignItems: 'center', gap: 'var(--spacing-1)', color: 'var(--color-content-muted)' }}><KeyCap>Ctrl</KeyCap>+<KeyCap>K</KeyCap></span></Row> }`,
    'chord.stories.tsx'
  )
  assert.equal(scenes.Chord.content.props.gap, 'spacing.5')
  const chord = scenes.Chord.content.children[0]
  assert.equal(chord.component, 'Box')
  assert.deepEqual(chord.props, {
    display: 'inline-flex',
    align: 'center',
    gap: 'spacing.1',
    tone: 'muted',
  })
  assert.deepEqual(chord.children, [
    { component: 'KeyCap', children: ['Ctrl'] },
    '+',
    { component: 'KeyCap', children: ['K'] },
  ])
})

test('static import expands fixtures while preserving text and layout values', () => {
  const { scenes } = extractStaticStories(
    `
    const labels = ['One', 'Two'] as const
    export const List = {render: () => <Col width="18rem">{labels.map(label => <Button key={label}>{label}</Button>)}</Col>}
  `,
    'fixture.stories.tsx'
  )
  assert.deepEqual(scenes.List.content, {
    component: 'Col',
    props: { width: 288 },
    children: ['One', 'Two'].map(text => ({
      component: 'Button',
      children: [text],
    })),
  })
})
test('static import never embeds imperative callbacks or unknown calls', () => {
  assert.throws(
    () =>
      extractStaticStories(
        'export const A = { render: () => { useEffect(() => fetch("/x")); return <Button/> } }',
        'a.tsx'
      ),
    /single pure JSX return/
  )
  assert.throws(
    () =>
      extractStaticStories(
        'export const A = { render: () => <Button onClick={() => fetch("/x")}>Go</Button> }',
        'a.tsx'
      ),
    /setter|handler|Unsupported static syntax/
  )
  assert.throws(
    () =>
      extractStaticStories(
        'export const A = { render: () => <Button>{loadRemoteData()}</Button> }',
        'a.tsx'
      ),
    /Unsupported static syntax/
  )
})
test('JSX whitespace and explicit spaces remain visible content', () => {
  const { scenes } = extractStaticStories(
    `export const A = { render: () => <Inline>Text:{' '}<KeyCap>⌘K</KeyCap></Inline> }`,
    'a.tsx'
  )
  assert.deepEqual(scenes.A.content.children, [
    'Text:',
    ' ',
    { component: 'KeyCap', children: ['⌘K'] },
  ])
})

test('selected input stories lift independent defaults into editable shared state', () => {
  const { scenes, replacements } = extractStaticStories(
    `
    export const Fields = { render: () => <Col><TextField defaultValue="Ada" /><TextField placeholder="Search" /></Col> }
    export const Later = { render: () => { return <Unknown /> } }
  `,
    'input.stories.tsx',
    { exports: ['Fields'] }
  )
  assert.deepEqual(scenes.Fields.state, { 'field-0': 'Ada', 'field-1': '' })
  assert.deepEqual(scenes.Fields.content.children[0].bind, { value: 'field-0' })
  assert.equal(scenes.Fields.content.children[0].props.defaultValue, undefined)
  assert.equal(scenes.Fields.content.children[1].props.placeholder, 'Search')
  assert.equal(replacements.length, 1)
})

test('host containers preserve dimensions and token references without raw CSS', () => {
  const { scenes } = extractStaticStories(
    `
    const frame = { width: '22.5rem', padding: 'var(--spacing-2)', background: 'var(--color-surface-chrome)', borderRadius: 'var(--radius-medium)' }
    export const Framed = { render: () => <ActionBar style={frame}><Button>Save</Button></ActionBar> }
  `,
    'framed.stories.tsx'
  )
  assert.deepEqual(scenes.Framed.content.props, {
    width: 360,
    padding: 'spacing.2',
    surface: 'color.surface.chrome',
    radius: 'radius.medium',
  })
  assert.equal(scenes.Framed.content.component, 'Box')
  assert.equal(scenes.Framed.content.children[0].component, 'ActionBar')
  assert.throws(
    () =>
      extractStaticStories(
        `export const A = { render: () => <div style={{ background: '#fff' }}><Button>A</Button></div> }`,
        'a.tsx'
      ),
    /Unsupported scene style/
  )
})

test('component-local state becomes declared state, bindings, derived props and events', () => {
  const source = `
    function Picker({ tint }) {
      const [theme, setTheme] = useState('system')
      return <SegmentedControl aria-label="Appearance" tint={tint} value={theme} onValueChange={setTheme}>
        <SegmentedItem value="light" label="Light" /><SegmentedItem value="system" label="System" />
      </SegmentedControl>
    }
    export const Two = { render: () => <Col><Picker tint="danger" /><Picker /></Col> }
    export const Flag = { render: () => {
      const [on, setOn] = useState(false)
      return <Row>
        <Switch aria-label="Sync" checked={on} onCheckedChange={setOn} />
        <Button disabled={!on} onClick={() => setOn(false)}>Reset</Button>
        <Button onClick={() => {}}>Nothing</Button>
        <span>{on ? 'On' : 'Off'}</span>
      </Row>
    } }`
  const { scenes } = extractStaticStories(source, 'state.stories.tsx')
  assert.deepEqual(scenes.Two.state, { theme: 'system', 'theme-2': 'system' })
  assert.deepEqual(
    scenes.Two.content.children.map(c => c.bind),
    [{ value: 'theme' }, { value: 'theme-2' }]
  )
  assert.equal(scenes.Two.content.children[0].props.label, 'Appearance')
  const [toggle, reset, nothing, text] = scenes.Flag.content.children
  assert.deepEqual(toggle.bind, { checked: 'on' })
  assert.deepEqual(reset.derive, { disabled: { not: { state: 'on' } } })
  assert.deepEqual(reset.on, { press: { set: 'on', value: false } })
  assert.equal(nothing.on, undefined)
  assert.deepEqual(text.children[0].value, {
    if: { state: 'on' },
    // oxlint-disable-next-line unicorn/no-thenable -- scene branch data
    then: 'On',
    else: 'Off',
  })
  assert.throws(
    () =>
      extractStaticStories(
        `export const A = { render: () => { const [v, setV] = useState('a'); return <SegmentedControl value="a" onValueChange={setV}><SegmentedItem value="a" label="A" /></SegmentedControl> } }`,
        'unpaired.tsx'
      ),
    /must pair with a controlled value/
  )
})
