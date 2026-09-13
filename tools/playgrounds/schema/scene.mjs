import { valueType } from './value.mjs'

/** Portable story data. Validation is shared by generation and the React host. */
export const COMPONENTS = {
  Box: [
    'width',
    'height',
    'maxWidth',
    'grow',
    'padding',
    'surface',
    'radius',
    'justify',
    'display',
    'gap',
    'align',
    'tone',
  ],
  Badge: ['variant', 'tint', 'size'],
  Card: ['variant', 'tint', 'size'],
  Callout: ['title', 'tint', 'size'],
  EmptyState: ['title'],
  Progress: ['value', 'size', 'tint', 'gradient'],
  StepList: [],
  Step: ['status', 'label', 'meta', 'tint'],
  Divider: ['tone', 'orientation'],
  SectionLabel: ['tint', 'faint'],
  Table: [],
  TableHead: [],
  TableRow: [],
  TableCell: ['head', 'align'],
  Button: ['variant', 'tint', 'size', 'disabled', 'expand', 'shortcut'],
  Page: ['title', 'description'],
  Section: ['title', 'description'],
  Stack: [],
  Grid: ['min'],
  Specimen: ['label'],
  Inline: ['tone', 'fontSize', 'mono'],
  Value: [],
  Switch: ['checked', 'disabled', 'size', 'tint', 'label'],
  Checkbox: ['checked', 'disabled', 'size', 'tint'],
  PreferenceSection: ['label'],
  PreferenceRow: ['title', 'subtitle'],
  Icon: ['name', 'size'],
  Spinner: ['size', 'tint', 'onAccent'],
  KeyCap: ['size', 'variant'],
  Row: ['gap'],
  Col: ['width', 'gap'],
  Text: [],
  FormField: ['label', 'hint', 'state'],
  TextField: [
    'placeholder',
    'readOnly',
    'disabled',
    'size',
    'state',
    'tint',
    'mono',
  ],
  Dialog: ['width', 'tone'],
  DialogHeader: ['title', 'subtitle'],
  DialogBody: [],
  DialogFooter: [],
  ActionBar: ['align', 'pushEnd'],
}
export const SLOTS = { Callout: ['icon', 'action'], EmptyState: ['action'] }
const fail = (path, message) => {
  throw new Error(`${path}: ${message}`)
}
const record = value =>
  value !== null && typeof value === 'object' && !Array.isArray(value)
const scalar = value =>
  ['string', 'boolean'].includes(typeof value) ||
  (typeof value === 'number' && Number.isFinite(value))
