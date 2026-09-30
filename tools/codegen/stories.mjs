import fs from 'node:fs/promises'
import path from 'node:path'
import ts from 'typescript'

/*
 * Reads the React example's stories and compiles each into data every
 * platform can draw.
 *
 * A story is ordinary TSX, and this does not run it: it evaluates it. Module
 * constants, `.map` over a constant list, a helper component called with
 * props — all of that is worked out here, ahead of time, so what comes out is
 * the finished tree the React page would render, with the handful of things
 * that stay live left as references: a `useState` becomes a slot, a setter
 * handed to an event becomes an action that writes it, a comparison against
 * a slot becomes an expression.
 *
 * Anything this cannot work out ahead of time is an error naming the line —
 * never a silent gap on one platform.
 */

/** The only modules a story may take values from. */
const KIT = '@dazzlabs/dazzui'
const ICONS = '@fluentui/react-icons'
const STAGE = '../components/stage'
const REACT = 'react'
const STAGE_COMPONENTS = new Set(['Row', 'Col', 'Frame', 'Text'])

class StoryError extends Error {}

/* The values evaluation produces beside plain JavaScript ones. */
class Element {
  constructor(component, props, children) {
    this.component = component
    this.props = props
    this.children = children
  }
}
class Component {
  constructor(name, stage) {
    this.name = name
    this.stage = stage
  }
}
class Glyph {
  constructor(name) {
    this.name = name
  }
}
class Closure {
  constructor(node, scope, name) {
    this.node = node
    this.scope = scope
    this.name = name
  }
}
/** Something only known when the scene runs: a slot, an argument, a test. */
class Live {
  constructor(json) {
    this.json = json
  }
}
class Setter {
  constructor(slot) {
    this.slot = slot
  }
}
class Toaster {}
class Hook {
  constructor(name) {
    this.name = name
  }
}

const live = value => value instanceof Live

class Scope {
  constructor(parent) {
    this.parent = parent
    this.values = new Map()
  }
  lookup(name) {
    for (let scope = this; scope; scope = scope.parent)
      if (scope.values.has(name)) return scope.values.get(name)
    return undefined
  }
  has(name) {
    for (let scope = this; scope; scope = scope.parent)
      if (scope.values.has(name)) return true
    return false
  }
}

/** A module constant, evaluated the first time something reads it. */
class Lazy {
  constructor(evaluate) {
    this.evaluate = evaluate
    this.state = 'pending'
  }
}

export class StoryCompiler {
  constructor(file, source, context) {
    this.file = file
    this.source = ts.createSourceFile(
      file,
      source,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TSX
    )
    this.context = context
    this.scene = null
  }

  fail(node, message) {
    const { line, character } = this.source.getLineAndCharacterOfPosition(
      node.getStart(this.source)
    )
    throw new StoryError(
      `${this.file}:${line + 1}:${character + 1}: ${message}`
    )
  }

  /* Module ------------------------------------------------------------- */

  module() {
    const scope = new Scope()
    let exported
    for (const statement of this.source.statements) {
      if (ts.isImportDeclaration(statement)) this.imports(statement, scope)
      else if (ts.isVariableStatement(statement))
        for (const declaration of statement.declarationList.declarations) {
          if (!ts.isIdentifier(declaration.name))
            this.fail(declaration, 'Destructure inside a function, not here')
          const init = declaration.initializer
          if (!init) this.fail(declaration, 'A module constant needs a value')
          scope.values.set(
            declaration.name.text,
            new Lazy(() => this.expression(init, scope))
          )
        }
      else if (ts.isFunctionDeclaration(statement)) {
        if (!statement.name) this.fail(statement, 'Name the function')
        scope.values.set(
          statement.name.text,
          new Closure(statement, scope, statement.name.text)
        )
      } else if (ts.isExportAssignment(statement))
        exported = this.expression(statement.expression, scope)
      else if (
        ts.isInterfaceDeclaration(statement) ||
        ts.isTypeAliasDeclaration(statement)
      )
        continue
      else
        this.fail(
          statement,
          `Unsupported statement: ${ts.SyntaxKind[statement.kind]}`
        )
    }
    return { scope, exported }
  }

