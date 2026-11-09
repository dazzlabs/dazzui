/** Compare saved native font readings with the host's installed Chromium.
 * Node 22+, no workspace dependencies: --native <json> --chrome <executable>
 * [--out <directory>]. Reads the committed React token stylesheet directly. */
import assert from 'node:assert/strict'
import { spawn } from 'node:child_process'
import { mkdir, mkdtemp, open, readFile, writeFile } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { join, resolve } from 'node:path'
import { setTimeout as pause } from 'node:timers/promises'
import { fileURLToPath, pathToFileURL } from 'node:url'
import { parseArgs } from 'node:util'

const { values: args } = parseArgs({
  options: {
    native: { type: 'string' },
    chrome: { type: 'string' },
    'no-sandbox': { type: 'boolean' },
    out: {
      type: 'string',
      default: join(tmpdir(), `dazzui-browser-fonts-${process.pid}`),
    },
  },
})
assert.ok(args.native && args.chrome, '--native and --chrome are required')
const repo = fileURLToPath(new URL('../../../../../', import.meta.url))
const out = resolve(args.out)
const native = JSON.parse(
  (await readFile(resolve(args.native), 'utf8')).replace(/^\uFEFF/, '')
)
assert.ok(native.readings?.length, 'native readings must not be empty')
const requests = native.readings.map(reading => reading.request)
await mkdir(out, { recursive: true })
const page = join(out, 'font-probes.html')
const stylesheet = pathToFileURL(
  join(repo, 'packages/js/dazzui/src/generated/style.css')
)
await writeFile(
  page,
  `<!doctype html><html lang="en"><head><link rel="stylesheet" href="${stylesheet}"></head><body><script>
const requests = ${JSON.stringify(requests).replaceAll('<', '\\u003c')}
const context = document.createElement('canvas').getContext('2d')
window.fontReferences = requests.map(request => {
  const specimen = document.createElement('span')
  specimen.className = 'font-probe'
  specimen.style.cssText = 'position:absolute;white-space:pre;'
  specimen.style.fontFamily = request.face === 'mono' ? 'var(--base-font-code)' : 'var(--base-font-ui)'
  specimen.style.fontSize = request.size + 'px'
  specimen.style.fontWeight = request.weight
  specimen.textContent = request.text
  document.body.append(specimen)
  context.font = request.weight + ' ' + request.size + 'px ' + getComputedStyle(specimen).fontFamily
  const metrics = context.measureText(request.text)
  return { font: context.font, width: metrics.width, ascent: metrics.fontBoundingBoxAscent, descent: metrics.fontBoundingBoxDescent }
})
</script></body></html>`
)
const profile = await mkdtemp(join(out, 'chromium-'))
const log = await open(join(out, 'browser.log'), 'w')
const child = spawn(
  args.chrome,
  [
    '--headless',
    '--disable-gpu',
    '--no-first-run',
    '--no-default-browser-check',
    '--remote-debugging-port=0',
    '--remote-debugging-address=127.0.0.1',
    ...(args['no-sandbox'] ? ['--no-sandbox'] : []),
    `--user-data-dir=${profile}`,
    pathToFileURL(page).href,
  ],
  { stdio: ['ignore', 'ignore', log.fd] }
)
let launchError,
  socket,
  sequence = 0
const pending = new Map()
child.on('error', error => {
  launchError = error
})
const send = (method, params = {}) =>
  new Promise((resolve, reject) => {
    const id = ++sequence
    const timer = setTimeout(() => {
      pending.delete(id)
      reject(new Error(`CDP timeout: ${method}`))
    }, 30000)
    pending.set(id, { resolve, reject, timer })
    socket.send(JSON.stringify({ id, method, params }))
  })
const readings = []
try {
  let port
  for (let attempt = 0; attempt < 150; attempt++) {
    if (launchError) throw launchError
    assert.equal(
      child.exitCode,
      null,
      'browser exited before opening its debug endpoint'
    )
    assert.equal(
      child.signalCode,
      null,
      'browser was terminated before opening its debug endpoint'
    )
    try {
      port = Number(
        (await readFile(join(profile, 'DevToolsActivePort'), 'utf8')).split(
          '\n'
        )[0]
      )
      break
    } catch {
      await pause(100)
    }
  }
  assert.ok(port, 'browser did not open a debug endpoint')
  let target
  for (let attempt = 0; attempt < 100; attempt++) {
    const targets = await (
      await fetch(`http://127.0.0.1:${port}/json/list`)
    ).json()
    target = targets.find(
      target =>
        target.type === 'page' && target.url === pathToFileURL(page).href
    )
    if (target) break
    await pause(100)
  }
  assert.ok(target, 'browser did not open the probe page')
  socket = new WebSocket(target.webSocketDebuggerUrl)
  socket.addEventListener('message', event => {
    const data = JSON.parse(event.data),
      call = pending.get(data.id)
    if (!call) return
    pending.delete(data.id)
    clearTimeout(call.timer)
    if (data.error) call.reject(new Error(JSON.stringify(data.error)))
    else call.resolve(data.result)
  })
  socket.addEventListener('close', () => {
    for (const call of pending.values()) {
      clearTimeout(call.timer)
      call.reject(new Error('browser connection closed'))
    }
    pending.clear()
  })
  await new Promise((resolve, reject) => {
    socket.addEventListener('open', resolve, { once: true })
    socket.addEventListener('error', reject, { once: true })
  })
  await send('DOM.enable')
  await send('CSS.enable')
  let references
  for (let attempt = 0; attempt < 100; attempt++) {
    const result = await send('Runtime.evaluate', {
      expression: 'window.fontReferences',
      returnByValue: true,
    })
    references = result.result.value
    if (references) break
    await pause(100)
  }
  assert.equal(
    references?.length,
    requests.length,
    'probe page did not measure every request'
  )
  const { root } = await send('DOM.getDocument')
  const { nodeIds } = await send('DOM.querySelectorAll', {
    nodeId: root.nodeId,
    selector: '.font-probe',
  })
  assert.equal(nodeIds.length, requests.length)
  for (const [index, actual] of native.readings.entries()) {
    const browser = references[index]
    const { fonts } = await send('CSS.getPlatformFontsForNode', {
      nodeId: nodeIds[index],
    })
    readings.push({
      ...actual.request,
      family: actual.family,
      families: actual.families,
      runMetrics: actual.runMetrics,
      nativeCarets: actual.carets,
      nativeGlyphs: actual.glyphs,
      browserMetrics: { ascent: browser.ascent, descent: browser.descent },
      browserFont: browser.font,
      browserFonts: fonts,
      reference: browser.width,
      actual: actual.width,
      difference: actual.width - browser.width,
    })
  }
  await send('Browser.close').catch(() => {})
} finally {
  socket?.close()
  for (const call of pending.values()) clearTimeout(call.timer)
  if (child.exitCode === null) {
    await pause(500)
    if (child.exitCode === null) child.kill()
  }
  await log.close()
}
const failures = readings.filter(reading => Math.abs(reading.difference) > 0.02)
await writeFile(
  join(out, 'report.json'),
  JSON.stringify(
    { platform: native.platform, cases: readings.length, failures, readings },
    null,
    2
  ) + '\n'
)
console.log(
  `${readings.length} host native/browser widths; ${failures.length} differences — ${out}`
)
if (failures.length) {
  console.error(JSON.stringify(failures.slice(0, 8), null, 2))
  process.exitCode = 1
}
