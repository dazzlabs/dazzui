import { readFile } from 'node:fs/promises'
import { dirname, join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const here = dirname(fileURLToPath(import.meta.url))
const registryRoot = resolve(here, '../registry')

export async function loadRegistry() {
  let raw
  try {
    raw = await readFile(join(registryRoot, 'registry.json'), 'utf8')
  } catch {
    throw new Error(
      'registry not built — run `pnpm registry` at the workspace root ' +
        '(published builds ship it via prepack)'
    )
  }
  return JSON.parse(raw)
}

export function readRegistryFile(itemName, fileName) {
  return readFile(join(registryRoot, 'files', itemName, fileName), 'utf8')
}

export function requireItem(registry, name) {
  const item = registry.items[name]
  if (!item) {
    const known = Object.keys(registry.items).join(', ')
    throw new Error(`unknown item '${name}' — the registry has: ${known}`)
  }
  return item
}

/*
 * The install closure of a set of names: their dependencies, recursively,
 * plus the foundation — which no component imports but every component's CSS
 * assumes, so it rides along implicitly the way the token sheet does for the
 * package. Returned in dependency-first order so installs read sensibly.
 */
export function closure(registry, names) {
  const ordered = []
  const seen = new Set()
  const visit = name => {
    if (seen.has(name)) return
    seen.add(name)
    const item = requireItem(registry, name)
    for (const dep of item.dependencies) visit(dep)
    ordered.push(name)
  }
  visit('foundation')
  for (const name of names) visit(name)
  return ordered
}
