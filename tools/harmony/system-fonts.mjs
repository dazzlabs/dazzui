import assert from 'node:assert/strict'
import fs from 'node:fs'
import path from 'node:path'
import { execFileSync } from 'node:child_process'

// Exercise the installed gallery's test-only page. Scale overrides are confined
// to that process; always stop it and relaunch the normal system-following app.
const mobile = process.argv[2] === 'mobile'
assert.ok(mobile || process.argv[2] === 'desktop', 'Choose desktop or mobile')
const home =
  process.env.DEVECO_HOME ?? '/Applications/DevEco-Studio.app/Contents'
const bundled = path.join(home, 'sdk/default/openharmony/toolchains/hdc')
const hdc = process.env.HDC ?? (fs.existsSync(bundled) ? bundled : 'hdc')
const output = args =>
  execFileSync(hdc, args, {
    encoding: 'utf8',
    timeout: 15000,
    maxBuffer: 8 * 1024 * 1024,
  }).trim()
let device = process.env.HARMONY_DEVICE
if (!device) {
  const targets = output(['list', 'targets'])
    .split(/\s+/)
    .filter(id => id && id !== '[Empty]')
  const matches = targets.filter(id => {
    const type = output([
      '-t',
      id,
      'shell',
      'param',
      'get',
      'const.product.devicetype',
    ])
    return mobile ? /^(phone|tablet)$/.test(type) : type === '2in1'
  })
  assert.equal(
    matches.length,
    1,
    'Connect one matching device or set HARMONY_DEVICE'
  )
  device = matches[0]
}
const shell = (...args) => output(['-t', device, 'shell', ...args])
const bundle =
  'dev.dazzlabs.dazzui.' + (mobile ? 'mobile.playground' : 'playground')
const tag = mobile ? 'DazzSystemFontTest' : 'DazzSystemStyleTest'
const marker = mobile ? 'FONT_SNAPSHOT ' : 'SNAPSHOT '
const runId = 'fonts-' + Date.now()
const captureDirectory = process.argv[3]
  ? path.resolve(process.argv[3])
  : undefined
