import fs from 'node:fs/promises'
import path from 'node:path'
import ts from 'typescript'
import { StoryCompiler, Element, Live, Closure } from './stories.mjs'
import {
  dateFormulaType,
  dateRangeFormula,
  dateRangeKitSource,
} from './harmony-dates.mjs'

const unwrap = node =>
  ts.isParenthesizedExpression(node) ? unwrap(node.expression) : node

function metric(value) {
  const match = /^var\(--(spacing|stroke)-([a-z0-9-]+)\)$/.exec(value)
  if (!match) throw new Error(`Unsupported native header metric: ${value}`)
  return `${match[1]}.${match[2].replaceAll('-', '.')}`
}
function color(value) {
  const match = /^var\(--color-([a-z0-9-]+)\)$/.exec(value)
  if (!match) throw new Error(`Unsupported native header color: ${value}`)
  return match[1]
}

/**
 * A preset's two whole-day offsets from today, as the sealed formulas the
 * native interpreter evaluates on the device: `[0, 13]` is today through a
 * fortnight from it.
 */
function daysFormula(days, compiler, node) {
  if (days.length !== 2 || days.some(day => !Number.isInteger(day)))
    compiler.fail(node, 'A preset’s days are two whole-day offsets from today')
  const offset = day =>
    day === 0
      ? { op: 'today' }
      : {
          op: 'addDays',
          args: [{ op: 'today' }, { op: 'number', number: day }],
        }
  const range = { start: offset(days[0]), end: offset(days[1]) }
  if (
    dateFormulaType(range.start) !== 'date' ||
    dateFormulaType(range.end) !== 'date'
  )
    compiler.fail(node, 'A preset’s range must stay a date formula')
  return range
}

/** Only HarmonyOS opts into these React-only forms; portable grammar stays strict. */
export class HarmonyReactCompiler extends StoryCompiler {
  json(value, node) {
    if (
      typeof value?.label === 'string' &&
      value.range instanceof Closure &&
      Object.keys(value).every(key => ['label', 'range'].includes(key))
    )
      return {
        label: value.label,
        range: { $dateRange: dateRangeFormula(value.range, this) },
      }
    if (
      typeof value?.label === 'string' &&
      Array.isArray(value.days) &&
      Object.keys(value).every(key => ['label', 'days'].includes(key))
    )
      return {
        label: value.label,
        range: { $dateRange: daysFormula(value.days, this, node) },
      }
    return super.json(value, node)
  }
  stageSource() {
    return this.file.startsWith('stories/react/')
      ? '../../components/stage'
      : '../components/stage'
  }
  expression(node, scope) {
    if (ts.isNewExpression(node)) {
      if (!ts.isIdentifier(node.expression) || node.expression.text !== 'Date')
        this.fail(node, 'Only local Date construction is mapped')
      const args = (node.arguments ?? []).map(arg =>
        this.expression(arg, scope)
      )
      if (
        args.length < 3 ||
        args.length > 7 ||
        args.some(arg => !Number.isInteger(arg))
      )
        this.fail(node, 'Date requires three to seven static integer parts')
      if (
        !Number.isFinite(
          new Date(
            args[0],
            args[1],
            args[2],
            args[3] ?? 0,
            args[4] ?? 0,
            args[5] ?? 0,
            args[6] ?? 0
          ).getTime()
        )
      )
        this.fail(node, 'Invalid local Date parts')
      return { $date: args }
    }
    return super.expression(node, scope)
  }

  member(target, name, node) {
    if (target instanceof Live && ['start', 'end', 'allDay'].includes(name))
      return new Live({ $get: [target.json, name] })
    return super.member(target, name, node)
  }

  call(node, scope) {
    if (
      ts.isPropertyAccessExpression(node.expression) &&
      ['toLocaleDateString', 'toLocaleString'].includes(
        node.expression.name.text
      )
    ) {
      const target = this.expression(node.expression.expression, scope)
      if (
        node.arguments.length ||
        (!(target instanceof Live) && !target?.$date)
      )
        this.fail(node, 'Only parameterless date formatting is mapped')
      return new Live({
        $dateText: [
          this.json(target, node),
          node.expression.name.text === 'toLocaleDateString'
            ? 'date'
            : 'datetime',
        ],
      })
    }
    return super.call(node, scope)
  }

  jsx(node, scope) {
    if (ts.isJsxFragment(node)) return super.jsx(node, scope)
    const opening = ts.isJsxElement(node) ? node.openingElement : node
    if (!ts.isIdentifier(opening.tagName) || opening.tagName.text !== 'div')
      return super.jsx(node, scope)
    const attributes = opening.attributes.properties
    if (
      attributes.length !== 1 ||
      !ts.isJsxAttribute(attributes[0]) ||
      attributes[0].name.text !== 'style'
    )
      this.fail(node, 'Native headers require one static style')
    const expression = attributes[0].initializer?.expression
    if (!expression) this.fail(node, 'A native header needs its style')
    const style = this.expression(expression, scope)
    // WindowFrame's Pane is layout only. Keep the accepted CSS narrow so a
    // new source style cannot silently disappear in the native example.
    if (
      Object.keys(style).length === 2 &&
      style.padding === 'var(--spacing-5)' &&
      style.paddingBlockStart === 0
    )
      return new Element(
        'WindowPane',
        {},
        ts.isJsxElement(node) ? this.children(node.children, scope) : []
      )
    const keys = [
      'display',
      'width',
      'maxWidth',
      'height',
      'paddingInline',
      'borderBlockEnd',
      'background',
    ]
    if (
      Object.keys(style).some(key => !keys.includes(key)) ||
      style.display !== 'flex' ||
      style.maxWidth !== '100%' ||
      !Number.isFinite(style.width) ||
      style.width < 0 ||
      !Number.isFinite(style.height) ||
      style.height < 0
    )
      this.fail(node, 'Unsupported native header layout')
    const border =
      /^(var\(--stroke-[a-z0-9-]+\)) solid (var\(--color-[a-z0-9-]+\))$/.exec(
        style.borderBlockEnd
      )
    if (!border) this.fail(node, 'Unsupported native header border')
    let props
    try {
      props = {
        width: style.width,
        height: style.height,
        padding: metric(style.paddingInline),
        background: color(style.background),
        borderWidth: metric(border[1]),
        borderColor: color(border[2]),
      }
    } catch (error) {
      this.fail(node, error.message)
    }
    return new Element(
      'ChromeBar',
      props,
      ts.isJsxElement(node) ? this.children(node.children, scope) : []
    )
  }

