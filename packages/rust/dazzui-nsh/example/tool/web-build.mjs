import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import { delimiter, dirname, resolve } from 'node:path'

const example = resolve(dirname(fileURLToPath(import.meta.url)), '..')
const run = spawnSync(
  'nsh',
  ['package', '--target', 'web', '--out', '../target/web'],
  {
    cwd: example,
    stdio: 'inherit',
    env: {
      ...process.env,
      CARGO_TARGET_DIR: resolve(example, '../target'),
      // Optional project-local CLI: wasm-bindgen must match the resolved Rust
      // crate, without changing the tool used by other projects on this host.
      PATH:
        resolve(example, '../target/web-tools/bin') +
        delimiter +
        process.env.PATH,
    },
  }
)
if (run.error) throw run.error
process.exitCode = run.status ?? 1
