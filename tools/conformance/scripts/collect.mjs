/*
 * Collect into isolation, then publish successful measurements. Incremental
 * runs replace only requested platform/theme/scenes; verdicts stay derived.
 */
import { execFileSync, spawnSync } from 'node:child_process'
import { randomUUID } from 'node:crypto'
import { cpSync, existsSync, mkdirSync, mkdtempSync, rmSync } from 'node:fs'
import { join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import { parseArgs } from 'node:util'

import {
  json,
  lockRun,
  mergeGaps,
  mergeReadings,
  publishImages,
  selectScenes,
  validateContract,
  writeJson,
} from './storage.mjs'

const here = fileURLToPath(new URL('..', import.meta.url))
const repo = join(here, '../..')
let commit = process.env.DAZZUI_CONFORMANCE_COMMIT
let workingTree = true
try {
  commit ??= execFileSync('git', ['rev-parse', '--short', 'HEAD'], {
    cwd: repo,
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'ignore'],
  }).trim()
  workingTree = Boolean(
    execFileSync('git', ['status', '--porcelain'], {
      cwd: repo,
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'ignore'],
    }).trim()
  )
} catch {
  commit ??= 'unknown'
}
const { values: options } = parseArgs({
  options: {
    out: { type: 'string', default: join(here, 'data') },
    incremental: { type: 'boolean', default: false },
    platforms: { type: 'string' },
    themes: { type: 'string', default: 'drawn' },
    scenes: { type: 'string' },
    'no-images': { type: 'boolean', default: false },
  },
})
const data = resolve(options.out)
const COLLECTORS = [
  ['react', 'node', ['scripts/conformance.mjs'], 'packages/js/dazzui/example'],
  ['flutter', 'tool/conformance.sh', [], 'packages/dart/dazzui/example'],
  ['swiftui', 'tool/conformance.sh', [], 'packages/swift/DazzUI/Example'],
  ['gpui', 'example/tool/conformance.sh', [], 'packages/rust/dazzui'],
  ['winui', 'scripts/conformance.sh', [], 'packages/csharp/DazzUI'],
  // GTK4 remains opt-in until its mapping gaps join the default baselines.
  [
    'gtk4',
    'example/tool/conformance.sh',
    [],
    'packages/rust/dazzui-gtk4',
    false,
  ],
]
const selected = options.platforms?.split(',').map(id => id.trim())
for (const id of selected ?? [])
  if (!COLLECTORS.some(([platform]) => platform === id))
    throw new Error(`Unknown platform: ${id || '(empty)'}`)
const collectors = COLLECTORS.filter(([id, , , , enabled]) =>
  selected ? selected.includes(id) : enabled !== false
)
const contract = json(join(here, 'scenes.json'))
const scenes = selectScenes(contract, options.scenes)
mkdirSync(data, { recursive: true })
const release = lockRun(data)
const staging = mkdtempSync(join(data, '.pending-'))
const collected = []
try {
  const oldRun = existsSync(join(data, 'run.json'))
    ? json(join(data, 'run.json'))
    : undefined
  if (options.incremental && oldRun)
    validateContract(
      json(join(data, 'scenes.json')),
      contract,
      scenes,
      oldRun.platforms,
      collectors.map(([id]) => id)
    )
  let failed = false
  for (const [platform, command, args, cwd] of collectors) {
    const out = join(staging, platform)
    const passed = ['--out', out, '--themes', options.themes]
    if (options.scenes) passed.push('--scenes', scenes.join(','))
    if (options['no-images']) passed.push('--no-images')
    const result = spawnSync(command, [...args, ...passed], {
      cwd: join(repo, cwd),
      stdio: 'inherit',
      env: { ...process.env, DAZZUI_CONFORMANCE_COMMIT: commit },
    })
    const file = join(out, 'readings.json')
    if (result.status !== 0 || result.error) {
      if (result.error) console.error(`${platform}: ${result.error.message}`)
      failed = true
    } else if (existsSync(file)) {
      const fresh = json(file)
      if (fresh.platform !== platform || !Object.keys(fresh.themes).length)
        throw new Error(`${platform}: collector wrote no valid themes`)
      const previous =
        options.incremental && existsSync(join(data, platform, 'readings.json'))
          ? json(join(data, platform, 'readings.json'))
          : undefined
      // Validate all merges before publishing any collector's files.
      mergeReadings(previous, fresh, scenes)
      const gapFile = join(out, 'gaps.json')
      const oldGaps = join(data, platform, 'gaps.json')
      const gaps = existsSync(gapFile)
        ? mergeGaps(
            options.incremental && existsSync(oldGaps) ? json(oldGaps) : {},
            json(gapFile),
            scenes
          )
        : undefined
      collected.push({ platform, out, fresh, previous, gaps })
    } else if (options.incremental || selected) {
      console.error(`${platform}: collector produced no readings`)
      failed = true
    }
  }
  if (failed)
    throw new Error('Collection failed; the previous run was retained')
  const platforms = options.incremental ? [...(oldRun?.platforms ?? [])] : []
  for (const { platform } of collected)
    if (!platforms.includes(platform)) platforms.push(platform)
  platforms.sort(
    (a, b) =>
      COLLECTORS.findIndex(([p]) => p === a) -
      COLLECTORS.findIndex(([p]) => p === b)
  )
  const hasReference =
    collected.some(({ fresh }) => fresh.role === 'reference') ||
    (options.incremental &&
      platforms.some(
        p =>
          existsSync(join(data, p, 'readings.json')) &&
          json(join(data, p, 'readings.json')).role === 'reference'
      ))
  if (!hasReference)
    throw new Error('The run needs a reference; include --platforms react')
  const prepared = collected.map(({ platform, out, fresh, previous, gaps }) => {
    const destination = join(staging, 'publish', platform)
    const published = publishImages(fresh, out, destination)
    return {
      platform,
      destination,
      readings: mergeReadings(previous, published, scenes),
      gaps,
    }
  })
  for (const { platform, destination, readings, gaps } of prepared) {
    const images = join(destination, 'images')
    if (existsSync(images))
      cpSync(images, join(data, platform, 'images'), {
        recursive: true,
        force: false,
      })
    writeJson(join(data, platform, 'readings.json'), readings)
    if (gaps) writeJson(join(data, platform, 'gaps.json'), gaps)
  }
  writeJson(join(data, 'scenes.json'), contract)
  writeJson(join(data, 'tolerances.json'), json(join(here, 'tolerances.json')))
  const snapshots = Object.fromEntries(
    platforms.map(platform => {
      const readings = json(join(data, platform, 'readings.json'))
      return [
        platform,
        { collected: readings.collected, commit: readings.commit },
      ]
    })
  )
  // Publish the notification marker last: the viewer reloads on its revision.
  writeJson(join(data, 'run.json'), {
    collected: new Date().toISOString(),
    commit,
    platforms,
    snapshots,
    revision: randomUUID(),
    incremental: options.incremental,
    workingTree:
      workingTree || Boolean(options.incremental && oldRun?.workingTree),
  })
  console.log(
    `conformance: ${collected.map(c => c.platform).join(', ')} ${options.incremental ? 'updated' : 'collected'} (${scenes.length} scenes)`
  )
} finally {
  rmSync(staging, { recursive: true, force: true })
  release()
}
