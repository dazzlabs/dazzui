import { createHash, randomUUID } from 'node:crypto'
import {
  copyFileSync,
  closeSync,
  existsSync,
  mkdirSync,
  openSync,
  readFileSync,
  renameSync,
  rmSync,
  writeFileSync,
} from 'node:fs'
import { dirname, isAbsolute, join, relative, resolve } from 'node:path'

export const json = path => JSON.parse(readFileSync(path, 'utf8'))

export function lockRun(directory) {
  const path = join(directory, '.collect.lock')
  try {
    const descriptor = openSync(path, 'wx')
    writeFileSync(descriptor, JSON.stringify({ pid: process.pid }))
    closeSync(descriptor)
  } catch (error) {
    if (error.code !== 'EEXIST') throw error
    let alive = true
    try {
      process.kill(json(path).pid, 0)
    } catch (failure) {
      if (failure.code === 'ESRCH') alive = false
    }
    if (alive) throw new Error('Another collection is updating this run')
    rmSync(path)
    return lockRun(directory)
  }
  return () => rmSync(path, { force: true })
}

export function writeJson(path, value) {
  mkdirSync(dirname(path), { recursive: true })
  const pending = `${path}.${randomUUID()}.tmp`
  try {
    writeFileSync(pending, JSON.stringify(value, null, 2) + '\n')
    renameSync(pending, path)
  } finally {
    rmSync(pending, { force: true })
  }
}

export function selectScenes(contract, selectors) {
  const all = contract.stories.flatMap(story => story.scenes)
  if (selectors === undefined) return all.map(scene => scene.id)
  const selected = new Set()
  for (const selector of selectors.split(',').map(id => id.trim())) {
    const matches = all.filter(
      scene => scene.id === selector || scene.id.startsWith(`${selector}/`)
    )
    if (!selector || !matches.length)
      throw new Error(`Unknown scene or story: ${selector || '(empty)'}`)
    for (const scene of matches) selected.add(scene.id)
  }
  return all.filter(scene => selected.has(scene.id)).map(scene => scene.id)
}

const sceneOf = address => address.slice(0, address.lastIndexOf('/'))

export function mergeGaps(previous, fresh, scenes) {
  const merged = { ...previous }
  for (const scene of scenes) delete merged[scene]
  for (const [scene, gaps] of Object.entries(fresh)) {
    if (!scenes.includes(scene))
      throw new Error(`Unrequested mapping gap: ${scene}`)
    merged[scene] = gaps
  }
  return merged
}
const stamp = readings => ({
  collected: readings.collected,
  commit: readings.commit,
})

/** Replace whole scenes, including specimens that no longer draw. */
export function mergeReadings(previous, fresh, scenes) {
  if (previous)
    for (const field of ['platform', 'role', 'scale', 'font'])
      if (previous[field] !== fresh[field])
        throw new Error(
          `${fresh.platform}: ${field} changed; collect a full run`
        )
  const merged = structuredClone(previous ?? fresh)
  const scope = new Set(scenes)
  merged.collected = fresh.collected
  merged.commit = fresh.commit
  merged.title = fresh.title
  merged.snapshots ??= {}
  if (previous)
    for (const [theme, entries] of Object.entries(previous.themes)) {
      const snapshots = (merged.snapshots[theme] ??= {})
      for (const scene of new Set([
        ...Object.keys(entries).map(sceneOf),
        ...Object.keys(previous.scenes[theme] ?? {}),
      ]))
        snapshots[scene] ??= stamp(previous)
    }
  for (const [theme, entries] of Object.entries(fresh.themes)) {
    for (const address of Object.keys(entries))
      if (!scope.has(sceneOf(address)))
        throw new Error(
          `${fresh.platform}: collector returned unrequested ${address}`
        )
    for (const scene of Object.keys(fresh.scenes[theme] ?? {}))
      if (!scope.has(scene))
        throw new Error(
          `${fresh.platform}: collector returned unrequested ${scene}`
        )
    const target = (merged.themes[theme] ??= {})
    const images = (merged.scenes[theme] ??= {})
    const snapshots = (merged.snapshots[theme] ??= {})
    for (const address of Object.keys(target))
      if (scope.has(sceneOf(address))) delete target[address]
    for (const scene of scenes) {
      delete images[scene]
      snapshots[scene] = stamp(fresh)
    }
    Object.assign(target, entries)
    Object.assign(images, fresh.scenes[theme] ?? {})
  }
  return merged
}

/** Keep image URLs immutable, so an open viewer cannot show cached old pixels. */
export function publishImages(readings, staging, destination) {
  const output = structuredClone(readings)
  const paths = new Map()
  const publish = path => {
    if (paths.has(path)) return paths.get(path)
    const source = resolve(staging, path)
    const rel = relative(resolve(staging), source)
    if (isAbsolute(path) || rel.startsWith('..') || isAbsolute(rel))
      throw new Error(`Image outside collector output: ${path}`)
    const bytes = readFileSync(source)
    const hash = createHash('sha256').update(bytes).digest('hex')
    const extension = path.endsWith('.svg') ? '.svg' : '.png'
    const name = `images/${hash}${extension}`
    const target = join(destination, name)
    mkdirSync(dirname(target), { recursive: true })
    if (!existsSync(target)) copyFileSync(source, target)
    paths.set(path, name)
    return name
  }
  for (const entries of Object.values(output.themes))
    for (const entry of Object.values(entries))
      if (entry.image) entry.image = publish(entry.image)
  for (const scenes of Object.values(output.scenes))
    for (const [scene, path] of Object.entries(scenes))
      scenes[scene] = publish(path)
  return output
}

/** A changed address contract must be refreshed on every retained platform. */
export function validateContract(previous, next, scenes, retained, selected) {
  if (!previous) return
  const entries = contract =>
    new Map(contract.stories.flatMap(story => story.scenes).map(s => [s.id, s]))
  const before = entries(previous)
  const after = entries(next)
  const changed = [...new Set([...before.keys(), ...after.keys()])].filter(
    id => JSON.stringify(before.get(id)) !== JSON.stringify(after.get(id))
  )
  if (!changed.length) return
  const outside = changed.filter(id => !scenes.includes(id))
  if (outside.length)
    throw new Error(
      `Scene contract changed outside this update: ${outside.join(', ')}; collect a full run`
    )
  const omitted = retained.filter(platform => !selected.includes(platform))
  if (omitted.length)
    throw new Error(
      `Scene contract changed; also update ${omitted.join(', ')} or collect a full run`
    )
}
