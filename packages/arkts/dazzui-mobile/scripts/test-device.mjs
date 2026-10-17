import assert from 'node:assert/strict'
import fs from 'node:fs'
import { fileURLToPath } from 'node:url'
import { deviceCommand, mobileDevice } from './device.mjs'

// Run against the installed debug gallery. Assertions use the real native
// theme callback and the OS accessibility tree, rather than a JS renderer.
const device = mobileDevice()
const command = (...args) => deviceCommand(device, args)
const shell = (...args) => command('shell', ...args)
const bundle = 'dev.dazzlabs.dazzui.mobile.playground'
const runId = 'mobile-' + Date.now()
const generated = fs.readFileSync(
  new URL('../src/main/ets/generated/DazzThemes.ets', import.meta.url),
  'utf8'
)
const registry =
  /DAZZ_THEME_FAMILIES: DazzThemeFamily\[\] = \[([\s\S]*?)\]/.exec(generated)
assert.ok(registry, 'Missing generated family registry')
const expectedScopes = registry[1].match(/DazzThemeFamily\./g).length * 2 + 2
const wait = ms => new Promise(resolve => setTimeout(resolve, ms))
const launch = (...args) => {
  shell('aa', 'force-stop', bundle)
  const result = shell(
    'aa',
    'start',
    '-b',
    bundle,
    '-a',
    'EntryAbility',
    ...args
  )
  assert.match(result, /start ability successfully/)
}
if (!process.argv.includes('--ui-only')) {
  launch('--pb', 'verification', 'true', '--ps', 'runId', runId)
  let verified = false
  const deadline = Date.now() + 60000
  while (Date.now() < deadline) {
    await wait(1000)
    const logs = shell('hilog', '-x', '-T', 'DazzMobileTest')
    const report = logs
      .split('\n')
      .find(line => line.includes('DAZZ_MOBILE_VERIFY ' + runId))
    if (report) {
      assert.ok(
        report.includes(`Checked ${expectedScopes} scopes; failures: 0 []`),
        report
      )
      console.log(report.trim())
      verified = true
      break
    }
  }
  assert.ok(
    verified,
    'Native theme verification did not finish within 60 seconds'
  )
}

const layoutFile = '/data/local/tmp/dazzui-mobile-test.json'
function flatten(node) {
  return [node.attributes, ...(node.children ?? []).flatMap(flatten)]
}
function layout() {
  shell('uitest', 'dumpLayout', '-p', layoutFile)
  return flatten(JSON.parse(shell('cat', layoutFile)))
}
function bounds(node) {
  return node.bounds.match(/-?\d+/g).map(Number)
}
function fullyVisible(node) {
  const box = bounds(node)
  const original = node.origBounds.match(/-?\d+/g).map(Number)
  return (
    node.visible === 'true' &&
    box[2] > box[0] &&
    box[3] - box[1] >= (original[3] - original[1]) * 0.9
  )
}
async function find(predicate, label, scroll = false) {
  for (let i = 0; i < 8; i++) {
    const nodes = layout()
    const node = nodes.find(node => predicate(node) && fullyVisible(node))
    if (node) return node
    if (scroll) {
      const container = nodes.find(
        node => node.type === 'Scroll' && node.scrollable === 'true'
      )
      if (container) {
        const [left, top, right, bottom] = bounds(container)
        const x = Math.round((left + right) / 2)
        shell(
          'uitest',
          'uiInput',
          'swipe',
          String(x),
          String(Math.round(top + (bottom - top) * 0.8)),
          String(x),
          String(Math.round(top + (bottom - top) * 0.3))
        )
      }
    }
    await wait(300)
  }
  throw new Error('Missing visible native node: ' + label)
}
const byText = (text, scroll = false) =>
  find(node => node.text === text, text, scroll)
const byId = (id, scroll = false) => find(node => node.id === id, id, scroll)
function center(node) {
  const [left, top, right, bottom] = node.bounds.match(/\d+/g).map(Number)
  assert.ok(right > left && bottom > top, 'Node is not visible: ' + node.id)
  return [Math.round((left + right) / 2), Math.round((top + bottom) / 2)]
}
async function click(node) {
  shell('uitest', 'uiInput', 'click', ...center(node).map(String))
  await wait(350)
}
launch('--ps', 'appearance', 'light')
await byText('DazzUI Mobile')
await click(await byText('Continue'))
await byText('Actions taken: 1')
assert.equal((await byText('Unavailable')).enabled, 'false')
await click(await byText('Show dialog'))
await byText('Save changes?')
await click(await byText('Save'))
await byText('Changes saved')
await click(await byText('Open sheet', true))
await byText('Make room for what matters')
await click(await byText('Done'))
await click(await byText('Inputs'))
shell(
  'uitest',
  'uiInput',
  'inputText',
  ...center(await byId('name-input')).map(String),
  'Harmony'
)
await byText('Hello, Harmony')
shell('uitest', 'uiInput', 'keyEvent', 'Back')
await wait(300)
await click(await byId('notification-toggle', true))
await byText('Notifications off')
await click(await byId('terms-checkbox', true))
assert.equal((await byId('terms-checkbox')).checked, 'true')
await click(await byId('family-select'))
await click(await byText('bright'))
await click(await byId('appearance-select'))
await click(await byText('Dark'))
await click(await byText('Explore'))
await byText('Progress')
await byText('Date & time', true)
console.log(
  'Native UI: buttons, disabled state, dialog, sheet, text input, toggle, checkbox, family and appearance passed on ' +
    device
)

// Leave the gallery open and preserve evidence under its ignored build tree.
launch('--ps', 'family', 'bright', '--ps', 'appearance', 'dark')
await byText('Continue')
const output = fileURLToPath(
  new URL('../example/entry/build/mobile-dark.jpeg', import.meta.url)
)
fs.mkdirSync(
  fileURLToPath(new URL('../example/entry/build/', import.meta.url)),
  { recursive: true }
)
shell('snapshot_display', '-f', '/data/local/tmp/dazzui-mobile-dark.jpeg')
command('file', 'recv', '/data/local/tmp/dazzui-mobile-dark.jpeg', output)
console.log('Screenshot: ' + output)
