import fs from 'node:fs'
import path from 'node:path'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import { hdc, mobileDevice } from './device.mjs'

const project = fileURLToPath(new URL('../example/', import.meta.url))
function run(command, args) {
  const result = spawnSync(command, args, { cwd: project, stdio: 'inherit' })
  if (result.error) throw result.error
  if (result.status !== 0) process.exit(result.status ?? 1)
}
const builder = fileURLToPath(new URL('./hvigor.mjs', import.meta.url))
run(process.execPath, [builder, 'install'])
run(process.execPath, [builder, 'hap'])
const outputs = path.join(project, 'entry/build/default/outputs/default')
const hap = ['entry-default-signed.hap', 'entry-default-unsigned.hap']
  .map(name => path.join(outputs, name))
  .find(file => fs.existsSync(file))
if (!hap) throw new Error('The HarmonyOS build produced no installable HAP.')
const device = ['-t', mobileDevice()]
run(hdc, [...device, 'install', '-r', hap])
run(hdc, [
  ...device,
  'shell',
  'aa',
  'start',
  '-b',
  'dev.dazzlabs.dazzui.mobile.playground',
  '-a',
  'EntryAbility',
])