  imports(statement, scope) {
    const from = statement.moduleSpecifier.text
    const clause = statement.importClause
    if (!clause || clause.isTypeOnly) return
    if (clause.name)
      this.fail(clause, `Import by name, not by default, from ${from}`)
    for (const element of clause.namedBindings?.elements ?? []) {
      if (element.isTypeOnly) continue
      const imported = (element.propertyName ?? element.name).text
      const local = element.name.text
      let value
      if (from === KIT && imported === 'useToast') value = new Hook(imported)
      else if (from === KIT)
        value =
          imported in this.context.kitValues
            ? this.context.kitValues[imported]
            : /^[A-Z]/.test(imported)
              ? new Component(imported, false)
              : this.fail(element, `${imported} is not a component`)
      else if (from === ICONS) value = new Glyph(imported)
      else if (from === STAGE)
        value = STAGE_COMPONENTS.has(imported)
          ? new Component(imported, true)
          : this.fail(element, `${imported} is not part of the stage`)
      else if (from === REACT)
        value =
          imported === 'useState'
            ? new Hook(imported)
            : this.fail(element, `${imported} cannot be compiled`)
      else this.fail(statement, `Stories may not import from ${from}`)
      scope.values.set(local, value)
    }
  }

  /* Expressions -------------------------------------------------------- */

  read(name, scope, node) {
    if (!scope.has(name)) {
      if (name === 'undefined') return undefined
      this.fail(node, `Unknown name: ${name}`)
    }
    const value = scope.lookup(name)
    if (!(value instanceof Lazy)) return value
    if (value.state === 'busy') this.fail(node, `${name} refers to itself`)
    if (value.state === 'pending') {
      value.state = 'busy'
      value.value = value.evaluate()
      value.state = 'done'
    }
    return value.value
  }

  expression(node, scope) {
    switch (node.kind) {
      case ts.SyntaxKind.StringLiteral:
      case ts.SyntaxKind.NoSubstitutionTemplateLiteral:
        return node.text
      case ts.SyntaxKind.NumericLiteral:
        return Number(node.text)
      case ts.SyntaxKind.TrueKeyword:
        return true
      case ts.SyntaxKind.FalseKeyword:
        return false
      case ts.SyntaxKind.NullKeyword:
        return null
      case ts.SyntaxKind.Identifier:
        return this.read(node.text, scope, node)
      case ts.SyntaxKind.ParenthesizedExpression:
      case ts.SyntaxKind.AsExpression:
      case ts.SyntaxKind.SatisfiesExpression:
      case ts.SyntaxKind.NonNullExpression:
      case ts.SyntaxKind.TypeAssertionExpression:
        return this.expression(node.expression, scope)
      case ts.SyntaxKind.TemplateExpression:
        return this.template(node, scope)
      case ts.SyntaxKind.ArrayLiteralExpression:
        return node.elements.flatMap(element =>
          ts.isSpreadElement(element)
            ? this.array(this.expression(element.expression, scope), element)
            : [this.expression(element, scope)]
        )
      case ts.SyntaxKind.ObjectLiteralExpression:
        return this.object(node, scope)
      case ts.SyntaxKind.ArrowFunction:
      case ts.SyntaxKind.FunctionExpression:
        return new Closure(node, scope)
      case ts.SyntaxKind.PropertyAccessExpression:
        return this.member(
          this.expression(node.expression, scope),
          node.name.text,
          node
        )
      case ts.SyntaxKind.ElementAccessExpression:
        return this.member(
          this.expression(node.expression, scope),
          this.expression(node.argumentExpression, scope),
          node
        )
      case ts.SyntaxKind.CallExpression:
        return this.call(node, scope)
      case ts.SyntaxKind.ConditionalExpression:
        return this.choose(
          this.expression(node.condition, scope),
          () => this.expression(node.whenTrue, scope),
          () => this.expression(node.whenFalse, scope)
        )
      case ts.SyntaxKind.PrefixUnaryExpression:
        return this.prefix(node, scope)
      case ts.SyntaxKind.BinaryExpression:
        return this.binary(node, scope)
      case ts.SyntaxKind.JsxElement:
      case ts.SyntaxKind.JsxSelfClosingElement:
      case ts.SyntaxKind.JsxFragment:
        return this.jsx(node, scope)
      default:
        this.fail(node, `Unsupported expression: ${ts.SyntaxKind[node.kind]}`)
    }
  }

