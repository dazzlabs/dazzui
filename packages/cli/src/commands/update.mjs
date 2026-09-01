import { rm } from 'node:fs/promises'
import { join, posix } from 'node:path'

import {
  loadContext,
  pristinePath,
  readPristine,
  saveLock,
  sha256,
  writePristine,
  writeProjectFile,
} from '../context.mjs'
import { inspectItem } from '../inspect.mjs'
import { mergeThreeWay } from '../merge.mjs'
import { loadRegistry, requireItem } from '../registry.mjs'

export async function update(root, names, { dryRun = false } = {}) {
  const registry = await loadRegistry()
  const ctx = await loadContext(root)

  for (const name of names) requireItem(registry, name)
  const targets = names.length > 0 ? names : Object.keys(ctx.lock.items).sort()

  let changed = 0
  let conflicted = 0

  for (const name of targets) {
    if (!ctx.lock.items[name]) {
      console.log(`= ${name} is not installed — \`dzui add ${name}\` instead`)
      continue
    }
    const state = await inspectItem(ctx, registry, name)
    // The foundation is vendored, so update also repairs local drift in it
    // even when upstream has not moved.
    const repair = state.type === 'foundation' && state.modified
    if (!state.updateAvailable && !repair) continue

    const lockFiles = { ...ctx.lock.items[name].files }

    for (const file of state.files) {
      const fresh = file.lockHash === null
      if (!fresh && !file.upstreamChanged) {
        if (state.type === 'foundation' && file.localState !== 'clean') {
          if (!dryRun) await writeProjectFile(ctx, file.path, file.content)
          changed += 1
          console.log(`~ ${file.path} (restored — foundation is vendored)`)
        }
        continue
      }

      const takeUpstream = async note => {
        if (!dryRun) {
          await writeProjectFile(ctx, file.path, file.content)
          await writePristine(ctx, name, file.name, file.content)
        }
        lockFiles[file.path] = file.plannedHash
        changed += 1
        console.log(`${fresh ? '+' : '~'} ${file.path}${note}`)
      }

      if (fresh || file.local === null || file.localState === 'clean') {
        await takeUpstream(fresh ? ' (new upstream file)' : '')
      } else if (state.type === 'foundation') {
        // Vendored: local edits to the foundation are overwritten, the same
        // way ds regenerates over a hand-edit in src/generated/.
        await takeUpstream(' (foundation is vendored — local edits dropped)')
      } else if (sha256(file.local) === file.plannedHash) {
        lockFiles[file.path] = file.plannedHash
        console.log(`= ${file.path} already matches upstream`)
      } else {
        const base = await readPristine(ctx, name, file.name)
        if (base === null) {
          console.log(
            `! ${file.path}: no pristine snapshot in .dzui/base — ` +
              'cannot merge, left untouched'
          )
          continue
        }
        const merged = await mergeThreeWay({
          ours: file.local,
          base,
          theirs: file.content,
        })
        if (!dryRun) {
          await writeProjectFile(ctx, file.path, merged.content)
          await writePristine(ctx, name, file.name, file.content)
        }
        lockFiles[file.path] = file.plannedHash
        changed += 1
        if (merged.conflicts > 0) {
          conflicted += 1
          console.log(
            `! ${file.path}: merged with ${merged.conflicts} conflict` +
              `${merged.conflicts === 1 ? '' : 's'} — resolve the markers`
          )
        } else {
          console.log(`~ ${file.path} (merged with local changes)`)
        }
      }
    }

    for (const file of state.removed) {
      delete lockFiles[file.path]
      if (file.localState === 'modified') {
        console.log(
          `! ${file.path}: removed upstream but locally modified — kept ` +
            '(now untracked)'
        )
        continue
      }
      if (!dryRun && file.localState === 'clean') {
        await rm(join(root, file.path), { force: true })
        await rm(join(root, pristinePath(name, posix.basename(file.path))), {
          force: true,
        })
      }
      changed += 1
      console.log(`- ${file.path} (removed upstream)`)
    }

    ctx.lock.items[name].files = lockFiles
  }

  if (!dryRun) {
    ctx.lock.version = registry.version
    ctx.lock.digest = registry.digest
    await saveLock(ctx)
  }

  if (changed === 0) {
    console.log('everything up to date')
  } else if (dryRun) {
    console.log(`\ndry run — ${changed} files would change, nothing written`)
  } else if (conflicted > 0) {
    console.log(`\n${conflicted} files have conflict markers to resolve`)
  }
}
