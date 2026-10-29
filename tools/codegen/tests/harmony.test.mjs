import assert from 'node:assert/strict'
import fs from 'node:fs'
import test from 'node:test'
import {
  HarmonyReactCompiler,
  readHarmonyReactStories,
} from '../harmony-react.mjs'
import { StoryCompiler } from '../stories.mjs'
import { dateRangeKitSource } from '../harmony-dates.mjs'
import {
  SUPPORT,
  sceneGaps,
  storiesData,
  storiesETS,
  value,
} from '../harmony.mjs'

const renderer = fs.readFileSync(
  new URL(
    '../../../packages/arkts/dazzui/example/entry/src/main/ets/lib/SceneNode.ets',
    import.meta.url
  ),
  'utf8'
)

test('every promised component has a native interpreter branch', () => {
  for (const name of Object.keys(SUPPORT))
    assert.ok(renderer.includes(`this.node.name === '${name}'`), name)
})

test('WindowFrame reads all actual React scenes, scoped slots and application state', async () => {
  const root = new URL('../../../', import.meta.url).pathname
  const source = fs.readFileSync(
    new URL(
      '../../../packages/js/dazzui/example/src/stories/react/window-frame.tsx',
      import.meta.url
    ),
    'utf8'
  )
  const story = (await readHarmonyReactStories(root)).stories.find(
    story => story.id === 'window-frame'
  )
  assert.deepEqual(
    story.examples.map(example => example.title),
    [
      'Platforms',
      'Inactive',
      'Fixed size',
      'Controls',
      'Titlebar slots',
      'Accessory',
      'Application window',
      'Content only',
      'Sizing',
      'Hidden title',
    ]
  )
  assert.equal(story.preview.content.props.width, 280)
  assert.deepEqual(
    story.examples[0].scene.content.children.map(frame => frame.props.platform),
    ['macos', 'macos15', 'windows', 'gnome', 'ubuntu', 'kde', 'omarchy']
  )
  assert.ok(
    story.examples[1].scene.content.children.every(
      frame => frame.props.inactive
    )
  )
  assert.ok(
    story.examples[2].scene.content.children.every(
      frame => frame.props.controls.close === true
    )
  )
  const slots = story.examples[4].scene
  assert.deepEqual(slots.state, { s0: false, s1: false, s2: false })
  for (const [index, frame] of slots.content.children.entries()) {
    assert.equal(
      frame.props.titlebar.toolbar[0].props.pressed.$state,
      `s${index}`
    )
    assert.equal(
      frame.props.titlebar.toolbar[0].props.onPressedChange.$do[0].set,
      `s${index}`
    )
  }
  assert.equal(slots.content.children[1].props.title.$el, 'Row')
  const workspace = story.examples[6].scene
  assert.equal(workspace.content.children.length, 5)
  for (const frame of workspace.content.children) {
    assert.equal(frame.props.height, '24rem')
    assert.equal(frame.props.titlebar, false)
    assert.equal(frame.props.fullSizeContent, true)
    const provider = frame.children[0].children[0]
    assert.equal(provider.$el, 'SidebarProvider')
    assert.equal(provider.children[1].children[0].$el, 'WindowTitlebar')
    assert.equal(provider.children[1].children[2].$el, 'WindowFooter')
  }
  for (const scene of [
    story.preview,
    ...story.examples.map(example => example.scene),
  ])
    assert.deepEqual(sceneGaps(story.id, scene), [])
  const changed = new HarmonyReactCompiler(
    'stories/react/window-frame.tsx',
    source.replace('width={280}', 'width={281}'),
    { kitValues: {} }
  ).story()
  assert.equal(changed.preview.content.props.width, 281)
  assert.throws(
    () =>
      new HarmonyReactCompiler(
        'stories/react/window-frame.tsx',
        source.replace('paddingBlockStart: 0', 'paddingBlockStart: 1'),
        { kitValues: {} }
      ).story(),
    /window-frame\.tsx:\d+:/
  )
  assert.throws(
    () =>
      new StoryCompiler(
        'stories/react/window-frame.tsx',
        source.replace('../../components/stage', '../components/stage'),
        { kitValues: {} }
      ).story(),
    /not portable/
  )
})

