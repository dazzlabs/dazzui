import fs from 'node:fs'
import path from 'node:path'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'

const project = fileURLToPath(new URL('../example/', import.meta.url))
const home =
  process.env.DEVECO_HOME ?? '/Applications/DevEco-Studio.app/Contents'
const command = process.argv[2] ?? 'hap'
if (!['hap', 'har', 'install'].includes(command))
  throw new Error(`Unknown HarmonyOS task: ${command}`)
function executable(override, candidates) {
  if (override) return override
  return candidates.find(candidate => fs.existsSync(candidate))
}
function run(command, args) {
  const result = spawnSync(command, args, { cwd: project, stdio: 'inherit' })
  if (result.error) throw result.error
  if (result.status !== 0) process.exit(result.status ?? 1)
}
if (command === 'install') {
  const ohpm = executable(process.env.OHPM, [
    path.join(home, 'tools/ohpm/bin/ohpm'),
  ])
  if (!ohpm)
    throw new Error(
      'ohpm is unavailable. Install DevEco Studio or set OHPM to its executable path.'
    )
  run(ohpm, ['install', '--all'])
} else {
  const hvigor = executable(process.env.HVIGOR, [
    path.join(project, 'hvigorw'),
    path.join(home, 'tools/hvigor/bin/hvigorw.js'),
  ])
  if (!hvigor)
    throw new Error(
      'HarmonyOS build requires DevEco Studio with a bundled HarmonyOS SDK (API 20 or newer). Open packages/arkts/dazzui-mobile/example in DevEco Studio and sync, or set HVIGOR to its launcher path.'
    )
  const args = [
    '--mode',
    'module',
    '-p',
    `module=${command === 'har' ? 'dazzui_mobile' : 'entry'}@default`,
    '-p',
    'product=default',
    '-p',
    'buildMode=debug',
    command === 'har' ? 'assembleHar' : 'assembleHap',
    '--no-daemon',
  ]
  if (hvigor.endsWith('.js')) {
    const node =
      executable(process.env.DEVECO_NODE, [
        path.join(home, 'tools/node/bin/node'),
      ]) ?? process.execPath
    run(node, [hvigor, ...args])
  } else run(hvigor, args)
}
