import fs from 'node:fs/promises'
import { readFileSync } from 'node:fs'
import path from 'node:path'
import ts from 'typescript'
import { valueType } from './schema/value.mjs'
import { spacingPixels } from './tokens.mjs'
import { fileURLToPath } from 'node:url'
import {
  validateScene,
  SLOTS,
  COMPONENTS,
  ITEMS,
  UNCONTROLLED,
  EVENTS,
  SPACING_STEPS,
} from './schema/scene.mjs'

// A value that depends on scene state, carried through evaluation until it
// lands in a prop (derive or bind), a child (Value node) or a handler.
const EXPR = Symbol('scene expression')
const SETTER = Symbol('state setter')
const dynamic = value =>
  value !== null && typeof value === 'object' && EXPR in value
const lift = value => (dynamic(value) ? value[EXPR] : value)
/** React change handlers that pair with a controlled prop of the same name. */
const BIND_HANDLERS = {
  onValueChange: 'value',
  onCheckedChange: 'checked',
  onOpenChange: 'open',
  onPressedChange: 'pressed',
  onSelectedChange: 'selected',
  onExpandedChange: 'expanded',
}
/** A no-op change handler that still makes its control live. */
const NOOP_EVENTS = { onSelectedChange: 'select' }
/** Lengths: rem to logical pixels, spacing variables to spacing steps. */
function lengthOf(value) {
  if (typeof value !== 'string') return value
  const rem = /^(\d+(?:\.\d+)?)rem$/.exec(value)
  if (rem) return parseFloat(rem[1]) * 16
  const step = /^var\(--spacing-([\d-]+)\)$/.exec(value)
  if (step) return `spacing.${step[1]}`
  return value
}
/** A spacing variable or a rem length that lands exactly on a spacing step. */
function spacingToken(value) {
  const named = /^var\(--spacing-([\d-]+)\)$/.exec(value)
  if (named) return `spacing.${named[1]}`
  const rem = /^(\d+(?:\.\d+)?)rem$/.exec(value)
  const step =
    rem &&
    SPACING_STEPS.find(
      token => spacingPixels(token) === parseFloat(rem[1]) * 16
    )
  if (!step) throw Error(`Unsupported scene style: gap ${value}`)
  return step
}
/** Handlers that fire an assignment rather than mirror a value. */
const EVENT_HANDLERS = {
  onClick: 'press',
  onSelect: 'select',
  onRemove: 'remove',
  onPick: 'pick',
  onOpen: 'open',
  onDismiss: 'dismiss',
  onSubmit: 'submit',
}

