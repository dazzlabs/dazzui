/*
 * `pnpm conformance`: a run. Copies the contract and the tolerances into
 * data/, has each platform's collector write its readings beside them, and
 * records which platforms took part in run.json. A collector's only contract
 * is the file it writes; see specs/conformance.md.
 *
 *   node scripts/collect.mjs [--themes drawn|all|<id,…>] [--scenes <story,…>]
 *                            [--no-images]
 */
import { execFileSync, spawnSync } from 'node:child_process'
import {
  copyFileSync,
  existsSync,
  mkdirSync,
  rmSync,
  writeFileSync,
} from 'node:fs'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

const here = fileURLToPath(new URL('..', import.meta.url))
const repo = join(here, '../..')
const data = join(here, 'data')
const passed = process.argv.slice(2)

/* The reference first; implementations join as their collectors are written. */
const COLLECTORS = [
  {
    platform: 'react',
    command: 'node',
    args: ['scripts/conformance.mjs'],
    cwd: join(repo, 'packages/js/dazzui-react/example'),
  },
  {
    platform: 'flutter',
    command: 'tool/conformance.sh',
    args: [],
    cwd: join(repo, 'packages/dart/dazzui/example'),
  },
  {
    // Runs on a Windows machine over SSH, or is skipped without one.
    platform: 'winui',
    command: 'scripts/conformance.sh',
    args: [],
    cwd: join(repo, 'packages/csharp/DazzUI'),
  },
]

rmSync(data, { recursive: true, force: true })
mkdirSync(data, { recursive: true })
copyFileSync(join(here, 'scenes.json'), join(data, 'scenes.json'))
copyFileSync(join(here, 'tolerances.json'), join(data, 'tolerances.json'))

const platforms = []
let failed = false
for (const { platform, command, args, cwd } of COLLECTORS) {
  const out = join(data, platform)
  const result = spawnSync(command, [...args, '--out', out, ...passed], {
    cwd,
    stdio: 'inherit',
  })
  if (result.status !== 0) failed = true
  if (existsSync(join(out, 'readings.json'))) platforms.push(platform)
}

writeFileSync(
  join(data, 'run.json'),
  JSON.stringify(
    {
      collected: new Date().toISOString(),
      commit: execFileSync('git', ['rev-parse', '--short', 'HEAD'], {
        cwd: repo,
        encoding: 'utf8',
      }).trim(),
      platforms,
    },
    null,
    2
  ) + '\n'
)
console.log(`conformance: ${platforms.join(', ') || 'no platform'} collected`)
if (failed) process.exitCode = 1
