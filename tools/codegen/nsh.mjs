import { gaps, CHILDREN as C } from './support.mjs'
import { glyphSvgs, glyphSvg, kitGlyphNames } from './gpui.mjs'

// Keep this list beside the native scene mapper: a listed prop must be read.
export const SUPPORT = {
  ActionBar: [C, 'align'],
  FormField: [C, 'label', 'hint', 'state', 'tint'],
  Preferences: [C],
  PreferenceGroup: [C, 'title', 'description', 'action'],
  PreferenceSection: [C, 'label', 'footer', 'action'],
  PreferenceRow: [C, 'title', 'subtitle', 'icon', 'onOpen', 'tint'],
  Menu: ['trigger', 'items'],
  KeyCap: [C, 'variant', 'size'],
  SectionLabel: [C, 'faint', 'tint'],
  EmptyState: [C, 'title', 'action'],
  Avatar: ['name', 'size', 'shape', 'tint', 'fallback', 'src'],
  Callout: [C, 'title', 'icon', 'action', 'size', 'tint'],
  Collapsible: [C, 'title', 'defaultOpen', 'open', 'onOpenChange'],
  RadioGroup: [
    'options',
    'value',
    'defaultValue',
    'onValueChange',
    'size',
    'tint',
    'aria-label',
  ],
  SegmentedControl: [
    'items',
    'value',
    'defaultValue',
    'onValueChange',
    'size',
    'tint',
    'stretch',
    'aria-label',
  ],
  Row: [C, 'gap', 'align', 'justify', 'width'],
  Col: [C, 'gap', 'align', 'width'],
  Frame: [C, 'width', 'height'],
  Text: [C, 'strong', 'tone', 'tint'],
  Button: [
    C,
    'size',
    'variant',
    'tint',
    'disabled',
    'fullWidth',
    'shortcut',
    'onClick',
  ],
  IconButton: [C, 'size', 'variant', 'tint', 'disabled', 'label', 'onClick'],
  Icon: ['icon', 'size'],
  Badge: [C, 'size', 'variant', 'tint'],
  Card: [C, 'size', 'variant'],
  Checkbox: [
    C,
    'size',
    'tint',
    'disabled',
    'checked',
    'defaultChecked',
    'indeterminate',
    'note',
    'onCheckedChange',
  ],
  Switch: [
    'size',
    'tint',
    'disabled',
    'checked',
    'defaultChecked',
    'onCheckedChange',
    'aria-label',
  ],
  Toggle: [
    C,
    'size',
    'variant',
    'tint',
    'disabled',
    'pressed',
    'defaultPressed',
    'onPressedChange',
    'label',
  ],
  TextField: [
    'size',
    'tint',
    'disabled',
    'value',
    'defaultValue',
    'onValueChange',
    'placeholder',
    'readOnly',
    'mono',
    'state',
    'aria-label',
  ],
  Select: [
    'size',
    'tint',
    'disabled',
    'value',
    'defaultValue',
    'onValueChange',
    'placeholder',
    'options',
    'aria-label',
  ],
  Slider: [
    'size',
    'tint',
    'disabled',
    'value',
    'defaultValue',
    'onValueChange',
    'min',
    'max',
    'step',
    'aria-label',
  ],
  Progress: ['size', 'tint', 'value', 'aria-label'],
  Divider: ['orientation'],
}

