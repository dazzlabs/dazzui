import { loadContext, saveLock } from '../context.mjs'
import { inspectItem } from '../inspect.mjs'
import { applyPlan, planItem } from '../install.mjs'
import { closure, loadRegistry, requireItem } from '../registry.mjs'

export async function add(root, names, { force = false } = {}) {
  if (names.length === 0) {
    throw new Error('nothing to add — pass one or more component names')
  }
  const registry = await loadRegistry()
  const ctx = await loadContext(root)
  for (const name of names) requireItem(registry, name)

  let wrote = false
  for (const name of closure(registry, names)) {
    if (ctx.lock.items[name] && !force) {
      if (names.includes(name)) {
        console.log(`= ${name} already installed (use --force to overwrite)`)
      }
      continue
    }
    if (ctx.lock.items[name] && force) {
      const state = await inspectItem(ctx, registry, name)
      if (state.modified) {
        console.log(`! ${name}: overwriting locally modified files`)
      }
    }
    const plan = await planItem(registry, ctx.config, name)
    await applyPlan(ctx, plan)
    wrote = true
    for (const file of plan.files) console.log(`+ ${file.path}`)
  }

  if (wrote) {
    ctx.lock.version = registry.version
    ctx.lock.digest = registry.digest
    await saveLock(ctx)
  }
}