function capture(name) {
  if (!captureDirectory) return
  fs.mkdirSync(captureDirectory, { recursive: true })
  const remote =
    '/data/local/tmp/dazzui-fonts-' +
    (mobile ? 'mobile-' : 'desktop-') +
    name +
    '.png'
  shell('uitest', 'screenCap', '-p', remote)
  output([
    '-t',
    device,
    'file',
    'recv',
    remote,
    path.join(captureDirectory, name + '.png'),
  ])
}
const wait = ms => new Promise(resolve => setTimeout(resolve, ms))
const scale = report => (mobile ? report.settings.fontScale : report.scale)
async function snapshot(predicate) {
  const deadline = Date.now() + 20000
  while (Date.now() < deadline) {
    await wait(500)
    const reports = shell('hilog', '-x', '-T', tag)
      .split('\n')
      .flatMap(line => {
        const position = line.indexOf(marker)
        if (position < 0) return []
        try {
          return [JSON.parse(line.slice(position + marker.length))]
        } catch {
          return []
        }
      })
    const report = reports.findLast(
      item => item.runId === runId && predicate(item)
    )
    if (report) return report
  }
  throw new Error('Missing native font snapshot: ' + runId)
}
const layoutFile = '/data/local/tmp/dazzui-system-fonts.json'
function flatten(node) {
  return [node.attributes, ...(node.children ?? []).flatMap(flatten)]
}
async function click(id) {
  shell('uitest', 'dumpLayout', '-p', layoutFile)
  const node = flatten(JSON.parse(shell('cat', layoutFile))).find(
    item => item.id === id
  )
  assert.ok(node, 'Missing native node: ' + id)
  const [left, top, right, bottom] = node.bounds.match(/-?\d+/g).map(Number)
  assert.ok(right > left && bottom > top, 'Invisible native node: ' + id)
  shell(
    'uitest',
    'uiInput',
    'click',
    String(Math.round((left + right) / 2)),
    String(Math.round((top + bottom) / 2))
  )
}
try {
  shell('aa', 'force-stop', bundle)
  assert.match(
    shell(
      'aa',
      'start',
      '-b',
      bundle,
      '-a',
      'EntryAbility',
      '--pb',
      mobile ? 'systemFontTest' : 'systemStyleTest',
      'true',
      '--ps',
      'runId',
      runId
    ),
    /start ability successfully/
  )
  const initial = await snapshot(() => true)
  assert.equal(mobile ? initial.settings.followsSystem : initial.follows, true)
  assert.ok(
    Array.isArray(initial.settings?.fontFamilies ?? initial.fontFamilies)
  )
  // Set a known baseline, even when the device owner uses a larger system font.
  await click(mobile ? 'font-scale-1' : 'scale-1')
  const baseline = await snapshot(
    item =>
      scale(item) === 1 &&
      item.sequence > initial.sequence &&
      (mobile || Math.abs(item.vp - item.nativeVp) <= 1)
  )
  capture('scale-1')
  await click(mobile ? 'font-scale-2' : 'scale-2')
  const larger = await snapshot(
    item =>
      scale(item) === 2 &&
      item.sequence > baseline.sequence &&
      (mobile || Math.abs(item.vp - item.nativeVp) <= 1)
  )
  capture('scale-2')
  if (mobile) {
    assert.ok(larger.height > baseline.height, 'Native text must grow')
    assert.ok(
      larger.font16 > baseline.font16,
      'fp conversion must follow native scale'
    )
    for (const role of ['buttonHeight', 'fieldHeight']) {
      assert.ok(
        Number.isFinite(baseline[role]) && baseline[role] > 0,
        'Native control has a measured height: ' + role
      )
      assert.ok(
        larger[role] >= baseline[role],
        'Larger fonts must not shrink the native control: ' + role
      )
    }
  } else {
    assert.ok(larger.textVp > baseline.textVp, 'Adapted text must grow')
    assert.ok(
      Math.abs(baseline.vp - baseline.nativeVp) <= 1,
      'Default button must match native height'
    )
    assert.ok(
      Math.abs(larger.vp - larger.nativeVp) <= 1,
      'Large-font button must match native height'
    )
    assert.ok(larger.vp > baseline.vp, 'Large fonts must grow the button')
    assert.equal(larger.designVp, baseline.designVp)
    assert.equal(larger.designTextVp, baseline.designTextVp)
    assert.ok(
      Math.abs(larger.vp - larger.expected) <= 1,
      'Button must use the resolved native resource height'
    )
    assert.ok(larger.label.weight >= 100 && larger.label.weight <= 1000)
    for (const report of [baseline, larger]) {
      assert.ok(
        report.sidebarVp >= report.sidebarRequiredVp - 1,
        'Collapsed sidebar must fit the native control and side insets'
      )
      assert.ok(Math.abs(report.sidebarRowVp - report.vp) <= 1)
      assert.ok(Math.abs(report.sidebarRowHeight - report.vp) <= 1)
      assert.ok(Math.abs(report.designSidebarVp - 48) <= 1)
    }
  }
  await click(mobile ? 'font-theme' : 'change-theme')
  const recolored = await snapshot(
    item =>
      scale(item) === 2 &&
      item.sequence > larger.sequence &&
      (mobile ? item.brand !== larger.brand : item.surface !== larger.surface)
  )
  capture('scale-2-recolored')
  if (mobile) {
    assert.equal(recolored.height, larger.height)
    assert.equal(recolored.buttonHeight, larger.buttonHeight)
    assert.equal(recolored.fieldHeight, larger.fieldHeight)
  } else {
    assert.equal(recolored.vp, larger.vp)
    assert.equal(recolored.textVp, larger.textVp)
    assert.equal(recolored.label.size, larger.label.size)
    assert.equal(recolored.sidebarVp, larger.sidebarVp)
    assert.equal(recolored.sidebarRowVp, larger.sidebarRowVp)
  }
  const report = JSON.stringify(
    { device, baseline, larger, recolored },
    null,
    2
  )
  if (captureDirectory)
    fs.writeFileSync(path.join(captureDirectory, 'results.json'), report + '\n')
  console.log(report)
  console.log('Native system-font scaling and independent palette passed')
} finally {
  shell('aa', 'force-stop', bundle)
  shell(
    'aa',
    'start',
    '-b',
    bundle,
    '-a',
    'EntryAbility',
    ...(mobile ? [] : ['--ps', 'sizing', 'system'])
  )
}