  action(closure) {
    try {
      return super.action(closure)
    } catch (original) {
      // A functional array removal, not arbitrary callback execution:
      // closed => setItems(current => current.filter(item => item.value !== closed)).
      const fn = closure.node
      const call = unwrap(fn.body)
      if (
        fn.parameters.length !== 1 ||
        !ts.isIdentifier(fn.parameters[0].name) ||
        !ts.isCallExpression(call) ||
        !ts.isIdentifier(call.expression) ||
        call.arguments.length !== 1
      )
        throw original
      const update = unwrap(call.arguments[0])
      if (
        !ts.isArrowFunction(update) ||
        update.parameters.length !== 1 ||
        !ts.isIdentifier(update.parameters[0].name)
      )
        throw original
      const filter = unwrap(update.body)
      if (
        !ts.isCallExpression(filter) ||
        !ts.isPropertyAccessExpression(filter.expression) ||
        filter.expression.name.text !== 'filter' ||
        !ts.isIdentifier(filter.expression.expression) ||
        filter.expression.expression.text !== update.parameters[0].name.text ||
        filter.arguments.length !== 1
      )
        throw original
      const predicate = unwrap(filter.arguments[0])
      if (
        !ts.isArrowFunction(predicate) ||
        predicate.parameters.length !== 1 ||
        !ts.isIdentifier(predicate.parameters[0].name)
      )
        throw original
      const compare = unwrap(predicate.body)
      if (
        !ts.isBinaryExpression(compare) ||
        compare.operatorToken.kind !==
          ts.SyntaxKind.ExclamationEqualsEqualsToken ||
        !ts.isPropertyAccessExpression(compare.left) ||
        !ts.isIdentifier(compare.left.expression) ||
        compare.left.expression.text !== predicate.parameters[0].name.text ||
        !ts.isIdentifier(compare.right) ||
        compare.right.text !== fn.parameters[0].name.text
      )
        throw original
      const setter = this.read(call.expression.text, closure.scope, call)
      if (
        typeof setter?.slot !== 'string' ||
        !Array.isArray(this.scene?.state[setter.slot])
      )
        throw original
      return {
        $do: [
          {
            remove: setter.slot,
            key: compare.left.name.text,
            match: { $arg: 0 },
          },
        ],
      }
    }
  }
}

export async function readHarmonyReactStories(root) {
  const stories = path.join(root, 'packages/js/dazzui/example/src/stories')
  const dir = path.join(stories, 'react')
  const kitSource = await fs.readFile(
    path.join(root, 'packages/js/dazzui/src/components/date-range-picker.tsx'),
    'utf8'
  )
  const kit = dateRangeKitSource(
    kitSource,
    'components/date-range-picker.tsx',
    HarmonyReactCompiler
  )
  /*
   * Calendar and DateRangePicker are portable stories now that every
   * platform draws them; HarmonyOS still reads them with its own grammar,
   * because a date reaches the native interpreter as integer parts rather
   * than the portable wire format's local civil time. Tabs and WindowFrame
   * are React-only and sit in `react/`.
   */
  const mapped = [
    { file: 'calendar.tsx', dir: stories, source: 'stories/calendar.tsx' },
    {
      file: 'date-range-picker.tsx',
      dir: stories,
      source: 'stories/date-range-picker.tsx',
    },
    { file: 'tabs.tsx', dir, source: 'stories/react/tabs.tsx' },
    {
      file: 'window-frame.tsx',
      dir,
      source: 'stories/react/window-frame.tsx',
    },
  ]
  const compiled = []
  for (const { file, dir: from, source: filePath } of mapped) {
    const source = await fs.readFile(path.join(from, file), 'utf8')
    const story = new HarmonyReactCompiler(filePath, source, {
      kitValues: { DATE_RANGE_PRESETS: kit.presets },
    }).story()
    if (`${story.id}.tsx` !== file)
      throw new Error(`${file}: story id differs from its filename`)
    compiled.push(story)
  }
  // A `StoryExtension` adds React-only examples to a compiled story's page;
  // it is not a page of its own.
  const pages = []
  for (const file of (await fs.readdir(dir)).sort()) {
    if (!file.endsWith('.tsx')) continue
    const source = await fs.readFile(path.join(dir, file), 'utf8')
    if (!/\bStoryExtension\b/.test(source))
      pages.push({ id: file.replace(/\.tsx$/, ''), mapped: true })
  }
  // A portable story the native reader compiles is a native page too.
  for (const { file } of mapped)
    if (!pages.some(page => page.id === file.replace(/\.tsx$/, '')))
      pages.push({ id: file.replace(/\.tsx$/, ''), mapped: true })
  return {
    stories: compiled,
    pages: pages.sort((a, b) => a.id.localeCompare(b.id)),
  }
}
