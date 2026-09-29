import { valueType, TRIM_CHARACTERS } from './schema/value.mjs'
import { spacing } from './tokens.mjs'

/** A declared length: logical points or a spacing step. */
const len = value =>
  typeof value === 'number' ? String(value) : spacing.swift(value)
const q = value =>
  JSON.stringify(value).replace(/\\u([0-9a-f]{4})/gi, '\\u{$1}')

/** Compile typed SwiftUI views, retaining native state bindings and actions. */
export function swiftScene(scene, index) {
  const state = new Map(
    Object.entries(scene.state ?? {}).map(([key, value], i) => [
      key,
      { name: `value${i}`, value },
    ])
  )
  const binding = key => state.get(key).name
  function expression(value) {
    if (typeof value !== 'object') return q(value)
    if (Object.hasOwn(value, 'state')) return binding(value.state)
    if ('multiply' in value)
      return `(Double(${expression(value.multiply[0])}) * Double(${expression(value.multiply[1])}))`
    if ('add' in value)
      return `(Double(${expression(value.add[0])}) + Double(${expression(value.add[1])}))`
    if ('gte' in value)
      return `(Double(${expression(value.gte[0])}) >= Double(${expression(value.gte[1])}))`
    if ('trim' in value)
      return `${expression(value.trim)}.trimmingCharacters(in: CharacterSet(charactersIn: "${[...TRIM_CHARACTERS].map(c => '\\u{' + c.charCodeAt(0).toString(16) + '}').join('')}"))`
    if ('text' in value) return `sceneText(Double(${expression(value.text)}))`
    if ('not' in value) return `(!${expression(value.not)})`
    if ('and' in value) return `(${value.and.map(expression).join(' && ')})`
    if ('or' in value) return `(${value.or.map(expression).join(' || ')})`
    if ('lower' in value) return `${expression(value.lower)}.lowercased()`
    if ('length' in value)
      return `Double(${expression(value.length)}.unicodeScalars.count)`
    if ('contains' in value)
      return `(${expression(value.contains[1])}.isEmpty || ${expression(value.contains[0])}.contains(${expression(value.contains[1])}))`
    if ('eq' in value) {
      const operand = v =>
        valueType(v, scene.state ?? {}) === 'number'
          ? `Double(${expression(v)})`
          : expression(v)
      return `(${operand(value.eq[0])} == ${operand(value.eq[1])})`
    }
    if ('concat' in value)
      return `(${value.concat.map(expression).join(' + ')})`
    return `(${expression(value.if)} ? ${expression(value.then)} : ${expression(value.else)})`
  }
  function node(n, fillWidth = false, intrinsicHeight = false) {
    if (typeof n === 'string') return `Text(${q(n)})`
    const p = n.props ?? {}
    const childViews = (n.children ?? []).map(child =>
      node(
        child,
        n.component === 'Grid' ||
          (n.component === 'Box' &&
            !p.display &&
            (p.width !== undefined || fillWidth)),
        n.component === 'Card' || (n.component === 'Box' && intrinsicHeight)
      )
    )
    const children = childViews.join('\n')
    const fieldState = n.derive?.state
      ? `(${expression(n.derive.state)} == "error" ? .error : .default)`
      : `.${p.state ?? 'default'}`
    const act = event =>
      n.on?.[event]?.set !== undefined
        ? `${binding(n.on[event].set)} = ${q(n.on[event].value)}`
        : ''
    const enabled = n.derive?.disabled
      ? `!(${expression(n.derive.disabled)})`
      : String(!p.disabled)
    let view
    switch (n.component) {
      case 'Table':
      case 'TableHead':
      case 'TableRow':
        view = `DazzUI.${n.component} { ${children} }`
        break
      case 'TableCell':
        view = `TableCell(head: ${!!p.head}, align: .${p.align ?? 'start'}) { ${children} }`
        break
      case 'Badge':
        view = `Badge(size: .${p.size ?? 'medium'}, variant: .${p.variant ?? 'tinted'}, tint: .${p.tint ?? 'primary'}) { HStack(spacing: 0) { ${children} } }`
        break
      case 'Value':
        view = `Text(${valueType(n.value, scene.state ?? {}) === 'string' ? expression(n.value) : `String(describing: ${expression(n.value)})`})`
        break
      case 'SegmentedItem':
      case 'RadioOption':
      case 'PillTab':
      case 'ToggleItem':
        // Read as data by the group that holds them.
        return 'EmptyView()'
      case 'RadioGroup': {
        const options = (n.children ?? [])
          .map(
            option =>
              `RadioGroupOption(value: ${q(option.props.value)}, label: ${q(option.props.label)}, enabled: ${!option.props.disabled})`
          )
          .join(', ')
        view = `RadioGroup(options: [${options}], value: ${n.bind?.value ? binding(n.bind.value) : q(p.value ?? '')}, onValueChange: ${n.bind?.value ? `{ ${binding(n.bind.value)} = $0 }` : 'nil'}, size: .${p.size ?? 'medium'}, tint: .${p.tint ?? 'primary'}, axis: .${p.direction === 'row' ? 'horizontal' : 'vertical'}, spacing: ${p.gap ? spacing.swift(p.gap) : 'nil'})${p.label ? `.accessibilityLabel(${q(p.label)})` : ''}`
        break
      }
      case 'PillTabs': {
        const items = (n.children ?? [])
          .map(
            item =>
              `PillTabItem(value: ${q(item.props.value)}, label: ${q(item.props.label)}, count: ${item.props.count ?? 'nil'}, disabled: ${!!item.props.disabled})`
          )
          .join(', ')
        view = `PillTabs(items: [${items}], value: ${n.bind?.value ? binding(n.bind.value) : q(p.value ?? '')}, onValueChange: ${n.bind?.value ? `{ ${binding(n.bind.value)} = $0 }` : 'nil'}, tint: .${p.tint ?? 'primary'})${p.label ? `.accessibilityLabel(${q(p.label)})` : ''}`
        break
      }
      case 'ToggleGroup': {
        const items = n.children ?? []
        const single = p.multiple === false
        const list = items
          .map(item => {
            const glyph = (item.children ?? []).find(
              child => typeof child === 'object' && child.component === 'Icon'
            )
            return `ToggleItem(value: ${q(item.props.value)}, label: ${q(item.props.label)}, enabled: ${!item.props.disabled}${glyph ? `, icon: .${glyph.props.name[0].toLowerCase() + glyph.props.name.slice(1)}` : ''})`
          })
          .join(', ')
        const pressed = item =>
          item.bind?.pressed
            ? binding(item.bind.pressed)
            : String(!!item.props.pressed)
        const value = single
          ? n.bind?.value
            ? `(${binding(n.bind.value)}.isEmpty ? [] : [${binding(n.bind.value)}])`
            : q(p.value ? [p.value] : [])
          : `[${items.map(item => `${pressed(item)} ? ${q(item.props.value)} : nil`).join(', ')}].compactMap { $0 }`
        const change = single
          ? n.bind?.value
            ? `{ ${binding(n.bind.value)} = $0.first ?? "" }`
            : 'nil'
          : `{ next in ${items
              .filter(item => item.bind?.pressed)
              .map(
                item =>
                  `${binding(item.bind.pressed)} = next.contains(${q(item.props.value)})`
              )
              .join('; ')} }`
        view = `ToggleGroup(items: [${list}], value: ${value}, multiple: ${!single}, size: .${p.size ?? 'small'}, variant: .${p.variant ?? 'recessed'}, tint: .${p.tint ?? 'primary'}, onChange: ${change})${p.label ? `.accessibilityLabel(${q(p.label)})` : ''}`
        break
      }
      case 'OptionCard': {
        const selected = n.derive?.selected
          ? expression(n.derive.selected)
          : String(!!p.selected)
        view = `OptionCard(selected: ${selected}, tint: .${p.tint ?? 'primary'}, enabled: ${!p.disabled}, action: { ${act('press')} }, title: { ${n.slots?.title !== undefined ? node(n.slots.title) : `Text(${q(p.title ?? '')})`} }, description: { ${p.description ? `Text(${q(p.description)})` : 'EmptyView()'} })`
        break
      }
      case 'Slider': {
        const keys = [n.bind?.value, n.bind?.end].filter(Boolean).map(binding)
        const values = keys.length
          ? `Binding(get: { [${keys.join(', ')}] }, set: { next in ${keys.map((key, i) => `if next.count > ${i} { ${key} = next[${i}] }`).join('; ')} })`
          : `.constant([${[p.value ?? 0, ...(p.end !== undefined ? [p.end] : [])].join(', ')}])`
        const readings = keys.length
          ? keys
          : [p.value ?? 0, ...(p.end !== undefined ? [p.end] : [])].map(String)
        // React prints the reading: a range as "a – b", a unit after one value.
        const label = !p.showValue
          ? 'nil'
          : p.unit
            ? `sceneText(${readings[0]}) + ${q(p.unit)}`
            : readings.map(r => `sceneText(${r})`).join(' + " – " + ')
        view = `DazzUI.Slider(values: ${values}, minimum: ${p.min ?? 0}, maximum: ${p.max ?? 100}, step: ${p.step ?? 1}, size: .${p.size ?? 'medium'}, tint: .${p.tint ?? 'primary'}, valueLabel: ${label}, enabled: ${!p.disabled})${p.label ? `.accessibilityLabel(${q(p.label)})` : ''}`
        break
      }
      case 'NumberField': {
        const value = n.bind?.value
          ? `Binding(get: { Double(${binding(n.bind.value)}) }, set: { ${binding(n.bind.value)} = $0.map(sceneText) ?? "" })`
          : `.constant(Double(${q(p.value ?? '')}))`
        view = `NumberField(value: ${value}, minimum: ${p.min ?? 'nil'}, maximum: ${p.max ?? 'nil'}, step: ${p.step ?? 1}, placeholder: ${q(p.placeholder ?? '')}, unit: ${p.unit ? q(p.unit) : 'nil'}, size: .${p.size ?? 'medium'}, state: ${fieldState}, tint: .${p.tint ?? 'primary'}, enabled: ${enabled})`
        break
      }
      case 'SearchField': {
        const key = n.bind?.value && binding(n.bind.value)
        view = `SearchField(value: ${key ?? '""'}, onValueChange: { ${key ? `${key} = $0` : '_ in'} }, placeholder: ${q(p.placeholder ?? 'Search')}, size: .${p.size ?? 'medium'}, tint: .${p.tint ?? 'primary'}, onDismiss: ${n.on?.dismiss ? `{ ${act('dismiss')} }` : 'nil'}, clearLabel: ${q(p.clearLabel ?? 'Clear search')}) { ${n.slots?.hint !== undefined ? node(n.slots.hint) : 'EmptyView()'} }`
        break
      }
      case 'OtpField':
        view = `OtpField(value: ${n.bind?.value ? `$${binding(n.bind.value)}` : `.constant(${q(p.value ?? '')})`}, length: ${p.length ?? 6}, groupAfter: ${p.groupAfter ?? 'nil'}, mask: ${!!p.mask}, size: .${p.size ?? 'medium'}, state: ${fieldState}, tint: .${p.tint ?? 'primary'}, enabled: ${enabled})`
        break
      case 'Show':
        return `if ${expression(n.value)} {\n${children}\n}`
      case 'Tag': {
        if (n.slots?.icon) throw Error('SwiftUI Tag has no glyph slot')
        const selectable = n.bind?.selected || n.on?.select
        const selected = n.bind?.selected
          ? binding(n.bind.selected)
          : selectable
            ? String(!!p.selected)
            : 'nil'
        const action = event =>
          n.on?.[event]?.set !== undefined
            ? `${binding(n.on[event].set)} = ${q(n.on[event].value)}`
            : ''
        const onSelect = selectable
          ? `{ selected in ${n.bind?.selected ? `${binding(n.bind.selected)} = selected;` : ''} ${action('select')} }`
          : 'nil'
        const onRemove = n.on?.remove ? `{ ${action('remove')} }` : 'nil'
        view = `DazzUI.Tag(${q((n.children ?? []).filter(c => typeof c === 'string').join(''))}, size: .${p.size ?? 'medium'}, variant: .${p.variant ?? 'tinted'}, tint: .${p.tint ?? 'primary'}, selected: ${selected}, enabled: ${!p.disabled}, removeLabel: ${q(p.removeLabel ?? 'Remove')}, onSelect: ${onSelect}, onRemove: ${onRemove})`
        break
      }
      case 'Link': {
        const press =
          n.on?.press?.set !== undefined
            ? `{ ${binding(n.on.press.set)} = ${q(n.on.press.value)} }`
            : '{}'
        view = `DazzUI.Link(${q((n.children ?? []).join(''))}, tint: .${p.tint ?? 'primary'}, underline: .${p.underline ?? 'hover'}, external: ${!!p.external}, enabled: ${!p.disabled}, action: ${press})`
        break
      }
      case 'Meter': {
        const min = p.min ?? 0,
          max = p.max ?? 100
        view = `Meter(value: ${p.value}, minimum: ${min}, maximum: ${max}, size: .${p.size ?? 'medium'}, tint: .${p.tint ?? 'primary'}, label: ${p.label ? q(p.label) : 'nil'}, valueLabel: ${p.showValue ? q(`${Math.round(((p.value - min) / (max - min)) * 100)}%`) : 'nil'})`
        break
      }
      case 'Skeleton': {
        const percent = typeof p.width === 'string' && p.width.endsWith('%')
        view = `Skeleton(shape: .${p.shape ?? 'text'}, width: ${p.width !== undefined && !percent ? len(p.width) : 'nil'}, height: ${p.height !== undefined ? len(p.height) : 'nil'})`
        if (percent)
          view = `FractionalWidth(${parseFloat(p.width) / 100}) { ${view} }`
        break
      }
      case 'SkeletonText':
        view = `SkeletonText(lines: ${p.lines ?? 3})`
        break
      case 'SegmentedControl': {
        const items = (n.children ?? [])
          .map(
            item =>
              `SegmentedItem(value: ${q(item.props.value)}, label: ${q(item.props.label)}, disabled: ${!!item.props.disabled})`
          )
          .join(', ')
        const value = n.bind?.value ? binding(n.bind.value) : q(p.value ?? '')
        const change = n.bind?.value
          ? `{ ${binding(n.bind.value)} = $0 }`
          : '{ _ in }'
        view = `DazzUI.SegmentedControl(items: [${items}], value: ${value}, onValueChange: ${change}, size: .${p.size ?? 'medium'}, tint: .${p.tint ?? 'primary'}, activeStyle: .${p.activeStyle ?? 'raised'}, stretch: ${!!p.stretch})${p.label ? `.accessibilityLabel(${q(p.label)})` : ''}`
        break
      }
      case 'Switch':
      case 'Checkbox': {
        const checked = n.bind?.checked
          ? binding(n.bind.checked)
          : String(p.checked ?? false)
        const change = n.bind?.checked
          ? `{ ${binding(n.bind.checked)} = $0 }`
          : 'nil'
        view = `DazzUI.${n.component}(checked: ${checked}, onCheckedChange: ${change}, size: .${p.size ?? 'medium'}, tint: .${p.tint ?? 'primary'}, enabled: ${enabled})${n.component === 'Checkbox' ? ` { ${children} }` : `.accessibilityLabel(${q(p.label ?? '')})`}`
        break
      }
      case 'PreferenceSection':
        view = `PreferenceSection(label: ${p.label ? q(p.label) : 'nil'}, action: { EmptyView() }) { ${children} }`
        break
      case 'PreferenceRow':
        view = `PreferenceRow(title: ${q(p.title)}, subtitle: ${p.subtitle ? q(p.subtitle) : 'nil'}, icon: { EmptyView() }) { ${children} }`
        break
      case 'Page':
        view = `Page(${q(p.title)}, description: ${p.description ? q(p.description) : 'nil'}) { ${children} }`
        break
      case 'Section':
        view = `Section(${q(p.title)}, description: ${p.description ? q(p.description) : 'nil'}) { ${children} }`
        break
      case 'Stack':
        view = `Stack { ${children} }`
        break
      case 'Grid':
        view = `Grid(min: ${p.min ?? 256}) { ${children} }`
        break
      case 'Specimen':
        view = `Specimen(${q(p.label)}) { ${(n.children ?? []).some(child => child.component === 'Divider' && child.props?.orientation === 'vertical') ? `HStack(spacing: theme.vars.spacing3) { ${children} }.fixedSize(horizontal: false, vertical: true)` : children} }`
        break
      case 'Inline':
        view = `HStack(spacing: 0) { ${children} }${p.fontSize ? `.font(.system(size: theme.vars.${p.fontSize === 'title.large' ? 'titleLarge' : 'bodySmall'}.size))` : ''}${p.mono ? '.fontDesign(.monospaced)' : ''}${p.tone ? `.foregroundStyle(theme.vars.colorContent${p.tone[0].toUpperCase() + p.tone.slice(1)})` : ''}`
        break
      case 'Icon':
        view = `Icon(.${p.name[0].toLowerCase() + p.name.slice(1)}${p.size ? `, size: ${typeof p.size === 'number' ? p.size : `theme.vars.spacing${p.size.split('.')[1]}`}` : ''})`
        break
      case 'StepList':
        view = `StepList { ${children} }`
        break
      case 'Step': {
        const status = n.derive?.status
          ? `(${expression(n.derive.status)} == "done" ? StepStatus.done : ${expression(n.derive.status)} == "active" ? StepStatus.active : StepStatus.idle)`
          : `.${p.status}`
        const meta = n.derive?.meta
          ? expression(n.derive.meta)
          : q(p.meta ?? '')
        view = `Step(status: ${status}, tint: .${p.tint ?? 'primary'}, label: { Text(${q(p.label)}) }, meta: { if !${meta}.isEmpty { Text(${meta}) } })`
        break
      }
      case 'Progress':
        view = `Progress(value: ${n.derive?.value ? expression(n.derive.value) : (p.value ?? 'nil')}, size: .${p.size ?? 'small'}, tint: .${p.tint ?? 'primary'}, gradient: ${!!p.gradient})`
        break
      case 'Divider':
        view = `DazzUI.Divider(tone: .${p.tone ?? 'default'}, orientation: .${p.orientation ?? 'horizontal'})`
        break
      case 'EmptyState': {
        const title = n.derive?.title ? expression(n.derive.title) : q(p.title)
        view =
          (n.slots?.action !== undefined
            ? `EmptyState(title: ${title}) { ${node(n.slots.action)} }`
            : `EmptyState(${title})`) +
          (intrinsicHeight
            ? '.fixedSize(horizontal: false, vertical: true)'
            : '')
        break
      }
      case 'Callout':
        view = `Callout(size: .${p.size ?? 'medium'}, tint: .${p.tint ?? 'neutral'}, title: ${p.title ? q(p.title) : 'nil'}, leading: { ${n.slots?.icon !== undefined ? node(n.slots.icon) : 'EmptyView()'} }, description: { ${children || 'EmptyView()'} }, trailing: { ${n.slots?.action !== undefined ? node(n.slots.action) : 'EmptyView()'} })`
        break
      case 'Card':
        view = `Card(size: .${p.size ?? 'medium'}, variant: .${p.variant ?? 'raised'}, tint: .${p.tint ?? 'primary'}) { ${fillWidth ? childViews.map(child => `${child}.frame(maxWidth: .infinity, alignment: .leading)`).join('\n') : children} }`
        break
      case 'SectionLabel':
        view = `SectionLabel(tint: ${p.tint ? `.${p.tint}` : 'nil'}, faint: ${!!p.faint}) { ${children} }`
        break
      case 'Spinner':
        view = `Spinner(size: .${p.size ?? 'medium'}, tint: .${p.tint ?? 'primary'}, onAccent: ${!!p.onAccent})`
        break
      case 'KeyCap':
        view = `KeyCap(size: .${p.size ?? 'medium'}, variant: .${p.variant ?? 'hint'}) { ${children} }`
        break
      case 'Row':
        view = `Row(spacing: ${p.gap ? `theme.vars.spacing${p.gap.split('.')[1]}` : 'nil'}) { ${children} }`
        break
      case 'Col':
        view = `Col(spacing: ${p.gap ? spacing.swift(p.gap) : 'nil'}, width: ${p.width !== undefined ? len(p.width) : 'nil'}) { ${children} }`
        break
      case 'Button': {
        const action = n.on?.press
        view = `DazzUI.Button(size: .${p.size ?? 'small'}, variant: .${p.variant ?? 'recessed'}, tint: .${p.tint ?? 'primary'}, fullWidth: ${!!p.expand}, shortcut: ${p.shortcut ? q(p.shortcut) : 'nil'}, enabled: ${enabled}, action: { ${action ? `${binding(action.set)} = ${q(action.value)}` : ''} }) { ${children} }`
        break
      }
      case 'IconButton': {
        const action = n.on?.press
        view = `DazzUI.IconButton(label: ${q(p.label)}, size: .${p.size ?? 'tiny'}, variant: .${p.variant ?? 'plain'}, tint: .${p.tint ?? 'neutral'}, enabled: ${enabled}, action: { ${action ? `${binding(action.set)} = ${q(action.value)}` : ''} }) { ${children} }`
        break
      }
      case 'Toggle': {
        const pressed = n.bind?.pressed
          ? binding(n.bind.pressed)
          : String(!!p.pressed)
        const glyphOnly =
          n.children?.length === 1 && typeof n.children[0] === 'object'
        const content = (n.children ?? [])
          .map(child =>
            typeof child === 'object' && child.props?.fit
              ? `Icon(.${child.props.name[0].toLowerCase() + child.props.name.slice(1)}, size: theme.vars.metrics(.${p.size ?? 'tiny'}).content.size)`
              : node(child)
          )
          .join('\n')
        view = `DazzUI.Toggle(pressed: ${pressed}, size: .${p.size ?? 'tiny'}, variant: .${p.variant ?? 'plain'}, tint: .${p.tint ?? 'neutral'}, enabled: ${enabled}, glyphOnly: ${glyphOnly}, accessibilityName: ${p.label ? q(p.label) : 'nil'}, action: ${n.bind?.pressed ? `{ ${pressed}.toggle() }` : 'nil'}) { HStack(spacing: 0) { ${content} } }`
        break
      }
      case 'Box':
        view = `${p.display && p.wrap ? `Flow(spacing: ${p.gap ? spacing.swift(p.gap) : '0'})` : p.display ? `HStack(spacing: ${p.gap ? spacing.swift(p.gap) : '0'})` : 'VStack(alignment: .leading, spacing: 0)'} { ${p.justify === 'between' ? childViews.join('\nSpacer(minLength: 0)\n') : children} }${p.tone ? '.foregroundStyle(theme.vars.colorContentMuted)' : ''}${p.padding ? `.padding(theme.vars.${p.padding === 'spacing.3' ? 'spacing3' : 'spacing2'})` : ''}${p.width ? `.frame(width: ${len(p.width)}, alignment: .leading)` : ''}${p.height ? `.frame(height: ${len(p.height)})` : ''}${p.maxWidth ? `.frame(maxWidth: ${len(p.maxWidth)}, alignment: .leading)` : p.grow ? '.frame(maxWidth: .infinity, alignment: .leading)' : ''}${p.surface ? `.background(theme.vars.${p.surface === 'color.surface.muted' ? 'colorSurfaceMuted' : 'colorSurfaceChrome'})` : ''}${p.radius ? `.clipShape(RoundedRectangle(cornerRadius: theme.vars.${p.radius === 'radius.large' ? 'radiusLarge' : 'radiusMedium'}))` : ''}`
        break
      case 'FormField':
        view = `FormField(label: ${q(p.label)}, hint: ${n.derive?.hint ? expression(n.derive.hint) : p.hint ? q(p.hint) : 'nil'}, state: ${fieldState}) { ${children} }`
        break
      case 'TextField':
        view = p.readOnly
          ? `FieldValue(size: .${p.size ?? 'medium'}, mono: ${!!p.mono}) { Text(${binding(n.bind.value)}).textSelection(.enabled) }`
          : `DazzUI.TextField(text: $${binding(n.bind.value)}, placeholder: ${q(p.placeholder ?? '')}, size: .${p.size ?? 'medium'}, state: ${fieldState}, tint: .${p.tint ?? 'primary'}, mono: ${!!p.mono}, enabled: ${enabled})`
        break
      case 'Text':
        view = `Text(${q(n.children[0])}).font(theme.vars.bodyMedium.font).foregroundStyle(theme.vars.colorContentSecondary)`
        break
      case 'Dialog':
        return `if ${binding(n.bind.open)} { DialogScrim { Dialog(width: ${p.width ?? 'nil'}, tone: .${p.tone === 'danger' ? 'danger' : 'default'}) { ${children} } } }`
      case 'DialogHeader':
        view = `DialogHeader(${q(p.title)}, subtitle: ${p.subtitle ? q(p.subtitle) : 'nil'})`
        break
      case 'DialogBody':
        view = `DialogBody { ${children} }`
        break
      case 'DialogFooter':
        view = `DialogFooter { ${children} }`
        break
      case 'ActionBar':
        view =
          p.align === 'center'
            ? `HStack { Spacer(); ActionBar { ${children} }; Spacer() }`
            : `ActionBar(align: .${p.align ?? 'start'}) { ${children} }`
        if (
          (n.children ?? []).some(
            child =>
              child.component === 'Divider' &&
              child.props?.orientation === 'vertical'
          )
        )
          view += '.fixedSize(horizontal: false, vertical: true)'
        break
      default:
        throw new Error(`SwiftUI scene compiler: unsupported ${n.component}`)
    }
    return view + (n.id ? `.accessibilityIdentifier(${q(n.id)})` : '')
  }
  return `private struct PortableScene${index}: View {
@Environment(\\.theme) private var theme
${[...state.values()].map(s => `@State private var ${s.name}${typeof s.value === 'number' ? ': Double' : ''} = ${q(s.value)}`).join('\n')}
var body: some View { ${node(scene.content)}${(scene.timers ?? [])
    .map(
      timer => `.task {
while !Task.isCancelled {
do { try await Task.sleep(for: .milliseconds(${timer.every})) } catch { return }
if Task.isCancelled { return }
${binding(timer.set)} = ${expression(timer.value)}
}
}`
    )
    .join('')} }
}`
}
