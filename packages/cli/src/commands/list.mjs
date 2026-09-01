import { loadContext } from '../context.mjs'
import { inspectItem } from '../inspect.mjs'
import { loadRegistry } from '../registry.mjs'

export async function list(root) {
  const registry = await loadRegistry()

  let ctx = null
  try {
    ctx = await loadContext(root)
  } catch {
    // outside an initialized project the plain inventory is still useful
  }

  const names = Object.keys(registry.items).sort((a, b) => {
    const rank = name => (registry.items[name].type === 'component' ? 1 : 0)
    return rank(a) - rank(b) || a.localeCompare(b)
  })

  for (const name of names) {
    const item = registry.items[name]
    let note = item.type === 'component' ? '' : `(${item.type})`
    if (ctx?.lock.items[name]) {
      const state = await inspectItem(ctx, registry, name)
      const marks = ['installed']
      if (state.modified) marks.push('modified')
      if (state.updateAvailable) marks.push('update available')
      note = [note, marks.join(', ')].filter(Boolean).join(' ')
    }
    console.log(note ? `${name.padEnd(20)} ${note}` : name)
  }
}
