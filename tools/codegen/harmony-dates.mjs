import ts from 'typescript'
import { Closure, StoryError } from './stories.mjs'

const unwrap = node =>
  ts.isParenthesizedExpression(node) || ts.isAsExpression(node)
    ? unwrap(node.expression)
    : node
const fail = (node, message) => {
  const source = node.getSourceFile()
  const { line, character } = source.getLineAndCharacterOfPosition(
    node.getStart(source)
  )
  throw new StoryError(
    `${source.fileName}:${line + 1}:${character + 1}: ${message}`
  )
}
const number = value => ({ op: 'number', number: value })

export function dateFormulaType(value) {
  if (
    !value ||
    typeof value !== 'object' ||
    Object.keys(value).some(
      key => !['op', 'args', 'number', 'name'].includes(key)
    )
  )
    return null
  const args = value.args ?? []
  if (!Array.isArray(args)) return null
  const types = args.map(dateFormulaType)
  const exact = (...expected) =>
    types.length === expected.length &&
    types.every((type, index) => type === expected[index])
  if (value.op === 'number')
    return exact() &&
      typeof value.number === 'number' &&
      Number.isFinite(value.number)
      ? 'number'
      : null
  if (value.number !== undefined) return null
  if (value.op === 'today') return exact() ? 'date' : null
  if (value.op === 'local')
    return types.length >= 3 &&
      types.length <= 7 &&
      types.every(type => type === 'number')
      ? 'date'
      : null
  if (value.op === 'timestamp') return exact('number') ? 'date' : null
  if (value.op === 'get')
    return exact('date') &&
      [
        'getTime',
        'getDay',
        'getFullYear',
        'getMonth',
        'getDate',
        'getHours',
        'getMinutes',
      ].includes(value.name)
      ? 'number'
      : null
  if (value.op === 'addDays') return exact('date', 'number') ? 'date' : null
  if (value.op === 'startOfMonth') return exact('date') ? 'date' : null
  if (
    ['add', 'subtract', 'multiply', 'divide', 'modulo', 'or'].includes(value.op)
  )
    return exact('number', 'number') ? 'number' : null
  return null
}

