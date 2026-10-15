import { gaps, CHILDREN as C } from './support.mjs'

// Keep this list paired with example/entry/.../lib/SceneNode.ets. A scene
// with any gap is shown as a gap in full; it is never partly rendered.
export const SUPPORT = {
  Row: [C, 'gap', 'align', 'justify', 'width'],
  Col: [C, 'gap', 'align', 'width'],
  Frame: [C, 'width', 'height'],
  Text: [C, 'strong', 'tone', 'tint'],
  Icon: ['icon', 'size'],
  Button: [
    C,
    'variant',
    'tint',
    'size',
    'disabled',
    'fullWidth',
    'shortcut',
    'onClick',
  ],
  IconButton: [C, 'label', 'size', 'variant', 'tint', 'disabled', 'onClick'],
  Toggle: [
    C,
    'label',
    'size',
    'variant',
    'tint',
    'disabled',
    'pressed',
    'defaultPressed',
    'onPressedChange',
  ],
  Badge: [C, 'variant', 'tint', 'size'],
  Avatar: ['name', 'size', 'shape', 'tint', 'fallback'],
  KeyCap: [C, 'variant', 'size'],
  Card: [C, 'variant', 'size'],
  Checkbox: [
    C,
    'checked',
    'defaultChecked',
    'onCheckedChange',
    'disabled',
    'tint',
    'size',
    'note',
  ],
  Switch: [
    'checked',
    'defaultChecked',
    'onCheckedChange',
    'size',
    'tint',
    'disabled',
    'aria-label',
  ],
  TextField: [
    'value',
    'defaultValue',
    'onValueChange',
    'placeholder',
    'aria-label',
    'type',
    'multiline',
    'rows',
    'state',
    'readOnly',
    'mono',
    'size',
    'disabled',
  ],
  SearchField: [
    'value',
    'onValueChange',
    'placeholder',
    'aria-label',
    'size',
    'disabled',
  ],
  Select: [
    'options',
    'value',
    'defaultValue',
    'onValueChange',
    'placeholder',
    'size',
    'disabled',
    'aria-label',
  ],
  RadioGroup: [
    'options',
    'value',
    'defaultValue',
    'onValueChange',
    'size',
    'tint',
    'disabled',
    'aria-label',
  ],
  Slider: [
    'value',
    'defaultValue',
    'onValueChange',
    'min',
    'max',
    'step',
    'size',
    'tint',
    'disabled',
    'showValue',
    'aria-label',
  ],
  Progress: ['value', 'size', 'tint', 'aria-label'],
  Meter: [
    'value',
    'min',
    'max',
    'label',
    'showValue',
    'size',
    'tint',
    'aria-label',
  ],
  Spinner: ['size', 'tint', 'onAccent'],
  SegmentedControl: [
    'items',
    'value',
    'defaultValue',
    'onValueChange',
    'size',
    'tint',
    'activeStyle',
    'aria-label',
  ],
  NumberField: [
    'value',
    'defaultValue',
    'onValueChange',
    'min',
    'max',
    'step',
    'unit',
    'size',
    'disabled',
    'aria-label',
  ],
  OptionCard: [
    'title',
    'description',
    'selected',
    'onClick',
    'disabled',
    'tint',
  ],
  FormField: [C, 'label', 'hint', 'state'],
  Fieldset: [C, 'legend', 'disabled'],
  Callout: [C, 'title', 'tint', 'action'],
  EmptyState: [C, 'title', 'action'],
  Preferences: [C],
  PreferenceGroup: [C, 'title', 'description'],
  PreferenceSection: [C, 'label', 'footer'],
  PreferenceRow: [C, 'title', 'subtitle'],
  Divider: ['tone', 'orientation'],
  Code: [C],
  CodeBlock: ['code'],
  SectionLabel: [C, 'faint', 'tint'],
  Collapsible: [C, 'title', 'defaultOpen', 'meta'],
  Skeleton: ['width', 'height', 'shape'],
  SkeletonText: ['lines'],
  Table: [C],
  TableHead: [C],
  TableRow: [C],
  TableCell: [C, 'head', 'align'],
  StepList: [C],
  Step: [C, 'status', 'label', 'meta'],
  Tag: [C, 'selected', 'onSelectedChange', 'onRemove', 'tint'],
  Splitter: [C, 'orientation'],
  SplitterPanel: [C, 'defaultSize', 'minSize', 'maxSize'],
  Dialog: [C, 'trigger', 'width'],
  DialogHeader: ['title', 'subtitle'],
  DialogBody: [C],
  DialogFooter: [C],
  DialogClose: [C, 'render'],
  Popover: [C, 'trigger', 'title', 'description', 'width', 'side'],
  Menu: ['trigger', 'items'],
  ContextMenu: [C, 'items'],
  Tooltip: [C, 'label', 'side'],
  ActionBar: [C, 'align'],
}

