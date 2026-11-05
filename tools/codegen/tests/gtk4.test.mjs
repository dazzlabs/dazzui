import assert from 'node:assert/strict'
import test from 'node:test'
import {
  sceneGaps,
  storyJSON,
  storyRust,
  storiesIndexRust,
  themesRust,
} from '../gtk4.mjs'

test('toast actions retain their provider scope and reject unsupported values', () => {
  const action = {
    $do: [
      {
        toast: {
          title: { $state: 'message' },
          action: {
            label: 'Undo',
            onClick: { $do: [{ set: 'message', to: 'Restored' }] },
          },
          onClose: { $do: [] },
        },
      },
    ],
  }
  const provider = {
    $el: 'ToastProvider',
    children: [
      { $el: 'Button', props: { onClick: action } },
      { $el: 'ToastViewport', props: { placement: 'top' } },
    ],
  }
  assert.deepEqual(sceneGaps('toast/0', { content: provider }), [])
  assert.match(
    sceneGaps('toast/0', {
      content: {
        ...provider,
        children: [
          { $el: 'Button' },
          { $el: 'Button' },
          { $el: 'ToastViewport' },
        ],
      },
    }).join('\n'),
    /authored Row or Col/
  )
  assert.match(
    sceneGaps('toast/0', {
      content: {
        $el: 'Row',
        children: [provider, { $el: 'Button', props: { onClick: action } }],
      },
    }).join('\n'),
    /requires a ToastProvider/
  )
  for (const options of [
    { title: { $el: 'Badge' } },
    { title: 'Saved', timeout: -1 },
    { title: 'Saved', action: { label: 'Undo' } },
    { title: 'Saved', unknown: true },
  ])
    assert.match(
      sceneGaps('toast/0', {
        content: {
          ...provider,
          children: [
            {
              $el: 'Button',
              props: { onClick: { $do: [{ toast: options }] } },
            },
            { $el: 'ToastViewport' },
          ],
        },
      }).join('\n'),
      /invalid toast options/
    )
  assert.match(
    sceneGaps('toast/0', {
      content: {
        $el: 'ToastProvider',
        children: [{ $el: 'ToastViewport', props: { placement: 'left' } }],
      },
    }).join('\n'),
    /toast placement/
  )
  assert.match(
    sceneGaps('toast/0', {
      content: {
        $el: 'ToastProvider',
        props: { limit: -1 },
        children: [{ $el: 'ToastViewport' }],
      },
    }).join('\n'),
    /toast limit/
  )
})

test('maps edge drawers and rejects unimplemented layout or state props', () => {
  const content = {
    $el: 'Drawer',
    props: {
      trigger: { $el: 'Button', children: ['Open'] },
      title: 'Filters',
      description: 'What the list leaves out.',
      side: 'bottom',
      size: '22rem',
      footer: {
        $el: 'DrawerClose',
        props: { render: { $el: 'Button', children: ['Done'] } },
      },
    },
    children: [{ $el: 'Switch' }],
  }
  assert.deepEqual(sceneGaps('drawer/0', { content }), [])
  for (const [prop, value, reason] of [
    ['side', { $state: 'side' }, /dynamic drawer side/],
    ['size', '80%', /positive fixed length/],
    [
      'title',
      { $el: 'Text', children: ['Title'] },
      /rich or dynamic drawer title/,
    ],
    ['open', true, /open/],
  ])
    assert.match(
      sceneGaps('drawer/0', {
        content: { ...content, props: { ...content.props, [prop]: value } },
      }).join('\n'),
      reason
    )
})