  template(node, scope) {
    const parts = [node.head.text]
    for (const span of node.templateSpans) {
      parts.push(this.expression(span.expression, scope), span.literal.text)
    }
    if (parts.some(live))
      return new Live({
        $concat: parts
          .filter(part => part !== '')
          .map(part => this.json(part, node)),
      })
    return parts.map(String).join('')
  }

  array(value, node) {
    if (!Array.isArray(value)) this.fail(node, 'Only a list can be spread here')
    return value
  }

  object(node, scope) {
    const result = {}
    for (const property of node.properties) {
      if (ts.isSpreadAssignment(property)) {
        const value = this.expression(property.expression, scope)
        if (typeof value !== 'object' || value === null || Array.isArray(value))
          this.fail(property, 'Only an object can be spread into an object')
        Object.assign(result, value)
      } else if (ts.isPropertyAssignment(property))
        result[this.key(property.name, scope)] = this.expression(
          property.initializer,
          scope
        )
      else if (ts.isShorthandPropertyAssignment(property))
        result[property.name.text] = this.read(
          property.name.text,
          scope,
          property
        )
      else if (ts.isMethodDeclaration(property))
        result[this.key(property.name, scope)] = new Closure(property, scope)
      else this.fail(property, 'Unsupported object member')
    }
    return result
  }

  key(name, scope) {
    if (ts.isIdentifier(name) || ts.isStringLiteral(name)) return name.text
    if (ts.isComputedPropertyName(name))
      return this.expression(name.expression, scope)
    this.fail(name, 'Unsupported property name')
  }

  member(target, name, node) {
    if (live(target)) this.fail(node, 'A live value cannot be read into')
    if (target instanceof Toaster && name === 'add') return target
    if (target === null || target === undefined)
      this.fail(node, `Cannot read ${name} of nothing`)
    if (Array.isArray(target) || typeof target === 'string') {
      if (name === 'length') return target.length
      if (typeof name === 'number') return target[name]
      if (typeof target[name] === 'function') return { method: name, target }
    }
    if (typeof target === 'object' && !(target instanceof Element))
      return target[name]
    this.fail(node, `Cannot read ${name}`)
  }

  call(node, scope) {
    const callee = this.expression(node.expression, scope)
    const args = node.arguments.map(argument =>
      this.expression(argument, scope)
    )
    if (callee instanceof Hook) return this.hook(callee, args, node)
    if (callee instanceof Toaster)
      this.fail(node, 'A toast can only be raised from an event')
    if (callee instanceof Closure) return this.apply(callee, args, node)
    if (callee?.method) return this.method(callee, args, node)
    this.fail(node, 'Unsupported call')
  }

  method({ method, target }, args, node) {
    const fn = args[0]
    const each = (item, index) => this.apply(fn, [item, index], node)
    switch (method) {
      case 'map':
        return target.map(each)
      case 'filter': {
        const kept = target.filter((item, index) => {
          const keep = each(item, index)
          if (live(keep))
            this.fail(node, 'A filter must be decided ahead of time')
          return keep
        })
        return kept
      }
      case 'find':
        return target.find((item, index) => each(item, index))
      case 'slice':
      case 'concat':
      case 'includes':
      case 'join':
      case 'toLowerCase':
      case 'toUpperCase':
        return target[method](...args)
      default:
        this.fail(node, `Unsupported method: ${method}`)
    }
  }

  hook(hook, args, node) {
    if (!this.scene) this.fail(node, 'Hooks belong inside a component')
    if (hook.name === 'useToast') return new Toaster()
    const slot = `s${Object.keys(this.scene.state).length}`
    this.scene.state[slot] = this.json(args[0] ?? null, node)
    return [new Live({ $state: slot }), new Setter(slot)]
  }

  choose(condition, whenTrue, whenFalse) {
    if (!live(condition)) return condition ? whenTrue() : whenFalse()
    return new Live({
      $if: [condition.json, this.json(whenTrue()), this.json(whenFalse())],
    })
  }

  prefix(node, scope) {
    const value = this.expression(node.operand, scope)
    switch (node.operator) {
      case ts.SyntaxKind.ExclamationToken:
        return live(value) ? new Live({ $not: value.json }) : !value
      case ts.SyntaxKind.MinusToken:
        if (live(value)) this.fail(node, 'Unsupported live arithmetic')
        return -value
      default:
        this.fail(node, 'Unsupported operator')
    }
  }

