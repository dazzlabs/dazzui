import { spawnSync } from 'node:child_process'

function run(command, args, input) {
  const result = spawnSync(command, args, {
    input,
    encoding: 'utf8',
    maxBuffer: 16 * 1024 * 1024,
  })
  if (result.error || result.status !== 0)
    throw new Error(
      `Generated-source formatter failed: ${command}\n${result.error?.message ?? result.stderr}`
    )
  return result.stdout
}

/** Format before comparing, so native formatting does not dirty generated code. */
export function formatOutputs(outputs) {
  for (const [file, content] of outputs) {
    if (file.endsWith('.dart'))
      outputs.set(
        file,
        run(
          process.env.DART ?? 'dart',
          ['format', '--output=show', '--summary=none'],
          content
        )
      )
  }
}
