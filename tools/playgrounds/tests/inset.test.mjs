import test from 'node:test'
import assert from 'node:assert/strict'
import { insetScene, insetSupports } from '../inset-scene.mjs'

test('Inset coverage rejects unsupported interactions and properties', () => {
  const content = { component: 'Button', children: ['Save'] }
  assert.equal(insetSupports({ content }), true)
  assert.equal(insetSupports({ state: { invalid: [] }, content }), false)
  assert.equal(
    insetSupports({
      content: { ...content, on: { press: { set: 'saved', value: true } } },
    }),
    false
  )
  assert.equal(
    insetSupports({ content: { ...content, props: { href: '/save' } } }),
    false
  )
  assert.equal(
    insetSupports({
      content: { component: 'Progress', props: { unknownGradient: true } },
    }),
    false
  )
})

test('Inset checks nested content and slots before claiming coverage', () => {
  const content = {
    component: 'EmptyState',
    props: { title: 'Empty' },
    slots: { action: { component: 'Button', children: ['Create'] } },
  }
  assert.equal(insetSupports({ content }), true)
  content.slots.action = { component: 'TextField', bind: { value: 'name' } }
  assert.equal(insetSupports({ content }), false)
  content.slots = { icon: { component: 'Spinner' } }
  assert.equal(insetSupports({ content }), false)
})

test('Inset vertical rules retain the declared row spacing', () => {
  const output = insetScene({
    content: {
      component: 'Row',
      props: { gap: 'spacing.2' },
      children: [
        { component: 'Button', children: ['Save'] },
        { component: 'Divider', props: { orientation: 'vertical' } },
        { component: 'Button', children: ['Cancel'] },
      ],
    },
  })
  assert.match(output, /IntrinsicHeight::new\(\).*spacing\(v.spacing_2\)/)
})

test('Inset compiles action alignment and table headers without accepting auto margins', () => {
  const action = {
    content: {
      component: 'ActionBar',
      props: { align: 'end' },
      children: [{ component: 'Button', children: ['Save'] }],
    },
  }
  assert.match(insetScene(action), /MainAxisAlignment::End/)
  action.content.props.pushEnd = true
  assert.equal(insetSupports(action), false)
  const table = {
    content: {
      component: 'Table',
      children: [
        {
          component: 'TableHead',
          children: [
            {
              component: 'TableCell',
              props: { head: true, align: 'end' },
              children: ['Total'],
            },
          ],
        },
      ],
    },
  }
  assert.match(insetScene(table), /head\(true\).end\(true\)/)
  table.content.children[0].children[0].props.colspan = 2
  assert.equal(insetSupports(table), false)
})

test('Inset boolean bindings and conditional text compile from one state declaration', () => {
  const scene = {
    state: { enabled: true },
    content: {
      component: 'Row',
      children: [
        { component: 'Switch', bind: { checked: 'enabled' }, id: 'toggle' },
        {
          component: 'Value',
          // Shared conditional data uses a branch name, not a Promise callback.
          // oxlint-disable-next-line unicorn/no-thenable
          value: { if: { state: 'enabled' }, then: 'On', else: 'Off' },
        },
      ],
    },
  }
  assert.equal(insetSupports(scene), true)
  assert.match(
    insetScene(scene),
    /scene_state::set\(app, &state, "enabled", value\)/
  )
  assert.match(insetScene(scene), /state.key\("toggle"\)/)
  scene.content.children[0].bind.extra = 'enabled'
  assert.equal(insetSupports(scene), false)
  delete scene.content.children[0].bind.extra
  scene.content.children[1].value = { unknown: 'enabled' }
  assert.equal(insetSupports(scene), false)
})

test('Inset supports text validation and assignments without accepting unknown events', () => {
  const scene = {
    state: { name: 'Ada' },
    content: {
      component: 'Button',
      on: { press: { set: 'name', value: '' } },
      derive: { disabled: { eq: [{ trim: { state: 'name' } }, ''] } },
      children: ['Clear'],
    },
  }
  assert.equal(insetSupports(scene), true)
  assert.match(insetScene(scene), /TRIM_CODE_POINTS/)
  assert.match(insetScene(scene), /scene_state::set/)
  scene.content.on.longPress = { set: 'name', value: '' }
  assert.equal(insetSupports(scene), false)
  delete scene.content.on.longPress
  scene.content.derive.hidden = true
  assert.equal(insetSupports(scene), false)
})

test('Inset maps declared flexible children and trailing actions in their parent row', () => {
  const scene = {
    content: {
      component: 'Box',
      props: { display: 'flex', justify: 'between', gap: 'spacing.2' },
      children: [
        {
          component: 'Box',
          props: { grow: true },
          children: [{ component: 'Divider' }],
        },
        {
          component: 'ActionBar',
          props: { pushEnd: true },
          children: [{ component: 'Button', children: ['Save'] }],
        },
      ],
    },
  }
  const output = insetScene(scene)
  assert.match(output, /MainAxisAlignment::SpaceBetween/)
  assert.match(output, /MainAxisAlignment::End/)
  assert.match(output, /Expanded::new/)
  assert.doesNotMatch(output, /Spacer::new/)
  assert.equal(insetSupports({ content: scene.content.children[0] }), false)
})

test('Inset retains explicit Fluent glyph sizes and Callout slots', () => {
  const scene = {
    content: {
      component: 'Callout',
      props: { size: 'small', title: 'Saved' },
      slots: {
        icon: { component: 'Icon', props: { name: 'Circle20Filled', size: 8 } },
        action: { component: 'Button', children: ['Undo'] },
      },
    },
  }
  assert.equal(insetSupports(scene), true)
  const output = insetScene(scene)
  assert.match(output, /glyph\("Circle20Filled"\)/)
  assert.match(output, /IconThemeData::new\(\).size\(8.0\)/)
  assert.match(output, /control.size = ui::ControlSize::Small/)
  scene.content.slots.icon.props.name = 'UnknownIcon'
  assert.equal(insetSupports(scene), false)
})

test('Inset emits progress gradients and step tints from declarations', () => {
  const scene = {
    content: {
      component: 'Col',
      children: [
        { component: 'Progress', props: { gradient: true, value: 43 } },
        {
          component: 'StepList',
          children: [
            {
              component: 'Step',
              props: { status: 'active', label: 'Working', tint: 'danger' },
            },
          ],
        },
      ],
    },
  }
  const output = insetScene(scene)
  assert.match(output, /gradient: true/)
  assert.match(output, /ui::Step \{ tint: ui::Tint::Danger/)
  scene.content.children[1].children[0].props.unknownState = true
  assert.equal(insetSupports(scene), false)
})
