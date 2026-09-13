import ts from 'typescript'

export const sanitize = text =>
  text
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '')

export const displayName = text =>
  text
    .replace(/([a-z0-9])([A-Z])/g, '$1 $2')
    .replace(/([A-Z])([A-Z][a-z])/g, '$1 $2')
    .replace(/_/g, ' ')

const property = (node, key) =>
  node?.properties?.find(p => p.name?.getText().replace(/['"]/g, '') === key)
    ?.initializer

const string = node =>
  node && ts.isStringLiteral(node) ? node.text : undefined

function object(node) {
  if (!node) return undefined
  if (ts.isSatisfiesExpression(node) || ts.isAsExpression(node))
    return object(node.expression)
  if (ts.isCallExpression(node) && node.expression.getText() === 'defineStory')
    return object(node.arguments[0])
  return ts.isObjectLiteralExpression(node) ? node : undefined
}

/** Read declarations, never execute arbitrary TSX during code generation. */
export function parseStories(source, file) {
  const ast = ts.createSourceFile(
    file,
    source,
    ts.ScriptTarget.Latest,
    true,
    ts.ScriptKind.TSX
  )
  if (ast.parseDiagnostics.length)
    throw new Error(`${file}: invalid TypeScript syntax`)
  let meta
  const exports = []
  const imports = new Map()
  for (const statement of ast.statements) {
    if (ts.isImportDeclaration(statement) && statement.importClause?.name)
      imports.set(
        statement.importClause.name.text,
        statement.moduleSpecifier.text
      )
    if (!ts.isVariableStatement(statement)) continue
    for (const declaration of statement.declarationList.declarations) {
      const value = object(declaration.initializer)
      if (declaration.name.getText() === 'meta') meta = value
      if (
        statement.modifiers?.some(m => m.kind === ts.SyntaxKind.ExportKeyword)
      ) {
        const call = declaration.initializer
        if (
          call &&
          ts.isCallExpression(call) &&
          call.expression.getText() === 'defineStory'
        ) {
          const ref = call.arguments[0]
          if (
            !ref ||
            !ts.isPropertyAccessExpression(ref) ||
            !imports.get(ref.expression.getText())?.endsWith('.scene.json')
          )
            throw new Error(
              `${file}: defineStory must reference an imported .scene.json entry`
            )
          exports.push([
            declaration.name.getText(),
            object(call.arguments[1]),
            {
              file: imports.get(ref.expression.getText()).replace(/^\.\//, ''),
              key: ref.name.text,
            },
          ])
          continue
        }
        if (!value)
          throw new Error(
            `${file}: ${declaration.name.getText()} must be a story object`
          )
        exports.push([declaration.name.getText(), value])
      }
    }
  }
  const title = string(property(meta, 'title'))
  if (!title || !/^(Components|Tokens)\/[^/]+$/.test(title))
    throw new Error(
      `${file}: expected a literal Components/Name or Tokens/Name title`
    )
  return exports.map(([exportName, value, scene]) => {
    const layout =
      string(property(property(value, 'parameters'), 'layout')) ??
      string(property(property(meta, 'parameters'), 'layout')) ??
      'fullscreen'
    if (!['centered', 'fullscreen', 'padded'].includes(layout))
      throw new Error(`${file}/${exportName}: invalid layout ${layout}`)
    return {
      id: `${sanitize(title)}--${sanitize(displayName(exportName))}`,
      title,
      name: string(property(value, 'name')) ?? displayName(exportName),
      exportName,
      importPath: `./stories/${file}`,
      type: 'story',
      layout,
      source: `stories/${file}`,
      ...(scene ? { scene } : {}),
    }
  })
}

export function sortStories(stories) {
  const groups = ['Tokens', 'Components']
  const pages = [
    'Color',
    'Typography',
    'Spacing',
    'Radius',
    'Effect',
    'Control',
    'Primitives',
    'Variables',
  ]
  const rank = (list, value) =>
    list.includes(value) ? list.indexOf(value) : list.length
  const ids = new Set()
  for (const story of stories) {
    if (ids.has(story.id)) throw new Error(`Duplicate story ID: ${story.id}`)
    ids.add(story.id)
  }
  return stories.toSorted((a, b) => {
    if (a.title === b.title) return 0
    const [ga, pa] = a.title.split('/')
    const [gb, pb] = b.title.split('/')
    return (
      rank(groups, ga) - rank(groups, gb) ||
      ga.localeCompare(gb) ||
      (ga === 'Tokens' ? rank(pages, pa) - rank(pages, pb) : 0) ||
      pa.localeCompare(pb)
    )
  })
}
