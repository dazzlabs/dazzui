import { valueType } from './schema/value.mjs'
const q = value =>
  JSON.stringify(value).replace(/\\u([0-9a-f]{4})/gi, '\\u{$1}')
const cap = value => value[0].toUpperCase() + value.slice(1)
const number = value => `${value}${Number.isInteger(value) ? '.0' : ''}`
const properties = {
  Button: 'variant size tint disabled expand shortcut',
  ActionBar: 'align pushEnd',
  Badge: 'variant size tint',
  Card: 'variant size tint',
  Row: 'gap',
  Col: 'gap width',
  Inline: 'tone fontSize mono',
  Text: '',
  Icon: 'name size',
  TextField: 'placeholder size state mono tint readOnly disabled',
  FormField: 'label hint state',
  Value: '',
  PreferenceSection: 'label',
  PreferenceRow: 'title subtitle',
  Box: 'width height maxWidth grow padding surface radius justify display gap align tone',
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
}

export function insetScene(scene) {
  if (
    Object.values(scene.state ?? {}).some(
      value => !['boolean', 'string', 'number'].includes(typeof value)
    ) ||
    scene.timers?.length
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
    if ('eq' in value)
      return `(${expression(value.eq[0])}) == (${expression(value.eq[1])})`
    if ('trim' in value)
      return `(${expression(value.trim)}).trim_matches(|c| crate::generated_stories::TRIM_CODE_POINTS.contains(&(c as u32))).to_string()`
    if ('concat' in value)
      return `vec![${value.concat.map(v => `(${expression(v)}).to_string()`).join(',')}].concat()`
    throw Error('Inset expression pending')
  }
  const row = (children, gap = 'v.spacing_3', stretch = false) =>
    `Row::new().main_axis_size(MainAxisSize::Min).cross_axis_alignment(CrossAxisAlignment::${stretch ? 'Stretch' : 'Center'}).spacing(${gap}).children(vec![${children.join(',')}]).into_widget()`
  const col = (children, gap = 'v.spacing_3') =>
    `Column::new().main_axis_size(MainAxisSize::Min).cross_axis_alignment(CrossAxisAlignment::Start).spacing(${gap}).children(vec![${children.join(',')}]).into_widget()`
  const text = (label, style) =>
    `Text::new(${q(label)}).style(${style}).into_widget()`
  const step = n => {
    if (
      Object.keys(n.props ?? {}).some(
        k => !['label', 'status', 'meta', 'tint'].includes(k)
      ) ||
      n.bind ||
      n.derive ||
      n.on ||
      n.slots
    )
      throw Error('Inset Step property pending')
    return `ui::Step { tint: ui::Tint::${cap(n.props.tint ?? 'primary')}, label: ${q(n.props.label)}.into(), status: ui::StepStatus::${cap(n.props.status)}, meta: ${n.props.meta !== undefined ? `Some(${q(n.props.meta)}.into())` : 'None'} }`
  }
  function node(n, parent) {
    if (typeof n === 'string') return `Text::new(${q(n)}).into_widget()`
    if (!(n.component in properties))
      throw Error(`Inset adapter pending: ${n.component}`)
    const binding =
      n.component === 'TextField'
        ? ['value', 'string']
        : ['Switch', 'Checkbox'].includes(n.component)
          ? ['checked', 'boolean']
          : null
    if (
      n.bind &&
      (!binding ||
        Object.keys(n.bind).some(key => key !== binding[0]) ||
        typeof scene.state?.[n.bind[binding[0]]] !== binding[1])
    )
      throw Error('Inset binding pending')
    const derives = {
      Button: ['disabled'],
      FormField: ['state', 'hint'],
      TextField: ['state'],
    }
    if (
      n.derive &&
      Object.keys(n.derive).some(
        key => !(derives[n.component] ?? []).includes(key)
      )
    )
      throw Error('Inset derived property pending')
    if (
      n.on &&
      (n.component !== 'Button' ||
        Object.keys(n.on).some(key => key !== 'press') ||
        !n.on.press ||
        typeof scene.state?.[n.on.press.set] !== typeof n.on.press.value)
    )
      throw Error('Inset event pending')
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
            (n.component === 'Callout' && k === 'icon')
          )
      )
    )
      throw Error('Inset slot pending')
    const horizontal =
      ['Row', 'ActionBar', 'Specimen'].includes(n.component) ||
      (n.component === 'Box' && !!p.display)
    const children = (n.children ?? []).flatMap(child => {
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
      (['Row', 'ActionBar', 'Specimen'].includes(parent.component) ||
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
      ? `v.${p.gap.replace('.', '_')}`
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
        view = `ui::PreferenceGroup::new(${q(p.label)}, vec![${children.join(',')}]).into_widget()`
        break
      case 'PreferenceRow':
        view = `{ let mut control = ui::PreferenceRow::new(${q(p.title)}, ${row(children, '0.0')}); control.description = ${q(p.subtitle ?? '')}.into(); control.into_widget() }`
        break
      case 'Table':
      case 'TableHead':
      case 'TableRow':
        view = `ui::${n.component}::new(vec![${children.join(',')}]).into_widget()`
        break
      case 'TableCell':
        view = `ui::TableCell::new(${row(children, '0.0')}).head(${!!p.head}).end(${p.align === 'end'}).into_widget()`
        break
      case 'ActionBar':
        view = `ui::ActionBar::new(vec![${children.join(',')}]).align(MainAxisAlignment::${{ start: 'Start', end: 'End', between: 'SpaceBetween' }[p.align ?? 'start']}).stretch(${vertical}).into_widget()`
        if (vertical)
          view = `IntrinsicHeight::new().child(${view}).into_widget()`
        break
      case 'Icon':
        if (!['ArrowDownload20Regular', 'Circle20Filled'].includes(p.name))
          throw Error('Inset glyph pending')
        view = `ui::FluentIcon(ui::FluentGlyph::${p.name}).into_widget()`
        if (p.size)
          view = `IconTheme::merge(None, IconThemeData::new().size(${number(p.size)}), ${view}).into_widget()`
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
          : row(
              children,
              p.gap ? `v.${p.gap.replace('.', '_')}` : 'v.spacing_3'
            )
        break
      case 'Col':
        view = `SizedBox::new()${p.width ? `.width(${number(p.width)})` : ''}.child(${col(children, p.gap ? `v.${p.gap.replace('.', '_')}` : 'v.spacing_3')}).into_widget()`
        break
      case 'Inline': {
        const style = `TextStyle::new()${p.tone ? '.color(v.color_content_muted)' : ''}${p.fontSize ? `.font_size(v.${p.fontSize.replace('.', '_')}.size)` : ''}${p.mono ? '.font_family("monospace")' : ''}`
        view = `DefaultTextStyle::new(DefaultTextStyle::of(app, context).style.merge(Some(&${style})), ${row(children, '0.0')}).into_widget()`
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
        view = `crate::storybook::stage::grid(${number(p.min ?? 256)}, vec![${children.join(',')}])`
        break
      case 'Spinner':
        view = `ui::Spinner { size: ${size}, tint: ${tint}, on_accent: ${!!p.onAccent} }.into_widget()`
        break
      case 'Progress':
        view = `ui::Progress { gradient: ${!!p.gradient}, value: ${p.value === undefined ? 'None' : `Some(${number(p.value / 100)})`}, size: ui::ControlSize::${cap(p.size ?? 'small')}, tint: ${tint} }.into_widget()`
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
      case 'Checkbox':
      case 'Switch':
        view = `{ ${n.bind ? 'let state = state.clone();' : ''} ui::${n.component}::new(${n.bind ? `app.read(&state.values)[${q(n.bind.checked)}].as_bool().unwrap()` : (p.checked ?? false)}).label(${q(p.label ?? label())}).size(${size}).tint(${tint})${p.disabled ? '' : n.bind ? `.on_changed(move |app, value| crate::scene_state::set(app, &state, ${q(n.bind.checked)}, ${n.component === 'Checkbox' ? 'value.unwrap_or(false)' : 'value'}))` : '.on_changed(|_, _| {})'}.into_widget() }`
        break
      case 'EmptyState':
        view = `{ let mut control = ui::EmptyState::new(${q(p.title)}, ""); control.action = ${n.slots?.action ? `Some(${node(n.slots.action)})` : 'None'}; control.into_widget() }`
        break
      case 'Callout':
        view = `{ let mut control = ui::Callout::new(${q(p.title ?? '')}, ${q(label())}); control.tint = ui::Tint::${cap(p.tint ?? 'neutral')}; control.size = ${size}; control.icon = ${n.slots?.icon ? `Some(${node(n.slots.icon)})` : 'None'}; control.actions = vec![${n.slots?.action ? node(n.slots.action) : ''}]; control.into_widget() }`
        break
      case 'Step':
        view = `${step(n)}.into_widget()`
        break
      case 'StepList':
        view = `ui::StepList::new(vec![${(n.children ?? []).map(step).join(',')}]).into_widget()`
        break
      case 'Box': {
        let content = p.display
          ? vertical
            ? grouped
            : row(children, p.gap ? `v.${p.gap.replace('.', '_')}` : '0.0')
          : col(children, '0.0')
        if (p.justify) {
          if (!p.display)
            throw Error('Inset distribution requires flex display')
          content = `Row::new().main_axis_size(MainAxisSize::Max).main_axis_alignment(MainAxisAlignment::SpaceBetween).cross_axis_alignment(CrossAxisAlignment::${vertical ? 'Stretch' : 'Center'}).spacing(${horizontalGap}).children(vec![${children.join(',')}]).into_widget()`
          if (vertical)
            content = `IntrinsicHeight::new().child(${content}).into_widget()`
        }
        if (p.tone)
          content = `DefaultTextStyle::new(DefaultTextStyle::of(app, context).style.color(v.color_content_muted), ${content}).into_widget()`
        view = `Container::new()${p.width ? `.width(${number(p.width)})` : ''}${p.height ? `.height(${number(p.height)})` : ''}${p.padding ? `.padding(EdgeInsetsGeometry::all(v.${p.padding.replace('.', '_')}))` : ''}${p.surface || p.radius ? `.decoration(BoxDecoration::new()${p.surface ? `.color(v.${p.surface === 'color.surface.muted' ? 'color_surface_muted' : 'color_surface_chrome'})` : ''}${p.radius ? `.border_radius(BorderRadiusGeometry::circular(v.${p.radius.replace('.', '_')}))` : ''})` : ''}.child(${content}).into_widget()`
        if (p.maxWidth)
          view = `ConstrainedBox::new(BoxConstraints { max_width: ${number(p.maxWidth)}, ..BoxConstraints::new() }).child(${view}).into_widget()`
        break
      }
    }
    return n.id
      ? `KeyedSubtree::new(${view}).key(state.key(${q(n.id)})).into_widget()`
      : view
  }
  return node(scene.content)
}
export function insetSupports(scene) {
  try {
    insetScene(scene)
    return true
  } catch {
    return false
  }
}
