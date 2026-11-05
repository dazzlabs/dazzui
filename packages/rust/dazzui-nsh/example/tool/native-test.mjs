import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import { dirname, resolve } from 'node:path'
const example = resolve(dirname(fileURLToPath(import.meta.url)), '..')
const target =
  process.platform === 'darwin'
    ? 'macos'
    : process.platform === 'win32'
      ? 'windows'
      : 'linux'
const run = spawnSync('nsh', ['test', 'native.test.ts', '--target', target], {
  cwd: example,
  stdio: 'inherit',
  env: { ...process.env, CARGO_TARGET_DIR: resolve(example, '../target') },
})
if (run.error) throw run.error
process.exitCode = run.status ?? 1
