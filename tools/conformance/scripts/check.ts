/*
 * `pnpm conformance:check`: holds the run in data/ to expectations/. Fails
 * on a check that is off or missing and not listed, and on a line that is
 * listed and now passes — so a fix deletes its lines in the same change,
 * and each file stays an honest account of what is left. See
 * specs/conformance.md.
 *
 *   node scripts/check.ts               hold the run to every file
 *   node scripts/check.ts --prune       delete the lines that now pass
 *   node scripts/check.ts --baseline p  write p's first file from the run —
 *                                       refused once the file lists anything
 */
import { existsSync, readFileSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { parseArgs } from 'node:util'

import {
  baseline,
  EVERY,
  format,
  hold,
  type Line,
  measure,
  type Measured,
  parse,
} from '../src/lib/expectations.ts'
import type { Readings, Run, Tolerances } from '../src/lib/model.ts'

const here = fileURLToPath(new URL('..', import.meta.url))
const data = join(here, 'data')
const { values: args } = parseArgs({
  options: {
    prune: { type: 'boolean', default: false },
    baseline: { type: 'string' },
  },
})

const json = <T>(path: string): T => JSON.parse(readFileSync(path, 'utf8')) as T

if (!existsSync(join(data, 'run.json'))) {
  console.error('No run in data/ — run pnpm conformance first.')
  process.exit(1)
}
const run = json<Run>(join(data, 'run.json'))
const tolerances = json<Tolerances>(join(data, 'tolerances.json'))
const readings = run.platforms.map(p =>
  json<Readings>(join(data, p, 'readings.json'))
)
const reference = readings.find(r => r.role === 'reference')
if (!reference) throw new Error('The run has no reference')

const fileOf = (platform: string) =>
  join(here, 'expectations', `${platform}.txt`)
const SHOWN = 20
let failed = false

for (const implementation of readings) {
  if (implementation === reference) continue
  const platform = implementation.platform
  if (args.baseline && args.baseline !== platform) continue
  const file = fileOf(platform)
  const text = existsSync(file) ? readFileSync(file, 'utf8') : ''
  const lines = parse(text)
  const measured = measure(reference, implementation, tolerances)
  const themes = `${measured.themes.length} theme${measured.themes.length === 1 ? '' : 's'}`

  if (args.baseline === platform) {
    if (lines.length) {
      console.error(
        `${platform}: expectations/${platform}.txt already lists ${lines.length} checks; a line may only be deleted`
      )
      failed = true
      continue
    }
    const header = text.split('\n').filter(line => line.startsWith('#'))
    const written = baseline(measured)
    writeFileSync(
      file,
      format(
        [
          ...header,
          `# Written from the run at ${run.commit} over ${measured.themes.join(', ')}.`,
          `# \`*\` for the theme is every theme a run measures; for the property, a specimen missing as a whole.`,
        ],
        written
      )
    )
    console.log(`${platform}: wrote ${written.length} lines over ${themes}`)
    continue
  }

  const { unexpected, stale } = hold(lines, measured)
  if (args.prune && stale.length) {
    writeFileSync(file, pruned(text, measured))
    console.log(`${platform}: pruned ${stale.length} lines that now pass`)
  }
  const staleNow = args.prune ? [] : stale
  if (!unexpected.length && !staleNow.length) {
    console.log(
      `${platform}: ${measured.failing.size} failing checks over ${themes}, all listed`
    )
    continue
  }
  failed = true
  if (unexpected.length) {
    console.error(
      `${platform}: ${unexpected.length} failing checks not listed in expectations/${platform}.txt — fix them in the port, or in the collector if it reads the wrong thing:`
    )
    for (const failure of unexpected.slice(0, SHOWN))
      console.error(`  ${failure}`)
    if (unexpected.length > SHOWN)
      console.error(`  … and ${unexpected.length - SHOWN} more`)
  }
  if (staleNow.length) {
    console.error(
      `${platform}: ${staleNow.length} listed checks now pass — delete their lines (pnpm conformance:check --prune):`
    )
    for (const { line, passes } of staleNow.slice(0, SHOWN))
      console.error(
        `  ${line.theme} ${line.address} ${line.property}${line.theme === EVERY ? ` (passes in ${passes.join(', ')})` : ''}`
      )
    if (staleNow.length > SHOWN)
      console.error(`  … and ${staleNow.length - SHOWN} more`)
  }
}

for (const platform of ['flutter', 'swiftui', 'winui'])
  if (!args.baseline && !run.platforms.includes(platform))
    console.log(`${platform}: not in this run — not held`)

if (failed) process.exitCode = 1

/**
 * The file with what now passes taken out, in place: comments stay where
 * they stand, a line that passes goes, and a `*` line that passes in some
 * themes becomes a line for each theme it still fails in.
 */
function pruned(text: string, measured: Measured): string {
  const out: string[] = []
  for (const raw of text.split('\n')) {
    const [line] = parse(raw) as (Line | undefined)[]
    if (!line) {
      out.push(raw)
      continue
    }
    const { stale } = hold([line], measured)
    if (!stale.length) {
      out.push(raw)
      continue
    }
    if (line.theme === EVERY)
      for (const theme of measured.themes)
        if (!stale[0]!.passes.includes(theme))
          out.push(raw.replace(/^\*/, theme))
  }
  return out.join('\n')
}
