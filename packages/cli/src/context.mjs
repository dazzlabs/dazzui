import { createHash } from 'node:crypto'
import { mkdir, readFile, writeFile } from 'node:fs/promises'
import { dirname, join } from 'node:path'

export const CONFIG_FILE = 'dzui.json'
export const LOCK_FILE = 'dzui.lock.json'

/*
 * The pristine snapshots live under `.dzui/base/<item>/<file>`: each is the
 * file exactly as the CLI wrote it, kept so `update` has a base for a
 * three-way merge once the user has edited their copy. Committed alongside
 * the lockfile — without them an update can only overwrite or give up.
 */
export const BASE_DIR = join('.dzui', 'base')

export const DEFAULT_CONFIG = {
  platform: 'react',
  paths: {
    components: 'src/components/ui',
    lib: 'src/lib/dzui',
    styles: 'src/styles/dzui',
  },
  aliases: {},
}

export const sha256 = content =>
  createHash('sha256').update(content).digest('hex')

export async function loadContext(root) {
  let raw
  try {
    raw = await readFile(join(root, CONFIG_FILE), 'utf8')
  } catch {
    throw new Error(`no ${CONFIG_FILE} in ${root} — run \`dzui init\` first`)
  }
  const config = { ...DEFAULT_CONFIG, ...JSON.parse(raw) }
  config.paths = { ...DEFAULT_CONFIG.paths, ...config.paths }

  let lock = { version: null, digest: null, items: {} }
  try {
    lock = JSON.parse(await readFile(join(root, LOCK_FILE), 'utf8'))
  } catch {
    // first run — an empty lockfile is the correct starting state
  }

  return { root, config, lock }
}

export async function saveLock(ctx) {
  const items = {}
  for (const name of Object.keys(ctx.lock.items).sort()) {
    items[name] = ctx.lock.items[name]
  }
  ctx.lock.items = items
  await writeFile(
    join(ctx.root, LOCK_FILE),
    JSON.stringify(ctx.lock, null, 2) + '\n'
  )
}

export async function writeProjectFile(ctx, relPath, content) {
  const abs = join(ctx.root, relPath)
  await mkdir(dirname(abs), { recursive: true })
  await writeFile(abs, content)
}

export async function readProjectFile(ctx, relPath) {
  try {
    return await readFile(join(ctx.root, relPath), 'utf8')
  } catch {
    return null
  }
}

export function pristinePath(itemName, fileName) {
  return join(BASE_DIR, itemName, fileName)
}

export async function writePristine(ctx, itemName, fileName, content) {
  await writeProjectFile(ctx, pristinePath(itemName, fileName), content)
}

export async function readPristine(ctx, itemName, fileName) {
  return readProjectFile(ctx, pristinePath(itemName, fileName))
}
