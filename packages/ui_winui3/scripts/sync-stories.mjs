import fs from 'node:fs/promises'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const root = fileURLToPath(new URL('../', import.meta.url))
export async function nativeCatalog() {
  const catalog = new Map()
  const swift = path.join(
    root,
    '../ui_swiftui/Example/Sources/Playground/Stories'
  )
  for (const file of await fs.readdir(swift, { recursive: true })) {
    if (!file.endsWith('Stories.swift')) continue
    const source = await fs.readFile(path.join(swift, file), 'utf8')
    const name = /StoryPage\("([^"]+)"/.exec(source)?.[1]
    if (!name) continue
    const stories = [
      ...source.matchAll(/Story\("([^"]+)"(?:, layout: \.(\w+))?/g),
    ].map(match => ({ name: match[1], layout: match[2] ?? 'centered' }))
    catalog.set(name, {
      name,
      group: file.includes('Tokens') ? 'Tokens' : 'Components',
      stories,
    })
  }
  const flutter = path.join(root, '../ui_flutter/example/lib/stories')
  for (const file of await fs.readdir(flutter, { recursive: true })) {
    if (!file.endsWith('_stories.dart')) continue
    const source = await fs.readFile(path.join(flutter, file), 'utf8')
    const name = /StoryPage\('([^']+)'/.exec(source)?.[1]
    if (!name) continue
    const page = catalog.get(name)
    if (!page)
      throw new Error(`Flutter page missing from Swift reference: ${name}`)
    for (const match of source.matchAll(/\bStory\(\s*'([^']+)'/g))
      if (
        !page.stories.some(
          story => story.name.toLowerCase() === match[1].toLowerCase()
        )
      )
        page.stories.push({
          name: match[1],
          layout: match[1] === 'Overview' ? 'page' : 'centered',
        })
  }
  // Button, Badge and StepList content has moved out of the handwritten Flutter/SwiftUI pages.
  // Its canonical catalog now comes directly from the shared declarations.
  const react = JSON.parse(
    await fs.readFile(
      path.join(root, '../ui_react/example/src/generated/catalog.json'),
      'utf8'
    )
  )
  for (const page of [
    'Button',
    'Badge',
    'StepList',
    'Callout',
    'Card',
    'Divider',
  ])
    catalog.set(page, {
      name: page,
      group: 'Components',
      stories: react.stories
        .filter(story => story.title === `Components/${page}`)
        .map(story => ({
          name: story.name,
          layout: story.exportName === 'Overview' ? 'page' : story.layout,
        })),
    })
  return [...catalog.values()]
}
const catalog = await nativeCatalog()
if (process.argv.includes('--check')) {
  const report = JSON.parse(await fs.readFile(process.argv.at(-1), 'utf8'))
  const actual = new Set(
    report.map(story =>
      `${story.group}/${story.page}/${story.name}`.toLowerCase()
    )
  )
  const expected = catalog.flatMap(page =>
    page.stories.map(story =>
      `${page.group}/${page.name}/${story.name}`.toLowerCase()
    )
  )
  const missing = expected.filter(id => !actual.has(id))
  const expectedSet = new Set(expected)
  const extra = [...actual].filter(id => !expectedSet.has(id))
  if (actual.size !== report.length)
    throw new Error('Duplicate native stories in the runtime registry')
  if (missing.length)
    throw new Error(`Missing native stories:\n${missing.join('\n')}`)
  if (extra.length)
    throw new Error(`Unexpected native stories:\n${extra.join('\n')}`)
  console.log(
    `Native registry parity: ${catalog.length} pages, ${expected.length} stories`
  )
} else {
  const output = path.join(root, 'example/Stories')
  for (const page of catalog) {
    // Owned by tools/playgrounds; never overwrite its generated catalog.
    if (
      page.group === 'Components' &&
      ['Button', 'Badge', 'StepList', 'Callout', 'Card', 'Divider'].includes(
        page.name
      )
    )
      continue
    const className = page.name + 'Stories'
    const directory = path.join(output, page.group === 'Tokens' ? 'Tokens' : '')
    await fs.mkdir(directory, { recursive: true })
    const lines = page.stories.map(
      story =>
        `        new(${JSON.stringify(story.name)}, theme => ${page.group === 'Tokens' ? 'TokenExamples' : 'ComponentExamples'}.${page.name}(theme, ${JSON.stringify(story.name)}), StoryLayout.${page.group === 'Tokens' ? 'Page' : story.layout[0].toUpperCase() + story.layout.slice(1)}),`
    )
    await fs.writeFile(
      path.join(directory, className + '.cs'),
      `// Story names synchronized with the Flutter/SwiftUI playgrounds.\nusing DazzUI.Playground.Storybook;\n\nnamespace DazzUI.Playground.Stories;\n\npublic static class ${className}\n{\n    public static StoryPage Page { get; } = new(${JSON.stringify(page.group)}, ${JSON.stringify(page.name)}, [\n${lines.join('\n')}\n    ]);\n}\n`
    )
  }
  const tokenOrder = [
    'Color',
    'Typography',
    'Spacing',
    'Radius',
    'Effect',
    'Control',
    'Themes',
  ]
  const pages = [
    ...catalog
      .filter(page => page.group === 'Tokens')
      .sort((a, b) => tokenOrder.indexOf(a.name) - tokenOrder.indexOf(b.name)),
    ...catalog
      .filter(page => page.group === 'Components')
      .sort((a, b) => a.name.localeCompare(b.name)),
  ]
  await fs.writeFile(
    path.join(root, 'example/Storybook/Registry.cs'),
    `// Synchronized native playground index.\nnamespace DazzUI.Playground.Storybook;\n\npublic static class Registry\n{\n    public static IReadOnlyList<StoryPage> Pages { get; } = [\n${pages.map(page => `        Stories.${page.name}Stories.Page,`).join('\n')}\n    ];\n}\n`
  )
  console.log(
    `Synchronized ${catalog.length} pages, ${catalog.reduce((n, p) => n + p.stories.length, 0)} stories`
  )
}