/** Import a static story into data. Unsupported syntax is an error, never source embedded in JSON. */
export function extractStaticStories(
  source,
  filename,
  { exports: selected } = {}
) {
  const ast = ts.createSourceFile(
    filename,
    source,
    ts.ScriptTarget.Latest,
    true,
    ts.ScriptKind.TSX
  )
  const vars = new Map()
  const imports = new Map()
  const expanding = new Set()
  // The story being imported: its declared state and the keys already taken.
  let sceneState = null
  const typeOf = value => valueType(lift(value), sceneState ?? {})
  /** A condition on state: text and numbers test as JavaScript truthiness. */
  const truthy = value => {
    if (!dynamic(value)) return value
    const type = typeOf(value)
    if (type === 'boolean') return value
    return {
      [EXPR]: { not: { eq: [lift(value), type === 'string' ? '' : 0] } },
    }
  }
  const kebab = name => name.replace(/\B[A-Z]/g, c => `-${c}`).toLowerCase()
  /** Runs `useState` declarations and pure bindings, then the return. */
  function runBody(statements, env, label) {
    const scope = { ...env }
    for (const [i, statement] of statements.entries()) {
      if (i === statements.length - 1) {
        if (!ts.isReturnStatement(statement) || !statement.expression)
          throw Error(`${label}: body must end in a JSX return`)
        return evaluate(statement.expression, scope)
      }
      if (!ts.isVariableStatement(statement))
        throw Error(`${label}: template requires a single pure JSX return`)
      for (const d of statement.declarationList.declarations) {
        const init = d.initializer && unbox(d.initializer)
        if (
          ts.isArrayBindingPattern(d.name) &&
          init &&
          ts.isCallExpression(init) &&
          init.expression.getText(ast) === 'useState'
        ) {
          const [value, setter] = d.name.elements
          if (!value?.name || !ts.isIdentifier(value.name))
            throw Error(`${label}: unsupported useState binding`)
          if (!sceneState) throw Error(`${label}: state outside a story`)
          const initial = init.arguments[0]
            ? evaluate(init.arguments[0], scope)
            : undefined
          if (!['string', 'number', 'boolean'].includes(typeof initial))
            throw Error(`${label}: useState needs a scalar initial value`)
          const base = kebab(value.name.text)
          let key = base
          for (let n = 2; Object.hasOwn(sceneState, key); n++)
            key = `${base}-${n}`
          sceneState[key] = initial
          scope[value.name.text] = { [EXPR]: { state: key } }
          if (setter?.name && ts.isIdentifier(setter.name))
            scope[setter.name.text] = { [SETTER]: key }
        } else if (ts.isIdentifier(d.name) && init) {
          scope[d.name.text] = evaluate(init, scope)
        } else
          throw Error(`${label}: template requires a single pure JSX return`)
      }
    }
    throw Error(`${label}: empty body`)
  }
  /** A handler: nothing, a setter passed through, or a scalar assignment. */
  function handler(raw, env) {
    const n = unbox(raw)
    if (!ts.isArrowFunction(n)) {
      const value = evaluate(n, env)
      if (value && SETTER in value) return { setter: value[SETTER] }
      throw Error('Unsupported handler')
    }
    let body = n.body
    if (ts.isBlock(body)) {
      if (body.statements.length === 0) return { noop: true }
      if (
        body.statements.length !== 1 ||
        !ts.isExpressionStatement(body.statements[0])
      )
        throw Error('Unsupported handler body')
      body = body.statements[0].expression
    }
    body = unbox(body)
    if (!ts.isCallExpression(body) || body.arguments.length !== 1)
      throw Error('Unsupported handler body')
    const scope = { ...env }
    for (const p of n.parameters)
      if (ts.isIdentifier(p.name))
        scope[p.name.text] = { [EXPR]: { event: true } }
    const callee = evaluate(body.expression, scope)
    if (!callee || !(SETTER in callee))
      throw Error('Handler must call a state setter')
    const arg = unbox(body.arguments[0])
    if (
      n.parameters.length === 1 &&
      ts.isIdentifier(arg) &&
      ts.isIdentifier(n.parameters[0].name) &&
      arg.text === n.parameters[0].name.text
    )
      return { setter: callee[SETTER] }
    const value = lift(evaluate(arg, scope))
    return { set: callee[SETTER], value }
  }
  const unbox = n =>
    ts.isNonNullExpression(n) ||
    ts.isAsExpression(n) ||
    ts.isSatisfiesExpression(n) ||
    ts.isParenthesizedExpression(n)
      ? unbox(n.expression)
      : n
  for (const statement of ast.statements) {
    if (ts.isImportDeclaration(statement)) {
      for (const spec of statement.importClause?.namedBindings?.elements ?? [])
        imports.set(spec.name.text, {
          module: statement.moduleSpecifier.text,
          name: spec.propertyName?.text ?? spec.name.text,
        })
    }
    if (ts.isVariableStatement(statement))
      for (const d of statement.declarationList.declarations)
        vars.set(d.name.getText(ast), d.initializer)
  }
  function evaluate(raw, env = {}) {
    if (!raw) throw Error('Missing expression')
    const n = unbox(raw)
    if (ts.isStringLiteral(n) || ts.isNoSubstitutionTemplateLiteral(n))
      return n.text
    if (ts.isTemplateExpression(n)) {
      const parts = [n.head.text]
      for (const span of n.templateSpans) {
        const value = evaluate(span.expression, env)
        if (dynamic(value)) {
          const type = typeOf(value)
          if (type === 'boolean')
            throw Error(
              'Template substitutions of state must be text or numbers'
            )
          parts.push(
            type === 'number' ? { [EXPR]: { text: lift(value) } } : value
          )
        } else if (['string', 'number', 'boolean'].includes(typeof value))
          parts.push(String(value))
        else throw Error('Template substitutions must be scalar')
        parts.push(span.literal.text)
      }
      if (!parts.some(dynamic)) return parts.join('')
      const pieces = []
      for (const part of parts.filter(part => part !== ''))
        if (!dynamic(part) && typeof pieces.at(-1) === 'string')
          pieces[pieces.length - 1] += part
        else pieces.push(part)
      return { [EXPR]: { concat: pieces.map(lift) } }
    }
    if (ts.isPropertyAccessExpression(n) || ts.isElementAccessExpression(n)) {
      const object = evaluate(n.expression, env)
      if (
        dynamic(object) &&
        ts.isPropertyAccessExpression(n) &&
        n.name.text === 'length'
      )
        return { [EXPR]: { length: lift(object) } }
      const key = ts.isPropertyAccessExpression(n)
        ? n.name.text
        : evaluate(n.argumentExpression, env)
      if (object == null && n.questionDotToken) return undefined
      if (object == null || !Object.hasOwn(object, key))
        throw Error(`Unknown static property ${key}`)
      return object[key]
    }
    if (ts.isNumericLiteral(n)) return Number(n.text)
    if (n.kind === ts.SyntaxKind.NullKeyword) return null
    if (ts.isIdentifier(n) && n.text === 'undefined') return undefined
    if (ts.isConditionalExpression(n)) {
      const test = truthy(evaluate(n.condition, env))
      if (!dynamic(test)) return evaluate(test ? n.whenTrue : n.whenFalse, env)
      const then = evaluate(n.whenTrue, env),
        otherwise = evaluate(n.whenFalse, env)
      for (const branch of [then, otherwise])
        if (branch !== null && typeof branch === 'object' && !dynamic(branch))
          throw Error('Conditional content needs a scene node for each branch')
      return {
        // oxlint-disable-next-line unicorn/no-thenable -- scene branch data
        [EXPR]: { if: lift(test), then: lift(then), else: lift(otherwise) },
      }
    }
    if (
      ts.isBinaryExpression(n) &&
      [
        ts.SyntaxKind.EqualsEqualsEqualsToken,
        ts.SyntaxKind.ExclamationEqualsEqualsToken,
      ].includes(n.operatorToken.kind)
    ) {
      const a = evaluate(n.left, env),
        b = evaluate(n.right, env)
      const equal =
        n.operatorToken.kind === ts.SyntaxKind.EqualsEqualsEqualsToken
      if (!dynamic(a) && !dynamic(b)) return equal ? a === b : a !== b
      const eq = { eq: [lift(a), lift(b)] }
      return { [EXPR]: equal ? eq : { not: eq } }
    }
    if (
      ts.isBinaryExpression(n) &&
      n.operatorToken.kind === ts.SyntaxKind.AmpersandAmpersandToken
    ) {
      const test = truthy(evaluate(n.left, env))
      if (!dynamic(test)) return test ? evaluate(n.right, env) : test
      const right = evaluate(n.right, env)
      if (dynamic(right) || typeof right === 'boolean')
        return { [EXPR]: { and: [lift(test), lift(truthy(right))] } }
      const content = [right].flat(Infinity)
      return { component: 'Show', value: lift(test), children: content }
    }
    if (
      ts.isBinaryExpression(n) &&
      n.operatorToken.kind === ts.SyntaxKind.BarBarToken
    ) {
      const left = truthy(evaluate(n.left, env))
      const right = truthy(evaluate(n.right, env))
      if (!dynamic(left) && !dynamic(right)) return left || right
      if (left === true || right === true) return true
      return {
        [EXPR]: { or: [left, right].filter(v => v !== false).map(lift) },
      }
    }
    const comparisons = {
      [ts.SyntaxKind.GreaterThanEqualsToken]: (a, b) => ({ gte: [a, b] }),
      [ts.SyntaxKind.LessThanEqualsToken]: (a, b) => ({ gte: [b, a] }),
      [ts.SyntaxKind.LessThanToken]: (a, b) => ({ not: { gte: [a, b] } }),
      [ts.SyntaxKind.GreaterThanToken]: (a, b) => ({ not: { gte: [b, a] } }),
    }
    if (ts.isBinaryExpression(n) && comparisons[n.operatorToken.kind]) {
      const a = evaluate(n.left, env),
        b = evaluate(n.right, env)
      if (!dynamic(a) && !dynamic(b))
        return new Function(
          'a',
          'b',
          `return a ${n.operatorToken.getText(ast)} b`
        )(a, b)
      return { [EXPR]: comparisons[n.operatorToken.kind](lift(a), lift(b)) }
    }
    if (
      ts.isPrefixUnaryExpression(n) &&
      n.operator === ts.SyntaxKind.ExclamationToken
    ) {
      const value = truthy(evaluate(n.operand, env))
      return dynamic(value) ? { [EXPR]: { not: lift(value) } } : !value
    }
    if (n.kind === ts.SyntaxKind.TrueKeyword) return true
    if (n.kind === ts.SyntaxKind.FalseKeyword) return false
    if (ts.isIdentifier(n)) {
      if (Object.hasOwn(env, n.text)) return env[n.text]
      if (imports.get(n.text)?.module === '@fluentui/react-icons')
        return imports.get(n.text).name
      if (
        n.text === 'TINTS' &&
        imports.get(n.text)?.module === '@dazzlabs/dazzui-react'
      ) {
        const file = new URL(
          '../../packages/js/dazzui-react/src/lib/tint.ts',
          import.meta.url
        )
        const tintAst = ts.createSourceFile(
          'tint.ts',
          readFileSync(file, 'utf8'),
          ts.ScriptTarget.Latest,
          true
        )
        const declaration = tintAst.statements
          .flatMap(s => s.declarationList?.declarations ?? [])
          .find(d => d.name.text === 'TINTS')
        return evaluate(declaration.initializer, env)
      }
      if (vars.has(n.text)) return evaluate(vars.get(n.text), env)
    }
    if (ts.isObjectLiteralExpression(n)) {
      return Object.fromEntries(
        n.properties.flatMap(prop => {
          if (ts.isSpreadAssignment(prop))
            return Object.entries(evaluate(prop.expression, env))
          if (!ts.isPropertyAssignment(prop))
            throw Error('Only literal style properties can be imported')
          return [[prop.name.text, evaluate(prop.initializer, env)]]
        })
      )
    }
    if (ts.isArrayLiteralExpression(n))
      return n.elements.flatMap(x =>
        ts.isSpreadElement(x) ? evaluate(x.expression, env) : [evaluate(x, env)]
      )
    if (ts.isCallExpression(n) && ts.isPropertyAccessExpression(n.expression)) {
      const target = evaluate(n.expression.expression, env)
      const method = n.expression.name.text
      if (method === 'slice' && Array.isArray(target))
        return target.slice(...n.arguments.map(arg => evaluate(arg, env)))
      if (
        method === 'toLowerCase' &&
        (dynamic(target) || typeof target === 'string')
      )
        return dynamic(target)
          ? { [EXPR]: { lower: lift(target) } }
          : target.toLowerCase()
      if (
        method === 'includes' &&
        (dynamic(target) || typeof target === 'string')
      ) {
        const needle = evaluate(n.arguments[0], env)
        if (!dynamic(target) && !dynamic(needle)) return target.includes(needle)
        return { [EXPR]: { contains: [lift(target), lift(needle)] } }
      }
    }
    if (ts.isCallExpression(n)) {
      if (
        n.expression.getText(ast) === 'label' &&
        imports.get('label')?.module.endsWith('/label')
      ) {
        const value = evaluate(n.arguments[0], env)
        return value[0].toUpperCase() + value.slice(1)
      }
      if (
        ts.isPropertyAccessExpression(n.expression) &&
        n.expression.name.text === 'map'
      ) {
        const values = evaluate(n.expression.expression, env)
        const fn = n.arguments[0]
        if (
          !Array.isArray(values) ||
          !ts.isArrowFunction(fn) ||
          fn.parameters.some(p => !ts.isIdentifier(p.name))
        )
          throw Error('Unsupported map')
        return values.map((value, i) =>
          evaluate(fn.body, {
            ...env,
            [fn.parameters[0].name.text]: value,
            ...(fn.parameters[1] ? { [fn.parameters[1].name.text]: i } : {}),
          })
        )
      }
    }
    if (ts.isJsxElement(n) || ts.isJsxSelfClosingElement(n)) {
      const opening = ts.isJsxElement(n) ? n.openingElement : n
      const tag = opening.tagName.getText(ast)
      // A bare Fluent component draws at its own grid size, as Icon would.
      if (imports.get(tag)?.module === '@fluentui/react-icons') {
        const name = imports.get(tag).name
        return { component: 'Icon', props: { name, fit: true } }
      }
      let component = tag === 'span' ? 'Inline' : tag === 'div' ? 'Box' : tag
      const props = {},
        slots = {},
        handlers = {}
      let itemChildren = null
      for (const attr of opening.attributes.properties) {
        if (!ts.isJsxAttribute(attr))
          throw Error('Spread attributes need an explicit scene mapping')
        const name = attr.name.getText(ast)
        if (name === 'key') continue
        const initializer =
          attr.initializer && ts.isJsxExpression(attr.initializer)
            ? attr.initializer.expression
            : undefined
        // `values => \`${values[0]}px\`` prints the reading with a unit suffix.
        if (
          name === 'formatValue' &&
          initializer &&
          ts.isArrowFunction(unbox(initializer)) &&
          ts.isTemplateExpression(unbox(unbox(initializer).body)) &&
          unbox(unbox(initializer).body).head.text === '' &&
          unbox(unbox(initializer).body).templateSpans.length === 1 &&
          unbox(unbox(initializer).body).templateSpans[0].expression.getText(
            ast
          ) === `${unbox(initializer).parameters[0].name.getText(ast)}[0]`
        ) {
          props.unit = unbox(
            unbox(initializer).body
          ).templateSpans[0].literal.text
          continue
        }
        if (
          /^on[A-Z]/.test(name) &&
          initializer &&
          (ts.isArrowFunction(unbox(initializer)) ||
            ts.isIdentifier(unbox(initializer)))
        ) {
          handlers[name] = handler(initializer, env)
          continue
        }
        const value = !attr.initializer
          ? true
          : ts.isJsxExpression(attr.initializer)
            ? evaluate(attr.initializer.expression, env)
            : evaluate(attr.initializer, env)
        if (value === undefined) continue
        if (
          component === 'Icon' &&
          name === 'size' &&
          typeof value === 'string'
        ) {
          const step = /^var\(--spacing-(\d+)\)$/.exec(value)
          if (!step) throw Error(`Unsupported icon size ${value}`)
          props.size = `spacing.${step[1]}`
          continue
        }
        if (name === 'aria-label') {
          props.label = value
          continue
        }
        if (
          ITEMS[component] &&
          ['items', 'options'].includes(name) &&
          Array.isArray(value)
        ) {
          itemChildren = value.map(item => {
            const props = Object.fromEntries(
              Object.entries(item).filter(([, v]) => v !== undefined)
            )
            // A node label becomes the item's content; `hint` names it.
            const children =
              props.label !== null && typeof props.label === 'object'
                ? [props.label]
                : []
            if (children.length) {
              if (props.hint) props.label = props.hint
              else delete props.label
            }
            return {
              component: ITEMS[component],
              props,
              ...(children.length ? { children } : {}),
            }
          })
          continue
        }
        if (
          (SLOTS[component] ?? []).includes(name) &&
          !(typeof value === 'string' && COMPONENTS[component]?.includes(name))
        ) {
          slots[name] = value
          continue
        }
        if (name === 'aria-disabled') {
          props.disabled = value === true || value === 'true'
          continue
        }
        if (
          ['width', 'height', 'maxWidth'].includes(name) ||
          (component === 'Grid' && name === 'min')
        )
          props[name] = lengthOf(value)
        else
          props[
            name === 'fullWidth' ? 'expand' : name === 'icon' ? 'name' : name
          ] = value
      }
      if (
        (component === 'Row' || component === 'Col') &&
        typeof props.gap === 'string'
      )
        props.gap = props.gap.replace(
          /^var\(--spacing-([\d-]+)\)$/,
          'spacing.$1'
        )
      const children = ts.isJsxElement(n)
        ? n.children.flatMap(child => {
            if (ts.isJsxText(child)) {
              const lines = child.text.replace(/\r/g, '').split('\n')
              const last = lines.findLastIndex(line => /[^\t ]/.test(line))
              const text = lines
                .map((line, i) => {
                  let s = line.replace(/\t/g, ' ')
                  if (i !== 0) s = s.replace(/^ +/, '')
                  if (i !== lines.length - 1) s = s.replace(/ +$/, '')
                  return s ? s + (i !== last ? ' ' : '') : ''
                })
                .join('')
              return text ? [text] : []
            }
            if (ts.isJsxExpression(child))
              return child.expression
                ? [evaluate(child.expression, env)]
                    .flat(Infinity)
                    .filter(value => value !== null && value !== false)
                    .map(value =>
                      dynamic(value)
                        ? { component: 'Value', value: value[EXPR] }
                        : value
                    )
                : []
            return [evaluate(child, env)]
          })
        : []
      if (!Object.hasOwn(COMPONENTS, component)) {
        const template = ast.statements.find(
          statement =>
            ts.isFunctionDeclaration(statement) && statement.name?.text === tag
        )
        if (template) {
          const parameter = template.parameters[0]
          const body = template.body?.statements
          if (
            template.parameters.length > 1 ||
            (parameter && !ts.isObjectBindingPattern(parameter.name)) ||
            !body?.length
          )
            throw Error(`${tag}: template requires a single pure JSX return`)
          if (expanding.has(tag)) throw Error(`${tag}: recursive template`)
          const args = { ...props, ...(children.length ? { children } : {}) }
          const scope = { ...env }
          const consumed = new Set()
          for (const field of parameter?.name.elements ?? []) {
            if (field.dotDotDotToken || !ts.isIdentifier(field.name))
              throw Error(`${tag}: unsupported template binding`)
            const name = field.propertyName?.getText(ast) ?? field.name.text
            consumed.add(name)
            scope[field.name.text] = Object.hasOwn(args, name)
              ? args[name]
              : field.initializer
                ? evaluate(field.initializer, env)
                : undefined
          }
          if (Object.keys(args).some(key => !consumed.has(key)))
            throw Error(`${tag}: unconsumed template prop`)
          expanding.add(tag)
          try {
            return runBody(body, scope, tag)
          } finally {
            expanding.delete(tag)
          }
        }
      }
      const box = {}
      for (const [property, value] of Object.entries(props.style ?? {})) {
        if (
          ['width', 'height', 'maxWidth'].includes(property) &&
          typeof value === 'string' &&
          (/^\d+(\.\d+)?rem$/.test(value) ||
            /^var\(--spacing-[\d-]+\)$/.test(value))
        )
          box[property] = lengthOf(value)
        else if (
          ['width', 'height', 'maxWidth'].includes(property) &&
          typeof value === 'number'
        )
          box[property] = value
        else {
          const mapping = {
            flexWrap: { wrap: ['wrap', true] },
            flexDirection: { row: ['direction', 'row'] },
            flex: { 1: ['grow', true] },
            display: {
              'inline-flex': ['display', 'inline-flex'],
              flex: ['display', 'flex'],
            },
            justifyContent: { 'space-between': ['justify', 'between'] },
            gap:
              property === 'gap'
                ? { [value]: ['gap', spacingToken(value)] }
                : {},
            alignItems: { center: ['align', 'center'] },
            color: {
              'var(--color-content-muted)': ['tone', 'muted'],
              'var(--color-content-subtle)': ['tone', 'subtle'],
            },
            fontSize: {
              'var(--title-large-font-size)': ['fontSize', 'title.large'],
              'var(--body-small-font-size)': ['fontSize', 'body.small'],
            },
            fontFamily: { 'var(--base-font-code)': ['mono', true] },
            padding: {
              'var(--spacing-2)': ['padding', 'spacing.2'],
              'var(--spacing-3)': ['padding', 'spacing.3'],
            },
            background: {
              'var(--color-surface-chrome)': [
                'surface',
                'color.surface.chrome',
              ],
              'var(--color-surface-muted)': ['surface', 'color.surface.muted'],
            },
            borderRadius: {
              'var(--radius-medium)': ['radius', 'radius.medium'],
              'var(--radius-large)': ['radius', 'radius.large'],
            },
          }[property]?.[value]
          if (!mapping) throw Error(`Unsupported scene style: ${property}`)
          box[mapping[0]] = mapping[1]
        }
      }
      if (
        component === 'Inline' &&
        ['inline-flex', 'flex'].includes(box.display)
      )
        component = 'Box'
      if (
        component === 'Inline' &&
        Object.keys(box).every(key =>
          ['tone', 'fontSize', 'mono'].includes(key)
        )
      ) {
        Object.assign(props, box)
        for (const key of Object.keys(box)) delete box[key]
      }
      delete props.style
      if (component === 'Box') Object.assign(props, box)
      // A radio group lays out its own options; the row and its gap are its own.
      if (component === 'RadioGroup')
        for (const key of ['direction', 'gap'])
          if (Object.hasOwn(box, key)) {
            props[key] = box[key]
            delete box[key]
          }
      if (itemChildren) children.push(...itemChildren)
      if (component === 'ToggleGroup' && Array.isArray(props.value)) {
        for (const item of children)
          if (props.value.includes(item.props?.value)) item.props.pressed = true
        delete props.value
      }
      const bind = {},
        on = {},
        derive = {}
      for (const [name, entry] of Object.entries(handlers)) {
        if (entry.noop) {
          const event = EVENT_HANDLERS[name] ?? NOOP_EVENTS[name]
          if (
            event &&
            (EVENTS[component] ?? []).includes(event) &&
            component !== 'Button'
          )
            on[event] = {}
          continue
        }
        if (entry.setter !== undefined) {
          const prop = BIND_HANDLERS[name]
          const bound = prop && props[prop]
          if (!dynamic(bound) || bound[EXPR].state !== entry.setter)
            throw Error(
              `${tag}: ${name} must pair with a controlled ${prop ?? 'prop'}`
            )
          bind[prop] = entry.setter
          delete props[prop]
          continue
        }
        const event = EVENT_HANDLERS[name]
        if (!event) throw Error(`${tag}: unsupported handler ${name}`)
        on[event] = { set: entry.set, value: entry.value }
      }
      const controlled = UNCONTROLLED[component]
      if (controlled && !bind[controlled]) {
        const fallback = `default${controlled[0].toUpperCase()}${controlled.slice(1)}`
        if (!Object.hasOwn(props, controlled)) {
          if (!sceneState)
            throw Error(`${tag}: uncontrolled state outside a story`)
          const base = kebab(component)
          let key = base
          for (let n = 2; Object.hasOwn(sceneState, key); n++)
            key = `${base}-${n}`
          const initial = props[fallback]
          const empty =
            { Toggle: false, Checkbox: false, Switch: false, Slider: 0 }[
              component
            ] ?? ''
          if (component === 'Slider' && Array.isArray(initial)) {
            sceneState[key] = initial[0]
            sceneState[`${key}-end`] = initial[1]
            bind.end = `${key}-end`
          } else
            sceneState[key] =
              component === 'NumberField' && typeof initial === 'number'
                ? String(initial)
                : (initial ?? empty)
          bind[controlled] = key
        }
        delete props[fallback]
      }
      for (const [name, value] of Object.entries(props))
        if (dynamic(value)) {
          derive[name] = value[EXPR]
          delete props[name]
        }
      const result = {
        component,
        ...(Object.keys(props).length ? { props } : {}),
        ...(Object.keys(derive).length ? { derive } : {}),
        ...(Object.keys(bind).length ? { bind } : {}),
        ...(Object.keys(on).length ? { on } : {}),
        ...(children.length ? { children } : {}),
        ...(Object.keys(slots).length ? { slots } : {}),
      }
      return component !== 'Box' && Object.keys(box).length
        ? { component: 'Box', props: box, children: [result] }
        : result
    }
    throw Error(`Unsupported static syntax: ${n.getText(ast).slice(0, 100)}`)
  }
  const scenes = {},
    replacements = []
  for (const statement of ast.statements) {
    if (
      !ts.isVariableStatement(statement) ||
      !statement.modifiers?.some(m => m.kind === ts.SyntaxKind.ExportKeyword)
    )
      continue
    for (const d of statement.declarationList.declarations) {
      if (selected && !selected.includes(d.name.text)) continue
      const value = unbox(d.initializer)
      if (
        ts.isCallExpression(value) &&
        value.expression.getText(ast) === 'defineStory'
      )
        continue
      const render = value.properties?.find(
        p => p.name?.getText(ast) === 'render'
      )?.initializer
      if (!render || !ts.isArrowFunction(render))
        throw Error(
          `${d.name.text}: imperative render requires manual migration`
        )
      // Native default values become explicit initial state, retaining editing
      // behavior without copying an uncontrolled fixture into every adapter.
      const state = {}
      sceneState = state
      const scene = {
        content: ts.isBlock(render.body)
          ? runBody(render.body.statements, {}, d.name.text)
          : evaluate(render.body),
      }
      sceneState = null
      const liftFields = (node, key) => {
        if (typeof node === 'string') return
        if (node.component === 'TextField') {
          const initial = node.props?.defaultValue ?? ''
          if (!['string', 'number'].includes(typeof initial))
            throw Error('Invalid default text value')
          node.id = key
          node.bind = { value: key }
          state[key] = String(initial)
          if (node.props) delete node.props.defaultValue
        }
        for (const [name, slot] of Object.entries(node.slots ?? {}))
          liftFields(slot, `${key}-slot-${name}`)
        ;(node.children ?? []).forEach((child, i) =>
          liftFields(child, `${key}-${i}`)
        )
      }
      liftFields(scene.content, 'field')
      if (Object.keys(state).length) scene.state = state
      validateScene(scene, d.name.text)
      scenes[d.name.text] = scene
      const options = value.properties
        .filter(p => p.name?.getText(ast) !== 'render')
        .map(p => p.getText(ast))
        .join(', ')
      replacements.push({
        start: d.initializer.getStart(ast),
        end: d.initializer.end,
        text: `defineStory(scenes.${d.name.text}${options ? `, {${options}}` : ''})`,
      })
    }
  }
  return { scenes, replacements }
}