  binary(node, scope) {
    const op = node.operatorToken.kind
    const left = () => this.expression(node.left, scope)
    const right = () => this.expression(node.right, scope)
    switch (op) {
      case ts.SyntaxKind.AmpersandAmpersandToken: {
        const value = left()
        return this.choose(value, right, () => value)
      }
      case ts.SyntaxKind.BarBarToken: {
        const value = left()
        return this.choose(value, () => value, right)
      }
      case ts.SyntaxKind.QuestionQuestionToken: {
        const value = left()
        if (live(value)) this.fail(node, 'Unsupported live ??')
        return value ?? right()
      }
      case ts.SyntaxKind.EqualsEqualsEqualsToken:
      case ts.SyntaxKind.ExclamationEqualsEqualsToken: {
        const a = left()
        const b = right()
        const negate = op === ts.SyntaxKind.ExclamationEqualsEqualsToken
        if (live(a) || live(b)) {
          const test = { $eq: [this.json(a), this.json(b)] }
          return new Live(negate ? { $not: test } : test)
        }
        return negate ? a !== b : a === b
      }
      case ts.SyntaxKind.PlusToken: {
        const a = left()
        const b = right()
        if (live(a) || live(b))
          return new Live({ $concat: [this.json(a), this.json(b)] })
        return a + b
      }
      case ts.SyntaxKind.MinusToken:
      case ts.SyntaxKind.AsteriskToken:
      case ts.SyntaxKind.SlashToken:
      case ts.SyntaxKind.LessThanToken:
      case ts.SyntaxKind.GreaterThanToken: {
        const a = left()
        const b = right()
        if (live(a) || live(b)) this.fail(node, 'Unsupported live arithmetic')
        return {
          [ts.SyntaxKind.MinusToken]: a - b,
          [ts.SyntaxKind.AsteriskToken]: a * b,
          [ts.SyntaxKind.SlashToken]: a / b,
          [ts.SyntaxKind.LessThanToken]: a < b,
          [ts.SyntaxKind.GreaterThanToken]: a > b,
        }[op]
      }
      default:
        this.fail(node, `Unsupported operator: ${node.operatorToken.getText()}`)
    }
  }

  /* Functions ---------------------------------------------------------- */

  apply(closure, args, node) {
    if (!(closure instanceof Closure))
      this.fail(node, 'Only a function can be called')
    const fn = closure.node
    const scope = new Scope(closure.scope)
    fn.parameters.forEach((parameter, index) =>
      this.bind(parameter.name, args[index], parameter.initializer, scope)
    )
    if (!ts.isBlock(fn.body)) return this.expression(fn.body, scope)
    for (const statement of fn.body.statements) {
      if (ts.isVariableStatement(statement))
        for (const declaration of statement.declarationList.declarations)
          this.bind(
            declaration.name,
            this.expression(declaration.initializer, scope),
            undefined,
            scope
          )
      else if (ts.isReturnStatement(statement))
        return statement.expression
          ? this.expression(statement.expression, scope)
          : undefined
      else if (ts.isIfStatement(statement)) {
        const test = this.expression(statement.expression, scope)
        if (live(test)) this.fail(statement, 'Branch on a live value in JSX')
        if (test || statement.elseStatement)
          this.fail(statement, 'Use an expression, not an if, in a component')
      } else this.fail(statement, 'Unsupported statement in a component')
    }
    return undefined
  }

  bind(name, value, fallback, scope) {
    if (value === undefined && fallback)
      value = this.expression(fallback, scope)
    if (ts.isIdentifier(name)) {
      scope.values.set(name.text, value)
      return
    }
    if (ts.isArrayBindingPattern(name)) {
      name.elements.forEach((element, index) => {
        if (ts.isOmittedExpression(element)) return
        this.bind(element.name, value?.[index], element.initializer, scope)
      })
      return
    }
    if (ts.isObjectBindingPattern(name)) {
      const used = new Set()
      for (const element of name.elements) {
        if (element.dotDotDotToken) {
          const rest = {}
          for (const [key, item] of Object.entries(value ?? {}))
            if (!used.has(key)) rest[key] = item
          this.bind(element.name, rest, undefined, scope)
          continue
        }
        const key = (element.propertyName ?? element.name).text
        used.add(key)
        this.bind(element.name, value?.[key], element.initializer, scope)
      }
      return
    }
    this.fail(name, 'Unsupported binding')
  }

