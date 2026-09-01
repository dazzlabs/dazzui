import { loadContext } from '../context.mjs'
import { inspectItem } from '../inspect.mjs'
import { loadRegistry } from '../registry.mjs'

export async function status(root) {
  const registry = await loadRegistry()
  const ctx = await loadContext(root)

  const names = Object.keys(ctx.lock.items).sort()
  if (names.length === 0) {
    console.log('nothing installed — run `dzui init`, then `dzui add`')
    return
  }

  let dirty = 0
  let behind = 0
  for (const name of names) {
    const state = await inspectItem(ctx, registry, name)
    const marks = []
    if (state.modified) marks.push('modified')
    if (state.updateAvailable) marks.push('update available')
    if (marks.length === 0) continue
    if (state.modified) dirty += 1
    if (state.updateAvailable) behind += 1
    console.log(`${name} — ${marks.join(', ')}`)
    for (const file of state.files) {
      const notes = []
      if (file.localState !== 'clean') notes.push(file.localState)
      if (file.upstreamChanged) notes.push('upstream changed')
      if (file.lockHash === null) notes.push('new upstream file')
      if (notes.length > 0) console.log(`  ${file.path}: ${notes.join(', ')}`)
    }
    for (const file of state.removed) {
      console.log(`  ${file.path}: removed upstream (${file.localState})`)
    }
  }

  if (dirty === 0 && behind === 0) {
    console.log(`${names.length} items installed, all clean and up to date`)
  } else {
    console.log(
      `\n${names.length} installed — ${dirty} locally modified, ` +
        `${behind} with updates (run \`dzui update\`)`
    )
  }
}
