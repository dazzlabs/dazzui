/** Pair real platform shaping with browser canvas advances. Build both example
 * binaries first, then run with --out <directory> [--binary <font-metrics>]. */
import assert from 'node:assert/strict'
import { execFile as execCallback } from 'node:child_process'
import { mkdir, writeFile } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { join, resolve } from 'node:path'
import { fileURLToPath, pathToFileURL } from 'node:url'
import { parseArgs, promisify } from 'node:util'

const repo = fileURLToPath(new URL('../../../../../', import.meta.url))
const { values: args } = parseArgs({
  options: {
    out: {
      type: 'string',
      default: join(tmpdir(), `dazzui-font-metrics-${process.pid}`),
    },
    binary: {
      type: 'string',
      default: join(repo, 'packages/rust/dazzui/target/debug/font-metrics'),
    },
    cases: { type: 'string' },
  },
})
const out = resolve(args.out)
await mkdir(out, { recursive: true })
const texts = [
  ['latin', 'abc def ghi jkl mno pqr '.repeat(2)],
  ['ligatures', 'office affine AV To ffi'],
  ['combining', 'cafe\u0301 fi\u0301x'],
  ['hebrew', 'שלום עולם'],
  ['mixed-bidi', 'abאבcd'],
  ['arabic', 'مرحبا بالعالم'],
  ['devanagari', 'नमस्ते दुनिया'],
  ['mixed-cjk', 'DazzUI 世界 end'],
  ['cjk', '世界'],
  ['mixed-emoji', 'Hello 👨‍👩‍👧‍👦 end'],
  ['emoji', '👨‍👩‍👧‍👦'],
]
const selected = args.cases?.split(',')
if (selected) {
  for (const id of selected)
    assert(
      texts.some(([name]) => name === id),
      id
    )
}
const requests = ['text', 'mono'].flatMap(face =>
  [
    ...[12, 13, 13.33, 14, 25, 26].map(size => ({ size, weight: 400 })),
    ...[500, 600, 700].map(weight => ({ size: 13.33, weight })),
  ].flatMap(style =>
    texts
      .filter(([id]) => !selected || selected.includes(id))
      .map(([id, text]) => ({ face, ...style, id, text }))
  )
)
const file = join(out, 'requests.json')
await writeFile(file, JSON.stringify(requests))
const { stdout, stderr } = await promisify(execCallback)(
  resolve(args.binary),
  [file],
  { maxBuffer: 8 * 1024 * 1024 }
)
await writeFile(join(out, 'native.json'), stdout)
await writeFile(join(out, 'native.log'), stderr)
const native = JSON.parse(stdout)
const example = join(repo, 'packages/js/dazzui/example')
const { createServer } = await import(
  pathToFileURL(join(example, 'node_modules/vite/dist/node/index.js'))
)
const { chromium } = await import(
  pathToFileURL(join(example, 'node_modules/playwright/index.mjs'))
)
const server = await createServer({
  root: example,
  cacheDir: 'node_modules/.vite-native-font-metrics',
  logLevel: 'silent',
  server: { host: '127.0.0.1', port: 0 },
})
await server.listen()
const browser = await chromium.launch({
  executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE || undefined,
})
const readings = []
try {
  const page = await browser.newPage()
  await page.goto(
    new URL('conformance.html', server.resolvedUrls.local[0]).href
  )
  await page.waitForFunction(() => window.conformance)
  await page.evaluate(() =>
    window.conformance.show('text-field/1', 'studio-light')
  )
  const reference = await page.evaluate(requests => {
    const fields = document.querySelectorAll('.dz-text-field')
    const families = {
      text: getComputedStyle(fields[0]).fontFamily,
      mono: getComputedStyle(fields[3]).fontFamily,
    }
    const context = document.createElement('canvas').getContext('2d')
    return requests.map((request, index) => {
      context.font = `${request.weight} ${request.size}px ${families[request.face]}`
      const specimen = document.createElement('span')
      specimen.id = `font-metrics-${index}`
      specimen.className = 'font-metrics-probe'
      specimen.style.cssText = 'position:absolute;white-space:pre;'
      specimen.style.font = context.font
      specimen.textContent = request.text
      document.body.append(specimen)
      return {
        font: context.font,
        width: context.measureText(request.text).width,
      }
    })
  }, requests)
  assert.equal(reference.length, native.readings.length)
  const session = await page.context().newCDPSession(page)
  await session.send('DOM.enable')
  await session.send('CSS.enable')
  const { root } = await session.send('DOM.getDocument')
  const { nodeIds } = await session.send('DOM.querySelectorAll', {
    nodeId: root.nodeId,
    selector: '.font-metrics-probe',
  })
  assert.equal(nodeIds.length, reference.length)
  const platformFonts = await Promise.all(
    nodeIds.map(nodeId =>
      session.send('CSS.getPlatformFontsForNode', { nodeId })
    )
  )
  for (const [index, actual] of native.readings.entries()) {
    const browser = reference[index]
    readings.push({
      ...actual.request,
      family: actual.family,
      families: actual.families,
      browserFont: browser.font,
      browserFonts: platformFonts[index].fonts,
      reference: browser.width,
      actual: actual.width,
      difference: actual.width - browser.width,
    })
  }
} finally {
  await browser.close()
  await server.close()
}
const failures = readings.filter(reading => Math.abs(reading.difference) > 0.02)
await writeFile(
  join(out, 'report.json'),
  JSON.stringify({ cases: readings.length, failures, readings }, null, 2) + '\n'
)
console.log(
  `${readings.length} native/browser font widths; ${failures.length} differences — ${out}`
)
if (failures.length) {
  console.error(JSON.stringify(failures.slice(0, 8), null, 2))
  process.exitCode = 1
}
