import { GLYPHS } from './schema/glyphs.mjs'
import { TRIM_CHARACTERS } from './schema/value.mjs'
// This is the adapter's capability contract, deliberately independent of schema
// growth. New content or events must not silently count as generated coverage.
const properties = {
  Box: 'width height maxWidth grow padding surface radius justify display gap align tone role wrap',
  Badge: 'variant tint size',
  Card: 'variant tint size',
  Callout: 'title tint size',
  EmptyState: 'title',
  Progress: 'value size tint gradient',
  Divider: 'tone orientation',
  SectionLabel: 'tint faint',
  StepList: '',
  Step: 'status label meta tint',
  Table: '',
  TableHead: '',
  TableRow: '',
  TableCell: 'head align',
  Button: 'variant tint size disabled expand shortcut',
  Page: 'title description',
  Section: 'title description',
  Stack: '',
  Grid: 'min',
  Specimen: 'label',
  Inline: 'tone fontSize mono',
  Value: '',
  Switch: 'checked disabled size tint label',
  Checkbox: 'checked disabled size tint',
  PreferenceSection: 'label',
  PreferenceRow: 'title subtitle',
  Icon: 'name size fit',
  Spinner: 'size tint onAccent',
  KeyCap: 'size variant',
  Row: 'gap',
  Col: 'width gap',
  Text: '',
  FormField: 'label hint state',
  TextField: 'placeholder readOnly disabled size state tint mono',
  Dialog: 'width tone',
  DialogHeader: 'title subtitle',
  DialogBody: '',
  DialogFooter: '',
  ActionBar: 'align pushEnd',
  SegmentedControl: 'label value size tint activeStyle stretch',
  SegmentedItem: 'value label disabled',
  Show: '',
  Tag: 'size variant tint selected disabled removeLabel',
  Link: 'href tint underline external disabled',
  Meter: 'value min max size tint label showValue',
  Skeleton: 'shape width height',
  SkeletonText: 'lines',
  IconButton: 'label size variant tint disabled',
  Toggle: 'label size variant tint disabled pressed',
  RadioGroup: 'label size tint direction gap value',
  RadioOption: 'value label disabled',
  PillTabs: 'label tint value',
  PillTab: 'value label count disabled',
  ToggleGroup: 'label size variant tint multiple value',
  ToggleItem: 'value label disabled pressed',
  OptionCard: 'title description selected tint disabled',
  Slider: 'label value end min max step size tint disabled showValue unit',
  NumberField: 'value min max step size state tint disabled placeholder unit',
  SearchField: 'placeholder size tint clearLabel',
  OtpField: 'length value groupAfter mask size state tint disabled',
}
const bindings = {
  TextField: 'value',
  Switch: 'checked',
  Checkbox: 'checked',
  SegmentedControl: 'value',
  Toggle: 'pressed',
  Tag: 'selected',
  Dialog: 'open',
  RadioGroup: 'value',
  PillTabs: 'value',
  ToggleGroup: 'value',
  ToggleItem: 'pressed',
  Slider: 'value end',
  NumberField: 'value',
  SearchField: 'value',
  OtpField: 'value',
}
const derived = {
  Button: 'disabled',
  Progress: 'value',
  Step: 'status meta',
  TextField: 'state',
  FormField: 'state hint',
  OptionCard: 'selected',
  EmptyState: 'title',
  NumberField: 'state',
  OtpField: 'state',
}
const slots = {
  Callout: 'icon action',
  EmptyState: 'action',
  OptionCard: 'title',
  SearchField: 'hint',
}
const fits = (record, allowed = '') =>
  Object.keys(record ?? {}).every(key => allowed.split(' ').includes(key))
const scalar = value => ['string', 'boolean', 'number'].includes(typeof value)
const expression = value =>
  scalar(value) ||
  (Array.isArray(value)
    ? value.every(expression)
    : value !== null &&
      typeof value === 'object' &&
      fits(
        value,
        'state if then else not and or lower length contains text trim concat eq add multiply gte'
      ) &&
      Object.values(value).every(expression))
export function androidSupports(scene) {
  const visit = node =>
    typeof node === 'string' ||
    (Object.hasOwn(properties, node.component) &&
      fits(node.props, properties[node.component]) &&
      fits(node.bind, bindings[node.component]) &&
      fits(node.derive, derived[node.component]) &&
      Object.values(node.derive ?? {}).every(expression) &&
      (!['Value', 'Show'].includes(node.component) || expression(node.value)) &&
      !(node.component === 'Tag' && node.slots?.icon) &&
      (node.component !== 'Icon' ||
        Object.hasOwn(GLYPHS, node.props?.name ?? '')) &&
      fits(
        node.on,
        {
          Button: 'press',
          IconButton: 'press',
          Link: 'press',
          Tag: 'select remove',
          OptionCard: 'press',
          SearchField: 'dismiss',
        }[node.component] ?? ''
      ) &&
      Object.values(node.on ?? {}).every(
        action =>
          fits(action, 'set value') &&
          (action.set === undefined || scalar(action.value))
      ) &&
      fits(node.slots, slots[node.component]) &&
      (node.children ?? []).every(visit) &&
      Object.values(node.slots ?? {}).every(visit))
  return (
    Object.values(scene.state ?? {}).every(scalar) &&
    (scene.timers ?? []).every(
      timer => fits(timer, 'every set value') && expression(timer.value)
    ) &&
    visit(scene.content)
  )
}
export function androidOutputs(stories, scenes) {
  return new Map([
    [
      'packages/mobile_ui_android/example/src/main/assets/scenes.json',
      JSON.stringify(
        stories
          .filter(s => scenes.has(s.id) && androidSupports(scenes.get(s.id)))
          .map(s => ({
            id: s.id,
            title: s.title,
            name: s.name,
            scene: scenes.get(s.id),
          })),
        null,
        2
      ) + '\n',
    ],
    [
      'packages/mobile_ui_android/example/src/main/kotlin/dev/dazzlabs/dazzui/mobile/example/GeneratedExpression.kt',
      `// Generated by tools/playgrounds. Edit the shared expression contract.\npackage dev.dazzlabs.dazzui.mobile.example\ninternal val sceneTrimCodeUnits = setOf(${[...TRIM_CHARACTERS].map(c => c.charCodeAt(0)).join(', ')})\n`,
    ],
  ])
}
