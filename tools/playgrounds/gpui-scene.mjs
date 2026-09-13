import { TRIM_CHARACTERS } from './schema/value.mjs'
import { spacing } from './tokens.mjs'

/** A declared length: logical pixels or a spacing step. */
const len = value =>
  typeof value === 'number' ? `gpui::px(${number(value)})` : spacing.gpui(value)
const q = JSON.stringify
const number = n => (Number.isInteger(n) ? `${n}.0` : String(n))
const cap = s => s[0].toUpperCase() + s.slice(1)
const supported = new Set([
  'Box',
  'Badge',
  'Card',
  'Callout',
  'EmptyState',
  'Progress',
  'StepList',
  'Step',
  'Divider',
  'SectionLabel',
  'Table',
  'TableHead',
  'TableRow',
  'TableCell',
  'Button',
  'Row',
  'Col',
  'Page',
  'Section',
  'Stack',
  'Grid',
  'Specimen',
  'Inline',
  'Icon',
  'Spinner',
  'KeyCap',
  'ActionBar',
  'Switch',
  'Checkbox',
  'PreferenceSection',
  'PreferenceRow',
  'Value',
  'Text',
  'TextField',
  'FormField',
  'Dialog',
  'DialogHeader',
  'DialogBody',
  'DialogFooter',
  'SegmentedControl',
  'SegmentedItem',
  'IconButton',
  'Toggle',
  'Show',
  'Tag',
  'Link',
  'Meter',
  'Skeleton',
  'SkeletonText',
  'RadioGroup',
  'RadioOption',
  'PillTabs',
  'PillTab',
  'ToggleGroup',
  'ToggleItem',
  'OptionCard',
  'Slider',
  'NumberField',
  'SearchField',
  'OtpField',
])
const inputs = ['TextField', 'NumberField', 'SearchField', 'OtpField']
export function gpuiSupports(scene) {
  const visit = n =>
    typeof n === 'string' ||
    (supported.has(n.component) &&
      (!n.bind ||
        [
          'Switch',
          'Checkbox',
          'TextField',
          'Dialog',
          'SegmentedControl',
          'Toggle',
          'Tag',
          'RadioGroup',
          'PillTabs',
          'ToggleGroup',
          'ToggleItem',
          'Slider',
          'NumberField',
          'SearchField',
          'OtpField',
        ].includes(n.component)) &&
      (!n.bind?.checked ||
        typeof scene.state?.[n.bind.checked] === 'boolean') &&
      (!n.bind?.open || typeof scene.state?.[n.bind.open] === 'boolean') &&
      (!inputs.includes(n.component) ||
        typeof scene.state?.[n.bind?.value] === 'string') &&
      !(
        n.component === 'Checkbox' &&
        (n.children ?? []).some(c => typeof c !== 'string')
      ) &&
      (!n.on ||
        [
          'Button',
          'IconButton',
          'Link',
          'Tag',
          'OptionCard',
          'SearchField',
        ].includes(n.component)) &&
      !(n.component === 'Tag' && n.slots?.icon) &&
      !(n.component === 'ActionBar' && n.props?.align === 'center') &&
      Object.values(n.slots ?? {}).every(visit) &&
      (n.children ?? []).every(visit))
  return visit(scene.content)
}
export function gpuiValue(value) {
  if (typeof value === 'string') return `Value::Text(${q(value)}.into())`
  if (typeof value === 'boolean') return `Value::Boolean(${value})`
  return `Value::from(${number(value)})`
}
function value(expression, branch = false) {
  if (expression === null || typeof expression !== 'object')
    return gpuiValue(expression)
  if ('multiply' in expression)
    return `Value::Number(${value(expression.multiply[0])}.number() * ${value(expression.multiply[1])}.number())`
  if ('add' in expression)
    return `Value::Number(${value(expression.add[0])}.number() + ${value(expression.add[1])}.number())`
  if ('gte' in expression)
    return `Value::Boolean(${value(expression.gte[0])}.number() >= ${value(expression.gte[1])}.number())`
  if ('trim' in expression)
    return `Value::Text(${value(expression.trim)}.text().trim_matches(|c: char| [${[...TRIM_CHARACTERS].map(c => c.charCodeAt(0) + 'u32').join(',')}].contains(&(c as u32))).to_owned())`
  if ('and' in expression || 'or' in expression) {
    const [op, operands] =
      'and' in expression ? ['&&', expression.and] : ['||', expression.or]
    return `Value::Boolean(${operands.map(v => `${value(v)}.boolean()`).join(` ${op} `)})`
  }
  if ('lower' in expression)
    return `Value::Text(${value(expression.lower)}.text().to_lowercase())`
  if ('length' in expression)
    return `Value::from(${value(expression.length)}.text().chars().count() as f64)`
  if ('contains' in expression)
    return `Value::Boolean(${value(expression.contains[0])}.text().contains(${value(expression.contains[1])}.text()))`
  if ('text' in expression)
    return `Value::Text(${value(expression.text)}.to_string())`
  if ('not' in expression)
    return `Value::Boolean(!${value(expression.not)}.boolean())`
  if ('eq' in expression)
    return `Value::Boolean(${value(expression.eq[0])} == ${value(expression.eq[1])})`
  if ('concat' in expression)
    return `Value::Text([${expression.concat.map(v => `${value(v)}.text().to_owned()`).join(',')}].concat())`
  if ('state' in expression)
    return `state.generated.values[${q(expression.state)}].clone()`
  const conditional = `if ${value(expression.if)}.boolean() { ${value(expression.then, true)} } else { ${value(expression.else, true)} }`
  return branch ? conditional : `(${conditional})`
}
export function gpuiDefinition(scene, key) {
  const fields = []
  function visit(node, path) {
    if (typeof node === 'string') return
    if (inputs.includes(node.component))
      fields.push(
        `crate::scene::Field { key: ${q(path)}, binding: ${q(node.bind.value)}, placeholder: ${q(node.props?.placeholder ?? '')}, read_only: ${!!node.props?.readOnly} }`
      )
    for (const [name, slot] of Object.entries(node.slots ?? {}))
      visit(slot, `${path}-slot-${name}`)
    ;(node.children ?? []).forEach((child, i) => visit(child, `${path}-${i}`))
  }
  visit(scene.content, key)
  return `crate::scene::Definition { values: std::collections::BTreeMap::from([${Object.entries(
    scene.state ?? {}
  )
    .map(([k, v]) => `(${q(k)}, ${gpuiValue(v)})`)
    .join(
      ','
    )}]), fields: vec![${fields.join(',')}], timers: vec![${(scene.timers ?? []).map(timer => `crate::scene::Timer { every: ${timer.every}, update: |state, cx| { let next = ${value(timer.value, true)}; state.generated.assign(${q(timer.set)}, next, cx); } }`).join(',')}] }`
}
/** An Icon node as the builder a control embeds, rather than an element. */
function gpuiIcon(node, fitted) {
  const p = node.props
  return `dazzui::Icon::new(crate::generated_glyphs::path(${q(p.name)})).size(${fitted ?? (typeof p.size === 'string' ? `state.theme.variables().spacing_${p.size.split('.')[1]}` : `gpui::px(${number(p.size ?? 20)})`)})`
}
export function gpuiScene(node, key) {
  const element = renderNode(node, key)
  if (typeof node === 'string' || !node.id) return element
  return `gpui::div().flex()${node.component === 'TextField' ? '.flex_1().min_w_0()' : ''}.debug_selector(|| ${q(node.id)}.into()).child(${element}).into_any_element()`
}
function renderNode(node, key) {
  if (typeof node === 'string')
    return `gpui::div().child(${q(node)}).into_any_element()`
  const p = node.props ?? {},
    children = (node.children ?? []).map((n, i) => gpuiScene(n, `${key}-${i}`))
  // A `Show` child contributes its children only while its condition holds,
  // so a hidden branch takes no slot in the flex it sits in.
  const attach = (node.children ?? [])
    .map((child, i) =>
      typeof child === 'object' && child.component === 'Show'
        ? `.children(if ${value(child.value)}.boolean() { vec![${(child.children ?? []).map((c, j) => gpuiScene(c, `${key}-${i}-${j}`)).join(',')}] } else { vec![] })`
        : `.child(${children[i]})`
    )
    .join('')
  const action = event => {
    const declared = node.on?.[event]
    if (!declared) return null
    return declared.set === undefined
      ? '{}'
      : `state.generated.assign(${q(declared.set)}, ${gpuiValue(declared.value)}, cx);`
  }
  const listener = (method, event, signature = '_, app') => {
    const body = action(event)
    if (body === null) return ''
    return body === '{}'
      ? `.${method}(|${signature}| {})`
      : `.${method}({ let entity = cx.entity(); move |${signature}| { entity.update(app, |state, cx| { ${body} }); } })`
  }
  const fieldState = node.derive?.state
    ? `if ${value(node.derive.state)}.text() == "error" { dazzui::TextFieldState::Error } else { dazzui::TextFieldState::Default }`
    : `dazzui::TextFieldState::${cap(p.state ?? 'default')}`
  const disabled = node.derive?.disabled
    ? `${value(node.derive.disabled)}.boolean()`
    : String(!!p.disabled)
  switch (node.component) {
    case 'Grid':
      return `crate::storybook::gallery::responsive_tiles(gpui::px(${number(p.min ?? 256)}), vec![${children.join(',')}], state.generated.grid_columns(${q(key)}), state.theme.variables().spacing_4).into_any_element()`
    case 'Table':
    case 'TableHead':
      return `dazzui::${node.component}::new()${attach}.into_any_element()`
    case 'TableRow':
      return `dazzui::TableRow::new(${q(node.id ?? key)})${attach}.into_any_element()`
    case 'TableCell':
      return `dazzui::TableCell::new().head(${!!p.head}).align(dazzui::TableAlign::${cap(p.align ?? 'start')})${attach}.into_any_element()`
    case 'Badge':
      return `dazzui::Badge::new("").size(dazzui::BadgeSize::${cap(p.size ?? 'medium')}).variant(dazzui::BadgeVariant::${cap(p.variant ?? 'tinted')}).tint(Tint::${cap(p.tint ?? 'primary')}).content(vec![${children.join(',')}]).into_any_element()`
    case 'Show':
      return `gpui::div().children(if ${value(node.value)}.boolean() { vec![${children.join(',')}] } else { vec![] }).into_any_element()`
    case 'Tag': {
      const selectable = node.bind?.selected || node.on?.select
      const selected = node.bind?.selected
        ? `state.generated.values[${q(node.bind.selected)}].boolean()`
        : String(!!p.selected)
      const select = node.bind?.selected
        ? `.on_select({ let entity = cx.entity(); move |_, app| { entity.update(app, |state, cx| { let next = !state.generated.values[${q(node.bind.selected)}].boolean(); state.generated.assign(${q(node.bind.selected)}, Value::Boolean(next), cx); ${action('select') === '{}' ? '' : (action('select') ?? '')} }); } })`
        : listener('on_select', 'select')
      return `dazzui::Tag::new(${q(node.id ?? key)}, ${q((node.children ?? []).filter(c => typeof c === 'string').join(''))}).size(dazzui::BadgeSize::${cap(p.size ?? 'medium')}).variant(dazzui::TagVariant::${cap(p.variant ?? 'tinted')}).tint(Tint::${cap(p.tint ?? 'primary')})${selectable ? `.selected(${selected})` : ''}.disabled(${!!p.disabled})${select}${listener('on_remove', 'remove')}.into_any_element()`
    }
    case 'Link':
      return `dazzui::Link::new(${q(node.id ?? key)}, ${q((node.children ?? []).join(''))}).tint(Tint::${cap(p.tint ?? 'primary')}).underline(dazzui::LinkUnderline::${cap(p.underline ?? 'hover')}).external(${!!p.external}).disabled(${!!p.disabled})${listener('on_click', 'press', '_, _, app') || '.on_click(|_, _, _| {})'}.into_any_element()`
    case 'Meter': {
      const min = p.min ?? 0,
        max = p.max ?? 100
      return `dazzui::Meter::new(${number(p.value)}).range(${number(min)}, ${number(max)}).size(dazzui::MeterSize::${cap(p.size ?? 'medium')}).tint(Tint::${cap(p.tint ?? 'primary')})${p.label ? `.label(${q(p.label)})` : ''}${p.showValue ? `.value_label(${q(`${Math.round(((p.value - min) / (max - min)) * 100)}%`)})` : ''}.into_any_element()`
    }
    case 'Skeleton': {
      const percent = typeof p.width === 'string' && p.width.endsWith('%')
      const skeleton = `dazzui::Skeleton::new().shape(dazzui::SkeletonShape::${cap(p.shape ?? 'text')})${p.width !== undefined && !percent ? `.width(${len(p.width)})` : ''}${p.height !== undefined ? `.height(${len(p.height)})` : ''}`
      return percent
        ? `gpui::div().w(gpui::relative(${parseFloat(p.width) / 100})).child(${skeleton}).into_any_element()`
        : `${skeleton}.into_any_element()`
    }
    case 'SkeletonText':
      return `dazzui::SkeletonText::new().lines(${p.lines ?? 3}).into_any_element()`
    case 'Box':
      return `gpui::div()${p.display ? '.flex()' : ''}${p.wrap ? '.flex_wrap()' : ''}${p.align ? '.items_center()' : ''}${p.justify ? '.justify_between()' : ''}${p.tone ? '.text_color(state.theme.variables().color_content_muted)' : ''}${p.gap ? `.gap(${spacing.gpui(p.gap)})` : ''}${p.width ? `.w(${len(p.width)})` : ''}${p.height ? `.h(${len(p.height)})` : ''}${p.maxWidth ? `.max_w(${len(p.maxWidth)})` : ''}${p.grow ? '.flex_1().min_w_0()' : ''}${p.padding ? `.p(state.theme.variables().${p.padding === 'spacing.3' ? 'spacing_3' : 'spacing_2'})` : ''}${p.surface ? `.bg(state.theme.variables().${p.surface === 'color.surface.muted' ? 'color_surface_muted' : 'color_surface_chrome'})` : ''}${p.radius ? `.rounded(state.theme.variables().${p.radius === 'radius.large' ? 'radius_large' : 'radius_medium'})` : ''}${attach}.into_any_element()`
    case 'FormField':
      return `dazzui::FormField::new(${q(p.label)})${node.derive?.hint ? `.hint(${value(node.derive.hint)}.text().to_owned())` : p.hint ? `.hint(${q(p.hint)})` : ''}.state(${fieldState})${attach}.into_any_element()`
    case 'TextField':
      return `dazzui::TextField::new(&state.generated.fields[${q(key)}]).size(dazzui::TextFieldSize::${cap(p.size ?? 'medium')}).state(${fieldState}).tint(Tint::${cap(p.tint ?? 'primary')}).mono(${!!p.mono}).disabled(${!!p.disabled}).into_any_element()`
    case 'Dialog': {
      const dialog = `dazzui::DialogScrim::new().child(dazzui::Dialog::new()${p.width ? `.width(gpui::px(${number(p.width)}))` : ''}.tone(dazzui::DialogTone::${cap(p.tone ?? 'default')})${attach}).into_any_element()`
      return node.bind?.open
        ? `if state.generated.values[${q(node.bind.open)}].boolean() { ${dialog} } else { gpui::div().into_any_element() }`
        : dialog
    }
    case 'DialogHeader':
      return `dazzui::DialogHeader::new(${q(p.title)})${p.subtitle ? `.subtitle(${q(p.subtitle)})` : ''}.into_any_element()`
    case 'DialogBody':
    case 'DialogFooter':
      return `dazzui::${node.component}::new()${attach}.into_any_element()`
    case 'Text':
      return `gpui::div()${attach}.into_any_element()`
    case 'Value':
      return `gpui::div().child(format!("{}", ${value(node.value)})).into_any_element()`
    case 'RadioOption':
    case 'PillTab':
    case 'ToggleItem':
      // Read as data by the group that holds them.
      return 'gpui::Empty.into_any_element()'
    case 'RadioGroup':
    case 'PillTabs': {
      const radio = node.component === 'RadioGroup'
      const items = (node.children ?? [])
        .map(item =>
          radio
            ? `.${item.props.disabled ? 'disabled_option' : 'option'}(${q(item.props.value)}, ${q(item.props.label)})`
            : `.entry(${q(item.props.value)}, ${q(item.props.label)}, ${item.props.count === undefined ? 'None' : `Some(${item.props.count})`}, ${!!item.props.disabled})`
        )
        .join('')
      const selected = node.bind?.value
        ? `state.generated.values[${q(node.bind.value)}].text().to_owned()`
        : q(p.value ?? '')
      const select = node.bind?.value
        ? `.on_select({ let entity = cx.entity(); move |value, _, app| { let value = value.to_string(); entity.update(app, |state, cx| state.generated.assign(${q(node.bind.value)}, Value::Text(value), cx)); } })`
        : ''
      return `dazzui::${node.component}::new(${q(node.id ?? key)}, ${selected})${items}${radio ? `.size(dazzui::RadioSize::${cap(p.size ?? 'medium')})` : ''}.tint(Tint::${cap(p.tint ?? 'primary')})${radio ? `.horizontal(${p.direction === 'row'})${p.gap ? `.gap(${spacing.gpui(p.gap)})` : ''}` : ''}${select}.into_any_element()`
    }
    case 'ToggleGroup': {
      const items = node.children ?? []
      const single = p.multiple === false
      const list = items
        .map(item => {
          const glyph = (item.children ?? []).find(
            child => typeof child === 'object' && child.component === 'Icon'
          )
          return `dazzui::ToggleItem::new(${q(item.props.value)}, ${q(item.props.label)}).disabled(${!!item.props.disabled})${glyph ? `.icon(${gpuiIcon(glyph)})` : ''}`
        })
        .join(',')
      const pressed = item =>
        item.bind?.pressed
          ? `state.generated.values[${q(item.bind.pressed)}].boolean()`
          : String(!!item.props.pressed)
      const value = single
        ? node.bind?.value
          ? `{ let value = state.generated.values[${q(node.bind.value)}].text(); if value.is_empty() { vec![] } else { vec![gpui::SharedString::from(value.to_owned())] } }`
          : `vec![${p.value ? `${q(p.value)}.into()` : ''}]`
        : `[${items.map(item => `(${pressed(item)}, ${q(item.props.value)})`).join(',')}].into_iter().filter(|(on, _)| *on).map(|(_, value)| gpui::SharedString::from(value)).collect()`
      const body = single
        ? node.bind?.value
          ? `let next = if state.generated.values[${q(node.bind.value)}].text() == value { String::new() } else { value.clone() }; state.generated.assign(${q(node.bind.value)}, Value::Text(next), cx);`
          : ''
        : items
            .filter(item => item.bind?.pressed)
            .map(
              item =>
                `if value == ${q(item.props.value)} { let next = !state.generated.values[${q(item.bind.pressed)}].boolean(); state.generated.assign(${q(item.bind.pressed)}, Value::Boolean(next), cx); }`
            )
            .join(' ')
      const toggle = body
        ? `.on_toggle({ let entity = cx.entity(); move |value, _, app| { let value = value.to_string(); entity.update(app, |state, cx| { ${body} }); } })`
        : ''
      return `dazzui::ToggleGroup::new(${q(node.id ?? key)}, vec![${list}]).value(${value}).size(ControlSize::${cap(p.size ?? 'small')}).variant(ControlVariant::${cap(p.variant ?? 'recessed')}).tint(Tint::${cap(p.tint ?? 'primary')})${toggle}.into_any_element()`
    }
    case 'OptionCard': {
      const selected = node.derive?.selected
        ? `${value(node.derive.selected)}.boolean()`
        : String(!!p.selected)
      return `dazzui::OptionCard::new(${q(node.id ?? key)}, ${q(p.title ?? '')})${p.description ? `.description(${q(p.description)})` : ''}${node.slots?.title !== undefined ? `.title_content(${gpuiScene(node.slots.title, `${key}-slot-title`)})` : ''}.selected(${selected}).tint(Tint::${cap(p.tint ?? 'primary')}).disabled(${!!p.disabled})${listener('on_select', 'press')}.into_any_element()`
    }
    case 'Slider': {
      const keys = [node.bind?.value, node.bind?.end].filter(Boolean)
      const readings = keys.length
        ? keys.map(k => `state.generated.values[${q(k)}].number()`)
        : [p.value ?? 0, ...(p.end !== undefined ? [p.end] : [])].map(number)
      // React prints the reading: a range as "a – b", a unit after one value.
      const label = !p.showValue
        ? ''
        : p.unit
          ? `.value_label(format!("{}${p.unit}", ${readings[0]}))`
          : `.value_label(format!("${readings.map(() => '{}').join(' – ')}", ${readings.join(', ')}))`
      const change = keys.length
        ? `.on_change(cx.listener(|state, values: &Vec<f32>, _, cx| { ${keys.map((k, i) => `if let Some(value) = values.get(${i}) { state.generated.assign(${q(k)}, Value::from((*value as f64 * 1e4).round() / 1e4), cx); }`).join(' ')} }))`
        : ''
      return `dazzui::Slider::range(${q(node.id ?? key)}, vec![${readings.map(r => `${r} as f32`).join(', ')}]).bounds(${number(p.min ?? 0)}, ${number(p.max ?? 100)}).step(${number(p.step ?? 1)}).size(dazzui::SliderSize::${cap(p.size ?? 'medium')}).tint(Tint::${cap(p.tint ?? 'primary')})${label}.disabled(${!!p.disabled})${change}.into_any_element()`
    }
    case 'NumberField':
      return `dazzui::NumberField::new(${q(node.id ?? key)}, &state.generated.fields[${q(key)}])${p.min !== undefined && p.max !== undefined ? `.bounds(${number(p.min)}, ${number(p.max)})` : ''}.step(${number(p.step ?? 1)}).size(dazzui::TextFieldSize::${cap(p.size ?? 'medium')}).state(${fieldState}).tint(Tint::${cap(p.tint ?? 'primary')})${p.unit ? `.unit(${q(p.unit)})` : ''}.disabled(${!!p.disabled}).on_change(cx.listener(|state, value: &f32, _, cx| { state.generated.assign(${q(node.bind.value)}, Value::Text((*value as f64).to_string()), cx); })).into_any_element()`
    case 'SearchField':
      return `dazzui::SearchField::new(${q(node.id ?? key)}, &state.generated.fields[${q(key)}]).size(dazzui::TextFieldSize::${cap(p.size ?? 'medium')}).tint(Tint::${cap(p.tint ?? 'primary')})${node.slots?.hint !== undefined ? `.hint(${gpuiScene(node.slots.hint, `${key}-slot-hint`)})` : ''}.clear_label(${q(p.clearLabel ?? 'Clear search')})${listener('on_dismiss', 'dismiss')}.into_any_element()`
    case 'OtpField':
      return `dazzui::OtpField::new(${q(node.id ?? key)}, &state.generated.fields[${q(key)}], ${p.length ?? 6})${p.groupAfter ? `.group_after(${p.groupAfter})` : ''}.mask(${!!p.mask}).size(dazzui::TextFieldSize::${cap(p.size ?? 'medium')}).state(${fieldState}).tint(Tint::${cap(p.tint ?? 'primary')}).disabled(${!!p.disabled}).into_any_element()`
    case 'SegmentedItem':
      // Read as data by its SegmentedControl; never rendered on its own.
      return 'gpui::Empty.into_any_element()'
    case 'SegmentedControl': {
      const items = (node.children ?? [])
        .map(
          item =>
            `.${item.props.disabled ? 'disabled_item' : 'item'}(${q(item.props.value)}, ${q(item.props.label)})`
        )
        .join('')
      const selected = node.bind?.value
        ? `state.generated.values[${q(node.bind.value)}].text().to_owned()`
        : q(p.value ?? '')
      const select = node.bind?.value
        ? `.on_select({ let entity = cx.entity(); move |value, _, app| { let value = value.to_string(); entity.update(app, |state, cx| state.generated.assign(${q(node.bind.value)}, Value::Text(value), cx)); } })`
        : ''
      return `dazzui::SegmentedControl::new(${q(node.id ?? key)}, ${selected})${items}.size(dazzui::SegmentedControlSize::${cap(p.size ?? 'medium')}).tint(Tint::${cap(p.tint ?? 'primary')}).active_style(dazzui::SegmentedActiveStyle::${cap(p.activeStyle ?? 'raised')}).stretch(${!!p.stretch})${select}.into_any_element()`
    }
    case 'Switch':
    case 'Checkbox': {
      const checked = node.bind?.checked
        ? `state.generated.values[${q(node.bind.checked)}].boolean()`
        : String(!!p.checked)
      const change = node.bind?.checked
        ? `.on_change(cx.listener(|state, checked: &bool, _, cx| { state.generated.assign(${q(node.bind.checked)}, Value::Boolean(*checked), cx); }))`
        : ''
      return `dazzui::${node.component}::new(${q(node.id ?? key)}, ${node.component === 'Checkbox' ? `${q((node.children ?? []).join(''))}, ` : ''}${checked}).size(dazzui::${node.component}Size::${cap(p.size ?? 'medium')}).tint(Tint::${cap(p.tint ?? 'primary')}).disabled(${!!p.disabled})${change}.into_any_element()`
    }
    case 'PreferenceSection':
      return `dazzui::PreferenceSection::new()${p.label ? `.label(${q(p.label)})` : ''}${attach}.into_any_element()`
    case 'PreferenceRow':
      return `dazzui::PreferenceRow::new(${q(node.id ?? key)}, ${q(p.title)})${p.subtitle ? `.subtitle(${q(p.subtitle)})` : ''}${attach}.into_any_element()`
    case 'Button':
      return `Button::new(${q(key)}, "").size(ControlSize::${cap(p.size ?? 'small')}).variant(ControlVariant::${cap(p.variant ?? 'recessed')}).tint(Tint::${cap(p.tint ?? 'primary')}).disabled(${node.id ? `state.generated.record_disabled(${q(node.id)}, ${disabled})` : disabled}).full_width(${!!p.expand})${p.shortcut ? `.shortcut(${q(p.shortcut)})` : ''}.content(vec![${children.join(',')}])${node.on?.press ? `.on_click(cx.listener(|state, _, _, cx| { state.generated.assign(${q(node.on.press.set)}, ${gpuiValue(node.on.press.value)}, cx); }))` : ''}.into_any_element()`
    case 'IconButton': {
      const icon = gpuiIcon(node.children[0])
      return `dazzui::IconButton::new(${q(node.id ?? key)}, ${q(p.label)}, ${icon}).size(ControlSize::${cap(p.size ?? 'tiny')}).variant(ControlVariant::${cap(p.variant ?? 'plain')}).tint(Tint::${cap(p.tint ?? 'neutral')}).disabled(${!!p.disabled})${node.on?.press ? `.on_click(cx.listener(|state, _, _, cx| { state.generated.assign(${q(node.on.press.set)}, ${gpuiValue(node.on.press.value)}, cx); }))` : ''}.into_any_element()`
    }
    case 'Toggle': {
      const glyph = node.children?.find(child => typeof child === 'object')
      const label = (node.children ?? [])
        .filter(child => typeof child === 'string')
        .join('')
      const pressed = node.bind?.pressed
        ? `state.generated.values[${q(node.bind.pressed)}].boolean()`
        : String(!!p.pressed)
      const toggle = node.bind?.pressed
        ? `.on_toggle({ let entity = cx.entity(); move |_, app| { entity.update(app, |state, cx| { let next = !state.generated.values[${q(node.bind.pressed)}].boolean(); state.generated.assign(${q(node.bind.pressed)}, Value::Boolean(next), cx); }); } })`
        : ''
      return `dazzui::Toggle::new(${q(node.id ?? key)}, ${q(label)})${glyph ? `.icon(${gpuiIcon(glyph, glyph.props.fit ? `state.theme.variables().metrics(ControlSize::${cap(p.size ?? 'tiny')}).content.size` : undefined)})` : ''}.size(ControlSize::${cap(p.size ?? 'tiny')}).variant(ControlVariant::${cap(p.variant ?? 'plain')}).tint(Tint::${cap(p.tint ?? 'neutral')}).pressed(${pressed}).disabled(${!!p.disabled})${toggle}.into_any_element()`
    }
    case 'Row':
      return `crate::storybook::stage::Row::new()${p.gap ? `.gap(${spacing.gpui(p.gap)})` : ''}${attach}.into_any_element()`
    case 'Col':
      return `crate::storybook::stage::Col::new()${p.gap ? `.gap(${spacing.gpui(p.gap)})` : ''}${p.width ? `.width(${len(p.width)})` : ''}${attach}.into_any_element()`
    case 'Page':
    case 'Section':
      return `crate::storybook::gallery::${node.component === 'Page' ? 'DocPage' : 'Section'}::new(${q(p.title)})${p.description ? `.description(${q(p.description)})` : ''}${attach}.into_any_element()`
    case 'Stack':
      return `crate::storybook::gallery::Specimens::new()${attach}.into_any_element()`
    case 'Specimen':
      return `crate::storybook::gallery::Specimen::new(${q(p.label)})${attach}.into_any_element()`
    case 'Inline':
      return `gpui::div().flex().items_center()${p.tone ? `.text_color(state.theme.variables().color_content_${p.tone})` : ''}${p.mono ? '.font_family("ui-monospace")' : ''}${p.fontSize ? `.text_size(state.theme.variables().${p.fontSize.replace('.', '_')}.size)` : ''}${attach}.into_any_element()`
    case 'Icon':
      return `dazzui::Icon::new(crate::generated_glyphs::path(${q(p.name)})).size(${typeof p.size === 'string' ? `state.theme.variables().spacing_${p.size.split('.')[1]}` : `gpui::px(${number(p.size ?? 20)})`}).into_any_element()`
    case 'StepList':
      return `dazzui::StepList::new()${attach}.into_any_element()`
    case 'Step': {
      const status = node.derive?.status
        ? `match ${value(node.derive.status)}.text() { "done" => dazzui::StepStatus::Done, "active" => dazzui::StepStatus::Active, _ => dazzui::StepStatus::Idle }`
        : `dazzui::StepStatus::${cap(p.status)}`
      const meta = node.derive?.meta
        ? `${value(node.derive.meta)}.text().to_owned()`
        : q(p.meta ?? '')
      return `{ let step = dazzui::Step::new(${q(p.label)}, ${status}).tint(Tint::${cap(p.tint ?? 'primary')}); let meta = ${meta}; if meta.is_empty() { step } else { step.meta(meta) } }.into_any_element()`
    }
    case 'Progress':
      return `dazzui::Progress::${node.derive?.value ? `new(${value(node.derive.value)}.number() as f32)` : p.value === undefined ? 'indeterminate()' : `new(${number(p.value)})`}.size(dazzui::ProgressSize::${cap(p.size ?? 'small')}).tint(Tint::${cap(p.tint ?? 'primary')}).gradient(${!!p.gradient}).into_any_element()`
    case 'Divider':
      return `dazzui::Divider::${p.orientation ?? 'horizontal'}().tone(dazzui::DividerTone::${cap(p.tone ?? 'default')}).into_any_element()`
    case 'EmptyState':
      return `dazzui::EmptyState::new(${node.derive?.title ? `${value(node.derive.title)}.text().to_owned()` : q(p.title)})${node.slots?.action !== undefined ? `.action(${gpuiScene(node.slots.action, `${key}-slot-action`)})` : ''}.into_any_element()`
    case 'Callout':
      return `dazzui::Callout::new().size(dazzui::CalloutSize::${cap(p.size ?? 'medium')}).tint(Tint::${cap(p.tint ?? 'neutral')})${p.title ? `.title(${q(p.title)})` : ''}${node.children?.length ? `.description(${q(node.children.join(''))})` : ''}${Object.entries(
        node.slots ?? {}
      )
        .map(
          ([name, slot]) =>
            `.${name}(${gpuiScene(slot, `${key}-slot-${name}`)})`
        )
        .join('')}.into_any_element()`
    case 'Card':
      return `dazzui::Card::new().size(dazzui::CardSize::${cap(p.size ?? 'medium')}).variant(dazzui::CardVariant::${cap(p.variant ?? 'raised')}).tint(Tint::${cap(p.tint ?? 'primary')})${attach}.into_any_element()`
    case 'SectionLabel':
      return `dazzui::SectionLabel::new(${q(node.children.join(''))})${p.tint ? `.tint(Tint::${cap(p.tint)})` : ''}.faint(${!!p.faint}).into_any_element()`
    case 'Spinner':
      return `dazzui::Spinner::new().size(dazzui::SpinnerSize::${cap(p.size ?? 'medium')}).tint(Tint::${cap(p.tint ?? 'primary')}).on_accent(${!!p.onAccent}).into_any_element()`
    case 'KeyCap':
      return `dazzui::KeyCap::new(${q(node.children.join(''))}).size(dazzui::KeyCapSize::${cap(p.size ?? 'medium')}).variant(dazzui::KeyCapVariant::${cap(p.variant ?? 'hint')}).into_any_element()`
    case 'ActionBar':
      return `${p.pushEnd ? 'gpui::div().ml_auto().child(' : ''}dazzui::ActionBar::new().align(dazzui::ActionBarAlign::${cap(p.align ?? 'start')})${attach}${p.pushEnd ? ')' : ''}.into_any_element()`
    default:
      throw Error(`No GPUI mapping for ${node.component}`)
  }
}

