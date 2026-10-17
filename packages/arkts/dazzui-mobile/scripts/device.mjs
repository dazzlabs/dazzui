import fs from 'node:fs'
import path from 'node:path'
import { execFileSync } from 'node:child_process'

const home =
  process.env.DEVECO_HOME ?? '/Applications/DevEco-Studio.app/Contents'
const bundled = path.join(home, 'sdk/default/openharmony/toolchains/hdc')
export const hdc = process.env.HDC ?? (fs.existsSync(bundled) ? bundled : 'hdc')
const output = args =>
  execFileSync(hdc, args, {
    encoding: 'utf8',
    timeout: 15000,
    maxBuffer: 8 * 1024 * 1024,
  }).trim()

export function mobileDevice() {
  if (process.env.HARMONY_DEVICE) return process.env.HARMONY_DEVICE
  const targets = output(['list', 'targets'])
    .split(/\s+/)
    .filter(target => target && target !== '[Empty]')
  const mobile = targets.filter(target =>
    /^(phone|tablet)$/.test(
      output([
        '-t',
        target,
        'shell',
        'param',
        'get',
        'const.product.devicetype',
      ])
    )
  )
  if (mobile.length !== 1)
    throw new Error(
      'Connect one phone/tablet or set HARMONY_DEVICE to its hdc target ID.'
    )
  return mobile[0]
}

export function deviceCommand(device, args) {
  return output(['-t', device, ...args])
}