Object.assign(SUPPORT, {
  Accordion: [
    C,
    'value',
    'defaultValue',
    'onValueChange',
    'multiple',
    'divided',
  ],
  AccordionItem: [C, 'value', 'title'],
  AvatarGroup: [C, 'max'],
  Code: [C],
  CodeBlock: ['code', 'language', 'title', 'copyable', 'lineNumbers'],
  NumberField: [
    'defaultValue',
    'min',
    'max',
    'step',
    'unit',
    'aria-label',
    'onValueChange',
  ],
  OtpField: [
    'length',
    'groupAfter',
    'size',
    'state',
    'aria-label',
    'onValueChange',
  ],
  SearchField: ['value', 'placeholder', 'aria-label', 'onValueChange'],
  Combobox: [
    'options',
    'defaultValue',
    'placeholder',
    'clearable',
    'empty',
    'onValueChange',
  ],
  Skeleton: ['width', 'height', 'shape'],
  SkeletonText: ['lines'],
  Spinner: ['size', 'onAccent'],
  Meter: ['value', 'label', 'showValue', 'size', 'tint', 'aria-label'],
  StepList: [C],
  Step: ['label', 'meta', 'status'],
  Table: [C],
  TableHead: [C],
  TableRow: [C],
  TableCell: [C, 'head', 'align'],
  OptionCard: [C, 'title', 'description', 'selected', 'disabled', 'onClick'],
  Link: [C, 'href', 'external', 'tint', 'underline'],
  PillTabs: ['items', 'value', 'defaultValue', 'onValueChange', 'aria-label'],
  ToggleGroup: [
    'items',
    'value',
    'multiple',
    'size',
    'aria-label',
    'onValueChange',
  ],
  Tag: [C, 'tint', 'selected', 'onSelectedChange', 'onRemove'],
  Dialog: [C, 'trigger', 'width', 'tone'],
  DialogHeader: ['title', 'subtitle'],
  DialogBody: [C],
  DialogFooter: [C],
  DialogClose: ['render'],
  Drawer: [C, 'trigger', 'title', 'description', 'side', 'footer'],
  DrawerClose: ['render'],
  Popover: [C, 'trigger', 'title', 'description', 'side', 'width'],
  PopoverClose: ['render'],
  PreviewCard: [C, 'trigger', 'width'],
  Tooltip: [C, 'label', 'side'],
})
Object.assign(SUPPORT, {
  Calendar: [
    'value',
    'defaultValue',
    'onValueChange',
    'mode',
    'months',
    'weekStartsOn',
    'today',
    'min',
    'max',
    'size',
    'tint',
  ],
  DateRangePicker: [
    'value',
    'defaultValue',
    'onValueChange',
    'today',
    'months',
    'presets',
    'durations',
    'timeStep',
    'disabled',
    'state',
    'size',
    'tint',
    'aria-label',
  ],
  ToastProvider: [C],
  ToastViewport: ['placement'],
  Form: [C, 'onSubmit'],
  Fieldset: [C, 'legend'],
  Autocomplete: ['items', 'placeholder', 'onValueChange'],
  ShortcutRecorder: [
    'value',
    'disabled',
    'state',
    'onValueChange',
    'onClear',
    'aria-label',
  ],
  Splitter: [C, 'orientation'],
  SplitterPanel: [C, 'defaultSize', 'minSize', 'maxSize'],
  Tree: [
    'items',
    'defaultExpanded',
    'selected',
    'onSelectedChange',
    'size',
    'aria-label',
  ],
  ContextMenu: [C, 'items'],
  Menubar: ['menus'],
  SidebarProvider: [C],
  Sidebar: [C, 'variant', 'collapsible'],
  SidebarHeader: [C],
  SidebarContent: [C],
  SidebarGroup: [C],
  SidebarGroupContent: [C],
  SidebarGroupLabel: [C],
  SidebarMenu: [C],
  SidebarMenuItem: [C],
  SidebarMenuButton: [C, 'icon', 'active', 'tooltip', 'onClick'],
  SidebarMenuBadge: [C],
  SidebarInset: [C],
  SidebarTrigger: [],
})
SUPPORT.Button.push('type')
SUPPORT.Slider.push('showValue')

SUPPORT.TextField.push('multiline', 'rows', 'type', 'required')
SUPPORT.Collapsible.push('meta')
SUPPORT.SegmentedControl.push('iconOnly', 'activeStyle')
SUPPORT.Progress.push('gradient')
SUPPORT.Divider.push('tone')

