import { TRIM_CHARACTERS } from './schema/value.mjs'
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
])
export function gpuiSupports(scene) {
  const visit = n =>
    typeof n === 'string' ||
    (supported.has(n.component) &&
      (!n.bind ||
        ['Switch', 'Checkbox', 'TextField', 'Dialog'].includes(n.component)) &&
      (!n.bind?.checked ||
        typeof scene.state?.[n.bind.checked] === 'boolean') &&
      (!n.bind?.open || typeof scene.state?.[n.bind.open] === 'boolean') &&
      (n.component !== 'TextField' ||
        typeof scene.state?.[n.bind?.value] === 'string') &&
      !(
        n.component === 'Checkbox' &&
        (n.children ?? []).some(c => typeof c !== 'string')
      ) &&
      (!n.on || n.component === 'Button') &&
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
    if (node.component === 'TextField')
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
  const attach = children.map(c => `.child(${c})`).join('')
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
    case 'Box':
      return `gpui::div()${p.display ? '.flex()' : ''}${p.align ? '.items_center()' : ''}${p.justify ? '.justify_between()' : ''}${p.tone ? '.text_color(state.theme.variables().color_content_muted)' : ''}${p.gap ? `.gap(state.theme.variables().spacing_${p.gap.split('.')[1]})` : ''}${p.width ? `.w(gpui::px(${number(p.width)}))` : ''}${p.height ? `.h(gpui::px(${number(p.height)}))` : ''}${p.maxWidth ? `.max_w(gpui::px(${number(p.maxWidth)}))` : ''}${p.grow ? '.flex_1().min_w_0()' : ''}${p.padding ? `.p(state.theme.variables().${p.padding === 'spacing.3' ? 'spacing_3' : 'spacing_2'})` : ''}${p.surface ? `.bg(state.theme.variables().${p.surface === 'color.surface.muted' ? 'color_surface_muted' : 'color_surface_chrome'})` : ''}${p.radius ? `.rounded(state.theme.variables().${p.radius === 'radius.large' ? 'radius_large' : 'radius_medium'})` : ''}${attach}.into_any_element()`
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
    case 'Row':
      return `crate::storybook::stage::Row::new()${p.gap ? `.gap(state.theme.variables().spacing_${p.gap.split('.')[1]})` : ''}${attach}.into_any_element()`
    case 'Col':
      return `crate::storybook::stage::Col::new()${p.gap ? `.gap(state.theme.variables().${p.gap.replace('.', '_')})` : ''}${p.width ? `.width(gpui::px(${number(p.width)}))` : ''}${attach}.into_any_element()`
    case 'Page':
    case 'Section':
      return `crate::storybook::gallery::${node.component === 'Page' ? 'DocPage' : 'Section'}::new(${q(p.title)})${p.description ? `.description(${q(p.description)})` : ''}${attach}.into_any_element()`
    case 'Stack':
      return `crate::storybook::gallery::Specimens::new()${attach}.into_any_element()`
    case 'Specimen':
      return `crate::storybook::gallery::Specimen::new(${q(p.label)})${attach}.into_any_element()`
    case 'Inline':
      return `gpui::div().flex().items_center()${p.tone === 'muted' ? '.text_color(state.theme.variables().color_content_muted)' : ''}${p.mono ? '.font_family("ui-monospace")' : ''}${p.fontSize ? `.text_size(state.theme.variables().${p.fontSize.replace('.', '_')}.size)` : ''}${attach}.into_any_element()`
    case 'Icon':
      return `dazzui::Icon::new(dazzui::assets::icons::${p.name === 'Circle20Filled' ? 'CIRCLE_20_FILLED' : 'ARROW_DOWNLOAD_20_REGULAR'}).size(gpui::px(${number(p.size ?? 20)})).into_any_element()`
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
      return `dazzui::EmptyState::new(${q(p.title)})${node.slots?.action !== undefined ? `.action(${gpuiScene(node.slots.action, `${key}-slot-action`)})` : ''}.into_any_element()`
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