/** A sealed date formula, never eval or executable source in the native app. */
export function dateRangeFormula(closure, compiler) {
  function body(fn, args, depth) {
    if (depth > 16 || fn.node.parameters.some(p => !ts.isIdentifier(p.name)))
      fail(fn.node, 'Unsupported date function parameters or recursion')
    const bindings = new Map()
    fn.node.parameters.forEach((p, i) =>
      bindings.set(
        p.name.text,
        args[i] ??
          (p.initializer
            ? expression(p.initializer, bindings, fn.scope, depth)
            : fail(p, 'Missing date argument'))
      )
    )
    if (!ts.isBlock(fn.node.body))
      return { node: unwrap(fn.node.body), bindings, scope: fn.scope }
    for (const statement of fn.node.body.statements) {
      if (ts.isVariableStatement(statement)) {
        if (!(statement.declarationList.flags & ts.NodeFlags.Const))
          fail(statement, 'Date formula locals must be constant')
        for (const declaration of statement.declarationList.declarations) {
          if (!ts.isIdentifier(declaration.name) || !declaration.initializer)
            fail(declaration, 'Unsupported date formula local')
          bindings.set(
            declaration.name.text,
            expression(declaration.initializer, bindings, fn.scope, depth)
          )
        }
      } else if (ts.isReturnStatement(statement) && statement.expression)
        return { node: unwrap(statement.expression), bindings, scope: fn.scope }
      else
        fail(
          statement,
          'Only constant locals and a return are mapped in date formulas'
        )
    }
    fail(fn.node, 'Date formula needs a return')
  }
  function read(name, scope, node) {
    if (!scope.has(name)) fail(node, `Unknown date formula name: ${name}`)
    return compiler.read(name, scope, node)
  }
  function expression(node, bindings, scope, depth) {
    node = unwrap(node)
    if (ts.isNumericLiteral(node)) return number(Number(node.text))
    if (ts.isIdentifier(node)) {
      if (bindings.has(node.text)) return bindings.get(node.text)
      const value = read(node.text, scope, node)
      if (typeof value === 'number' && Number.isFinite(value))
        return number(value)
      if (value?.$date) return { op: 'local', args: value.$date.map(number) }
      fail(node, 'Only dates and numbers may be captured in a date formula')
    }
    if (
      ts.isPrefixUnaryExpression(node) &&
      node.operator === ts.SyntaxKind.MinusToken
    )
      return {
        op: 'subtract',
        args: [number(0), expression(node.operand, bindings, scope, depth)],
      }
    if (ts.isBinaryExpression(node)) {
      const op = {
        [ts.SyntaxKind.PlusToken]: 'add',
        [ts.SyntaxKind.MinusToken]: 'subtract',
        [ts.SyntaxKind.AsteriskToken]: 'multiply',
        [ts.SyntaxKind.SlashToken]: 'divide',
        [ts.SyntaxKind.PercentToken]: 'modulo',
        [ts.SyntaxKind.BarBarToken]: 'or',
      }[node.operatorToken.kind]
      if (!op) fail(node, 'Unsupported date formula operator')
      return {
        op,
        args: [
          expression(node.left, bindings, scope, depth),
          expression(node.right, bindings, scope, depth),
        ],
      }
    }
    if (
      ts.isNewExpression(node) &&
      ts.isIdentifier(node.expression) &&
      node.expression.text === 'Date'
    ) {
      const args = (node.arguments ?? []).map(arg =>
        expression(arg, bindings, scope, depth)
      )
      if (args.length !== 1 && (args.length < 3 || args.length > 7))
        fail(node, 'Unsupported date formula constructor')
      return { op: args.length === 1 ? 'timestamp' : 'local', args }
    }
    if (ts.isCallExpression(node)) {
      if (ts.isPropertyAccessExpression(node.expression)) {
        const name = node.expression.name.text
        if (
          ![
            'getTime',
            'getDay',
            'getFullYear',
            'getMonth',
            'getDate',
            'getHours',
            'getMinutes',
          ].includes(name) ||
          node.arguments.length
        )
          fail(node, 'Unsupported date getter')
        return {
          op: 'get',
          name,
          args: [
            expression(node.expression.expression, bindings, scope, depth),
          ],
        }
      }
      if (!ts.isIdentifier(node.expression))
        fail(node, 'Unsupported date formula call')
      const fn = read(node.expression.text, scope, node)
      const args = node.arguments.map(arg =>
        expression(arg, bindings, scope, depth)
      )
      if (fn?.dateIntrinsic) return { op: fn.dateIntrinsic, args }
      if (!(fn instanceof Closure))
        fail(node, 'Date formulas may only call known date helpers')
      const next = body(fn, args, depth + 1)
      return expression(next.node, next.bindings, next.scope, depth + 1)
    }
    fail(node, `Unsupported date formula: ${ts.SyntaxKind[node.kind]}`)
  }
  const result = body(closure, [{ op: 'today' }], 0)
  if (!ts.isObjectLiteralExpression(result.node))
    fail(result.node, 'A preset must return its start and end')
  const range = {}
  for (const property of result.node.properties) {
    const key = property.name?.text
    if (
      !['start', 'end'].includes(key) ||
      (!ts.isPropertyAssignment(property) &&
        !ts.isShorthandPropertyAssignment(property))
    )
      fail(property, 'Unsupported preset range member')
    range[key] = expression(
      ts.isPropertyAssignment(property) ? property.initializer : property.name,
      result.bindings,
      result.scope,
      0
    )
  }
  if (!range.start || !range.end)
    fail(result.node, 'A preset needs both endpoints')
  if (
    dateFormulaType(range.start) !== 'date' ||
    dateFormulaType(range.end) !== 'date'
  )
    fail(result.node, 'Preset endpoints must be date formulas')
  return range
}

export function dateRangeKitSource(source, file, Compiler) {
  const parsed = ts.createSourceFile(
    file,
    source,
    ts.ScriptTarget.Latest,
    true,
    ts.ScriptKind.TSX
  )
  let stripped = source
  const intrinsics = {}
  for (const node of [...parsed.statements].reverse())
    if (ts.isImportDeclaration(node)) {
      if (node.moduleSpecifier.text === '../lib/date')
        for (const item of node.importClause?.namedBindings?.elements ?? []) {
          const name = (item.propertyName ?? item.name).text
          if (['addDays', 'startOfMonth'].includes(name))
            intrinsics[item.name.text] = { dateIntrinsic: name }
        }
      stripped =
        stripped.slice(0, node.getStart(parsed)) +
        stripped
          .slice(node.getStart(parsed), node.end)
          .replace(/[^\r\n]/g, ' ') +
        stripped.slice(node.end)
    }
  const compiler = new Compiler(
    file,
    stripped +
      '\nexport default { presets: DATE_RANGE_PRESETS, durations: DATE_RANGE_DURATIONS, labels: LABELS }',
    { kitValues: {} }
  )
  const { scope, exported } = compiler.module()
  for (const [name, value] of Object.entries(intrinsics))
    scope.values.set(name, value)
  return exported
}
