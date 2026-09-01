import { readFile, writeFile } from 'node:fs/promises'
import { join, posix } from 'node:path'

import {
  CONFIG_FILE,
  DEFAULT_CONFIG,
  loadContext,
  saveLock,
} from '../context.mjs'
import { applyPlan, planItem } from '../install.mjs'
import { closure, loadRegistry } from '../registry.mjs'

export async function init(root, { force = false } = {}) {
  const registry = await loadRegistry()

  let existing = null
  try {
    existing = await readFile(join(root, CONFIG_FILE), 'utf8')
  } catch {
    // no config yet — the normal case
  }
  if (existing === null) {
    await writeFile(
      join(root, CONFIG_FILE),
      JSON.stringify(DEFAULT_CONFIG, null, 2) + '\n'
    )
    console.log(`+ ${CONFIG_FILE}`)
  } else if (!force) {
    console.log(`${CONFIG_FILE} already exists — keeping it`)
  } else {
    await writeFile(
      join(root, CONFIG_FILE),
      JSON.stringify(DEFAULT_CONFIG, null, 2) + '\n'
    )
    console.log(`~ ${CONFIG_FILE} (reset to defaults)`)
  }

  const ctx = await loadContext(root)
  for (const name of closure(registry, ['lib'])) {
    const plan = await planItem(registry, ctx.config, name)
    await applyPlan(ctx, plan)
    for (const file of plan.files) console.log(`+ ${file.path}`)
  }
  ctx.lock.version = registry.version
  ctx.lock.digest = registry.digest
  await saveLock(ctx)

  const entry = posix.join(ctx.config.paths.styles, 'index.css')
  console.log(`
Done. Import the foundation stylesheet (${entry}) once from your app
entry, then \`dzui add <component>\` to copy components in. Commit
dzui.json, dzui.lock.json and .dzui/ — updates merge against them.`)
}
