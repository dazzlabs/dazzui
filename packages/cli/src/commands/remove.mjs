import { rm } from 'node:fs/promises'
import { join } from 'node:path'

import { BASE_DIR, loadContext, saveLock } from '../context.mjs'
import { inspectItem } from '../inspect.mjs'
import { loadRegistry, requireItem } from '../registry.mjs'

export async function remove(root, names, { force = false } = {}) {
  if (names.length === 0) {
    throw new Error('nothing to remove — pass one or more component names')
  }
  const registry = await loadRegistry()
  const ctx = await loadContext(root)

  for (const name of names) {
    const item = requireItem(registry, name)
    if (!ctx.lock.items[name]) {
      console.log(`= ${name} is not installed`)
      continue
    }

    const dependents = Object.keys(ctx.lock.items).filter(
      other =>
        other !== name &&
        !names.includes(other) &&
        registry.items[other]?.dependencies.includes(name)
    )
    if (item.type !== 'component') {
      const components = Object.keys(ctx.lock.items).filter(
        other => registry.items[other]?.type === 'component'
      )
      if (components.length > 0) {
        throw new Error(
          `cannot remove ${name} while components are installed ` +
            `(${components.join(', ')})`
        )
      }
    } else if (dependents.length > 0) {
      throw new Error(
        `cannot remove ${name} — still needed by ${dependents.join(', ')}`
      )
    }

    const state = await inspectItem(ctx, registry, name)
    if (state.modified && !force) {
      throw new Error(
        `${name} has local modifications — remove with --force to discard`
      )
    }

    for (const path of Object.keys(ctx.lock.items[name].files)) {
      await rm(join(root, path), { force: true })
      console.log(`- ${path}`)
    }
    await rm(join(root, BASE_DIR, name), { recursive: true, force: true })
    delete ctx.lock.items[name]
    // Persist per item, so a refusal later in the list cannot leave already
    // deleted files still claimed by the lock.
    await saveLock(ctx)
  }
}
