import { readProjectFile, sha256 } from './context.mjs'
import { planItem } from './install.mjs'

/*
 * Compare one installed item's three copies of the truth: the lockfile hash
 * (what the CLI last wrote), the working file (what the user has), and the
 * freshly planned content (what the CLI would write today). Every command
 * reasons from this one comparison — "modified" is local vs lock, "update
 * available" is planned vs lock.
 */
export async function inspectItem(ctx, registry, name) {
  const plan = await planItem(registry, ctx.config, name)
  const lockEntry = ctx.lock.items[name]
  const lockFiles = lockEntry ? { ...lockEntry.files } : {}

  const files = []
  for (const file of plan.files) {
    const lockHash = lockFiles[file.path] ?? null
    delete lockFiles[file.path]
    const local = await readProjectFile(ctx, file.path)
    const plannedHash = sha256(file.content)
    files.push({
      name: file.name,
      path: file.path,
      content: file.content,
      plannedHash,
      lockHash,
      localState:
        local === null
          ? 'missing'
          : sha256(local) === lockHash
            ? 'clean'
            : 'modified',
      local,
      upstreamChanged: lockHash !== null && plannedHash !== lockHash,
    })
  }

  // Anything still in the lock entry was removed upstream.
  const removed = []
  for (const [path, lockHash] of Object.entries(lockFiles)) {
    const local = await readProjectFile(ctx, path)
    removed.push({
      path,
      localState:
        local === null
          ? 'missing'
          : sha256(local) === lockHash
            ? 'clean'
            : 'modified',
    })
  }

  return {
    name,
    type: plan.type,
    installed: Boolean(lockEntry),
    files,
    removed,
    modified: files.some(file => file.localState !== 'clean'),
    updateAvailable:
      files.some(file => file.upstreamChanged || file.lockHash === null) ||
      removed.length > 0,
  }
}
