import fs from 'node:fs/promises'
import { readFileSync } from 'node:fs'
import path from 'node:path'
import ts from 'typescript'
import { fileURLToPath } from 'node:url'
import { validateScene, SLOTS, COMPONENTS } from './schema/scene.mjs'

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
  const unbox = n =>
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
    if (ts.isTemplateExpression(n))
      return (
        n.head.text +
        n.templateSpans
          .map(span => {
            const value = evaluate(span.expression, env)
            if (!['string', 'number', 'boolean'].includes(typeof value))
              throw Error('Template substitutions must be scalar')
            return String(value) + span.literal.text
          })
          .join('')
      )
    if (ts.isPropertyAccessExpression(n) || ts.isElementAccessExpression(n)) {
      const object = evaluate(n.expression, env)
      const key = ts.isPropertyAccessExpression(n)
        ? n.name.text
        : evaluate(n.argumentExpression, env)
      if (object == null && n.questionDotToken) return undefined
      if (object == null || !Object.hasOwn(object, key))
        throw Error(`Unknown static property ${key}`)
      return object[key]
    }
    if (ts.isNumericLiteral(n)) return Number(n.text)
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
          '../../packages/ui_react/src/lib/tint.ts',
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
        n.properties.map(prop => {
          if (!ts.isPropertyAssignment(prop))
            throw Error('Only literal style properties can be imported')
          return [prop.name.text, evaluate(prop.initializer, env)]
        })
      )
    }
    if (ts.isArrayLiteralExpression(n))
      return n.elements.map(x => evaluate(x, env))
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
      let component = tag === 'span' ? 'Inline' : tag === 'div' ? 'Box' : tag
      const props = {},
        slots = {}
      for (const attr of opening.attributes.properties) {
        if (!ts.isJsxAttribute(attr))
          throw Error('Spread attributes need an explicit scene mapping')
        const name = attr.name.getText(ast)
        if (name === 'key') continue
        const value = !attr.initializer
          ? true
          : ts.isJsxExpression(attr.initializer)
            ? evaluate(attr.initializer.expression, env)
            : evaluate(attr.initializer, env)
        if (value === undefined) continue
        if ((SLOTS[component] ?? []).includes(name)) {
          slots[name] = value
          continue
        }
        if (
          (name === 'width' || (component === 'Grid' && name === 'min')) &&
          typeof value === 'string' &&
          /^\d+(\.\d+)?rem$/.test(value)
        )
          props[name] = parseFloat(value) * 16
        else
          props[
            name === 'fullWidth' ? 'expand' : name === 'icon' ? 'name' : name
          ] = value
      }
      if (
        (component === 'Row' || component === 'Col') &&
        typeof props.gap === 'string'
      )
        props.gap = props.gap.replace(/^var\(--spacing-(\d+)\)$/, 'spacing.$1')
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
                ? [evaluate(child.expression, env)].flat(Infinity)
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
            template.parameters.length !== 1 ||
            !parameter ||
            !ts.isObjectBindingPattern(parameter.name) ||
            body?.length !== 1 ||
            !ts.isReturnStatement(body[0]) ||
            !body[0].expression
          )
            throw Error(`${tag}: template requires a single pure JSX return`)
          if (expanding.has(tag)) throw Error(`${tag}: recursive template`)
          const args = { ...props, ...(children.length ? { children } : {}) }
          const scope = { ...env }
          const consumed = new Set()
          for (const field of parameter.name.elements) {
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
            return evaluate(body[0].expression, scope)
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
          /^\d+(\.\d+)?rem$/.test(value)
        )
          box[property] = parseFloat(value) * 16
        else if (
          ['width', 'height', 'maxWidth'].includes(property) &&
          typeof value === 'number'
        )
          box[property] = value
        else {
          const mapping = {
            flex: { 1: ['grow', true] },
            display: {
              'inline-flex': ['display', 'inline-flex'],
              flex: ['display', 'flex'],
            },
            justifyContent: { 'space-between': ['justify', 'between'] },
            gap: {
              'var(--spacing-1)': ['gap', 'spacing.1'],
              'var(--spacing-2)': ['gap', 'spacing.2'],
              'var(--spacing-3)': ['gap', 'spacing.3'],
            },
            alignItems: { center: ['align', 'center'] },
            color: { 'var(--color-content-muted)': ['tone', 'muted'] },
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
      if (component === 'Inline' && box.display === 'inline-flex')
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
      const result = {
        component,
        ...(Object.keys(props).length ? { props } : {}),
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
      if (!render || !ts.isArrowFunction(render) || ts.isBlock(render.body))
        throw Error(
          `${d.name.text}: imperative render requires manual migration`
        )
      const scene = { content: evaluate(render.body) }
      // Native default values become explicit initial state, retaining editing
      // behavior without copying an uncontrolled fixture into every adapter.
      const state = {}
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
  await fs.writeFile(
    sceneFile,
    JSON.stringify({ ...previous, ...scenes }, null, 2) + '\n'
  )
  await fs.writeFile(file, output)
  console.log(
    `Imported ${Object.keys(scenes).length} stories as data. Remove unused TSX imports, regenerate and verify screenshots.`
  )
}
