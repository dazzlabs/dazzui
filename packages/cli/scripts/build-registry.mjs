/*
 * Build the registry the CLI distributes from, by reading the ui_react
 * sources. Nothing here is hand-maintained: the item list comes from the
 * component file pairs, and the dependency edges come from parsing each
 * component's imports — the same way the package's own registries
 * (`components/index.ts`, `styles/components.css`) are the source of truth
 * for what ships.
 *
 * The output (`registry/`) is staging like `ds`'s dist/: gitignored, rebuilt
 * by `pnpm registry`, and packed into the published tarball by `prepack`.
 */

import { createHash } from 'node:crypto'
import { mkdir, readFile, readdir, rm, writeFile } from 'node:fs/promises'
import { dirname, join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const here = dirname(fileURLToPath(import.meta.url))
const uiRoot = resolve(here, '../../ui_react')
const outRoot = resolve(here, '../registry')

const sha256 = content => createHash('sha256').update(content).digest('hex')

/*
 * The foundation is everything a component's CSS assumes is already on the
 * page: the token sheet, the resolved recipes, and the ground rules. In the
 * package these live in two directories (`generated/` and `styles/`); the
 * registry flattens them into one item because a consumer installs and
 * updates them as a unit.
 */
const FOUNDATION = [
  ['style.css', 'src/generated/style.css'],
  ['control.css', 'src/generated/control.css'],
  ['base.css', 'src/styles/base.css'],
]

const LIB = [
  ['cx.ts', 'src/lib/cx.ts'],
  ['tint.ts', 'src/lib/tint.ts'],
]

async function main() {
  const pkg = JSON.parse(await readFile(join(uiRoot, 'package.json'), 'utf8'))
  await rm(outRoot, { recursive: true, force: true })
  await mkdir(join(outRoot, 'files'), { recursive: true })

  const items = {}

  const addItem = async (name, type, files, dependencies = []) => {
    const entries = []
    await mkdir(join(outRoot, 'files', name), { recursive: true })
    for (const [fileName, content] of files) {
      await writeFile(join(outRoot, 'files', name, fileName), content)
      entries.push({ name: fileName, hash: sha256(content) })
    }
    items[name] = { type, dependencies, files: entries }
  }

  const readSources = paths =>
    Promise.all(
      paths.map(async ([name, rel]) => [
        name,
        await readFile(join(uiRoot, rel), 'utf8'),
      ])
    )

  await addItem('foundation', 'foundation', await readSources(FOUNDATION))
  await addItem('lib', 'lib', await readSources(LIB))

  const componentsDir = join(uiRoot, 'src/components')
  const names = (await readdir(componentsDir))
    .filter(file => file.endsWith('.tsx'))
    .map(file => file.replace(/\.tsx$/, ''))
    .sort()

  for (const name of names) {
    const tsx = await readFile(join(componentsDir, `${name}.tsx`), 'utf8')
    const css = await readFile(join(componentsDir, `${name}.css`), 'utf8')

    const dependencies = new Set()
    if (/from '\.\.\/lib\//.test(tsx)) dependencies.add('lib')
    for (const match of tsx.matchAll(/from '\.\/([a-z-]+)'/g)) {
      dependencies.add(match[1])
    }

    await addItem(
      name,
      'component',
      [
        [`${name}.tsx`, tsx],
        [`${name}.css`, css],
      ],
      [...dependencies].sort()
    )
  }

  for (const [name, item] of Object.entries(items)) {
    for (const dep of item.dependencies) {
      if (!items[dep]) {
        throw new Error(
          `${name} depends on ${dep}, which is not in the registry`
        )
      }
    }
  }

  const digest = sha256(
    Object.entries(items)
      .flatMap(([name, item]) =>
        item.files.map(file => `${name}/${file.name}:${file.hash}`)
      )
      .sort()
      .join('\n')
  )

  const registry = {
    name: pkg.name,
    version: pkg.version,
    digest,
    items,
  }
  await writeFile(
    join(outRoot, 'registry.json'),
    JSON.stringify(registry, null, 2) + '\n'
  )

  const count = Object.keys(items).length
  console.log(`registry: ${count} items (digest ${digest.slice(0, 12)})`)
}

await main()
