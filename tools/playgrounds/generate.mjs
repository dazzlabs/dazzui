import fs from 'node:fs/promises'
import { createHash } from 'node:crypto'
import { requireComplete } from './completeness.mjs'
import { formatOutputs } from './format.mjs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { parseStories, sortStories } from './catalog.mjs'
import { nativeOutputs } from './native.mjs'
import { validateScene } from './schema/scene.mjs'

const root = fileURLToPath(new URL('../../', import.meta.url))
const example = path.join(root, 'packages/js/dazzui-react/example')
const files = (await fs.readdir(path.join(example, 'stories')))
  .filter(f => f.endsWith('.stories.tsx'))
  .sort()
const stories = sortStories(
  (
    await Promise.all(
      files.map(async file =>
        parseStories(
          await fs.readFile(path.join(example, 'stories', file), 'utf8'),
          file
        )
      )
    )
  ).flat()
)
const manifest = {
  version: 1,
  source: 'packages/js/dazzui-react/example/stories',
  stories,
}
const scenes = new Map()
for (const story of stories) {
  if (!story.scene) continue
  const source = JSON.parse(
    await fs.readFile(path.join(example, 'stories', story.scene.file), 'utf8')
  )
  const scene = validateScene(
    source[story.scene.key],
    `${story.scene.file}/${story.scene.key}`
  )
  scenes.set(story.id, scene)
}
const outputs = new Map([
  [
    'packages/js/dazzui-react/example/src/generated/catalog.json',
    JSON.stringify(manifest, null, 2) + '\n',
  ],
  [
    'packages/js/dazzui-react/example/src/generated/scenes.json',
    JSON.stringify(Object.fromEntries(scenes), null, 2) + '\n',
  ],
])
for (const [file, content] of nativeOutputs(stories, scenes))
  outputs.set(file, content)
if (process.argv.includes('--require-complete'))
  requireComplete(
    stories,
    scenes,
    JSON.parse(outputs.get('tools/playgrounds/coverage.json'))
  )
formatOutputs(outputs)
const ownershipPath = 'tools/playgrounds/outputs.json'
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
    !relative.startsWith('tools/playgrounds/')
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
  if (process.argv.includes('--check')) stale.push(relative)
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
  if (process.argv.includes('--check')) stale.push(relative)
  else {
    await fs.mkdir(path.dirname(file), { recursive: true })
    await fs.writeFile(file, content)
  }
}
if (stale.length)
  throw new Error(
    `Run pnpm playgrounds:generate; stale outputs:\n${stale.join('\n')}`
  )
console.log(
  `${stories.length} stories across ${new Set(stories.map(s => s.title)).size} pages; ${outputs.size} generated files checked`
)