test('HarmonyOS story reader compiles the actual React-only Tabs page without changing the portable grammar', async () => {
  const root = new URL('../../../', import.meta.url).pathname
  const source = fs.readFileSync(
    new URL(
      '../../../packages/js/dazzui/example/src/stories/react/tabs.tsx',
      import.meta.url
    ),
    'utf8'
  )
  const result = await readHarmonyReactStories(root)
  const story = result.stories.find(story => story.id === 'tabs')
  assert.equal(story.id, 'tabs')
  assert.deepEqual(
    story.examples.map(example => example.title),
    ['Sections', 'Compact', 'Overflowing', 'Documents']
  )
  assert.equal(story.preview.content.props.width, 520)
  assert.deepEqual(
    story.examples.map(example => example.scene.content.props.width),
    [560, 340, 200, 520]
  )
  const docs = story.examples[3].scene
  const tabs = docs.content.children[0]
  assert.deepEqual(tabs.props.onClose, {
    $do: [{ remove: 's0', key: 'value', match: { $arg: 0 } }],
  })
  assert.equal(docs.state.s1, 'a')
  assert.equal(docs.state.s0[1].busy, true)
  assert.equal(tabs.props.trailing.props.label, 'New')
  for (const scene of [
    story.preview,
    ...story.examples.map(example => example.scene),
  ])
    assert.deepEqual(sceneGaps('tabs', scene), [])
  assert.deepEqual(
    result.pages.filter(page => !page.mapped).map(page => page.id),
    []
  )
  const changed = new HarmonyReactCompiler(
    'tabs.tsx',
    source.replace('width={200}', 'width={184}'),
    { kitValues: {} }
  ).story()
  assert.equal(
    changed.examples[2].scene.content.props.width,
    184,
    'Native data follows the source'
  )
  assert.throws(
    () => new StoryCompiler('tabs.tsx', source, { kitValues: {} }).story(),
    /not portable/
  )
  assert.throws(() =>
    new HarmonyReactCompiler(
      'tabs.tsx',
      source.replace('item.value !== closed', 'item.value === closed'),
      { kitValues: {} }
    ).story()
  )
  assert.throws(
    () =>
      new HarmonyReactCompiler(
        'tabs.tsx',
        source.replace('height: 36', "height: 'auto'"),
        { kitValues: {} }
      ).story(),
    /Unsupported native header/
  )
  for (const [from, to, expected] of [
    ['height: 36', 'height: -1', /tabs\.tsx:\d+:.*Unsupported native header/],
    [
      "paddingInline: 'var(--spacing-3)'",
      "paddingInline: '12px'",
      /tabs\.tsx:\d+:.*Unsupported native header metric/,
    ],
    [
      "background: 'var(--color-surface-chrome)'",
      "background: '#fff'",
      /tabs\.tsx:\d+:.*Unsupported native header color/,
    ],
  ])
    assert.throws(
      () =>
        new HarmonyReactCompiler('tabs.tsx', source.replace(from, to), {
          kitValues: {},
        }).story(),
      expected
    )
})

