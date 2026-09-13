import { GLYPHS } from './schema/glyphs.mjs'
import { spacing } from './tokens.mjs'
import { valueType } from './schema/value.mjs'
const q = value =>
  JSON.stringify(value).replace(/\\u([0-9a-f]{4})/gi, '\\u{$1}')
const cap = value => value[0].toUpperCase() + value.slice(1)
const number = value => `${value}${Number.isInteger(value) ? '.0' : ''}`
/** A declared length: logical pixels or a spacing step. */
const len = value =>
  typeof value === 'number' ? number(value) : spacing.inset(value)
const properties = {
  Button: 'variant size tint disabled expand shortcut',
  ActionBar: 'align pushEnd',
  Badge: 'variant size tint',
  Card: 'variant size tint',
  Row: 'gap',
  Col: 'gap width',
  Inline: 'tone fontSize mono',
  Text: '',
  Icon: 'name size fit',
  TextField: 'placeholder size state mono tint readOnly disabled',
  FormField: 'label hint state',
  Value: '',
  PreferenceSection: 'label',
  PreferenceRow: 'title subtitle',
  Box: 'width height maxWidth grow padding surface radius justify display gap align tone role wrap',
  Page: 'title description',
  Section: 'title description',
  Stack: '',
  Grid: 'min',
  Specimen: 'label',
  Spinner: 'size tint onAccent',
  Progress: 'value size tint gradient',
  KeyCap: 'size variant',
  SectionLabel: 'tint faint',
  Divider: 'tone orientation',
  Checkbox: 'checked disabled size tint',
  Switch: 'checked disabled size tint label',
  EmptyState: 'title',
  Callout: 'title tint size',
  Step: 'status label meta tint',
  StepList: '',
  Table: '',
  TableHead: '',
  TableRow: '',
  TableCell: 'head align',
  Dialog: 'width',
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
  DialogHeader: 'title subtitle',
  DialogBody: '',
  DialogFooter: '',
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

// The most recently compiled scene's expression compiler, reused for timers.
let expressionFor = null
export function insetScene(scene) {
  if (
    Object.values(scene.state ?? {}).some(
      value => !['boolean', 'string', 'number'].includes(typeof value)
    )
  )
    throw Error('Inset scene state adapter pending')
  const expression = value => {
    valueType(value, scene.state ?? {})
    if (typeof value === 'string') return `${q(value)}.to_string()`
    if (typeof value === 'boolean') return String(value)
    if (typeof value === 'number') return number(value)
    if ('state' in value) {
      const read = `app.read(state)[${q(value.state)}]`
      return typeof scene.state[value.state] === 'boolean'
        ? `${read}.as_bool().unwrap()`
        : typeof scene.state[value.state] === 'string'
          ? `${read}.as_str().unwrap().to_string()`
          : `${read}.as_f64().unwrap()`
    }
    if ('if' in value)
      return `if ${expression(value.if)} { ${expression(value.then)} } else { ${expression(value.else)} }`
    for (const [key, op] of [
      ['add', '+'],
      ['multiply', '*'],
      ['gte', '>='],
    ])
      if (key in value)
        return `((${expression(value[key][0])}) ${op} (${expression(value[key][1])}))`
    if ('not' in value) return `(!(${expression(value.not)}))`
    if ('and' in value)
      return `(${value.and.map(v => `(${expression(v)})`).join(' && ')})`
    if ('or' in value)
      return `(${value.or.map(v => `(${expression(v)})`).join(' || ')})`
    if ('lower' in value) return `(${expression(value.lower)}).to_lowercase()`
    if ('length' in value)
      return `((${expression(value.length)}).chars().count() as f64)`
    if ('contains' in value)
      return `(${expression(value.contains[0])}).contains((${expression(value.contains[1])}).as_str())`
    if ('eq' in value)
      return `(${expression(value.eq[0])}) == (${expression(value.eq[1])})`
    if ('text' in value) return `(${expression(value.text)}).to_string()`
    if ('trim' in value)
      return `(${expression(value.trim)}).trim_matches(|c| crate::generated_stories::TRIM_CODE_POINTS.contains(&(c as u32))).to_string()`
    if ('concat' in value)
      return `vec![${value.concat.map(v => `(${expression(v)}).to_string()`).join(',')}].concat()`
    throw Error('Inset expression pending')
  }
  expressionFor = expression
  // A child list; a `Show` entry pushes its items only while it holds, so a
  // hidden branch takes no slot and no spacing.
  const vecOf = children =>
    children.some(child => typeof child === 'object')
      ? `{ let mut children: Vec<WidgetRef> = Vec::new(); ${children.map(child => (typeof child === 'object' ? `if ${child.show} { ${child.items.map(item => `children.push(${item});`).join(' ')} }` : `children.push(${child});`)).join(' ')} children }`
      : `vec![${children.join(',')}]`
  const row = (children, gap = 'v.spacing_3', stretch = false) =>
    `Row::new().main_axis_size(MainAxisSize::Min).cross_axis_alignment(CrossAxisAlignment::${stretch ? 'Stretch' : 'Center'}).spacing(${gap}).children(${vecOf(children)}).into_widget()`
  const col = (children, gap = 'v.spacing_3') =>
    `Column::new().main_axis_size(MainAxisSize::Min).cross_axis_alignment(CrossAxisAlignment::Start).spacing(${gap}).children(${vecOf(children)}).into_widget()`
  const text = (label, style) =>
    `Text::new(${q(label)}).style(${style}).into_widget()`
  const step = n => {
    if (
      Object.keys(n.props ?? {}).some(
        k => !['label', 'status', 'meta', 'tint'].includes(k)
      ) ||
      Object.keys(n.derive ?? {}).some(k => !['status', 'meta'].includes(k)) ||
      n.bind ||
      n.on ||
      n.slots
    )
      throw Error('Inset Step property pending')
    const status = n.derive?.status
      ? `match (${expression(n.derive.status)}).as_str() { "done" => ui::StepStatus::Done, "active" => ui::StepStatus::Active, _ => ui::StepStatus::Idle }`
      : `ui::StepStatus::${cap(n.props.status)}`
    const meta = n.derive?.meta
      ? `{ let meta = ${expression(n.derive.meta)}; (!meta.is_empty()).then(|| meta.into()) }`
      : n.props.meta !== undefined
        ? `Some(${q(n.props.meta)}.into())`
        : 'None'
    return `ui::Step { tint: ui::Tint::${cap(n.props.tint ?? 'primary')}, label: ${q(n.props.label)}.into(), status: ${status}, meta: ${meta} }`
  }
  function node(n, parent) {
    if (typeof n === 'string') return `Text::new(${q(n)}).into_widget()`
    if (!(n.component in properties))
      throw Error(`Inset adapter pending: ${n.component}`)
    const bindings = {
      TextField: { value: 'string' },
      Switch: { checked: 'boolean' },
      Checkbox: { checked: 'boolean' },
      Dialog: { open: 'boolean' },
      SegmentedControl: { value: 'string' },
      Toggle: { pressed: 'boolean' },
      Tag: { selected: 'boolean' },
      RadioGroup: { value: 'string' },
      PillTabs: { value: 'string' },
      ToggleGroup: { value: 'string' },
      ToggleItem: { pressed: 'boolean' },
      Slider: { value: 'number', end: 'number' },
      NumberField: { value: 'string' },
      SearchField: { value: 'string' },
      OtpField: { value: 'string' },
    }
    if (
      Object.entries(n.bind ?? {}).some(
        ([key, name]) =>
          bindings[n.component]?.[key] === undefined ||
          typeof scene.state?.[name] !== bindings[n.component][key]
      )
    )
      throw Error('Inset binding pending')
    const derives = {
      Button: ['disabled'],
      Progress: ['value'],
      Step: ['status', 'meta'],
      FormField: ['state', 'hint'],
      TextField: ['state'],
      OptionCard: ['selected'],
      EmptyState: ['title'],
      NumberField: ['state'],
      OtpField: ['state'],
    }
    if (
      n.derive &&
      Object.keys(n.derive).some(
        key => !(derives[n.component] ?? []).includes(key)
      )
    )
      throw Error('Inset derived property pending')
    const events = {
      Button: ['press'],
      IconButton: ['press'],
      Link: ['press'],
      Tag: ['select', 'remove'],
      OptionCard: ['press'],
      SearchField: ['dismiss'],
    }
    for (const [event, action] of Object.entries(n.on ?? {}))
      if (
        !(events[n.component] ?? []).includes(event) ||
        (action.set !== undefined &&
          typeof scene.state?.[action.set] !== typeof action.value)
      )
        throw Error('Inset event pending')
    /** A callback for a declared event, or null when it is not declared. */
    const handler = (event, parameters = 'app') => {
      const action = n.on?.[event]
      if (!action) return null
      return action.set === undefined
        ? `|${parameters.replace(/\w+/g, '_')}| {}`
        : `{ let state = state.clone(); move |${parameters}| crate::scene_state::set(app, &state, ${q(action.set)}, ${expression(action.value)}) }`
    }
    const p = n.props ?? {}
    if (
      Object.keys(p).some(k => !properties[n.component].split(' ').includes(k))
    )
      throw Error(`Inset property pending: ${n.component}`)
    if (
      Object.keys(n.slots ?? {}).some(
        k =>
          !(
            (['EmptyState', 'Callout'].includes(n.component) &&
              k === 'action') ||
            (n.component === 'Callout' && k === 'icon') ||
            (n.component === 'OptionCard' && k === 'title') ||
            (n.component === 'SearchField' && k === 'hint')
          )
      )
    )
      throw Error('Inset slot pending')
    const horizontal =
      ['Row', 'ActionBar', 'Specimen', 'DialogFooter'].includes(n.component) ||
      (n.component === 'Box' && !!p.display)
    const children = (n.children ?? []).flatMap(child => {
      if (typeof child === 'object' && child.component === 'Show')
        return [
          {
            show: expression(child.value),
            items: (child.children ?? []).map(item => node(item, n)),
          },
        ]
      let rendered = node(child, n)
      if (
        horizontal &&
        (child.props?.grow ||
          (n.component === 'ActionBar' && child.component === 'TextField'))
      )
        rendered = `Expanded::new(${rendered}).into_widget()`
      if (horizontal && child.props?.pushEnd)
        return [
          `Expanded::new(Row::new().main_axis_size(MainAxisSize::Max).main_axis_alignment(MainAxisAlignment::End).children(vec![${rendered}])).into_widget()`,
        ]
      return [rendered]
    })
    const parentHorizontal =
      parent &&
      (['Row', 'ActionBar', 'Specimen', 'DialogFooter'].includes(
        parent.component
      ) ||
        (parent.component === 'Box' && !!parent.props?.display))
    if ((p.grow || p.pushEnd) && !parentHorizontal)
      throw Error('Inset flex item requires a horizontal parent')
    const label = () => {
      if ((n.children ?? []).some(c => typeof c !== 'string'))
        throw Error('Inset rich label pending')
      return (n.children ?? []).join('')
    }
    const derived = (key, fallback) =>
      expression(n.derive?.[key] ?? p[key] ?? fallback)
    const size = `ui::ControlSize::${cap(typeof p.size === 'string' ? p.size : 'medium')}`
    const tint = `ui::Tint::${cap(p.tint ?? 'primary')}`
    const vertical = (n.children ?? []).some(
      c => c.component === 'Divider' && c.props?.orientation === 'vertical'
    )
    const horizontalGap = p.gap
      ? spacing.inset(p.gap)
      : n.component === 'Box'
        ? '0.0'
        : 'v.spacing_3'
    const grouped = vertical
      ? `IntrinsicHeight::new().child(${row(children, horizontalGap, true)}).into_widget()`
      : col(children)
    let view
    switch (n.component) {
      case 'Button': {
        const content = [...children]
        if (p.shortcut)
          content.push(`ui::KeyCap::new(${q(p.shortcut)}).into_widget()`)
        const buttonDisabled = derived('disabled', false)
        const press = n.on
          ? `move |app| crate::scene_state::set(app, &state, ${q(n.on.press.set)}, ${expression(n.on.press.value)})`
          : '|_| {}'
        view = `{ ${n.on ? 'let state = state.clone();' : ''} let disabled = ${buttonDisabled}; let control = ui::Button::child(${row(content, 'v.control_small_gap')}).size(ui::ControlSize::${cap(p.size ?? 'small')}).variant(ui::ControlVariant::${cap(p.variant ?? 'recessed')}).tint(${tint}).expand(${!!p.expand}); if disabled { control.into_widget() } else { control.on_pressed(${press}).into_widget() } }`
        break
      }
      case 'TextField':
        if (!n.bind)
          throw Error('Inset TextField requires a shared value binding')
        view = `{ let invalid = (${derived('state', 'default')}) == "error"; let controller = state.controller(app, ${q(n.bind.value)}); let state = state.clone(); ui::TextField::new().controller(controller).placeholder(${q(p.placeholder ?? '')}).size(${size}).invalid(invalid).disabled(${!!p.disabled}).read_only(${!!p.readOnly}).mono(${!!p.mono}).tint(${tint}).on_changed(move |app, value| crate::scene_state::set(app, &state, ${q(n.bind.value)}, value)).into_widget() }`
        break
      case 'FormField':
        view = `{ let mut control = ui::FormField::new(${q(p.label ?? '')}, ${col(children, '0.0')}); control.label = ${p.label === undefined ? 'None' : `Some(${q(p.label)}.to_string())`}; control.hint = ${n.derive?.hint || p.hint !== undefined ? `Some(${derived('hint', '')})` : 'None'}; control.invalid = (${derived('state', 'default')}) == "error"; control.into_widget() }`
        break
      case 'Value':
        view = `Text::new((${expression(n.value)}).to_string()).into_widget()`
        break
      case 'PreferenceSection':
        view = `ui::PreferenceGroup::new(${q(p.label)}, ${vecOf(children)}).into_widget()`
        break
      case 'PreferenceRow':
        view = `{ let mut control = ui::PreferenceRow::new(${q(p.title)}, ${row(children, '0.0')}); control.description = ${q(p.subtitle ?? '')}.into(); control.into_widget() }`
        break
      case 'Table':
      case 'TableHead':
      case 'TableRow':
        view = `ui::${n.component}::new(${vecOf(children)}).into_widget()`
        break
      case 'TableCell':
        view = `ui::TableCell::new(${row(children, '0.0')}).head(${!!p.head}).end(${p.align === 'end'}).into_widget()`
        break
      case 'ActionBar':
        view = `ui::ActionBar::new(${vecOf(children)}).align(MainAxisAlignment::${{ start: 'Start', end: 'End', between: 'SpaceBetween' }[p.align ?? 'start']}).stretch(${vertical}).into_widget()`
        if (vertical)
          view = `IntrinsicHeight::new().child(${view}).into_widget()`
        break
      case 'Icon':
        if (!Object.hasOwn(GLYPHS, p.name)) throw Error('Inset glyph unknown')
        view = `ui::FluentIcon(crate::generated_glyphs::glyph(${q(p.name)})).into_widget()`
        if (p.size)
          view = `IconTheme::merge(None, IconThemeData::new().size(${typeof p.size === 'string' ? `v.spacing_${p.size.split('.')[1]}` : number(p.size)}), ${view}).into_widget()`
        break
      case 'Badge':
        view = `ui::Badge::new("").child(${row(children, 'v.spacing_1')}).size(${size}).variant(ui::BadgeVariant::${cap(p.variant ?? 'tinted')}).tint(${tint}).into_widget()`
        break
      case 'Card':
        view = `ui::Card::new(${col(children, '0.0')}).size(${size}).variant(ui::CardVariant::${cap(p.variant ?? 'raised')}).tint(${tint}).into_widget()`
        break
      case 'Row':
        view = vertical
          ? grouped
          : row(children, p.gap ? spacing.inset(p.gap) : 'v.spacing_3')
        break
      case 'Col':
        view = `SizedBox::new()${p.width ? `.width(${len(p.width)})` : ''}.child(${col(children, p.gap ? spacing.inset(p.gap) : 'v.spacing_3')}).into_widget()`
        break
      case 'Inline': {
        const style = `TextStyle::new()${p.tone ? `.color(v.color_content_${p.tone})` : ''}${p.fontSize ? `.font_size(v.${p.fontSize.replace('.', '_')}.size)` : ''}${p.mono ? '.font_family("monospace")' : ''}`
        const inner = p.tone
          ? `IconTheme::merge(None, IconThemeData::new().color(v.color_content_${p.tone}), ${row(children, '0.0')}).into_widget()`
          : row(children, '0.0')
        view = `DefaultTextStyle::new(DefaultTextStyle::of(app, context).style.merge(Some(&${style})), ${inner}).into_widget()`
        break
      }
      case 'Text':
        view = text(
          label(),
          'v.body_medium.text_style().color(v.color_content_secondary)'
        )
        break
      case 'Page':
      case 'Section': {
        const title = text(
          p.title,
          `v.${n.component === 'Page' ? 'headline_medium' : 'title_medium'}.text_style()`
        )
        const body = p.description
          ? [
              text(
                p.description,
                'v.body_small.text_style().color(v.color_content_muted)'
              ),
            ]
          : []
        view = col(
          [title, ...body, ...children],
          n.component === 'Page' ? 'v.spacing_8' : 'v.spacing_4'
        )
        if (n.component === 'Page')
          view = `Padding::new(EdgeInsetsGeometry::all(v.spacing_8)).child(${view}).into_widget()`
        break
      }
      case 'Stack':
        view = col(children, 'v.spacing_3')
        break
      case 'Specimen':
        view = `crate::storybook::stage::specimen(${q(p.label)}, ${vertical ? grouped : row(children)})`
        break
      case 'Grid':
        view = `crate::storybook::stage::grid(${number(p.min ?? 256)}, ${vecOf(children)})`
        break
      case 'Spinner':
        view = `ui::Spinner { size: ${size}, tint: ${tint}, on_accent: ${!!p.onAccent} }.into_widget()`
        break
      case 'Progress':
        view = `ui::Progress { gradient: ${!!p.gradient}, value: ${n.derive?.value ? `Some((${expression(n.derive.value)}).clamp(0.0, 100.0) / 100.0)` : p.value === undefined ? 'None' : `Some(${number(p.value / 100)})`}, size: ui::ControlSize::${cap(p.size ?? 'small')}, tint: ${tint} }.into_widget()`
        break
      case 'KeyCap':
        view = `ui::KeyCap { label: ${q(label())}.into(), size: ${size}, variant: ui::KeyCapVariant::${cap(p.variant ?? 'hint')} }.into_widget()`
        break
      case 'SectionLabel':
        view = `ui::SectionLabel { label: ${q(label())}.into(), faint: ${!!p.faint}, tint: ${p.tint ? `Some(${tint})` : 'None'} }.into_widget()`
        break
      case 'Divider':
        view = `ui::Divider { vertical: ${p.orientation === 'vertical'}, muted: ${p.tone === 'muted'} }.into_widget()`
        break
      case 'IconButton': {
        const press = n.on
          ? `{ let state = state.clone(); move |app| crate::scene_state::set(app, &state, ${q(n.on.press.set)}, ${expression(n.on.press.value)}) }`
          : '|_| {}'
        view = `{ let control = ui::IconButton::new(${children[0]}).size(ui::ControlSize::${cap(p.size ?? 'tiny')}).variant(ui::ControlVariant::${cap(p.variant ?? 'plain')}).tint(ui::Tint::${cap(p.tint ?? 'neutral')}); ${p.disabled ? 'control.into_widget()' : `control.on_pressed(${press}).into_widget()`} }`
        break
      }
      case 'Toggle': {
        const pressed = n.bind
          ? `app.read(&state.values)[${q(n.bind.pressed)}].as_bool().unwrap()`
          : String(!!p.pressed)
        const changed = n.bind
          ? `{ let state = state.clone(); move |app, value| crate::scene_state::set(app, &state, ${q(n.bind.pressed)}, value) }`
          : '|_, _| {}'
        const fitted = (n.children ?? []).some(
          c => typeof c === 'object' && c.props?.fit
        )
        const content = fitted
          ? `IconTheme::merge(None, IconThemeData::new().size(v.metrics(ui::ControlSize::${cap(p.size ?? 'tiny')}).content.size), ${row(children, 'v.control_small_gap')}).into_widget()`
          : row(children, 'v.control_small_gap')
        view = `{ let mut control = ui::Toggle::new("").pressed(${pressed}).on_changed(${changed}); control.child = ${content}; control.enabled = ${!p.disabled}; control.size = ui::ControlSize::${cap(p.size ?? 'tiny')}; control.variant = ui::ControlVariant::${cap(p.variant ?? 'plain')}; control.tint = ui::Tint::${cap(p.tint ?? 'neutral')}; control.into_widget() }`
        break
      }
      case 'Show':
        view = `if ${expression(n.value)} { ${col(children, '0.0')} } else { SizedBox::shrink().into_widget() }`
        break
      case 'Tag': {
        const selectable = n.bind || n.on?.select
        const selected = n.bind
          ? `app.read(&state.values)[${q(n.bind.selected)}].as_bool().unwrap()`
          : String(!!p.selected)
        const select = n.bind
          ? `{ let state = state.clone(); move |app, value| crate::scene_state::set(app, &state, ${q(n.bind.selected)}, value) }`
          : n.on?.select?.set !== undefined
            ? `{ let state = state.clone(); move |app, _| crate::scene_state::set(app, &state, ${q(n.on.select.set)}, ${expression(n.on.select.value)}) }`
            : '|_, _| {}'
        const remove = handler('remove')
        view = `{ let mut control = ui::Tag::new(${q((n.children ?? []).filter(c => typeof c === 'string').join(''))}); control.size = ${size}; control.tint = ${tint}; control.variant = ui::BadgeVariant::${cap(p.variant ?? 'tinted')}; control.disabled = ${!!p.disabled}; ${selectable ? `control.selected = Some(${selected}); control.on_selected_changed = Some(std::rc::Rc::new(${select}));` : ''} ${remove ? `control.on_remove = Some(std::rc::Rc::new(${remove}));` : ''} control.into_widget() }`
        break
      }
      case 'Link': {
        const press = p.disabled ? null : (handler('press') ?? '|_| {}')
        view = `{ let mut control = ui::Link::new(${q((n.children ?? []).join(''))}); control.tint = ${tint}; control.underline = ui::LinkUnderline::${cap(p.underline ?? 'hover')}; control.external = ${!!p.external}; ${press ? `control.on_pressed = Some(std::rc::Rc::new(${press}));` : ''} control.into_widget() }`
        break
      }
      case 'Meter': {
        const min = p.min ?? 0,
          max = p.max ?? 100
        view = `{ let mut control = ui::Meter::new(${number(p.value)}, ${number(min)}, ${number(max)}); control.size = ${size}; control.tint = ${tint}; control.label = ${p.label ? `Some(${q(p.label)}.to_string())` : 'None'}; control.value_label = ${p.showValue ? `Some(${q(`${Math.round(((p.value - min) / (max - min)) * 100)}%`)}.to_string())` : 'None'}; control.into_widget() }`
        break
      }
      case 'Skeleton': {
        const percent = typeof p.width === 'string' && p.width.endsWith('%')
        view = `ui::Skeleton { shape: ui::SkeletonShape::${cap(p.shape ?? 'text')}, width: ${p.width !== undefined && !percent ? `Some(${len(p.width)})` : 'None'}, height: ${p.height !== undefined ? `Some(${len(p.height)})` : 'None'} }.into_widget()`
        if (percent)
          view = `FractionallySizedBox::new().width_factor(${number(parseFloat(p.width) / 100)}).alignment(AlignmentGeometry::CENTER_LEFT).child(${view}).into_widget()`
        break
      }
      case 'SkeletonText':
        view = `ui::SkeletonText { lines: ${p.lines ?? 3} }.into_widget()`
        break
      case 'RadioOption':
      case 'PillTab':
      case 'ToggleItem':
        // Read as data by the group that holds them.
        view = 'SizedBox::shrink().into_widget()'
        break
      case 'RadioGroup': {
        const key = n.bind?.value
        const options = (n.children ?? []).map(option => {
          const choose = option.props.disabled
            ? ''
            : key
              ? `{ let state = state.clone(); control = control.on_selected(move |app| crate::scene_state::set(app, &state, ${q(key)}, ${q(option.props.value)})); }`
              : 'control = control.on_selected(|_| {});'
          return `{ let mut control = ui::Radio::new(selected == ${q(option.props.value)}).label(${q(option.props.label)}).size(${size}).tint(${tint}); ${choose} control.into_widget() }`
        })
        const gap = p.gap ? spacing.inset(p.gap) : 'v.spacing_2_5'
        view = `{ let selected = ${key ? `app.read(&state.values)[${q(key)}].as_str().unwrap().to_string()` : `${q(p.value ?? '')}.to_string()`}; ${p.direction === 'row' ? row(options, gap) : col(options, gap)} }`
        break
      }
      case 'PillTabs': {
        const items = (n.children ?? [])
          .map(
            item =>
              `{ let mut item = ui::ChoiceItem::new(${q(item.props.value)}.to_string(), ${q(item.props.label)}); item.enabled = ${!item.props.disabled}; item.count = ${item.props.count === undefined ? 'None' : `Some(${item.props.count})`}; item }`
          )
          .join(',')
        const value = n.bind
          ? `app.read(&state.values)[${q(n.bind.value)}].as_str().unwrap().to_string()`
          : `${q(p.value ?? '')}.to_string()`
        const changed = n.bind
          ? `{ let state = state.clone(); move |app, value: String| crate::scene_state::set(app, &state, ${q(n.bind.value)}, value) }`
          : '|_, _: String| {}'
        view = `{ let mut control = ui::PillTabs::new(vec![${items}], Some(${value})).on_changed(${changed}); control.tint = ${tint}; control.into_widget() }`
        break
      }
      case 'ToggleGroup': {
        const items = n.children ?? []
        const single = p.multiple === false
        const controlSize = `ui::ControlSize::${cap(p.size ?? 'small')}`
        const list = items
          .map(item => {
            const glyph = (item.children ?? []).find(
              child => typeof child === 'object' && child.component === 'Icon'
            )
            return `{ let mut item = ui::ToggleItem::new(${q(item.props.value)}, ${q(item.props.label)}); item.enabled = ${!item.props.disabled}; ${glyph ? `item.child = IconTheme::merge(None, IconThemeData::new().size(v.metrics(${controlSize}).content.size), ${node(glyph, item)}).into_widget();` : ''} item }`
          })
          .join(',')
        const pressed = item =>
          item.bind?.pressed
            ? `app.read(&state.values)[${q(item.bind.pressed)}].as_bool().unwrap()`
            : String(!!item.props.pressed)
        const value = single
          ? n.bind
            ? `{ let value = app.read(&state.values)[${q(n.bind.value)}].as_str().unwrap().to_string(); if value.is_empty() { vec![] } else { vec![value] } }`
            : `vec![${p.value ? `${q(p.value)}.to_string()` : ''}]`
          : `[${items.map(item => `(${pressed(item)}, ${q(item.props.value)})`).join(',')}].into_iter().filter(|(on, _)| *on).map(|(_, value)| value.to_string()).collect()`
        const body = single
          ? n.bind
            ? `crate::scene_state::set(app, &state, ${q(n.bind.value)}, next.first().cloned().unwrap_or_default());`
            : ''
          : items
              .filter(item => item.bind?.pressed)
              .map(
                item =>
                  `crate::scene_state::set(app, &state, ${q(item.bind.pressed)}, next.contains(&${q(item.props.value)}.to_string()));`
              )
              .join(' ')
        view = `{ let mut control = ui::ToggleGroup::new(vec![${list}], ${value}); control.multiple = ${!single}; control.size = ${controlSize}; control.variant = ui::ControlVariant::${cap(p.variant ?? 'recessed')}; control.tint = ${tint}; ${body ? `let state = state.clone(); control = control.on_changed(move |app, next: Vec<String>| { ${body} });` : ''} control.into_widget() }`
        break
      }
      case 'OptionCard': {
        const press = handler('press') ?? '|_| {}'
        view = `{ let mut control = ui::OptionCard::new(${q(p.title ?? '')}, ${q(p.description ?? '')}); control.selected = ${derived('selected', false)}; control.tint = ${tint}; control.enabled = ${!p.disabled}; control.title_child = ${n.slots?.title ? `Some(${node(n.slots.title)})` : 'None'}; control.on_pressed = Some(std::rc::Rc::new(${press})); control.into_widget() }`
        break
      }
      case 'Slider': {
        const read = key =>
          `app.read(&state.values)[${q(key)}].as_f64().unwrap()`
        const first = n.bind?.value ? read(n.bind.value) : number(p.value ?? 0)
        const end = n.bind?.end
          ? read(n.bind.end)
          : p.end !== undefined
            ? number(p.end)
            : null
        // React prints the reading: a range as "a – b", a unit after one value.
        const label = !p.showValue
          ? 'None'
          : p.unit
            ? `Some(format!("{}${p.unit}", ${first}))`
            : end
              ? `Some(format!("{} – {}", ${first}, ${end}))`
              : `Some(format!("{}", ${first}))`
        const listen = (method, key) =>
          key
            ? `{ let state = state.clone(); control = control.${method}(move |app, value| crate::scene_state::set(app, &state, ${q(key)}, value)); }`
            : ''
        view = `{ let mut control = ui::Slider::new(${first}, ${number(p.min ?? 0)}, ${number(p.max ?? 100)}).step(${number(p.step ?? 1)}); control.end = ${end ? `Some(${end})` : 'None'}; control.size = ui::ControlSize::${cap(p.size ?? 'medium')}; control.tint = ${tint}; control.enabled = ${!p.disabled}; control.value_label = ${label}; control = control.on_changed(|_, _| {}); ${listen('on_changed', n.bind?.value)} ${listen('on_end_changed', n.bind?.end)} control.into_widget() }`
        break
      }
      case 'NumberField': {
        const key = n.bind?.value
        view = `{ let mut control = ui::NumberField::new(${key ? `app.read(&state.values)[${q(key)}].as_str().unwrap().parse::<f64>().ok()` : `${q(p.value ?? '')}.parse::<f64>().ok()`}).range(${number(p.min ?? -Infinity).replace('-Infinity', 'f64::NEG_INFINITY')}, ${number(p.max ?? Infinity).replace('Infinity', 'f64::INFINITY')}).step(${number(p.step ?? 1)}); control.size = ${size}; control.placeholder = ${q(p.placeholder ?? '')}.to_string(); control.unit = ${p.unit ? `Some(${q(p.unit)}.to_string())` : 'None'}; control.invalid = (${derived('state', 'default')}) == "error"; control.tint = ${tint}; control.enabled = ${!p.disabled}; ${key ? `{ let state = state.clone(); control = control.on_changed(move |app, value| crate::scene_state::set(app, &state, ${q(key)}, value.to_string())); } { let state = state.clone(); control.on_cleared = Some(std::rc::Rc::new(move |app| crate::scene_state::set(app, &state, ${q(key)}, ""))); }` : 'control = control.on_changed(|_, _| {});'} control.into_widget() }`
        break
      }
      case 'SearchField': {
        if (!n.bind)
          throw Error('Inset SearchField requires a shared value binding')
        const key = n.bind.value
        const dismiss = handler('dismiss')
        view = `{ let value = app.read(&state.values)[${q(key)}].as_str().unwrap().to_string(); let controller = state.controller(app, ${q(key)}); let mut control = { let state = state.clone(); ui::SearchField::new(controller).on_changed(move |app, value| crate::scene_state::set(app, &state, ${q(key)}, value)) }; control.field.placeholder = ${q(p.placeholder ?? 'Search')}.to_string(); control.field.size = ${size}; control.field.tint = ${tint}; control.value = Some(value); control.hint = ${n.slots?.hint ? `Some(${node(n.slots.hint)})` : 'None'}; control.on_dismiss = ${dismiss ? `Some(std::rc::Rc::new(${dismiss}))` : 'None'}; control.into_widget() }`
        break
      }
      case 'OtpField': {
        const key = n.bind?.value
        view = `{ let mut control = ui::OtpField::new(${p.length ?? 6}, ${key ? `app.read(&state.values)[${q(key)}].as_str().unwrap().to_string()` : `${q(p.value ?? '')}`}); control.group_after = ${p.groupAfter ? `Some(${p.groupAfter})` : 'None'}; control.mask = ${!!p.mask}; control.size = ${size}; control.invalid = (${derived('state', 'default')}) == "error"; control.tint = ${tint}; control.enabled = ${!p.disabled}; ${key ? `{ let state = state.clone(); control = control.on_changed(move |app, value| crate::scene_state::set(app, &state, ${q(key)}, value)); }` : 'control = control.on_changed(|_, _| {});'} control.into_widget() }`
        break
      }
      case 'SegmentedItem':
        // Read as data by its SegmentedControl.
        view = 'SizedBox::shrink().into_widget()'
        break
      case 'SegmentedControl': {
        const items = (n.children ?? [])
          .map(
            item =>
              `{ let mut item = ui::ChoiceItem::new(${q(item.props.value)}.to_string(), ${q(item.props.label)}); item.enabled = ${!item.props.disabled}; item }`
          )
          .join(',')
        const value = n.bind
          ? `app.read(&state.values)[${q(n.bind.value)}].as_str().unwrap().to_string()`
          : `${q(p.value ?? '')}.to_string()`
        const changed = n.bind
          ? `{ let state = state.clone(); move |app, value: String| crate::scene_state::set(app, &state, ${q(n.bind.value)}, value) }`
          : '|_, _: String| {}'
        view = `{ let mut control = ui::SegmentedControl::new(vec![${items}], Some(${value})).on_changed(${changed}); control.size = ${size}; control.tint = ${tint}; control.active_style = ui::SegmentedActiveStyle::${cap(p.activeStyle ?? 'raised')}; control.stretch = ${!!p.stretch}; control.into_widget() }`
        break
      }
      case 'Checkbox':
      case 'Switch':
        view = `{ ${n.bind ? 'let state = state.clone();' : ''} ui::${n.component}::new(${n.bind ? `app.read(&state.values)[${q(n.bind.checked)}].as_bool().unwrap()` : (p.checked ?? false)}).label(${q(p.label ?? label())}).size(${size}).tint(${tint})${p.disabled ? '' : n.bind ? `.on_changed(move |app, value| crate::scene_state::set(app, &state, ${q(n.bind.checked)}, ${n.component === 'Checkbox' ? 'value.unwrap_or(false)' : 'value'}))` : '.on_changed(|_, _| {})'}.into_widget() }`
        break
      case 'EmptyState':
        view = `{ let mut control = ui::EmptyState::new(${n.derive?.title ? expression(n.derive.title) : q(p.title)}, ""); control.action = ${n.slots?.action ? `Some(${node(n.slots.action)})` : 'None'}; control.into_widget() }`
        break
      case 'Callout':
        view = `{ let mut control = ui::Callout::new(${q(p.title ?? '')}, ${q(label())}); control.tint = ui::Tint::${cap(p.tint ?? 'neutral')}; control.size = ${size}; control.icon = ${n.slots?.icon ? `Some(${node(n.slots.icon)})` : 'None'}; control.actions = vec![${n.slots?.action ? node(n.slots.action) : ''}]; control.into_widget() }`
        break
      case 'Step':
        view = `${step(n)}.into_widget()`
        break
      case 'StepList':
        view = `ui::StepList { steps: vec![], items: ${vecOf(children)} }.into_widget()`
        break
      case 'Dialog': {
        if (!n.bind?.open || p.tone) throw Error('Inset Dialog variant pending')
        const header = (n.children ?? []).find(
          c => c.component === 'DialogHeader'
        )
        const rest = (n.children ?? [])
          .filter(c => c.component !== 'DialogHeader')
          .map(c => node(c, n))
        const dialog = `{ let mut control = ui::Dialog::new(${q(header?.props?.title ?? '')}, ${col(rest, 'v.spacing_4')}); control.description = ${q(header?.props?.subtitle ?? '')}.into(); control.width = ${p.width ? `Some(${number(p.width)})` : 'None'}; control.into_widget() }`
        view = `if app.read(&state.values)[${q(n.bind.open)}].as_bool().unwrap() { Container::new().color(v.dialog_scrim_color).child(Center::new().child(${dialog})).into_widget() } else { SizedBox::shrink().into_widget() }`
        break
      }
      case 'DialogHeader':
        // Dialog reads the header's title and subtitle into its own panel.
        if (parent?.component !== 'Dialog')
          throw Error('Inset DialogHeader must be a direct Dialog child')
        view = 'SizedBox::shrink().into_widget()'
        break
      case 'DialogBody':
        view = col(children, 'v.spacing_2')
        break
      case 'DialogFooter':
        view = `Row::new().main_axis_size(MainAxisSize::Max).cross_axis_alignment(CrossAxisAlignment::Center).spacing(v.spacing_2).children(${vecOf(children)}).into_widget()`
        break
      case 'Box': {
        let content = p.display
          ? vertical
            ? grouped
            : row(children, p.gap ? spacing.inset(p.gap) : '0.0')
          : col(children, '0.0')
        if (p.justify) {
          if (!p.display)
            throw Error('Inset distribution requires flex display')
          content = `Row::new().main_axis_size(MainAxisSize::Max).main_axis_alignment(MainAxisAlignment::SpaceBetween).cross_axis_alignment(CrossAxisAlignment::${vertical ? 'Stretch' : 'Center'}).spacing(${horizontalGap}).children(${vecOf(children)}).into_widget()`
          if (vertical)
            content = `IntrinsicHeight::new().child(${content}).into_widget()`
        }
        if (p.tone)
          content = `DefaultTextStyle::new(DefaultTextStyle::of(app, context).style.color(v.color_content_muted), ${content}).into_widget()`
        view = `Container::new()${p.width ? `.width(${len(p.width)})` : ''}${p.height ? `.height(${len(p.height)})` : ''}${p.padding ? `.padding(EdgeInsetsGeometry::all(v.${p.padding.replace('.', '_')}))` : ''}${p.surface || p.radius ? `.decoration(BoxDecoration::new()${p.surface ? `.color(v.${p.surface === 'color.surface.muted' ? 'color_surface_muted' : 'color_surface_chrome'})` : ''}${p.radius ? `.border_radius(BorderRadiusGeometry::circular(v.${p.radius.replace('.', '_')}))` : ''})` : ''}.child(${content}).into_widget()`
        if (p.maxWidth)
          view = `ConstrainedBox::new(BoxConstraints { max_width: ${len(p.maxWidth)}, ..BoxConstraints::new() }).child(${view}).into_widget()`
        break
      }
    }
    return n.id
      ? `KeyedSubtree::new(${view}).key(state.key(${q(n.id)})).into_widget()`
      : view
  }
  return node(scene.content)
}
/** Rust timer definitions; each tick assigns the typed shared expression. */
export function insetTimers(scene) {
  insetScene(scene)
  const expression = expressionFor
  return `vec![${(scene.timers ?? [])
    .map(timer => {
      const next = expression(timer.value)
      return `crate::scene_state::Timer { every: ${timer.every}, update: std::rc::Rc::new(|app: &mut App, state: &crate::scene_state::SceneHandle| { let next = ${next}; crate::scene_state::set(app, state, ${q(timer.set)}, serde_json::json!(next)); }) }`
    })
    .join(',')}]`
}
export function insetSupports(scene) {
  try {
    insetScene(scene)
    return true
  } catch {
    return false
  }
}
