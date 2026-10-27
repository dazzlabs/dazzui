import assert from 'node:assert/strict'
import fs from 'node:fs'
import path from 'node:path'
import { execFile, execFileSync } from 'node:child_process'
import { checkShadowPaint, readPNG } from './paint.mjs'

const home =
  process.env.DEVECO_HOME ?? '/Applications/DevEco-Studio.app/Contents'
const bundled = path.join(home, 'sdk/default/openharmony/toolchains/hdc')
const hdc = process.env.HDC ?? (fs.existsSync(bundled) ? bundled : 'hdc')
const output = args =>
  execFileSync(hdc, args, {
    encoding: 'utf8',
    timeout: 15000,
    maxBuffer: 16 * 1024 * 1024,
  }).trim()
let device = process.env.HARMONY_DEVICE
if (!device) {
  const targets = output(['list', 'targets'])
    .split(/\s+/)
    .filter(id => id && id !== '[Empty]')
  const matches = targets.filter(
    id =>
      output([
        '-t',
        id,
        'shell',
        'param',
        'get',
        'const.product.devicetype',
      ]) === '2in1'
  )
  assert.equal(
    matches.length,
    1,
    'Connect one PC/2in1 device or set HARMONY_DEVICE'
  )
  device = matches[0]
}
const command = (...args) => output(['-t', device, ...args])
const shell = (...args) => command('shell', ...args)
const bundle = 'dev.dazzlabs.dazzui.playground'
const wait = ms => new Promise(resolve => setTimeout(resolve, ms))
const file = '/data/local/tmp/dazzui-gallery-test.json'
const flatten = node => [
  node.attributes,
  ...(node.children ?? []).flatMap(flatten),
]
const findNode = (node, predicate) =>
  predicate(node.attributes)
    ? node
    : (node.children ?? [])
        .map(child => findNode(child, predicate))
        .find(Boolean)
const findLabeledButton = (node, label) =>
  node.attributes.type === 'Button' &&
  (node.attributes.text === label ||
    flatten(node).some(item => item.type === 'Text' && item.text === label))
    ? node
    : (node.children ?? [])
        .map(child => findLabeledButton(child, label))
        .find(Boolean)
let startingGallery = false
function tree() {
  shell('uitest', 'dumpLayout', '-p', file)
  const current = JSON.parse(shell('cat', file))
  if (
    !startingGallery &&
    !findNode(current, node => node.id === 'document-column') &&
    !shell('pidof', bundle)
  )
    throw new Error(
      'The native gallery exited; inspect its crash log before continuing UI input'
    )
  return current
}
function box(node) {
  return node.bounds.match(/-?\d+/g).map(Number)
}
async function inspectNative(id, keys) {
  const requestId = 'inspector-' + Date.now()
  assert.ok(shell('pidof', bundle), 'Inspect only a running gallery')
  shell(
    'aa',
    'start',
    '-b',
    bundle,
    '-a',
    'EntryAbility',
    '--pb',
    'inspectOnly',
    'true',
    '--ps',
    'inspectorId',
    id,
    '--ps',
    'requestId',
    requestId,
    ...(keys ? ['--ps', 'inspectorKeys', keys.join(',')] : [])
  )
  for (let attempt = 0; attempt < 20; attempt++) {
    const chunks = new Map()
    for (const line of shell('hilog', '-x', '-T', 'DazzUIInspector').split(
      '\n'
    )) {
      const start = line.indexOf('{"requestId":')
      if (start < 0) continue
      const chunk = JSON.parse(line.slice(start))
      if (chunk.requestId === requestId && chunk.id === id)
        chunks.set(chunk.index, chunk)
    }
    const first = chunks.get(0)
    if (first && chunks.size === first.total) {
      const raw = Array.from(
        { length: first.total },
        (_, index) => chunks.get(index).data
      ).join('')
      assert.ok(raw, 'Native inspector found ' + id)
      const result = JSON.parse(raw)
      assert.ok(!result.error, result.error)
      return result
    }
    await wait(100)
  }
  throw new Error('Native inspector did not report ' + id)
}
// A menu subwindow must retain focus while measured. The window service's
// element dump reports layout precision without activating the main ability.
function inspectWindowSurface(surface, outputFile) {
  assert.match(surface.hostWindowId, /^\d+$/)
  const dump = shell(
    'hidumper',
    '-s',
    'WindowManagerService',
    '-a',
    '-w ' + surface.hostWindowId + ' -element'
  )
  fs.writeFileSync(outputFile, dump)
  const ancestors = []
  for (const block of dump.split(/\n\s*\|-> /)) {
    const depth = block.match(/\| Depth: (\d+)/)
    const size = block.match(
      /\| FrameRect: RectT \([^)]*\) - \[([\d.]+) x ([\d.]+)\]/
    )
    const position = block.match(/\| top: ([-\d.]+) left: ([-\d.]+)/)
    const paint = block.match(
      /\| PaintRect without transform: RectT \(([-\d.]+), ([-\d.]+)\) - \[([\d.]+) x ([\d.]+)\]/
    )
    if (!depth || !size || !position || !paint) continue
    while (ancestors.length && ancestors.at(-1).depth >= Number(depth[1]))
      ancestors.pop()
    const x = Number(position[2]),
      y = Number(position[1])
    const paintX = ancestors.reduce(
      (sum, node) => sum + node.paintOffset[0],
      Number(paint[1])
    )
    const paintY = ancestors.reduce(
      (sum, node) => sum + node.paintOffset[1],
      Number(paint[2])
    )
    const node = {
      type: block.match(/^(\w+)/)?.[1],
      depth: Number(depth[1]),
      layoutBounds: [x, y, x + Number(size[1]), y + Number(size[2])],
      paintOffset: [Number(paint[1]), Number(paint[2])],
      bounds: [
        paintX,
        paintY,
        paintX + Number(paint[3]),
        paintY + Number(paint[4]),
      ],
    }
    if (
      block.split('\n').some(line => line.trim() === '| compid: ' + surface.id)
    ) {
      const visibleBounds =
        surface.origBounds?.match(/-?\d+/g)?.map(Number) ?? box(surface)
      assert.ok(
        node.bounds.every(
          (value, index) => Math.abs(value - visibleBounds[index]) <= 1
        ),
        'The exact frame belongs to the visible accessibility surface'
      )
      return { ...node, ancestors }
    }
    ancestors.push(node)
  }
  throw new Error(
    'Window dump did not contain the visible surface: ' + surface.id
  )
}
async function reveal(predicate, label) {
  for (let attempt = 0; attempt < 18; attempt++) {
    const current = tree()
    const node = flatten(current).find(predicate)
    const viewport = box(
      findNode(current, item => item.id === 'document-column').attributes
    )
    const target = node?.origBounds?.match(/-?\d+/g)?.map(Number)
    if (
      node &&
      visible(node) &&
      (!target ||
        target.every((value, index) => Math.abs(value - box(node)[index]) <= 1))
    )
      return node
    const x = Math.round((viewport[0] + viewport[2]) / 2)
    const height = viewport[3] - viewport[1]
    const distance = target
      ? target[3] > viewport[3]
        ? Math.min(height * 0.5, target[3] - viewport[3] + 24)
        : -Math.min(height * 0.5, viewport[1] - target[1] + 24)
      : height * 0.5
    const start =
      distance > 0 ? viewport[1] + height * 0.8 : viewport[1] + height * 0.2
    shell(
      'uitest',
      'uiInput',
      'swipe',
      String(x),
      String(Math.round(start)),
      String(x),
      String(Math.round(start - distance))
    )
    await wait(200)
  }
  throw new Error('Missing fully visible native specimen: ' + label)
}
function visible(node) {
  const [left, top, right, bottom] = box(node)
  const original = node.origBounds?.match(/-?\d+/g)?.map(Number)
  return (
    node.visible === 'true' &&
    right > left &&
    bottom > top &&
    (!original || bottom - top >= (original[3] - original[1]) * 0.9)
  )
}
async function until(predicate, label, attempts = 16) {
  for (let i = 0; i < attempts; i++) {
    const current = tree()
    const result = predicate(current)
    if (result) return result
    await wait(250)
  }
  throw new Error('Native assertion timed out: ' + label)
}
async function launch(page, parameters = []) {
  startingGallery = true
  shell('aa', 'force-stop', bundle)
  assert.match(
    shell(
      'aa',
      'start',
      '-b',
      bundle,
      '-a',
      'EntryAbility',
      ...(page ? ['--ps', 'page', page] : []),
      ...parameters
    ),
    /start ability successfully/
  )
  await until(
    current => flatten(current).find(node => node.id === 'document-column'),
    'gallery startup: ' + page
  )
  startingGallery = false
  // Keep synthetic input inside this application's own, unobscured window.
  const current = tree()
  const nodes = flatten(current)
  const document = nodes.find(node => node.id === 'document-column')
  const title = nodes.find(
    node =>
      node.id === 'ContainerModalTitleRow' &&
      node.hostWindowId === document.hostWindowId
  )
  const maximize = nodes.find(
    node =>
      node.id === 'EnhanceMaximizeBtn' &&
      node.hostWindowId === document.hostWindowId
  )
  if (title && maximize) {
    const [left, top, right] = box(title)
    const display = box(current.attributes)
    if (left > display[0] || top > display[1] || right < display[2]) {
      const [l, t, r, b] = box(maximize)
      shell(
        'uitest',
        'uiInput',
        'click',
        String(Math.round((l + r) / 2)),
        String(Math.round((t + b) / 2))
      )
    }
  }
  // A restored desktop window can move after its first frame. Wait for the
  // document viewport to settle before injecting screen coordinates.
  let previousBounds = ''
  await until(current => {
    const bounds = flatten(current).find(
      node => node.id === 'document-column'
    )?.bounds
    const stable = bounds && bounds === previousBounds
    previousBounds = bounds
    return stable
  }, 'stable gallery viewport: ' + page)
}
async function click(predicate, label, scroll = false) {
  for (let i = 0; i < 12; i++) {
    const current = tree()
    const node = flatten(current).find(node => predicate(node) && visible(node))
    if (node) {
      const [left, top, right, bottom] = box(node)
      shell(
        'uitest',
        'uiInput',
        'click',
        String(Math.round((left + right) / 2)),
        String(Math.round((top + bottom) / 2))
      )
      await wait(250)
      return
    }
    if (!scroll) break
    const document = flatten(current).find(
      node => node.id === 'document-column'
    )
    assert.ok(document)
    const [left, top, right, bottom] = box(document)
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
    await wait(200)
  }
  throw new Error('Missing visible native control: ' + label)
}
const source = fs.readFileSync(
  new URL(
    '../../packages/arkts/dazzui/example/entry/src/main/ets/generated/Stories.ets',
    import.meta.url
  ),
  'utf8'
)
const stories = JSON.parse(
  JSON.parse(source.match(/JSON\.parse\((.*)\) as Story\[\]/)[1])
)
const restored = [
  'accordion',
  'avatar',
  'code',
  'link',
  'pill-tabs',
  'toggle',
  'checkbox',
  'menu',
  'popover',
  'preferences',
  'progress',
  'spinner',
  'form',
  'dialog',
  'select',
  'segmented-control',
  'combobox',
  'icon',
  'drawer',
  'switch',
  'shortcut-recorder',
  'otp-field',
  'tree',
  'sidebar',
  'preview-card',
  'slider',
  'toast',
  'tabs',
  'calendar',
  'date-range-picker',
  'window-frame',
]
const option = name => {
  const index = process.argv.indexOf(name)
  return index >= 0 ? process.argv[index + 1] : undefined
}
const checks = option('--checks')?.split(',')
const performed = new Set()
async function check(name, action) {
  if (!checks || checks.includes(name)) {
    await action()
    performed.add(name)
  }
}
const requested = option('--pages')
  ? option('--pages').split(',')
  : checks || process.argv.includes('--interactions-only')
    ? []
    : process.argv.includes('--all')
      ? stories.map(story => story.id)
      : restored
for (const id of requested) {
  await launch(id)
  const current = tree()
  const nodes = flatten(current)
  const expected = stories.find(story => story.id === id)
  assert.ok(expected)
  const seen = new Set()
  const mappingGaps = new Set()
  for (let step = 0; step < 16; step++) {
    const viewport = tree()
    const page = findNode(viewport, node => node.id === 'document-column')
    for (const node of flatten(page)) {
      if (node.id?.startsWith('example-')) seen.add(node.id)
      if (node.text === 'Mapping gaps') mappingGaps.add(node.text)
    }
    if (seen.size === expected.examples.length) break
    const [left, top, right, bottom] = box(page.attributes)
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
    await wait(200)
  }
  assert.deepEqual(
    [...seen].sort(),
    expected.examples.map(scene => 'example-' + scene.id).sort(),
    id
  )
  if (restored.includes(id)) assert.equal(mappingGaps.size, 0, id)
  const columns = ['component-list', 'document-column', 'theme-panel'].map(id =>
    nodes.find(node => node.id === id)
  )
  assert.ok(columns.every(Boolean), 'All three columns must exist')
  const [list, center, themes] = columns.map(box)
  assert.ok(
    list[2] <= center[0] + 2 && center[2] <= themes[0] + 2,
    'Columns must not overlap'
  )
  assert.ok(
    list[2] - list[0] >= 200 && themes[2] - themes[0] >= 280,
    'Sidebars must remain visible'
  )
  console.log('Rendered ' + id + ': ' + expected.examples.length + ' examples')
}
if (process.argv.includes('--renders-only')) process.exit(0)
async function captureWindowPaint(directory, theme, inactive, omarchy = false) {
  await launch('', [
    '--pb',
    'paintTest',
    'true',
    '--pb',
    'paintInactive',
    String(inactive),
    '--pb',
    'paintOmarchy',
    String(omarchy),
    '--ps',
    'themeName',
    theme,
  ])
  const current = tree()
  const stage = box(flatten(current).find(node => node.id === 'paint-stage'))
  const frame = findNode(current, node => node.id === 'paint-window')
  const bounds = box(frame.attributes)
  const surface = flatten(frame).find(node =>
    node.id?.startsWith('dazzui-window-surface-')
  )
  assert.ok(surface, 'The sample uses the actual window renderer')
  const attributes = (await inspectNative(surface.id)).$attrs
  const role = omarchy ? 'none' : inactive ? 'shadow.md' : 'shadow.xl'
  const stem = 'native-' + role.replace('.', '-') + '-' + theme
  const remote = '/data/local/tmp/dazzui-' + stem + '.png'
  shell('uitest', 'screenCap', '-p', remote)
  command('file', 'recv', remote, path.join(directory, stem + '.png'))
  const file = path.join(directory, stem + '.json')
  fs.writeFileSync(
    file,
    JSON.stringify(
      {
        theme,
        renderer: 'native-window',
        role,
        density: (stage[2] - stage[0]) / 640,
        stage,
        bounds,
        shadow: attributes.shadow,
        background: attributes.backgroundColor,
      },
      null,
      2
    )
  )
  return file
}

await check('shadow-paint', async () => {
  const theme = option('--paint-theme') ?? 'studio-light'
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification'
  )
  fs.mkdirSync(directory, { recursive: true })
  for (const inactive of [false, true])
    await captureWindowPaint(directory, theme, inactive)
  console.log(
    'Isolated native window paint captures and measured geometry saved to ' +
      directory
  )
})

await check('button-paint', async () => {
  const nativeButton = async address => {
    await reveal(node => node.id === address, address)
    const root = findNode(tree(), node => node.id === address)
    const button = flatten(root).find(node => node.type === 'Button')
    assert.ok(button?.id, 'Actual native button inside ' + address)
    return button
  }
  for (const theme of ['studio-light', 'studio-dark', 'bright-light']) {
    await launch('button', [
      '--ps',
      'sizing',
      'design',
      '--ps',
      'themeName',
      theme,
    ])
    for (const address of ['button/0/0', 'button/0/3', 'button/7/4']) {
      const button = await nativeButton(address)
      const actual = (await inspectNative(button.id)).$attrs.shadow
      assert.equal(
        actual.color,
        '#00000000',
        `${theme}/${address}: native foreground does not cast a duplicate shadow`
      )
      assert.equal(Number(actual.radius), 0)
    }
    await launch('icon-button', [
      '--ps',
      'sizing',
      'design',
      '--ps',
      'themeName',
      theme,
    ])
    const button = await nativeButton('icon-button/0/0')
    assert.equal(
      (await inspectNative(button.id)).$attrs.shadow.color,
      '#00000000',
      'Filled IconButton keeps its own flat recipe'
    )
  }
  await launch('toggle')
  const toggle = await nativeButton('toggle/0/4')
  const labelID = toggle.id + '-label'
  const before = (await inspectNative(labelID)).$attrs.fontWeight
  await click(node => node.id === toggle.id, 'latch the text toggle')
  const after = (await inspectNative(labelID)).$attrs.fontWeight
  assert.notEqual(
    after,
    before,
    'A latched Toggle carries the strong label weight'
  )
  assert.equal(
    (await inspectNative(toggle.id)).$attrs.shadow.color,
    '#00000000',
    'Latched Toggle stays flat'
  )
  await click(node => node.id === toggle.id, 'release the text toggle')
  assert.equal(
    (await inspectNative(labelID)).$attrs.fontWeight,
    before,
    'Releasing restores the profile label weight'
  )
  console.log(
    'Native filled-button lift, disabled/normal/IconButton flattening and latched Toggle typography passed'
  )
})

await check('shadows', async () => {
  const themes = JSON.parse(
    fs.readFileSync(
      path.resolve('packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json'),
      'utf8'
    )
  )
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/paint-' +
        Date.now()
  )
  fs.mkdirSync(directory, { recursive: true })
  const reports = []
  for (const theme of ['studio-light', 'studio-dark', 'bright-light']) {
    for (const inactive of [false, true]) {
      const file = await captureWindowPaint(directory, theme, inactive)
      const report = checkShadowPaint(file, themes)
      assert.ok(
        report.passed,
        'Actual native shadow pixels: ' + JSON.stringify(report)
      )
      reports.push(report)
    }
  }
  const plain = await captureWindowPaint(directory, 'studio-light', false, true)
  const report = checkShadowPaint(plain, themes)
  assert.ok(
    report.passed,
    'Omarchy has no outer shadow: ' + JSON.stringify(report)
  )
  reports.push(report)
  console.log(JSON.stringify(reports, null, 2))
  console.log(
    'Native Gaussian window shadow pixels passed on four straight edges in Studio light/dark and Bright light; Omarchy stays shadowless'
  )
})

await check('shadow-lifecycle', async () => {
  await launch('window-frame')
  const processID = shell('pidof', bundle)
  const source = JSON.parse(
    fs.readFileSync(
      path.resolve('packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json'),
      'utf8'
    )
  )
  const background = source['studio-light']['color.surface'].slice(1)
  const open = async (query, page) => {
    const search = findNode(tree(), node => node.id === 'component-search')
    const input = flatten(search).find(node => node.type === 'TextInput')
    assert.ok(input?.id)
    await click(node => node.id === input.id, 'gallery search before ' + page)
    shell('uitest', 'uiInput', 'keyEvent', '2070')
    const [left, top, right, bottom] = box(input)
    shell(
      'uitest',
      'uiInput',
      'inputText',
      String(Math.round((left + right) / 2)),
      String(Math.round((top + bottom) / 2)),
      query
    )
    // Commit Latin text when the user's active IME keeps it in pre-edit.
    shell('uitest', 'uiInput', 'keyEvent', '2054')
    // The active IME may alter Latin casing; gallery search ignores case.
    await until(
      root =>
        flatten(root).some(
          node =>
            node.id === input.id &&
            node.text.toLowerCase() === query.toLowerCase()
        ),
      'gallery query ' + query
    )
    await click(
      node => node.id === 'nav-' + page,
      'open ' + page + ' in the same process'
    )
    return input.id
  }
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification'
  )
  fs.mkdirSync(directory, { recursive: true })
  for (let cycle = 0; cycle < 2; cycle++) {
    await open('Button', 'button')
    await until(
      root =>
        !flatten(root).some(node =>
          node.id?.startsWith('dazzui-window-surface-')
        ),
      'window drawing scopes unmount'
    )
    const inputID = await open('WindowFrame', 'window-frame')
    const frame = await until(
      root => flatten(root).find(node => node.id === 'window-frame/0/0'),
      'window remounts'
    )
    await click(node => node.id === inputID, 'clear the gallery filter')
    shell('uitest', 'uiInput', 'keyEvent', '2070')
    await until(
      root =>
        flatten(root).some(node => node.id === inputID && node.text === ''),
      'gallery filter clears'
    )
    assert.equal(
      shell('pidof', bundle),
      processID,
      'Gallery process stays alive'
    )
    const [left, top, right, bottom] = box(frame)
    const density = (right - left) / 300
    const remote = '/data/local/tmp/dazzui-window-remount.png'
    shell('uitest', 'screenCap', '-p', remote)
    const file = path.join(directory, 'native-window-remount-' + cycle + '.png')
    command('file', 'recv', remote, file)
    const pixel = readPNG(fs.readFileSync(file)).pixel(
      Math.round(left - 5 * density),
      Math.round((top + bottom) / 2)
    )
    assert.ok(
      pixel
        .slice(0, 3)
        .some((channel, index) => channel < background[index] - 2),
      'A remounted native drawing node paints its shadow beyond the window bounds'
    )
  }
  console.log(
    'Native window shadow scopes unmount/remount twice in one process and repaint outside their bounds'
  )
})

await check('window-control-paint', async () => {
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/window-controls'
  )
  fs.mkdirSync(directory, { recursive: true })
  const themes = option('--paint-theme')
    ? [option('--paint-theme')]
    : ['studio-light', 'studio-dark', 'bright-light']
  const platforms = option('--paint-platform')
    ? [option('--paint-platform')]
    : ['macos', 'macos15']
  const states = option('--paint-state')
    ? [option('--paint-state')]
    : ['enabled', 'inactive', 'disabled', 'hidden']
  assert.ok(
    platforms.every(platform => ['macos', 'macos15'].includes(platform))
  )
  assert.ok(
    states.every(state =>
      ['enabled', 'inactive', 'disabled', 'hidden'].includes(state)
    )
  )
  for (const theme of themes)
    for (const platform of platforms)
      for (const state of states) {
        await launch('', [
          '--pb',
          'paintTest',
          'true',
          '--ps',
          'paintPopup',
          'window-controls-' + platform,
          '--ps',
          'paintWindowState',
          state,
          '--ps',
          'themeName',
          theme,
        ])
        const document = flatten(tree()).find(
          node => node.id === 'document-column'
        )
        const stage = box(
          flatten(tree()).find(node => node.id === 'window-control-stage')
        )
        const density = (stage[2] - stage[0]) / 640
        shell(
          'uinput',
          '-M',
          '-m',
          String(Math.round(stage[0] + 10 * density)),
          String(Math.round(stage[1] + 10 * density))
        )
        const nodes = flatten(tree())
        const controls = nodes.filter(node =>
          /^dazz-window-control-(close|minimize|maximize)$/.test(node.id)
        )
        assert.equal(controls.length, state === 'hidden' ? 0 : 3)
        const measurements = []
        const canvases = nodes.filter(node =>
          node.id.startsWith('dazz-window-light-canvas-')
        )
        for (const [index, control] of controls.entries()) {
          const native = await inspectNative(control.id)
          const { frame, reference } = native.$geometry
          const origin = [
            box(document)[0] - reference.x,
            box(document)[1] - reference.y,
          ]
          let paintBounds, paintNative
          if (canvases[index]) {
            paintNative = await inspectNative(
              canvases[index].id.replace('-canvas-', '-paint-')
            )
            const paintNode = nodes.find(
              node =>
                node.id === canvases[index].id.replace('-canvas-', '-paint-')
            )
            assert.ok(
              paintNode,
              'The native drawing node exposes its actual painted bounds'
            )
            const drawingFrame = box(paintNode)
            const inspectorPaint = paintNative.$rect
              .match(/-?\d+(?:\.\d+)?/g)
              .map(Number)
            assert.ok(
              drawingFrame.every(
                (value, index) => Math.abs(value - inspectorPaint[index]) <= 1
              ),
              'The public inspector and accessibility tree agree on the canvas paint rectangle'
            )
            const pad = Math.ceil(2 * density)
            paintBounds = [
              drawingFrame[0] + pad,
              drawingFrame[1] + pad,
              drawingFrame[0] + pad + frame.width,
              drawingFrame[1] + pad + frame.height,
            ]
          }
          measurements.push({
            kind: control.id.slice('dazz-window-control-'.length),
            enabled: control.enabled === 'true',
            bounds: [
              frame.x + origin[0],
              frame.y + origin[1],
              frame.x + origin[0] + frame.width,
              frame.y + origin[1] + frame.height,
            ],
            paintedBounds: box(control),
            paintBounds,
            paintNative,
            native,
          })
          assert.equal(control.enabled === 'true', state !== 'disabled')
        }
        const capture = hovered => {
          const stem =
            'native-window-controls-' +
            platform +
            '-' +
            state +
            '-' +
            theme +
            (hovered ? '-hover' : '')
          const remote = '/data/local/tmp/dazzui-' + stem + '.png'
          shell('uitest', 'screenCap', '-p', remote)
          command('file', 'recv', remote, path.join(directory, stem + '.png'))
          fs.writeFileSync(
            path.join(directory, stem + '.json'),
            JSON.stringify(
              {
                theme,
                platform,
                state,
                hovered,
                density,
                controls: measurements,
              },
              null,
              2
            )
          )
        }
        capture(false)
        if (controls.length) {
          const bounds = box(controls[0])
          const group = nodes.find(node =>
            node.id.startsWith('dazz-window-controls-group-')
          )
          assert.ok(group, 'The presentation controls expose their hover group')
          const groupBounds = box(group)
          assert.ok(
            Math.abs(
              groupBounds[3] - groupBounds[1] - (bounds[3] - bounds[1])
            ) <= 1,
            'The macOS hover group has the height of a traffic light'
          )
          shell(
            'uinput',
            '-M',
            '-m',
            String(Math.round((bounds[0] + bounds[2]) / 2)),
            String(Math.round(bounds[1] - 2 * density))
          )
          await until(root => {
            const glyphs = flatten(root).filter(node =>
              node.id.startsWith('dazz-window-control-glyph-')
            )
            return (
              glyphs.length === 3 &&
              glyphs.every(node => Number(node.opacity) === 0)
            )
          }, 'Moving above the traffic lights keeps the glyphs hidden')
          shell(
            'uinput',
            '-M',
            '-m',
            String(Math.round((bounds[0] + bounds[2]) / 2)),
            String(Math.round((bounds[1] + bounds[3]) / 2))
          )
          await until(root => {
            const glyphs = flatten(root).filter(node =>
              node.id.startsWith('dazz-window-control-glyph-')
            )
            return (
              glyphs.length === 3 &&
              glyphs.every(
                node => Number(node.opacity) === (state === 'disabled' ? 0 : 1)
              )
            )
          }, 'Hover reveals the enabled group glyphs and keeps disabled glyphs hidden')
          capture(true)
        }
        for (const control of controls)
          await click(
            node => node.id === control.id,
            'presentation ' + control.id
          )
        await until(
          root =>
            flatten(root).some(
              node =>
                node.id === 'window-control-count' &&
                node.text ===
                  (state === 'disabled' || state === 'hidden'
                    ? 'Close: 0; Minimize: 0; Maximize: 0'
                    : 'Close: 1; Minimize: 1; Maximize: 1')
            ),
          'Only enabled presentation window controls invoke callbacks'
        )
        console.log(
          'Captured ' +
            platform +
            ' ' +
            state +
            ' ' +
            theme +
            ' with native callback checks'
        )
      }
})

await check('window-title', async () => {
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/window-titles'
  )
  fs.mkdirSync(directory, { recursive: true })
  const story = stories.find(story => story.id === 'window-frame')
  for (const theme of option('--paint-theme')
    ? [option('--paint-theme')]
    : ['studio-light', 'studio-dark', 'bright-light']) {
    await launch('window-frame', [
      '--ps',
      'sizing',
      'design',
      '--ps',
      'themeName',
      theme,
    ])
    const frames = []
    for (const section of [0, 1, 4])
      for (const [index, frame] of story.examples[
        section
      ].content.items.entries()) {
        const platform = frame.fields.platform.text
        if (
          process.argv.includes('--rich-only') &&
          (section !== 4 || platform !== 'windows')
        )
          continue
        if (platform === 'omarchy') continue
        const id = `window-frame/${section}/${frame.address}`
        let scope
        for (let attempt = 0; attempt < 18; attempt++) {
          const root = tree()
          scope = findNode(root, node => node.id === id)
          if (scope && (section !== 4 || visible(scope.attributes))) break
          const document = flatten(root).find(
            node => node.id === 'document-column'
          )
          const [left, top, right, bottom] = box(document)
          shell(
            'uitest',
            'uiInput',
            'swipe',
            String(Math.round((left + right) / 2)),
            String(Math.round(top + (bottom - top) * 0.8)),
            String(Math.round((left + right) / 2)),
            String(Math.round(top + (bottom - top) * 0.3))
          )
          await wait(200)
        }
        assert.ok(scope, id)
        const nodes = flatten(scope)
        const title = nodes.find(node =>
          node.id.startsWith('dazz-window-title-text-')
        )
        assert.ok(title, 'Native title text in ' + id)
        const subtitle = nodes.find(node =>
          node.id.startsWith('dazz-window-subtitle-')
        )
        const iconScope =
          section === 4 && platform === 'windows'
            ? findNode(
                scope,
                node =>
                  node.id ===
                  `window-frame/4/${frame.fields.title.items[0].address}`
              )
            : undefined
        const icon = iconScope
          ? flatten(iconScope).find(node =>
              node.id.startsWith('dazz-icon-image-')
            )
          : undefined
        if (section === 4 && platform === 'windows')
          assert.ok(icon, 'The rich title exposes its native image')
        let inkPaint
        if (section === 4 && platform === 'windows') {
          const all = node => [node, ...(node.items ?? []).flatMap(all)]
          const bodySource = all(frame).find(node => node.name === 'Text')
          assert.ok(
            bodySource.items.every(node => node.kind === 'string'),
            'The source ink specimen is plain text'
          )
          const label = bodySource.items.map(node => node.text).join('')
          const body = nodes.find(
            node => node.type === 'Text' && node.text === label
          )
          assert.ok(
            body && visible(body),
            'The actual source body text is visible'
          )
          const file = 'native-window-title-ink-' + theme + '.png'
          const remote = '/data/local/tmp/' + file
          shell('uitest', 'screenCap', '-p', remote)
          command('file', 'recv', remote, path.join(directory, file))
          inkPaint = {
            file,
            title: { bounds: box(title), text: title.text },
            body: { bounds: box(body), text: body.text },
          }
        }
        frames.push({
          section,
          index,
          platform,
          title: { text: title.text, native: await inspectNative(title.id) },
          subtitle: subtitle
            ? { text: subtitle.text, native: await inspectNative(subtitle.id) }
            : undefined,
          icon: icon ? await inspectNative(icon.id) : undefined,
          inkPaint,
        })
      }
    fs.writeFileSync(
      path.join(directory, 'native-window-titles-' + theme + '.json'),
      JSON.stringify({ theme, frames }, null, 2)
    )
    const remote = '/data/local/tmp/dazzui-window-titles.png'
    shell('uitest', 'screenCap', '-p', remote)
    command(
      'file',
      'recv',
      remote,
      path.join(directory, 'native-window-titles-' + theme + '.png')
    )
    console.log('Measured native plain/rich title typography in ' + theme)
  }
})

await check('window-title-live', async () => {
  await launch('window-frame', [
    '--ps',
    'sizing',
    'design',
    '--ps',
    'themeName',
    'studio-light',
  ])
  const processID = shell('pidof', bundle)
  const frame = stories.find(story => story.id === 'window-frame').examples[4]
    .content.items[1]
  const id = `window-frame/4/${frame.address}`
  let scope
  for (let attempt = 0; attempt < 18; attempt++) {
    const current = tree()
    scope = findNode(current, node => node.id === id)
    if (scope && visible(scope.attributes)) break
    const [left, top, right, bottom] = box(
      flatten(current).find(node => node.id === 'document-column')
    )
    shell(
      'uitest',
      'uiInput',
      'swipe',
      String(Math.round((left + right) / 2)),
      String(Math.round(top + (bottom - top) * 0.8)),
      String(Math.round((left + right) / 2)),
      String(Math.round(top + (bottom - top) * 0.3))
    )
    await wait(200)
  }
  assert.ok(
    scope && visible(scope.attributes),
    'The actual rich title is on screen'
  )
  const pinID = `window-frame/4/${frame.fields.titlebar.fields.toolbar.items[0].address}`
  await click(
    node => node.id === pinID,
    'Pin before changing the live title palette'
  )
  const themes = JSON.parse(
    fs.readFileSync(
      'packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json',
      'utf8'
    )
  )
  const reports = []
  for (const [theme, trigger] of [
    ['studio-light', null],
    ['studio-dark', 'appearance-toggle'],
    ['bright-dark', 'theme-bright'],
    ['bright-light', 'appearance-toggle'],
    ['studio-light', 'theme-studio'],
  ]) {
    if (trigger)
      await click(node => node.id === trigger, 'Live title palette ' + theme)
    const root = tree()
    const current = findNode(root, node => node.id === id)
    assert.ok(current)
    const nodes = flatten(current)
    const title = nodes.find(node =>
      node.id.startsWith('dazz-window-title-text-')
    )
    const iconScope = findNode(
      current,
      node =>
        node.id === `window-frame/4/${frame.fields.title.items[0].address}`
    )
    const image = iconScope
      ? flatten(iconScope).find(
          node =>
            node.id.startsWith('dazz-icon-image-') && node.type === 'Image'
        )
      : undefined
    assert.ok(title && image)
    const expected =
      '#' +
      themes[theme]['color.content']
        .map(n => Math.round(n).toString(16).padStart(2, '0'))
        .join('')
        .toUpperCase()
    const titleInk = (await inspectNative(title.id)).$attrs.fontColor
    const imageInk = (await inspectNative(image.id)).$attrs.fillColor
    assert.equal(
      titleInk,
      expected,
      'The rich title updates without remounting: ' + theme
    )
    assert.equal(
      imageInk,
      expected,
      'The rich glyph updates without remounting: ' + theme
    )
    assert.ok(
      flatten(findNode(root, node => node.id === pinID)).some(
        node => node.selected === 'true'
      ),
      'The source Pin state survives the palette change'
    )
    assert.equal(shell('pidof', bundle), processID)
    reports.push({ theme, titleInk, imageInk, pinned: true })
  }
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/window-titles-final'
  )
  fs.mkdirSync(directory, { recursive: true })
  fs.writeFileSync(
    path.join(directory, 'live-palette.json'),
    JSON.stringify(reports, null, 2)
  )
  console.log(
    'Native rich title and glyph follow live appearance/family changes while Pin state survives in one process'
  )
})

await check('window-frame', async () => {
  await launch('window-frame', ['--ps', 'sizing', 'design'])
  const story = stories.find(story => story.id === 'window-frame')
  const sceneID = (index, node) => `window-frame/${index}/${node.address}`
  const descendants = node => [node, ...(node.items ?? []).flatMap(descendants)]
  const density = (() => {
    const r = box(
      findNode(tree(), node => node.id === 'component-list').attributes
    )
    return (r[2] - r[0]) / 272
  })()
  const near = (actual, expected, label) =>
    assert.ok(
      Math.abs(actual / density - expected) <= 2,
      `${label}: ${actual / density} vs ${expected}`
    )
  const reveal = async id => {
    for (let step = 0; step < 18; step++) {
      const current = tree()
      const found = findNode(current, node => node.id === id && visible(node))
      if (found) return found
      const [left, top, right, bottom] = box(
        findNode(current, node => node.id === 'document-column').attributes
      )
      const x = String(Math.round((left + right) / 2))
      shell(
        'uitest',
        'uiInput',
        'swipe',
        x,
        String(Math.round(top + (bottom - top) * 0.8)),
        x,
        String(Math.round(top + (bottom - top) * 0.3))
      )
      await wait(200)
    }
    throw new Error('Missing visible window specimen ' + id)
  }
  const scoped = (id, current = tree()) => {
    const node = findNode(current, node => node.id === id)
    assert.ok(node, id)
    return flatten(node)
  }
  if (!process.argv.includes('--workspace-only')) {
    const frames = story.examples[0].content.items
    for (const [index, height] of [
      134, 130, 134, 149, 149, 132, 126,
    ].entries()) {
      const id = sceneID(0, frames[index])
      const root = await reveal(id)
      const r = box(root.attributes)
      near(r[2] - r[0], 300, 'Platform frame width')
      near(r[3] - r[1], height, 'Platform frame height')
      const buttons = scoped(id).filter(node =>
        /^dazz-window-control-(close|minimize|maximize)$/.test(node.id)
      )
      assert.equal(buttons.length, index === 6 ? 0 : 3)
    }
    const slots = story.examples[4].content.items
    const pinID = index =>
      sceneID(4, slots[index].fields.titlebar.fields.toolbar.items[0])
    await reveal(pinID(0))
    await click(node => node.id === pinID(0), 'native titlebar Pin')
    await until(
      current =>
        scoped(pinID(0), current).some(node => node.selected === 'true'),
      'Pin controlled state'
    )
    for (const index of [1, 2]) {
      await reveal(pinID(index))
      assert.ok(
        !scoped(pinID(index)).some(node => node.selected === 'true'),
        'Sibling Pin remains off'
      )
    }
    const accessory = story.examples[5].content.fields.titlebar.fields.accessory
    const accessoryID = sceneID(5, accessory)
    await reveal(accessoryID)
    const boardButton = current => {
      const root = findNode(current, node => node.id === accessoryID)
      const search = node =>
        node.attributes.type === 'Button' &&
        flatten(node).some(item => item.text === 'Board')
          ? node
          : (node.children ?? []).map(search).find(Boolean)
      return root ? search(root) : undefined
    }
    const board = boardButton(tree())
    assert.ok(board)
    const r = box(board.attributes)
    shell(
      'uitest',
      'uiInput',
      'click',
      String(Math.round((r[0] + r[2]) / 2)),
      String(Math.round((r[1] + r[3]) / 2))
    )
    await until(
      current => boardButton(current)?.attributes.selected === 'true',
      'Native accessory selection'
    )
  }
  const workspace = story.examples[6].content.items[0]
  const nodes = descendants(workspace)
  const sidebar = nodes.find(node => node.name === 'Sidebar')
  const titlebar = nodes.find(node => node.name === 'WindowTitlebar')
  const glossaries = nodes.find(
    node =>
      node.name === 'SidebarMenuButton' && node.items[0].text === 'Glossaries'
  )
  const frameID = sceneID(6, workspace)
  const sidebarID = sceneID(6, sidebar)
  await reveal(sidebarID)
  const expanded = box(
    findNode(tree(), node => node.id === sidebarID).attributes
  )
  near(expanded[2] - expanded[0], 172, 'Workspace expanded sidebar')
  await click(
    node => node.id === sceneID(6, glossaries),
    'Workspace Glossaries'
  )
  await until(
    current =>
      scoped(sceneID(6, titlebar), current).some(
        node => node.text === 'Glossaries'
      ),
    'Workspace live title'
  )
  await click(
    node => node.id === sceneID(6, titlebar.fields.leading),
    'Workspace sidebar trigger'
  )
  await until(current => {
    const node = findNode(current, node => node.id === sidebarID)
    if (!node) return false
    const b = box(node.attributes)
    return Math.abs((b[2] - b[0]) / density - 48) <= 2
  }, 'Workspace collapsed icon sidebar')
  const searchID = sceneID(6, titlebar.fields.toolbar)
  const searchScope = current => findNode(current, node => node.id === searchID)
  const inputNode = current => {
    const root = searchScope(current)
    return root ? findNode(root, node => node.type === 'TextInput') : undefined
  }
  const searchBounds = box(searchScope(tree()).attributes)
  near(searchBounds[2] - searchBounds[0], 184, 'Inline search natural width')
  const input = inputNode(tree())
  assert.ok(input)
  const inputBounds = box(input.attributes)
  shell(
    'uitest',
    'uiInput',
    'inputText',
    String(Math.round((inputBounds[0] + inputBounds[2]) / 2)),
    String(Math.round((inputBounds[1] + inputBounds[3]) / 2)),
    '31415'
  )
  await until(
    current => inputNode(current)?.attributes.text === '31415',
    'Workspace search controlled value'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  await until(
    current => inputNode(current)?.attributes.text === '',
    'Search Escape clears its value'
  )
  assert.ok(
    scoped(sceneID(6, titlebar)).some(node => node.text === 'Glossaries'),
    'Search retains the live pane title'
  )
  const fixed = box(findNode(tree(), node => node.id === frameID).attributes)
  near(fixed[3] - fixed[1], 384, 'Workspace fixed outer height')
  if (!process.argv.includes('--workspace-only')) {
    const sizing = descendants(story.examples[8].content).filter(
      node => node.name === 'WindowFrame'
    )
    assert.equal(sizing.length, 3)
    const tokens = JSON.parse(
      fs.readFileSync(
        'packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json',
        'utf8'
      )
    )['studio-light']
    const width = tokens['frame.window.width']
    const expected = [
      width.value * (width.unit === 'rem' ? 16 : 1),
      sizing[1].fields.minWidth.number,
      sizing[2].fields.width.number,
    ]
    for (const [index, frame] of sizing.entries()) {
      const node = await reveal(sceneID(8, frame))
      const r = box(node.attributes)
      near(r[2] - r[0], expected[index], 'Source sizing width ' + index)
      if (frame.fields.maxHeight)
        near(
          r[3] - r[1],
          frame.fields.maxHeight.number,
          'Source max-height clipping'
        )
    }
  }
  console.log(
    process.argv.includes('--workspace-only')
      ? 'Native window workspace title, sidebar, fixed height, search width/input and Escape checks passed'
      : 'Native window platform geometry, scoped slots, accessory and workspace checks passed'
  )
})
await check('date-range-picker', async () => {
  await launch('date-range-picker', ['--ps', 'sizing', 'design'])
  const story = stories.find(story => story.id === 'date-range-picker')
  const captionID =
    'date-range-picker/0/' + story.examples[0].content.items[1].address
  const sheet = (current = tree()) =>
    findNode(current, node => /^dazz-date-range-\d+-sheet$/.test(node.id))
  const caption = (current = tree()) => {
    const node = findNode(current, node => node.id === captionID)
    return node?.attributes.text || node?.attributes.description
  }
  const nodes = (current = tree()) => {
    const root = sheet(current)
    return root ? flatten(root) : []
  }
  const keys = (...codes) =>
    shell('uitest', 'uiInput', 'keyEvent', ...codes.map(String))
  const sourceNodes = node => [node, ...(node.items ?? []).flatMap(sourceNodes)]
  const sheetButton = (label, current = tree()) => {
    const panel = sheet(current)
    return panel ? findLabeledButton(panel, label) : undefined
  }
  const press = async label => {
    const node = sheetButton(label)
    assert.ok(
      node && visible(node.attributes),
      'Visible picker button: ' + label
    )
    const [l, t, r, b] = box(node.attributes)
    shell(
      'uitest',
      'uiInput',
      'click',
      String(Math.round((l + r) / 2)),
      String(Math.round((t + b) / 2))
    )
    await wait(250)
  }
  const open = async label => {
    const controls = story.examples.flatMap((scene, index) =>
      sourceNodes(scene.content)
        .filter(
          node =>
            node.name === 'DateRangePicker' &&
            node.fields?.['aria-label']?.text === label
        )
        .map(node => `date-range-picker/${index}/${node.address}`)
    )
    assert.ok(controls.length, 'Source-authored picker label: ' + label)
    await click(
      node => controls.includes(node.id),
      'open picker ' + label,
      true
    )
    await until(current => sheet(current), 'native date picker popup ' + label)
  }
  const closed = () => until(current => !sheet(current), 'date picker closed')
  const time = (start, expected) =>
    until(current => {
      const panel = sheet(current)
      if (!panel) return false
      const control = findNode(panel, node =>
        node.id?.endsWith(start ? '-start-time' : '-end-time')
      )
      return control && flatten(control).some(node => node.text === expected)
    }, 'picker time ' + expected)
  const initial = caption()
  assert.ok(initial)
  const rail = box(
    findNode(tree(), node => node.id === 'component-list').attributes
  )
  const density = (rail[2] - rail[0]) / 272
  const near = (value, expected, label) =>
    assert.ok(
      Math.abs(value / density - expected) <= 2,
      `${label}: ${value / density} vs ${expected}`
    )
  await open('When')
  let bounds = box(sheet().attributes)
  near(bounds[2] - bounds[0], 404, 'One-month popup width')
  near(bounds[3] - bounds[1], 276, 'Default popup height')
  await until(
    current =>
      (() => {
        const panel = sheet(current)
        const popup =
          panel && findNode(panel, node => node.id.endsWith('-popup-focus'))
        return (
          popup &&
          popup.attributes.focused === 'true' &&
          !flatten(popup).some(
            node => node.type === 'Button' && node.focused === 'true'
          )
        )
      })(),
    'Touch opening focuses the popup itself'
  )
  await press('2 hours')
  assert.equal(caption(), initial, 'Duration must not publish before Done')
  await time(false, '16:30')
  await press('Cancel')
  await closed()
  assert.equal(caption(), initial)
  console.log(
    'DateRangePicker dimensions, initial focus and draft duration/Cancel passed'
  )
  await open('When')
  await time(false, '15:30')
  await click(node => node.id?.endsWith('-start-time'), 'open start time')
  await until(
    current => findNode(current, node => node.type === 'Menu'),
    'Native start-time menu'
  )
  keys(2013)
  keys(2054)
  await time(true, '14:45')
  await time(false, '15:45')
  await click(node => node.id?.endsWith('-end-time'), 'open end time')
  await until(
    current => findNode(current, node => node.type === 'Menu'),
    'Native end-time menu'
  )
  keys(2081)
  keys(2054)
  await time(true, '23:00')
  await time(false, '00:00')
  assert.equal(caption(), initial, 'Time changes remain a draft')
  await press('Cancel')
  await closed()
  console.log(
    'DateRangePicker coordinated start/end times and reopening passed'
  )
  await open('When')
  await press('All day')
  await until(current => {
    const panel = sheet(current)
    const collect = node => [node, ...(node.children ?? []).flatMap(collect)]
    const controls = panel
      ? collect(panel)
          .filter(node => /-(start|end)-time$/.test(node.attributes.id))
          .map(
            node => findNode(node, item => item.type === 'Button')?.attributes
          )
      : []
    return (
      controls.length === 2 && controls.every(node => node?.enabled === 'false')
    )
  }, 'All day disables both time lists')
  await press('Done')
  await closed()
  await until(
    current => caption(current)?.endsWith('(all day)'),
    'Done reaches the controlled caption'
  )
  const allDay = caption()
  console.log(
    'DateRangePicker all-day time disabling and controlled Done passed'
  )
  await open('When')
  await press('All day')
  await time(true, '14:30')
  await time(false, '15:30')
  await press('Quick pick')
  await click(node => node.text === 'Tomorrow', 'choose Tomorrow')
  assert.equal(caption(), allDay)
  keys(2070)
  await closed()
  assert.equal(
    caption(),
    allDay,
    'Escape discards the preset and all-day changes'
  )
  await open('When')
  await press('All day')
  await click(
    node =>
      node.type === 'Text' &&
      node.text === 'DateRangePicker' &&
      box(node)[0] > rail[2],
    'outside press on the document heading'
  )
  await closed()
  assert.equal(caption(), allDay, 'Outside press cannot publish')
  console.log(
    'DateRangePicker Escape and outside-press draft cancellation passed'
  )
  await open('Offsite')
  bounds = box(sheet().attributes)
  near(bounds[2] - bounds[0], 614, 'Two-month popup width')
  assert.equal(
    nodes().filter(node => /^dazz-calendar-\d+-[01]-\d+$/.test(node.id)).length,
    84
  )
  for (const label of ['30 min', '1 hour', '2 hours'])
    assert.equal(sheetButton(label)?.attributes.enabled, 'false')
  await press('Cancel')
  await closed()
  await open('Sprint')
  assert.ok(sheetButton('Working day'))
  await press('Quick pick')
  const menu = await until(
    current => findNode(current, node => node.type === 'Menu'),
    'preset menu'
  )
  for (const label of ['Today', 'This sprint', 'Next sprint'])
    assert.ok(flatten(menu).some(node => node.text === label))
  await click(
    node => node.text === 'Next sprint',
    'run the source sprint formula'
  )
  keys(2070)
  await closed()
  console.log(
    'Native DateRangePicker draft/Done/Cancel/Escape/outside, time coordination, all-day, custom presets and dimensions passed'
  )
})
await check('date-range-live', async () => {
  await launch('', [
    '--pb',
    'paintTest',
    'true',
    '--ps',
    'paintPopup',
    'date-range-live',
  ])
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/date-range-live'
  )
  fs.mkdirSync(directory, { recursive: true })
  const keys = (...codes) =>
    shell('uitest', 'uiInput', 'keyEvent', ...codes.map(String))
  const sheet = (root = tree()) =>
    findNode(root, node => /^dazz-date-range-\d+-sheet$/.test(node.id))
  const state = (root = tree()) =>
    JSON.parse(flatten(root).find(node => node.id === 'picker-live-state').text)
  const open = async (keyboard, mouse = false) => {
    if (keyboard) {
      await click(
        node => node.id === 'picker-live-before',
        'Activate keyboard focus before picker'
      )
      keys(2049)
      keys(2054)
    } else if (mouse) {
      const b = box(
        findNode(tree(), node => node.id === 'picker-live-trigger').attributes
      )
      shell(
        'uinput',
        '-M',
        '-m',
        String(Math.round((b[0] + b[2]) / 2)),
        String(Math.round((b[1] + b[3]) / 2)),
        '-c',
        '0'
      )
    } else
      await click(
        node => node.id === 'picker-live-trigger',
        'Open the controlled picker'
      )
    await until(root => sheet(root), 'Controlled picker is visible')
    await until(
      root =>
        flatten(sheet(root)).some(
          node =>
            node.id.endsWith(
              keyboard || mouse ? '-navigation' : '-popup-focus'
            ) && node.focused === 'true'
        ),
      'Picker receives focus for the opening input'
    )
    if (keyboard)
      await until(
        root =>
          flatten(root).some(
            node =>
              node.id.startsWith('dazzui-tooltip-surface-') &&
              node.text === 'Previous month'
          ),
        'Icon-button focus opens its label'
      )
  }
  const closed = () => until(root => !sheet(root), 'Controlled picker closes')
  const press = async label => {
    const target = findLabeledButton(sheet(), label)
    assert.ok(target, 'Sheet action: ' + label)
    await click(
      node => node.id === target.attributes.id,
      'Picker action: ' + label
    )
  }
  const snapshots = []
  const capture = async name => {
    const root = tree(),
      panel = sheet(root)
    const focused = []
    for (const node of flatten(root).filter(
      node => node.type === 'Button' && node.focused === 'true'
    )) {
      const native = await inspectNative(node.id)
      focused.push({ id: node.id, label: native.$attrs.accessibilityText })
    }
    const time = start => {
      const control =
        panel &&
        findNode(panel, node =>
          node.id.endsWith(start ? '-start-time' : '-end-time')
        )
      return control
        ? flatten(control).find(
            node => node.type === 'Text' && /^\d{2}:\d{2}$/.test(node.text)
          )?.text
        : null
    }
    snapshots.push({
      name,
      state: state(root),
      open: !!panel,
      focused,
      focusOwner: focused.length
        ? focused.some(
            node => node.id.endsWith('-trigger') && node.label === 'When'
          )
          ? 'trigger'
          : 'control'
        : panel &&
            flatten(panel).some(
              node =>
                node.id.endsWith('-popup-focus') && node.focused === 'true'
            )
          ? 'popup'
          : null,
      tooltips: flatten(root)
        .filter(
          node =>
            node.id.startsWith('dazzui-tooltip-surface-') &&
            node.visible === 'true'
        )
        .map(node => node.text),
      startTime: time(true),
      endTime: time(false),
      selectedDays: panel
        ? flatten(panel)
            .filter(
              node =>
                /^dazz-calendar-\d+-[01]-\d+$/.test(node.id) &&
                node.selected === 'true'
            )
            .map(node => Number(node.id.match(/-(\d+)$/)[1]))
        : [],
      trigger: (
        await inspectNative(
          flatten(
            findNode(root, node => node.id === 'picker-live-trigger')
          ).find(node => node.type === 'Button').id
        )
      ).$attrs.accessibilityDescription,
    })
    fs.writeFileSync(
      path.join(directory, 'native-date-range-live.json'),
      JSON.stringify(snapshots, null, 2)
    )
    const remote = '/data/local/tmp/dazzui-picker-live.png'
    shell('uitest', 'screenCap', '-p', remote)
    command(
      'file',
      'recv',
      remote,
      path.join(directory, 'native-picker-' + name + '.png')
    )
    console.log('Native picker checkpoint: ' + name)
  }
  await capture('initial')
  await open(true)
  await capture('opened')
  await press('2 hours')
  assert.equal(state().commits, 0)
  await capture('draft')
  await press('Cancel')
  await closed()
  await capture('cancelled')
  await open(false)
  await press('2 hours')
  await press('Done')
  await closed()
  assert.equal(state().commits, 1)
  await capture('accepted')
  await click(
    node => node.id === 'picker-live-refuse',
    'Refuse controlled value changes'
  )
  const fixed = state()
  await open(false)
  await press('30 min')
  await press('Done')
  await closed()
  assert.equal(state().commits, 2)
  assert.equal(state().end, fixed.end)
  assert.notEqual(state().proposedEnd, fixed.end)
  await capture('refused')
  await open(false)
  await capture('reopened-refused')
  keys(2070)
  await closed()
  await capture('escape')
  await open(false)
  keys(2049)
  await until(
    root =>
      flatten(root).some(
        node =>
          node.id.startsWith('dazzui-tooltip-surface-') &&
          node.text === 'Previous month'
      ),
    'Touch-to-keyboard navigation opens its label'
  )
  await capture('touch-tab-entry')
  keys(2070)
  assert.ok(sheet(), 'Escape dismisses the icon hint before its parent picker')
  await capture('hint-escape')
  keys(2070)
  await closed()
  await open(false)
  await press('All day')
  await click(
    node => node.id === 'picker-live-outside',
    'Outside action cancels the draft'
  )
  await closed()
  assert.equal(state().commits, 2)
  assert.equal(state().allDay, fixed.allDay)
  await capture('outside')
  await open(false, true)
  await capture('mouse-opened')
  keys(2070)
  await closed()
  shell('uinput', '-M', '-m', '20', '120')
  await open(false)
  const timeControl = flatten(sheet()).find(node =>
    node.id.endsWith('-start-time')
  )
  await click(
    node => node.id === timeControl.id,
    'Open a nested native time list'
  )
  await until(
    root => findNode(root, node => node.type === 'Menu'),
    'Nested time list is visible'
  )
  keys(2070)
  await until(
    root => !findNode(root, node => node.type === 'Menu'),
    'Escape closes the time list'
  )
  assert.ok(sheet(), 'A nested Escape retains the picker')
  await capture('nested-escape')
  keys(2070)
  await closed()
  await capture('nested-popup-escape')
  await open(true)
  for (let i = 0; i < 12; i++) keys(2049)
  await wait(300)
  await capture('tab-out')
  assert.equal(snapshots.at(-1).open, false)
  assert.deepEqual(
    snapshots.at(-1).focused.map(node => node.label),
    ['Outside action']
  )
  await open(true)
  keys(2047, 2049)
  await wait(300)
  await capture('reverse-tab-out')
  assert.equal(snapshots.at(-1).open, true)
  assert.equal(snapshots.at(-1).focusOwner, 'trigger')
  keys(2070)
  await closed()
  await capture('trigger-escape')
  await open(true)
  keys(2047, 2049)
  keys(2049)
  await wait(300)
  await capture('reverse-reentry')
  assert.equal(snapshots.at(-1).open, true)
  assert.deepEqual(
    snapshots.at(-1).focused.map(node => node.label),
    ['Previous month']
  )
  keys(2047, 2049)
  keys(2047, 2049)
  await wait(300)
  await capture('reverse-tab-before')
  assert.equal(snapshots.at(-1).open, false)
  assert.deepEqual(
    snapshots.at(-1).focused.map(node => node.label),
    ['Before']
  )
  console.log(
    'Native controlled picker commit/refusal, reopening and focus transitions captured'
  )
})

await check('date-range-tab-trace', async () => {
  await launch('', [
    '--pb',
    'paintTest',
    'true',
    '--ps',
    'paintPopup',
    'date-range-live',
  ])
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/date-range-final'
  )
  fs.mkdirSync(directory, { recursive: true })
  const keys = (...codes) =>
    shell('uitest', 'uiInput', 'keyEvent', ...codes.map(String))
  const panel = root =>
    findNode(root, node => /^dazz-date-range-\d+-sheet$/.test(node.id))
  await click(
    node => node.id === 'picker-live-before',
    'Focus before tab trace'
  )
  keys(2049)
  keys(2054)
  await until(root => panel(root), 'Open picker for native tab trace')
  const trace = []
  const capture = step => {
    const root = tree()
    const focused = flatten(root)
      .filter(node => node.focused === 'true')
      .sort((a, b) => b.hierarchy.length - a.hierarchy.length)[0]
    assert.ok(focused, 'A native focus leaf is present')
    const node = findNode(root, item => item.hierarchy === focused.hierarchy)
    const row = {
      step,
      open: !!panel(root),
      id: focused.id,
      type: focused.type,
      text: focused.text,
      content: flatten(node)
        .filter(item => item.type === 'Text')
        .map(item => item.text),
    }
    trace.push(row)
    console.log(JSON.stringify(row))
    fs.writeFileSync(
      path.join(directory, 'native-picker-tab-trace.json'),
      JSON.stringify(trace, null, 2)
    )
  }
  capture(0)
  for (let i = 1; i <= 16; i++) {
    keys(2049)
    await wait(50)
    capture(i)
  }
  if (panel(tree())) keys(2070)
})

await check('calendar', async () => {
  await launch('calendar', ['--ps', 'sizing', 'design'])
  const story = stories.find(story => story.id === 'calendar')
  const calendarNodes = node => [
    ...(node.name === 'Calendar' ? [node] : []),
    ...(node.items ?? []).flatMap(calendarNodes),
  ]
  const scope = (index, current = tree(), calendarIndex = 0) =>
    findNode(
      current,
      node =>
        node.id ===
        `calendar/${index}/${calendarNodes(story.examples[index].content)[calendarIndex].address}`
    )
  const cells = (index, current = tree(), calendarIndex = 0) => {
    const root = scope(index, current, calendarIndex)
    return root
      ? flatten(root).filter(
          node =>
            /^dazz-calendar-\d+-[01]-\d+$/.test(node.id) &&
            node.type === 'Button'
        )
      : []
  }
  const keys = (...codes) =>
    shell('uitest', 'uiInput', 'keyEvent', ...codes.map(String))
  const selected = (index, current = tree(), calendarIndex = 0) =>
    cells(index, current, calendarIndex).filter(
      node => node.selected === 'true'
    )
  const rail = box(
    findNode(tree(), node => node.id === 'component-list').attributes
  )
  const density = (rail[2] - rail[0]) / 272
  const near = (value, expected, label) =>
    assert.ok(
      Math.abs(value / density - expected) <= 2,
      `${label}: ${value / density} vs ${expected}`
    )
  let current = tree()
  let single = selected(0, current)[0]
  assert.ok(single, 'Controlled selected day is mounted')
  const originalID = single.id
  const rootBounds = box(scope(0, current).attributes)
  const cellBounds = box(single)
  near(rootBounds[2] - rootBounds[0], 208, 'Studio month width')
  near(rootBounds[3] - rootBounds[1], 230, 'Studio six-week height')
  near(cellBounds[2] - cellBounds[0], 28, 'Studio medium day width')
  near(cellBounds[3] - cellBounds[1], 28, 'Studio medium day height')
  await click(
    node => node.id === originalID,
    'focus selected Calendar day',
    true
  )
  keys(2015)
  const moved = await until(
    current =>
      cells(0, current).find(
        node => node.focused === 'true' && node.id !== originalID
      ),
    'Calendar ArrowRight focus'
  )
  assert.equal(selected(0)[0].id, originalID, 'Arrow navigation does not pick')
  keys(2054)
  await until(
    current => selected(0, current)[0]?.id === moved.id,
    'Calendar controlled selection updates'
  )
  keys(2049)
  await until(
    current => !cells(0, current).some(node => node.focused === 'true'),
    'Calendar grid leaves in one Tab'
  )
  keys(2047, 2049)
  await until(
    current =>
      cells(0, current).some(
        node => node.id === moved.id && node.focused === 'true'
      ),
    'Calendar Shift Tab restores the roving day'
  )
  keys(2069)
  await until(
    current =>
      cells(0, current).some(
        node => node.focused === 'true' && node.id !== moved.id
      ),
    'Calendar PageDown mounts and focuses the next month'
  )

  await click(node => node.text === 'A span', 'scroll to range Calendar', true)
  current = tree()
  const rangeCells = cells(1, current)
  const startIndex = rangeCells.findIndex(node => node.selected === 'true')
  assert.ok(startIndex >= 2)
  assert.equal(selected(1, current).length, 5)
  await click(
    node => node.id === rangeCells[startIndex - 2].id,
    'start an earlier Calendar range',
    true
  )
  await until(
    current => selected(1, current).length === 1,
    'Range start replaces the completed span'
  )
  keys(2014)
  keys(2054)
  await until(
    current => selected(1, current).length === 2,
    'Calendar range completes in reverse order'
  )

  await click(
    node => node.text === 'Two months, small',
    'scroll to two-month Calendar',
    true
  )
  await reveal(
    node =>
      node.id ===
      `calendar/2/${calendarNodes(story.examples[2].content)[0].address}`,
    'two complete month grids'
  )
  current = tree()
  const small = scope(2, current)
  assert.ok(small)
  const smallBounds = box(small.attributes)
  near(smallBounds[2] - smallBounds[0], 404, 'Two small months width')
  near(smallBounds[3] - smallBounds[1], 218, 'Small six-week height')
  assert.equal(cells(2, current).length, 84)
  assert.equal(
    flatten(small).filter(node =>
      /^dazz-calendar-\d+-(previous|next)$/.test(node.id)
    ).length,
    2
  )

  await click(
    node => node.text === 'Bounds and week start',
    'scroll to bounded Calendar',
    true
  )
  await reveal(
    node =>
      node.id ===
      `calendar/3/${calendarNodes(story.examples[3].content)[0].address}`,
    'bounded Calendar'
  )
  current = tree()
  const boundedCells = cells(3, current)
  const selectedIndex = boundedCells.findIndex(node => node.selected === 'true')
  assert.ok(selectedIndex >= 7)
  const fixed = boundedCells[selectedIndex].id
  const unavailable = boundedCells[selectedIndex - 7].id
  await click(node => node.id === unavailable, 'focus a day before min', true)
  keys(2054)
  assert.equal(
    selected(3)[0].id,
    fixed,
    'Bounds reject pointer and keyboard selection'
  )
  const sundayCells = cells(3, tree(), 1)
  assert.equal(sundayCells.length, 42)
  const firstSunday = await inspectNative(sundayCells[0].id)
  assert.equal(
    firstSunday.$attrs.accessibilityText,
    'Sunday, September 27, 2026',
    'Actual native day label preserves explicit locale and Sunday week start'
  )
  console.log(
    'Native Calendar geometry, controlled day, range, keyboard, bounds, two months and locale passed'
  )
})
await check('calendar-live', async () => {
  await launch('', [
    '--pb',
    'paintTest',
    'true',
    '--ps',
    'paintPopup',
    'calendar-live',
  ])
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/calendar-live'
  )
  fs.mkdirSync(directory, { recursive: true })
  const keys = (...codes) =>
    shell('uitest', 'uiInput', 'keyEvent', ...codes.map(String))
  const scope = (root = tree()) =>
    findNode(root, node => node.id === 'calendar-live-control')
  const days = (root = tree()) =>
    flatten(scope(root)).filter(
      node =>
        node.type === 'Button' && /^dazz-calendar-\d+-[01]-\d+$/.test(node.id)
    )
  const day = (label, root = tree(), index = 0) =>
    days(root).find(
      node =>
        dateLabel(node) === label &&
        node.id.match(/-(\d)-\d+$/)?.[1] === String(index)
    )
  const state = () =>
    JSON.parse(
      flatten(tree()).find(node => node.id === 'calendar-live-state').text
    )
  const focusedDay = (root = tree()) =>
    days(root).find(node => node.focused === 'true')
  const dateLabel = node =>
    node
      ? new Intl.DateTimeFormat('en-US', {
          dateStyle: 'full',
          timeZone: 'Asia/Shanghai',
        }).format(new Date(Number(node.id.match(/-(\d+)$/)[1])))
      : undefined
  const focused = async label =>
    until(
      root => dateLabel(focusedDay(root)) === label,
      'Calendar focus: ' + label
    )
  const snapshots = []
  const capture = async name => {
    const root = tree()
    const attributes = await inspectNative(
      'calendar-cells',
      days(root).map(node => node.id)
    )
    snapshots.push({
      name,
      state: JSON.parse(
        flatten(root).find(node => node.id === 'calendar-live-state').text
      ),
      titles: flatten(scope(root))
        .filter(
          node => node.type === 'Text' && /^[A-Z][a-z]+ \d{4}$/.test(node.text)
        )
        .map(node => node.text),
      days: days(root).map(node => ({
        id: node.id,
        label: attributes.find(cell => cell.id === node.id).label,
        tabIndex: attributes.find(cell => cell.id === node.id).tabIndex,
        selected: node.selected === 'true',
        focused: node.focused === 'true',
        bounds: box(node),
      })),
    })
    fs.writeFileSync(
      path.join(directory, 'native-calendar-live.json'),
      JSON.stringify(snapshots, null, 2)
    )
    const remote = '/data/local/tmp/dazzui-calendar-live.png'
    shell('uitest', 'screenCap', '-p', remote)
    command(
      'file',
      'recv',
      remote,
      path.join(directory, 'native-calendar-' + name + '.png')
    )
  }
  await capture('initial')
  await click(
    node => node.id === 'calendar-june',
    'Show a month without today or the selected day'
  )
  await click(
    node => node.id === 'calendar-before',
    'Focus before the calendar'
  )
  keys(2049)
  keys(2049)
  keys(2049)
  await focused('Tuesday, June 1, 2027')
  await capture('fallback')
  const first = focusedDay()
  assert.equal((await inspectNative(first.id)).$attrs.tabIndex, 0)
  keys(2049)
  await until(
    root =>
      flatten(root).some(
        node => node.id === 'calendar-after' && node.focused === 'true'
      ),
    'One Tab leaves the grid'
  )
  keys(2047, 2049)
  await focused('Tuesday, June 1, 2027')
  await click(node => node.id === 'calendar-two', 'Add the second month')
  await click(
    node => node.id === 'calendar-before',
    'Re-enter the two-month calendar'
  )
  keys(2049)
  keys(2049)
  await focused('Tuesday, June 1, 2027')
  assert.equal(days().length, 84)
  keys(2049)
  await until(
    root =>
      flatten(scope(root)).some(
        node => node.id.endsWith('-next') && node.focused === 'true'
      ),
    'Two-month Tab reaches the second header navigation after the grid'
  )
  keys(2049)
  await until(
    root =>
      flatten(root).some(
        node => node.id === 'calendar-after' && node.focused === 'true'
      ),
    'Two months still have one date Tab stop'
  )
  await capture('two-month-fallback')
  await click(node => node.id === 'calendar-two', 'Return to one month')
  await click(node => node.id === 'calendar-october', 'Return to October')
  await click(node => node.id === 'calendar-refuse', 'Refuse month proposals')
  const fixedMonth = state().month
  const requests = state().requests
  await click(node => node.id.endsWith('-next'), 'Propose a refused month')
  assert.equal(state().month, fixedMonth)
  assert.equal(state().requests, requests + 1)
  assert.ok(state().proposal > fixedMonth)
  await capture('refused-navigation')
  const last = day('Saturday, October 31, 2026')
  await click(node => node.id === last.id, 'Focus a bounded but navigable date')
  const selected = state().selected
  keys(2069)
  await wait(300)
  assert.equal(state().month, fixedMonth)
  assert.equal(
    state().selected,
    selected,
    'Month navigation never selects a disabled day'
  )
  assert.equal(
    focusedDay()?.id,
    last.id,
    'A refused month does not focus a nonexistent cell'
  )
  await capture('refused-keyboard')
  await click(node => node.id === 'calendar-refuse', 'Accept month proposals')
  await click(node => node.id === last.id, 'Refocus month-end')
  keys(2069)
  await focused('Monday, November 30, 2026')
  assert.equal(state().selected, selected)
  await capture('accepted-keyboard')
  await click(
    node => node.id === 'calendar-october',
    'Return to the pending range'
  )
  await click(
    node => node.id === 'calendar-range',
    'Preview a controlled pending range'
  )
  const preview = day('Thursday, October 22, 2026')
  const b = box(preview)
  shell(
    'uinput',
    '-M',
    '-m',
    String(Math.round((b[0] + b[2]) / 2)),
    String(Math.round((b[1] + b[3]) / 2))
  )
  await until(
    root => days(root).filter(node => node.selected === 'true').length === 3,
    'Mouse previews the pending range'
  )
  await capture('range-hover')
  shell('uinput', '-M', '-m', '20', '120')
  const startID = day('Tuesday, October 20, 2026').id
  await click(node => node.id === startID, 'Complete the pending range')
  await click(
    node => node.id === startID,
    'Restart the range at the focused day'
  )
  keys(2014)
  await focused('Monday, October 19, 2026')
  await until(
    root => days(root).filter(node => node.selected === 'true').length === 2,
    'Keyboard previews after the pointer leaves'
  )
  await capture('range-keyboard')
  keys(2054)
  assert.ok(state().end > state().start)
  await capture('range-complete')
  console.log(
    'Native Calendar fallback Tab entry, controlled month acceptance/refusal and range hover/keyboard transitions passed'
  )
})

await check('font-metrics', async () => {
  await launch('', [
    '--pb',
    'paintTest',
    'true',
    '--ps',
    'paintPopup',
    'font-metrics',
  ])
  const report = await until(root => {
    const text = flatten(root).find(
      node => node.id === 'font-metrics-report'
    )?.text
    return text && JSON.parse(text)
  }, 'Public font configuration and native text widths')
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/font-metrics'
  )
  fs.mkdirSync(directory, { recursive: true })
  fs.writeFileSync(
    path.join(directory, 'native-font-metrics.json'),
    JSON.stringify(report, null, 2)
  )
  assert.equal(report.measurements.length, 11)
  console.log(JSON.stringify(report.measurements))
})

await check('tabs', async () => {
  await launch('tabs', ['--ps', 'sizing', 'design'])
  const story = stories.find(story => story.id === 'tabs')
  const section = (index, current = tree()) =>
    findNode(current, node => node.id === 'example-tabs/' + index)
  const labels = (index, current = tree()) =>
    flatten(section(index, current))
      .filter(node => node.type === 'Text')
      .map(node => node.text)
  const rail = box(
    findNode(tree(), node => node.id === 'component-list').attributes
  )
  const density = (rail[2] - rail[0]) / 272
  for (const [index, example] of story.examples.entries()) {
    const header = await reveal(
      node => node.id === `tabs/${index}/${example.content.address}`,
      'authored tabs header ' + index
    )
    const [left, top, right, bottom] = box(header)
    assert.ok(
      Math.abs(
        (right - left) / density - example.content.fields.width.number
      ) <= 1,
      'Authored header width ' + index
    )
    assert.ok(
      Math.abs(
        (bottom - top) / density - example.content.fields.height.number
      ) <= 1,
      'Authored header height ' + index
    )
  }
  await reveal(node => node.id === 'tabs/2/0', 'overflow header')
  const more = flatten(section(2)).find(node => node.id?.endsWith('-more'))
  assert.ok(more, 'Native overflow menu trigger')
  const moreLabel = await inspectNative(more.id)
  assert.equal(moreLabel.$attrs.accessibilityText, '4 more')
  await click(node => node.id === more.id, 'overflow tabs menu')
  const menu = findNode(tree(), node => node.type === 'Menu')
  assert.ok(menu, 'Native overflow menu')
  const history = flatten(menu).find(node => node.text === 'History')
  assert.ok(history, 'Hidden tab remains in the menu')
  const [left, top, right, bottom] = box(history)
  shell(
    'uitest',
    'uiInput',
    'click',
    String(Math.round((left + right) / 2)),
    String(Math.round((top + bottom) / 2))
  )
  await until(
    current => labels(2, current).includes('History'),
    'hidden active tab becomes a full visible tab'
  )
  await until(
    current => !findNode(current, node => node.type === 'Menu'),
    'overflow selection closes native menu'
  )
  const selected = findLabeledButton(section(2), 'History')?.attributes
  assert.ok(selected)
  const [l, t, r, b] = box(selected)
  const original = selected.origBounds.match(/-?\d+/g).map(Number)
  assert.ok(
    Math.abs(r - l - (original[2] - original[0])) <= 1,
    'Promoted active tab remains fully visible'
  )
  const remaining = flatten(section(2)).find(node => node.id?.endsWith('-more'))
  assert.ok(
    remaining && visible(remaining),
    'Overflow remains available after promotion'
  )
  assert.ok(
    box(remaining)[0] >= r - 1,
    'Overflow cannot overlap the active tab'
  )
  shell(
    'uitest',
    'uiInput',
    'click',
    String(Math.round((l + r) / 2)),
    String(Math.round((t + b) / 2))
  )
  shell('uitest', 'uiInput', 'keyEvent', '2081')
  await until(
    current => labels(2, current).includes('Pull Requests'),
    'Home selects the first visible tab'
  )
  const close = await reveal(
    node => node.id?.endsWith('-a-close'),
    'selected document close'
  )
  const closeLabel = await inspectNative(close.id)
  assert.equal(closeLabel.$attrs.accessibilityText, 'Close Query 1')
  await click(node => node.id === close.id, 'selected document close', true)
  await until(
    current => !labels(3, current).includes('Query 1'),
    'document is removed from source state'
  )
  const afterClose = flatten(section(3))
  assert.ok(
    !afterClose.some(
      node => node.type === 'Button' && node.selected === 'true'
    ),
    'Closing does not invent an adjacent selection'
  )
  assert.ok(labels(3).includes('sessions'))
  await click(
    node =>
      node.id ===
      'tabs/3/' + story.examples[3].content.items[0].fields.trailing.address,
    'source-authored trailing action'
  )
  assert.ok(
    !labels(3).includes('Query 1'),
    'The source New button has no add handler'
  )
  console.log(
    'Native navigation tabs overflow, hidden selection, Home, removal and trailing checks passed'
  )
})

await check('button-surfaces', async () => {
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/button-surfaces'
  )
  fs.mkdirSync(directory, { recursive: true })
  const variants = [
    'normal',
    'recessed',
    'filled',
    'tinted',
    'outlined',
    'plain',
  ]
  const sizes = ['tiny', 'small', 'medium', 'large']
  const results = []
  for (const theme of ['studio-light', 'studio-dark', 'bright-light']) {
    await launch('', [
      '--pb',
      'paintTest',
      'true',
      '--ps',
      'paintPopup',
      'button-surfaces',
      '--ps',
      'themeName',
      theme,
    ])
    shell('uinput', '-M', '-m', '20', '120')
    const capture = async (state, entries, stem) => {
      await wait(220)
      const measured = []
      for (const entry of entries) {
        const id = 'surface-' + entry.size + '-' + entry.variant,
          native = await inspectNative(id)
        assert.ok(
          native.$paint,
          'The actual drawing arguments are inspectable: ' + id
        )
        assert.equal(
          native.$attrs.backgroundColor,
          '#00000000',
          'Native input does not paint a duplicate face'
        )
        measured.push({
          theme,
          state,
          ...entry,
          native,
          image: 'native-surface-' + theme + '-' + stem + '.png',
        })
      }
      const remote = '/data/local/tmp/dazzui-button-surfaces.png'
      shell('uitest', 'screenCap', '-p', remote)
      command('file', 'recv', remote, path.join(directory, measured[0].image))
      results.push(...measured)
      fs.writeFileSync(
        path.join(directory, 'native-button-surfaces.json'),
        JSON.stringify(results, null, 2)
      )
      console.log('Native button surfaces: ' + theme + '/' + stem)
    }
    await capture(
      'normal',
      sizes.flatMap(size => variants.map(variant => ({ size, variant }))),
      'normal'
    )
    for (const variant of variants) {
      const id = 'surface-small-' + variant,
        node = flatten(tree()).find(node => node.id === id),
        b = box(node)
      const x = Math.round((b[0] + b[2]) / 2),
        y = Math.round((b[1] + b[3]) / 2)
      shell('uinput', '-M', '-m', String(x), String(y))
      await capture('hovered', [{ size: 'small', variant }], 'hover-' + variant)
      const done = new Promise(resolve =>
        execFile(
          hdc,
          [
            '-t',
            device,
            'shell',
            'uinput',
            '-M',
            '-m',
            String(x),
            String(y),
            '-d',
            '0',
            '-i',
            '10000',
            '-u',
            '0',
          ],
          { timeout: 15000 },
          (error, stdout, stderr) => resolve({ error, stdout, stderr })
        )
      )
      try {
        await wait(500)
        await capture(
          'pressed',
          [{ size: 'small', variant }],
          'pressed-' + variant
        )
      } finally {
        const result = await done
        assert.ifError(result.error)
      }
    }
    await click(
      node => node.id === 'surface-disable',
      'Disable all native button profiles'
    )
    shell('uinput', '-M', '-m', '20', '120')
    await capture(
      'disabled',
      variants.map(variant => ({ size: 'small', variant })),
      'disabled'
    )
  }
  assert.equal(results.length, 126)
  console.log('Native button surface captures: ' + results.length)
})

await check('button-focus', async () => {
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/button-focus'
  )
  fs.mkdirSync(directory, { recursive: true })
  const results = []
  for (const theme of ['studio-light', 'studio-dark', 'bright-light']) {
    await launch('', [
      '--pb',
      'paintTest',
      'true',
      '--ps',
      'paintPopup',
      'keyboard-actions',
      '--ps',
      'themeName',
      theme,
    ])
    await click(
      node => node.id === 'keyboard-before',
      'Focus before the button'
    )
    shell('uinput', '-M', '-m', '20', '120')
    shell('uitest', 'uiInput', 'keyEvent', '2049')
    const node = await until(
      root =>
        flatten(root).find(
          node => node.id === 'keyboard-button' && node.focused === 'true'
        ),
      'Keyboard focus on the actual button'
    )
    const native = await inspectNative(node.id)
    const remote = '/data/local/tmp/dazzui-button-focus.png'
    shell('uitest', 'screenCap', '-p', remote)
    command(
      'file',
      'recv',
      remote,
      path.join(directory, 'native-focus-' + theme + '.png')
    )
    shell('uitest', 'uiInput', 'keyEvent', '2047', '2049')
    await until(
      root =>
        flatten(root).some(
          node => node.id === 'keyboard-before' && node.focused === 'true'
        ),
      'Blur the measured button'
    )
    shell('uitest', 'screenCap', '-p', remote)
    command(
      'file',
      'recv',
      remote,
      path.join(directory, 'native-unfocused-' + theme + '.png')
    )
    results.push({ theme, bounds: box(node), native, unfocused: true })
    fs.writeFileSync(
      path.join(directory, 'native-focus.json'),
      JSON.stringify(results, null, 2)
    )
    console.log('Native button focus capture: ' + theme)
  }
})

await check('button-pointer', async () => {
  await launch('', [
    '--pb',
    'paintTest',
    'true',
    '--ps',
    'paintPopup',
    'keyboard-actions',
  ])
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/pointer-actions'
  )
  fs.mkdirSync(directory, { recursive: true })
  const target = (kind, root = tree()) =>
    flatten(root).find(
      node =>
        node.type === 'Button' &&
        (kind === 'close'
          ? node.id.endsWith('-a-close')
          : node.id === 'keyboard-' + kind)
    )
  const results = []
  const capture = async (name, kind) => {
    await wait(160)
    const root = tree(),
      node = target(kind, root)
    const focused = flatten(root)
      .filter(node => node.type === 'Button' && node.focused === 'true')
      .map(button =>
        button.id === 'keyboard-icon'
          ? 'Settings'
          : button.id.endsWith('-a-close')
            ? 'Close Document 1'
            : flatten(findNode(root, node => node.id === button.id))
                .filter(node => node.type === 'Text')
                .map(node => node.text)
                .join('')
      )
    const measured = await inspectNative(node.id)
    const row = {
      name,
      state: JSON.parse(
        flatten(root).find(node => node.id === 'keyboard-actions-state').text
      ),
      focused,
      menu: !!findNode(root, node => node.type === 'Menu'),
      background: measured.$paint?.surface ?? measured.$attrs.backgroundColor,
    }
    const remote = '/data/local/tmp/dazzui-pointer-surface.png'
    const imageFile = path.join(directory, 'native-pointer-' + name + '.png')
    shell('uitest', 'screenCap', '-p', remote)
    command('file', 'recv', remote, imageFile)
    const bounds = box(node),
      density = measured.$geometry.density
    row.surfacePixel = readPNG(fs.readFileSync(imageFile)).pixel(
      Math.round(bounds[0] + 3 * density),
      Math.floor((bounds[1] + bounds[3]) / 2)
    )
    results.push(row)
    fs.writeFileSync(
      path.join(directory, 'native-pointer-actions.json'),
      JSON.stringify(results, null, 2)
    )
    console.log(JSON.stringify(row))
  }
  let pending = null
  const drag = points => {
    const started = Date.now()
    const args = ['-m', ...points[0].map(String), '-d', '0']
    for (const point of points.slice(1))
      args.push('-i', '10000', '-m', ...point.map(String))
    args.push('-i', '10000', '-u', '0')
    const done = new Promise(resolve =>
      execFile(
        hdc,
        ['-t', device, 'shell', 'uinput', '-M', ...args],
        { timeout: 45000 },
        (error, stdout, stderr) => resolve({ error, stdout, stderr })
      )
    )
    pending = done
    return {
      done,
      checkpoint: async index =>
        wait(Math.max(0, started + index * 10000 + 500 - Date.now())),
    }
  }
  const finish = async input => {
    const result = await input.done
    if (pending === input.done) pending = null
    assert.ifError(result.error)
  }
  try {
    for (const kind of ['button', 'icon', 'close']) {
      const b = box(target(kind)),
        x = Math.round((b[0] + b[2]) / 2),
        y = Math.round((b[1] + b[3]) / 2)
      const outside = [x, y + (b[3] - b[1]) + 24]
      shell('uinput', '-M', '-m', String(x), String(y))
      await capture(kind + '-hover', kind)
      let input = drag([[x, y], [x + 4, y], outside, [x, y]])
      await input.checkpoint(0)
      await capture(kind + '-down', kind)
      await input.checkpoint(1)
      await capture(kind + '-move-inside', kind)
      await input.checkpoint(2)
      await capture(kind + '-outside', kind)
      await input.checkpoint(3)
      await capture(kind + '-reentered', kind)
      await finish(input)
      await capture(kind + '-up', kind)
      input = drag([[x, y], outside])
      await input.checkpoint(1)
      await capture(kind + '-cancel-outside', kind)
      await finish(input)
      await capture(kind + '-cancelled', kind)
      shell('uinput', '-M', '-m', String(x), String(y))
      await capture(kind + '-hover-after-cancel', kind)
    }
  } finally {
    if (pending) {
      const result = await pending
      assert.ifError(result.error)
    }
  }
  assert.deepEqual(results.at(-1).state, { button: 1, icon: 1, closed: 1 })
  console.log('Native held-pointer action states captured: ' + results.length)
})

await check('keyboard-actions', async () => {
  await launch('', [
    '--pb',
    'paintTest',
    'true',
    '--ps',
    'paintPopup',
    'keyboard-actions',
  ])
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/keyboard-actions'
  )
  fs.mkdirSync(directory, { recursive: true })
  const keys = (...codes) =>
    shell('uitest', 'uiInput', 'keyEvent', ...codes.map(String))
  // Keep down/up in one uinput process. A fresh process has no pressed-key
  // item to release, so separate '-u' invocations do not test key-up delivery.
  let pending = null
  const held = (code, repeat = false) => {
    const started = Date.now()
    const done = new Promise(resolve =>
      execFile(
        hdc,
        [
          '-t',
          device,
          'shell',
          'uinput',
          '-K',
          '-d',
          String(code),
          '-i',
          '8000',
          ...(repeat ? ['-d', String(code), '-i', '8000'] : []),
          '-u',
          String(code),
        ],
        { timeout: 25000 },
        (error, stdout, stderr) => resolve({ error, stdout, stderr })
      )
    )
    pending = done
    return {
      done,
      checkpoint: async index =>
        wait(Math.max(0, started + index * 8000 + 500 - Date.now())),
    }
  }
  const finish = async input => {
    const result = await input.done
    if (pending === input.done) pending = null
    assert.ifError(result.error)
  }
  const target = (kind, root = tree()) =>
    flatten(root).find(
      node =>
        node.type === 'Button' &&
        (kind === 'close'
          ? node.id.endsWith('-a-close')
          : kind === 'more'
            ? node.id.endsWith('-more')
            : node.id === 'keyboard-' + kind)
    )
  const results = []
  const capture = async (name, kind) => {
    await wait(140)
    const root = tree(),
      node = target(kind, root),
      focused = []
    for (const button of flatten(root).filter(
      node => node.type === 'Button' && node.focused === 'true'
    )) {
      const texts = flatten(findNode(root, node => node.id === button.id))
        .filter(node => node.type === 'Text')
        .map(node => node.text)
        .join('')
      focused.push(
        button.id === 'keyboard-icon'
          ? 'Settings'
          : button.id.endsWith('-a-close')
            ? 'Close Document 1'
            : button.id.endsWith('-more')
              ? String(
                  4 -
                    flatten(
                      findNode(root, node => node.id === 'keyboard-tabs')
                    ).filter(
                      node => node.type === 'Button' && /-[abcd]$/.test(node.id)
                    ).length
                ) + ' more'
              : texts
      )
    }
    const row = {
      name,
      state: JSON.parse(
        flatten(root).find(node => node.id === 'keyboard-actions-state').text
      ),
      focused,
      menu: !!findNode(root, node => node.type === 'Menu'),
      background: await (async () => {
        const measured = await inspectNative(node.id)
        return measured.$paint?.surface ?? measured.$attrs.backgroundColor
      })(),
    }
    results.push(row)
    fs.writeFileSync(
      path.join(directory, 'native-keyboard-actions.json'),
      JSON.stringify(results, null, 2)
    )
    console.log(JSON.stringify(row))
  }
  const focus = async kind => {
    if (kind === 'close' || kind === 'more') {
      await click(
        node => node.type === 'Button' && node.id.endsWith('-a'),
        'Focus active tab'
      )
      keys(2049)
      if (kind === 'more') keys(2049)
    } else {
      await click(node => node.id === 'keyboard-before', 'Focus before action')
      keys(2049)
      if (kind === 'icon') keys(2049)
    }
    shell('uinput', '-M', '-m', '20', '120')
    assert.equal(target(kind).focused, 'true', kind + ' focus')
  }
  try {
    for (const kind of ['button', 'icon', 'close']) {
      await focus(kind)
      await capture(kind + '-rest', kind)
      let input = held(2050, true)
      await input.checkpoint(0)
      await capture(kind + '-space-down', kind)
      await input.checkpoint(1)
      await capture(kind + '-space-repeat', kind)
      await finish(input)
      await capture(kind + '-space-up', kind)
      input = held(2050)
      await input.checkpoint(0)
      keys(2049)
      await capture(kind + '-space-blur', kind)
      await finish(input)
      await capture(kind + '-space-cancelled', kind)
      await focus(kind)
      input = held(2054, true)
      await input.checkpoint(0)
      await capture(kind + '-enter-down', kind)
      await input.checkpoint(1)
      await capture(kind + '-enter-repeat', kind)
      await finish(input)
      await capture(kind + '-enter-up', kind)
    }
    await focus('more')
    await capture('more-rest', 'more')
    const input = held(2050)
    await input.checkpoint(0)
    await capture('more-space-down', 'more')
    await finish(input)
    await capture('more-space-up', 'more')
    keys(2070)
    await capture('more-escape', 'more')
  } finally {
    if (pending) {
      const result = await pending
      assert.ifError(result.error)
    }
  }
  assert.deepEqual(results.at(-1).state, { button: 3, icon: 3, closed: 3 })
  assert.equal(results.find(row => row.name === 'more-space-down').menu, false)
  assert.equal(results.find(row => row.name === 'more-space-up').menu, true)
  console.log('Native held-key action states captured: ' + results.length)
})

await check('tabs-actions', async () => {
  await launch('', [
    '--pb',
    'paintTest',
    'true',
    '--ps',
    'paintPopup',
    'tabs-actions',
  ])
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/tabs-actions'
  )
  fs.mkdirSync(directory, { recursive: true })
  const keys = (...codes) =>
    shell('uitest', 'uiInput', 'keyEvent', ...codes.map(String))
  const control = (root = tree()) =>
    findNode(root, node => node.id === 'tabs-actions-control')
  const action = (kind, root = tree()) =>
    flatten(control(root)).find(
      node =>
        node.type === 'Button' &&
        node.id.endsWith(kind === 'more' ? '-more' : '-a-close')
    )
  const state = (root = tree()) =>
    JSON.parse(
      flatten(root).find(node => node.id === 'tabs-actions-state').text
    )
  const tips = root =>
    flatten(root)
      .filter(node => node.id.startsWith('dazzui-tooltip-surface-'))
      .map(node => node.text)
  const hover = target => {
    const b = box(target)
    shell(
      'uinput',
      '-M',
      '-m',
      String(Math.round((b[0] + b[2]) / 2)),
      String(Math.round((b[1] + b[3]) / 2))
    )
  }
  const park = () => shell('uinput', '-M', '-m', '20', '120')
  const selected = (root = tree()) =>
    flatten(control(root))
      .filter(node => node.type === 'Button' && node.selected === 'true')
      .map(node => node.id.split('-').at(-1))
  const snapshots = []
  const capture = async (name, kind) => {
    const root = tree(),
      target = kind ? action(kind, root) : null
    let measured = null
    if (target) {
      const native = await inspectNative(target.id)
      const image = flatten(
        findNode(control(root), node => node.id === target.id)
      ).find(node => node.type === 'Image')
      const glyph = await inspectNative(image.id)
      measured = {
        label: native.$attrs.accessibilityText,
        background: native.$attrs.backgroundColor,
        ink: glyph.$attrs.fillColor,
        width: native.$geometry.frame.width / native.$geometry.density,
        height: native.$geometry.frame.height / native.$geometry.density,
        glyphWidth: glyph.$geometry.frame.width / glyph.$geometry.density,
        glyphHeight: glyph.$geometry.frame.height / glyph.$geometry.density,
        glyphBounds: box(image),
        backdrop: flatten(root).find(node => node.id === 'tabs-actions-header')
          .backgroundColor,
      }
    }
    const focused = []
    for (const node of flatten(root).filter(
      node => node.type === 'Button' && node.focused === 'true'
    ))
      focused.push((await inspectNative(node.id)).$attrs.accessibilityText)
    snapshots.push({
      name,
      state: state(root),
      selected: selected(root),
      tooltips: tips(root),
      focused,
      menu: !!findNode(root, node => node.type === 'Menu'),
      action: measured,
    })
    fs.writeFileSync(
      path.join(directory, 'native-tabs-actions.json'),
      JSON.stringify(snapshots, null, 2)
    )
    const remote = '/data/local/tmp/dazzui-tabs-actions.png'
    shell('uitest', 'screenCap', '-p', remote)
    command(
      'file',
      'recv',
      remote,
      path.join(directory, 'native-tabs-actions-' + name + '.png')
    )
    console.log('Native tabs action checkpoint: ' + name)
  }
  await capture('rest', 'close')
  hover(action('close'))
  await until(
    root => tips(root).includes('Close Document 1'),
    'Close button exposes its label'
  )
  await capture('hover-close', 'close')
  keys(2070)
  await until(
    root => tips(root).length === 0,
    'Escape dismisses the close hint'
  )
  assert.equal(state().closed, 0)
  await capture('escape-close', 'close')
  park()
  const first = flatten(control()).find(
    node => node.type === 'Button' && node.id.endsWith('-a')
  )
  await click(node => node.id === first.id, 'Focus the active document')
  keys(2049)
  await until(
    root =>
      action('close', root)?.focused === 'true' &&
      tips(root).includes('Close Document 1'),
    'Tab enters the close button and shows its hint'
  )
  await capture('focus-close', 'close')
  keys(2054)
  await until(
    root => state(root).closed === 1,
    'Enter closes the document exactly once'
  )
  assert.deepEqual(state().values, ['b', 'c', 'd'])
  assert.deepEqual(selected(), [], 'Closing does not choose a peer')
  assert.deepEqual(
    tips(tree()),
    [],
    'Removing the button also removes its hint'
  )
  await capture('closed')
  await click(
    node => node.id === 'tabs-actions-reset',
    'Reset the same mounted tabs'
  )
  await capture('reset')
  await click(
    node => node.id === 'tabs-actions-narrow',
    'Narrow the document strip'
  )
  await until(root => action('more', root), 'Overflow remains available')
  await capture('narrow', 'more')
  const moreLabel = (await inspectNative(action('more').id)).$attrs
    .accessibilityText
  hover(action('more'))
  await until(
    root => tips(root).includes(moreLabel),
    'More button exposes the current hidden count'
  )
  await capture('hover-more', 'more')
  keys(2070)
  await until(root => tips(root).length === 0, 'Escape dismisses the more hint')
  assert.ok(!findNode(tree(), node => node.type === 'Menu'))
  await capture('escape-more', 'more')
  park()
  await click(
    node => node.id === first.id,
    'Start traversal at the selected tab'
  )
  keys(2049)
  keys(2049)
  await until(
    root =>
      action('more', root)?.focused === 'true' &&
      tips(root).includes(moreLabel),
    'Tab reaches the overflow control'
  )
  await capture('focus-more', 'more')
  keys(2054)
  await until(
    root => findNode(root, node => node.type === 'Menu'),
    'Enter opens the overflow menu'
  )
  assert.deepEqual(tips(tree()), [], 'Opening the menu dismisses the hint')
  await capture('opened-menu', 'more')
  keys(2070)
  await until(
    root => !findNode(root, node => node.type === 'Menu'),
    'Escape closes the overflow menu'
  )
  await capture('menu-escape', 'more')
  await click(node => node.id === action('more').id, 'Open overflow with touch')
  const menu = findNode(tree(), node => node.type === 'Menu')
  const last = flatten(menu).find(node => node.text === 'Document 4')
  assert.ok(last)
  const b = box(last)
  shell(
    'uitest',
    'uiInput',
    'click',
    String(Math.round((b[0] + b[2]) / 2)),
    String(Math.round((b[1] + b[3]) / 2))
  )
  await until(
    root =>
      state(root).selected === 'd' &&
      !findNode(root, node => node.type === 'Menu'),
    'Hidden document is promoted'
  )
  await capture('selected-hidden', 'more')
  await click(
    node => node.id === 'tabs-actions-new',
    'The trailing action remains usable'
  )
  assert.equal(state().created, 1)
  await capture('trailing', 'more')
  await click(
    node => node.id === 'tabs-actions-reset',
    'Reset before controlled refusal'
  )
  await click(
    node => node.id === 'tabs-actions-refuse',
    'Refuse a proposed close'
  )
  await click(node => node.id === first.id, 'Focus the retained tab')
  keys(2049)
  keys(2054)
  await until(
    root => state(root).closed === 1,
    'The host received one close proposal'
  )
  assert.deepEqual(state().values, ['a', 'b', 'c', 'd'])
  assert.equal(
    action('close').focused,
    'true',
    'Refusal retains the actual close button focus'
  )
  await capture('refused-close', 'close')
  await click(node => node.id === 'tabs-actions-refuse', 'Allow changes again')
  await click(
    node => node.id === 'tabs-actions-toggle-close',
    'Remove close controls from the same strip'
  )
  await click(
    node => node.id === first.id,
    'Focus a tab without close controls'
  )
  keys(2049)
  await until(
    root => action('more', root)?.focused === 'true',
    'One Tab reaches more without a close control'
  )
  await capture('no-close-more', 'more')
  keys(2047, 2049)
  await until(
    root =>
      flatten(control(root)).some(
        node => node.id === first.id && node.focused === 'true'
      ),
    'Reverse Tab returns to the active tab'
  )
  await capture('no-close-reverse', 'more')
  keys(2047, 2049)
  await capture('no-close-before', 'more')
  assert.deepEqual(snapshots.at(-1).focused, ['Toggle close'])
  keys(2049)
  await capture('no-close-reentry', 'more')
  keys(2049)
  keys(2049)
  await capture('no-close-tail', 'more')
  assert.deepEqual(snapshots.at(-1).focused, ['New'])
  await click(
    node => node.id === 'tabs-actions-reset',
    'Reset before a fresh touch opening'
  )
  await click(
    node => node.id === 'tabs-actions-narrow',
    'Show overflow for pointer-only input'
  )
  await click(
    node => node.id === action('more').id,
    'Touch a previously unfocused overflow trigger'
  )
  await until(
    root => findNode(root, node => node.type === 'Menu'),
    'Touch opens the native menu'
  )
  await capture('touch-menu', 'more')
  const touchLast = flatten(
    findNode(tree(), node => node.type === 'Menu')
  ).find(node => node.text === 'Document 4')
  const touchBox = box(touchLast)
  shell(
    'uitest',
    'uiInput',
    'click',
    String(Math.round((touchBox[0] + touchBox[2]) / 2)),
    String(Math.round((touchBox[1] + touchBox[3]) / 2))
  )
  await until(
    root =>
      state(root).selected === 'd' &&
      !findNode(root, node => node.type === 'Menu'),
    'Touch selection returns to the trigger'
  )
  await capture('touch-selected', 'more')
  console.log(
    'Native Tabs close/more hints, chrome, keyboard activation, menu focus and trailing actions captured'
  )
})

await check('tabs-live', async () => {
  await launch('', [
    '--pb',
    'paintTest',
    'true',
    '--ps',
    'paintPopup',
    'tabs-live',
  ])
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/tabs'
  )
  fs.mkdirSync(directory, { recursive: true })
  const snapshots = []
  const inputs = {
    width: 560,
    value: 'alpha',
    renamed: false,
    wideCount: false,
    wideTail: false,
    showGamma: true,
    busy: true,
  }
  const capture = async name => {
    const root = tree()
    const control = findNode(root, node => node.id === 'tabs-live-control')
    const nativeTabs = flatten(control).filter(
      node =>
        node.type === 'Button' &&
        /^dazz-navigation-tabs-\d+-/.test(node.id) &&
        !node.id.endsWith('-close') &&
        !node.id.endsWith('-more')
    )
    const nodes = []
    for (const tab of nativeTabs)
      nodes.push({
        id: tab.id,
        bounds: box(tab),
        selected: tab.selected,
        text: flatten(findNode(control, node => node.id === tab.id))
          .filter(node => node.type === 'Text')
          .map(node => node.text),
        native: await inspectNative(tab.id),
      })
    const more = flatten(control).find(node => node.id.endsWith('-more'))
    snapshots.push({
      name,
      inputs: { ...inputs },
      header: box(flatten(root).find(node => node.id === 'tabs-live-header')),
      control: box(control.attributes),
      tabs: nodes,
      more: more
        ? { bounds: box(more), native: await inspectNative(more.id) }
        : undefined,
      trailing: box(
        flatten(control).find(node => node.id === 'tabs-live-trailing')
      ),
      state: flatten(root).find(node => node.id === 'tabs-live-state').text,
    })
    fs.writeFileSync(
      path.join(directory, 'native-tabs-live.json'),
      JSON.stringify(snapshots, null, 2)
    )
    const remote = '/data/local/tmp/dazzui-tabs-' + name + '.png'
    shell('uitest', 'screenCap', '-p', remote)
    command(
      'file',
      'recv',
      remote,
      path.join(directory, 'native-tabs-' + name + '.png')
    )
    return snapshots[snapshots.length - 1]
  }
  const wide = await capture('wide')
  assert.equal(wide.tabs.length, 5, 'All five tabs are available initially')
  const alpha = wide.tabs.find(tab => tab.id.endsWith('-alpha'))
  const b = alpha.bounds
  shell(
    'uinput',
    '-M',
    '-m',
    String(Math.round((b[0] + b[2]) / 2)),
    String(Math.round((b[1] + b[3]) / 2))
  )
  await until(
    root => flatten(root).some(node => node.text === 'Alpha details'),
    'Explicit title uses a native system hint'
  )
  shell('uinput', '-M', '-m', '20', '120')
  await click(
    node => node.id === 'tabs-width-200',
    'Shrink the same mounted tabs'
  )
  inputs.width = 200
  await until(
    root => flatten(root).some(node => node.id.endsWith('-more')),
    'Overflow after live shrink'
  )
  const narrow = await capture('narrow')
  assert.ok(
    narrow.tabs.some(tab => tab.id === alpha.id),
    'The active tab remains mounted across fitting'
  )
  const beta = narrow.tabs.find(tab => tab.id.endsWith('-beta'))
  const hover = tab =>
    shell(
      'uinput',
      '-M',
      '-m',
      String(Math.round((tab.bounds[0] + tab.bounds[2]) / 2)),
      String(Math.round((tab.bounds[1] + tab.bounds[3]) / 2))
    )
  hover(beta)
  await until(
    root =>
      flatten(root).some(node => node.type === 'Text' && node.text === 'Beta'),
    'Compact labels become native hover titles'
  )
  shell('uinput', '-M', '-m', '20', '120')
  await click(node => node.id.endsWith('-more'), 'Open live overflow')
  await click(node => node.text === 'Gamma', 'Promote a hidden tab')
  await until(
    root =>
      flatten(root).some(
        node =>
          node.id === 'tabs-live-state' &&
          node.text.startsWith('Selected: gamma')
      ),
    'Hidden selection changes the controlled value'
  )
  inputs.value = 'gamma'
  await capture('promoted')
  await click(
    node => node.id === 'tabs-width-560',
    'Grow without restarting the gallery'
  )
  inputs.width = 560
  await until(
    root => !flatten(root).some(node => node.id.endsWith('-more')),
    'All tabs return after live growth'
  )
  const grown = await capture('grown')
  await click(node => node.id === 'tabs-rename', 'Live label width change')
  await click(node => node.id === 'tabs-count', 'Live count width change')
  inputs.renamed = true
  inputs.wideCount = true
  const updated = await capture('updated')
  const renamed = updated.tabs.find(tab => tab.id.endsWith('-alpha'))
  const previous = grown.tabs.find(tab => tab.id.endsWith('-alpha'))
  assert.equal(
    renamed.native.$attrs.accessibilityText,
    'Alpha with a longer name',
    'Updated names reach the real native button'
  )
  assert.equal(
    renamed.native.$ID,
    previous.native.$ID,
    'Content updates preserve the native button instance'
  )
  assert.ok(
    updated.tabs.find(tab => tab.id.endsWith('-beta')).text.includes('400000'),
    'Updated counts reach the visible native label'
  )
  hover(updated.tabs.find(tab => tab.id.endsWith('-delta')))
  await wait(1000)
  assert.ok(
    !flatten(tree()).some(
      node => node.type === 'Text' && node.text === 'Delta'
    ),
    'An explicit empty title suppresses the compact fallback'
  )
  shell('uinput', '-M', '-m', '20', '120')
  hover(updated.tabs.find(tab => tab.id.endsWith('-epsilon')))
  await until(
    root => flatten(root).some(node => node.text === 'Unavailable tab'),
    'Disabled tabs retain their informational title'
  )
  shell('uinput', '-M', '-m', '20', '120')
  await click(node => node.id === 'tabs-tail', 'Live trailing width change')
  inputs.wideTail = true
  await capture('wide-trailing')
  await click(node => node.id === 'tabs-gamma', 'Remove the selected busy tab')
  inputs.showGamma = false
  await capture('removed')
  await wait(1600)
  await click(node => node.id === 'tabs-gamma', 'Restore the busy tab')
  inputs.showGamma = true
  await capture('restored')
  console.log(
    'Native Tabs live widths, content updates, trailing control, promotion and busy-item lifetime captured'
  )
})

await check('tabs-pulse', async () => {
  await launch('', [
    '--pb',
    'paintTest',
    'true',
    '--ps',
    'paintPopup',
    'tabs-live',
  ])
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/tabs'
  )
  fs.mkdirSync(directory, { recursive: true })
  const dotID = () =>
    flatten(tree()).find(node =>
      /^dazz-navigation-tabs-\d+-gamma$/.test(node.id)
    ).id + '-status'
  const readDot = async () => {
    const native = await inspectNative(dotID())
    const frame = native.$geometry.frame
    const color = native.$attrs.fill
    assert.match(color, /^#[A-Fa-f\d]{8}$/)
    return {
      native,
      frame,
      color: [3, 5, 7].map(i => parseInt(color.slice(i, i + 2), 16)),
    }
  }
  let dot = await readDot()
  const records = []
  const capture = async (phase, count) => {
    const samples = []
    for (let i = 0; i < count; i++) {
      await wait(117)
      const file = path.join(directory, `native-tabs-pulse-${phase}-${i}.png`)
      const remote = '/data/local/tmp/dazzui-tabs-pulse.png'
      const started = Date.now()
      shell('uitest', 'screenCap', '-p', remote)
      const ended = Date.now()
      command('file', 'recv', remote, file)
      const image = readPNG(fs.readFileSync(file))
      const x = Math.round(dot.frame.x + dot.frame.width / 2)
      const y = Math.round(dot.frame.y + dot.frame.height / 2)
      const foreground = image.pixel(x, y).slice(0, 3)
      const background = image.pixel(x, y - 12).slice(0, 3)
      const channel = [0, 1, 2].sort(
        (a, b) =>
          Math.abs(dot.color[b] - background[b]) -
          Math.abs(dot.color[a] - background[a])
      )[0]
      const opacity =
        (foreground[channel] - background[channel]) /
        (dot.color[channel] - background[channel])
      samples.push({ started, ended, foreground, background, opacity })
    }
    records.push({ phase, dot, samples })
    fs.writeFileSync(
      path.join(directory, 'native-tabs-pulse.json'),
      JSON.stringify(records, null, 2)
    )
    return samples
  }
  const varying = samples => {
    const values = samples.map(sample => sample.opacity)
    assert.ok(Math.min(...values) < 0.65, 'The rendered dot visibly dims')
    assert.ok(Math.max(...values) > 0.8, 'The rendered dot visibly brightens')
  }
  varying(await capture('running', 12))
  await click(node => node.id === 'tabs-busy', 'Stop the running dot')
  await wait(1600)
  const stopped = await capture('stopped', 3)
  assert.ok(
    stopped.every(sample => Math.abs(sample.opacity - 1) < 0.02),
    'Stopping restores opaque paint and remains stopped beyond one cycle'
  )
  await click(node => node.id === 'tabs-busy', 'Restart the same mounted dot')
  varying(await capture('restarted', 12))
  const removedID = dotID().replace(/-status$/, '')
  await click(node => node.id === 'tabs-gamma', 'Dispose a running dot')
  await wait(1600)
  assert.ok(
    !flatten(tree()).some(node => node.id === removedID),
    'The busy tab is removed'
  )
  await click(node => node.id === 'tabs-gamma', 'Mount the busy tab again')
  dot = await readDot()
  varying(await capture('remounted', 12))
  console.log(
    'Native Tabs pulse pixels dim, brighten, stop, restart and resume after remount'
  )
})

await check('toast-paint', async () => {
  const themes = JSON.parse(
    fs.readFileSync(
      path.resolve('packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json'),
      'utf8'
    )
  )
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/toast-paint'
  )
  fs.mkdirSync(directory, { recursive: true })
  const reports = []
  for (const theme of ['studio-light', 'studio-dark', 'bright-light']) {
    for (const long of [false, true]) {
      await launch('', [
        '--pb',
        'paintTest',
        'true',
        '--pb',
        'paintToast',
        'true',
        '--pb',
        'paintToastLong',
        String(long),
        '--ps',
        'themeName',
        theme,
      ])
      const stage = box(
        flatten(tree()).find(node => node.id === 'toast-paint-stage')
      )
      const density = (stage[2] - stage[0]) / 640
      const viewport = box(
        flatten(tree()).find(node => node.id === 'document-column')
      )
      await click(
        node => node.id === 'paint-toast-trigger',
        'raise real paint receipt'
      )
      const surfaceID = 'dazzui-toast-surface/paint-toast-viewport/paint-toast'
      await until(
        root => flatten(root).find(node => node.id === surfaceID),
        'isolated native toast surface'
      )
      await wait(themes[theme]['toast.transition.duration'].value + 100)
      const surface = findNode(tree(), node => node.id === surfaceID)
      const bounds = box(surface.attributes)
      const maximum = themes[theme]['toast.max-width']
      const maximumWidth = maximum.value * (maximum.unit === 'rem' ? 16 : 1)
      if (long)
        assert.ok(
          Math.abs((bounds[2] - bounds[0]) / density - maximumWidth) <= 1,
          'Long receipt respects the shared width cap'
        )
      if (long) {
        const nodes = flatten(surface)
        const title = nodes.find(
          node =>
            node.type === 'Text' &&
            node.text.startsWith('Your changes were saved')
        )
        const action = findLabeledButton(surface, 'Undo')
        const dismiss = nodes.find(
          node =>
            node.type === 'Button' &&
            node.id?.startsWith('dazzui-toast-dismiss/')
        )
        assert.ok(
          title && action && dismiss,
          'The full message, action and dismiss controls remain mounted'
        )
        const textBounds = (title.origBounds ?? title.bounds)
          .match(/-?\d+/g)
          .map(Number)
        const actionBounds = box(action.attributes)
        const dismissBounds = box(dismiss)
        const gap = themes[theme]['spacing.2.5']
        const gapPixels = gap.value * (gap.unit === 'rem' ? 16 : 1) * density
        assert.ok(
          textBounds[2] - textBounds[0] > 0,
          'Long message retains visible width'
        )
        assert.ok(
          actionBounds[0] - textBounds[2] >= gapPixels - 1 &&
            dismissBounds[0] - actionBounds[2] >= gapPixels - 1,
          'Truncation preserves both control gaps instead of pushing actions out'
        )
        assert.ok(
          actionBounds[3] <= bounds[3] && dismissBounds[2] <= bounds[2],
          'Native controls fit inside the capped receipt'
        )
      }
      const stem =
        'native-shadow-md-toast-' + (long ? 'long-' : 'short-') + theme
      const remote = '/data/local/tmp/dazzui-' + stem + '.png'
      shell('uitest', 'screenCap', '-p', remote)
      command('file', 'recv', remote, path.join(directory, stem + '.png'))
      const file = path.join(directory, stem + '.json')
      fs.writeFileSync(
        file,
        JSON.stringify(
          {
            theme,
            renderer: 'native-toast',
            role: 'shadow.md',
            radiusRole: 'radius.big',
            density,
            stage,
            viewport,
            bounds,
          },
          null,
          2
        )
      )
      const report = checkShadowPaint(file, themes)
      reports.push(report)
      assert.ok(
        report.passed,
        'Actual rounded native toast shadow pixels: ' + JSON.stringify(report)
      )
      if (long) {
        await click(
          node => node.text === 'Undo',
          'live action in the width-capped receipt'
        )
        await until(
          root =>
            flatten(root).some(
              node =>
                node.id === 'paint-toast-actions' && node.text === 'Actions: 1'
            ),
          'toast action executes exactly once'
        )
      } else {
        await click(
          node =>
            node.type === 'Button' &&
            node.id?.startsWith('dazzui-toast-dismiss/paint-toast-viewport/'),
          'dismiss the short receipt'
        )
      }
      await until(
        root => !flatten(root).some(node => node.id === surfaceID),
        'paint receipt unmounts'
      )
    }
  }
  console.log(JSON.stringify(reports, null, 2))
  console.log(
    'Short and width-capped native toast shadows pass the rounded Gaussian edge/corner probe in three themes; actions and dismissal stay live'
  )
})

await check('toast', async () => {
  await launch('toast')
  const viewport = (id, current = tree()) =>
    findNode(current, node => node.id === id)
  const receipts = (id, current = tree()) => {
    const root = viewport(id, current)
    return root
      ? flatten(root).filter(node => node.id?.startsWith(id + '/dazzui-toast-'))
      : []
  }
  const density = (() => {
    const rail = flatten(tree()).find(node => node.id === 'component-list')
    const bounds = box(rail)
    return (bounds[2] - bounds[0]) / 272
  })()
  await click(node => node.id === 'toast/0/6', 'danger toast button', true)
  await until(
    current => receipts('toast/0/7', current).length === 1,
    'native persistent toast'
  )
  const danger = receipts('toast/0/7')[0]
  const dangerBox = box(danger)
  assert.ok(
    Math.abs((dangerBox[3] - dangerBox[1]) / density - 38) <= 1,
    'React notice height'
  )
  const bottom = box(viewport('toast/0/7').attributes)
  assert.ok(
    Math.abs((bottom[3] - dangerBox[3]) / density - 16) <= 1,
    'Window-edge bottom padding'
  )
  const current = flatten(tree())
  const left = box(current.find(node => node.id === 'component-list'))[0]
  const right = box(current.find(node => node.id === 'theme-panel'))[2]
  assert.ok(
    Math.abs(bottom[0] - left) <= 3 && Math.abs(bottom[2] - right) <= 3,
    'Toast viewport spans the window rather than the scene'
  )
  await click(node => node.id === 'toast/1/1', 'archive action button', true)
  await until(
    current => receipts('toast/1/2', current).length === 1,
    'native action toast'
  )
  const actionBox = box(receipts('toast/1/2')[0])
  assert.ok(
    Math.abs((actionBox[3] - actionBox[1]) / density - 40) <= 1,
    'React action height'
  )
  await click(node => node.text === 'Undo', 'toast Undo action')
  await until(
    current => !viewport('toast/1/2', current),
    'action removes its own receipt'
  )
  assert.equal(
    receipts('toast/0/7').length,
    1,
    'Provider queues remain independent'
  )
  await wait(5500)
  assert.equal(
    receipts('toast/0/7').length,
    1,
    'Danger notices do not time out'
  )
  await click(node => node.id === 'toast/2/1', 'top toast button', true)
  await until(
    current => receipts('toast/2/2', current).length === 1,
    'native top toast'
  )
  const topBox = box(receipts('toast/2/2')[0])
  const topViewport = box(viewport('toast/2/2').attributes)
  assert.ok(
    Math.abs((topBox[1] - topViewport[1]) / density - 16) <= 1,
    'Window-edge top padding'
  )
  assert.ok(
    topBox[1] < dangerBox[1],
    'Top placement is independent of the story position'
  )
  await until(
    current => !viewport('toast/2/2', current),
    'ordinary notice times out',
    40
  )
  await click(
    node =>
      node.type === 'Button' &&
      node.id?.startsWith('dazzui-toast-dismiss/toast/0/7/'),
    'native toast dismiss'
  )
  await until(
    current => !viewport('toast/0/7', current),
    'dismiss removes the persistent receipt'
  )
  console.log(
    'Native toast placement, geometry, action, scope and timeout checks passed'
  )
})

await check('toast-keyboard', async () => {
  await launch('toast')
  const focused = (root, id) =>
    flatten(root).some(node => node.id === id && node.focused === 'true')
  await click(
    node => node.id === 'toast/0/6',
    'persistent keyboard toast',
    true
  )
  const source = findNode(tree(), node => node.id === 'toast/0/6')
  const sourceID = flatten(source).find(node => node.type === 'Button').id
  assert.ok(sourceID)
  assert.ok(
    focused(tree(), sourceID),
    'Showing a toast must retain the trigger focus'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2095')
  await until(
    root => focused(root, 'toast/0/7-focus'),
    'F6 enters the notification region'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2047', '2049')
  await until(
    root => focused(root, sourceID),
    'Shift+Tab returns to the original native button'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2095')
  await until(
    root => focused(root, 'toast/0/7-focus'),
    'F6 re-enters notifications'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2049')
  const receipt = await until(
    root =>
      flatten(root).find(
        node =>
          node.id?.startsWith('dazzui-toast-root-focus/toast/0/7/') &&
          node.focused === 'true'
      ),
    'Tab enters the first toast'
  )
  const receiptID = receipt.id.replace('dazzui-toast-root-focus/', '')
  assert.deepEqual(
    box(receipt),
    box(flatten(tree()).find(node => node.id === receiptID)),
    'The root focus target covers the complete receipt'
  )
  if (option('--toast-focus-image'))
    shell('snapshot_display', '-f', option('--toast-focus-image'))
  const dismissID = receipt.id.replace(
    'dazzui-toast-root-focus/',
    'dazzui-toast-dismiss/'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2049')
  await until(
    root => focused(root, dismissID),
    'Tab reaches its dismiss button'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2049')
  await until(
    root => focused(root, sourceID),
    'Tab past the last toast restores the original control'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2095')
  await until(
    root => focused(root, 'toast/0/7-focus'),
    'Region before keyboard dismissal'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2049')
  await until(root => focused(root, receipt.id), 'Toast root before Escape')
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  await until(
    root =>
      focused(root, sourceID) &&
      !flatten(root).some(node => node.id === 'toast/0/7'),
    'Escape dismisses the last toast and returns focus'
  )

  await click(node => node.id === 'toast/1/1', 'keyboard action toast', true)
  const archive = findNode(tree(), node => node.id === 'toast/1/1')
  const archiveID = flatten(archive).find(node => node.type === 'Button').id
  shell('uitest', 'uiInput', 'keyEvent', '2095')
  await until(
    root => focused(root, 'toast/1/2-focus'),
    'F6 enters the action viewport'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2049')
  const actionRoot = await until(
    root =>
      flatten(root).find(
        node =>
          node.id?.startsWith('dazzui-toast-root-focus/toast/1/2/') &&
          node.focused === 'true'
      ),
    'Action toast root is the first Tab stop'
  )
  const actionID = actionRoot.id.replace(
    'dazzui-toast-root-focus/',
    'dazzui-toast-action/'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2049')
  await until(root => focused(root, actionID), 'Tab reaches Undo')
  shell('uitest', 'uiInput', 'keyEvent', '2047', '2049')
  await until(
    root => focused(root, actionRoot.id),
    'Shift+Tab returns from Undo to its receipt'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2049')
  await until(root => focused(root, actionID), 'Undo can be reached again')
  shell('uitest', 'uiInput', 'keyEvent', '2054')
  await until(
    root =>
      focused(root, archiveID) &&
      !flatten(root).some(node => node.id === 'toast/1/2'),
    'Enter activates Undo, removes its receipt and restores the trigger'
  )

  await click(node => node.id === 'toast/0/6', 'toast before editing', true)
  const search = findNode(tree(), node => node.id === 'component-search')
  const searchID = flatten(search).find(node => node.type === 'TextInput').id
  await click(node => node.id === searchID, 'gallery search focus')
  await until(root => focused(root, searchID), 'Native editor takes focus')
  shell('uitest', 'uiInput', 'keyEvent', '2095')
  await until(
    root => focused(root, 'toast/0/7-focus'),
    'F6 works while an editor is focused'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2047', '2049')
  await until(
    root => focused(root, searchID),
    'Shift+Tab restores the exact editor'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2095')
  await until(
    root => focused(root, 'toast/0/7-focus'),
    'Re-enter after returning to the editor'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2049')
  await until(
    root =>
      flatten(root).some(
        node =>
          node.id?.startsWith('dazzui-toast-root-focus/toast/0/7/') &&
          node.focused === 'true'
      ),
    'Receipt before editor-return dismissal'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  await until(
    root =>
      focused(root, searchID) &&
      !flatten(root).some(node => node.id === 'toast/0/7'),
    'Last-toast dismissal returns to the editor'
  )

  await click(node => node.id === 'toast/2/1', 'ordinary top toast', true)
  shell('uitest', 'uiInput', 'keyEvent', '2095')
  await until(
    root => focused(root, 'toast/2/2-focus'),
    'F6 enters the top viewport'
  )
  await wait(5500)
  assert.ok(
    flatten(tree()).some(node => node.id === 'toast/2/2'),
    'Keyboard focus pauses the notice timeout'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2047', '2049')
  await until(
    root => !flatten(root).some(node => node.id === 'toast/2/2'),
    'Leaving notifications resumes the timer',
    40
  )
  console.log(
    'Native toast F6, region/root/action traversal, focus return, Escape and timer pause passed'
  )
})

await check('toast-keyboard-stack', async () => {
  await launch('toast')
  const viewport = 'toast/0/7'
  const receiptPrefix = viewport + '/'
  const ids = []
  for (let index = 0; index < 4; index++) {
    await click(
      node => node.id === 'toast/0/6',
      'persistent stack item ' + index,
      true
    )
    const added = await until(
      root =>
        flatten(root).find(
          node => node.id?.startsWith(receiptPrefix) && !ids.includes(node.id)
        ),
      'new stack receipt'
    )
    ids.push(added.id)
  }
  const rootID = id => 'dazzui-toast-root-focus/' + id
  const focused = (root, id) =>
    flatten(root).some(node => node.id === id && node.focused === 'true')
  const sourceID = flatten(
    findNode(tree(), node => node.id === 'toast/0/6')
  ).find(node => node.type === 'Button').id
  shell('uitest', 'uiInput', 'keyEvent', '2095')
  await until(
    root => focused(root, viewport + '-focus'),
    'F6 enters the nonempty stack'
  )
  for (const id of ids.slice(1).reverse()) {
    shell('uitest', 'uiInput', 'keyEvent', '2049')
    await until(
      root => focused(root, rootID(id)),
      'Tab follows source order: ' + id
    )
    shell('uitest', 'uiInput', 'keyEvent', '2049')
    await until(
      root => focused(root, 'dazzui-toast-dismiss/' + id),
      'Dismiss stop: ' + id
    )
  }
  shell('uitest', 'uiInput', 'keyEvent', '2049')
  await until(
    root => focused(root, sourceID),
    'The limited fourth receipt is skipped'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2095')
  await until(root => focused(root, viewport + '-focus'), 'Re-enter the stack')
  shell('uitest', 'uiInput', 'keyEvent', '2049')
  await until(
    root => focused(root, rootID(ids[3])),
    'Newest receipt before closing'
  )
  for (const next of [ids[2], ids[1], ids[0]]) {
    shell('uitest', 'uiInput', 'keyEvent', '2070')
    await until(
      root => focused(root, rootID(next)),
      'Closing moves focus to the next eligible receipt'
    )
  }
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  await until(
    root =>
      focused(root, sourceID) &&
      !flatten(root).some(node => node.id === viewport),
    'Closing the final receipt restores its original source'
  )
  console.log(
    'Native stacked toast source order, limited-item skipping and close focus transfer passed'
  )
})

await check('toast-keyboard-providers', async () => {
  await launch('toast')
  await click(node => node.id === 'toast/0/6', 'first provider notice', true)
  await click(node => node.id === 'toast/1/1', 'second provider notice', true)
  const archiveID = flatten(
    findNode(tree(), node => node.id === 'toast/1/1')
  ).find(node => node.type === 'Button').id
  const focused = (root, id) =>
    flatten(root).some(node => node.id === id && node.focused === 'true')
  shell('uitest', 'uiInput', 'keyEvent', '2095')
  await until(
    root => focused(root, 'toast/1/2-focus'),
    'The last nonempty provider receives F6 last'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2095')
  await until(
    root => focused(root, 'toast/0/7-focus'),
    'F6 skips the original region event target'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2095')
  await until(
    root => focused(root, 'toast/1/2-focus'),
    'F6 returns to the other live region'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2047', '2049')
  await until(
    root => focused(root, 'toast/0/7-focus'),
    'Return follows the captured previous region'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2047', '2049')
  // The first viewport's previous element was the other region on the second
  // F6. This deliberately follows Base UI's live listener semantics.
  await until(
    root => focused(root, 'toast/1/2-focus'),
    "Repeated F6 preserves each region's previous element"
  )
  await click(node => node.id === archiveID, 'leave both notification regions')
  await until(
    root => focused(root, archiveID),
    'Page controls remain clickable beneath multiple providers'
  )
  console.log(
    'Native multiple-provider F6 dispatch and previous-region focus passed'
  )
})

await check('toast-swipe', async () => {
  await launch('toast')
  const viewport = 'toast/0/7'
  const receipt = (root = tree()) =>
    flatten(root).find(node => node.id?.startsWith(viewport + '/dazzui-toast-'))
  const density = (() => {
    const bounds = box(
      flatten(tree()).find(node => node.id === 'component-list')
    )
    return (bounds[2] - bounds[0]) / 272
  })()
  const present = async () => {
    await click(
      node => node.id === 'toast/0/6',
      'persistent swipe notice',
      true
    )
    return await until(root => receipt(root), 'swipe receipt')
  }
  const point = node => {
    const [left, top, right, bottom] = box(node)
    return [
      Math.round(left + Math.min(40 * density, (right - left) * 0.35)),
      Math.round((top + bottom) / 2),
    ]
  }
  const swipe = (start, dx, dy) =>
    shell(
      'uitest',
      'uiInput',
      'swipe',
      ...start.map(String),
      String(Math.round(start[0] + dx * density)),
      String(Math.round(start[1] + dy * density))
    )
  let current = await present()
  const original = box(current)
  for (const [dx, dy] of [
    [20, 0],
    [-70, 0],
    [0, -70],
  ]) {
    swipe(point(receipt()), dx, dy)
    await wait(400)
    const remaining = receipt()
    assert.ok(
      remaining,
      'A small or disallowed-direction swipe preserves the receipt'
    )
    assert.equal(remaining.id, current.id)
    assert.ok(
      flatten(tree()).some(
        node =>
          node.id === 'dazzui-toast-root-focus/' + current.id &&
          node.focused === 'true'
      ),
      'Pressing the notification body focuses its whole receipt'
    )
    assert.ok(
      box(remaining).every(
        (value, index) => Math.abs(value - original[index]) <= 2
      ),
      'Cancelled movement returns to the resting rectangle'
    )
  }
  swipe(point(receipt()), 70, 0)
  await until(root => !receipt(root), 'Rightward touch swipe dismisses')
  current = await present()
  swipe(point(current), 0, 60)
  await until(root => !receipt(root), 'Downward touch swipe dismisses')
  current = await present()
  const before = box(current)
  const [x, y] = point(current)
  const endX = Math.round(x + 220 * density)
  const input = args =>
    new Promise(resolve =>
      execFile(
        hdc,
        ['-t', device, 'shell', 'uinput', '-M', ...args.map(String)],
        { timeout: 15000 },
        error => resolve(error)
      )
    )
  const held = input(['-g', x, y, endX, y, 7000])
  try {
    await wait(4700)
    const dragged = await until(
      root => {
        const node = receipt(root)
        return node && box(node)[0] - before[0] > 40 * density
          ? node
          : undefined
      },
      'Mouse drag moves the actual native receipt before release',
      3
    )
    assert.ok(
      box(dragged)[0] - before[0] > before[2] - before[0],
      "Native capture follows pointer movement beyond the receipt's original bounds"
    )
    assert.ok(
      Math.abs(box(dragged)[1] - before[1]) <= 2,
      'The horizontal lock preserves vertical position'
    )
    if (option('--toast-swipe-image'))
      shell('snapshot_display', '-f', option('--toast-swipe-image'))
  } finally {
    await held
  }
  const heldError = await held
  if (heldError) throw heldError
  await until(
    root => !receipt(root),
    'Mouse release beyond the threshold dismisses'
  )

  current = await present()
  const resting = box(current)
  const [rx, ry] = point(current)
  const shortDrag = await input([
    '-g',
    rx,
    ry,
    Math.round(rx + 20 * density),
    ry,
    400,
  ])
  if (shortDrag) throw shortDrag
  await wait(400)
  assert.ok(
    receipt(),
    'A held-button mouse drag below the threshold preserves the receipt'
  )
  assert.ok(
    box(receipt()).every(
      (value, index) => Math.abs(value - resting[index]) <= 2
    ),
    'A small mouse drag animates to rest'
  )
  await click(
    node => node.id?.startsWith('dazzui-toast-dismiss/' + viewport + '/'),
    'dismiss notice after small mouse drag'
  )
  await until(
    root => !receipt(root),
    'Dismiss remains clickable after a cancelled drag'
  )

  await click(
    node => node.id === 'toast/1/1',
    'action before pointer drag',
    true
  )
  const actionReceipt = root =>
    flatten(root).find(node => node.id?.startsWith('toast/1/2/dazzui-toast-'))
  const action = await until(
    root =>
      flatten(root).find(node =>
        node.id?.startsWith('dazzui-toast-action/toast/1/2/')
      ),
    'native action target'
  )
  const [al, at, ar, ab] = box(action)
  const ax = Math.round((al + ar) / 2),
    ay = Math.round((at + ab) / 2)
  shell(
    'uinput',
    '-M',
    '-g',
    String(ax),
    String(ay),
    String(Math.round(ax + 80 * density)),
    String(ay),
    '400'
  )
  assert.ok(
    actionReceipt(tree()),
    'Dragging from an action button does not swipe or activate the toast'
  )
  await click(node => node.id === action.id, 'Undo after button drag')
  await until(
    root => !actionReceipt(root),
    'The action still activates on a normal click'
  )
  current = await present()
  const dismissID = 'dazzui-toast-dismiss/' + current.id
  const dismiss = flatten(tree()).find(node => node.id === dismissID)
  const [dl, dt, dr, db] = box(dismiss)
  const dismissX = Math.round((dl + dr) / 2),
    dismissY = Math.round((dt + db) / 2)
  shell(
    'uinput',
    '-M',
    '-g',
    String(dismissX),
    String(dismissY),
    String(Math.round(dismissX + 80 * density)),
    String(dismissY),
    '400'
  )
  assert.ok(
    receipt(),
    'Dragging from the native dismiss button does not dismiss or swipe the receipt'
  )
  await click(
    node => node.id === dismissID,
    'normal dismiss after its ignored drag'
  )
  await until(
    root => !receipt(root),
    'Dismiss still works after a drag from its button'
  )
  console.log(
    'Native toast touch/mouse directions, live capture, threshold, return to rest and button isolation passed'
  )
})

await check('toast-touch-timers', async () => {
  await launch('toast')
  const receipt = root =>
    flatten(root).find(node => node.id?.startsWith('toast/2/2/dazzui-toast-'))
  await click(
    node => node.id === 'toast/2/1',
    'ordinary notice before touch hold',
    true
  )
  const node = await until(receipt, 'ordinary receipt before touch hold')
  const [left, top, right, bottom] = box(node)
  const x = Math.round(left + (right - left) * 0.35),
    y = Math.round((top + bottom) / 2)
  const held = new Promise(resolve =>
    execFile(
      hdc,
      [
        '-t',
        device,
        'shell',
        'uinput',
        '-T',
        '-d',
        String(x),
        String(y),
        '-i',
        '7000',
        '-u',
        String(x),
        String(y),
      ],
      { timeout: 12000 },
      error => resolve(error)
    )
  )
  try {
    await wait(5500)
    assert.ok(
      receipt(tree()),
      'A held touch pauses the 5-second notice even without a pan'
    )
  } finally {
    await held
  }
  const error = await held
  if (error) throw error
  await until(
    root => !receipt(root),
    'Releasing the touch resumes the remaining timeout',
    40
  )
  await click(
    node => node.id === 'toast/0/6',
    'persistent notice in the other provider',
    true
  )
  const other = await until(
    root =>
      flatten(root).find(node =>
        node.id?.startsWith('toast/0/7/dazzui-toast-')
      ),
    'other provider notice'
  )
  await click(
    node => node.id === 'toast/2/1',
    'ordinary notice before a mixed-input pause',
    true
  )
  shell('uinput', '-M', '-m', String(x), String(y))
  await wait(5500)
  assert.ok(receipt(tree()), 'A parked mouse pauses the ordinary notification')
  const [ol, ot, or, ob] = box(other)
  shell(
    'uitest',
    'uiInput',
    'click',
    String(Math.round(ol + (or - ol) * 0.35)),
    String(Math.round((ot + ob) / 2))
  )
  await until(
    root => !receipt(root),
    'Touching the other provider clears the parked-mouse pause',
    40
  )
  assert.ok(
    flatten(tree()).some(node => node.id === other.id),
    'The touched persistent notice remains visible'
  )
  console.log(
    'Native touch hold/release and cross-provider mixed-input notification timers passed'
  )
})

await check('toast-underlay', async () => {
  await launch('', ['--pb', 'modalTest', 'true'])
  const receipts = root =>
    flatten(root).filter(node =>
      node.id?.startsWith('toast-underlay-viewport/dazzui-toast-')
    )
  const count = (root = tree()) =>
    Number(
      flatten(root)
        .find(node => node.id === 'toast-underlay-count')
        .text.match(/\d+$/)[0]
    )
  await click(
    node => node.id === 'toast-underlay-show',
    'notification above a real control'
  )
  const first = await until(root => receipts(root)[0], 'underlay notification')
  const underlay = flatten(tree()).find(
    node => node.id === 'toast-underlay-action'
  )
  const [ul, ut, ur, ub] = box(underlay)
  const tap = (x, y) => {
    assert.ok(
      x > ul && x < ur && y > ut && y < ub,
      'The test point lies on the real background button'
    )
    shell(
      'uitest',
      'uiInput',
      'click',
      String(Math.round(x)),
      String(Math.round(y))
    )
  }
  const [left, top, right, bottom] = box(first)
  tap(left + (right - left) * 0.35, (top + bottom) / 2)
  assert.equal(
    count(),
    0,
    'Pressing the receipt does not activate the covered button'
  )
  const viewport = box(
    flatten(tree()).find(node => node.id === 'toast-underlay-viewport')
  )
  tap(viewport[0] + 4, (top + bottom) / 2)
  await until(
    root => count(root) === 1,
    'Horizontal viewport padding passes input to the actual button'
  )
  tap(left - 8, (top + bottom) / 2)
  await until(root => count(root) === 2, 'The visible left shadow passes input')
  tap(right + 8, (top + bottom) / 2)
  await until(
    root => count(root) === 3,
    'The visible right shadow passes input'
  )
  await click(
    node => node.id === 'toast-underlay-show',
    'second notice over the button'
  )
  const pair = await until(
    root => (receipts(root).length === 2 ? receipts(root) : undefined),
    'two underlay notices'
  )
  const boxes = pair.map(box).sort((a, b) => a[1] - b[1])
  assert.ok(boxes[1][1] > boxes[0][3], 'The stack has an actual gap')
  tap((left + right) / 2, (boxes[0][3] + boxes[1][1]) / 2)
  await until(
    root => count(root) === 4,
    'The gap between receipts passes input to the actual button'
  )
  const upper = box(
    flatten(tree()).find(node => node.id === 'toast-underlay-viewport')
  )
  tap((left + right) / 2, upper[1] + 4)
  await until(
    root => count(root) === 5,
    'The top viewport padding also passes input'
  )
  const dismiss = flatten(tree()).find(node =>
    node.id?.startsWith('dazzui-toast-dismiss/toast-underlay-viewport/')
  )
  await click(
    node => node.id === dismiss.id,
    'dismiss above the covered control'
  )
  await until(
    root => receipts(root).length === 1,
    'Only the chosen notice closes'
  )
  assert.equal(
    count(),
    5,
    'Dismiss does not click through to the background button'
  )
  console.log(
    'Native toast body/dismiss block covered controls while visible shadows, viewport padding and inter-receipt gaps pass input'
  )
})

await check('toast-modal', async () => {
  await launch('', ['--pb', 'modalTest', 'true'])
  await click(
    node => node.text === 'Edit notifications',
    'notification modal trigger'
  )
  await until(
    root =>
      flatten(root).some(node => node.text === 'Notification verification'),
    'notification modal'
  )
  await click(
    node => node.id === 'toast-probe-add',
    'persistent notice in a dialog'
  )
  const rootPrefix = 'dazzui-toast-root-focus/toast-probe-viewport/'
  const focused = (root, id) =>
    flatten(root).some(node => node.id === id && node.focused === 'true')
  shell('uitest', 'uiInput', 'keyEvent', '2095')
  await until(
    root => focused(root, 'toast-probe-viewport-focus'),
    'F6 from a modal enters its notifications'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2049')
  await until(
    root =>
      flatten(root).some(
        node => node.id?.startsWith(rootPrefix) && node.focused === 'true'
      ),
    'Modal notification receipt takes focus'
  )
  const modalTree = tree()
  const modalViewport = flatten(modalTree).find(
    node => node.id === 'toast-probe-viewport'
  )
  const modalWindow = flatten(modalTree).find(node => node.type === 'Dialog')
  assert.ok(
    Math.abs(box(modalViewport)[3] - box(modalWindow)[3]) <= 1,
    'A notification raised inside a modal stays on the window edge'
  )
  if (option('--toast-modal-image'))
    shell('snapshot_display', '-f', option('--toast-modal-image'))
  shell('uitest', 'uiInput', 'keyEvent', '2047', '2049')
  const add = findNode(tree(), node => node.id === 'toast-probe-add')
  const addID = flatten(add).find(node => node.type === 'Button').id
  await until(
    root => focused(root, addID),
    'Modal notification returns to its trigger'
  )
  for (const wrapper of ['toast-probe-editor', 'toast-probe-search']) {
    const nativeID = flatten(
      findNode(tree(), node => node.id === wrapper)
    ).find(node => node.type === 'TextInput').id
    await click(node => node.id === nativeID, wrapper)
    await until(
      root => focused(root, nativeID),
      'The modal editor receives its pointer input: ' + wrapper
    )
    shell('uitest', 'uiInput', 'keyEvent', '2095')
    await until(
      root => focused(root, 'toast-probe-viewport-focus'),
      'F6 from ' + wrapper
    )
    shell('uitest', 'uiInput', 'keyEvent', '2047', '2049')
    await until(root => focused(root, nativeID), 'Return to ' + wrapper)
  }
  for (const nativeID of ['toast-probe-native', 'toast-probe-native-editor']) {
    await click(node => node.id === nativeID, nativeID)
    await until(
      root => focused(root, nativeID),
      'The host control receives its pointer input: ' + nativeID
    )
    shell('uitest', 'uiInput', 'keyEvent', '2095')
    await until(
      root => focused(root, 'toast-probe-viewport-focus'),
      'F6 from ' + nativeID
    )
    shell('uitest', 'uiInput', 'keyEvent', '2047', '2049')
    await until(
      root => focused(root, nativeID),
      'Native scope restores ' + nativeID
    )
  }
  await click(
    node => node.text === 'Count action',
    'toast action above the modal mask'
  )
  await until(
    root =>
      flatten(root).some(
        node =>
          node.id === 'toast-probe-actions' && node.text === 'Toast actions: 1'
      ),
    'The receipt action receives pointer input above the modal mask'
  )
  assert.ok(
    flatten(tree()).some(node => node.text === 'Notification verification'),
    'Toast actions keep their owning dialog open'
  )
  console.log(
    'Native modal toast entry, pointer action and DazzUI/host editor/button focus return passed'
  )
})

await check('slider-paint', async () => {
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/slider'
  )
  fs.mkdirSync(directory, { recursive: true })
  const state = option('--paint-state') ?? 'rest'
  assert.ok(['rest', 'hover', 'focus'].includes(state))
  for (const theme of option('--paint-theme')
    ? [option('--paint-theme')]
    : [
        'studio-light',
        'studio-dark',
        'bright-light',
        'omarchy-tokyo-night-dark',
      ]) {
    await launch('', [
      '--pb',
      'paintTest',
      'true',
      '--ps',
      'paintPopup',
      'slider',
      '--ps',
      'themeName',
      theme,
    ])
    shell('uinput', '-M', '-m', '20', '120')
    if (state === 'focus') shell('uitest', 'uiInput', 'keyEvent', '2049')
    if (state === 'hover') {
      const small = findNode(tree(), node => node.id === 'slider-paint-small')
      const target = box(flatten(small).find(node => node.type === 'Slider'))
      shell(
        'uinput',
        '-M',
        '-m',
        String(Math.round(target[0] + (target[2] - target[0]) * 0.4)),
        String(Math.round((target[1] + target[3]) / 2))
      )
    }
    await wait(400)
    const root = tree()
    const stage = box(
      flatten(root).find(node => node.id === 'slider-paint-stage')
    )
    const sliders = []
    for (const key of [
      'small',
      'medium',
      'large',
      'min',
      'max',
      'range',
      'disabled',
    ]) {
      const node = findNode(root, node => node.id === 'slider-paint-' + key)
      assert.ok(node, 'Native Slider ' + key)
      const handles = []
      for (const handle of flatten(node).filter(node => node.type === 'Slider'))
        handles.push({
          bounds: box(handle),
          native: await inspectNative(handle.id),
        })
      sliders.push({ key, bounds: box(node.attributes), handles })
    }
    if (state === 'focus')
      assert.equal(
        sliders[0].handles[0].native.$attrs.focused,
        true,
        'Keyboard focus reaches the first native Slider'
      )
    const stem = 'native-slider-' + theme + '-' + state
    const remote = '/data/local/tmp/' + stem + '.png'
    shell('uitest', 'screenCap', '-p', remote)
    command('file', 'recv', remote, path.join(directory, stem + '.png'))
    fs.writeFileSync(
      path.join(directory, stem + '.json'),
      JSON.stringify(
        { theme, state, stage, density: (stage[2] - stage[0]) / 320, sliders },
        null,
        2
      )
    )
    console.log('Native Slider paint captured: ' + theme)
  }
})

await check('slider', async () => {
  await launch('slider')
  await click(node => node.text === '20 – 80', 'React range output', true)
  const section = findNode(tree(), node => node.id === 'example-slider/1')
  assert.ok(section)
  assert.equal(
    flatten(section).filter(node => node.type === 'Slider').length,
    3,
    'One scalar and two range thumbs'
  )
  assert.ok(!flatten(section).some(node => node.text === 'Mapping gaps'))
  console.log('Native slider story contains both scalar and range controls')
})

await check('slider-pointer', async () => {
  await launch('', [
    '--pb',
    'paintTest',
    'true',
    '--ps',
    'paintPopup',
    'slider',
  ])
  const medium = findNode(tree(), node => node.id === 'slider-paint-medium')
  const input = flatten(medium).find(node => node.type === 'Slider')
  const [left, top, right, bottom] = box(input)
  const y = Math.round((top + bottom) / 2)
  const x = value => String(Math.round(left + (right - left) * value))
  const value = async () =>
    Number((await inspectNative(input.id)).$attrs.constructor.value)
  const drag = async (from, to, direction) => {
    let finished = false
    const held = new Promise(resolve =>
      execFile(
        hdc,
        [
          '-t',
          device,
          'shell',
          'uinput',
          '-M',
          '-g',
          String(from),
          String(y),
          String(to),
          String(y),
          '5000',
        ],
        { timeout: 10000 },
        error => {
          finished = true
          resolve(error)
        }
      )
    )
    try {
      await wait(2800)
      const current = await value()
      assert.equal(
        finished,
        false,
        'The original mouse input process is still holding the button'
      )
      assert.ok(
        direction > 0 ? current > 60 : current < 60,
        'Held mouse updates the native value before release: ' + current
      )
    } finally {
      await held
    }
    const error = await held
    if (error) throw error
  }
  await drag(Number(x(0.4)), right + 40, 1)
  assert.equal(
    await value(),
    100,
    'Native capture reaches the maximum outside the track'
  )
  await drag(right + 4, left - 40, -1)
  assert.equal(
    await value(),
    0,
    'The visible half of an endpoint thumb can start a drag outside the track'
  )
  console.log('Held native Slider mouse capture and both endpoints passed')
})

await check('slider-modal', async () => {
  await launch('', ['--pb', 'modalTest', 'true'])
  await click(node => node.text === 'Edit range', 'range modal trigger')
  const range = (id, current = tree()) =>
    findNode(current, node => node.id === id)
  const values = current =>
    flatten(current).find(node => node.id === 'slider-accepted-values')?.text
  const count = (id, current = tree()) =>
    Number(
      flatten(current)
        .find(node => node.id === id)
        ?.text.match(/\d+$/)?.[0]
    )
  const tapRange = (id, fraction) => {
    const root = range(id)
    const handle = flatten(root).find(node => node.type === 'Slider')
    assert.ok(handle, id + ' native slider')
    const [left, top, right, bottom] = box(handle)
    shell(
      'uitest',
      'uiInput',
      'click',
      String(Math.round(left + fraction * (right - left))),
      String(Math.round((top + bottom) / 2))
    )
  }
  const key = code => shell('uitest', 'uiInput', 'keyEvent', String(code))
  assert.equal(
    flatten(range('slider-accepted')).filter(node => node.type === 'Slider')
      .length,
    2
  )
  tapRange('slider-accepted', 0.2)
  key(2081)
  await until(current => values(current) === 'Range: 0,80', 'lower thumb Home')
  key(2015)
  await until(
    current => values(current) === 'Range: 1,80',
    'lower thumb one step'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2047', '2015')
  await until(
    current => values(current) === 'Range: 11,80',
    'Shift arrow large step'
  )
  key(2068)
  await until(
    current => values(current) === 'Range: 21,80',
    'Page Up large step'
  )
  key(2082)
  await until(
    current => values(current) === 'Range: 80,80',
    'lower thumb End stops at its neighbour'
  )
  const before = count('slider-accepted-count')
  key(2015)
  await wait(250)
  assert.equal(
    count('slider-accepted-count'),
    before,
    'Clamped keyboard values do not emit'
  )
  key(2049)
  key(2082)
  await until(
    current => values(current) === 'Range: 80,100',
    'Tab reaches upper thumb independently'
  )
  key(2081)
  await until(
    current => values(current) === 'Range: 80,80',
    'upper thumb Home stops at its neighbour'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2047', '2049')
  key(2081)
  await until(
    current => values(current) === 'Range: 0,80',
    'Shift Tab returns to the lower thumb'
  )
  await click(node => node.text === 'Reset range', 'reset range')
  tapRange('slider-accepted', 0.2)
  const native = flatten(range('slider-accepted')).find(
    node => node.type === 'Slider'
  )
  const [left, top, right, bottom] = box(native)
  const y = Math.round((top + bottom) / 2)
  shell(
    'uitest',
    'uiInput',
    'drag',
    String(Math.round(left + (right - left) * 0.2)),
    String(y),
    String(Math.round(left + (right - left) * 0.95)),
    String(y)
  )
  await until(current => {
    const pair = values(current)?.slice('Range: '.length).split(',').map(Number)
    return pair?.length === 2 && pair[0] > 80 && pair[0] === pair[1]
  }, 'dragging lower thumb pushes its neighbour')
  tapRange('slider-refused', 0.2)
  key(2015)
  await until(
    current =>
      flatten(current).some(
        node =>
          node.id === 'slider-refused-proposal' &&
          node.text === 'Refused proposal: 21,80'
      ),
    'refused range uses the controlled value on the next key'
  )
  assert.ok(
    flatten(range('slider-refused')).some(node => node.text === '20 – 80')
  )
  assert.deepEqual(
    flatten(range('slider-refused'))
      .filter(node => node.type === 'Slider')
      .map(node => Number(node.text)),
    [20, 80],
    'Both native handles retain the authoritative controlled values'
  )
  const refused = count('slider-refused-count')
  tapRange('slider-disabled', 0.4)
  await wait(250)
  assert.equal(
    count('slider-refused-count'),
    refused,
    'Disabled range cannot emit'
  )
  key(2070)
  await until(
    current =>
      !flatten(current).some(node => node.text === 'Range verification'),
    'range modal Escape'
  )
  console.log(
    'Native range keyboard, push, controlled proposals and disabled checks passed'
  )
})

await check('accordion', async () => {
  await launch('accordion')
  const firstSection = current =>
    findNode(current, node => node.id === 'example-accordion/0')
  const where =
    'Every colour, length and face is a token. A component reads the variable; it never writes a value of its own.'
  const themes =
    'Six drawn families in light and dark, and the imported ones beside them.'
  assert.ok(flatten(firstSection(tree())).some(node => node.text === where))
  const target = flatten(firstSection(tree())).find(
    node => node.type === 'Button' && node.id.endsWith('-themes')
  )
  assert.ok(target, 'Accordion second header')
  const [l, t, r, b] = box(target)
  shell(
    'uitest',
    'uiInput',
    'click',
    String(Math.round((l + r) / 2)),
    String(Math.round((t + b) / 2))
  )
  await until(current => {
    const nodes = flatten(firstSection(current))
    return (
      nodes.some(node => node.text === themes) &&
      !nodes.some(node => node.text === where)
    )
  }, 'single accordion opens one item and closes the previous')
  console.log('Native accordion single-selection interaction passed')
})

await check('code', async () => {
  await launch('code')
  await click(
    node => node.type === 'Button' && node.description === 'Copy code',
    'code copy button',
    true
  )
  await until(
    current => flatten(current).some(node => node.description === 'Copied'),
    'clipboard completion'
  )
  console.log('Native code clipboard interaction passed')
})

await check('link', async () => {
  await launch('link')
  await click(node => node.id === 'toggle-sidebar', 'Link keyboard entry')
  let focused = false
  for (let attempt = 0; attempt < 8; attempt++) {
    shell('uitest', 'uiInput', 'keyEvent', '2049')
    const first = findNode(tree(), node => node.id === 'link/0/0')
    if (first && flatten(first).some(node => node.focused === 'true')) {
      focused = true
      break
    }
  }
  assert.ok(focused, 'The first native Link is reachable by Tab')
  shell('uitest', 'uiInput', 'keyEvent', '2049')
  const next = findNode(tree(), node => node.id === 'link/0/1')
  assert.ok(
    next && flatten(next).some(node => node.focused === 'true'),
    'Tab advances to the next native Link'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2047', '2049')
  const first = findNode(tree(), node => node.id === 'link/0/0')
  assert.ok(
    first && flatten(first).some(node => node.focused === 'true'),
    'Shift+Tab returns to the preceding Link'
  )
  console.log('Native Link forward and reverse keyboard traversal passed')
})

await check('preview-card', async () => {
  await launch('preview-card')
  const nodes = root => flatten(root)
  const current = (id, root = tree()) =>
    nodes(root).find(node => node.id === id)
  const rail = box(current('component-list'))
  const density = (rail[2] - rail[0]) / 272
  const nativeBox = node =>
    (node.origBounds || node.bounds).match(/-?\d+/g).map(Number)
  const near = (actual, expected, label) =>
    assert.ok(
      Math.abs(actual - expected * density) <= 3,
      `${label}: ${actual / density}`
    )
  const move = (x, y) =>
    shell('uinput', '-M', '-m', String(Math.round(x)), String(Math.round(y)))
  const hover = id => {
    const b = nativeBox(current(id))
    move((b[0] + b[2]) / 2, (b[1] + b[3]) / 2)
    return b
  }
  const surfaces = root =>
    nodes(root).filter(
      node => /^dazzui-preview-\d+-surface$/.test(node.id) && visible(node)
    )
  await until(
    root => current('preview-card/0/0', root),
    'Inline link anchor is measured after text layout'
  )
  assert.ok(
    nodes(tree()).some(node => node.text === 'Reviewed by @ada three days ago.')
  )
  const link = hover('preview-card/0/0')
  const person = await until(
    root =>
      nodes(root).some(node => node.text === 'Ada Lovelace') &&
      surfaces(root)[0],
    'Inline link opens its person preview'
  )
  const personBox = nativeBox(person)
  near(personBox[3] - personBox[1], 76, 'Person card height')
  near(personBox[1] - link[3], 4, 'Preview gap')
  assert.ok(personBox[2] - personBox[0] <= 320 * density + 3)
  const avatar = nativeBox(current('preview-card/0/2'))
  near(avatar[0] - personBox[0], 19, 'Preview padding plus stroke')
  move((personBox[0] + personBox[2]) / 2, (personBox[1] + personBox[3]) / 2)
  await wait(600)
  assert.ok(
    nodes(tree()).some(node => node.text === 'Ada Lovelace'),
    'The pointer can enter the preview'
  )
  move(personBox[2] + 100 * density, personBox[3] + 100 * density)
  await until(
    root => !surfaces(root).length,
    'Preview closes after pointer leaves'
  )
  const button = hover('preview-card/1/1')
  const project = await until(
    root =>
      nodes(root).some(
        node => node.text === 'One design, drawn once for every toolkit.'
      ) && surfaces(root)[0],
    'Button opens the repository preview'
  )
  const projectBox = nativeBox(project)
  near(projectBox[2] - projectBox[0], 288, 'Explicit preview width')
  near(projectBox[3] - projectBox[1], 85.4, 'Preview text and badge height')
  near(projectBox[1] - button[3], 4, 'Button preview gap')
  shell(
    'uinput',
    '-M',
    '-m',
    String(Math.round((button[0] + button[2]) / 2)),
    String(Math.round((button[1] + button[3]) / 2)),
    '-c',
    '0'
  )
  assert.ok(
    surfaces(tree()).length,
    'Mouse activation of the trigger does not dismiss the preview'
  )
  near(
    nativeBox(current('preview-card/1/2'))[3] -
      nativeBox(current('preview-card/1/2'))[1],
    21.4,
    'Small badge line height'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  await until(root => !surfaces(root).length, 'Escape closes the preview')
  // Native focus must reach the inline link, rather than a replacement row of text.
  await click(node => node.id === 'toggle-sidebar', 'Keyboard entry point')
  let focused = false
  for (let attempt = 0; attempt < 12; attempt++) {
    shell('uitest', 'uiInput', 'keyEvent', '2049')
    const scope = findNode(tree(), node => node.id === 'preview-card/0/0')
    if (scope && flatten(scope).some(node => node.focused === 'true')) {
      focused = true
      break
    }
  }
  assert.ok(focused, 'The inline link participates in keyboard focus order')
  await until(
    root => nodes(root).some(node => node.text === 'Ada Lovelace'),
    'Keyboard focus previews the person'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  await until(
    root => !surfaces(root).length,
    'Escape dismisses the focused preview'
  )
  console.log(
    'Native PreviewCard inline text, pointer travel, keyboard focus, sizing and Escape passed'
  )
})

await check('preview-card-modal', async () => {
  await launch('', ['--pb', 'modalTest', 'true'])
  await click(node => node.text === 'Edit preview', 'Preview probe trigger')
  await until(
    root => flatten(root).some(node => node.id === 'preview-default-body'),
    'defaultOpen preview'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  await until(
    root => !flatten(root).some(node => node.id === 'preview-default-body'),
    'Escape closes only the default preview'
  )
  assert.ok(flatten(tree()).some(node => node.text === 'Preview verification'))
  const hover = id => {
    const node = flatten(tree()).find(node => node.id === id)
    const [left, top, right, bottom] = box(node)
    shell(
      'uinput',
      '-M',
      '-m',
      String(Math.round((left + right) / 2)),
      String(Math.round((top + bottom) / 2))
    )
  }
  hover('preview-accepted-trigger')
  await until(
    root => flatten(root).some(node => node.id === 'preview-count'),
    'Controlled open is accepted'
  )
  assert.ok(
    flatten(tree()).some(
      node =>
        node.id === 'preview-accepted' && node.text === 'Preview changes: 1'
    )
  )
  const mouseClick = node => {
    assert.ok(node)
    const [left, top, right, bottom] = box(node)
    shell(
      'uinput',
      '-M',
      '-m',
      String(Math.round((left + right) / 2)),
      String(Math.round((top + bottom) / 2)),
      '-c',
      '0'
    )
  }
  await click(
    node => node.text === 'Increment preview',
    'Touch action in preview while the mouse remains over its trigger'
  )
  await until(
    root => flatten(root).some(node => node.text === 'Preview count: 1'),
    'Preview children remain interactive'
  )
  mouseClick(flatten(tree()).find(node => node.text === 'Increment preview'))
  await until(
    root => flatten(root).some(node => node.text === 'Preview count: 2'),
    'Mouse actions also remain interactive inside the preview'
  )
  mouseClick(flatten(tree()).find(node => node.text === 'Preview destination'))
  await until(
    root => flatten(root).some(node => node.text === 'Preview visits: 1'),
    'Native link pointer activation'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2050')
  assert.ok(
    flatten(tree()).some(node => node.text === 'Preview visits: 1'),
    'Space does not activate a link'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2054')
  await until(
    root => flatten(root).some(node => node.text === 'Preview visits: 2'),
    'Enter activates the link once'
  )
  const editor = flatten(tree()).find(node => node.type === 'TextInput')
  assert.ok(editor)
  mouseClick(editor)
  shell('uinput', '-K', '-t', '31415')
  await until(
    root => flatten(root).some(node => node.text === 'Preview value: 31415'),
    'Native editor in preview content'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2072', '2017')
  // Physical N/I keys can reach the active IME's pre-edit path; text injection
  // may insert committed text directly and cannot verify composition handling.
  shell('uitest', 'uiInput', 'keyEvent', '2030')
  shell('uitest', 'uiInput', 'keyEvent', '2025')
  const composing = flatten(tree()).some(
    node => node.text === 'Preview composing: true'
  )
  if (composing) {
    shell('uitest', 'uiInput', 'keyEvent', '2070')
    await until(
      root =>
        flatten(root).some(node => node.text === 'Preview composing: false'),
      'Escape cancels native IME pre-edit'
    )
    assert.ok(
      flatten(tree()).some(node => node.id === 'preview-count'),
      'IME Escape keeps the preview open'
    )
    console.log(
      'Native IME composition owns its first Escape; the preview remains open'
    )
  } else {
    console.log(
      'The active input method did not compose the Latin probe; IME Escape is unverified'
    )
  }
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  await until(
    root => !flatten(root).some(node => node.id === 'preview-count'),
    'Escape closes the controlled preview'
  )
  assert.ok(
    flatten(tree()).some(
      node =>
        node.id === 'preview-accepted' && node.text === 'Preview changes: 2'
    )
  )
  hover('preview-refused-trigger')
  await until(
    root =>
      flatten(root).some(
        node =>
          node.id === 'preview-refused' && node.text === 'Preview refused: 1'
      ),
    'Host receives the refused proposal'
  )
  assert.ok(!flatten(tree()).some(node => node.id === 'preview-refused-body'))
  hover('preview-accepted-trigger')
  await until(
    root => flatten(root).some(node => node.id === 'preview-count'),
    'Preview reopens after leaving and returning'
  )
  const beforeOutside = flatten(tree())
  assert.equal(
    beforeOutside.find(node => node.id === 'preview-accepted')?.text,
    'Preview changes: 3',
    'Reopening emits one controlled proposal'
  )
  const popup = beforeOutside.find(
    node =>
      node.id?.startsWith('dazzui-preview-') && node.id.endsWith('-surface')
  )
  const outside = beforeOutside.find(
    node => node.text === 'Outside preview action'
  )
  assert.ok(popup && outside)
  const [pl, pt, pr, pb] = box(popup)
  const [ol, ot, or, ob] = box(outside)
  const ox = (ol + or) / 2
  const oy = (ot + ob) / 2
  assert.ok(
    ox < pl || ox > pr || oy < pt || oy > pb,
    'The outside-action test point must lie outside the actual popup'
  )
  await click(
    node => node.text === 'Outside preview action',
    'Outside action after refused preview'
  )
  await until(
    root => !flatten(root).some(node => node.id === 'preview-count'),
    'A touch outside dismisses even while the mouse remains over the trigger'
  )
  assert.ok(
    flatten(tree()).some(
      node =>
        node.id === 'preview-accepted' && node.text === 'Preview changes: 4'
    )
  )
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  await until(
    root => !flatten(root).some(node => node.text === 'Preview verification'),
    'Parent modal still owns its dismissal'
  )
  console.log(
    'Native PreviewCard default state, controlled acceptance/refusal and editable popup content passed'
  )
})

await check('popover', async () => {
  await launch('popover')
  await click(node => node.text === 'Share', 'share popover trigger')
  await until(
    current =>
      flatten(current).some(node => node.text === 'Share this project'),
    'popover opens'
  )
  await click(node => node.text === 'Done', 'popover close button')
  await until(
    current =>
      !flatten(current).some(node => node.text === 'Share this project'),
    'popover closes'
  )
  console.log('Native popover close interaction passed')
})

await check('dialog', async () => {
  const themes = JSON.parse(
    fs.readFileSync(
      path.resolve('packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json'),
      'utf8'
    )
  )
  const argb = value =>
    '#' +
    value
      .map(byte => byte.toString(16).padStart(2, '0'))
      .join('')
      .toUpperCase()
  for (const theme of ['studio-light', 'studio-dark', 'bright-light']) {
    await launch('dialog', ['--ps', 'themeName', theme])
    const rail = box(flatten(tree()).find(node => node.id === 'component-list'))
    const density = (rail[2] - rail[0]) / 272
    await click(node => node.text === 'Rename…', 'dialog trigger')
    const surface = await until(
      root =>
        flatten(root).find(node => node.id?.startsWith('dazzui-focus-dialog-')),
      'native dialog surface'
    )
    const attrs = (await inspectNative(surface.id)).$attrs
    const bounds = box(surface)
    assert.ok(
      (bounds[3] - bounds[1]) / density < 300,
      'A short dialog keeps its natural content height'
    )
    assert.equal(
      attrs.backgroundColor,
      argb(themes[theme]['color.surface-overlay'])
    )
    assert.equal(
      attrs.clip,
      'true',
      'Dialog content clips to its rounded surface'
    )
    await click(node => node.text === 'Cancel', 'dialog cancel button')
    await until(
      root => !flatten(root).some(node => node.text === 'Rename project'),
      'dialog closes'
    )
  }
  console.log(
    'Native dialog overlay palette, rounded clipping, activation and dismissal passed in three themes'
  )
})

await check('control-shadow-paint', async () => {
  const themes = JSON.parse(
    fs.readFileSync(
      path.resolve('packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json'),
      'utf8'
    )
  )
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/control-shadow'
  )
  fs.mkdirSync(directory, { recursive: true })
  const selectedTheme = option('--paint-theme')
  const reports = []
  for (const theme of selectedTheme
    ? [selectedTheme]
    : ['studio-light', 'studio-dark', 'bright-light']) {
    await launch('', [
      '--pb',
      'paintTest',
      'true',
      '--ps',
      'paintPopup',
      'control',
      '--ps',
      'themeName',
      theme,
    ])
    const root = tree()
    const scope = findNode(root, node => node.id === 'paint-direct-control')
    const button = flatten(scope).find(node => node.type === 'Button')
    const stage = box(
      flatten(root).find(node => node.id === 'control-paint-stage')
    )
    const density = (stage[2] - stage[0]) / 640
    const native = await inspectNative(button.id)
    const viewport = box(
      flatten(root).find(node => node.id === 'document-column')
    )
    const { frame, reference } = native.$geometry
    // The separate drawing layer follows the foreground's actual pixel phase.
    // Keep its native fractional size instead of rounding all four edges.
    const left = Math.fround(viewport[0] + frame.x - reference.x)
    const top = Math.fround(viewport[1] + frame.y - reference.y)
    const bounds = [left, top, left + frame.width, top + frame.height]
    const heightToken = themes[theme]['control.small.size']
    assert.ok(['rem', 'px'].includes(heightToken.unit))
    const height = heightToken.value * (heightToken.unit === 'rem' ? 16 : 1)
    assert.ok(
      Math.abs((bounds[3] - bounds[1]) / density - height) <= 1,
      'Native control height matches the shared profile'
    )
    const stem = 'native-shadow-2xs-control-' + theme
    const remote = '/data/local/tmp/dazzui-' + stem + '.png'
    shell('uitest', 'screenCap', '-p', remote)
    command('file', 'recv', remote, path.join(directory, stem + '.png'))
    const file = path.join(directory, stem + '.json')
    fs.writeFileSync(
      file,
      JSON.stringify(
        {
          theme,
          renderer: 'native-control',
          role: 'shadow.2xs',
          radiusRole: 'control.small.radius',
          outline: 'none',
          sampling: 'pixels',
          density,
          stage,
          bounds,
          viewport,
          nativeShadow: native.$attrs.shadow,
          nativeGeometry: native.$geometry,
        },
        null,
        2
      )
    )
    const report = checkShadowPaint(file, themes)
    reports.push(report)
    assert.ok(
      report.passed,
      'Actual filled-button shadow pixels: ' + JSON.stringify(report)
    )
    shell(
      'uitest',
      'uiInput',
      'click',
      String(Math.round((bounds[0] + bounds[2]) / 2)),
      String(Math.ceil(bounds[3]) + 1)
    )
    await until(
      root =>
        flatten(root).some(
          node =>
            node.id === 'paint-control-count' &&
            node.text === 'Button actions: 0; underlay: 1'
        ),
      'button shadow passes input to the actual underlay'
    )
    await click(
      node => node.id === button.id,
      'activate the native button foreground'
    )
    await until(
      root =>
        flatten(root).some(
          node =>
            node.id === 'paint-control-count' &&
            node.text === 'Button actions: 1; underlay: 1'
        ),
      'button foreground blocks the underlay'
    )
  }
  assert.equal(reports.length, selectedTheme ? 1 : 3)
  console.log(JSON.stringify(reports, null, 2))
  console.log(
    'Native filled-button shadow edge/corner pixels and foreground/underlay input passed at actual scale'
  )
})

await check('select-opening', async () => {
  await launch('', [
    '--pb',
    'paintTest',
    'true',
    '--ps',
    'paintPopup',
    'select-long',
  ])
  const isSurface = node =>
    node.type === 'Menu' && /^dazz-select-\d+-menu$/.test(node.id)
  const close = async () => {
    shell('uitest', 'uiInput', 'keyEvent', '2070')
    await until(
      root =>
        !flatten(root).some(isSurface) &&
        flatten(root).some(
          node =>
            node.type === 'Button' &&
            node.id.endsWith('-trigger') &&
            node.focused === 'true'
        ),
      'Escape closes Select and returns focus to its trigger'
    )
  }
  await click(
    node => node.id === 'paint-menu-trigger',
    'establish trigger focus'
  )
  await until(root => flatten(root).some(isSurface), 'The pointer opens Select')
  await close()
  for (const [choice, actions] of [
    [1, 0],
    [60, 1],
  ]) {
    for (const [name, key] of [
      ['Up', 2012],
      ['Down', 2013],
      ['Enter', 2054],
      ['Space', 2050],
    ]) {
      shell('uitest', 'uiInput', 'keyEvent', String(key))
      await until(
        root =>
          flatten(root).some(
            node =>
              node.type === 'MenuItem' &&
              node.id.endsWith('-option-choice-' + choice) &&
              node.focused === 'true' &&
              visible(node)
          ),
        name +
          ' opens Select at the ' +
          (actions ? 'selected' : 'first enabled') +
          ' option'
      )
      assert.ok(
        flatten(tree()).some(
          node =>
            node.id === 'paint-menu-count' &&
            node.text === 'Menu actions: ' + actions
        ),
        name + ' opens without committing on key release'
      )
      await close()
    }
    if (!actions) {
      shell('uitest', 'uiInput', 'keyEvent', '2054')
      await until(
        root => flatten(root).some(isSurface),
        'Open before committing the last choice'
      )
      shell('uitest', 'uiInput', 'keyEvent', '2082')
      shell('uitest', 'uiInput', 'keyEvent', '2054')
      await until(
        root =>
          !flatten(root).some(isSurface) &&
          flatten(root).some(
            node =>
              node.id === 'paint-menu-count' && node.text === 'Menu actions: 1'
          ),
        'Commit the final choice once'
      )
    }
  }
  console.log(
    'Native Select Up/Down/Enter/Space opening, first/selected focus, key-release containment and Escape return passed'
  )
})

await check('menu-viewport', async () => {
  const themes = JSON.parse(
    fs.readFileSync(
      'packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json',
      'utf8'
    )
  )
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/menu-viewport'
  )
  fs.mkdirSync(directory, { recursive: true })
  const reports = []
  const selectedKind = option('--paint-menu')
  assert.ok(!selectedKind || ['menu', 'select'].includes(selectedKind))
  for (const kind of selectedKind ? [selectedKind] : ['menu', 'select']) {
    await launch('', [
      '--pb',
      'paintTest',
      'true',
      '--ps',
      'paintPopup',
      kind + '-long',
    ])
    const document = flatten(tree()).find(node => node.id === 'document-column')
    await click(
      node =>
        node.id === 'EnhanceMaximizeBtn' &&
        node.hostWindowId === document.hostWindowId,
      'restore a floating window before live menu resizing'
    )
    await until(root => {
      const next = flatten(root).find(node => node.id === 'document-column')
      return (
        box(next)[2] - box(next)[0] < box(document)[2] - box(document)[0] - 100
      )
    }, 'the native window is floating')
    const isSurface = node =>
      node.type === 'Menu' &&
      (kind === 'select'
        ? /^dazz-select-\d+-menu$/.test(node.id)
        : /^dazzui-menu-surface-\d+$/.test(node.id))
    const pathTo = (node, id, ancestors = []) =>
      node.attributes.id === id
        ? ancestors
        : (node.children ?? [])
            .map(child => pathTo(child, id, [...ancestors, node.attributes]))
            .find(Boolean)
    const geometry = (root = tree()) => {
      const surface = flatten(root).find(isSurface)
      assert.ok(surface, 'The native menu remains open')
      const ancestors = pathTo(root, surface.id)
      const host = ancestors.findLast(node => node.type === 'Menu')
      const rows = flatten(
        findNode(root, node => node.id === surface.id)
      ).filter(node => node.type === 'MenuItem' && visible(node))
      return {
        surface,
        bounds: box(host),
        rows,
        document: box(
          flatten(root).find(node => node.id === 'document-column')
        ),
      }
    }
    const choiceIndex = node =>
      Number(
        node.id.match(
          kind === 'select' ? /-option-choice-(\d+)$/ : /-item-(\d+)$/
        )?.[1]
      ) + (kind === 'select' ? 0 : 1)
    const firstVisible = () => Math.min(...geometry().rows.map(choiceIndex))
    const open = async () => {
      await click(
        node => node.id === 'paint-menu-trigger',
        'open the long ' + kind
      )
      await until(
        root => flatten(root).some(isSurface),
        'the native sheet opens'
      )
    }
    const close = async () => {
      shell('uitest', 'uiInput', 'keyEvent', '2070')
      await until(
        root => !flatten(root).some(isSurface),
        'Escape closes the native sheet'
      )
    }
    const capture = name => {
      const current = geometry()
      assert.ok(
        current.rows.length,
        'The resized viewport has visible native rows'
      )
      assert.ok(
        current.bounds.every((value, index) =>
          index < 2
            ? value >= current.document[index] - 1
            : value <= current.document[index] + 1
        ),
        'The native menu fits the resized window'
      )
      assert.ok(
        current.rows.every(row =>
          box(row).every((value, index) =>
            index < 2
              ? value >= current.bounds[index] - 1
              : value <= current.bounds[index] + 1
          )
        ),
        'Native rows remain clipped to the resized menu viewport'
      )
      const stage = box(
        flatten(tree()).find(node => node.id === 'menu-paint-stage')
      )
      const stem = 'native-shadow-menu-viewport-' + kind + '-' + name
      const remote = '/data/local/tmp/dazzui-' + stem + '.png'
      shell('uitest', 'screenCap', '-p', remote)
      command('file', 'recv', remote, path.join(directory, stem + '.png'))
      const file = path.join(directory, stem + '.json')
      fs.writeFileSync(
        file,
        JSON.stringify(
          {
            theme: 'studio-light',
            renderer: 'native-' + kind,
            role: 'shadow.lg',
            radiusRole: 'radius.large',
            density: (stage[2] - stage[0]) / 640,
            bounds: current.bounds,
            viewport: current.document,
            profileClipping: 'viewport',
            geometry: current,
          },
          null,
          2
        )
      )
      const report = checkShadowPaint(file, themes)
      reports.push(report)
      fs.writeFileSync(
        path.join(directory, 'results.json'),
        JSON.stringify(reports, null, 2)
      )
      assert.ok(
        report.passed,
        'The resized viewport casts the shared shadow: ' +
          JSON.stringify(report)
      )
      return current
    }
    await open()
    const first = firstVisible()
    const initial = geometry().bounds
    const x = Math.round((initial[0] + initial[2]) / 2)
    const y = Math.round((initial[1] + initial[3]) / 2)
    shell('uinput', '-M', '-m', String(x), String(y), '-s', '80')
    await until(
      () => firstVisible() > first + 1,
      'A real mouse wheel scrolls the native menu'
    )
    const afterWheel = firstVisible()
    shell(
      'uitest',
      'uiInput',
      'swipe',
      String(x),
      String(Math.round(initial[1] + (initial[3] - initial[1]) * 0.8)),
      String(x),
      String(Math.round(initial[1] + (initial[3] - initial[1]) * 0.35))
    )
    await until(
      () => firstVisible() > afterWheel + 1,
      'Touch dragging scrolls the menu without activating a row'
    )
    await until(
      root =>
        flatten(root).some(
          node =>
            node.id === 'paint-menu-count' && node.text === 'Menu actions: 0'
        ),
      'Scrolling keeps all row actions idle'
    )
    console.log('Native ' + kind + ' wheel and touch scrolling passed')
    await close()
    const heights = []
    for (const [name, height] of [
      ['small', 600],
      ['large', 820],
    ]) {
      await click(
        node => node.id === 'menu-resize-' + name,
        'arm the actual window resize to ' + height + ' vp'
      )
      await open()
      let scrolledFirst
      if (name === 'small') {
        const bounds = geometry().bounds
        shell(
          'uinput',
          '-M',
          '-m',
          String(Math.round((bounds[0] + bounds[2]) / 2)),
          String(Math.round((bounds[1] + bounds[3]) / 2)),
          '-s',
          '80'
        )
        await until(
          () => firstVisible() > 2,
          'Scroll before shrinking the open menu'
        )
        scrolledFirst = firstVisible()
      } else {
        shell('uitest', 'uiInput', 'keyEvent', '2082')
        await until(
          root =>
            flatten(root).some(
              node =>
                node.type === 'MenuItem' &&
                choiceIndex(node) === 60 &&
                node.focused === 'true' &&
                visible(node)
            ),
          'Focus the final row before growing the open menu'
        )
      }
      await until(
        root =>
          flatten(root).some(
            node =>
              node.id === 'menu-resize-status' && node.text === String(height)
          ),
        'The native floating window resizes while the menu is open',
        30
      )
      await wait(500)
      const resized = capture(name)
      if (scrolledFirst !== undefined)
        assert.ok(
          Math.abs(firstVisible() - scrolledFirst) <= 1,
          'Shrinking preserves the scroll position instead of reopening at the top'
        )
      else
        assert.ok(
          resized.rows.some(
            node => choiceIndex(node) === 60 && node.focused === 'true'
          ),
          'Growing preserves focus on the previously focused final row'
        )
      console.log(
        'Native ' +
          kind +
          ' ' +
          name +
          ' viewport: ' +
          JSON.stringify(resized.bounds)
      )
      heights.push(resized.bounds[3] - resized.bounds[1])
      shell('uitest', 'uiInput', 'keyEvent', '2082')
      await until(
        root =>
          flatten(root).some(
            node =>
              node.type === 'MenuItem' &&
              choiceIndex(node) === 60 &&
              node.focused === 'true' &&
              visible(node)
          ),
        'The final row remains reachable after live resize'
      )
      shell('uitest', 'uiInput', 'keyEvent', '2081')
      await until(
        root =>
          flatten(root).some(
            node =>
              node.type === 'MenuItem' &&
              choiceIndex(node) === 1 &&
              node.focused === 'true' &&
              visible(node)
          ),
        'Home restores the first row after live resize'
      )
      await close()
    }
    assert.ok(
      heights[1] > heights[0] + 100,
      'The viewport can grow again after shrinking'
    )
    if (kind === 'select') {
      // The reference's trigger-aligned mouse/keyboard list closes on resize;
      // its touch popup uses anchored positioning and remains open instead.
      await open()
      shell('uitest', 'uiInput', 'keyEvent', '2082')
      shell('uitest', 'uiInput', 'keyEvent', '2054')
      await until(
        root =>
          !flatten(root).some(isSurface) &&
          flatten(root).some(
            node =>
              node.id === 'paint-menu-count' && node.text === 'Menu actions: 1'
          ),
        'Commit the final option before testing mouse-opened resize dismissal'
      )
      await click(
        node => node.id === 'menu-resize-small',
        'arm resize for a mouse-opened Select'
      )
      const anchor = box(
        flatten(tree()).find(node => node.id === 'paint-menu-trigger')
      )
      shell(
        'uinput',
        '-M',
        '-m',
        String(Math.round((anchor[0] + anchor[2]) / 2)),
        String(Math.round((anchor[1] + anchor[3]) / 2)),
        '-c',
        '0'
      )
      await until(
        root => flatten(root).some(isSurface),
        'Mouse opens the native Select'
      )
      await until(
        root =>
          flatten(root).some(
            node => node.id === 'menu-resize-status' && node.text === '600'
          ) && !flatten(root).some(isSurface),
        'The mouse-opened Select closes on window resize',
        30
      )
      assert.ok(
        flatten(tree()).some(
          node =>
            node.type === 'Button' && node.text === 'Documentation choice 60'
        ),
        'Resize dismissal retains the committed selection'
      )
      console.log(
        'Native mouse-opened Select resize dismissal retains its value'
      )
      await click(
        node => node.id === 'menu-resize-large',
        'arm resize for a keyboard-opened Select'
      )
      let focusedTrigger
      for (let attempt = 0; attempt < 8; attempt++) {
        focusedTrigger = flatten(tree()).find(
          node =>
            node.type === 'Button' &&
            node.text === 'Documentation choice 60' &&
            node.focused === 'true'
        )
        if (focusedTrigger) break
        shell('uitest', 'uiInput', 'keyEvent', '2049')
      }
      assert.ok(focusedTrigger, 'Native Tab reaches the Select trigger')
      shell('uitest', 'uiInput', 'keyEvent', '2054')
      await until(
        root => flatten(root).some(isSurface),
        'Enter opens the native Select'
      )
      await until(
        root =>
          flatten(root).some(
            node => node.id === 'menu-resize-status' && node.text === '820'
          ) && !flatten(root).some(isSurface),
        'The keyboard-opened Select closes on window resize',
        30
      )
      assert.ok(
        flatten(tree()).some(
          node =>
            node.type === 'Button' && node.text === 'Documentation choice 60'
        ),
        'Keyboard resize dismissal retains the committed selection'
      )
      console.log(
        'Native keyboard-opened Select resize dismissal retains its value'
      )
    }
  }
  console.log(
    'Native Menu/Select wheel, touch scrolling, live window shrink/grow, clipping, shadow and focus passed'
  )
})

await check('menu-scroll', async () => {
  const themes = JSON.parse(
    fs.readFileSync(
      'packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json',
      'utf8'
    )
  )
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/menu-scroll'
  )
  fs.mkdirSync(directory, { recursive: true })
  for (const kind of ['menu', 'select']) {
    await launch('', [
      '--pb',
      'paintTest',
      'true',
      '--ps',
      'paintPopup',
      kind + '-long',
    ])
    await click(
      node => node.id === 'paint-menu-trigger',
      'open sixty native ' + kind + ' items'
    )
    const surface = await until(
      root =>
        flatten(root).find(
          node =>
            node.type === 'Menu' &&
            (kind === 'select'
              ? /^dazz-select-\d+-menu$/.test(node.id)
              : /^dazzui-menu-surface-\d+$/.test(node.id))
        ),
      'long native sheet'
    )
    const pathTo = (node, id, ancestors = []) =>
      node.attributes.id === id
        ? ancestors
        : (node.children ?? [])
            .map(child => pathTo(child, id, [...ancestors, node.attributes]))
            .find(Boolean)
    const ancestors = pathTo(tree(), surface.id)
    const viewportNode = ancestors?.findLast(node => node.type === 'Menu')
    const viewport = viewportNode ? { bounds: box(viewportNode) } : undefined
    const geometry = { surface, ancestors }
    assert.ok(
      viewport,
      'Native Menu retains its positioning and scroll viewport'
    )
    const stage = box(
      flatten(tree()).find(node => node.id === 'menu-paint-stage')
    )
    const density = (stage[2] - stage[0]) / 640
    const rows = flatten(
      findNode(tree(), node => node.id === surface.id)
    ).filter(node => node.type === 'MenuItem')
    assert.ok(
      rows.length > 0 && rows.length <= 60,
      'The accessibility tree exposes the currently visible native rows'
    )
    const escaped = rows.filter(
      node =>
        visible(node) &&
        box(node).some((value, index) =>
          index < 2
            ? value < viewport.bounds[index] - 1
            : value > viewport.bounds[index] + 1
        )
    )
    const capture = '/data/local/tmp/dazzui-long-menu-' + kind + '.png'
    shell('uitest', 'screenCap', '-p', capture)
    command('file', 'recv', capture, path.join(directory, kind + '.png'))
    const file = path.join(directory, kind + '.json')
    fs.writeFileSync(
      file,
      JSON.stringify(
        {
          theme: 'studio-light',
          renderer: 'native-long-' + kind,
          role: 'shadow.lg',
          radiusRole: 'radius.large',
          density,
          bounds: viewport.bounds,
          viewport: box(
            flatten(tree()).find(node => node.id === 'document-column')
          ),
          geometry,
          escaped,
          profileClipping: 'viewport',
        },
        null,
        2
      )
    )
    assert.deepEqual(
      escaped.map(node => node.id),
      [],
      'Long menu rows stay inside the native viewport'
    )
    const report = checkShadowPaint(file, themes)
    assert.ok(
      report.passed,
      'The viewport, not the full scroll content, casts the shadow: ' +
        JSON.stringify(report)
    )
    const focusedChoice = async (index, label) =>
      until(
        root =>
          flatten(root).some(
            node =>
              node.type === 'MenuItem' &&
              node.id.endsWith(
                kind === 'select'
                  ? '-option-choice-' + index
                  : '-item-' + (index - 1)
              ) &&
              node.focused === 'true' &&
              visible(node)
          ),
        label
      )
    shell('uitest', 'uiInput', 'keyEvent', '2013')
    await focusedChoice(1, 'Down enters the initially unselected native list')
    shell('uitest', 'uiInput', 'keyEvent', '2082')
    await focusedChoice(
      60,
      'End brings the last native option into the viewport'
    )
    shell('uitest', 'uiInput', 'keyEvent', '2012')
    await focusedChoice(59, 'Native Up still traverses the scrolled rows')
    shell('uitest', 'uiInput', 'keyEvent', '2081')
    await focusedChoice(1, 'Home restores the first row and viewport')
    shell('uitest', 'uiInput', 'keyEvent', '2013')
    await focusedChoice(2, 'Native Down still traverses after Home')
    shell('uitest', 'uiInput', 'keyEvent', '2082')
    await focusedChoice(60, 'Repeated End reveals the final row')
    shell('uitest', 'uiInput', 'keyEvent', '2054')
    await until(
      root =>
        !flatten(root).some(node => node.id === surface.id) &&
        flatten(root).some(
          node =>
            node.id === 'paint-menu-count' && node.text === 'Menu actions: 1'
        ),
      'The final option stays reachable and activates once'
    )
    if (kind === 'select') {
      await click(
        node => node.id === 'paint-menu-trigger',
        'reopen the long selected list'
      )
      await focusedChoice(60, 'Reopening reveals the selected final option')
      shell('uitest', 'uiInput', 'keyEvent', '2070')
      await until(
        root => !flatten(root).some(node => node.id === surface.id),
        'Escape dismisses the reopened long list'
      )
    }
  }
  console.log(
    'Long native Menu and Select clipping, viewport shadow, arrows, Home/End, selected reopening and confirmation passed'
  )
})

await check('menu-paint', async () => {
  const themes = JSON.parse(
    fs.readFileSync(
      'packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json',
      'utf8'
    )
  )
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/menu-paint'
  )
  fs.mkdirSync(directory, { recursive: true })
  const selectedTheme = option('--paint-theme')
  const selectedKind = option('--paint-menu')
  assert.ok(
    !selectedKind || ['menu', 'select', 'context'].includes(selectedKind)
  )
  const reports = []
  for (const theme of selectedTheme
    ? [selectedTheme]
    : ['studio-light', 'studio-dark', 'bright-light']) {
    for (const kind of selectedKind
      ? [selectedKind]
      : ['menu', 'select', 'context']) {
      await launch('', [
        '--pb',
        'paintTest',
        'true',
        '--ps',
        'paintPopup',
        kind,
        '--ps',
        'themeName',
        theme,
      ])
      const stage = box(
        flatten(tree()).find(node => node.id === 'menu-paint-stage')
      )
      const density = (stage[2] - stage[0]) / 640
      const openSheet = async () => {
        if (kind === 'context') {
          const anchor = box(
            flatten(tree()).find(node => node.id === 'paint-menu-trigger')
          )
          shell(
            'uinput',
            '-M',
            '-m',
            String(Math.round((anchor[0] + anchor[2]) / 2)),
            String(Math.round((anchor[1] + anchor[3]) / 2)),
            '-c',
            '1'
          )
          await wait(300)
        } else
          await click(
            node => node.id === 'paint-menu-trigger',
            'open the isolated ' + kind
          )
        return until(
          root =>
            flatten(root).find(
              node =>
                node.type === 'Menu' &&
                (kind === 'select'
                  ? /^dazz-select-\d+-menu$/.test(node.id)
                  : /^dazzui-menu-surface-\d+$/.test(node.id))
            ),
          'actual native ' + kind + ' sheet'
        )
      }
      const capture = (bounds, suffix, role = 'shadow.lg', geometry) => {
        const stem = 'native-menu-' + kind + '-' + theme + suffix
        const remote = '/data/local/tmp/dazzui-' + stem + '.png'
        shell('uitest', 'screenCap', '-p', remote)
        command('file', 'recv', remote, path.join(directory, stem + '.png'))
        const file = path.join(directory, stem + '.json')
        fs.writeFileSync(
          file,
          JSON.stringify(
            {
              theme,
              renderer: 'native-' + kind,
              role,
              radiusRole: 'radius.large',
              density,
              stage,
              bounds,
              viewport: stage,
              nativeGeometry: geometry,
            },
            null,
            2
          )
        )
        const report = checkShadowPaint(file, themes)
        reports.push(report)
        fs.writeFileSync(
          path.join(directory, 'results.json'),
          JSON.stringify(reports, null, 2)
        )
        assert.ok(
          report.passed,
          'Native ' + kind + ' shadow pixels: ' + JSON.stringify(report)
        )
      }
      const captureSheet = async (surface, suffix) => {
        const geometry =
          kind === 'context'
            ? inspectWindowSurface(
                surface,
                path.join(
                  directory,
                  kind + '-' + theme + suffix + '-element.txt'
                )
              )
            : undefined
        const bounds = geometry?.bounds ?? box(surface)
        capture(bounds, suffix, 'shadow.lg', geometry)
        return bounds
      }
      let surface = await openSheet()
      await captureSheet(surface, '')
      await click(
        node =>
          node.type === 'MenuItem' &&
          node.id.endsWith(kind === 'select' ? '-option-second' : '-item-1'),
        'native ' + kind + ' foreground action'
      )
      await until(
        root =>
          !flatten(root).some(node => node.id === surface.id) &&
          flatten(root).some(
            node =>
              node.id === 'paint-menu-count' && node.text === 'Menu actions: 1'
          ),
        'the native action stays live and dismisses the ' + kind
      )
      surface = await openSheet()
      const bounds = await captureSheet(surface, '-reopened')
      shell(
        'uitest',
        'uiInput',
        'click',
        String(Math.ceil(bounds[2] + 4 * density)),
        String(Math.round((bounds[1] + bounds[3]) / 2))
      )
      await until(
        root =>
          !flatten(root).some(node => node.id === surface.id) &&
          flatten(root).some(
            node =>
              node.id === 'paint-menu-count' && node.text === 'Menu actions: 1'
          ),
        'the painted shadow remains outside the native menu response area'
      )
      capture(bounds, '-closed', 'none')
    }
  }
  console.log(JSON.stringify(reports, null, 2))
  console.log(
    'Native Menu, Select and context-menu Gaussian edges/corners and foreground actions passed'
  )
})

await check('popup-paint', async () => {
  const themes = JSON.parse(
    fs.readFileSync(
      path.resolve('packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json'),
      'utf8'
    )
  )
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/popup-paint'
  )
  fs.mkdirSync(directory, { recursive: true })
  const reports = []
  const selectedTheme = option('--paint-theme')
  const selectedSide = option('--paint-side')
  assert.ok(
    !selectedSide ||
      ['top', 'bottom', 'left', 'right', 'preview'].includes(selectedSide),
    'Choose a popup side or preview'
  )
  for (const theme of selectedTheme
    ? [selectedTheme]
    : ['studio-light', 'studio-dark', 'bright-light']) {
    for (const side of selectedSide
      ? [selectedSide]
      : ['top', 'bottom', 'left', 'right', 'preview']) {
      await launch('', [
        '--pb',
        'paintTest',
        'true',
        '--ps',
        'paintPopup',
        side,
        '--ps',
        'themeName',
        theme,
      ])
      const root = tree()
      const stage = box(
        flatten(root).find(node => node.id === 'popup-paint-stage')
      )
      const density = (stage[2] - stage[0]) / 640
      const trigger = box(
        flatten(root).find(node => node.id === 'paint-popup-trigger')
      )
      if (side !== 'preview')
        await click(
          node => node.id === 'paint-popup-trigger',
          'open native ' + side + ' popup'
        )
      else
        await click(
          node => node.id === 'paint-preview-open',
          'open controlled preview after window layout'
        )
      const surface = await until(
        root =>
          flatten(root).find(node =>
            node.id?.startsWith('dazzui-popup-surface-')
          ),
        'real popup foreground'
      )
      const bounds = box(surface)
      const gap =
        side === 'top'
          ? trigger[1] - bounds[3]
          : side === 'left'
            ? trigger[0] - bounds[2]
            : side === 'right'
              ? bounds[0] - trigger[2]
              : bounds[1] - trigger[3]
      assert.ok(
        Math.abs(gap / density - 4) <= 1,
        `${side}: visual popup gap ${gap / density}vp`
      )
      const stem = 'native-shadow-lg-popup-' + side + '-' + theme
      const remote = '/data/local/tmp/dazzui-' + stem + '.png'
      shell('uitest', 'screenCap', '-p', remote)
      command('file', 'recv', remote, path.join(directory, stem + '.png'))
      const file = path.join(directory, stem + '.json')
      fs.writeFileSync(
        file,
        JSON.stringify(
          {
            theme,
            renderer: 'native-popup',
            role: 'shadow.lg',
            radiusRole: 'radius.large',
            density,
            stage,
            bounds,
            viewport: box(
              flatten(root).find(node => node.id === 'document-column')
            ),
          },
          null,
          2
        )
      )
      const report = checkShadowPaint(file, themes)
      reports.push(report)
      assert.ok(
        report.passed,
        'Real popup shadow pixels: ' + JSON.stringify(report)
      )
      await click(
        node => node.id === 'paint-popup-action',
        'foreground popup action'
      )
      await until(
        root =>
          flatten(root).some(
            node =>
              node.id === 'paint-popup-count' &&
              node.text === 'Popup actions: 1'
          ),
        'popup keeps its foreground action'
      )
      shell('uitest', 'uiInput', 'keyEvent', '2070')
      await until(
        root => !flatten(root).some(node => node.id === surface.id),
        'native popup Escape dismissal'
      )
      if (side !== 'preview') {
        await click(
          node => node.id === 'paint-popup-trigger',
          'reopen native popup'
        )
        const reopened = await until(
          root =>
            flatten(root).find(node =>
              node.id?.startsWith('dazzui-popup-surface-')
            ),
          'reopened popup surface'
        )
        const rectangle = box(reopened)
        const outerPoint =
          side === 'left'
            ? [rectangle[0] - 4 * density, (rectangle[1] + rectangle[3]) / 2]
            : side === 'top'
              ? [(rectangle[0] + rectangle[2]) / 2, rectangle[1] - 4 * density]
              : side === 'bottom'
                ? [
                    (rectangle[0] + rectangle[2]) / 2,
                    rectangle[3] + 4 * density,
                  ]
                : [
                    rectangle[2] + 4 * density,
                    (rectangle[1] + rectangle[3]) / 2,
                  ]
        shell(
          'uitest',
          'uiInput',
          'click',
          String(Math.round(outerPoint[0])),
          String(Math.round(outerPoint[1]))
        )
        await until(
          root => !flatten(root).some(node => node.id === reopened.id),
          'a press in the visible shadow remains outside the popup'
        )
      }
    }
  }
  assert.equal(reports.length, (selectedTheme ? 1 : 3) * (selectedSide ? 1 : 5))
  console.log(JSON.stringify(reports, null, 2))
  console.log(
    `Native popup Gaussian edge/corner paint, visual anchor gap and foreground/Escape passed for ${reports.length} cases`
  )
})

await check('dialog-paint', async () => {
  const themes = JSON.parse(
    fs.readFileSync(
      path.resolve('packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json'),
      'utf8'
    )
  )
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/dialog-paint'
  )
  fs.mkdirSync(directory, { recursive: true })
  const reports = []
  for (const theme of ['studio-light', 'studio-dark', 'bright-light']) {
    await launch('', [
      '--pb',
      'paintTest',
      'true',
      '--pb',
      'paintDialog',
      'true',
      '--ps',
      'themeName',
      theme,
    ])
    const stage = box(
      flatten(tree()).find(node => node.id === 'dialog-paint-stage')
    )
    await click(
      node => node.id === 'paint-dialog-trigger',
      'open isolated real dialog'
    )
    const surface = await until(
      root =>
        flatten(root).find(node => node.id?.startsWith('dazzui-focus-dialog-')),
      'isolated dialog surface'
    )
    const bounds = box(surface)
    const stem = 'native-shadow-lg-dialog-' + theme
    const remote = '/data/local/tmp/dazzui-' + stem + '.png'
    shell('uitest', 'screenCap', '-p', remote)
    command('file', 'recv', remote, path.join(directory, stem + '.png'))
    const file = path.join(directory, stem + '.json')
    fs.writeFileSync(
      file,
      JSON.stringify(
        {
          theme,
          renderer: 'native-dialog',
          role: 'shadow.lg',
          radiusRole: 'frame.window.radius',
          backdrop: 'dialog-scrim',
          density: (stage[2] - stage[0]) / 640,
          stage,
          bounds,
        },
        null,
        2
      )
    )
    const report = checkShadowPaint(file, themes)
    reports.push(report)
    assert.ok(
      report.passed,
      'Actual dialog shadow pixels: ' + JSON.stringify(report)
    )
    shell('uitest', 'uiInput', 'keyEvent', '2070')
    await until(
      root => !flatten(root).some(node => node.id === surface.id),
      'paint dialog Escape dismissal'
    )
  }
  console.log(JSON.stringify(reports, null, 2))
  console.log(
    'Native dialog Gaussian shadow pixels passed on four straight edges and four outer corners over the shared scrim in three themes'
  )
})

await check('dialog-overflow', async () => {
  await launch('', [
    '--pb',
    'paintTest',
    'true',
    '--pb',
    'paintDialog',
    'true',
    '--pb',
    'dialogOverflow',
    'true',
  ])
  await click(
    node => node.id === 'paint-dialog-trigger',
    'open overflowing dialog'
  )
  const current = await until(
    root =>
      flatten(root).some(node => node.id === 'dialog-scroll-body') && root,
    'overflow dialog body'
  )
  const nodes = flatten(current)
  const header = nodes.find(node => node.text === 'Overflow verification')
  const footer = nodes.find(node => node.id === 'dialog-pinned-footer')
  const body = findNode(current, node => node.id === 'dialog-scroll-body')
  const scroller = flatten(body).find(node => node.type === 'Scroll')
  assert.ok(
    scroller && visible(scroller),
    'The body owns a visible native scroll viewport'
  )
  const [left, top, right, bottom] = box(scroller)
  const headerBounds = box(header)
  const footerBounds = box(footer)
  const stage = box(nodes.find(node => node.id === 'dialog-paint-stage'))
  const density = (stage[2] - stage[0]) / 640
  const tokens = JSON.parse(
    fs.readFileSync(
      path.resolve('packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json'),
      'utf8'
    )
  )['studio-light']
  const metric = id => tokens[id].value * (tokens[id].unit === 'rem' ? 16 : 1)
  const footerHeight =
    metric('control.small.size') +
    2 * metric('spacing.3') +
    metric('stroke.hairline')
  assert.ok(
    Math.abs((footerBounds[3] - footerBounds[1]) / density - footerHeight) <= 1,
    `Pinned footer retains its full button, padding and border height: ${(footerBounds[3] - footerBounds[1]) / density} vs ${footerHeight}`
  )
  assert.ok(
    headerBounds[3] <= top + 1 && footerBounds[1] >= bottom - 1,
    'Header and footer sit outside the scrolling body'
  )
  const viewport = box(nodes.find(node => node.id === 'document-column'))
  assert.ok(
    headerBounds[1] >= viewport[1] && footerBounds[3] <= viewport[3],
    'The dialog actions fit within the viewport'
  )
  const surface = nodes.find(node =>
    node.id?.startsWith('dazzui-focus-dialog-')
  )
  const surfaceBounds = box(surface)
  assert.ok(
    Math.abs(
      (surfaceBounds[3] - surfaceBounds[1]) / density -
        ((viewport[3] - viewport[1]) / density - 2 * metric('spacing.6'))
    ) <= 2,
    'The overflowing dialog uses the shared viewport padding instead of a percentage inset'
  )
  assert.ok(
    !nodes.some(node => node.id === 'dialog-end' && visible(node)),
    'Long content initially extends below the body viewport'
  )
  for (let step = 0; step < 14; step++) {
    if (flatten(tree()).some(node => node.id === 'dialog-end' && visible(node)))
      break
    shell(
      'uitest',
      'uiInput',
      'swipe',
      String(Math.round((left + right) / 2)),
      String(Math.round(top + (bottom - top) * 0.8)),
      String(Math.round((left + right) / 2)),
      String(Math.round(top + (bottom - top) * 0.2))
    )
    await wait(200)
  }
  const after = flatten(tree())
  assert.ok(
    after.some(node => node.id === 'dialog-end' && visible(node)),
    'The final content action can be scrolled into view'
  )
  assert.deepEqual(
    box(after.find(node => node.text === 'Overflow verification')),
    headerBounds,
    'Header stays pinned'
  )
  assert.deepEqual(
    box(after.find(node => node.id === 'dialog-pinned-footer')),
    footerBounds,
    'Footer stays pinned'
  )
  await click(
    node => node.text === 'Close overflow dialog',
    'pinned footer remains actionable'
  )
  await until(
    root => !flatten(root).some(node => node.id === 'dialog-scroll-body'),
    'overflow dialog closes'
  )
  console.log(
    'Native overflowing dialog scrolls only its body, keeps header/footer pinned and preserves short-dialog sizing'
  )
})

await check('dialog-scenes', async () => {
  await launch('dialog')
  const rail = box(flatten(tree()).find(node => node.id === 'component-list'))
  const density = (rail[2] - rail[0]) / 272
  const tokens = JSON.parse(
    fs.readFileSync(
      path.resolve('packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json'),
      'utf8'
    )
  )['studio-light']
  const metric = id => tokens[id].value * (tokens[id].unit === 'rem' ? 16 : 1)
  const descend = node => [node, ...(node.items ?? []).flatMap(descend)]
  for (const example of stories.find(story => story.id === 'dialog').examples) {
    const declarations = descend(example.content)
    const header = declarations.find(node => node.name === 'DialogHeader')
    const footer = declarations.find(node => node.name === 'DialogFooter')
    const body = declarations.find(node => node.name === 'DialogBody')
    const address = node => example.id + '/' + node.address
    const trigger = address(example.content)
    await reveal(node => node.id === trigger, example.title)
    await click(node => node.id === trigger, 'open ' + example.title)
    const current = await until(
      root => flatten(root).some(node => node.id === address(footer)) && root,
      'real scene footer ' + example.id
    )
    const headerBounds = box(
      flatten(current).find(node => node.id === address(header))
    )
    const footerNode = findNode(current, node => node.id === address(footer))
    const footerBounds = box(footerNode.attributes)
    const buttons = flatten(footerNode).filter(node => node.type === 'Button')
    assert.ok(buttons.length, 'Real scene has footer actions')
    const buttonHeight = Math.max(
      ...buttons.map(node => {
        const b = box(node)
        return (b[3] - b[1]) / density
      })
    )
    const expected =
      buttonHeight + 2 * metric('spacing.3') + metric('stroke.hairline')
    assert.ok(
      Math.abs((footerBounds[3] - footerBounds[1]) / density - expected) <= 1,
      example.id + ': address wrappers preserve complete footer geometry'
    )
    assert.ok(
      (headerBounds[3] - headerBounds[1]) / density >= metric('spacing.11') - 1,
      example.id + ': header keeps its minimum height'
    )
    if (body) {
      const bounds = box(
        flatten(current).find(node => node.id === address(body))
      )
      assert.ok(
        headerBounds[3] <= bounds[1] + 1 && bounds[3] <= footerBounds[1] + 1,
        example.id + ': body stays between fixed chrome'
      )
    }
    await click(
      node => node.id === buttons[0].id,
      'close real scene through its first footer action'
    )
    await until(
      root => !flatten(root).some(node => node.id === address(footer)),
      'scene closes: ' + example.id
    )
  }
  console.log(
    'All five generated React dialog examples preserve header/body/footer geometry and dismiss through their live footer actions'
  )
})

await check('switch', async () => {
  await launch('switch')
  const rail = box(
    findNode(tree(), node => node.id === 'component-list').attributes
  )
  const density = (rail[2] - rail[0]) / 272
  const sizes = [
    [28, 16],
    [32, 18],
    [44, 24],
  ]
  for (const [index, [width, height]] of sizes.entries()) {
    const scope = section('switch/0')
    const control = findNode(scope, node => node.id === 'switch/0/' + index)
    assert.ok(control, 'Switch specimen ' + index)
    const bounds = box(control.attributes)
    assert.ok(
      Math.abs(bounds[2] - bounds[0] - width * density) <= 3,
      'Switch profile width has no extra native margin'
    )
    assert.ok(
      Math.abs(bounds[3] - bounds[1] - height * density) <= 3,
      'Switch profile height has no extra native margin'
    )
    tap(findNode(control, node => node.type === 'Toggle').attributes)
    await until(() => {
      const control = findNode(
        section('switch/0'),
        node => node.id === 'switch/0/' + index
      )
      return (
        findNode(control, node => node.type === 'Toggle').attributes.checked ===
        'false'
      )
    }, 'Native switch toggles after the margin reset')
  }
  console.log('Native switch size profiles and interaction passed')
})

await check('sidebar-paint', async () => {
  const themes = JSON.parse(
    fs.readFileSync(
      path.resolve('packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json'),
      'utf8'
    )
  )
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/sidebar-paint'
  )
  fs.mkdirSync(directory, { recursive: true })
  const reports = []
  const selectedTheme = option('--paint-theme')
  const selectedKind = option('--paint-sidebar')
  assert.ok(!selectedKind || ['floating', 'inset'].includes(selectedKind))
  for (const theme of selectedTheme
    ? [selectedTheme]
    : ['studio-light', 'studio-dark', 'bright-light']) {
    const tokens = themes[theme]
    const metric = id => tokens[id].value * (tokens[id].unit === 'rem' ? 16 : 1)
    for (const kind of selectedKind ? [selectedKind] : ['floating', 'inset']) {
      await launch('', [
        '--pb',
        'paintTest',
        'true',
        '--ps',
        'paintSidebar',
        kind,
        '--ps',
        'themeName',
        theme,
      ])
      const stage = box(
        flatten(tree()).find(node => node.id === 'sidebar-paint-stage')
      )
      const density = (stage[2] - stage[0]) / 640
      const prefix =
        kind === 'floating'
          ? 'dazzui-sidebar-surface-'
          : 'dazzui-sidebar-inset-surface-'
      const capture = async (phase, expectedWidth) => {
        let previous = ''
        const surface = await until(root => {
          const node = flatten(root).find(node => node.id?.startsWith(prefix))
          if (!node) return false
          const b = box(node)
          const matches =
            Math.abs((b[2] - b[0]) / density - expectedWidth) <= 1 &&
            Math.abs(
              (b[3] - b[1]) / density - (300 - 2 * metric('spacing.2'))
            ) <= 1
          const stable = previous === node.bounds
          previous = node.bounds
          return matches && stable && node
        }, 'settled sidebar ' + phase)
        const bounds = box(surface)
        const geometry = (await inspectNative(surface.id)).$geometry
        assert.ok(geometry, 'Record the independently queried native frame')
        assert.ok(
          Math.abs(geometry.frame.width - (bounds[2] - bounds[0])) <= 1 &&
            Math.abs(geometry.frame.height - (bounds[3] - bounds[1])) <= 1,
          'The screenshot bounds agree with the native measured size'
        )
        const stem = 'native-shadow-sm-sidebar-' + phase + '-' + theme
        const remote = '/data/local/tmp/dazzui-' + stem + '.png'
        shell('uitest', 'screenCap', '-p', remote)
        command('file', 'recv', remote, path.join(directory, stem + '.png'))
        const file = path.join(directory, stem + '.json')
        fs.writeFileSync(
          file,
          JSON.stringify(
            {
              theme,
              renderer: 'native-sidebar',
              role: 'shadow.sm',
              radiusRole: 'radius.large',
              backdropRole:
                kind === 'inset' ? 'color.surface-column' : undefined,
              geometry,
              density,
              stage,
              viewport: stage,
              bounds,
            },
            null,
            2
          )
        )
        const report = checkShadowPaint(file, themes)
        reports.push(report)
        assert.ok(
          report.passed,
          'Actual native sidebar pixels: ' + JSON.stringify(report)
        )
        return bounds
      }
      const expected =
        kind === 'floating'
          ? metric('frame.sidebar.width') - 2 * metric('spacing.2')
          : 300 - 2 * metric('spacing.2')
      const expanded = await capture(kind + '-expanded', expected)
      const counter = async (actions, underlay) =>
        until(
          root =>
            flatten(root).some(
              node =>
                node.id === 'sidebar-paint-counts' &&
                node.text === `Actions: ${actions}; underlay: ${underlay}`
            ),
          'sidebar foreground/background actions'
        )
      await click(
        node =>
          node.id ===
          (kind === 'floating'
            ? 'sidebar-paint-action'
            : 'sidebar-paint-pane-action'),
        'live sidebar foreground'
      )
      await counter(1, 0)
      const provider = box(
        flatten(tree()).find(node => node.id === 'sidebar-paint-provider')
      )
      shell(
        'uitest',
        'uiInput',
        'click',
        String(provider[0] - 6),
        String(Math.round((expanded[1] + expanded[3]) / 2))
      )
      await counter(1, 1)
      if (kind === 'floating') {
        await click(
          node => node.id === 'sidebar-paint-toggle',
          'collapse floating paint sidebar'
        )
        const collapsed = await capture(
          'floating-icon',
          metric('frame.sidebar.icon-width') + 2 * metric('stroke.hairline')
        )
        await click(
          node => node.id === 'sidebar-paint-action',
          'collapsed glyph remains actionable'
        )
        await counter(2, 1)
        shell(
          'uitest',
          'uiInput',
          'click',
          String(provider[0] - 6),
          String(Math.round((collapsed[1] + collapsed[3]) / 2))
        )
        await counter(2, 2)
        await click(
          node => node.id === 'sidebar-paint-toggle',
          'restore floating paint sidebar'
        )
        await capture('floating-restored', expected)
      }
    }
  }
  assert.equal(
    reports.length,
    (selectedTheme ? 1 : 3) *
      (!selectedKind ? 4 : selectedKind === 'floating' ? 3 : 1)
  )
  console.log(JSON.stringify(reports, null, 2))
  console.log(
    `Native sidebar Gaussian edge/corner pixels and control/input checks passed for ${reports.length} cases across ${new Set(reports.map(report => report.theme)).size} themes`
  )
})

await check('sidebar-labels', async () => {
  const originalBox = node =>
    (node.origBounds || node.bounds).match(/-?\d+/g).map(Number)
  let cases = 0
  for (const sizing of ['design', 'system']) {
    await launch('', [
      '--pb',
      'paintTest',
      'true',
      '--ps',
      'paintSidebar',
      'labels',
      '--ps',
      'sizing',
      sizing,
    ])
    const measure = long => {
      const root = tree()
      return ['small', 'medium', 'large'].map(size => {
        const scope = findNode(
          root,
          node => node.id === 'sidebar-label-button-' + size
        )
        const button = flatten(scope).find(node => node.type === 'Button')
        const label = flatten(scope).find(node => node.type === 'Text')
        const badge = flatten(root).find(
          node => node.id === 'sidebar-label-badge-' + size
        )
        assert.ok(
          button && label && badge,
          `${sizing}/${size}: expanded row parts`
        )
        assert.equal(
          label.text,
          long ? 'Documents and shared workspace resources' : 'Docs'
        )
        const labelBox = originalBox(label)
        const badgeBox = originalBox(badge)
        const buttonBox = originalBox(button)
        assert.ok(
          labelBox[2] < badgeBox[0],
          `${sizing}/${size}: label must end before badge; label ${labelBox}, badge ${badgeBox}`
        )
        assert.ok(
          labelBox[0] > buttonBox[0] && labelBox[2] > labelBox[0],
          `${sizing}/${size}: label retains positive space after its glyph`
        )
        assert.ok(
          labelBox[1] >= buttonBox[1] && labelBox[3] <= buttonBox[3],
          `${sizing}/${size}: label fits row height`
        )
        cases++
        return { label: labelBox, badge: badgeBox, button: buttonBox }
      })
    }
    const long = measure(true)
    await click(
      node => node.id === 'sidebar-label-length',
      'shorten sidebar labels'
    )
    await until(
      root =>
        ['small', 'medium', 'large'].every(size => {
          const row = findNode(
            root,
            node => node.id === 'sidebar-label-button-' + size
          )
          return (
            row &&
            flatten(row).some(
              node => node.type === 'Text' && node.text === 'Docs'
            )
          )
        }),
      'live sidebar labels update'
    )
    const short = measure(false)
    for (let i = 0; i < long.length; i++) {
      assert.deepEqual(
        short[i].label.slice(0, 1),
        long[i].label.slice(0, 1),
        'Label leading edge stays aligned when its content changes'
      )
      assert.deepEqual(
        short[i].button,
        long[i].button,
        'Label changes preserve row geometry'
      )
      assert.deepEqual(
        short[i].badge,
        long[i].badge,
        'Label changes preserve badge placement'
      )
    }
    await click(
      node => node.id === 'sidebar-label-toggle',
      'collapse label sidebar'
    )
    await until(
      root =>
        ['small', 'medium', 'large'].every(size => {
          const scope = findNode(
            root,
            node => node.id === 'sidebar-label-button-' + size
          )
          return (
            scope &&
            !flatten(scope).some(node => node.type === 'Text') &&
            !flatten(root).some(
              node => node.id === 'sidebar-label-badge-' + size
            )
          )
        }),
      'all labels and counts disappear in icon mode'
    )
    await click(
      node => node.id === 'sidebar-label-button-medium',
      'activate collapsed navigation'
    )
    await until(
      root =>
        flatten(root).some(
          node =>
            node.id === 'sidebar-label-actions' &&
            node.text === 'Label actions: 1'
        ),
      'collapsed menu retains its action'
    )
    await click(
      node => node.id === 'sidebar-label-toggle',
      'restore label sidebar'
    )
    await until(
      root =>
        flatten(root).some(node => node.id === 'sidebar-label-badge-medium'),
      'restore badge after icon mode'
    )
    measure(false)
  }
  assert.equal(cases, 18)
  for (const sizing of ['design', 'system']) {
    await launch('sidebar', ['--ps', 'sizing', sizing])
    for (const [scene, base] of [
      ['sidebar/0', 0],
      ['sidebar/1', 0],
      ['sidebar/2', 0],
      ['sidebar/2', 21],
    ]) {
      const id = `${scene}/${base + 9}`
      await reveal(node => node.id === id, `${sizing} gallery menu row`)
      const root = tree()
      const label = flatten(findNode(root, node => node.id === id)).find(
        node => node.type === 'Text'
      )
      const badge = flatten(root).find(
        node => node.id === `${scene}/${base + 11}`
      )
      assert.ok(
        label && badge,
        `${sizing}/${scene}: real gallery row and count`
      )
      assert.ok(
        originalBox(label)[2] < originalBox(badge)[0],
        `${sizing}/${scene}: real gallery label leaves the count clear`
      )
      cases++
    }
  }
  assert.equal(cases, 26)
  console.log(
    'Native Sidebar short/long labels stay clear of badges in all 3 row sizes, design/system sizing and restored icon mode; all 4 real gallery variants pass in both sizing modes'
  )
})

await check('sidebar', async () => {
  await launch('sidebar')
  const originalBox = node =>
    (node.origBounds || node.bounds).match(/-?\d+/g).map(Number)
  const current = (id, root = tree()) =>
    flatten(root).find(node => node.id === id)
  const parentOf = (node, id) => {
    if ((node.children ?? []).some(child => child.attributes.id === id))
      return node
    for (const child of node.children ?? []) {
      const found = parentOf(child, id)
      if (found) return found
    }
    return undefined
  }
  const frame = parentOf(tree(), 'sidebar/0/0')
  assert.equal(frame.attributes.type, 'Row')
  const frameBox = originalBox(frame.attributes)
  const density = (frameBox[2] - frameBox[0]) / 576
  const near = (actual, expected, label) =>
    assert.ok(
      Math.abs(actual - expected * density) <= 3,
      `${label}: ${actual}px; expected ${expected * density}px`
    )
  const width = (id, root = tree()) => {
    const item = current(id, root)
    if (!item) return 0
    const bounds = originalBox(item)
    return bounds[2] - bounds[0]
  }
  const button = (id, root = tree()) => {
    const scope = findNode(root, node => node.id === id)
    return flatten(scope).find(node => node.type === 'Button')
  }
  for (const [scene, base, x, y, rowWidth, insetWidth, insetHeight] of [
    ['sidebar/0', 0, 10, 85, 151, 402, 254],
    ['sidebar/1', 0, 10, 85, 151, 402, 254],
    ['sidebar/2', 0, 19, 94, 134, 402, 254],
    ['sidebar/2', 21, 18, 93, 136, 394, 238],
  ]) {
    await click(
      node => node.id === `${scene}/${base + 5}`,
      'show sidebar variant',
      true
    )
    const root = tree()
    const side = originalBox(current(`${scene}/${base + 1}`, root))
    const row = originalBox(current(`${scene}/${base + 9}`, root))
    near(side[2] - side[0], 172, 'Expanded sidebar width')
    near(row[0] - side[0], x, 'Menu leading inset')
    near(row[1] - side[1], y, 'First menu row position')
    near(row[2] - row[0], rowWidth, 'Menu row width')
    near(row[3] - row[1], 28, 'Menu row height')
    const badge = findNode(root, node => node.id === `${scene}/${base + 11}`)
    const badgeText = flatten(badge).find(node => node.type === 'Text')
    assert.ok(badgeText, 'Count badge is mounted beside the expanded menu row')
    const badgeBounds = originalBox(badgeText)
    near(row[2] - badgeBounds[2], 4, 'Count badge trailing inset')
    near(
      (badgeBounds[1] + badgeBounds[3] - row[1] - row[3]) / 2,
      0,
      'Count badge vertical alignment'
    )
    // The inset wrapper fills the pane; its inner surface accounts for margins.
    const pane = findNode(root, node => node.id === `${scene}/${base + 18}`)
    const surfaces = flatten(pane).filter(
      node => node.type === 'Column' && node.backgroundColor !== '#00000000'
    )
    const surface = originalBox(surfaces.at(-1))
    near(surface[2] - surface[0], insetWidth, 'Content surface width')
    near(surface[3] - surface[1], insetHeight, 'Content surface height')
  }
  await launch('sidebar')
  tap(current('sidebar/0/16'))
  await until(
    root => button('sidebar/0/16', root).selected === 'true',
    'Sidebar navigation changes pane'
  )
  assert.ok(
    flatten(findNode(tree(), node => node.id === 'sidebar/0/18')).some(
      node => node.text === 'Settings'
    )
  )
  tap(current('sidebar/0/20'))
  await until(
    root => width('sidebar/0/1', root) <= 2,
    'Offcanvas sidebar closes'
  )
  tap(current('sidebar/0/20'))
  await until(
    root => width('sidebar/0/1', root) > 170 * density,
    'Offcanvas sidebar returns'
  )
  assert.equal(
    button('sidebar/0/16').selected,
    'true',
    'Offcanvas retains selection'
  )
  tap(current('sidebar/1/20'))
  await until(
    root => Math.abs(width('sidebar/1/1', root) - 48 * density) <= 3,
    'Icon sidebar narrows'
  )
  near(width('sidebar/1/9'), 28, 'Collapsed menu square')
  assert.ok(
    !flatten(findNode(tree(), node => node.id === 'sidebar/1/9')).some(
      node => node.type === 'Text'
    )
  )
  assert.ok(width('sidebar/1/11') <= 2, 'Count is hidden in icon mode')
  tap(current('sidebar/1/13'))
  await until(
    root => button('sidebar/1/13', root).selected === 'true',
    'Collapsed icons still navigate'
  )
  tap(current('sidebar/1/20'))
  await until(
    root => width('sidebar/1/1', root) > 170 * density,
    'Icon sidebar expands'
  )
  assert.equal(button('sidebar/1/13').selected, 'true')
  const keys = (...codes) =>
    shell('uitest', 'uiInput', 'keyEvent', ...codes.map(String))
  const search = findNode(tree(), node => node.id === 'component-search')
  tap(flatten(search).find(node => node.type === 'TextInput'))
  keys(2072, 2018)
  await until(
    root =>
      !flatten(root).some(node => node.id === 'component-list') &&
      width('sidebar/0/1', root) <= 2 &&
      Math.abs(width('sidebar/1/1', root) - 48 * density) <= 3,
    'Ctrl B toggles the gallery and each mounted provider'
  )
  const scrollDocument = up => {
    const bounds = box(current('document-column'))
    const x = String(Math.round((bounds[0] + bounds[2]) / 2))
    const a = Math.round(bounds[1] + (bounds[3] - bounds[1]) * (up ? 0.3 : 0.8))
    const b = Math.round(bounds[1] + (bounds[3] - bounds[1]) * (up ? 0.8 : 0.3))
    shell('uitest', 'uiInput', 'swipe', x, String(a), x, String(b))
  }
  scrollDocument(false)
  await until(
    root =>
      current('sidebar/2/41', root) && visible(current('sidebar/2/41', root)),
    'Collapsed variants remain reachable'
  )
  assert.ok(
    width('sidebar/2/1') <= 2 && width('sidebar/2/22') <= 2,
    'The same window key closes floating and inset columns'
  )
  await click(
    node => node.id === 'appearance-toggle',
    'focus outside sidebar examples'
  )
  keys(2072, 2018)
  await until(
    root =>
      flatten(root).some(node => node.id === 'component-list') &&
      width('sidebar/2/1', root) > 170 * density &&
      width('sidebar/2/22', root) > 170 * density,
    'Ctrl B restores all providers from outside their subtrees'
  )
  for (let attempt = 0; attempt < 8; attempt++) {
    const target = current('sidebar/0/16')
    if (target && visible(target)) break
    scrollDocument(true)
  }
  near(width('sidebar/0/1'), 172, 'Offscreen offcanvas provider was restored')
  near(width('sidebar/1/1'), 172, 'Offscreen icon provider was restored')
  assert.equal(button('sidebar/0/16').selected, 'true')
  assert.equal(button('sidebar/1/13').selected, 'true')
  console.log(
    'Native Sidebar geometry, pane selection, collapse and window shortcut passed'
  )
})
await check('tooltip-paint', async () => {
  const themes = JSON.parse(
    fs.readFileSync(
      'packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json',
      'utf8'
    )
  )
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/tooltip'
  )
  fs.mkdirSync(directory, { recursive: true })
  const reports = []
  const multiline = option('--paint-label') === 'multiline'
  for (const theme of option('--paint-theme')
    ? [option('--paint-theme')]
    : ['studio-light', 'studio-dark', 'bright-light'])
    for (const side of option('--paint-side')
      ? [option('--paint-side')]
      : ['top', 'bottom', 'left', 'right']) {
      await launch('', [
        '--pb',
        'paintTest',
        'true',
        '--ps',
        'paintPopup',
        (multiline ? 'tooltip-multiline-' : 'tooltip-') + side,
        '--ps',
        'themeName',
        theme,
      ])
      const stage = box(
        flatten(tree()).find(node => node.id === 'tooltip-paint-stage')
      )
      const density = (stage[2] - stage[0]) / 640
      const target = box(
        flatten(tree()).find(node => node.id === 'tooltip-native-target')
      )
      shell(
        'uinput',
        '-M',
        '-m',
        String(Math.round((target[0] + target[2]) / 2)),
        String(Math.round((target[1] + target[3]) / 2))
      )
      const surface = await until(
        root =>
          flatten(root).find(node =>
            node.id.startsWith('dazzui-tooltip-surface-')
          ),
        'actual ' + side + ' tooltip after hover rest'
      )
      const bounds = box(surface)
      const gap =
        side === 'top'
          ? target[1] - bounds[3]
          : side === 'bottom'
            ? bounds[1] - target[3]
            : side === 'left'
              ? target[0] - bounds[2]
              : bounds[0] - target[2]
      assert.ok(
        Math.abs(gap / density - 4) <= 1,
        'Tooltip positioner keeps the source gap'
      )
      const stem =
        'native-tooltip-' + (multiline ? 'multiline-' : '') + theme + '-' + side
      const remote = '/data/local/tmp/' + stem + '.png'
      shell('uitest', 'screenCap', '-p', remote)
      command('file', 'recv', remote, path.join(directory, stem + '.png'))
      const file = path.join(directory, stem + '.json')
      fs.writeFileSync(
        file,
        JSON.stringify(
          {
            theme,
            side,
            label: multiline ? 'Helpful label\nSecond line' : 'Helpful label',
            renderer: 'native-tooltip',
            target,
            gap: gap / density,
            role: 'shadow.sm',
            radiusRole: 'radius.small',
            density,
            stage,
            bounds,
            viewport: box(
              flatten(tree()).find(node => node.id === 'document-column')
            ),
            native: await inspectNative(surface.id),
          },
          null,
          2
        )
      )
      const report = checkShadowPaint(file, themes)
      reports.push(report)
      fs.writeFileSync(
        path.join(directory, 'paint-results.json'),
        JSON.stringify(reports, null, 2)
      )
      assert.ok(
        report.passed,
        'Native tooltip Gaussian shadow: ' + JSON.stringify(report)
      )
      shell(
        'uinput',
        '-M',
        '-m',
        String(Math.round(stage[0] + 8 * density)),
        String(Math.round(stage[1] + 8 * density))
      )
      await until(
        root => !flatten(root).some(node => node.id === surface.id),
        'Tooltip closes on pointer exit'
      )
      console.log('Tooltip paint and placement passed: ' + theme + ' ' + side)
    }
})

await check('tooltip', async () => {
  await launch('', [
    '--pb',
    'paintTest',
    'true',
    '--ps',
    'paintPopup',
    'tooltip-top',
  ])
  const tips = root =>
    flatten(root).filter(node => node.id.startsWith('dazzui-tooltip-surface-'))
  const count = text =>
    until(
      root =>
        flatten(root).some(
          node => node.id === 'tooltip-actions' && node.text === text
        ),
      text
    )
  await click(
    node => node.id === 'tooltip-focus',
    'Keyboard-style focus on tooltip trigger'
  )
  await until(root => tips(root)[0], 'Focus opens the tooltip without hover')
  assert.ok(
    flatten(tree()).some(
      node => node.id === 'tooltip-native-target' && node.focused === 'true'
    ),
    'The popup does not take focus'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  await until(
    root => tips(root).length === 0,
    'Escape dismisses only the tooltip'
  )
  assert.ok(
    flatten(tree()).some(
      node => node.id === 'tooltip-native-target' && node.focused === 'true'
    )
  )
  await click(
    node => node.id === 'tooltip-focus',
    'Focus after blur resets Escape suppression'
  )
  const reopened = await until(
    root => tips(root)[0],
    'Refocusing reopens the tooltip'
  )
  const b = box(reopened)
  shell(
    'uitest',
    'uiInput',
    'click',
    String(Math.round((b[0] + b[2]) / 2)),
    String(Math.round((b[1] + b[3]) / 2))
  )
  await count('Trigger: 0; Under: 1')
  await until(
    root => tips(root).length === 0,
    'Press through the hint dismisses it'
  )
  await click(
    node => node.id === 'tooltip-native-target',
    'Touch activates the original target'
  )
  await count('Trigger: 1; Under: 1')
  await wait(750)
  assert.equal(tips(tree()).length, 0, 'Touch does not start a hover tooltip')
  await click(node => node.id === 'tooltip-disable', 'Disable the hint only')
  const target = box(
    flatten(tree()).find(node => node.id === 'tooltip-native-target')
  )
  shell(
    'uinput',
    '-M',
    '-m',
    String(Math.round((target[0] + target[2]) / 2)),
    String(Math.round((target[1] + target[3]) / 2))
  )
  await wait(750)
  assert.equal(tips(tree()).length, 0, 'Disabled hints stay closed')
  await click(
    node => node.id === 'tooltip-native-target',
    'Disabled hint keeps its control enabled'
  )
  await count('Trigger: 2; Under: 1')
  console.log(
    'Native Tooltip focus, Escape, pointer transparency, touch activation and disabled-hint checks passed'
  )
})

await check('tooltip-nesting', async () => {
  await launch('', [
    '--pb',
    'paintTest',
    'true',
    '--ps',
    'paintPopup',
    'tooltip-nesting',
  ])
  const tips = root =>
    flatten(root).filter(node => node.id.startsWith('dazzui-tooltip-surface-'))
  const target = flatten(tree()).find(node => node.id === 'tooltip-nested-icon')
  assert.ok(target)
  const b = box(target)
  shell(
    'uinput',
    '-M',
    '-m',
    String(Math.round((b[0] + b[2]) / 2)),
    String(Math.round((b[1] + b[3]) / 2))
  )
  await until(
    root => tips(root).some(node => node.text === 'Settings'),
    'The innermost icon hint opens'
  )
  assert.equal(tips(tree()).length, 1, 'Nested wrappers draw only one hint')
  assert.ok(
    box(tips(tree())[0])[3] < b[1],
    'The inner top placement wins over the outer bottom placement'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  await until(
    root => tips(root).length === 0,
    'One Escape dismisses the nested hint'
  )
  await wait(750)
  assert.equal(
    tips(tree()).length,
    0,
    'The outer wrapper does not reopen after Escape'
  )
  shell('uinput', '-M', '-m', '20', '120')
  await click(
    node => node.id === 'tooltip-nested-focus',
    'Keyboard focus on the nested icon'
  )
  await until(
    root => tips(root).length === 1 && tips(root)[0].text === 'Settings',
    'Keyboard opens one inner hint'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2054')
  await until(
    root =>
      flatten(root).some(
        node =>
          node.id === 'tooltip-nested-actions' && node.text === 'Actions: 1'
      ),
    'Enter still activates the icon exactly once'
  )
  assert.equal(tips(tree()).length, 0)
  await click(
    node => node.id === 'tooltip-nested-icon',
    'Touch the nested icon'
  )
  await until(
    root =>
      flatten(root).some(
        node =>
          node.id === 'tooltip-nested-actions' && node.text === 'Actions: 2'
      ),
    'Touch still activates the icon'
  )
  assert.equal(tips(tree()).length, 0)
  console.log(
    'Nested IconButton hints preserve the inner label/placement, one-Escape dismissal and pointer/keyboard actions'
  )
})

await check('tooltip-gallery', async () => {
  await launch('tooltip')
  const rail = box(flatten(tree()).find(node => node.id === 'component-list'))
  const density = (rail[2] - rail[0]) / 272
  const cases = [
    ['tooltip/0/1', 'Settings', 'top'],
    ...['top', 'bottom', 'left', 'right'].map((side, index) => [
      'tooltip/1/' + (index * 2 + 1),
      'On the ' + side,
      side,
    ]),
  ]
  for (const [id, label, side] of cases) {
    const target = box(await reveal(node => node.id === id, id))
    shell(
      'uinput',
      '-M',
      '-m',
      String(Math.round((target[0] + target[2]) / 2)),
      String(Math.round((target[1] + target[3]) / 2))
    )
    const hint = await until(
      root =>
        flatten(root).find(
          node =>
            node.id.startsWith('dazzui-tooltip-surface-') && node.text === label
        ),
      label
    )
    const b = box(hint)
    const gap =
      side === 'top'
        ? target[1] - b[3]
        : side === 'bottom'
          ? b[1] - target[3]
          : side === 'left'
            ? target[0] - b[2]
            : b[0] - target[2]
    assert.ok(
      Math.abs(gap / density - 4) <= 1,
      'Generated ' + side + ' placement'
    )
    shell('uitest', 'uiInput', 'keyEvent', '2070')
    await until(
      root =>
        !flatten(root).some(node =>
          node.id.startsWith('dazzui-tooltip-surface-')
        ),
      'Escape dismisses a hovered hint with focus elsewhere'
    )
    await wait(750)
    assert.ok(
      !flatten(tree()).some(node =>
        node.id.startsWith('dazzui-tooltip-surface-')
      ),
      'A stationary pointer does not undo Escape'
    )
    shell(
      'uinput',
      '-M',
      '-m',
      String(Math.round((rail[0] + rail[2]) / 2)),
      String(Math.round((rail[1] + rail[3]) / 2))
    )
  }
  console.log(
    'All five generated Tooltip triggers preserve source labels, sides and hovered Escape dismissal'
  )
})

await check('sidebar-tooltip', async () => {
  await launch('sidebar')
  await click(node => node.id === 'sidebar/1/20', 'Collapse sidebar to icons')
  const collapsed = await until(root => {
    const side = flatten(root).find(node => node.id === 'sidebar/1/1')
    const rail = flatten(root).find(node => node.id === 'component-list')
    const b = side && box(side)
    const r = rail && box(rail)
    return b && r && Math.abs(((b[2] - b[0]) / (r[2] - r[0])) * 272 - 48) < 2
      ? root
      : undefined
  }, 'Collapsed sidebar before hover')
  const rail = box(
    flatten(collapsed).find(node => node.id === 'component-list')
  )
  const density = (rail[2] - rail[0]) / 272
  const target = box(
    flatten(collapsed).find(node => node.id === 'sidebar/1/13')
  )
  shell(
    'uinput',
    '-M',
    '-m',
    String(Math.round((target[0] + target[2]) / 2)),
    String(Math.round((target[1] + target[3]) / 2))
  )
  const tooltip = await until(
    root =>
      flatten(root).find(
        node =>
          node.type === 'Text' &&
          node.text === 'Glossaries' &&
          node.backgroundColor !== '#00000000'
      ),
    'Collapsed icon tooltip'
  )
  const tip = (tooltip.origBounds || tooltip.bounds).match(/-?\d+/g).map(Number)
  assert.ok(
    Math.abs(tip[3] - tip[1] - 25 * density) <= 3,
    'Tooltip follows the shared label, padding and stroke'
  )
  assert.ok(
    Math.abs(tip[0] - target[2] - 4 * density) <= 3,
    'Tooltip uses the shared popup gap'
  )
  await click(
    node => node.id === 'sidebar/1/20',
    'Expand while tooltip is open'
  )
  await until(root => {
    const side = flatten(root).find(node => node.id === 'sidebar/1/1')
    const b = side && box(side)
    return b && Math.abs(b[2] - b[0] - 172 * density) <= 3
  }, 'Tooltip does not consume the trigger click')
  console.log(
    'Native Sidebar hover tooltip geometry and nonblocking dismissal passed'
  )
})
await check('sidebar-preview', async () => {
  await launch('')
  const search = findNode(tree(), node => node.id === 'component-search')
  tap(flatten(search).find(node => node.type === 'TextInput'))
  shell('uinput', '-K', '-t', 'Sidebar')
  const tile = await until(root => {
    const tile = findNode(root, node => node.id === 'overview-sidebar')
    return tile && visible(tile.attributes) ? tile : undefined
  }, 'Sidebar overview preview')
  const nodes = flatten(tile)
  assert.ok(!nodes.some(node => node.text === 'Mapping gaps'))
  const side = nodes.find(node => node.id === 'sidebar/preview/1')
  assert.ok(side)
  const rail = box(
    findNode(tree(), node => node.id === 'component-list').attributes
  )
  const density = (rail[2] - rail[0]) / 272
  const bounds = (side.origBounds || side.bounds).match(/-?\d+/g).map(Number)
  assert.ok(Math.abs(bounds[2] - bounds[0] - 150 * density) <= 3)
  assert.ok(Math.abs(bounds[3] - bounds[1] - 176 * density) <= 3)
  assert.ok(!nodes.some(node => node.description === 'Toggle sidebar'))
  await click(node => node.id === 'sidebar/preview/16', 'Preview settings row')
  assert.equal(
    flatten(findNode(tree(), node => node.id === 'component-search')).find(
      node => node.type === 'TextInput'
    ).text,
    'Sidebar',
    'Search survives losing focus to a preview control'
  )
  await until(root => {
    const pane = findNode(root, node => node.id === 'sidebar/preview/18')
    return pane && flatten(pane).some(node => node.text === 'Settings')
  }, 'Preview pane follows navigation')
  console.log(
    'Native Sidebar preview fixed width, height and live navigation passed'
  )
})
await check('sidebar-modal', async () => {
  await launch('', ['--pb', 'modalTest', 'true'])
  await click(node => node.text === 'Edit sidebar', 'sidebar probe trigger')
  const current = (id, root = tree()) =>
    flatten(root).find(node => node.id === id)
  const width = (id, root = tree()) => {
    const node = current(id, root)
    return node
      ? (node.origBounds || node.bounds)
          .match(/-?\d+/g)
          .map(Number)
          .reduce((n, value, i, values) => (i === 2 ? value - values[0] : n), 0)
      : 0
  }
  const status = (id, text) =>
    until(root => current(id, root)?.text === text, text)
  const keys = (...codes) =>
    shell('uitest', 'uiInput', 'keyEvent', ...codes.map(String))
  const trigger = id =>
    flatten(findNode(tree(), node => node.id === id)).find(
      node => node.type === 'Button'
    )
  await click(node => node.text === 'Measure sidebars', 'native layout report')
  const report = await until(root => {
    const value = current('sidebar-probe-report', root)?.text
    return value?.startsWith('Scale: ') ? value : undefined
  }, 'native scale report')
  const scale = Number(report.match(/Scale: ([\d.]+)/)[1])
  const near = (actual, expected, label) =>
    assert.ok(Math.abs(actual - expected * scale) <= 3, label)
  near(
    width('sidebar-probe-accepted-side'),
    64,
    'Controlled inset starts collapsed'
  )
  near(
    width('sidebar-probe-local-side'),
    64,
    'Uncontrolled right floating column starts collapsed'
  )
  near(
    width('sidebar-probe-refused-side'),
    172,
    'Refused column starts expanded'
  )
  tap(trigger('sidebar-probe-frame'))
  await status('sidebar-probe-accepted', 'Accepted: 1')
  near(
    width('sidebar-probe-accepted-side'),
    172,
    'Accepted controlled expansion'
  )
  near(
    width('sidebar-probe-local-side'),
    64,
    'Pointer toggle is scoped to its provider'
  )
  tap(trigger('sidebar-probe-local'))
  await until(
    root => width('sidebar-probe-local-side', root) > 210 * scale,
    'Right column opens'
  )
  near(
    width('sidebar-probe-local-side'),
    214,
    'Right column uses the aside width'
  )
  tap(trigger('sidebar-probe-refused'))
  await status('sidebar-probe-attempts', 'Refused: 1')
  near(
    width('sidebar-probe-refused-side'),
    172,
    'Refused controlled state remains unchanged'
  )
  let measureFocused = false
  for (let i = 0; i < 12; i++) {
    const target = findNode(
      tree(),
      node => node.type === 'Button' && node.focused === 'true'
    )
    if (
      target &&
      flatten(target).some(node => node.text === 'Measure sidebars')
    ) {
      measureFocused = true
      break
    }
    keys(2049)
  }
  assert.ok(measureFocused, 'Focus outside all provider subtrees')
  keys(2072, 2018)
  await status('sidebar-probe-accepted', 'Accepted: 2')
  await status('sidebar-probe-attempts', 'Refused: 2')
  near(
    width('sidebar-probe-accepted-side'),
    64,
    'Modal window key toggles controlled column'
  )
  near(
    width('sidebar-probe-local-side'),
    64,
    'Modal window key toggles uncontrolled column'
  )
  const recorder = flatten(tree()).find(node =>
    /^dazzui-recorder-\d+$/.test(node.id)
  )
  tap(recorder)
  keys(2072, 2018)
  await until(
    root => current(recorder.id, root)?.description === '⌃B',
    'Recorder captures the sidebar chord'
  )
  await status('sidebar-probe-accepted', 'Accepted: 2')
  await status('sidebar-probe-attempts', 'Refused: 2')
  keys(2070)
  await until(
    root => !flatten(root).some(node => node.text === 'Sidebar verification'),
    'Sidebar probe closes'
  )
  console.log(
    'Native controlled/default/refused Sidebar, right column and shortcut cancellation passed; ' +
      report
  )
})
await check('tree', async () => {
  await launch('tree')
  const scope = (current = tree(), index = 0) =>
    findNode(current, node => node.id === 'example-tree/' + index)
  const rows = (current = tree(), index = 0) =>
    flatten(scope(current, index)).filter(
      node => /^dazzui-tree-\d+-/.test(node.id) && !node.id.endsWith('-twisty')
    )
  const row = (id, current = tree(), index = 0) =>
    rows(current, index).find(node => node.id.endsWith('-' + id))
  const keys = (...codes) =>
    shell('uitest', 'uiInput', 'keyEvent', ...codes.map(String))
  const focused = id =>
    until(current => row(id, current)?.focused === 'true', 'Tree focus: ' + id)
  const selected = id =>
    until(
      current => row(id, current)?.selected === 'true',
      'Tree selection: ' + id
    )
  assert.equal(rows().length, 8)
  await selected('tree.tsx')
  const rail = box(
    findNode(tree(), node => node.id === 'component-list').attributes
  )
  const density = (rail[2] - rail[0]) / 272
  const src = box(row('src'))
  assert.ok(
    Math.abs(src[2] - src[0] - 320 * density) <= 3,
    'Tree fills its React story column'
  )
  assert.ok(
    Math.abs(src[3] - src[1] - 28 * density) <= 3,
    'Tree medium row height'
  )
  const twisty = flatten(scope()).find(
    node => node.id === row('src').id + '-twisty'
  )
  tap(twisty)
  await until(current => rows(current).length === 3, 'Twisty closes branch')
  assert.ok(
    !rows().some(node => node.selected === 'true'),
    'Twisty does not select its folder'
  )
  tap(flatten(scope()).find(node => node.id === row('src').id + '-twisty'))
  await until(current => rows(current).length === 8, 'Twisty reopens branch')
  await selected('tree.tsx')
  tap(row('tree.tsx'))
  keys(2014)
  await focused('components')
  await selected('tree.tsx')
  keys(2014)
  await until(
    current => rows(current).length === 6,
    'Left closes focused folder'
  )
  keys(2015)
  await until(current => rows(current).length === 8, 'Right opens folder')
  await focused('components')
  keys(2015)
  await focused('button.tsx')
  keys(2013)
  await focused('tree.tsx')
  keys(2082)
  await focused('README.md')
  keys(2054)
  await selected('README.md')
  keys(2081)
  await focused('src')
  keys(2050)
  await selected('src')
  await until(
    current => rows(current).length === 3,
    'Space selects and closes folder'
  )
  keys(2015)
  await until(current => rows(current).length === 8, 'Right restores children')
  // One continuous physical sequence must fit the shared type-ahead window.
  shell(
    'uinput',
    '-K',
    '-d',
    '2036',
    '-u',
    '2036',
    '-i',
    '40',
    '-d',
    '2034',
    '-u',
    '2034'
  )
  await focused('tree.tsx')
  await selected('src')
  keys(2082)
  await focused('README.md')
  keys(2049)
  await until(
    current =>
      scope(current).attributes.focused !== 'true' &&
      scope(current, 1).attributes.focused === 'true',
    'Tab leaves whole tree'
  )
  keys(2047, 2049)
  await until(
    current => scope(current).attributes.focused === 'true',
    'Shift Tab returns to tree'
  )
  keys(2054)
  await selected('README.md')
  const changed = rows(tree(), 1)
  assert.equal(changed.length, 4)
  const blocked = row('c', tree(), 1)
  tap(blocked)
  assert.equal(row('c', tree(), 1).selected, 'false')
  keys(2081)
  keys(2015)
  keys(2013)
  keys(2013)
  await until(
    current => row('b', current, 1)?.focused === 'true',
    'Disabled adjacent row blocks ArrowDown as in React'
  )
  keys(2082)
  await until(
    current => row('c', current, 1)?.focused === 'true',
    'End can focus an unavailable row'
  )
  keys(2054)
  assert.equal(row('c', tree(), 1).selected, 'false')
  const surface = flatten(tree()).find(
    node => node.id === 'document-column'
  ).backgroundColor
  const beforeTheme = box(row('README.md'))
  await click(node => node.id === 'appearance-toggle', 'Tree dark mode')
  await until(
    current =>
      flatten(current).find(node => node.id === 'document-column')
        .backgroundColor !== surface &&
      row('README.md', current)?.selected === 'true',
    'Tree selection survives palette change'
  )
  assert.deepEqual(
    box(row('README.md')),
    beforeTheme,
    'Tree geometry is independent of palette'
  )
  console.log(
    'Native Tree pointer, controlled selection, navigation and type-ahead passed'
  )
})
await check('tree-modal', async () => {
  await launch('', ['--pb', 'modalTest', 'true'])
  await click(node => node.text === 'Edit tree', 'tree probe trigger')
  const nodes = (current = tree()) => flatten(current)
  const row = (id, current = tree()) =>
    nodes(current).find(
      node => /^dazzui-tree-\d+-/.test(node.id) && node.id.endsWith('-' + id)
    )
  const keys = (...codes) =>
    shell('uitest', 'uiInput', 'keyEvent', ...codes.map(String))
  const status = (id, value) =>
    until(
      current =>
        nodes(current).some(node => node.id === id && node.text === value),
      value
    )
  await status('tree-expanded', 'Expanded: ')
  tap(row('root'))
  await status('tree-expanded', 'Expanded: root')
  await status('tree-selected', 'Selected: root')
  keys(2015)
  await until(
    current => row('empty', current)?.focused === 'true',
    'Right descends controlled tree'
  )
  keys(2015)
  await status('tree-expanded', 'Expanded: root,empty')
  assert.ok(
    row('empty').description.includes('expanded'),
    'Empty branch has open state'
  )
  keys(2015)
  await until(
    current => row('leaf', current)?.focused === 'true',
    'Right on empty open folder moves to next visible row'
  )
  keys(2054)
  await status('tree-selected', 'Selected: leaf')
  await click(node => node.text === 'Rename tree', 'rename visible nodes')
  await until(
    current =>
      ['Renamed folder', 'Renamed file'].every(label =>
        nodes(current).some(
          node => node.description === label || node.text === label
        )
      ),
    'Stable row IDs refresh changed labels'
  )
  tap(row('root'))
  await status('tree-expanded', 'Expanded: empty')
  keys(2015)
  await status('tree-expanded', 'Expanded: empty,root')
  // Match the explicit `text`, independently of the renamed visible label.
  shell('uinput', '-K', '-d', '2030', '-u', '2030')
  await until(
    current => row('leaf', current)?.focused === 'true',
    'Tree text override type-ahead'
  )
  keys(2049)
  await until(current => {
    const button = findNode(
      current,
      node => node.type === 'Button' && node.focused === 'true'
    )
    return button && flatten(button).some(node => node.text === 'After tree')
  }, 'Tree is one Tab stop')
  keys(2047, 2049)
  await until(
    current => row('leaf', current)?.focused === 'true',
    'Shift Tab restores roving tree stop'
  )
  keys(2054)
  await status('tree-selected', 'Selected: leaf')
  tap(row('refused'))
  await status('tree-attempts', 'Attempts: 2')
  assert.ok(row('refused').description.includes('collapsed'))
  assert.equal(
    row('refused').selected,
    'false',
    'Refused controlled selection stays unchanged'
  )
  keys(2070)
  await until(
    current => !nodes(current).some(node => node.text === 'Tree verification'),
    'Tree leaves Escape to native dialog'
  )
  console.log(
    'Native controlled Tree updates, empty branches, Tab and modal Escape passed'
  )
})
await check('tree-preview', async () => {
  await launch('')
  const search = findNode(tree(), node => node.id === 'component-search')
  tap(flatten(search).find(node => node.type === 'TextInput'))
  shell('uinput', '-K', '-t', 'Tree')
  const tile = await until(current => {
    const tile = findNode(current, node => node.id === 'overview-tree')
    return tile && visible(tile.attributes) ? tile : undefined
  }, 'Tree overview preview')
  const rows = flatten(tile).filter(
    node => /^dazzui-tree-\d+-/.test(node.id) && !node.id.endsWith('-twisty')
  )
  assert.equal(rows.length, 8)
  assert.ok(!flatten(tile).some(node => node.text === 'Mapping gaps'))
  const rail = box(
    findNode(tree(), node => node.id === 'component-list').attributes
  )
  const density = (rail[2] - rail[0]) / 272
  const bounds = box(rows[0])
  assert.ok(
    Math.abs(bounds[2] - bounds[0] - 240 * density) <= 3,
    'Tree preview column width'
  )
  assert.ok(
    Math.abs(bounds[3] - bounds[1] - 24 * density) <= 3,
    'Tree small row height'
  )
  console.log('Native Tree preview width, small rows and hierarchy passed')
})
await check('otp-field', async () => {
  await launch('otp-field')
  const fields = (scene, current = tree()) => {
    const scope = findNode(current, node => node.id === 'example-' + scene)
    return scope
      ? flatten(scope).filter(
          node =>
            node.type === 'TextInput' && /^dazzui-otp-\d+-\d+$/.test(node.id)
        )
      : []
  }
  const initial = tree()
  const rail = box(
    findNode(initial, node => node.id === 'component-list').attributes
  )
  const density = (rail[2] - rail[0]) / 272
  const plain = fields('otp-field/0', initial)
  const grouped = fields('otp-field/1', initial)
  const profiles = fields('otp-field/2', initial)
  const cellBox = node =>
    findNode(initial, item => item.id === node.id + '-box').attributes
  assert.equal(plain.length, 6)
  assert.equal(grouped.length, 6)
  assert.equal(profiles.length, 12)
  const dimensions = (node, size) => {
    const [left, top, right, bottom] = box(node)
    assert.ok(Math.abs(right - left - size * density) <= 2, 'OTP cell width')
    assert.ok(Math.abs(bottom - top - size * density) <= 2, 'OTP cell height')
  }
  plain.forEach(node => dimensions(cellBox(node), 28))
  profiles.forEach((node, index) =>
    dimensions(cellBox(node), index < 4 ? 26 : index < 8 ? 32 : 28)
  )
  assert.ok(
    Math.abs(
      box(cellBox(plain[1]))[0] - box(cellBox(plain[0]))[2] - 8 * density
    ) <= 2
  )
  assert.ok(
    Math.abs(
      box(cellBox(grouped[3]))[0] - box(cellBox(grouped[2]))[2] - 20 * density
    ) <= 2
  )
  const currentCell = (index, current = tree()) =>
    fields('otp-field/0', current)[index]
  const focus = index =>
    until(
      current => currentCell(index, current)?.focused === 'true',
      'OTP cell ' + index + ' focuses'
    )
  const keys = (...codes) =>
    shell('uitest', 'uiInput', 'keyEvent', ...codes.map(String))
  tap(plain[5])
  await focus(0)
  keys(2001)
  await focus(1)
  keys(2002)
  await focus(2)
  await until(
    current =>
      currentCell(0, current).text === '1' &&
      currentCell(1, current).text === '2',
    'numeric input advances'
  )
  keys(2014)
  await focus(1)
  keys(2009)
  await focus(2)
  await until(
    current => currentCell(1, current).text === '9',
    'A focused digit is overwritten'
  )
  keys(2055)
  await focus(1)
  await until(
    current =>
      currentCell(0, current).text === '1' &&
      currentCell(1, current).text === '',
    'Backspace from an empty cell removes the previous digit'
  )
  keys(2081)
  await focus(0)
  keys(2071)
  await until(
    current => currentCell(0, current).text === '',
    'Delete removes the focused digit'
  )
  keys(2014)
  await focus(0)
  shell('uinput', '-K', '-t', '123456')
  await focus(5)
  await until(
    current =>
      fields('otp-field/0', current)
        .map(node => node.text)
        .join('') === '123456',
    'A complete native insertion fills every cell'
  )
  keys(2014)
  await focus(4)
  keys(2049)
  await until(
    current => fields('otp-field/1', current)[0]?.focused === 'true',
    'Tab leaves a filled code'
  )
  keys(2047, 2049)
  await focus(5)
  keys(2072, 2055)
  await focus(0)
  await until(
    current => fields('otp-field/0', current).every(node => !node.text),
    'Ctrl Backspace clears the code'
  )
  for (const value of ['987654', '012345']) {
    shell('uinput', '-K', '-t', value)
    await focus(5)
    await until(
      current =>
        fields('otp-field/0', current)
          .map(node => node.text)
          .join('') === value,
      'Rapid native typing retains every digit'
    )
    keys(2072, 2055)
    await focus(0)
  }
  keys(2017)
  await until(
    current =>
      currentCell(0, current).text === '' &&
      currentCell(0, current).focused === 'true',
    'Numeric validation rejects a letter'
  )
  keys(2049)
  await until(
    current => fields('otp-field/1', current)[0]?.focused === 'true',
    'One Tab leaves the OTP group'
  )
  assert.ok(
    fields('otp-field/0').every(node => !node.text),
    'Separate groups retain independent state'
  )
  console.log(
    'Native OTP dimensions, grouping, typing, overwrite, deletion, filtering, focus and Tab traversal passed'
  )
})

await check('otp-theme', async () => {
  await launch('otp-field')
  const fields = current =>
    flatten(
      findNode(current, node => node.id === 'example-otp-field/0')
    ).filter(
      node => node.type === 'TextInput' && /^dazzui-otp-\d+-\d+$/.test(node.id)
    )
  const start = fields(tree())[0]
  tap(start)
  shell('uinput', '-K', '-t', '654321')
  await until(
    current =>
      fields(current)
        .map(node => node.text)
        .join('') === '654321',
    'Code before palette switch'
  )
  const before = findNode(
    tree(),
    node => node.id === start.id + '-box'
  ).attributes
  await click(node => node.id === 'appearance-toggle', 'dark appearance')
  await until(current => {
    const changed = findNode(
      current,
      node => node.id === start.id + '-box'
    )?.attributes
    return (
      changed &&
      changed.backgroundColor !== before.backgroundColor &&
      fields(current)
        .map(node => node.text)
        .join('') === '654321'
    )
  }, 'A live palette switch recolors the retained code')
  const after = findNode(
    tree(),
    node => node.id === start.id + '-box'
  ).attributes
  assert.deepEqual(
    box(after),
    box(before),
    'Palette switching preserves cell dimensions and position'
  )
  await click(node => node.id === 'appearance-toggle', 'light appearance')
  await until(
    current =>
      findNode(current, node => node.id === start.id + '-box')?.attributes
        .backgroundColor === before.backgroundColor,
    'Light appearance restores'
  )
  console.log(
    'Native OTP palette switching preserves live values and cell geometry'
  )
})

await check('otp-modal', async () => {
  await launch('', ['--pb', 'modalTest', 'true'])
  await click(node => node.text === 'Edit code', 'OTP probe trigger')
  await until(
    current => flatten(current).some(node => node.text === 'Code verification'),
    'OTP probe opens'
  )
  const fields = (current = tree()) =>
    flatten(current).filter(
      node => node.type === 'TextInput' && /^dazzui-otp-\d+-\d+$/.test(node.id)
    )
  assert.equal(fields().length, 18)
  const keys = (...codes) =>
    shell('uitest', 'uiInput', 'keyEvent', ...codes.map(String))
  const status = (id, value) =>
    until(
      current =>
        flatten(current).some(node => node.id === id && node.text === value),
      value
    )
  await click(node => node.text === 'Submit code', 'required code submit')
  await status('otp-submitted', 'Submitted: 0')
  assert.equal(fields()[0].focused, 'true')
  await click(node => node.description === 'Copy code', 'copy synthetic OTP')
  await until(
    current => flatten(current).some(node => node.description === 'Copied'),
    'Synthetic code is copied'
  )
  tap(fields()[0])
  keys(2072, 2038)
  await status('otp-stored', 'Code: 123456')
  assert.equal(
    fields()
      .slice(0, 6)
      .map(node => node.text)
      .join(''),
    '123456'
  )
  await status('otp-completed', 'Completed: 1')
  await status('otp-rejected', 'Rejected: 1')
  await status('otp-submitted', 'Submitted: 1')
  tap(fields()[0])
  keys(2072, 2038)
  await status('otp-stored', 'Code: 123456')
  await status('otp-completed', 'Completed: 2')
  await status('otp-submitted', 'Submitted: 2')
  await click(node => node.text === 'Reset code', 'reset controlled code')
  await status('otp-stored', 'Code: ')
  assert.ok(
    fields()
      .slice(0, 6)
      .every(node => !node.text)
  )
  tap(fields()[0])
  shell('uinput', '-K', '-t', '246810')
  await status('otp-stored', 'Code: 246810')
  await status('otp-completed', 'Completed: 3')
  await status('otp-submitted', 'Submitted: 3')
  assert.equal(
    fields()
      .slice(0, 6)
      .map(node => node.text)
      .join(''),
    '246810',
    'Controlled rapid typing retains every digit'
  )
  tap(fields()[6])
  keys(2009)
  keys(2055)
  assert.equal(
    fields()
      .slice(6, 10)
      .map(node => node.text)
      .join(''),
    '1234',
    'Read-only code retains its value'
  )
  assert.ok(
    fields()
      .slice(10, 14)
      .every(node => node.enabled === 'false'),
    'Every disabled cell is disabled'
  )
  tap(fields()[14])
  keys(2009)
  await status('otp-attempted', 'Attempted: 1')
  assert.equal(
    fields()
      .slice(14)
      .map(node => node.text)
      .join(''),
    '11',
    'A refused controlled value stays unchanged'
  )
  console.log(
    'Native OTP clipboard paste, filtering, callbacks, auto-submit, required validation, reset, read-only, disabled and refused controlled values passed'
  )
})

await check('shortcut-recorder', async () => {
  await launch('shortcut-recorder')
  const recorders = id =>
    flatten(section(id)).filter(
      node => node.type === 'Button' && /^dazzui-recorder-\d+$/.test(node.id)
    )
  const first = recorders('shortcut-recorder/0')[0].id
  const second = recorders('shortcut-recorder/0')[1].id
  const current = (id, root = tree()) =>
    flatten(root).find(node => node.id === id)
  const value = (id, label) =>
    until(
      root => current(id, root)?.description === label,
      'Shortcut label: ' + label
    )
  const keys = (...codes) =>
    shell('uitest', 'uiInput', 'keyEvent', ...codes.map(String))
  const begin = async id => {
    tap(current(id))
    await value(id, 'Press keys…')
  }
  const rail = box(
    findNode(tree(), node => node.id === 'component-list').attributes
  )
  const density = (rail[2] - rail[0]) / 272
  const bounds = box(current(first))
  assert.ok(
    Math.abs(bounds[2] - bounds[0] - 132 * density) <= 3,
    'Recorder minimum width comes from ds'
  )
  assert.ok(
    Math.abs(bounds[3] - bounds[1] - 28 * density) <= 3,
    'Recorder medium field height'
  )
  await begin(first)
  // uinput retains pressed-key items only within one invocation. A single
  // continuous sequence models one physical keyboard, including key releases.
  const heldModifiers = new Promise(resolve => {
    execFile(
      hdc,
      [
        '-t',
        device,
        'shell',
        'uinput',
        '-K',
        '-d',
        '2045',
        '-i',
        '8000',
        '-d',
        '2047',
        '-i',
        '8000',
        '-u',
        '2047',
        '-i',
        '8000',
        '-u',
        '2045',
      ],
      { timeout: 35000 },
      error => resolve(error)
    )
  })
  try {
    await value(first, '⌥')
    await value(first, '⌥⇧')
    await value(first, '⌥')
  } finally {
    await heldModifiers
    shell(
      'uinput',
      '-K',
      '-d',
      '2047',
      '-u',
      '2047',
      '-d',
      '2045',
      '-u',
      '2045'
    )
  }
  const heldError = await heldModifiers
  if (heldError) throw heldError
  await value(first, 'Press keys…')
  const heldControl = new Promise(resolve => {
    execFile(
      hdc,
      [
        '-t',
        device,
        'shell',
        'uinput',
        '-K',
        '-d',
        '2072',
        '-i',
        '15000',
        '-u',
        '2072',
      ],
      { timeout: 20000 },
      error => resolve(error)
    )
  })
  try {
    await value(first, '⌃')
    tap(current(first))
    await value(first, '⌃')
  } finally {
    await heldControl
    shell('uinput', '-K', '-d', '2072', '-u', '2072')
  }
  const controlError = await heldControl
  if (controlError) throw controlError
  await value(first, 'Press keys…')
  keys(2036)
  await value(first, 'Press keys…')
  assert.equal(
    current(first).selected,
    'true',
    'Plain typing keeps recording active'
  )
  keys(2049)
  await value(first, 'Press keys…')
  assert.equal(
    current(first).focused,
    'true',
    'Unmodified Tab is consumed while recording'
  )
  keys(2072, 2047, 2036)
  await value(first, '⌃⇧T')
  assert.equal(current(first).selected, 'false')
  assert.equal(
    current(second).description,
    '⌥ Space',
    'Other recorder state stays isolated'
  )
  await begin(first)
  keys(2070)
  await value(first, '⌃⇧T')
  await begin(first)
  keys(2055)
  await value(first, 'Record shortcut')
  await begin(first)
  keys(2094)
  await value(first, 'F5')
  await begin(first)
  keys(2072, 2050)
  await value(first, '⌃ Space')
  await begin(first)
  tap(current(second))
  await value(first, '⌃ Space')
  await value(second, 'Press keys…')
  keys(2070)
  await value(second, '⌥ Space')
  const clear = current(first + '-clear')
  assert.ok(clear, 'Native clear button')
  tap(clear)
  await value(first, 'Record shortcut')
  tap(current(first))
  keys(2070)
  keys(2054)
  await value(first, 'Press keys…')
  keys(2072, 2027)
  await value(first, '⌃K')
  keys(2049)
  await until(
    () => current(first + '-clear')?.focused === 'true',
    'Tab reaches the clear button'
  )
  keys(2054)
  await value(first, 'Record shortcut')
  tap(current(first))
  keys(2070)
  keys(2050)
  await value(first, 'Press keys…')
  keys(2070)
  await value(first, 'Record shortcut')
  await click(
    node =>
      node.type === 'Button' &&
      node.id.startsWith('dazzui-recorder-') &&
      node.description === '⌥⇧2',
    'disabled shortcut recorder',
    true
  )
  const disabled = recorders('shortcut-recorder/1').find(
    node => node.description === '⌥⇧2'
  )
  assert.ok(disabled)
  assert.equal(disabled.enabled, 'false')
  assert.equal(disabled.selected, 'false')
  await click(
    node =>
      node.type === 'Button' &&
      node.id.startsWith('dazzui-recorder-') &&
      node.description === '⌘C',
    'conflicting shortcut recorder',
    true
  )
  keys(2072, 2027)
  await until(
    () =>
      recorders('shortcut-recorder/1').some(
        node => node.description === '⌘C' && node.selected === 'false'
      ),
    'Controlled recorder preserves rejected host value'
  )
  console.log(
    'Native shortcut recording, modifiers, cancel, clear, keyboard, function keys, isolation and disabled state passed'
  )
})

await check('shortcut-background', async () => {
  await launch('shortcut-recorder')
  const unbound = node =>
    node.type === 'Button' &&
    /^dazzui-recorder-\d+$/.test(node.id) &&
    node.description === 'Record shortcut'
  await click(unbound, 'unbound recorder')
  const recording = current =>
    flatten(current).find(
      node =>
        node.type === 'Button' &&
        /^dazzui-recorder-\d+$/.test(node.id) &&
        node.selected === 'true'
    )
  await until(recording, 'Recorder listens on the document page')
  await click(
    node =>
      node.text === 'A field that takes the keys pressed in it as its value.',
    'noninteractive document summary'
  )
  await until(
    current => !recording(current),
    'A document background press cancels recording'
  )
  await click(unbound, 'unbound recorder')
  const active = await until(recording, 'Recorder listens before mouse input')
  const mouseClick = node => {
    const [left, top, right, bottom] = box(node)
    shell(
      'uinput',
      '-M',
      '-m',
      String(Math.round((left + right) / 2)),
      String(Math.round((top + bottom) / 2)),
      '-c',
      '0'
    )
  }
  mouseClick(active)
  assert.ok(recording(tree()), 'An internal mouse click keeps recording')
  mouseClick(
    flatten(tree()).find(
      node =>
        node.text === 'A field that takes the keys pressed in it as its value.'
    )
  )
  await until(
    current => !recording(current),
    'A mouse press on the document background cancels recording'
  )
  console.log('Native shortcut touch and mouse background cancellation passed')
})

await check('shortcut-modal', async () => {
  await launch('', ['--pb', 'modalTest', 'true'])
  await click(node => node.text === 'Edit shortcut', 'modal shortcut trigger')
  const title = current =>
    flatten(current).some(node => node.text === 'Shortcut modal')
  await until(title, 'shortcut dialog opens')
  const recorder = () =>
    flatten(tree()).find(
      node => node.type === 'Button' && /^dazzui-recorder-\d+$/.test(node.id)
    )
  const keys = (...codes) =>
    shell('uitest', 'uiInput', 'keyEvent', ...codes.map(String))
  tap(recorder())
  await until(
    () => recorder()?.description === 'Listening…',
    'Custom recording label'
  )
  keys(2070)
  await until(
    current => title(current) && recorder()?.description === '⌃T',
    'Escape cancels recording while preserving the dialog'
  )
  tap(recorder())
  await until(
    () => recorder()?.description === 'Listening…',
    'Recorder re-enters listening before clear'
  )
  keys(2055)
  await until(
    () => recorder()?.description === 'Choose keys',
    'Clear falls back to onValueChange without onClear'
  )
  tap(recorder())
  await until(
    () => recorder()?.description === 'Listening…',
    'Recorder re-enters listening before a chord'
  )
  keys(2072, 2027)
  await until(
    () => recorder()?.description === '⌃K',
    'Modal controlled value updates'
  )
  tap(recorder())
  await until(
    () => recorder()?.description === 'Listening…',
    'Recorder listens before blur'
  )
  await click(node => node.text === 'Other modal control', 'blur target')
  await until(
    () => recorder()?.description === '⌃K' && recorder()?.selected === 'false',
    'Blur cancels recording'
  )
  await until(
    current =>
      flatten(current).some(
        node =>
          node.id === 'modal-shortcut-clicks' &&
          node.text === 'Outside actions: 1'
      ),
    'The outside button still acts on the same click'
  )
  tap(recorder())
  await until(
    () => recorder()?.description === 'Listening…',
    'Recorder listens before a background press'
  )
  await click(
    node => node.text === 'Shortcut modal',
    'noninteractive modal header'
  )
  await until(
    () => recorder()?.description === '⌃K' && recorder()?.selected === 'false',
    'A background press cancels recording'
  )
  keys(2070)
  await until(
    current => !title(current),
    'Idle Escape closes the native dialog'
  )
  console.log(
    'Native shortcut Escape containment, custom labels, fallback clear and modal blur passed'
  )
})

await check('drawer', async () => {
  await launch('drawer')
  const initial = tree()
  const viewport = box(
    findNode(initial, node => node.type === 'RowSplit').attributes
  )
  const rail = box(
    findNode(initial, node => node.id === 'component-list').attributes
  )
  const density = (rail[2] - rail[0]) / 272
  const extent = 352 * density
  const panel = current =>
    findNode(current, node => node.id === 'dazz-drawer-surface')
  const closed = () => until(current => !panel(current), 'drawer closes')
  for (const side of ['left', 'right', 'top', 'bottom']) {
    await click(node => node.text === side, side + ' drawer', true)
    await until(current => panel(current), side + ' drawer opens')
    const current = tree()
    const surface = panel(current)
    const bounds = box(surface.attributes)
    const closeTo = (actual, expected, label) =>
      assert.ok(
        Math.abs(actual - expected) <= 3,
        side + ': ' + label + ' (' + actual + ' / ' + expected + ')'
      )
    if (side === 'left' || side === 'right') {
      closeTo(bounds[2] - bounds[0], extent, 'token width')
      closeTo(bounds[1], viewport[1], 'full-height top')
      closeTo(bounds[3], viewport[3], 'full-height bottom')
      closeTo(
        side === 'left' ? bounds[0] : bounds[2],
        side === 'left' ? viewport[0] : viewport[2],
        'flush edge'
      )
    } else {
      closeTo(bounds[3] - bounds[1], extent, 'token height')
      closeTo(bounds[0], viewport[0], 'full-width left')
      closeTo(bounds[2], viewport[2], 'full-width right')
      closeTo(
        side === 'top' ? bounds[1] : bounds[3],
        side === 'top' ? viewport[1] : viewport[3],
        'flush edge'
      )
    }
    const body = findNode(surface, node => node.type === 'Scroll')
    const bodyContent = findNode(body, node => node.type === 'Column')
    closeTo(
      box(bodyContent.attributes)[1],
      box(body.attributes)[1],
      'body starts at the top of its viewport'
    )
    const nodes = flatten(surface)
    assert.ok(
      nodes.some(node => node.text === 'Filters'),
      'Drawer title'
    )
    assert.ok(
      nodes.some(node => node.text === 'What the list leaves out.'),
      'Drawer description'
    )
    const switches = nodes.filter(node => node.type === 'Toggle')
    assert.equal(switches.length, 2, 'Both native filters are rendered')
    assert.equal(switches[0].checked, 'false')
    assert.equal(switches[1].checked, 'true')
    assert.ok(
      box(switches[0])[0] > (bounds[0] + bounds[2]) / 2,
      'Filter controls align to the far edge'
    )
    tap(switches[0])
    await until(current => {
      const surface = panel(current)
      return (
        surface &&
        flatten(surface).filter(node => node.type === 'Toggle')[0]?.checked ===
          'true'
      )
    }, 'native drawer filter toggles')
    const done = nodes.find(node => node.text === 'Done')
    assert.ok(
      box(done)[3] <= bounds[3] && box(done)[1] > bounds[3] - 80 * density,
      'Footer stays at the end of the drawer'
    )
    await click(node => node.text === 'Done', 'drawer footer')
    await closed()
    console.log(
      'Native ' + side + ' drawer geometry, controls and footer passed'
    )
  }
  await click(node => node.text === 'Open filters', 'default drawer')
  const defaultPanel = await until(
    current => panel(current),
    'default right drawer opens'
  )
  assert.ok(
    Math.abs(box(defaultPanel.attributes)[2] - viewport[2]) <= 3,
    'The omitted edge defaults to the right'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  await closed()
  await click(node => node.text === 'Open filters', 'default drawer again')
  await until(current => panel(current), 'drawer reopens after Escape')
  shell(
    'uitest',
    'uiInput',
    'click',
    String(viewport[0] + 20),
    String(viewport[1] + 100)
  )
  await closed()
  assert.ok(
    section('drawer/0'),
    'The modal mask consumes its click without navigating the page below'
  )
  console.log('Native drawer Escape and backdrop dismissal passed')
})

await check('drawer-swipes', async () => {
  await launch('drawer')
  for (const side of ['left', 'right', 'top', 'bottom']) {
    await click(
      node => node.text === side,
      side + ' drawer swipe trigger',
      true
    )
    const surface = await until(
      current => findNode(current, node => node.id === 'dazz-drawer-surface'),
      side + ' drawer opens'
    )
    const [left, top, right, bottom] = box(surface.attributes)
    const start = [
      Math.round((left + right) / 2),
      Math.round((top + bottom) / 2),
    ]
    const end =
      side === 'left'
        ? [left + 2, start[1]]
        : side === 'right'
          ? [right - 2, start[1]]
          : side === 'top'
            ? [start[0], top + 2]
            : [start[0], bottom - 2]
    const inward =
      side === 'left'
        ? [right - 2, start[1]]
        : side === 'right'
          ? [left + 2, start[1]]
          : side === 'top'
            ? [start[0], bottom - 2]
            : [start[0], top + 2]
    shell(
      'uitest',
      'uiInput',
      'swipe',
      ...start.map(String),
      ...inward.map(String)
    )
    assert.ok(
      findNode(tree(), node => node.id === 'dazz-drawer-surface'),
      'An inward gesture preserves ' + side
    )
    shell(
      'uitest',
      'uiInput',
      'swipe',
      ...start.map(String),
      ...end.map(String)
    )
    await until(
      current => !findNode(current, node => node.id === 'dazz-drawer-surface'),
      side + ' native swipe dismisses'
    )
  }
  console.log('Native drawer dismissal gestures passed on all four edges')
})

await check('drawer-keyboard', async () => {
  await launch('drawer')
  await click(node => node.text === 'right', 'keyboard drawer trigger', true)
  const panel = current =>
    findNode(current, node => node.id === 'dazz-drawer-surface')
  await until(current => panel(current), 'keyboard drawer opens')
  let focused = false
  for (let attempt = 0; attempt < 8; attempt++) {
    const surface = panel(tree())
    assert.ok(surface, 'Tab stays in the native modal')
    focused = Boolean(
      findNode(
        surface,
        node => node.type === 'Button' && node.focused === 'true'
      )
    )
    if (focused) break
    shell('uitest', 'uiInput', 'keyEvent', '2049')
  }
  assert.ok(focused, 'Tab reaches the close button')
  shell('uitest', 'uiInput', 'keyEvent', '2054')
  await until(current => !panel(current), 'Enter closes the native drawer')
  shell('uitest', 'uiInput', 'keyEvent', '2054')
  await until(current => panel(current), 'focus returns to the drawer trigger')
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  await until(current => !panel(current), 'Escape closes the reopened drawer')
  console.log(
    'Native drawer keyboard close and trigger focus restoration passed'
  )
})

await check('drawer-overflow', async () => {
  await launch('', ['--pb', 'modalTest', 'true'])
  const panel = current =>
    findNode(current, node => node.id === 'dazz-drawer-surface')
  const rowVisible = (surface, index) => {
    const row = findNode(surface, node => node.id === 'modal-row-' + index)
    return row && visible(row.attributes)
  }
  const atEdge = (surface, start) => {
    const body = findNode(surface, node => node.type === 'Scroll')
    const content = body.children[0].attributes
    assert.ok(
      content.origBounds,
      'Native scroll content exposes its unclipped bounds'
    )
    const bounds = content.origBounds.match(/-?\d+/g).map(Number)
    const viewport = box(body.attributes)
    return start ? bounds[1] >= viewport[1] - 2 : bounds[3] <= viewport[3] + 2
  }
  const scroll = (surface, down = false) => {
    const body = findNode(surface, node => node.type === 'Scroll')
    const [left, top, right, bottom] = box(body.attributes)
    const x = Math.round(left + (right - left) * 0.35)
    const high = Math.round(top + (bottom - top) * 0.2)
    const low = Math.round(top + (bottom - top) * 0.8)
    shell(
      'uitest',
      'uiInput',
      'swipe',
      String(x),
      String(down ? high : low),
      String(x),
      String(down ? low : high)
    )
  }
  for (const side of ['top', 'bottom']) {
    await click(
      node => node.text === 'Scrollable ' + side,
      'long ' + side + ' drawer'
    )
    let surface = await until(current => panel(current), 'long drawer opens')
    const footer = box(
      findNode(surface, node => node.text === 'Close').attributes
    )
    assert.ok(rowVisible(surface, 0), 'Long body starts at its first row')
    scroll(surface)
    await until(current => {
      const surface = panel(current)
      return surface && !rowVisible(surface, 0)
    }, 'body scrolls without dismissing the drawer')
    surface = panel(tree())
    assert.deepEqual(
      box(findNode(surface, node => node.text === 'Close').attributes),
      footer,
      'Footer stays fixed while the body scrolls'
    )
    const target = side === 'top' ? 39 : 0
    for (let attempt = 0; attempt < 20; attempt++) {
      surface = panel(tree())
      assert.ok(surface, 'Scroll gestures preserve an overflowing drawer')
      if (rowVisible(surface, target) && atEdge(surface, side === 'bottom'))
        break
      scroll(surface, side === 'bottom')
    }
    surface = panel(tree())
    assert.ok(
      rowVisible(surface, target),
      'Native scrolling reaches row ' + target
    )
    assert.ok(
      atEdge(surface, side === 'bottom'),
      'Scrolling reaches the body edge, including padding'
    )
    scroll(surface, side === 'bottom')
    await until(
      current => !panel(current),
      'An outward gesture at the body edge dismisses ' + side
    )
  }
  const viewport = box(
    findNode(tree(), node => node.id === 'document-column').attributes
  )
  await click(node => node.text === 'Without footer', 'drawer without a footer')
  const surface = await until(
    current => panel(current),
    'optional-footer drawer opens'
  )
  const bounds = box(surface.attributes)
  assert.ok(
    Math.abs((bounds[2] - bounds[0]) * 2 - (viewport[2] - viewport[0])) <= 3,
    'Percentage sizing uses the application viewport'
  )
  assert.ok(
    !flatten(surface).some(node => node.text === 'Close'),
    'No default footer is inserted'
  )
  const body = findNode(surface, node => node.type === 'Scroll')
  assert.ok(
    Math.abs(box(body.attributes)[3] - bounds[3]) <= 3,
    'The body occupies the space of the omitted footer'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  await until(current => !panel(current), 'optional-footer drawer closes')
  console.log(
    'Native drawer overflow, scroll handoff, fixed footer and percentage sizing passed'
  )
})

await check('form', async () => {
  await launch('form')
  await click(node => node.text === 'Create account', 'form submit')
  await until(
    current =>
      flatten(current).filter(
        node => node.text === 'Please fill out this field.'
      ).length === 2,
    'both required fields validate'
  )
  function formEditors() {
    const scope = findNode(tree(), node => node.id === 'example-form/0')
    return flatten(scope).filter(node => node.type === 'TextInput')
  }
  for (const [index, value] of ['Ada', 'ada@example.com'].entries()) {
    const [left, top, right, bottom] = box(formEditors()[index])
    shell(
      'uitest',
      'uiInput',
      'inputText',
      String(Math.round((left + right) / 2)),
      String(Math.round((top + bottom) / 2)),
      value
    )
    // The desktop IME may leave injected Latin text in pre-edit state.
    shell('uitest', 'uiInput', 'keyEvent', '2054')
    await wait(250)
  }
  await click(node => node.text === 'Create account', 'valid form submit')
  await until(
    current =>
      !flatten(current).some(
        node => node.text === 'Please fill out this field.'
      ),
    'valid form has no errors'
  )
  await click(node => node.text === 'Clear', 'form reset')
  await until(
    () => formEditors().every(node => !node.text),
    'form reset clears both editors'
  )
  console.log('Native form validation and reset passed')
})

function tap(node) {
  const [left, top, right, bottom] = box(node)
  shell(
    'uitest',
    'uiInput',
    'click',
    String(Math.round((left + right) / 2)),
    String(Math.round((top + bottom) / 2))
  )
}
function section(id) {
  return findNode(tree(), node => node.id === 'example-' + id)
}
async function editorFor(id) {
  for (let attempt = 0; attempt < 12; attempt++) {
    const scope = section(id)
    const editor =
      scope &&
      flatten(scope).find(node => node.type === 'TextInput' && visible(node))
    if (editor) return editor
    const document = flatten(tree()).find(node => node.id === 'document-column')
    const [left, top, right, bottom] = box(document)
    shell(
      'uitest',
      'uiInput',
      'swipe',
      String(Math.round((left + right) / 2)),
      String(Math.round(top + (bottom - top) * 0.8)),
      String(Math.round((left + right) / 2)),
      String(Math.round(top + (bottom - top) * 0.3))
    )
  }
  throw new Error('Missing editor in ' + id)
}
await check('menubar-keyboard', async () => {
  await launch('', [
    '--pb',
    'paintTest',
    'true',
    '--ps',
    'paintPopup',
    'menubar',
  ])
  const primary = () =>
    flatten(findNode(tree(), node => node.id === 'bar-primary')).filter(
      node => node.type === 'Button' && /-trigger-\d+$/.test(node.id)
    )
  const buttons = primary()
  assert.equal(buttons.length, 4)
  assert.equal(buttons[1].enabled, 'false')
  const focused = index =>
    primary().some(
      node => node.id === buttons[index].id && node.focused === 'true'
    )
  const menuRows = root => {
    const panel = findNode(root, node =>
      node.id?.startsWith('dazzui-menu-surface-')
    )
    return panel ? flatten(panel).filter(node => node.type === 'MenuItem') : []
  }
  const hasMenu = root =>
    flatten(root).some(
      node => node.type === 'Menu' && /^dazzui-menu-surface-\d+$/.test(node.id)
    )
  await click(node => node.id === 'bar-before', 'focus before the composite')
  shell('uitest', 'uiInput', 'keyEvent', '2049')
  await until(
    () => focused(0),
    'Tab enters the bar at its first enabled trigger'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2015')
  await until(() => focused(2), 'Right skips the disabled trigger')
  assert.ok(!hasMenu(tree()), 'Closed-bar arrows only move focus')
  shell('uitest', 'uiInput', 'keyEvent', '2082')
  await until(() => focused(3), 'End reaches Help')
  shell('uitest', 'uiInput', 'keyEvent', '2015')
  await until(() => focused(0), 'Right wraps to File')
  shell('uitest', 'uiInput', 'keyEvent', '2049')
  await until(
    root =>
      flatten(root).some(
        node => node.id === 'bar-after' && node.focused === 'true'
      ),
    'Tab exits the whole composite instead of visiting every label'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2047', '2049')
  await until(() => focused(0), 'Reverse Tab restores the remembered trigger')
  shell('uitest', 'uiInput', 'keyEvent', '2013')
  await until(
    root =>
      menuRows(root).some(
        node => node.id.endsWith('-item-0') && node.focused === 'true'
      ),
    'Down opens File and focuses its first action'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2015')
  await until(
    root =>
      flatten(root).some(node => node.text === 'Sidebar') &&
      menuRows(root).filter(node => node.focused === 'true').length === 1,
    'Right from the popup opens View and focuses its first item'
  )
  assert.equal(
    flatten(tree()).filter(
      node => node.type === 'Menu' && /^dazzui-menu-surface-\d+$/.test(node.id)
    ).length,
    1,
    'Only the selected top-level menu stays open'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  await until(
    root => !hasMenu(root) && focused(2),
    'Escape returns to the active View trigger'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2014')
  await until(
    () => focused(0),
    'Left returns to File across the disabled label'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2012')
  await until(
    root =>
      menuRows(root).some(
        node => node.id.endsWith('-item-1') && node.focused === 'true'
      ),
    'Up opens File at its last enabled action'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  await until(root => !hasMenu(root) && focused(0), 'Escape returns to File')
  shell('uitest', 'uiInput', 'keyEvent', '2050')
  await until(
    root =>
      menuRows(root).some(
        node => node.id.endsWith('-item-0') && node.focused === 'true'
      ),
    'Space opens at the first action without double activation'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2049')
  await until(
    root =>
      !hasMenu(root) &&
      flatten(root).some(
        node => node.id === 'bar-after' && node.focused === 'true'
      ),
    'Tab from a popup closes it and continues after the bar'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2047', '2049')
  await until(() => focused(0), 'Reverse Tab returns to File')
  shell('uitest', 'uiInput', 'keyEvent', '2013')
  await until(
    root => menuRows(root).some(node => node.focused === 'true'),
    'File opens before reverse popup traversal'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2047', '2049')
  await until(
    root =>
      !hasMenu(root) &&
      flatten(root).some(
        node => node.id === 'bar-before' && node.focused === 'true'
      ),
    'Reverse Tab from a popup closes it and continues before the bar'
  )
  console.log(
    'Native menubar roving focus, disabled skipping, sibling switching, opening endpoints, Escape and Tab passed'
  )
})

await check('menubar-pointer', async () => {
  await launch('', [
    '--pb',
    'paintTest',
    'true',
    '--ps',
    'paintPopup',
    'menubar',
  ])
  const buttons = flatten(
    findNode(tree(), node => node.id === 'bar-primary')
  ).filter(node => node.type === 'Button' && /-trigger-\d+$/.test(node.id))
  const move = (node, press = false) => {
    const b = box(node)
    shell(
      'uinput',
      '-M',
      '-m',
      String(Math.round((b[0] + b[2]) / 2)),
      String(Math.round((b[1] + b[3]) / 2)),
      ...(press ? ['-c', '0'] : [])
    )
  }
  const openPanels = root =>
    flatten(root).filter(
      node => node.type === 'Menu' && /^dazzui-menu-surface-\d+$/.test(node.id)
    )
  move(buttons[2])
  await wait(200)
  assert.equal(openPanels(tree()).length, 0, 'Hover does not open an idle bar')
  await click(node => node.id === buttons[0].id, 'open File from the pointer')
  await until(
    root => flatten(root).some(node => node.text === 'Rename'),
    'File actions'
  )
  move(buttons[3])
  await until(
    root =>
      flatten(root).some(node => node.text === 'Documentation') &&
      openPanels(root).length === 1,
    'Hover opens Help without another click'
  )
  move(buttons[1])
  await wait(200)
  assert.ok(
    flatten(tree()).some(node => node.text === 'Documentation'),
    'Disabled label does not replace the open menu'
  )
  move(buttons[2])
  await until(
    root =>
      flatten(root).some(node => node.text === 'Sidebar') &&
      openPanels(root).length === 1,
    'Hover opens View'
  )
  const openFace = (await inspectNative(buttons[2].id + '-face')).$attrs
  assert.notEqual(
    openFace.backgroundColor,
    '#00000000',
    'The open trigger retains its wash'
  )
  await click(
    node => node.id === 'bar-after',
    'outside press on a live underlying control'
  )
  await until(
    root => openPanels(root).length === 0,
    'Outside press dismisses the modal menu'
  )
  assert.ok(
    flatten(tree()).some(
      node => node.id === 'bar-count' && node.text === 'Actions: 0; outside: 0'
    ),
    'The modal outside press is consumed before the underlying action'
  )
  await click(
    node => node.id === 'bar-after',
    'underlying action after dismissal'
  )
  await until(
    root =>
      flatten(root).some(
        node =>
          node.id === 'bar-count' && node.text === 'Actions: 0; outside: 1'
      ),
    'Underlying controls become live again after dismissal'
  )
  await click(
    node => node.id === buttons[0].id,
    'reopen File for touch handoff'
  )
  await until(
    root => flatten(root).some(node => node.text === 'Rename'),
    'File reopens'
  )
  await click(
    node => node.id === buttons[3].id,
    'touch Help while File is open'
  )
  await until(
    root =>
      flatten(root).some(node => node.text === 'Documentation') &&
      openPanels(root).length === 1,
    'A touch changes the active top-level menu'
  )
  await click(
    node => node.id === buttons[1].id,
    'touch the disabled top-level label'
  )
  await wait(200)
  assert.ok(
    flatten(tree()).some(node => node.text === 'Documentation'),
    'A disabled bar label keeps the current menu open'
  )
  await click(
    node => node.id === buttons[3].id,
    'touch the open Help label again'
  )
  await until(
    root => openPanels(root).length === 0,
    'The open label toggles its own menu closed'
  )
  await click(node => node.id === 'bar-toggle-modal', 'choose the nonmodal bar')
  await until(
    root =>
      flatten(root).some(
        node => node.id === 'bar-modal' && node.text === 'Nonmodal'
      ),
    'Nonmodal fixture state'
  )
  await click(node => node.id === buttons[3].id, 'open a nonmodal Help menu')
  await until(
    root => flatten(root).some(node => node.text === 'Documentation'),
    'Nonmodal Help actions'
  )
  await click(node => node.id === 'bar-after', 'nonmodal outside action')
  await until(
    root =>
      openPanels(root).length === 0 &&
      flatten(root).some(
        node =>
          node.id === 'bar-count' && node.text === 'Actions: 0; outside: 2'
      ),
    'Nonmodal outside press both dismisses and activates its underlying control'
  )
  const secondary = flatten(
    findNode(tree(), node => node.id === 'bar-secondary')
  ).filter(node => node.type === 'Button' && /-trigger-\d+$/.test(node.id))
  await click(
    node => node.id === secondary[0].id,
    'open the independent secondary bar'
  )
  await until(
    root => flatten(root).some(node => node.text === 'Arrange'),
    'Secondary Window actions'
  )
  move(buttons[2])
  await wait(200)
  assert.ok(
    flatten(tree()).some(node => node.text === 'Arrange'),
    'Hovering an idle sibling bar does not steal the active scope'
  )
  move(secondary[1])
  await until(
    root =>
      flatten(root).some(node => node.text === 'Inspect') &&
      openPanels(root).length === 1,
    'Only the secondary bar performs its own hover handoff'
  )
  await click(
    node => node.text === 'Inspect',
    'activate a secondary menu action'
  )
  await until(
    root =>
      openPanels(root).length === 0 &&
      flatten(root).some(
        node =>
          node.id === 'bar-count' && node.text === 'Actions: 1; outside: 2'
      ),
    'The secondary menu retains its own action and closes'
  )
  await click(
    node => node.id === 'bar-toggle-modal',
    'restore the default modal mouse path'
  )
  await until(
    root =>
      flatten(root).some(
        node => node.id === 'bar-modal' && node.text === 'Modal'
      ),
    'Default modal fixture state'
  )
  move(buttons[3], true)
  await until(
    root => flatten(root).some(node => node.text === 'Documentation'),
    'A physical mouse click opens Help'
  )
  move(buttons[1], true)
  await until(
    root => flatten(root).some(node => node.text === 'Documentation'),
    'A mouse press on the disabled label preserves Help'
  )
  move(buttons[0])
  await until(
    root => flatten(root).some(node => node.text === 'Rename'),
    'Mouse movement hands off to File'
  )
  move(buttons[0], true)
  await until(
    root => openPanels(root).length === 0,
    'A physical click toggles the open File label closed'
  )
  console.log(
    'Native menubar hover/touch/mouse handoff, disabled labels, toggle, open wash, modal/nonmodal dismissal and independent scopes passed'
  )
})

await check('menubar-layout', async () => {
  const themes = JSON.parse(
    fs.readFileSync(
      'packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json',
      'utf8'
    )
  )
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/menubar-layout'
  )
  fs.mkdirSync(directory, { recursive: true })
  const length = value => value.value * (value.unit === 'rem' ? 16 : 1)
  const color = value =>
    '#' +
    value
      .map(byte => byte.toString(16).padStart(2, '0'))
      .join('')
      .toUpperCase()
  for (const theme of ['studio-light', 'studio-dark', 'bright-light']) {
    await launch('menu', [
      '--ps',
      'sizing',
      'design',
      '--ps',
      'themeName',
      theme,
    ])
    await reveal(node => node.id === 'menu/2/0', 'the source menubar')
    const buttons = flatten(
      findNode(tree(), node => node.id === 'menu/2/0')
    ).filter(node => node.type === 'Button' && /-trigger-\d+$/.test(node.id))
    assert.deepEqual(
      buttons.map(node => node.text),
      ['File', 'View', 'Help']
    )
    const first = await inspectNative(buttons[0].id)
    const density = first.$geometry.density
    const tokens = themes[theme]
    const near = (actual, expected, label) =>
      assert.ok(
        Math.abs(actual / density - expected) <= 1,
        `${theme} ${label}: ${actual / density}, expected ${expected}`
      )
    const details = []
    for (const [index, button] of buttons.entries()) {
      const native = index ? await inspectNative(button.id) : first
      const face = await inspectNative(button.id + '-face')
      near(
        native.$geometry.frame.height,
        length(tokens['control.small.size']),
        'trigger height'
      )
      assert.equal(
        Number.parseFloat(face.$attrs.fontSize),
        length(tokens['control.small.content'].fontSize)
      )
      assert.equal(
        Number.parseFloat(face.$attrs.fontWeight),
        tokens['control.small.content'].fontWeight
      )
      assert.equal(face.$attrs.fontColor, color(tokens['color.content']))
      assert.equal(
        native.$attrs.backgroundColor,
        '#00000000',
        'The native trigger remains flat'
      )
      assert.ok(
        native.$geometry.frame.width / density >
          2 * length(tokens['spacing.2']),
        'The intrinsic trigger includes its label and both inline insets'
      )
      if (index)
        near(
          native.$geometry.frame.x -
            details[index - 1].native.$geometry.frame.x -
            details[index - 1].native.$geometry.frame.width,
          length(tokens['spacing.0.5']),
          'bar gap'
        )
      details.push({ native, face })
    }
    await click(node => node.id === buttons[0].id, 'open the source File menu')
    await until(
      root => flatten(root).some(node => node.text === 'Rename'),
      'File menu drawing'
    )
    const openFace = await inspectNative(buttons[0].id + '-face')
    assert.equal(
      openFace.$attrs.backgroundColor,
      color(tokens['control.color.normal.surface.hovered.color']),
      'The open label uses the shared neutral wash'
    )
    fs.writeFileSync(
      path.join(directory, theme + '.json'),
      JSON.stringify({ theme, details, openFace }, null, 2)
    )
    const capture = '/data/local/tmp/dazzui-menubar-' + theme + '.png'
    shell('uitest', 'screenCap', '-p', capture)
    command('file', 'recv', capture, path.join(directory, theme + '.png'))
  }
  console.log(
    'Source menubar control heights, type, gaps, neutral open wash and native captures passed in three themes'
  )
})

await check('menu-layout', async () => {
  const themes = JSON.parse(
    fs.readFileSync(
      'packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json',
      'utf8'
    )
  )
  const directory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/menu-layout'
  )
  fs.mkdirSync(directory, { recursive: true })
  const length = value => value.value * (value.unit === 'rem' ? 16 : 1)
  const reports = []
  for (const theme of ['studio-light', 'studio-dark', 'bright-light']) {
    const tokens = themes[theme]
    await launch('menu', [
      '--ps',
      'sizing',
      'design',
      '--ps',
      'themeName',
      theme,
    ])
    await click(node => node.id === 'menu/0/0', 'open the action menu', true)
    const panel = await until(
      root =>
        flatten(root).find(node => node.id?.startsWith('dazzui-menu-surface-')),
      'native action menu surface'
    )
    const native = await inspectNative(panel.id)
    const density = native.$geometry.density
    const near = (actual, expected, label) =>
      assert.ok(
        Math.abs(actual / density - expected) <= 1,
        `${theme} ${label}: ${actual / density}, expected ${expected}`
      )
    const rows = flatten(findNode(tree(), node => node.id === panel.id)).filter(
      node => node.type === 'MenuItem'
    )
    assert.equal(
      rows.length,
      3,
      'The three source actions remain native MenuItems'
    )
    const details = []
    for (const [index, row] of rows.entries()) {
      const drawing = await inspectNative(row.id)
      near(
        drawing.$geometry.frame.height,
        Math.max(
          length(tokens.labelQuiet.fontSize),
          length(tokens['icon.medium'])
        ) +
          2 * length(tokens['menu.item-padding']),
        'icon action row height'
      )
      const label = await inspectNative(row.id + '-label')
      assert.equal(
        Number.parseFloat(label.$attrs.fontSize),
        length(tokens.labelQuiet.fontSize)
      )
      assert.equal(
        Number.parseFloat(label.$attrs.fontWeight),
        tokens.labelQuiet.fontWeight
      )
      assert.equal(row.enabled, index === 2 ? 'false' : 'true')
      details.push({ row, drawing, label })
    }
    fs.writeFileSync(
      path.join(directory, theme + '-measured.json'),
      JSON.stringify({ native, details }, null, 2)
    )
    near(
      native.$geometry.frame.width,
      length(tokens['menu.min-width']),
      'intrinsic action panel width'
    )
    const anchor = box(
      findNode(tree(), node => node.id === 'menu/0/0').attributes
    )
    near(
      native.$geometry.frame.y - anchor[3],
      length(tokens['spacing.1']),
      'menu target gap'
    )
    near(
      native.$geometry.frame.height,
      3 *
        (Math.max(
          length(tokens.labelQuiet.fontSize),
          length(tokens['icon.medium'])
        ) +
          2 * length(tokens['menu.item-padding'])) +
        2 * (length(tokens['spacing.1']) + length(tokens['stroke.hairline'])),
      'panel has only the shared border, padding and rows'
    )
    const shortcut = await inspectNative(rows[1].id + '-shortcut')
    assert.equal(
      Number.parseFloat(shortcut.$attrs.fontSize),
      length(tokens.labelSmall.fontSize)
    )
    assert.equal(
      Number.parseFloat(shortcut.$attrs.fontWeight),
      tokens.labelSmall.fontWeight
    )
    const hovered = box(rows[1])
    shell(
      'uinput',
      '-M',
      '-m',
      String(Math.round((hovered[0] + hovered[2]) / 2)),
      String(Math.round((hovered[1] + hovered[3]) / 2))
    )
    await wait(200)
    const paint = (await inspectNative(rows[1].id)).$attrs
    const color = value =>
      '#' +
      value
        .map(byte => byte.toString(16).padStart(2, '0'))
        .join('')
        .toUpperCase()
    assert.equal(
      paint.backgroundColor,
      color(tokens['control.color.normal.surface.hovered.color']),
      'Pointer highlight uses the shared neutral normal recipe'
    )
    assert.equal(Number(paint.shadow.radius), 0, 'Highlighted rows remain flat')
    shell('uitest', 'uiInput', 'keyEvent', '2081')
    assert.equal(
      (await inspectNative(rows[0].id)).$attrs.backgroundColor,
      color(tokens['control.color.normal.surface.hovered.color']),
      'Keyboard Home moves the shared highlight'
    )
    assert.equal(
      (await inspectNative(rows[1].id)).$attrs.backgroundColor,
      '#00000000',
      'A stationary pointer does not leave a second highlighted row'
    )
    const report = { theme, native, details, shortcut }
    reports.push(report)
    fs.writeFileSync(
      path.join(directory, theme + '.json'),
      JSON.stringify(report, null, 2)
    )
    const capture = '/data/local/tmp/dazzui-menu-layout.png'
    shell('uitest', 'screenCap', '-p', capture)
    command('file', 'recv', capture, path.join(directory, theme + '.png'))
    tap(rows[2])
    await wait(150)
    assert.ok(
      flatten(tree()).some(node => node.id === panel.id),
      'Disabled action cannot dismiss the menu'
    )
    await click(node => node.id === rows[1].id, 'activate Duplicate')
    await until(
      root => !flatten(root).some(node => node.id === panel.id),
      'native action dismisses menu'
    )
  }
  const viewButton = findLabeledButton(tree(), 'View')
  assert.ok(viewButton)
  tap(viewButton.attributes)
  const radioPanel = await until(
    root =>
      flatten(root).find(node => node.id?.startsWith('dazzui-menu-surface-')),
    'View radio menu'
  )
  const radioRows = flatten(
    findNode(tree(), node => node.id === radioPanel.id)
  ).filter(node => node.type === 'MenuItem')
  assert.deepEqual(
    radioRows.map(row => row.selected),
    ['true', 'false', 'false'],
    'React radio groups resolve multiple supplied checks to the first checked item'
  )
  for (const [index, row] of radioRows.entries()) {
    const label = await inspectNative(row.id + '-label')
    const frame = (await inspectNative(row.id)).$geometry
    const tokens = themes['bright-light']
    const expectedInset =
      length(tokens['spacing.2']) +
      (index < 2
        ? length(tokens['icon.small']) + length(tokens['spacing.2'])
        : 0)
    assert.ok(
      Math.abs(
        (label.$geometry.frame.x - frame.frame.x) / frame.density -
          expectedInset
      ) <= 1,
      'Only checkable rows reserve a leading check column'
    )
    if (index < 2)
      assert.equal(
        (await inspectNative(row.id + '-check')).$attrs.visibility,
        index === 0 ? 'Visibility.Visible' : 'Visibility.Hidden'
      )
  }
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  await launch('select', ['--ps', 'sizing', 'design'])
  await click(node => node.id === 'select/0/0', 'open a language select', true)
  const panel = await until(
    root => flatten(root).find(node => /^dazz-select-\d+-menu$/.test(node.id)),
    'native select panel'
  )
  const native = await inspectNative(panel.id)
  const trigger = box(
    findNode(tree(), node => node.id === 'select/0/0').attributes
  )
  const source = themes['studio-light']
  const hex = value =>
    '#' +
    value
      .map(byte => byte.toString(16).padStart(2, '0'))
      .join('')
      .toUpperCase()
  const triggerStyle = (
    await inspectNative(panel.id.replace(/-menu$/, '-surface'))
  ).$attrs
  assert.equal(
    triggerStyle.backgroundColor,
    hex(source['color.surface']),
    'Open Select retains its active field surface'
  )
  assert.equal(
    triggerStyle.borderColor,
    hex(source['color.primary.600']),
    'Open Select retains its active border'
  )
  assert.equal(
    Number.parseFloat(triggerStyle.outlineWidth),
    length(source['focus.width'])
  )
  assert.ok(
    Math.abs(native.$geometry.frame.width - (trigger[2] - trigger[0])) <= 2,
    'Select popup is at least the width of its 20rem anchor'
  )
  assert.ok(
    Math.abs(
      (native.$geometry.frame.y - trigger[3]) / native.$geometry.density -
        length(themes['studio-light']['spacing.1'])
    ) <= 1,
    'Select target gap follows the shared spacing role'
  )
  const rows = flatten(findNode(tree(), node => node.id === panel.id)).filter(
    node => node.type === 'MenuItem'
  )
  assert.equal(rows.length, 4)
  for (const row of rows) {
    const drawing = await inspectNative(row.id)
    assert.ok(
      Math.abs(
        drawing.$geometry.frame.height / native.$geometry.density -
          (length(themes['studio-light'].labelQuiet.fontSize) +
            2 * length(themes['studio-light']['menu.item-padding']))
      ) <= 1,
      'Plain select options use the shared compact row height'
    )
  }
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  for (const [index, disabled] of [
    [0, false],
    [1, true],
  ]) {
    const id = 'select/1/' + index
    await reveal(
      node => node.id === id,
      disabled ? 'disabled field' : 'placeholder field'
    )
    const field = flatten(findNode(tree(), node => node.id === id)).find(node =>
      /^dazz-select-\d+-trigger$/.test(node.id)
    )
    assert.ok(field)
    const actual = await inspectNative(field.id)
    const surface = await inspectNative(
      field.id.replace(/-trigger$/, '-surface')
    )
    const label = await inspectNative(field.id.replace(/-trigger$/, '-value'))
    assert.equal(
      label.$attrs.fontColor,
      hex(
        source[
          disabled
            ? 'control.color.normal.content.disabled.color'
            : 'color.content-faint'
        ]
      )
    )
    if (disabled) {
      assert.equal(
        surface.$attrs.backgroundColor,
        hex(source['control.color.normal.surface.disabled.color'])
      )
      assert.equal(actual.$attrs.enabled, false)
      tap(field)
      assert.ok(
        !flatten(tree()).some(node => node.type === 'Menu'),
        'Disabled Select remains inert through its painted face'
      )
      assert.equal(
        surface.$attrs.opacity,
        1,
        'The field drawing is outside the native disabled-opacity boundary'
      )
      const capture = path.join(directory, 'select-disabled.png')
      shell(
        'uitest',
        'screenCap',
        '-p',
        '/data/local/tmp/dazzui-select-disabled.png'
      )
      command(
        'file',
        'recv',
        '/data/local/tmp/dazzui-select-disabled.png',
        capture
      )
      const image = readPNG(fs.readFileSync(capture))
      const b = box(field)
      const actualFill = image
        .pixel(Math.round((b[0] + b[2]) / 2), Math.round((b[1] + b[3]) / 2))
        .slice(0, 3)
      const expectedFill =
        source['control.color.normal.surface.disabled.color'].slice(1)
      assert.ok(
        actualFill.every(
          (value, index) => Math.abs(value - expectedFill[index]) <= 1
        ),
        'Rendered disabled fill is the shared recipe, without a second native fade: ' +
          actualFill
      )
      const frame = label.$geometry.frame
      const expectedInk =
        source['control.color.normal.content.disabled.color'].slice(1)
      let closestInk = Infinity
      let actualInk
      for (
        let y = Math.floor(frame.y);
        y < Math.ceil(frame.y + frame.height);
        y++
      ) {
        for (
          let x = Math.floor(frame.x);
          x < Math.ceil(frame.x + frame.width);
          x++
        ) {
          const pixel = image.pixel(x, y).slice(0, 3)
          const error = Math.max(
            ...pixel.map((value, index) => Math.abs(value - expectedInk[index]))
          )
          if (error < closestInk) {
            closestInk = error
            actualInk = pixel
          }
        }
      }
      assert.ok(
        closestInk <= 3,
        'Rendered disabled text reaches the shared ink, without a second native fade: ' +
          actualInk
      )
      fs.writeFileSync(
        path.join(directory, 'select-disabled.json'),
        JSON.stringify(
          {
            actualFill,
            expectedFill,
            actualInk,
            expectedInk,
            closestInk,
            surface,
            label,
          },
          null,
          2
        )
      )
    }
  }
  console.log(
    `Menu/Select shared row metrics, typography, intrinsic/anchor width and disabled activation passed in ${reports.length} palettes`
  )
})

await check('select', async () => {
  await launch('select')
  await click(node => node.id === 'select/2/0', 'grouped select', true)
  await until(
    current =>
      ['People', 'Agents', 'Grace', 'Codex'].every(label =>
        flatten(current).some(node => node.text === label)
      ),
    'native option groups and selected label'
  )
  const panel = flatten(tree()).find(node =>
    /^dazz-select-\d+-menu$/.test(node.id)
  )
  assert.ok(panel)
  const tokens = JSON.parse(
    fs.readFileSync(
      'packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json',
      'utf8'
    )
  )['studio-light']
  const length = value => value.value * (value.unit === 'rem' ? 16 : 1)
  const headerHeight =
    length(tokens.captionSmall.fontSize) +
    length(tokens['spacing.1.5']) +
    length(tokens['spacing.1'])
  const menu = await inspectNative(panel.id)
  const headings = flatten(
    findNode(tree(), node => node.id === panel.id)
  ).filter(node => /^dazz-select-\d+-group-\d+$/.test(node.id))
  assert.equal(headings.length, 2)
  for (const heading of headings) {
    const native = await inspectNative(heading.id)
    assert.ok(
      Math.abs(
        native.$geometry.frame.height / menu.$geometry.density - headerHeight
      ) <= 1,
      'Group captions use the shared heading height instead of a native option minimum'
    )
    assert.equal(
      native.$attrs.focusable,
      false,
      'Group captions never take option focus'
    )
  }
  const rowHeight =
    length(tokens.labelQuiet.fontSize) + 2 * length(tokens['menu.item-padding'])
  const expectedHeight =
    5 * rowHeight +
    2 * headerHeight +
    length(tokens['spacing.1']) +
    2 * (length(tokens['spacing.1']) + length(tokens['stroke.hairline']))
  assert.ok(
    Math.abs(
      menu.$geometry.frame.height / menu.$geometry.density - expectedHeight
    ) <= 1,
    'Grouped popup counts only shared headings, rows, group gap and panel insets'
  )
  const captureDirectory = path.resolve(
    option('--paint-output') ??
      'packages/arkts/dazzui/example/entry/build/verification/menu-layout'
  )
  fs.mkdirSync(captureDirectory, { recursive: true })
  fs.writeFileSync(
    path.join(captureDirectory, 'select-groups.json'),
    JSON.stringify({ menu, headings, expectedHeight, headerHeight }, null, 2)
  )
  shell('uitest', 'screenCap', '-p', '/data/local/tmp/dazzui-select-groups.png')
  command(
    'file',
    'recv',
    '/data/local/tmp/dazzui-select-groups.png',
    path.join(captureDirectory, 'select-groups.png')
  )
  tap(headings[0])
  await wait(150)
  assert.ok(
    flatten(tree()).some(node => node.id === panel.id),
    'Pressing a heading keeps the option list open'
  )
  await click(node => node.text === 'Codex', 'grouped option')
  await until(
    () => flatten(section('select/2')).some(node => node.text === 'Codex'),
    'grouped selection updates field'
  )
  await click(node => node.id === 'select/2/0', 'reopen grouped select')
  await until(
    current => findNode(current, node => node.type === 'Menu'),
    'grouped menu for Home'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2081')
  shell('uitest', 'uiInput', 'keyEvent', '2054')
  await until(
    () => flatten(section('select/2')).some(node => node.text === 'Unassigned'),
    'Home reaches the first ungrouped option'
  )
  await click(node => node.id === 'select/2/0', 'reopen grouped select for End')
  await until(
    current => findNode(current, node => node.type === 'Menu'),
    'grouped menu for End'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2082')
  shell('uitest', 'uiInput', 'keyEvent', '2054')
  await until(
    () => flatten(section('select/2')).some(node => node.text === 'Codex'),
    'End reaches the last grouped option'
  )
  await click(
    node => node.id === 'select/2/0',
    'reopen grouped select for arrows'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2081')
  shell('uitest', 'uiInput', 'keyEvent', '2013')
  shell('uitest', 'uiInput', 'keyEvent', '2054')
  await until(
    () => flatten(section('select/2')).some(node => node.text === 'Ada'),
    'Down skips the People heading'
  )
  await click(
    node => node.id === 'select/2/0',
    'reopen grouped select for Space'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2082')
  shell('uitest', 'uiInput', 'keyEvent', '2012')
  shell('uitest', 'uiInput', 'keyEvent', '2050')
  await until(
    () => flatten(section('select/2')).some(node => node.text === 'Claude'),
    'Up and Space choose the preceding option'
  )
  console.log(
    'Native grouped select heading geometry, pointer, Home/End, arrow skipping and Space passed'
  )
})

await check('select-typeahead', async () => {
  await launch('select')
  await click(node => node.id === 'select/0/0', 'open language options', true)
  await until(
    root => findNode(root, node => node.type === 'Menu'),
    'language menu for type-ahead'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2023')
  assert.ok(
    !flatten(section('select/0')).some(node => node.text === 'German'),
    'Open-list type-ahead changes focus before committing a value'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2054')
  await until(
    () => flatten(section('select/0')).some(node => node.text === 'German'),
    'Typing G reaches German without changing the value before Enter'
  )
  await until(
    () =>
      flatten(section('select/0')).some(
        node =>
          /^dazz-select-\d+-trigger$/.test(node.id) && node.focused === 'true'
      ),
    'Selection returns native focus to its own trigger'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2021')
  await until(
    () => flatten(section('select/0')).some(node => node.text === 'English'),
    'Closed-trigger type-ahead directly selects English'
  )
  assert.ok(
    !flatten(tree()).some(node => node.type === 'Menu'),
    'Closed-trigger type-ahead does not open a popup'
  )
  await wait(800)
  shell('uitest', 'uiInput', 'keyEvent', '2026')
  await until(
    () => flatten(section('select/0')).some(node => node.text === 'Japanese'),
    'A later typing session starts a fresh prefix'
  )
  const button = flatten(section('select/0')).find(node =>
    /^dazz-select-\d+-trigger$/.test(node.id)
  )
  const geometry = (await inspectNative(button.id)).$geometry
  const tokens = JSON.parse(
    fs.readFileSync(
      'packages/csharp/DazzUI/src/DazzUI/Generated/tokens.json',
      'utf8'
    )
  )['studio-light']
  const length = value => value.value * (value.unit === 'rem' ? 16 : 1)
  const inset =
    length(tokens['spacing.3']) +
    length(tokens['stroke.hairline']) +
    length(tokens['icon.small']) / 2
  const b = box(button)
  shell(
    'uitest',
    'uiInput',
    'click',
    String(Math.round(b[2] - inset * geometry.density)),
    String(Math.round((b[1] + b[3]) / 2))
  )
  await until(
    root => findNode(root, node => node.type === 'Menu'),
    'The painted chevron passes activation to the native Select'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  await until(
    () =>
      flatten(section('select/0')).some(
        node =>
          /^dazz-select-\d+-trigger$/.test(node.id) && node.focused === 'true'
      ) && !flatten(tree()).some(node => node.type === 'Menu'),
    'Escape returns focus without changing the selected language'
  )
  assert.ok(flatten(section('select/0')).some(node => node.text === 'Japanese'))
  console.log(
    'Native Select open-list confirmation and closed-trigger type-ahead passed'
  )
})

await check('menu-keyboard', async () => {
  await launch('menu')
  await click(
    node => node.id === 'menu/0/0',
    'open action menu for keyboard',
    true
  )
  const panel = await until(
    root =>
      flatten(root).find(node => node.id?.startsWith('dazzui-menu-surface-')),
    'action menu for keyboard'
  )
  const focused = index =>
    flatten(tree()).some(
      node => node.id === panel.id + '-item-' + index && node.focused === 'true'
    )
  shell('uitest', 'uiInput', 'keyEvent', '2082')
  await until(() => focused(1), 'End skips the disabled Delete action')
  shell('uitest', 'uiInput', 'keyEvent', '2081')
  await until(() => focused(0), 'Home focuses Rename')
  shell('uitest', 'uiInput', 'keyEvent', '2020')
  await until(
    () => focused(1),
    'Typing D reaches Duplicate and skips disabled Delete'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2054')
  await until(
    root => !flatten(root).some(node => node.id === panel.id),
    'Enter activates the highlighted action'
  )
  await until(
    root => findLabeledButton(root, 'Actions')?.attributes.focused === 'true',
    'Action menu restores focus to its native trigger button'
  )
  await click(node => node.id === 'menu/0/0', 'reopen action menu for Escape')
  await until(
    root => flatten(root).some(node => node.id === panel.id),
    'action menu for Escape'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  await until(
    root =>
      !flatten(root).some(node => node.id === panel.id) &&
      findLabeledButton(root, 'Actions')?.attributes.focused === 'true',
    'Escape dismisses and restores native trigger focus'
  )
  await reveal(node => node.id === 'menu/1/0', 'context menu card')
  const card = box(findNode(tree(), node => node.id === 'menu/1/0').attributes)
  shell(
    'uinput',
    '-M',
    '-m',
    String(Math.round((card[0] + card[2]) / 2)),
    String(Math.round((card[1] + card[3]) / 2)),
    '-c',
    '1'
  )
  const context = await until(
    root =>
      flatten(root).find(node => node.id?.startsWith('dazzui-menu-surface-')),
    'native right-click menu'
  )
  const rows = flatten(findNode(tree(), node => node.id === context.id)).filter(
    node => node.type === 'MenuItem'
  )
  assert.equal(rows.length, 3)
  await click(node => node.id === rows[1].id, 'Duplicate from the context menu')
  await until(
    root => !flatten(root).some(node => node.id === context.id),
    'Native context-menu action closes its own panel'
  )
  console.log(
    'Native Menu Home/End, disabled skipping, type-ahead, Enter and right-click activation passed'
  )
})
await check('segmented-control', async () => {
  await launch('segmented-control')
  await click(node => node.text === 'Board', 'segment')
  await until(current => {
    const scope = findNode(
      current,
      node => node.id === 'example-segmented-control/0'
    )
    const tabs = findNode(
      scope,
      node => node.id?.startsWith('dazz-tabs-') && node.id.endsWith('-group')
    )
    return Boolean(
      findNode(tabs, node => node.selected === 'true') &&
      flatten(findNode(tabs, node => node.selected === 'true')).some(
        node => node.text === 'Board'
      )
    )
  }, 'controlled segment selection')
  shell('uitest', 'uiInput', 'keyEvent', '2015')
  await until(() => {
    const scope = section('segmented-control/0')
    const tabs = findNode(
      scope,
      node => node.id?.startsWith('dazz-tabs-') && node.id.endsWith('-group')
    )
    const selected = findNode(tabs, node => node.selected === 'true')
    return (
      selected &&
      flatten(selected).some(node => node.text === 'Board') &&
      flatten(tabs).some(node => node.focused === 'true')
    )
  }, 'segment arrows preserve the selected option')
  shell('uitest', 'uiInput', 'keyEvent', '2054')
  await until(() => {
    const tabs = findNode(
      section('segmented-control/0'),
      node => node.id?.startsWith('dazz-tabs-') && node.id.endsWith('-group')
    )
    const selected = findNode(tabs, node => node.selected === 'true')
    return selected && flatten(selected).some(node => node.text === 'Timeline')
  }, 'Enter confirms the focused segment')
  shell('uitest', 'uiInput', 'keyEvent', '2081')
  shell('uitest', 'uiInput', 'keyEvent', '2050')
  await until(() => {
    const selected = findNode(
      section('segmented-control/0'),
      node => node.selected === 'true'
    )
    return selected && flatten(selected).some(node => node.text === 'List')
  }, 'Home and Space confirm the first segment')
  shell('uitest', 'uiInput', 'keyEvent', '2082')
  shell('uitest', 'uiInput', 'keyEvent', '2050')
  await until(() => {
    const selected = findNode(
      section('segmented-control/0'),
      node => node.selected === 'true'
    )
    return selected && flatten(selected).some(node => node.text === 'Timeline')
  }, 'End and Space confirm the last segment')
  console.log('Native segmented selection and keyboard passed')
})
await check('combobox', async () => {
  await launch('combobox')
  const editor = await editorFor('combobox/0')
  tap(editor)
  await until(
    current => flatten(current).some(node => node.text === 'Japan'),
    'searchable choices open'
  )
  shell('uitest', 'uiInput', 'text', 'jap')
  shell('uitest', 'uiInput', 'keyEvent', '2054')
  await until(
    current =>
      flatten(current).some(node => node.text === 'Japan') &&
      !flatten(current).some(node => node.text === 'Germany'),
    'query filters choices'
  )
  await click(node => node.text === 'Japan', 'filtered choice')
  await until(
    () =>
      flatten(section('combobox/0')).some(
        node => node.type === 'TextInput' && node.text === 'Japan'
      ),
    'chosen label replaces query'
  )
  const clear = flatten(section('combobox/0')).filter(
    node => node.type === 'Button'
  )[0]
  tap(clear)
  await until(
    () =>
      flatten(section('combobox/0')).some(
        node => node.type === 'TextInput' && node.text === ''
      ),
    'selection clears'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  const autocomplete = await editorFor('combobox/2')
  tap(autocomplete)
  await until(
    current => flatten(current).some(node => node.text === 'Inter'),
    'autocomplete suggestions open'
  )
  await click(node => node.text === 'JetBrains Mono', 'autocomplete suggestion')
  await until(
    () =>
      flatten(section('combobox/2')).some(
        node => node.type === 'TextInput' && node.text === 'JetBrains Mono'
      ),
    'autocomplete accepts suggestion'
  )
  console.log(
    'Native combobox search, selection, clear and autocomplete passed'
  )
})

await check('combobox-keyboard', async () => {
  await launch('combobox')
  tap(await editorFor('combobox/0'))
  await until(
    current => flatten(current).some(node => node.text === 'Japan'),
    'keyboard choices open'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2013')
  shell('uitest', 'uiInput', 'keyEvent', '2013')
  shell('uitest', 'uiInput', 'keyEvent', '2054')
  await until(
    () =>
      flatten(section('combobox/0')).some(
        node => node.type === 'TextInput' && node.text === 'Germany'
      ),
    'Down and Enter choose the highlighted item'
  )
  tap(await editorFor('combobox/0'))
  shell('uitest', 'uiInput', 'keyEvent', '2072', '2017')
  shell('uitest', 'uiInput', 'text', 'zzzz')
  shell('uitest', 'uiInput', 'keyEvent', '2054')
  await until(
    current =>
      flatten(current).some(node => node.text === 'No country by that name'),
    'empty query result'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  await until(
    () =>
      flatten(section('combobox/0')).some(
        node => node.type === 'TextInput' && node.text === 'Germany'
      ),
    'Escape restores selected label'
  )
  tap(await editorFor('combobox/2'))
  shell('uitest', 'uiInput', 'text', 'xyz')
  shell('uitest', 'uiInput', 'keyEvent', '2054')
  await until(
    () =>
      flatten(section('combobox/2')).some(
        node => node.type === 'TextInput' && node.text === 'xyz'
      ),
    'autocomplete accepts arbitrary text'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2070')
  assert.ok(
    flatten(section('combobox/2')).some(
      node => node.type === 'TextInput' && node.text === 'xyz'
    ),
    'Escape preserves free text'
  )
  console.log(
    'Native query keyboard, empty result, Escape and free-text input passed'
  )
})

await check('panels', async () => {
  await launch('')
  await click(node => node.id === 'toggle-theme-panel', 'theme panel toggle')
  await until(
    current => !flatten(current).some(node => node.id === 'theme-panel'),
    'theme panel collapsed'
  )
  await click(node => node.id === 'toggle-theme-panel', 'theme panel toggle')
  await until(
    current => flatten(current).some(node => node.id === 'theme-panel'),
    'theme panel restored'
  )
  await click(node => node.id === 'toggle-sidebar', 'sidebar toggle')
  await until(
    current => !flatten(current).some(node => node.id === 'component-list'),
    'sidebar collapsed'
  )
  await click(node => node.id === 'toggle-sidebar', 'sidebar toggle')
  await until(
    current => flatten(current).some(node => node.id === 'component-list'),
    'sidebar restored'
  )
  console.log('Native gallery panel controls passed')
})
await check('pill-tabs', async () => {
  await launch('pill-tabs')
  const scope = section('pill-tabs/0')
  const root = findNode(
    scope,
    node => node.id?.startsWith('dazz-tabs-') && node.id.endsWith('-group')
  )
  const outer = box(scope.attributes)
  const inner = box(root.attributes)
  assert.ok(
    inner[2] - inner[0] < (outer[2] - outer[0]) / 2,
    'Pills retain intrinsic width'
  )
  assert.ok(
    Math.abs(inner[0] + inner[2] - outer[0] - outer[2]) <= 4,
    'Pills are centered'
  )
  assert.ok(
    ['24', '3'].every(count =>
      flatten(scope).some(node => node.text === count)
    ),
    'Counts are separate label runs'
  )
  await click(node => node.text === 'All', 'all pill')
  const selectedLabel = () => {
    const selected = findNode(
      section('pill-tabs/0'),
      node => node.selected === 'true'
    )
    return selected
      ? flatten(selected)
          .map(node => node.text)
          .join(' ')
      : ''
  }
  await until(() => selectedLabel().includes('All'), 'pill click selects')
  shell('uitest', 'uiInput', 'keyEvent', '2015')
  assert.ok(selectedLabel().includes('All'), 'Pill arrows preserve selection')
  shell('uitest', 'uiInput', 'keyEvent', '2054')
  await until(
    () => selectedLabel().includes('Unread'),
    'Enter confirms the focused pill'
  )
  shell('uitest', 'uiInput', 'keyEvent', '2049')
  await until(() => {
    const tabs = findNode(
      section('pill-tabs/0'),
      node => node.id?.startsWith('dazz-tabs-') && node.id.endsWith('-group')
    )
    return tabs.attributes.focused !== 'true'
  }, 'Tab leaves the pill group in one step')
  assert.ok(
    selectedLabel().includes('Unread'),
    'Tab preserves the selected pill'
  )
  console.log('Native pill width, count layout, selection and keyboard passed')
})

await check('overview', async () => {
  await launch('')
  const seen = new Set()
  for (let step = 0; step < 48; step++) {
    const current = tree()
    const document = findNode(current, node => node.id === 'document-column')
    for (const node of flatten(document)) {
      if (!node.id?.startsWith('overview-')) continue
      const id = node.id.slice('overview-'.length)
      seen.add(id)
      if (restored.includes(id)) {
        const tile = findNode(document, item => item.id === node.id)
        assert.ok(
          !flatten(tile).some(item => item.text === 'Mapping gaps'),
          'Preview maps ' + id
        )
      }
    }
    if (seen.size === stories.length) break
    const [left, top, right, bottom] = box(document.attributes)
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
  assert.deepEqual(
    [...seen].sort(),
    stories.map(story => story.id).sort(),
    'Every overview preview is reachable'
  )
  console.log('Native overview rendered ' + seen.size + ' preview tiles')
})

await check('stage', async () => {
  await launch('icon')
  const scope = section('icon/0')
  const row = findNode(scope, node => node.type === 'Flex')
  const card = findNode(scope, node => node.id === 'stage-icon/0')
  assert.ok(row, 'Native story row exists')
  assert.ok(card, 'Native stage card exists')
  const rail = box(
    findNode(tree(), node => node.id === 'component-list').attributes
  )
  const density = (rail[2] - rail[0]) / 272
  const stage = box(card.attributes)
  assert.ok(
    Math.abs(stage[3] - stage[1] - 160 * density) <= 3,
    'The section card minimum includes padding, matching React'
  )
  const outer = box(scope.attributes)
  const inner = box(row.attributes)
  assert.ok(
    inner[2] - inner[0] < (outer[2] - outer[0]) / 2,
    'An unsized row takes its contents width'
  )
  assert.ok(
    Math.abs(inner[0] + inner[2] - outer[0] - outer[2]) <= 4,
    'The story row is centered in the card'
  )
  assert.ok(
    Math.abs(inner[1] + inner[3] - stage[1] - stage[3]) <= 4,
    'The story row is vertically centered within the card'
  )
  console.log('Native intrinsic story layout and centering passed')
})

await check('window', async () => {
  await launch('segmented-control')
  await click(node => node.text === 'Board', 'segment before resizing')
  const geometry = (current = tree()) => {
    const nodes = flatten(current)
    const document = nodes.find(node => node.id === 'document-column')
    const root = nodes.find(
      node =>
        node.type === 'RowSplit' && node.hostWindowId === document.hostWindowId
    )
    const columns = ['component-list', 'document-column', 'theme-panel'].map(
      id => box(nodes.find(node => node.id === id))
    )
    return { nodes, root: box(root), columns, window: document.hostWindowId }
  }
  const settledGeometry = async () => {
    let previous
    return until(current => {
      const latest = geometry(current)
      const stable =
        previous &&
        JSON.stringify(latest.columns) === JSON.stringify(previous.columns)
      previous = latest
      return stable ? latest : undefined
    }, 'split geometry settles')
  }
  const initial = geometry()
  const verify = result => {
    const [list, center, themes] = result.columns
    assert.ok(
      Math.abs(themes[2] - result.root[2]) <= 2,
      'Columns fill the window'
    )
    assert.ok(
      Math.abs(center[0] - list[2]) <= 3 &&
        Math.abs(themes[0] - center[2]) <= 3,
      'No gaps or overlap'
    )
    assert.ok(
      result.columns.every(column => Math.abs(column[3] - result.root[3]) <= 2),
      'Columns follow window height'
    )
  }
  verify(initial)
  tap(
    initial.nodes.find(
      node =>
        node.id === 'EnhanceMaximizeBtn' && node.hostWindowId === initial.window
    )
  )
  await until(current => {
    const changed = geometry(current)
    return (
      Math.abs(
        changed.root[2] - changed.root[0] - (initial.root[2] - initial.root[0])
      ) > 100
    )
  }, 'window restores')
  const restored = geometry()
  verify(restored)
  for (const i of [0, 2])
    assert.ok(
      Math.abs(
        restored.columns[i][2] -
          restored.columns[i][0] -
          (initial.columns[i][2] - initial.columns[i][0])
      ) <= 2,
      'Sidebars retain their widths'
    )
  const chosen = findNode(
    section('segmented-control/0'),
    node => node.selected === 'true'
  )
  assert.ok(
    flatten(chosen).some(node => node.text === 'Board'),
    'Resizing preserves story state'
  )
  tap(
    restored.nodes.find(
      node =>
        node.id === 'EnhanceMaximizeBtn' &&
        node.hostWindowId === restored.window
    )
  )
  await until(current => {
    const changed = geometry(current)
    return (
      Math.abs(
        changed.root[2] - changed.root[0] - (initial.root[2] - initial.root[0])
      ) < 3
    )
  }, 'window maximizes')
  for (const column of [0, 2]) {
    const beforeDrag = await settledGeometry()
    const x =
      column === 0 ? beforeDrag.columns[0][2] + 1 : beforeDrag.columns[2][0] - 1
    const y = Math.round((beforeDrag.root[1] + beforeDrag.root[3]) / 2)
    shell(
      'uitest',
      'uiInput',
      'drag',
      String(x),
      String(y),
      String(x + (column === 0 ? 100 : -100)),
      String(y)
    )
    await until(current => {
      const changed = geometry(current)
      return (
        changed.columns[column][2] - changed.columns[column][0] >
        beforeDrag.columns[column][2] - beforeDrag.columns[column][0] + 50
      )
    }, 'native splitter drags')
    const dragged = await settledGeometry()
    verify(dragged)
    const chosenWidth = dragged.columns[column][2] - dragged.columns[column][0]
    tap(
      dragged.nodes.find(
        node =>
          node.id === 'EnhanceMaximizeBtn' &&
          node.hostWindowId === dragged.window
      )
    )
    let lastGeometry = dragged
    await until(current => {
      const resized = geometry(current)
      lastGeometry = resized
      return (
        Math.abs(
          resized.root[2] -
            resized.root[0] -
            (dragged.root[2] - dragged.root[0])
        ) > 100 &&
        Math.abs(
          resized.columns[column][2] - resized.columns[column][0] - chosenWidth
        ) <= 2
      )
    }, 'dragged sidebar width survives restoring the window').catch(error => {
      throw new Error(
        `${error.message}; column ${column}, expected ${chosenWidth}px, ` +
          `actual ${JSON.stringify(lastGeometry.columns[column])}`,
        { cause: error }
      )
    })
    const resized = geometry()
    tap(
      resized.nodes.find(
        node =>
          node.id === 'EnhanceMaximizeBtn' &&
          node.hostWindowId === resized.window
      )
    )
    await until(current => {
      const expanded = geometry(current)
      return (
        Math.abs(
          expanded.root[2] -
            expanded.root[0] -
            (dragged.root[2] - dragged.root[0])
        ) < 3 &&
        Math.abs(
          expanded.columns[column][2] -
            expanded.columns[column][0] -
            chosenWidth
        ) <= 2
      )
    }, 'dragged sidebar width survives maximizing the window')
    verify(geometry())
  }
  console.log(
    'Native window resizing, state retention and splitter drag passed'
  )
})
if (checks)
  for (const name of checks)
    assert.ok(performed.has(name), 'Unknown check: ' + name)
console.log('Gallery checks passed on ' + device)