  /* JSX ---------------------------------------------------------------- */

  jsx(node, scope) {
    if (ts.isJsxFragment(node)) return this.children(node.children, scope)
    const opening = ts.isJsxElement(node) ? node.openingElement : node
    const tag = opening.tagName
    if (!ts.isIdentifier(tag) || /^[a-z]/.test(tag.text))
      this.fail(
        tag,
        `<${tag.getText()}> is not portable — use a kit or stage component`
      )
    const type = this.read(tag.text, scope, tag)
    const props = {}
    for (const attribute of opening.attributes.properties) {
      if (ts.isJsxSpreadAttribute(attribute)) {
        Object.assign(props, this.expression(attribute.expression, scope))
        continue
      }
      const name = attribute.name.getText()
      if (name === 'key') continue
      if (name === 'style' || name === 'className')
        this.fail(attribute, `${name} is not portable — use the stage`)
      const init = attribute.initializer
      props[name] = !init
        ? true
        : ts.isStringLiteral(init)
          ? init.text
          : init.expression
            ? this.expression(init.expression, scope)
            : this.fail(attribute, 'Empty attribute')
    }
    const children = ts.isJsxElement(node)
      ? this.children(node.children, scope)
      : []
    if (type instanceof Component) {
      const ownChildren = props.children
      delete props.children
      return new Element(
        type.name,
        props,
        children.length ? children : this.flatten(ownChildren)
      )
    }
    // A Fluent glyph is a component of its own on the web, and a glyph node
    // everywhere else.
    if (type instanceof Glyph) return type
    if (type instanceof Closure) {
      if (children.length)
        props.children = children.length === 1 ? children[0] : children
      return this.apply(type, [props], opening)
    }
    this.fail(tag, `${tag.text} is not a component`)
  }

  children(nodes, scope) {
    const result = []
    for (const child of nodes) {
      if (ts.isJsxText(child)) {
        const text = jsxText(child.text)
        if (text) result.push(text)
      } else if (ts.isJsxExpression(child)) {
        if (child.expression)
          result.push(...this.flatten(this.expression(child.expression, scope)))
      } else result.push(...this.flatten(this.jsx(child, scope)))
    }
    return result
  }

  flatten(value) {
    if (
      value === undefined ||
      value === null ||
      value === false ||
      value === true
    )
      return []
    if (Array.isArray(value)) return value.flatMap(item => this.flatten(item))
    if (typeof value === 'number') return [String(value)]
    return [value]
  }

  /* Data ---------------------------------------------------------------- */

  /** A value as the data every platform reads. */
  json(value, node) {
    if (value === undefined) return null
    if (
      value === null ||
      ['string', 'number', 'boolean'].includes(typeof value)
    )
      return value
    if (Array.isArray(value)) return value.map(item => this.json(item, node))
    if (value instanceof Live) return value.json
    if (value instanceof Glyph) return { $icon: value.name }
    if (value instanceof Element) return this.element(value, node)
    if (value instanceof Setter)
      return { $do: [{ set: value.slot, to: { $arg: 0 } }] }
    if (value instanceof Closure) return this.action(value)
    if (value instanceof Component)
      return this.element(new Element(value.name, {}, []), node)
    if (typeof value === 'object') {
      const result = {}
      for (const [key, item] of Object.entries(value))
        if (item !== undefined) result[key] = this.json(item, node)
      return result
    }
    throw new StoryError(`${this.file}: cannot write ${String(value)} as data`)
  }

  element(element, node) {
    const out = { $el: element.component }
    const props = this.json(element.props, node)
    if (Object.keys(props).length) out.props = props
    if (element.children.length)
      out.children = element.children.map(child => this.json(child, node))
    return out
  }

