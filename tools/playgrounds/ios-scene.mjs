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
      ...(n.children ?? []).map(node),
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
        view = `Image(systemName: ${q(p.name === 'Circle20Filled' ? 'circle.fill' : 'arrow.down.to.line')})`
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
        view = `VStack { Text(${q(p.title)}).font(.headline)\n${slot('action')} }`
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
        view = `HStack(spacing: 0) {\n${children} }${p.mono || p.fontSize ? `.font(.system(.${p.fontSize === 'title.large' ? 'title' : p.fontSize === 'body.small' ? 'caption' : 'body'}, design: .${p.mono ? 'monospaced' : 'default'}))` : ''}${p.tone ? '.foregroundStyle(.secondary)' : ''}`
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