/**
 * Removes imports and module-private declarations that nothing references once
 * stories have become data, repeating until nothing more falls away.
 */
export function pruneUnused(source, filename) {
  for (;;) {
    const ast = ts.createSourceFile(
      filename,
      source,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TSX
    )
    const used = new Map()
    const count = node => {
      if (ts.isIdentifier(node))
        used.set(node.text, (used.get(node.text) ?? 0) + 1)
      ts.forEachChild(node, count)
    }
    for (const statement of ast.statements)
      if (!ts.isImportDeclaration(statement)) count(statement)
    const exported = s =>
      s.modifiers?.some(m => m.kind === ts.SyntaxKind.ExportKeyword)
    const edits = []
    for (const statement of ast.statements) {
      if (ts.isImportDeclaration(statement)) {
        const clause = statement.importClause
        if (!clause || statement.moduleSpecifier.text.endsWith('.css')) continue
        const names = [
          ...(clause.name ? [clause.name] : []),
          ...(clause.namedBindings && ts.isNamedImports(clause.namedBindings)
            ? clause.namedBindings.elements.map(e => e.name)
            : clause.namedBindings
              ? [clause.namedBindings.name]
              : []),
        ]
        const kept = names.filter(name => used.has(name.text))
        if (kept.length === names.length) continue
        if (!kept.length) {
          edits.push([statement.getFullStart(), statement.end, ''])
          continue
        }
        const named =
          clause.namedBindings && ts.isNamedImports(clause.namedBindings)
            ? clause.namedBindings.elements.filter(e => used.has(e.name.text))
            : []
        const typeOnly = clause.isTypeOnly ? 'type ' : ''
        const def =
          clause.name && used.has(clause.name.text) ? clause.name.text : ''
        const parts = [
          def,
          named.length
            ? `{ ${named.map(e => e.getText(ast)).join(', ')} }`
            : '',
        ].filter(Boolean)
        edits.push([
          statement.getStart(ast),
          statement.end,
          `import ${typeOnly}${parts.join(', ')} from ${statement.moduleSpecifier.getText(ast)}`,
        ])
      } else if (
        ts.isFunctionDeclaration(statement) &&
        !exported(statement) &&
        statement.name &&
        (used.get(statement.name.text) ?? 0) <= 1
      ) {
        edits.push([statement.getFullStart(), statement.end, ''])
      } else if (
        ts.isVariableStatement(statement) &&
        !exported(statement) &&
        statement.declarationList.declarations.every(
          d => ts.isIdentifier(d.name) && (used.get(d.name.text) ?? 0) <= 1
        )
      ) {
        edits.push([statement.getFullStart(), statement.end, ''])
      }
    }
    if (!edits.length) return source
    for (const [start, end, text] of edits.toReversed())
      source = source.slice(0, start) + text + source.slice(end)
  }
}