  /**
   * An event handler, as the writes it makes. Each statement of its body must
   * be a setter called, a toast raised, or nothing at all — anything else
   * would need the handler to run, and it is never run here.
   */
  action(closure) {
    const fn = closure.node
    const scope = new Scope(closure.scope)
    fn.parameters.forEach((parameter, index) =>
      this.bind(parameter.name, new Live({ $arg: index }), undefined, scope)
    )
    const statements = ts.isBlock(fn.body)
      ? fn.body.statements.map(statement => {
          if (!ts.isExpressionStatement(statement))
            this.fail(statement, 'An event handler may only call')
          return statement.expression
        })
      : [fn.body]
    const actions = []
    for (const statement of statements) {
      if (!ts.isCallExpression(statement))
        this.fail(statement, 'An event handler may only call')
      const callee = statement.expression
      if (
        ts.isPropertyAccessExpression(callee) &&
        callee.name.text === 'preventDefault'
      )
        continue
      const target = this.expression(callee, scope)
      const args = statement.arguments.map(argument =>
        this.expression(argument, scope)
      )
      if (target instanceof Setter)
        actions.push({ set: target.slot, to: this.json(args[0], statement) })
      else if (target instanceof Toaster)
        actions.push({ toast: this.json(args[0], statement) })
      else
        this.fail(
          statement,
          'An event handler may only set state or raise a toast'
        )
    }
    return { $do: actions }
  }

  /* Stories ------------------------------------------------------------- */

  /** Renders a component the way a page would, into one scene. */
  render(component, node) {
    const scene = { state: {} }
    const previous = this.scene
    this.scene = scene
    try {
      const value =
        component instanceof Closure
          ? this.apply(component, [{}], node)
          : component
      const nodes = this.flatten(value)
      if (nodes.length !== 1)
        this.fail(node, 'A story renders exactly one component')
      scene.content = this.json(nodes[0], node)
    } finally {
      this.scene = previous
    }
    if (!Object.keys(scene.state).length) delete scene.state
    return scene
  }

  story() {
    const { exported } = this.module()
    const where = this.source.statements.at(-1)
    if (!exported || typeof exported !== 'object')
      this.fail(where, 'A story file exports its story by default')
    for (const key of ['id', 'name', 'summary'])
      if (typeof exported[key] !== 'string')
        this.fail(where, `The story needs a ${key}`)
    return {
      id: exported.id,
      name: exported.name,
      summary: exported.summary,
      preview: this.render(exported.Preview, where),
      examples: exported.examples.map(example => {
        if (typeof example.title !== 'string')
          this.fail(where, 'Every example needs a title')
        return {
          title: example.title,
          ...(example.note ? { note: example.note } : {}),
          scene: this.render(example.Demo, where),
        }
      }),
    }
  }
}

/** JSX's own rule for text: lines trimmed, blank ones dropped, joined by a space. */
function jsxText(text) {
  const lines = text.split(/\r\n|\n|\r/)
  const kept = []
  lines.forEach((line, index) => {
    let value = line.replace(/\t/g, ' ')
    if (index !== 0) value = value.replace(/^ +/, '')
    if (index !== lines.length - 1) value = value.replace(/ +$/, '')
    if (value) kept.push(value)
  })
  return kept.join(' ')
}

/** Reads a TypeScript module's exported constants the same way. */
export function evaluateModule(file, source) {
  const compiler = new StoryCompiler(file, source, { kitValues: {} })
  const { scope } = compiler.module()
  return name => compiler.read(name, scope, compiler.source)
}

export async function readStories(root) {
  const example = path.join(root, 'packages/js/dazzui/example/src')
  const tints = await fs.readFile(
    path.join(root, 'packages/js/dazzui/src/lib/tint.ts'),
    'utf8'
  )
  const context = {
    kitValues: {
      TINTS: evaluateModule('tint.ts', tints)('TINTS'),
    },
  }
  const dir = path.join(example, 'stories')
  const files = (await fs.readdir(dir))
    .filter(file => file.endsWith('.tsx'))
    .sort()
  const stories = []
  for (const file of files) {
    const source = await fs.readFile(path.join(dir, file), 'utf8')
    const story = new StoryCompiler(`stories/${file}`, source, context).story()
    if (`${story.id}.tsx` !== file)
      throw new StoryError(`stories/${file}: the id must be the file's name`)
    stories.push(story)
  }
  return stories.sort((a, b) => a.name.localeCompare(b.name))
}

export async function readThemes(root) {
  const file = 'packages/js/dazzui/example/src/lib/themes.ts'
  const read = evaluateModule(
    file,
    await fs.readFile(path.join(root, file), 'utf8')
  )
  return read('FAMILY_GROUPS')
}

export { StoryError }