export function sceneGaps(id, scene) {
  const problems = gaps(
    [{ id, preview: scene, examples: [] }],
    SUPPORT,
    'Native Shell'
  )
  const visit = value => {
    if (!value || typeof value !== 'object') return
    if (value.$el === 'Badge' && value.props?.variant === 'weak')
      problems.push(`${id}: Badge variant ${value.props.variant} is not mapped`)
    if (value.$el === 'RadioGroup') {
      const items = value.props?.items ?? value.props?.options
      if (
        !Array.isArray(items) ||
        items.some(
          o =>
            typeof o.label !== 'string' ||
            Object.keys(o).some(
              k => !['value', 'label', 'disabled'].includes(k)
            )
        )
      )
        problems.push(`${id}: rich choice items are not mapped`)
    }
    if (['Menu', 'ContextMenu', 'Menubar'].includes(value.$el)) {
      const validMenu = items =>
        Array.isArray(items) &&
        items.every(item =>
          item.type === 'separator'
            ? Object.keys(item).every(k => k === 'type')
            : typeof item.label === 'string' &&
              Object.keys(item).every(k =>
                [
                  'label',
                  'icon',
                  'shortcut',
                  'disabled',
                  'onSelect',
                  'checked',
                  'items',
                ].includes(k)
              ) &&
              (!item.items || validMenu(item.items))
        )
      const valid =
        value.$el === 'Menubar'
          ? Array.isArray(value.props?.menus) &&
            value.props.menus.every(
              m =>
                typeof m.label === 'string' &&
                Object.keys(m).every(k => ['label', 'items'].includes(k)) &&
                validMenu(m.items)
            )
          : validMenu(value.props?.items)
      if (!valid) problems.push(`${id}: unsupported menu item data`)
    }
    if (['SegmentedControl', 'PillTabs', 'ToggleGroup'].includes(value.$el)) {
      const items = value.props?.items
      if (
        !Array.isArray(items) ||
        items.some(
          o =>
            typeof o.value !== 'string' ||
            Object.keys(o).some(
              k =>
                ![
                  'value',
                  'label',
                  'disabled',
                  'icon',
                  'hint',
                  'count',
                ].includes(k)
            )
        )
      )
        problems.push(`${id}: unsupported choice item data`)
    }
    if (value.$el === 'Tree') {
      const validTree = items =>
        Array.isArray(items) &&
        items.every(
          o =>
            typeof o.id === 'string' &&
            typeof o.label === 'string' &&
            Object.keys(o).every(k =>
              ['id', 'label', 'icon', 'meta', 'disabled', 'children'].includes(
                k
              )
            ) &&
            (!o.children || validTree(o.children))
        )
      if (!validTree(value.props?.items))
        problems.push(`${id}: unsupported tree item data`)
    }
    if (value.$el === 'Select' || value.$el === 'Combobox') {
      const options = value.props?.options
      const valid = items =>
        Array.isArray(items) &&
        items.every(o =>
          o.options
            ? typeof o.label === 'string' &&
              Object.keys(o).every(k => ['label', 'options'].includes(k)) &&
              valid(o.options)
            : typeof o.label === 'string' &&
              Object.keys(o).every(k =>
                ['value', 'label', 'disabled'].includes(k)
              )
        )
      if (!valid(options))
        problems.push(`${id}: rich select options are not mapped`)
      if (
        value.props?.placeholder &&
        typeof value.props.placeholder !== 'string'
      )
        problems.push(`${id}: rich select placeholders are not mapped`)
    }
    if (
      '$do' in value &&
      value.$do.some(action => !('set' in action) && !('toast' in action))
    )
      problems.push(`${id}: action other than a state setter is not mapped`)
    Object.values(value).forEach(v =>
      Array.isArray(v) ? v.forEach(visit) : visit(v)
    )
  }
  visit(scene)
  return [...new Set(problems)]
}

export function storyData(story) {
  const scene = (id, title, value, note) => ({
    id,
    title,
    ...(note ? { note } : {}),
    ...value,
    gaps: sceneGaps(id, value),
  })
  return {
    id: story.id,
    name: story.name,
    summary: story.summary,
    preview: scene(`${story.id}/preview`, 'Preview', story.preview),
    examples: story.examples.map((e, i) =>
      scene(`${story.id}/${i}`, e.title, e.scene, e.note)
    ),
  }
}

export function unsupported(stories) {
  return stories.flatMap(story => {
    const data = storyData(story)
    return [data.preview, ...data.examples].flatMap(scene => scene.gaps)
  })
}

export function outputs(root, stories, themes) {
  const base = 'packages/rust/dazzui-nsh'
  const data = stories.map(storyData)
  const files = new Map(
    data.map(story => [
      `${base}/example/src/stories/${story.id}.json`,
      JSON.stringify(story, null, 2) + '\n',
    ])
  )
  files.set(
    `${base}/example/src/stories/mod.rs`,
    `// Generated by tools/codegen from React stories. Do not edit.\npub const STORIES: &[&str] = &[\n${data.map(s => `    include_str!(${JSON.stringify(s.id + '.json')}),`).join('\n')}\n];\n`
  )
  files.set(
    `${base}/example/src/generated/themes.json`,
    JSON.stringify(themes, null, 2) + '\n'
  )
  files.set(
    `${base}/example/coverage.json`,
    JSON.stringify(
      data
        .flatMap(s => [s.preview, ...s.examples])
        .map(s => ({ id: s.id, supported: s.gaps.length === 0, gaps: s.gaps })),
      null,
      2
    ) + '\n'
  )
  const icons = glyphSvgs(root, stories)
  for (const name of kitGlyphNames(root)) icons.set(name, glyphSvg(root, name))
  files.set(
    `${base}/src/generated/glyphs.rs`,
    `// Generated from React's Fluent icons by tools/codegen. Do not edit.\npub const GLYPHS: &[(&str, &str)] = &[\n${[
      ...icons,
    ]
      .sort(([a], [b]) => a.localeCompare(b))
      .map(
        ([name, svg]) =>
          `    (${JSON.stringify(name)}, r#"${svg.replaceAll('fill="#000"', 'fill="currentColor"')}"#),`
      )
      .join('\n')}\n];\n`
  )
  return files
}
