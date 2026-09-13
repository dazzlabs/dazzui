import { spacingPixels } from './tokens.mjs'
import { GLYPHS } from './schema/glyphs.mjs'
import { valueType, TRIM_CHARACTERS } from './schema/value.mjs'
const q = value =>
  JSON.stringify(value).replace(/\\u([0-9a-f]{4})/gi, '\\u{$1}')

// Mobile maps semantics to system controls. Desktop geometry and recipes are
// intentionally not emitted; mobileTheme is the sole palette installation.
export function iosScene(scene, index) {
  const state = new Map(
    Object.entries(scene.state ?? {}).map(([key, value], i) => [
      key,
      { name: `value${i}`, value },
    ])
  )
  const binding = key => state.get(key).name
  function expression(value) {
    if (typeof value !== 'object') return q(value)
    if ('state' in value) return binding(value.state)
    for (const [key, op] of [
      ['add', '+'],
      ['multiply', '*'],
      ['gte', '>='],
    ])
      if (key in value)
        return `(Double(${expression(value[key][0])}) ${op} Double(${expression(value[key][1])}))`
    if ('trim' in value)
      return `${expression(value.trim)}.trimmingCharacters(in: CharacterSet(charactersIn: "${[...TRIM_CHARACTERS].map(c => '\\u{' + c.charCodeAt(0).toString(16) + '}').join('')}"))`
    if ('text' in value)
      return `mobileNumberText(Double(${expression(value.text)}))`
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
  const methods = []
  let serial = 0
  function node(n) {
    if (typeof n === 'string') return `Text(${q(n)})`
    const method = `node${serial++}`
    const p = n.props ?? {}
    const children = [
      ...([
        'SegmentedControl',
        'RadioGroup',
        'PillTabs',
        'ToggleGroup',
      ].includes(n.component)
        ? []
        : (n.children ?? []).map(node)),
      ...(n.component === 'Button' && p.shortcut
        ? [`Text(${q(p.shortcut)}).font(.caption)`]
        : []),
    ].join(
      (n.component === 'ActionBar' && p.align === 'between') ||
        (n.component === 'Box' && p.justify === 'between')
        ? '\nSpacer()\n'
        : '\n'
    )
    const vertical = `VStack(alignment: .leading) {\n${children} }`
    const horizontal = `HStack {\n${children} }`
    const prop = (key, fallback) =>
      n.derive?.[key] ? expression(n.derive[key]) : q(p[key] ?? fallback)
    const slot = key => (n.slots?.[key] ? node(n.slots[key]) : '')
    let view
    switch (n.component) {
      case 'Button': {
        const action = n.on?.press
        view = `Button${p.tint === 'danger' ? '(role: .destructive)' : ''} { ${action ? `${binding(action.set)} = ${q(action.value)}` : ''} } label: { ${horizontal} }.disabled(${prop('disabled', false)})`
        if (p.variant === 'filled') view += '.buttonStyle(.borderedProminent)'
        else if (p.variant === 'plain' || p.variant === 'normal')
          view += '.buttonStyle(.plain)'
        else view += '.buttonStyle(.bordered)'
        if (p.shortcut) view += `.accessibilityHint(${q(p.shortcut)})`
        break
      }
      case 'Show':
        view = `Group {\nif ${expression(n.value)} {\n${children}\n}\n}`
        break
      case 'Tag': {
        if (n.slots?.icon) throw Error('iOS Tag glyph slot pending')
        const label = q(
          (n.children ?? []).filter(c => typeof c === 'string').join('')
        )
        const assign = event =>
          n.on?.[event]?.set !== undefined
            ? `${binding(n.on[event].set)} = ${q(n.on[event].value)}`
            : ''
        const selectable = n.bind?.selected || n.on?.select
        // System controls: a button-styled toggle for a selectable chip, a
        // bordered label with a dismiss button for a removable one.
        const chip = selectable
          ? `Toggle(isOn: ${n.bind?.selected ? `$${binding(n.bind.selected)}` : `.constant(${!!p.selected})`}) { Text(${label}) }.toggleStyle(.button)`
          : `Text(${label}).padding(.horizontal, 8).padding(.vertical, 4).overlay(Capsule().stroke(.secondary))`
        view = n.on?.remove
          ? `HStack(spacing: 4) {\n${chip}\nButton { ${assign('remove')} } label: { Image(systemName: "xmark.circle.fill") }.buttonStyle(.borderless).accessibilityLabel(${q(p.removeLabel ?? 'Remove')})\n}`
          : chip
        view += `.font(.caption).disabled(${!!p.disabled})`
        break
      }
      case 'Link':
        view = `Button { ${n.on?.press?.set !== undefined ? `${binding(n.on.press.set)} = ${q(n.on.press.value)}` : ''} } label: { HStack(spacing: 2) { Text(${q((n.children ?? []).join(''))})${p.underline === 'always' ? '.underline()' : ''}${p.external ? '\nImage(systemName: "arrow.up.right").font(.caption2)' : ''} } }.buttonStyle(.borderless).disabled(${!!p.disabled})`
        break
      case 'Meter': {
        const min = p.min ?? 0,
          max = p.max ?? 100
        view = `Gauge(value: ${p.value}, in: ${min}...${max}) { ${p.label ? `Text(${q(p.label)})` : 'EmptyView()'} } currentValueLabel: { ${p.showValue ? `Text(${q(`${Math.round(((p.value - min) / (max - min)) * 100)}%`)})` : 'EmptyView()'} }.gaugeStyle(.linearCapacity)`
        break
      }
      case 'Skeleton': {
        const shape = {
          circle: 'Circle()',
          rect: 'RoundedRectangle(cornerRadius: 6)',
          text: 'Capsule()',
        }[p.shape ?? 'text']
        const size = value =>
          typeof value === 'number'
            ? value
            : typeof value === 'string' && value.startsWith('spacing.')
              ? spacingPixels(value)
              : null
        const width = size(p.width),
          height = size(p.height)
        const side = p.shape === 'circle' ? (width ?? height ?? 40) : null
        view = `${shape}.fill(.quaternary).frame(width: ${side ?? width ?? 'nil'}, height: ${side ?? height ?? (p.shape === 'rect' ? 'nil' : 16)})`
        if (typeof p.width === 'string' && p.width.endsWith('%'))
          view = `MobileFraction(${parseFloat(p.width) / 100}) { ${view}.frame(maxWidth: .infinity) }`
        else if (width === null && p.shape !== 'circle')
          view += '.frame(maxWidth: .infinity)'
        break
      }
      case 'SkeletonText':
        view = `VStack(alignment: .leading, spacing: 6) {\n${Array.from({ length: p.lines ?? 3 }, (_, i) => (i + 1 === (p.lines ?? 3) ? 'MobileFraction(0.6) { Capsule().fill(.quaternary).frame(height: 16).frame(maxWidth: .infinity) }' : 'Capsule().fill(.quaternary).frame(height: 16).frame(maxWidth: .infinity)')).join('\n')}\n}`
        break
      case 'IconButton': {
        const action = n.on?.press
        view = `Button { ${action ? `${binding(action.set)} = ${q(action.value)}` : ''} } label: { ${horizontal} }.disabled(${prop('disabled', false)}).accessibilityLabel(${q(p.label)})`
        view +=
          p.variant === 'filled'
            ? '.buttonStyle(.borderedProminent)'
            : p.variant === 'plain' || !p.variant
              ? '.buttonStyle(.borderless)'
              : '.buttonStyle(.bordered)'
        break
      }
      case 'Toggle': {
        const isOn = n.bind?.pressed
          ? `$${binding(n.bind.pressed)}`
          : `.constant(${!!p.pressed})`
        view = `Toggle(isOn: ${isOn}) { ${horizontal} }.toggleStyle(.button).disabled(${prop('disabled', false)})${p.label ? `.accessibilityLabel(${q(p.label)})` : ''}`
        break
      }
      case 'SegmentedControl': {
        const selection = n.bind?.value
          ? `$${binding(n.bind.value)}`
          : `.constant(${q(p.value ?? '')})`
        const segments = (n.children ?? [])
          .map(
            item =>
              `Text(${q(item.props.label)}).tag(${q(item.props.value)}).modifier(MobileSegmentAvailability(disabled: ${!!item.props.disabled}))`
          )
          .join('\n')
        view = `Picker(${q(p.label ?? '')}, selection: ${selection}) {\n${segments}\n}.pickerStyle(.segmented)`
        break
      }
      case 'RadioGroup': {
        const selection = n.bind?.value
          ? `$${binding(n.bind.value)}`
          : `.constant(${q(p.value ?? '')})`
        const options = (n.children ?? [])
          .map(
            option =>
              `Text(${q(option.props.label)}).tag(${q(option.props.value)}).modifier(MobileSegmentAvailability(disabled: ${!!option.props.disabled}))`
          )
          .join('\n')
        view = `Picker(${q(p.label ?? '')}, selection: ${selection}) {\n${options}\n}.pickerStyle(.inline)`
        break
      }
      case 'PillTabs': {
        const value = n.bind?.value ? binding(n.bind.value) : q(p.value ?? '')
        // System buttons in a scrolling row; the chosen filter is prominent.
        const pills = (n.children ?? [])
          .map(item => {
            const label = `HStack(spacing: 4) { Text(${q(item.props.label)})${item.props.count !== undefined ? `\nText(${q(String(item.props.count))}).foregroundStyle(.secondary)` : ''} }`
            const press = n.bind?.value
              ? `${binding(n.bind.value)} = ${q(item.props.value)}`
              : ''
            return `Group { if ${value} == ${q(item.props.value)} { Button { ${press} } label: { ${label} }.buttonStyle(.borderedProminent) } else { Button { ${press} } label: { ${label} }.buttonStyle(.bordered) } }.disabled(${!!item.props.disabled})`
          })
          .join('\n')
        view = `ScrollView(.horizontal, showsIndicators: false) { HStack {\n${pills}\n} }.font(.caption)${p.label ? `.accessibilityLabel(${q(p.label)})` : ''}`
        break
      }
      case 'ToggleGroup': {
        const single = p.multiple === false
        const toggles = (n.children ?? [])
          .map(item => {
            const isOn = single
              ? n.bind?.value
                ? `Binding(get: { ${binding(n.bind.value)} == ${q(item.props.value)} }, set: { ${binding(n.bind.value)} = $0 ? ${q(item.props.value)} : "" })`
                : `.constant(${q(p.value ?? '')} == ${q(item.props.value)})`
              : item.bind?.pressed
                ? `$${binding(item.bind.pressed)}`
                : `.constant(${!!item.props.pressed})`
            const glyph = (item.children ?? []).find(
              child => typeof child === 'object' && child.component === 'Icon'
            )
            return `Toggle(isOn: ${isOn}) { ${glyph ? `Image(systemName: ${q(GLYPHS[glyph.props.name].symbol)})` : `Text(${q(item.props.label)})`} }.toggleStyle(.button).disabled(${!!item.props.disabled}).accessibilityLabel(${q(item.props.label)})`
          })
          .join('\n')
        view = `ControlGroup {\n${toggles}\n}${p.label ? `.accessibilityLabel(${q(p.label)})` : ''}`
        break
      }
      case 'OptionCard': {
        const press =
          n.on?.press?.set !== undefined
            ? `${binding(n.on.press.set)} = ${q(n.on.press.value)}`
            : ''
        const selected = prop('selected', false)
        view = `Button { ${press} } label: { HStack(alignment: .top) { VStack(alignment: .leading, spacing: 4) { ${n.slots?.title ? slot('title') : `Text(${q(p.title ?? '')})`}.font(.headline)\n${p.description ? `Text(${q(p.description)}).font(.caption).foregroundStyle(.secondary)` : ''} }\nSpacer(minLength: 8)\nImage(systemName: ${selected} ? "checkmark.circle.fill" : "circle") }.multilineTextAlignment(.leading).frame(maxWidth: .infinity, alignment: .leading) }.buttonStyle(.bordered).disabled(${!!p.disabled}).accessibilityAddTraits(${selected} ? .isSelected : [])`
        break
      }
      case 'Slider': {
        const range = `${p.min ?? 0}...${p.max ?? 100}`
        const bound = key =>
          key ? `$${binding(key)}` : `.constant(${p.value ?? 0})`
        const reading = key =>
          key ? `mobileNumberText(${binding(key)})` : q(String(p.value ?? 0))
        const label = !p.showValue
          ? ''
          : p.unit
            ? `\nText(${reading(n.bind?.value)} + ${q(p.unit)}).monospacedDigit()`
            : n.bind?.end
              ? `\nText(${reading(n.bind.value)} + " – " + ${reading(n.bind.end)}).monospacedDigit()`
              : `\nText(${reading(n.bind?.value)}).monospacedDigit()`
        // The system has no two-thumb slider: a range is two, each held to its side.
        const sliders = n.bind?.end
          ? `Slider(value: Binding(get: { ${binding(n.bind.value)} }, set: { ${binding(n.bind.value)} = min($0, ${binding(n.bind.end)}) }), in: ${range}, step: ${p.step ?? 1})\nSlider(value: Binding(get: { ${binding(n.bind.end)} }, set: { ${binding(n.bind.end)} = max($0, ${binding(n.bind.value)}) }), in: ${range}, step: ${p.step ?? 1})`
          : `Slider(value: ${bound(n.bind?.value)}, in: ${range}, step: ${p.step ?? 1})`
        view = `HStack {\n${n.bind?.end ? `VStack {\n${sliders}\n}` : sliders}${label}\n}.disabled(${!!p.disabled})${p.label ? `.accessibilityLabel(${q(p.label)})` : ''}`
        break
      }
      case 'NumberField': {
        const key = n.bind?.value
        const text = key ? `$${binding(key)}` : `.constant(${q(p.value ?? '')})`
        const min = p.min ?? -1e9,
          max = p.max ?? 1e9
        const stepper = key
          ? `\nStepper("", value: Binding(get: { Double(${binding(key)}) ?? ${p.min ?? 0} }, set: { ${binding(key)} = mobileNumberText(min(max($0, ${min}), ${max})) }), in: ${min}...${max}, step: ${p.step ?? 1}).labelsHidden()`
          : ''
        view = `HStack {\nTextField(${q(p.placeholder ?? '')}, text: ${text}).keyboardType(.numberPad).textFieldStyle(.roundedBorder)${p.unit ? `\nText(${q(p.unit)}).foregroundStyle(.secondary)` : ''}${stepper}\n}.disabled(${!!p.disabled}).accessibilityHint(${prop('state', 'default')} == "error" ? "Invalid value" : "")`
        break
      }
      case 'SearchField': {
        const key = binding(n.bind.value)
        const dismiss =
          n.on?.dismiss?.set !== undefined
            ? `${binding(n.on.dismiss.set)} = ${q(n.on.dismiss.value)}`
            : ''
        view = `HStack {\nImage(systemName: "magnifyingglass").foregroundStyle(.secondary)\nTextField(${q(p.placeholder ?? 'Search')}, text: $${key}).textFieldStyle(.plain)\nif ${key}.isEmpty {\n${slot('hint')}\n} else {\nButton { ${key} = "" } label: { Image(systemName: "xmark.circle.fill") }.buttonStyle(.borderless).foregroundStyle(.secondary).accessibilityLabel(${q(p.clearLabel ?? 'Clear search')})\n}\n}.padding(8).background(.quaternary, in: RoundedRectangle(cornerRadius: 10)).modifier(MobileEscape { if ${key}.isEmpty { ${dismiss} } else { ${key} = "" } })`
        break
      }
      case 'OtpField': {
        const key = n.bind?.value
        const length = p.length ?? 6
        const text = key
          ? `Binding(get: { ${binding(key)} }, set: { ${binding(key)} = String($0.filter(\\.isNumber).prefix(${length})) })`
          : `.constant(${q(p.value ?? '')})`
        view = `${p.mask ? 'SecureField' : 'TextField'}(${q('•'.repeat(length))}, text: ${text}).keyboardType(.numberPad).textContentType(.oneTimeCode).font(.system(.title3, design: .monospaced)).textFieldStyle(.roundedBorder).disabled(${!!p.disabled}).accessibilityHint(${prop('state', 'default')} == "error" ? "Invalid value" : "")`
        break
      }
      case 'Switch':
      case 'Checkbox':
        view = `Toggle(isOn: ${n.bind?.checked ? '$' + binding(n.bind.checked) : `.constant(${p.checked ?? false})`}) { ${p.label ? `Text(${q(p.label)})` : children || 'Text(" ")'} }.disabled(${prop('disabled', false)})`
        break
      case 'TextField':
        view = p.readOnly
          ? `Text(${binding(n.bind.value)}).textSelection(.enabled)`
          : `TextField(${q(p.placeholder ?? '')}, text: $${binding(n.bind.value)}).textFieldStyle(.roundedBorder).disabled(${prop('disabled', false)})`
        if (p.mono) view += '.font(.system(.body, design: .monospaced))'
        view += `.accessibilityHint(${prop('state', 'default')} == "error" ? "Invalid value" : "")`
        break
      case 'Value':
        view = `Text(${valueType(n.value, scene.state ?? {}) === 'number' ? `mobileNumberText(Double(${expression(n.value)}))` : valueType(n.value, scene.state ?? {}) === 'boolean' ? `(${expression(n.value)} ? "true" : "false")` : expression(n.value)})`
        break
      case 'Text':
        view = children
        break
      case 'Icon':
        view = `Image(systemName: ${q(GLYPHS[p.name].symbol)})${p.size ? `.font(.system(size: ${typeof p.size === 'number' ? p.size : Number(p.size.split('.')[1]) * 4}))` : ''}`
        break
      case 'Spinner':
        view = 'ProgressView()'
        break
      case 'Progress':
        view =
          p.value !== undefined || n.derive?.value
            ? `ProgressView(value: Double(${prop('value', 0)}), total: 100)`
            : 'ProgressView()'
        break
      case 'Divider':
        view = 'Divider()'
        break
      case 'Card':
        view = `GroupBox { ${vertical} }`
        break
      case 'Callout':
        view = `GroupBox { VStack(alignment: .leading) {\n${slot('icon')}\n${p.title ? `Text(${q(p.title)}).font(.headline)` : ''}\n${children}\n${slot('action')} } }`
        break
      case 'EmptyState':
        view = `VStack { Text(${prop('title', '')}).font(.headline)\n${slot('action')} }`
        break
      case 'Step':
        view = `HStack { Image(systemName: ${prop('status', 'idle')} == "done" ? "checkmark.circle.fill" : ${prop('status', 'idle')} == "active" ? "circle.inset.filled" : "circle")\nText(${q(p.label)})\nSpacer()\nText(${prop('meta', '')}).foregroundStyle(.secondary) }.accessibilityValue(${prop('status', 'idle')})`
        break
      case 'Page':
      case 'Section':
        view = `VStack(alignment: .leading) { Text(${q(p.title)}).font(.title2)\n${p.description ? `Text(${q(p.description)}).foregroundStyle(.secondary)` : ''}\n${children} }`
        break
      case 'Specimen':
        view = `VStack(alignment: .leading) { Text(${q(p.label)}).font(.caption).foregroundStyle(.secondary)\n${children} }`
        break
      case 'PreferenceSection':
        view = `Section${p.label ? `(${q(p.label)})` : ''} {\n${children} }`
        break
      case 'PreferenceRow':
        view = `VStack(alignment: .leading) { Text(${q(p.title)})\n${p.subtitle ? `Text(${q(p.subtitle)}).foregroundStyle(.secondary)` : ''}\n${children} }`
        break
      case 'FormField':
        view = `VStack(alignment: .leading) { Text(${q(p.label)}).font(.headline)\n${children}\nText(${prop('hint', '')}).font(.caption).foregroundStyle(${prop('state', 'default')} == "error" ? palette.colorDanger[palette.nativeTintShade] : palette.colorContentMuted) }`
        break
      case 'Dialog':
        view = `Color.clear.sheet(isPresented: $${binding(n.bind.open)}) { ScrollView { ${vertical}.padding().accessibilityElement(children: .contain).accessibilityIdentifier("scene-dialog") } }`
        break
      case 'DialogHeader':
        view = `VStack(alignment: .leading) { Text(${q(p.title)}).font(.headline)\n${p.subtitle ? `Text(${q(p.subtitle)})` : ''} }`
        break
      case 'ActionBar':
        view = `HStack { ${['end', 'center'].includes(p.align) ? 'Spacer()' : ''}\n${children}\n${p.align === 'center' ? 'Spacer()' : ''} }`
        break
      case 'Grid':
        view = `LazyVGrid(columns: [GridItem(.adaptive(minimum: 160))], alignment: .leading) {\n${children} }`
        break
      case 'Table':
        view = `Grid(alignment: .leading) {\n${children} }`
        break
      // Keep GridRow directly inside Grid; a helper View would hide its cells.
      case 'TableHead':
        return children
      case 'TableRow':
        return `GridRow {\n${children} }`
      case 'TableCell':
        view = `${horizontal}.frame(maxWidth: .infinity, alignment: .${p.align === 'end' ? 'trailing' : p.align === 'center' ? 'center' : 'leading'})${p.head ? '.fontWeight(.semibold)' : ''}`
        break
      case 'Row':
        view = horizontal
        break
      case 'Inline':
        view = `HStack(spacing: 0) {\n${children} }${p.mono || p.fontSize ? `.font(.system(.${p.fontSize === 'title.large' ? 'title' : p.fontSize === 'body.small' ? 'caption' : 'body'}, design: .${p.mono ? 'monospaced' : 'default'}))` : ''}${p.tone ? `.foregroundStyle(palette.colorContent${p.tone[0].toUpperCase() + p.tone.slice(1)})` : ''}`
        break
      case 'KeyCap':
        view = `${horizontal}.font(.system(.caption, design: .monospaced))`
        break
      case 'Badge':
        view = `${horizontal}.font(.caption)`
        break
      case 'SectionLabel':
        view = `${horizontal}.font(.headline)`
        break
      case 'Box':
        view = p.display ? horizontal : vertical
        if (p.padding) view += '.padding()'
        if (p.surface)
          view += `.background(palette.${p.surface === 'color.surface.muted' ? 'colorSurfaceMuted' : 'colorSurfaceChrome'})`
        if (p.tone) view += '.foregroundStyle(palette.colorContentMuted)'
        break
      case 'Col':
      case 'Stack':
      case 'StepList':
      case 'DialogBody':
      case 'DialogFooter':
        view = vertical
        break
      default:
        throw Error(`iOS scene compiler: unsupported ${n.component}`)
    }
    if (p.tint)
      view += `.${['Badge', 'SectionLabel'].includes(n.component) ? 'foregroundStyle' : 'tint'}(palette.color${p.tint[0].toUpperCase() + p.tint.slice(1)}[palette.nativeTintShade])`
    if (
      n.id &&
      !['Button', 'Switch', 'Checkbox', 'TextField', 'Value', 'Text'].includes(
        n.component
      )
    )
      view += '.accessibilityElement(children: .contain)'
    methods.push(
      `private func ${method}() -> some View { ${view}${n.id ? `.accessibilityIdentifier(${q(n.id)})` : ''} }`
    )
    return `${method}()`
  }
  const content = node(scene.content)
  return `private struct MobileScene${index}: View {
@Environment(\\.mobilePalette) private var palette
${[...state.values()].map(s => `@State private var ${s.name}${typeof s.value === 'number' ? ': Double' : ''} = ${q(s.value)}`).join('\n')}
var body: some View { ${content}${(scene.timers ?? [])
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
${methods.join('\n')}
}`
}

export function iosCatalog(stories, scenes) {
  const portable = stories.filter(story => scenes.has(story.id))
  return `// Generated by tools/playgrounds. Edit the React example scene declarations.
import SwiftUI
import DazzMobileUI

private func mobileNumberText(_ value: Double) -> String {
let text = String(value)
return text.hasSuffix(".0") ? String(text.dropLast(2)) : text
}

/// A declared unavailable segment; the system can only express it from iOS 17.
private struct MobileSegmentAvailability: ViewModifier {
let disabled: Bool
func body(content: Content) -> some View {
if #available(iOS 17, *) { content.selectionDisabled(disabled) } else { content }
}
}

/// Escape from a hardware keyboard; the system only reports it from iOS 17.
private struct MobileEscape: ViewModifier {
let action: () -> Void
func body(content: Content) -> some View {
if #available(iOS 17, *) { content.onKeyPress(.escape) { action(); return .handled } } else { content }
}
}

/// A fraction of the offered width, for a length declared as a percentage.
private struct MobileFraction<Content: View>: View {
let fraction: CGFloat
@ViewBuilder let content: () -> Content
init(_ fraction: CGFloat, @ViewBuilder content: @escaping () -> Content) { self.fraction = fraction; self.content = content }
var body: some View { MobileFractionLayout(fraction: fraction) { content() } }
}
private struct MobileFractionLayout: Layout {
let fraction: CGFloat
func sizeThatFits(proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) -> CGSize {
let width = proposal.width ?? 0
let child = subviews.first?.sizeThatFits(ProposedViewSize(width: width * fraction, height: proposal.height)) ?? .zero
return CGSize(width: width, height: child.height)
}
func placeSubviews(in bounds: CGRect, proposal: ProposedViewSize, subviews: Subviews, cache: inout ()) {
subviews.first?.place(at: bounds.origin, proposal: ProposedViewSize(width: bounds.width * fraction, height: bounds.height))
}
}

struct MobileStory: Identifiable {
let id: String
let title: String
let name: String
let build: () -> AnyView
}
let mobileStories: [MobileStory] = [
${portable.map((s, i) => `MobileStory(id: ${q(s.id)}, title: ${q(s.title)}, name: ${q(s.name)}, build: { AnyView(MobileScene${i}()) }),`).join('\n')}
]
${portable.map((s, i) => iosScene(scenes.get(s.id), i)).join('\n\n')}
`
}