export function gpuiCheckpointTests(stories, scenes) {
  return `#[cfg(test)]
mod shared_checkpoints {
use crate::scene::Value;
${stories
  .flatMap(story =>
    (scenes.get(story.id).checkpoints ?? []).map(
      (checkpoint, i) => `
#[gpui::test]
fn ${story.id.replace(/-+/g, '_')}_${i}(cx: &mut gpui::TestAppContext) {
  let (view, cx) = cx.add_window_view(|_, cx| crate::Playground::new(cx));
  view.update(cx, |state, cx| {
    state.selection = crate::storybook::StoryRef::new(${q(story.title.split('/')[1])}, ${q(story.name)});
    cx.notify();
  });
  cx.run_until_parked();
  ${checkpoint.steps
    .map(step =>
      step.action === 'advance'
        ? `for _ in 0..${step.milliseconds} { cx.background_executor.advance_clock(std::time::Duration::from_millis(1)); cx.run_until_parked(); }`
        : `let target = cx.debug_bounds(${q(step.target)}).expect("shared checkpoint target is visible").center(); cx.simulate_click(target, gpui::Modifiers::default());
cx.run_until_parked();
${step.action === 'fill' ? `cx.simulate_keystrokes("cmd-a backspace"); cx.simulate_input(${q(step.value)}); cx.run_until_parked();` : ''}`
    )
    .join('\n')}
  view.update(cx, |state, _| {
    ${Object.entries(checkpoint.expectDisabled ?? {})
      .map(
        ([key, expected]) =>
          `assert_eq!(state.generated.disabled_values.borrow()[${q(key)}], ${expected});`
      )
      .join('\n')}
    ${Object.entries(checkpoint.expect ?? {})
      .map(
        ([key, expected]) =>
          `assert_eq!(state.generated.values[${q(key)}], ${gpuiValue(expected)});`
      )
      .join('\n')}
  });
}
`
    )
  )
  .join('\n')}
}`
}