if (
  process.argv[1] &&
  path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)
) {
  const file = path.resolve(process.argv[2])
  const source = await fs.readFile(file, 'utf8')
  const { scenes, replacements } = extractStaticStories(
    source,
    path.basename(file),
    {
      exports: process.argv
        .find(arg => arg.startsWith('--stories='))
        ?.slice('--stories='.length)
        .split(','),
    }
  )
  const sceneFile = file.replace(/\.stories\.tsx$/, '.scene.json')
  if (sceneFile === file) throw Error('Expected .stories.tsx')
  const previous = JSON.parse(
    await fs.readFile(sceneFile, 'utf8').catch(error => {
      if (error.code !== 'ENOENT') throw error
      return '{}'
    })
  )
  let output = source
  for (const item of replacements.toReversed())
    output = output.slice(0, item.start) + item.text + output.slice(item.end)
  if (!source.includes('import { defineStory }'))
    output =
      `import { defineStory } from '../src/lib/scene'\nimport scenes from './${path.basename(sceneFile)}'\n` +
      output
  output = pruneUnused(output, path.basename(file))
  await fs.writeFile(
    sceneFile,
    JSON.stringify({ ...previous, ...scenes }, null, 2) + '\n'
  )
  await fs.writeFile(file, output)
  console.log(
    `Imported ${Object.keys(scenes).length} stories as data. Remove unused TSX imports, regenerate and verify screenshots.`
  )
}
