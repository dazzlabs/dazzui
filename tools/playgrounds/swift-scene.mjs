import { valueType, TRIM_CHARACTERS } from './schema/value.mjs'
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
        view = `HStack(spacing: 0) { ${children} }${p.fontSize ? `.font(.system(size: theme.vars.${p.fontSize === 'title.large' ? 'titleLarge' : 'bodySmall'}.size))` : ''}${p.mono ? '.fontDesign(.monospaced)' : ''}${p.tone === 'muted' ? '.foregroundStyle(theme.vars.colorContentMuted)' : ''}`
        break
      case 'Icon':
        view = `Icon(.${p.name === 'Circle20Filled' ? 'circle20Filled' : 'arrowDownload20Regular'}${p.size ? `, size: ${p.size}` : ''})`
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
      case 'EmptyState':
        view =
          (n.slots?.action !== undefined
            ? `EmptyState(title: ${q(p.title)}) { ${node(n.slots.action)} }`
            : `EmptyState(${q(p.title)})`) +
          (intrinsicHeight
            ? '.fixedSize(horizontal: false, vertical: true)'
            : '')
        break
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
        view = `Col(spacing: ${p.gap ? `theme.vars.spacing${p.gap.split('.')[1]}` : 'nil'}, width: ${p.width ?? 'nil'}) { ${children} }`
        break
      case 'Button': {
        const action = n.on?.press
        view = `DazzUI.Button(size: .${p.size ?? 'small'}, variant: .${p.variant ?? 'recessed'}, tint: .${p.tint ?? 'primary'}, fullWidth: ${!!p.expand}, shortcut: ${p.shortcut ? q(p.shortcut) : 'nil'}, enabled: ${enabled}, action: { ${action ? `${binding(action.set)} = ${q(action.value)}` : ''} }) { ${children} }`
        break
      }
      case 'Box':
        view = `${p.display ? `HStack(spacing: ${p.gap ? `theme.vars.spacing${p.gap.split('.')[1]}` : '0'})` : 'VStack(alignment: .leading, spacing: 0)'} { ${p.justify === 'between' ? childViews.join('\nSpacer(minLength: 0)\n') : children} }${p.tone ? '.foregroundStyle(theme.vars.colorContentMuted)' : ''}${p.padding ? `.padding(theme.vars.${p.padding === 'spacing.3' ? 'spacing3' : 'spacing2'})` : ''}${p.width ? `.frame(width: ${p.width}, alignment: .leading)` : ''}${p.height ? `.frame(height: ${p.height})` : ''}${p.maxWidth ? `.frame(maxWidth: ${p.maxWidth}, alignment: .leading)` : p.grow ? '.frame(maxWidth: .infinity, alignment: .leading)' : ''}${p.surface ? `.background(theme.vars.${p.surface === 'color.surface.muted' ? 'colorSurfaceMuted' : 'colorSurfaceChrome'})` : ''}${p.radius ? `.clipShape(RoundedRectangle(cornerRadius: theme.vars.${p.radius === 'radius.large' ? 'radiusLarge' : 'radiusMedium'}))` : ''}`
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