test('reports unsupported props and slider alignment instead of dropping them', () => {
  assert.match(
    sceneGaps('button/0', {
      content: { $el: 'Button', props: { unsupportedProp: 'x' } },
    })[0],
    /unsupportedProp/
  )
  assert.match(
    sceneGaps('slider/0', {
      content: {
        $el: 'Slider',
        props: { defaultValue: [20, 80], thumbAlignment: 'edge' },
      },
    }).join('\n'),
    /inset slider thumbs/
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

test('maps loading shapes and native numeric state but keeps formatting gaps', () => {
  for (const content of [
    { $el: 'Skeleton', props: { shape: 'circle', width: 36, height: 36 } },
    { $el: 'Skeleton', props: { width: '40%' } },
    { $el: 'SkeletonText', props: { lines: 4 } },
    {
      $el: 'NumberField',
      props: {
        value: { $state: 'n' },
        min: 0,
        max: 100,
        step: 5,
        unit: '%',
        onValueChange: { $do: [{ set: 'n', to: { $arg: 0 } }] },
      },
    },
  ])
    assert.deepEqual(sceneGaps('native/0', { content }), [])
  assert.match(
    sceneGaps('number/0', {
      content: {
        $el: 'NumberField',
        props: { format: { style: 'currency', currency: 'USD' } },
      },
    }).join('\n'),
    /NumberField format/
  )
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
test('maps code panels and native preference actions', () => {
  for (const content of [
    {
      $el: 'CodeBlock',
      props: {
        code: 'one\n two\n',
        title: 'example.rs',
        language: 'rust',
        lineNumbers: true,
        copyable: true,
        wrap: true,
        actions: { $el: 'Button', children: ['Run'] },
      },
    },
    {
      $el: 'PreferenceRow',
      props: { title: 'Profile', onOpen: { $do: [] }, tint: 'neutral' },
    },
  ])
    assert.deepEqual(sceneGaps('native/0', { content }), [])
})

test('maps native menus and keeps dynamic model structure gaps explicit', () => {
  const items = [{ label: 'One', checked: true, onSelect: { $do: [] } }]
  for (const content of [
    { $el: 'Menu', props: { items, trigger: { $el: 'Button' } } },
    { $el: 'ContextMenu', props: { items }, children: [{ $el: 'Card' }] },
    { $el: 'Menubar', props: { menus: [{ label: 'File', items }] } },
  ])
    assert.deepEqual(sceneGaps('menu/0', { content }), [])
  for (const [props, expected] of [
    [{ items: { $state: 'items' } }, /dynamic menu items/],
    [{ items: [{ label: 'One', children: [] }] }, /menu item children/],
    [{ items, open: true }, /<Menu open>/],
  ])
    assert.match(
      sceneGaps('menu/0', { content: { $el: 'Menu', props } }).join('\n'),
      expected
    )
  assert.match(
    sceneGaps('menu/0', {
      content: { $el: 'Menubar', props: { menus: { $state: 'menus' } } },
    }).join('\n'),
    /dynamic menu arrays/
  )
})

test('maps table cells and row actions while keeping sorting gaps explicit', () => {
  assert.deepEqual(
    sceneGaps('table/0', {
      content: {
        $el: 'Table',
        props: { tint: 'info', 'aria-label': 'Services' },
        children: [
          {
            $el: 'TableHead',
            children: [
              { $el: 'TableCell', props: { head: true }, children: ['Name'] },
            ],
          },
          {
            $el: 'TableRow',
            props: {
              active: { $state: 'active' },
              interactive: true,
              onClick: { $do: [{ set: 'active', to: true }] },
            },
            children: [
              {
                $el: 'TableCell',
                props: { align: 'end' },
                children: [{ $el: 'Badge', children: ['Live'] }],
              },
            ],
          },
        ],
      },
    }),
    []
  )
  assert.match(
    sceneGaps('table/0', {
      content: { $el: 'TableCell', props: { 'aria-sort': 'ascending' } },
    }).join('\n'),
    /TableCell aria-sort/
  )
  assert.match(
    sceneGaps('table/0', {
      content: { $el: 'Table', children: { $state: 'rows' } },
    }).join('\n'),
    /dynamic Table children/
  )
})

test('maps bounded splitter panels and keeps unsupported frame sizing explicit', () => {
  assert.deepEqual(
    sceneGaps('splitter/0', {
      content: {
        $el: 'Frame',
        props: { width: '100%', height: '9rem' },
        children: [
          {
            $el: 'Splitter',
            props: { orientation: 'horizontal', onSizesChange: { $do: [] } },
            children: [
              {
                $el: 'SplitterPanel',
                props: { defaultSize: 35, minSize: 10, maxSize: 80 },
                children: [{ $el: 'Text', children: ['Left'] }],
              },
              { $el: 'SplitterPanel', props: { defaultSize: 65 } },
            ],
          },
        ],
      },
    }),
    []
  )
  assert.match(
    sceneGaps('splitter/0', {
      content: { $el: 'Frame', props: { height: '50%' } },
    }).join('\n'),
    /fixed length/
  )
  assert.match(
    sceneGaps('splitter/0', {
      content: { $el: 'Splitter', children: [{ $el: 'Card' }] },
    }).join('\n'),
    /children must be panels/
  )
  assert.match(
    sceneGaps('splitter/0', {
      content: { $el: 'SplitterPanel', props: { minSize: { $state: 'min' } } },
    }).join('\n'),
    /dynamic splitter constraints/
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

test('maps native collections and presentation slots without hiding controlled-popup gaps', () => {
  assert.deepEqual(
    sceneGaps('accordion/0', {
      content: {
        $el: 'Accordion',
        props: { multiple: true, defaultValue: ['one'], divided: true },
        children: [
          {
            $el: 'AccordionItem',
            props: { value: 'one', title: 'One' },
            children: ['Body'],
          },
        ],
      },
    }),
    []
  )
  assert.deepEqual(
    sceneGaps('dialog/0', {
      content: {
        $el: 'Dialog',
        props: {
          trigger: { $el: 'Button', children: ['Open'] },
          width: '24rem',
        },
        children: [
          { $el: 'DialogHeader', props: { title: 'Title' } },
          {
            $el: 'DialogFooter',
            children: [
              {
                $el: 'DialogClose',
                props: { render: { $el: 'Button', children: ['Close'] } },
              },
            ],
          },
        ],
      },
    }),
    []
  )
  assert.match(
    sceneGaps('popover/0', {
      content: {
        $el: 'Popover',
        props: { open: true, trigger: { $el: 'Button' } },
      },
    }).join('\n'),
    /<Popover open>/
  )
})

test('maps recursive tree slots and keeps structural gaps explicit', () => {
  const tree = items => ({
    content: {
      $el: 'Tree',
      props: {
        items,
        selected: { $state: 'id' },
        onSelectedChange: { $do: [] },
        expanded: { $state: 'open' },
      },
    },
  })
  assert.deepEqual(
    sceneGaps(
      'tree/0',
      tree([
        {
          id: 'src',
          label: 'src',
          children: [
            {
              id: 'a',
              label: { $el: 'Text', children: ['A'] },
              text: 'A',
              meta: { $el: 'Badge', children: ['M'] },
              disabled: { $state: 'disabled' },
            },
          ],
        },
        { id: 'empty', label: 'empty', children: [] },
      ])
    ),
    []
  )
  assert.match(
    sceneGaps('tree/0', tree({ $state: 'items' })).join('\n'),
    /dynamic tree arrays/
  )
  assert.match(
    sceneGaps(
      'tree/0',
      tree([{ id: 'a', label: 'A', children: { $state: 'children' } }])
    ).join('\n'),
    /dynamic tree arrays/
  )
  assert.match(
    sceneGaps(
      'tree/0',
      tree([{ id: 'a', label: 'A', children: [{ id: 'a', label: 'again' }] }])
    ).join('\n'),
    /ids must be unique/
  )
  assert.match(
    sceneGaps('tree/0', tree([{ id: 'a', label: 'A', href: '/a' }])).join('\n'),
    /tree item href/
  )
})

test('maps shortcut recording, controlled callbacks and accessible prompts', () => {
  assert.deepEqual(
    sceneGaps('shortcut-recorder/0', {
      content: {
        $el: 'ShortcutRecorder',
        props: {
          value: { $state: 'binding' },
          onValueChange: { $do: [] },
          onClear: { $do: [] },
          placeholder: 'Choose',
          recordingLabel: 'Waiting',
          clearLabel: 'Remove',
          state: 'error',
          disabled: false,
          tint: 'danger',
          'aria-label': 'Action',
        },
      },
    }),
    []
  )
  assert.match(
    sceneGaps('shortcut-recorder/0', {
      content: { $el: 'ShortcutRecorder', props: { onKeyDown: {} } },
    }).join('\n'),
    /onKeyDown/
  )
})

test('maps native OTP cells and preserves unsupported normalization/form gaps', () => {
  assert.deepEqual(
    sceneGaps('otp-field/0', {
      content: {
        $el: 'OtpField',
        props: {
          length: 6,
          groupAfter: 3,
          size: 'small',
          state: 'error',
          tint: 'info',
          value: { $state: 'code' },
          onValueChange: { $do: [] },
          onValueInvalid: { $do: [] },
          onValueComplete: { $do: [] },
          validationType: 'numeric',
          mask: true,
          readOnly: false,
          disabled: false,
          required: true,
          'aria-label': 'Code',
        },
      },
    }),
    []
  )
  assert.match(
    sceneGaps('otp-field/0', {
      content: { $el: 'OtpField', props: { length: { $state: 'length' } } },
    }).join('\n'),
    /positive committed integer/
  )
  assert.match(
    sceneGaps('otp-field/0', {
      content: { $el: 'OtpField', props: { length: 6, normalizeValue: {} } },
    }).join('\n'),
    /normalizeValue/
  )
  assert.match(
    sceneGaps('otp-field/0', {
      content: { $el: 'OtpField', props: { length: 6, autoSubmit: true } },
    }).join('\n'),
    /autoSubmit/
  )
})

test('maps numeric and range sliders with shared geometry and callbacks', () => {
  assert.deepEqual(
    sceneGaps('slider/0', {
      content: {
        $el: 'Slider',
        props: {
          value: { $state: 'range' },
          defaultValue: [20, 80],
          min: 0,
          max: 100,
          step: 2,
          largeStep: 10,
          minStepsBetweenValues: 2,
          thumbCollisionBehavior: 'push',
          thumbAlignment: 'center',
          showValue: true,
          size: 'small',
          tint: 'success',
          disabled: false,
          onValueChange: { $do: [] },
          onValueCommitted: { $do: [] },
          'aria-label': 'Range',
        },
      },
    }),
    []
  )
  assert.match(
    sceneGaps('slider/0', {
      content: {
        $el: 'Slider',
        props: { defaultValue: [20, 80], thumbCollisionBehavior: 'swap' },
      },
    }).join('\n'),
    /swap collision/
  )
  assert.match(
    sceneGaps('slider/0', {
      content: { $el: 'Slider', props: { defaultValue: 20, formatValue: {} } },
    }).join('\n'),
    /formatValue/
  )
})

test('maps editable combobox choices and free-text suggestions without hiding unsupported content', () => {
  for (const content of [
    {
      $el: 'Combobox',
      props: {
        options: [
          {
            label: 'Countries',
            options: [{ value: 'jp', label: 'Japan', disabled: true }],
          },
        ],
        value: null,
        clearable: true,
        onValueChange: { $do: [{ set: 'country', to: { $arg: 0 } }] },
      },
    },
    {
      $el: 'Autocomplete',
      props: {
        items: ['Inter', 'JetBrains Mono'],
        value: { $state: 'font' },
        mono: true,
        readOnly: true,
      },
    },
  ])
    assert.deepEqual(sceneGaps('combobox/0', { content }), [])
  for (const props of [
    { empty: { $el: 'Text', children: ['Empty'] } },
    { name: 'country' },
    { options: [{ value: 'jp', label: { $el: 'Text', children: ['Japan'] } }] },
  ]) {
    assert.notEqual(
      sceneGaps('combobox/0', { content: { $el: 'Combobox', props } }).length,
      0
    )
  }
})

test('maps text form submission, reset actions and validation boundaries with explicit association gaps', () => {
  const input = { $el: 'TextField', props: { type: 'email', required: true } }
  const field = {
    $el: 'FormField',
    props: { label: 'Email', name: 'email' },
    children: [input],
  }
  assert.deepEqual(
    sceneGaps('form/0', {
      content: {
        $el: 'Form',
        props: {
          validationMode: 'onBlur',
          errors: { email: 'Taken' },
          onSubmit: { $do: [] },
        },
        children: [
          field,
          { $el: 'Button', props: { type: 'reset' } },
          { $el: 'Button', props: { type: 'submit' } },
        ],
      },
    }),
    []
  )
  for (const content of [
    { $el: 'Form', children: { $state: 'children' } },
    { $el: 'Form', children: [{ $el: 'Select', props: { options: [] } }] },
    { $el: 'Form', children: [{ $el: 'Form', children: [] }] },
    { $el: 'Form', props: { action: '/submit' } },
    {
      $el: 'FormField',
      props: { label: 'Email', validate: { $fn: 'custom' } },
    },
    { $el: 'Button', props: { type: { $state: 'type' } } },
  ])
    assert.notEqual(sceneGaps('form/0', { content }).length, 0)
})

test('maps sidebar disclosure and native navigation without hiding part gaps', () => {
  const button = {
    $el: 'SidebarMenuButton',
    props: {
      active: { $state: 'active' },
      tooltip: 'Library',
      onClick: { $do: [{ set: 'active', to: true }] },
    },
    children: ['Library'],
  }
  assert.deepEqual(
    sceneGaps('sidebar/0', {
      content: {
        $el: 'SidebarProvider',
        props: { open: { $state: 'open' } },
        children: [
          {
            $el: 'Sidebar',
            props: {
              collapsible: 'icon',
              collapsed: { $state: 'hidden' },
              width: '12rem',
            },
            children: [
              {
                $el: 'SidebarMenu',
                children: [
                  {
                    $el: 'SidebarMenuItem',
                    children: [
                      button,
                      { $el: 'SidebarMenuBadge', children: ['12'] },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      },
    }),
    []
  )
  for (const content of [
    { $el: 'SidebarMenuButton', children: { $state: 'caption' } },
    { $el: 'SidebarTrigger', props: { label: { $state: 'label' } } },
    { $el: 'SidebarRail' },
    { $el: 'SidebarMenuAction' },
    { $el: 'SidebarMenuSub' },
    { $el: 'Sidebar', props: { width: '40%' } },
    { $el: 'Sidebar', props: { variant: { $state: 'variant' } } },
    { ...button, children: [{ $el: 'Badge', children: ['Library'] }] },
    { ...button, props: { tooltip: { $el: 'Text', children: ['Hint'] } } },
    { $el: 'SidebarTrigger', children: [{ $el: 'Icon' }] },
  ])
    assert.ok(sceneGaps('sidebar/0', { content }).length)
})

test('maps calendar selection, month control and date bounds with explicit dynamic structure gaps', () => {
  assert.deepEqual(
    sceneGaps('calendar/0', {
      content: {
        $el: 'Calendar',
        props: {
          mode: 'range',
          value: { $state: 'range' },
          onValueChange: { $do: [{ set: 'range', to: { $arg: 0 } }] },
          month: { $state: 'month' },
          onMonthChange: { $do: [{ set: 'month', to: { $arg: 0 } }] },
          size: 'small',
          months: 2,
          weekStartsOn: 0,
          navigation: false,
          locale: 'zh-CN',
          today: '2026-10-16T00:00',
          min: '2026-10-12T00:00',
          max: '2026-10-23T00:00',
        },
      },
    }),
    []
  )
  for (const props of [
    { months: 3 },
    { months: { $state: 'count' } },
    { weekStartsOn: 7 },
    { mode: { $state: 'mode' } },
    { navigation: { $state: 'nav' } },
    { locale: { $state: 'locale' } },
  ])
    assert.ok(
      sceneGaps('calendar/0', { content: { $el: 'Calendar', props } }).length
    )
})

test('maps date range values and static picker configuration while exposing unsupported structure', () => {
  assert.deepEqual(
    sceneGaps('date-range-picker/0', {
      content: {
        $el: 'DateRangePicker',
        props: {
          value: { $state: 'range' },
          onValueChange: { $do: [{ set: 'range', to: { $arg: 0 } }] },
          presets: [{ label: 'Sprint', days: [0, 13] }],
          durations: [{ label: 'Working day', minutes: 480 }],
          labels: { done: 'Save' },
          months: 2,
          weekStartsOn: 0,
          timeStep: 30,
          locale: 'zh-CN',
          today: '2026-10-16T00:00',
          size: 'small',
          tint: 'danger',
          state: 'error',
        },
      },
    }),
    []
  )
  for (const props of [
    { months: 3 },
    { timeStep: 0 },
    { timeStep: 1.5 },
    { weekStartsOn: 7 },
    { presets: [{ label: 'Sprint', days: [0] }] },
    { presets: [{ label: 'Sprint', value: { $state: 'range' } }] },
    { durations: [{ label: 'Working day', minutes: 1.5 }] },
    { locale: { $state: 'locale' } },
    { labels: { $state: 'labels' } },
    { labels: { done: { $state: 'caption' } } },
    { presets: [{ label: { $state: 'preset' }, days: [0, 13] }] },
    { durations: [{ label: { $state: 'duration' }, minutes: 60 }] },
    { today: { $state: 'today' } },
  ])
    assert.ok(
      sceneGaps('date-range-picker/0', {
        content: { $el: 'DateRangePicker', props },
      }).length
    )
})