export function sceneGaps(id, scene) {
  const problems = gaps(
    [{ id, preview: scene, examples: [] }],
    SUPPORT,
    'HarmonyOS'
  )
  const visit = value => {
    if (!value || typeof value !== 'object') return
    const props = value.props ?? {}
    if (
      value.$el === 'Slider' &&
      Array.isArray(props.defaultValue ?? props.value)
    )
      problems.push(`${id}: range sliders are not yet mapped on HarmonyOS`)
    if (['Select', 'RadioGroup', 'SegmentedControl'].includes(value.$el)) {
      const options = props.options ?? props.items
      if (
        !Array.isArray(options) ||
        options.some(
          o =>
            typeof o.label !== 'string' ||
            o.options ||
            o.icon ||
            (value.$el !== 'RadioGroup' && o.disabled)
        )
      )
        problems.push(
          `${id}: grouped, rich, dynamic or disabled options are not yet mapped on HarmonyOS`
        )
    }
    if (
      value.$el === 'Select' &&
      props.placeholder &&
      typeof props.placeholder !== 'string'
    )
      problems.push(
        `${id}: rich select placeholders are not yet mapped on HarmonyOS`
      )
    if (
      value.$el === 'TextField' &&
      props.type &&
      !['text', 'password', 'email', 'number', 'tel', 'url'].includes(
        props.type
      )
    )
      problems.push(
        `${id}: input type ${props.type} is not mapped on HarmonyOS`
      )
    if (
      ['Button', 'IconButton', 'Toggle'].includes(value.$el) &&
      value.children?.some(
        child => typeof child === 'object' && child.$el && child.$el !== 'Icon'
      )
    )
      problems.push(
        `${id}: composite button labels are not yet mapped on HarmonyOS`
      )
    if (
      ['Menu', 'ContextMenu'].includes(value.$el) &&
      (!Array.isArray(props.items) ||
        props.items.some(
          item =>
            Object.keys(item).some(
              key =>
                ![
                  'label',
                  'disabled',
                  'onSelect',
                  'icon',
                  'shortcut',
                  'checked',
                ].includes(key)
            ) || typeof item.label !== 'string'
        ))
    )
      problems.push(`${id}: rich menu items are not yet mapped on HarmonyOS`)
    if (value.$do?.some(action => !Object.hasOwn(action, 'set')))
      problems.push(`${id}: non-state actions are not yet mapped on HarmonyOS`)
    if (
      [
        'Badge',
        'Checkbox',
        'KeyCap',
        'Code',
        'Text',
        'SectionLabel',
        'DialogClose',
      ].includes(value.$el) &&
      value.children?.some(child => typeof child === 'object' && child.$el)
    )
      problems.push(`${id}: rich labels are not yet mapped on HarmonyOS`)
    for (const [key, child] of Object.entries(value)) {
      if (
        key.startsWith('$') &&
        !['$el', '$at', '$icon', '$do', '$state', '$eq', '$arg'].includes(key)
      )
        problems.push(`${id}: expression ${key} is not mapped on HarmonyOS`)
      if (Array.isArray(child)) child.forEach(visit)
      else visit(child)
    }
  }
  visit(scene)
  return [...new Set(problems)]
}

/** A typed wire format: the ArkTS interpreter needs neither any nor reflection. */
export function value(input) {
  if (input == null) return { kind: 'null' }
  if (typeof input === 'string') return { kind: 'string', text: input }
  if (typeof input === 'number') return { kind: 'number', number: input }
  if (typeof input === 'boolean') return { kind: 'boolean', boolean: input }
  if (Array.isArray(input)) return { kind: 'array', items: input.map(value) }
  if (input.$el)
    return {
      kind: 'element',
      name: input.$el,
      address: input.$at ?? -1,
      fields: Object.fromEntries(
        Object.entries(input.props ?? {}).map(([key, v]) => [key, value(v)])
      ),
      items: (input.children ?? []).map(value),
    }
  if (input.$icon) return { kind: 'icon', text: input.$icon }
  if (input.$state) return { kind: 'state', text: input.$state }
  if (input.$arg !== undefined) return { kind: 'argument', number: input.$arg }
  if (input.$eq) return { kind: 'eq', items: input.$eq.map(value) }
  if (input.$do) return { kind: 'actions', items: input.$do.map(value) }
  return {
    kind: 'object',
    fields: Object.fromEntries(
      Object.entries(input).map(([key, v]) => [key, value(v)])
    ),
  }
}

export function storiesData(stories) {
  const scene = (id, title, scene, note = '') => ({
    id,
    title,
    note,
    content: value(scene.content),
    initial: Object.fromEntries(
      Object.entries(scene.state ?? {}).map(([key, v]) => [key, value(v)])
    ),
    gaps: sceneGaps(id, scene),
  })
  return stories.map(story => ({
    id: story.id,
    name: story.name,
    summary: story.summary,
    preview: scene(`${story.id}/preview`, 'Preview', story.preview),
    examples: story.examples.map((example, i) =>
      scene(`${story.id}/${i}`, example.title, example.scene, example.note)
    ),
  }))
}

const header =
  '// Generated by tools/codegen from the React example. Do not edit.\n'
export function storiesETS(stories) {
  return `${header}import { Story } from '../lib/SceneTypes'\n\nexport const STORIES: Story[] = JSON.parse(${JSON.stringify(JSON.stringify(storiesData(stories)))}) as Story[]\n`
}
export function themesETS(groups) {
  return `${header}import { ThemeGroup } from '../lib/SceneTypes'\n\nexport const THEME_GROUPS: ThemeGroup[] = ${JSON.stringify(groups, null, 2)}\n`
}

export function coverageJSON(stories) {
  const scenes = storiesData(stories).flatMap(story => [
    story.preview,
    ...story.examples,
  ])
  return (
    JSON.stringify(
      {
        platform: 'harmony',
        total: scenes.length,
        mapped: scenes.filter(scene => scene.gaps.length === 0).length,
        gaps: Object.fromEntries(
          scenes
            .filter(scene => scene.gaps.length)
            .map(scene => [scene.id, scene.gaps])
        ),
      },
      null,
      2
    ) + '\n'
  )
}