test('unsupported controls, props, ranges and actions stay explicit', () => {
  for (const content of [
    { $el: 'WindowFrame', props: { platform: 'invalid' } },
    { $el: 'WindowFrame', props: { controls: true } },
    { $el: 'WindowFrame', props: { controls: { close: 'unexpected' } } },
    { $el: 'WindowFrame', props: { controls: { resize: true } } },
    { $el: 'WindowFrame', props: { titlebar: { unexpected: 'x' } } },
    { $el: 'WindowFrame', props: { width: 'calc(100% - 1px)' } },
    { $el: 'DateRangePicker', props: { value: 'tomorrow' } },
    {
      $el: 'DateRangePicker',
      props: { value: { start: { $date: [2026, 9, 16] }, allDay: false } },
    },
    { $el: 'DateRangePicker', props: { timeStep: 0 } },
    { $el: 'DateRangePicker', props: { months: 3 } },
    { $el: 'DateRangePicker', props: { weekStartsOn: 7 } },
    { $el: 'DateRangePicker', props: { labels: { unexpected: 'x' } } },
    {
      $el: 'DateRangePicker',
      props: { durations: [{ label: 'bad', minutes: '60' }] },
    },
    {
      $el: 'DateRangePicker',
      props: {
        presets: [
          {
            label: 'bad',
            range: {
              $dateRange: {
                start: { op: 'number', number: 0 },
                end: { op: 'today' },
              },
            },
          },
        ],
      },
    },
    { $el: 'Calendar', props: { mode: 'multiple' } },
    { $el: 'Calendar', props: { months: 3 } },
    { $el: 'Calendar', props: { size: 'tiny' } },
    { $el: 'Calendar', props: { weekStartsOn: 7 } },
    { $el: 'Calendar', props: { today: '2026-10-16' } },
    { $el: 'Calendar', props: { value: { $date: [2026, '9', 16] } } },
    {
      $el: 'Calendar',
      props: { mode: 'range', value: { start: { $date: [2026, 9, 16] } } },
    },
    { $el: 'Text', children: [{ $get: [{ $state: 's0' }, 'arbitrary'] }] },
    { $el: 'Text', children: [{ $dateText: [{ $state: 's0' }, 'arbitrary'] }] },
    { $el: 'Tree' },
    { $el: 'PreviewCard', props: { trigger: { $el: 'Badge' } } },
    { $el: 'PreviewCard', props: { trigger: { $el: 'Button' }, delay: -1 } },
    { $el: 'PreviewCard', props: { trigger: { $el: 'Button' }, width: '50%' } },
    {
      $el: 'PreviewCard',
      props: { trigger: { $el: 'Button' }, align: { $state: 's0' } },
    },
    {
      $el: 'Text',
      children: [{ $el: 'PreviewCard', props: { trigger: { $el: 'Button' } } }],
    },
    { $el: 'Sidebar', props: { collapsible: 'sheet' } },
    { $el: 'Sidebar', props: { variant: { $state: 's0' } } },
    {
      $el: 'SidebarMenuItem',
      children: [{ $el: 'Text', children: ['Action'] }],
    },
    { $el: 'SidebarMenuButton', props: { tooltip: { $el: 'Badge' } } },
    { $el: 'SidebarMenuButton', props: { size: { $state: 's0' } } },
    { $el: 'SidebarMenuButton', props: { variant: 'custom' } },
    {
      $el: 'SidebarMenuButton',
      props: {
        icon: {
          $el: 'Icon',
          props: { icon: { $icon: 'Book20Regular' }, size: { $state: 's0' } },
        },
      },
    },
    { $el: 'Drawer', props: { side: 'center' } },
    { $el: 'Drawer', props: { title: { $el: 'Badge' } } },
    { $el: 'Drawer', props: { size: 'calc(100% - 1rem)' } },
    { $el: 'Drawer', props: { trigger: { $el: 'Badge' } } },
    { $el: 'DrawerClose', props: { render: { $el: 'Badge' } } },
    {
      $el: 'DrawerClose',
      children: [
        { $el: 'Icon', props: { icon: { $icon: 'Dismiss16Regular' } } },
      ],
    },
    { $el: 'Drawer', props: { footer: { $state: 's0' } } },
    { $el: 'Combobox', props: { options: [], empty: { $el: 'Badge' } } },
    { $el: 'Autocomplete', props: { items: [{ label: 'Inter' }] } },
    {
      $el: 'Select',
      props: {
        options: [
          {
            label: 'A',
            options: [{ label: 'B', options: [{ label: 'C', value: 'c' }] }],
          },
        ],
      },
    },
    { $el: 'Button', props: { madeUp: true } },
    { $el: 'Slider', props: { defaultValue: [] } },
    { $el: 'Slider', props: { defaultValue: [20] } },
    { $el: 'Slider', props: { defaultValue: [20, '80'] } },
    { $el: 'Slider', props: { min: 100, max: 20 } },
    { $el: 'Slider', props: { step: 0 } },
    { $el: 'Slider', props: { size: 'tiny' } },
    { $el: 'ToastViewport' },
    { $el: 'ToastProvider', props: { limit: -1 } },
    {
      $el: 'ToastProvider',
      children: [{ $el: 'ToastViewport', props: { placement: 'left' } }],
    },
    {
      $el: 'ToastProvider',
      children: [
        {
          $el: 'Button',
          props: { onClick: { $do: [{ toast: { title: { $el: 'Badge' } } }] } },
        },
      ],
    },
    {
      $el: 'ToastProvider',
      children: [
        {
          $el: 'Button',
          props: {
            onClick: { $do: [{ toast: { title: 'Saved', timeout: -1 } }] },
          },
        },
      ],
    },
    {
      $el: 'ToastProvider',
      children: [
        {
          $el: 'Button',
          props: {
            onClick: {
              $do: [{ toast: { title: 'Saved', action: { label: 'Undo' } } }],
            },
          },
        },
      ],
    },
    {
      $el: 'Button',
      props: { onClick: { $do: [{ toast: { title: 'Saved' } }] } },
    },
    {
      $el: 'Select',
      props: { options: [{ label: { $el: 'Badge' }, value: 'x' }] },
    },
  ])
    assert.ok(sceneGaps('test', { content }).length)
  assert.deepEqual(
    sceneGaps('live', {
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

test('HarmonyOS reads every actual DateRangePicker example and seals its data presets', async () => {
  const root = new URL('../../../', import.meta.url).pathname
  const source = fs.readFileSync(
    new URL(
      '../../../packages/js/dazzui/example/src/stories/date-range-picker.tsx',
      import.meta.url
    ),
    'utf8'
  )
  const kit = dateRangeKitSource(
    fs.readFileSync(
      new URL(
        '../../../packages/js/dazzui/src/components/date-range-picker.tsx',
        import.meta.url
      ),
      'utf8'
    ),
    'components/date-range-picker.tsx',
    HarmonyReactCompiler
  )
  const compile = source =>
    new HarmonyReactCompiler('stories/date-range-picker.tsx', source, {
      kitValues: { DATE_RANGE_PRESETS: kit.presets },
    }).story()
  const story = (await readHarmonyReactStories(root)).stories.find(
    story => story.id === 'date-range-picker'
  )
  assert.deepEqual(
    story.examples.map(example => example.title),
    [
      'A meeting',
      'Several days, and all day',
      'Empty, and in a form',
      'The product’s own presets and durations',
      'Sizes and states',
    ]
  )
  assert.deepEqual(story.preview.state.s0, {
    start: { $date: [2026, 9, 16, 14, 30] },
    end: { $date: [2026, 9, 16, 15, 30] },
    allDay: false,
  })
  assert.equal(story.examples[1].scene.content.props.width, '24rem')
  assert.equal(story.examples[1].scene.content.children[0].props.months, 2)
  assert.equal(story.examples[2].scene.content.children[1].props.disabled, true)
  const custom = story.examples[3].scene.content.props
  assert.deepEqual(
    custom.presets.map(preset => preset.label),
    ['Today', 'This sprint', 'Next sprint']
  )
  // A preset that is data becomes the same sealed formula the interpreter
  // already evaluates for a callback: whole-day offsets from today.
  assert.deepEqual(custom.presets[0].range.$dateRange, {
    start: { op: 'today' },
    end: { op: 'today' },
  })
  assert.deepEqual(custom.presets[1].range.$dateRange, {
    start: { op: 'today' },
    end: {
      op: 'addDays',
      args: [{ op: 'today' }, { op: 'number', number: 13 }],
    },
  })
  assert.deepEqual(custom.presets[2].range.$dateRange.start, {
    op: 'addDays',
    args: [{ op: 'today' }, { op: 'number', number: 14 }],
  })
  assert.deepEqual(custom.durations, [{ label: 'Working day', minutes: 480 }])
  assert.equal(custom.timeStep, 30)
  for (const scene of [
    story.preview,
    ...story.examples.map(example => example.scene),
  ])
    assert.deepEqual(sceneGaps('date-range-picker', scene), [])
  const changed = compile(source.replace('days: [0, 13]', 'days: [0, 12]'))
  assert.notDeepEqual(
    changed.examples[3].scene.content.props.presets[1],
    custom.presets[1],
    'Native formulas follow the source data'
  )
  for (const replacement of [
    'days: [0, 13, 27]',
    "days: [0, '13']",
    'days: [0, Math.random()]',
  ])
    assert.throws(
      () => compile(source.replace('days: [0, 13]', replacement)),
      /date-range-picker\.tsx:\d+:/
    )
})

test('HarmonyOS reads the Calendar source dates, ranges, notes and tints without duplicating its fixtures', async () => {
  const root = new URL('../../../', import.meta.url).pathname
  const source = fs.readFileSync(
    new URL(
      '../../../packages/js/dazzui/example/src/stories/calendar.tsx',
      import.meta.url
    ),
    'utf8'
  )
  const result = await readHarmonyReactStories(root)
  const story = result.stories.find(story => story.id === 'calendar')
  assert.deepEqual(
    story.examples.map(example => example.title),
    ['One day', 'A span', 'Two months, small', 'Bounds and week start', 'Tints']
  )
  assert.deepEqual(story.preview.state.s0, { $date: [2026, 9, 20] })
  assert.deepEqual(story.examples[1].scene.state.s0, {
    start: { $date: [2026, 9, 20] },
    end: { $date: [2026, 9, 24] },
  })
  assert.deepEqual(story.examples[1].scene.content.props.onValueChange, {
    $do: [{ set: 's0', to: { $arg: 0 } }],
  })
  assert.equal(story.examples[2].scene.content.props.months, 2)
  assert.equal(story.examples[2].scene.content.props.size, 'small')
  assert.equal(
    story.examples[3].scene.content.children[1].props.weekStartsOn,
    0
  )
  assert.deepEqual(
    story.examples[4].scene.content.children.map(node => node.props.tint),
    ['primary', 'success', 'danger']
  )
  for (const scene of [
    story.preview,
    ...story.examples.map(example => example.scene),
  ])
    assert.deepEqual(sceneGaps('calendar', scene), [])
  assert.deepEqual(
    result.pages.map(page => page.id),
    ['calendar', 'date-range-picker', 'tabs', 'window-frame'],
    'The pages the native reader compiles, portable or React-only'
  )
  const changed = new HarmonyReactCompiler(
    'stories/calendar.tsx',
    source.replace('new Date(2026, 9, 20)', 'new Date(2026, 10, 23)'),
    { kitValues: {} }
  ).story()
  assert.deepEqual(changed.preview.state.s0, { $date: [2026, 10, 23] })
  for (const [from, to] of [
    ['new Date(2026, 9, 16)', 'new Date()'],
    ['new Date(2026, 9, 16)', 'new URL(2026, 9, 16)'],
    ['new Date(2026, 9, 16)', 'new Date(99999999999, 9, 16)'],
  ])
    assert.throws(
      () =>
        new HarmonyReactCompiler(
          'stories/calendar.tsx',
          source.replace(from, to),
          { kitValues: {} }
        ).story(),
      /calendar\.tsx:\d+:/
    )
  // The same source is portable now: every other platform reads its dates as
  // local civil time, and HarmonyOS reads the same days as integer parts.
  const portable = new StoryCompiler('stories/calendar.tsx', source, {
    kitValues: {},
  }).story()
  assert.equal(portable.preview.state.s0, '2026-10-20T00:00')
  assert.deepEqual(portable.examples[1].scene.state.s0, {
    start: '2026-10-20T00:00',
    end: '2026-10-24T00:00',
  })
})

test('the typed wire format preserves live state, expressions, actions and specimen addresses', () => {
  const node = value({
    $el: 'Switch',
    $at: 7,
    props: {
      checked: { $eq: [{ $state: 's0' }, 'on'] },
      onCheckedChange: { $do: [{ set: 's0', to: { $arg: 0 } }] },
    },
  })
  assert.equal(node.address, 7)
  assert.equal(node.fields.checked.items[0].kind, 'state')
  assert.equal(node.fields.onCheckedChange.items[0].fields.to.kind, 'argument')
  assert.deepEqual(value(null), { kind: 'null' })
})

test('range sliders preserve endpoint arrays and controlled actions in the wire format', () => {
  const content = {
    $el: 'Slider',
    props: {
      value: { $state: 's0' },
      defaultValue: [20, 80],
      showValue: true,
      onValueChange: { $do: [{ set: 's0', to: { $arg: 0 } }] },
    },
  }
  assert.deepEqual(sceneGaps('range', { state: { s0: [20, 80] }, content }), [])
  const node = value(content)
  assert.deepEqual(
    node.fields.defaultValue.items.map(item => item.number),
    [20, 80]
  )
  assert.equal(node.fields.value.kind, 'state')
  assert.equal(node.fields.onValueChange.items[0].fields.to.kind, 'argument')
})

test('toast provider scopes preserve nested actions without authorizing a sibling outside the scope', () => {
  const action = {
    $do: [
      {
        toast: {
          title: 'Archived',
          action: {
            label: 'Undo',
            onClick: { $do: [{ set: 's0', to: 'restored' }] },
          },
          onClose: { $do: [] },
        },
      },
    ],
  }
  const content = {
    $el: 'ToastProvider',
    children: [
      { $el: 'Button', props: { onClick: action } },
      { $el: 'ToastViewport', props: { placement: 'top' } },
    ],
  }
  assert.deepEqual(
    sceneGaps('toast', { state: { s0: 'archived' }, content }),
    []
  )
  const wire = value(content)
  assert.equal(
    wire.items[0].fields.onClick.items[0].fields.toast.fields.action.fields
      .onClick.kind,
    'actions'
  )
  assert.ok(
    sceneGaps('outside', {
      content: {
        $el: 'Col',
        children: [content, { $el: 'Button', props: { onClick: action } }],
      },
    }).some(gap => gap.includes('requires a ToastProvider'))
  )
})

test('tree mapping preserves hierarchy, empty branches, icons and badges', () => {
  const items = [
    {
      id: 'folder',
      label: 'Files',
      children: [
        { id: 'empty', label: 'Empty', children: [] },
        {
          id: 'leaf',
          label: 'tree.tsx',
          icon: {
            $el: 'Icon',
            props: {
              icon: { $icon: 'Document20Regular' },
              size: 20,
            },
          },
          meta: { $el: 'Badge', props: { size: 'small' }, children: ['M'] },
        },
      ],
    },
  ]
  assert.deepEqual(
    sceneGaps('tree', { content: { $el: 'Tree', props: { items } } }),
    []
  )
  for (const bad of [
    [{ id: 'rich', label: { $el: 'Badge', children: ['Files'] } }],
    [{ id: 'icon', label: 'Icon', icon: { $el: 'Button' } }],
    [{ id: 'child', label: 'Child', children: { $state: 's0' } }],
    [{ id: 'unknown', label: 'Unknown', href: '/' }],
  ])
    assert.ok(
      sceneGaps('tree', { content: { $el: 'Tree', props: { items: bad } } })
        .length
    )
})

test('generated stories keep previews separate, scene ids, notes and initial state', () => {
  const story = {
    id: 'button',
    name: 'Button',
    summary: 'Press',
    preview: { content: { $el: 'Button', children: ['Save'] } },
    examples: [
      {
        title: 'Live',
        note: 'A note',
        scene: {
          state: { s0: false },
          content: { $el: 'Switch', props: { checked: { $state: 's0' } } },
        },
      },
    ],
  }
  const [output] = storiesData([story])
  assert.equal(output.preview.id, 'button/preview')
  assert.equal(output.examples[0].id, 'button/0')
  assert.equal(output.examples[0].note, 'A note')
  assert.equal(output.examples[0].initial.s0.boolean, false)
  assert.deepEqual(output.examples[0].gaps, [])
  assert.match(storiesETS([story]), /export const STORIES: Story\[\]/)
})

test('the restored families map every real React preview and example', async () => {
  const { readStories } = await import('../stories.mjs')
  const { fileURLToPath } = await import('node:url')
  const stories = await readStories(
    fileURLToPath(new URL('../../../', import.meta.url))
  )
  const ids = [
    'accordion',
    'avatar',
    'code',
    'link',
    'pill-tabs',
    'toggle',
    'checkbox',
    'menu',
    'popover',
    'preferences',
    'progress',
    'spinner',
    'form',
    'dialog',
    'select',
    'segmented-control',
    'combobox',
    'icon',
    'drawer',
    'switch',
    'shortcut-recorder',
    'otp-field',
    'tree',
    'sidebar',
    'preview-card',
    'slider',
    'toast',
  ]
  for (const story of storiesData(stories).filter(story =>
    ids.includes(story.id)
  )) {
    for (const scene of [story.preview, ...story.examples])
      assert.deepEqual(scene.gaps, [], scene.id)
  }
  assert.equal(
    stories.filter(story => ids.includes(story.id)).length,
    ids.length
  )
})
