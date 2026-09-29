import fs from 'node:fs/promises'
import { createHash } from 'node:crypto'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import {
  glyphsDart,
  storiesIndexDart,
  storyDart,
  storyFile,
  themesDart,
  unsupported,
} from './flutter.mjs'
import { formatOutputs } from './format.mjs'
import * as swiftui from './swiftui.mjs'
import { readStories, readThemes } from './stories.mjs'

const root = fileURLToPath(new URL('../../', import.meta.url))
const check = process.argv.includes('--check')

const stories = await readStories(root)
const gaps = [
  ...unsupported(stories).map(
    gap =>
      `${gap} — map it in packages/dart/dazzui/example/lib/lib/mapping.dart and list it in tools/codegen/flutter.mjs`
  ),
  ...swiftui
    .unsupported(stories)
    .map(
      gap =>
        `${gap} — map it in packages/swift/DazzUI/Example/Sources/Example/Lib/Mapping.swift and list it in tools/codegen/swiftui.mjs`
    ),
]
if (gaps.length)
  throw new Error(
    `A platform cannot draw what these stories ask for:\n${gaps.join('\n')}`
  )
const themes = await readThemes(root)
const variables = await fs.readFile(
  path.join(
    root,
    'packages/dart/dazzui/lib/src/generated/theme_variables.dart'
  ),
  'utf8'
)

/*
 * The Flutter example is laid out as the React one is: `stories/` a file
 * per component, `lib/` the story type and the themes.
 */
const flutter = 'packages/dart/dazzui/example/lib'
const outputs = new Map([
  ...stories.map(story => [
    `${flutter}/stories/${storyFile(story.id)}`,
    storyDart(story),
  ]),
  [`${flutter}/stories/index.dart`, storiesIndexDart(stories)],
  [`${flutter}/lib/glyphs.dart`, glyphsDart(stories)],
  [`${flutter}/lib/themes.dart`, themesDart(themes, variables)],
])

/* The SwiftUI example, laid out the same way. */
const swift = 'packages/swift/DazzUI/Example/Sources/Example'
const themeNames = await fs.readFile(
  path.join(
    root,
    'packages/swift/DazzUI/Sources/DazzUI/Generated/ThemeName.swift'
  ),
  'utf8'
)
for (const story of stories)
  outputs.set(
    `${swift}/Stories/${swiftui.storyFile(story.id)}`,
    swiftui.storySwift(story)
  )
outputs.set(`${swift}/Stories/Index.swift`, swiftui.storiesIndexSwift(stories))
outputs.set(`${swift}/Lib/Glyphs.swift`, swiftui.glyphsSwift(stories))
outputs.set(
  `${swift}/Lib/Themes.swift`,
  swiftui.themesSwift(themes, themeNames)
)
formatOutputs(outputs)

/*
 * `outputs.json` records what this owns and the hash it last wrote. A file it
 * owned and no longer writes is deleted — but only while it still matches
 * that hash, so a hand edit is never thrown away.
 */
const ownershipPath = 'tools/codegen/outputs.json'
const previous = JSON.parse(
  await fs.readFile(path.join(root, ownershipPath), 'utf8').catch(error => {
    if (error.code !== 'ENOENT') throw error
    return '{}'
  })
)
const hash = content => createHash('sha256').update(content).digest('hex')
const stale = []
for (const [relative, expectedHash] of Object.entries(previous)) {
  if (outputs.has(relative)) continue
  if (
    !relative.startsWith('packages/') &&
    !relative.startsWith('tools/codegen/')
  )
    throw new Error(`Invalid owned output: ${relative}`)
  const file = path.resolve(root, relative)
  if (!file.startsWith(root))
    throw new Error(`Output escapes the repository: ${relative}`)
  const content = await fs.readFile(file, 'utf8').catch(error => {
    if (error.code !== 'ENOENT') throw error
    return null
  })
  if (content === null) continue
  if (hash(content) !== expectedHash)
    throw new Error(`Refusing to delete modified generated file: ${relative}`)
  if (check) stale.push(relative)
  else await fs.unlink(file)
}
outputs.set(
  ownershipPath,
  JSON.stringify(
    Object.fromEntries(
      [...outputs].map(([file, content]) => [file, hash(content)])
    ),
    null,
    2
  ) + '\n'
)
for (const [relative, content] of outputs) {
  const file = path.join(root, relative)
  const current = await fs.readFile(file, 'utf8').catch(error => {
    if (error.code !== 'ENOENT') throw error
    return null
  })
  if (current === content) continue
  if (check) stale.push(relative)
  else {
    await fs.mkdir(path.dirname(file), { recursive: true })
    await fs.writeFile(file, content)
  }
}
if (stale.length)
  throw new Error(`Run pnpm codegen; stale outputs:\n${stale.join('\n')}`)
console.log(
  `${stories.length} stories, ${stories.reduce((n, s) => n + s.examples.length, 0)} examples; ${outputs.size} generated files checked`
)
