import { spawnSync } from 'node:child_process'
import { mkdtemp, rm, writeFile } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { join } from 'node:path'

/*
 * Three-way merge via `git merge-file`, which is diff3 with the conflict
 * markers every editor and agent already knows how to finish. Exit status is
 * the number of conflicts (negative on error), so a clean merge and a
 * conflicted one both come back as content — the caller decides what a
 * conflict means.
 */
export async function mergeThreeWay({ ours, base, theirs }) {
  const dir = await mkdtemp(join(tmpdir(), 'dzui-merge-'))
  try {
    const paths = {
      ours: join(dir, 'ours'),
      base: join(dir, 'base'),
      theirs: join(dir, 'theirs'),
    }
    await writeFile(paths.ours, ours)
    await writeFile(paths.base, base)
    await writeFile(paths.theirs, theirs)
    const result = spawnSync(
      'git',
      [
        'merge-file',
        '-p',
        '-L',
        'local',
        '-L',
        'base',
        '-L',
        'upstream',
        paths.ours,
        paths.base,
        paths.theirs,
      ],
      { encoding: 'utf8' }
    )
    if (result.error) {
      throw new Error(
        `git merge-file failed (${result.error.message}) — ` +
          'updates to locally modified files need git on PATH'
      )
    }
    if (result.status < 0) {
      throw new Error(`git merge-file errored: ${result.stderr}`)
    }
    return { content: result.stdout, conflicts: result.status }
  } finally {
    await rm(dir, { recursive: true, force: true })
  }
}