const exact = (value, keys, path) => {
  for (const key of Object.keys(value))
    if (!keys.includes(key)) fail(path, `unknown field ${key}`)
}
export function validateScene(scene, source = 'scene') {
  if (!record(scene)) fail(source, 'expected an object')
  exact(scene, ['state', 'content', 'checkpoints', 'timers'], source)
  const state = scene.state ?? {}
  if (!record(state) || Object.values(state).some(v => !scalar(v)))
    fail(source, 'state must contain scalar values')
  if (scene.timers !== undefined && !Array.isArray(scene.timers))
    fail(source, 'timers must be an array')
  for (const timer of scene.timers ?? []) {
    if (!record(timer)) fail(source, 'timer must be an object')
    exact(timer, ['every', 'set', 'value'], source)
    if (
      !Number.isSafeInteger(timer.every) ||
      timer.every <= 0 ||
      timer.every > 2147483647
    )
      fail(source, 'timer interval must be positive milliseconds')
    if (
      !Object.hasOwn(state, timer.set) ||
      valueType(timer.value, state) !== typeof state[timer.set]
    )
      fail(source, 'timer must update matching declared state')
  }
  const ids = new Map()
  function visit(node, path) {
    if (typeof node === 'string') return
    if (!record(node)) fail(path, 'expected a node or text')
    exact(
      node,
      [
        'component',
        'id',
        'props',
        'derive',
        'bind',
        'on',
        'children',
        'value',
        'slots',
      ],
      path
    )
    if (!Object.hasOwn(COMPONENTS, node.component))
      fail(path, `unsupported component ${node.component}`)
    if (node.component === 'Value') {
      valueType(node.value, state)
      if (node.children?.length) fail(path, 'Value cannot have children')
    } else if (node.value !== undefined)
      fail(path, 'value expressions belong to Value nodes')
    if (node.id !== undefined) {
      if (
        typeof node.id !== 'string' ||
        !/^[a-z][a-z0-9-]*$/.test(node.id) ||
        ids.has(node.id)
      )
        fail(path, `invalid or duplicate node id ${node.id}`)
      ids.set(node.id, node.component)
    }
    if (node.bind !== undefined && !record(node.bind))
      fail(path, 'bindings must be an object')
    if (node.on !== undefined && !record(node.on))
      fail(path, 'events must be an object')
    if (node.derive !== undefined) {
      if (!record(node.derive)) fail(path, 'derived props must be an object')
      const allowed =
        {
          Button: { disabled: 'boolean' },
          Progress: { value: 'number' },
          Step: { status: 'string', meta: 'string' },
          TextField: { state: 'string' },
          FormField: { state: 'string', hint: 'string' },
        }[node.component] ?? {}
      exact(node.derive, Object.keys(allowed), path)
      for (const [key, expr] of Object.entries(node.derive)) {
        if (Object.hasOwn(node.props ?? {}, key))
          fail(path, `duplicate static and derived prop ${key}`)
        if (valueType(expr, state) !== allowed[key])
          fail(path, `invalid derived ${key} type`)
        const states = expression => {
          if (
            (key === 'status'
              ? ['done', 'active', 'idle']
              : ['default', 'error']
            ).includes(expression)
          )
            return true
          return (
            record(expression) &&
            Object.hasOwn(expression, 'if') &&
            states(expression.then) &&
            states(expression.else)
          )
        }
        if (['state', 'status'].includes(key) && !states(expr))
          fail(path, 'derived state must select valid field states')
      }
    }
    if (
      node.component === 'Step' &&
      (typeof node.props?.label !== 'string' ||
        (node.props.status === undefined && node.derive?.status === undefined))
    )
      fail(path, 'Step requires a label and status')
    if (node.component === 'FormField' && typeof node.props?.label !== 'string')
      fail(path, 'FormField requires a label')
    if (node.component === 'TextField' && !node.bind?.value)
      fail(path, 'TextField requires a value binding')
    if (
      node.component === 'Dialog' &&
      (path !== `${source}/content` || !node.bind?.open)
    )
      fail(path, 'Dialog must be the root and requires an open binding')
    if (
      node.component === 'Text' &&
      (node.children?.length !== 1 || typeof node.children[0] !== 'string')
    )
      fail(path, 'Text requires one text child')
    if (
      [
        'DialogHeader',
        'TextField',
        'Icon',
        'Spinner',
        'Progress',
        'Divider',
        'Step',
        'EmptyState',
      ].includes(node.component) &&
      node.children?.length
    )
      fail(path, 'this component has no children')
    if (
      ['DialogHeader', 'Page', 'Section', 'EmptyState'].includes(
        node.component
      ) &&
      typeof node.props?.title !== 'string'
    )
      fail(path, `${node.component} requires a title`)
    if (node.component === 'Specimen' && typeof node.props?.label !== 'string')
      fail(path, 'Specimen requires a label')
    if (
      node.component === 'Icon' &&
      !['ArrowDownload20Regular', 'Circle20Filled'].includes(node.props?.name)
    )
      fail(path, 'Icon requires a supported glyph name')
    if (
      ['KeyCap', 'SectionLabel'].includes(node.component) &&
      (!node.children?.length ||
        node.children.some(child => typeof child !== 'string'))
    )
      fail(path, `${node.component} requires text children`)
    if (node.props !== undefined) {
      if (!record(node.props)) fail(path, 'props must be an object')
      exact(node.props, COMPONENTS[node.component], path)
      for (const [key, value] of Object.entries(node.props)) {
        if (!scalar(value)) fail(path, `prop ${key} must be scalar`)
        if (
          [
            'meta',
            'placeholder',
            'hint',
            'shortcut',
            'title',
            'subtitle',
            'description',
            'label',
            'name',
          ].includes(key) &&
          typeof value !== 'string'
        )
          fail(path, `${key} must be text`)
        const enums = {
          variant: [
            'normal',
            'recessed',
            'filled',
            'tinted',
            'outlined',
            'plain',
          ],
          tint: ['primary', 'neutral', 'success', 'warning', 'danger', 'info'],
          size: ['small', 'medium', 'large'],
          fontSize: ['title.large', 'body.small'],
          state: ['default', 'error'],
          status: ['done', 'active', 'idle'],
          tone:
            node.component === 'Divider'
              ? ['default', 'muted']
              : ['Inline', 'Box'].includes(node.component)
                ? ['muted']
                : ['normal', 'danger'],
          gap: ['Row', 'Col'].includes(node.component)
            ? ['spacing.1', 'spacing.2', 'spacing.3', 'spacing.4', 'spacing.5']
            : node.component === 'Box'
              ? ['spacing.1', 'spacing.2', 'spacing.3']
              : ['spacing.3', 'spacing.4'],
          align:
            node.component === 'Box'
              ? ['center']
              : node.component === 'TableCell'
                ? ['start', 'end']
                : ['start', 'end', 'between'],
          padding: ['spacing.2', 'spacing.3'],
          display: ['inline-flex', 'flex'],
          justify: ['between'],
          orientation: ['horizontal', 'vertical'],
          surface: ['color.surface.chrome', 'color.surface.muted'],
          radius: ['radius.medium', 'radius.large'],
        }
        if (key === 'variant' && node.component === 'Card')
          enums.variant = ['raised', 'sunken', 'outlined', 'tinted']
        if (key === 'variant' && node.component === 'Badge')
          enums.variant = ['filled', 'tinted', 'outlined', 'raised']
        if (key === 'variant' && node.component === 'KeyCap')
          enums.variant = ['hint', 'strong', 'key']
        if (key === 'size' && node.component === 'Icon') {
          if (typeof value !== 'number' || value <= 0)
            fail(path, 'icon size must be positive logical pixels')
          delete enums.size
        }
        if (
          key === 'name' &&
          node.component === 'Icon' &&
          !['ArrowDownload20Regular', 'Circle20Filled'].includes(value)
        )
          fail(path, `unsupported icon ${value}`)
        if (enums[key] && !enums[key].includes(value))
          fail(path, `invalid ${key}: ${value}`)
        if (
          [
            'disabled',
            'readOnly',
            'expand',
            'mono',
            'pushEnd',
            'checked',
            'head',
            'faint',
            'onAccent',
            'gradient',
            'grow',
          ].includes(key) &&
          typeof value !== 'boolean'
        )
          fail(path, `${key} must be boolean`)
        if (
          key === 'value' &&
          (typeof value !== 'number' || !Number.isFinite(value))
        )
          fail(path, 'value must be a finite number')
        if (
          ['width', 'height', 'maxWidth', 'min'].includes(key) &&
          (typeof value !== 'number' || value <= 0 || !Number.isFinite(value))
        )
          fail(path, 'width must be positive logical pixels')
      }
    }
    for (const [prop, key] of Object.entries(node.bind ?? {})) {
      if (
        !(
          (node.component === 'TextField' &&
            prop === 'value' &&
            typeof state[key] === 'string') ||
          (['Switch', 'Checkbox'].includes(node.component) &&
            prop === 'checked' &&
            typeof state[key] === 'boolean') ||
          (node.component === 'Dialog' &&
            prop === 'open' &&
            typeof state[key] === 'boolean')
        )
      )
        fail(path, `invalid state binding ${prop}: ${key}`)
    }
    for (const [event, action] of Object.entries(node.on ?? {})) {
      if (node.component !== 'Button' || event !== 'press')
        fail(path, `unsupported event ${event}`)
      if (!record(action)) fail(path, 'action must be an object')
      exact(action, ['set', 'value'], path)
      if (
        !Object.hasOwn(state, action.set) ||
        typeof state[action.set] !== typeof action.value
      )
        fail(path, 'action must assign a declared state of the same type')
    }
    if (node.slots !== undefined) {
      if (!record(node.slots)) fail(path, 'slots must be an object')
      exact(node.slots, SLOTS[node.component] ?? [], path)
      for (const [name, slot] of Object.entries(node.slots))
        visit(slot, `${path}/slot-${name}`)
    }
    if (
      node.component === 'Callout' &&
      (node.children ?? []).some(child => typeof child !== 'string')
    )
      fail(path, 'Callout description must be text')
    if (node.children !== undefined && !Array.isArray(node.children))
      fail(path, 'children must be an array')
    for (const [i, child] of (node.children ?? []).entries())
      visit(child, `${path}/${i}`)
  }
  visit(scene.content, `${source}/content`)
  const checkpoints = scene.checkpoints ?? [{ id: 'initial', steps: [] }]
  const keys = new Set()
  if (!Array.isArray(checkpoints) || !checkpoints.length)
    fail(source, 'checkpoints must be nonempty')
  for (const checkpoint of checkpoints) {
    if (!record(checkpoint)) fail(source, 'checkpoint must be an object')
    if (checkpoint.expect !== undefined && !record(checkpoint.expect))
      fail(source, 'expected state must be an object')
    exact(
      checkpoint,
      ['id', 'steps', 'expect', 'expectText', 'expectDisabled'],
      source
    )
    if (
      typeof checkpoint.id !== 'string' ||
      !/^[a-z][a-z0-9-]*$/.test(checkpoint.id) ||
      keys.has(checkpoint.id)
    )
      fail(source, 'invalid or duplicate checkpoint id')
    keys.add(checkpoint.id)
    if (!Array.isArray(checkpoint.steps))
      fail(source, 'checkpoint steps must be an array')
    for (const step of checkpoint.steps) {
      if (!record(step)) fail(source, 'step must be an object')
      if (step.action === 'advance') {
        exact(step, ['action', 'milliseconds'], source)
        if (
          !Number.isSafeInteger(step.milliseconds) ||
          step.milliseconds < 0 ||
          step.milliseconds > 60000
        )
          fail(source, 'advance requires 0–60000 milliseconds')
        continue
      }
      exact(step, ['action', 'target', 'value'], source)
      if (!ids.has(step.target))
        fail(source, `unknown checkpoint target ${step.target}`)
      if (!['click', 'fill'].includes(step.action))
        fail(source, `unsupported checkpoint action ${step.action}`)
      if (
        step.action === 'fill' &&
        (ids.get(step.target) !== 'TextField' || typeof step.value !== 'string')
      )
        fail(source, 'fill requires a text field and string value')
      if (
        step.action === 'click' &&
        !['Button', 'Switch', 'Checkbox'].includes(ids.get(step.target))
      )
        fail(source, 'click requires a button')
    }
    if (checkpoint.expectText !== undefined && !record(checkpoint.expectText))
      fail(source, 'expected text must be an object')
    for (const [target, text] of Object.entries(checkpoint.expectText ?? {})) {
      if (!ids.has(target) || typeof text !== 'string')
        fail(source, 'expected text needs a node id and string')
    }
    if (
      checkpoint.expectDisabled !== undefined &&
      !record(checkpoint.expectDisabled)
    )
      fail(source, 'expected disabled must be an object')
    for (const [target, disabled] of Object.entries(
      checkpoint.expectDisabled ?? {}
    )) {
      if (ids.get(target) !== 'Button' || typeof disabled !== 'boolean')
        fail(source, 'expected disabled needs a button id and boolean')
    }
    for (const [key, value] of Object.entries(checkpoint.expect ?? {})) {
      if (!Object.hasOwn(state, key) || typeof state[key] !== typeof value)
        fail(source, `invalid expected state ${key}`)
    }
  }
  return scene
}
