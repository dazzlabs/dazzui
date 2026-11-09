/** Compare native soft rows with the browser textarea's actual editor ranges.
 * Build font-metrics first. Runs on the host whose fonts are being compared.
 * Saved readings retain source ranges, glyphs and per-grapheme rectangles.
 */
import assert from 'node:assert/strict'
import { createHash } from 'node:crypto'
import { execFile as callback } from 'node:child_process'
import { mkdir, readFile, writeFile } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { join, resolve } from 'node:path'
import { fileURLToPath, pathToFileURL } from 'node:url'
import { parseArgs, promisify } from 'node:util'

const repo = fileURLToPath(new URL('../../../../../', import.meta.url))
const { values: args } = parseArgs({
  options: {
    out: {
      type: 'string',
      default: join(tmpdir(), `dazzui-font-wrap-${process.pid}`),
    },
    binary: {
      type: 'string',
      default: join(repo, 'packages/rust/dazzui/target/debug/font-metrics'),
    },
    cases: { type: 'string' },
    reference: { type: 'string' },
  },
})
const out = resolve(args.out)
const texts = [
  ['lam-alef', 'لا'],
  ['double-lam', 'لل'],
  ['allah', 'الله'],
  ['arabic-word', 'لاختبار'],
  ['latin-ffi', 'ffi'],
  ['latin-kerning', 'AV To office'],
  ['arabic-marks', 'مَرْحَبًا'],
  ['hebrew', 'שלום עולם'],
  ['indic', 'क्षि कर्मि नमस्ते'],
  ['cjk-emoji', '中A👨‍👩‍👧‍👦B文'],
  ['embedding', 'ab \u202bאבג 123 xyz\u202c end'],
  ['override', 'ab \u202eAV To\u202c cd'],
]
const selected = args.cases?.split(',')
for (const name of selected ?? [])
  assert(
    texts.some(([id]) => id === name),
    name
  )
const requests = ['text', 'mono'].flatMap(face =>
  [
    { size: 12, weight: 400 },
    { size: 13.33, weight: 600 },
  ].flatMap(style =>
    texts
      .filter(([id]) => !selected || selected.includes(id))
      .flatMap(([id, text]) =>
        [2, 4, 6, 8, 12, 32, 80, 214].map(wrap => ({
          id,
          text,
          face,
          ...style,
          wrap,
        }))
      )
  )
)
await mkdir(out, { recursive: true })
const file = join(out, 'requests.json')
await writeFile(file, JSON.stringify(requests))
const nativeResult = await promisify(callback)(resolve(args.binary), [file], {
  maxBuffer: 64 * 1024 * 1024,
})
await writeFile(join(out, 'native.json'), nativeResult.stdout)
await writeFile(join(out, 'native.log'), nativeResult.stderr)
const native = JSON.parse(nativeResult.stdout)
assert.equal(native.readings.length, requests.length)
const readings = [],
  failures = [],
  references = []
let referenceSource = null
if (args.reference) {
  const data = await readFile(resolve(args.reference))
  const previous = JSON.parse(data)
  assert.deepEqual(
    previous.readings.map(r => r.request),
    requests,
    'saved browser reference must have the same requests'
  )
  references.push(...previous.readings.map(r => r.reference))
  referenceSource = {
    path: resolve(args.reference),
    sha256: createHash('sha256').update(data).digest('hex'),
  }
} else {
  const example = join(repo, 'packages/js/dazzui/example')
  const { createServer } = await import(
    pathToFileURL(join(example, 'node_modules/vite/dist/node/index.js'))
  )
  const { chromium } = await import(
    pathToFileURL(join(example, 'node_modules/playwright/index.mjs'))
  )
  const server = await createServer({
    root: example,
    logLevel: 'silent',
    server: { port: 0, host: '127.0.0.1' },
  })
  await server.listen()
  const browser = await chromium.launch({
    executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE || undefined,
  })
  try {
    const page = await browser.newPage()
    const cdp = await page.context().newCDPSession(page)
    await page.goto(
      new URL('conformance.html', server.resolvedUrls.local[0]).href
    )
    await page.waitForFunction(() => window.conformance)
    await page.evaluate(() =>
      window.conformance.show('text-field/1', 'studio-light')
    )
    const families = await page.evaluate(() => {
      const fields = document.querySelectorAll('.dz-text-field')
      return {
        text: getComputedStyle(fields[0]).fontFamily,
        mono: getComputedStyle(fields[3]).fontFamily,
      }
    })
    for (const request of requests) {
      await page.evaluate(() =>
        window.conformance.show('text-field/2', 'studio-light')
      )
      const input = page.locator('#root .dz-text-field').first()
      await input.evaluate(
        (node, { request, families }) => {
          const style = getComputedStyle(node)
          const chrome = [
            'paddingLeft',
            'paddingRight',
            'borderLeftWidth',
            'borderRightWidth',
          ].reduce((sum, key) => sum + parseFloat(style[key]), 0)
          Object.assign(node.style, {
            position: 'fixed',
            left: '160px',
            top: '24px',
            width: `${request.wrap + chrome}px`,
            height: '600px',
            margin: '0',
            fontFamily: families[request.face],
            fontSize: `${request.size}px`,
            fontWeight: request.weight,
          })
          node.setAttribute('data-wrap-probe', '')
        },
        { request, families }
      )
      await input.fill(request.text)
      const { root } = await cdp.send('DOM.getDocument', {
        depth: -1,
        pierce: true,
      })
      let editor
      const find = node => {
        if ((node.attributes ?? []).includes('data-wrap-probe')) {
          const visit = node => {
            if (
              node.nodeName === 'DIV' &&
              node.children?.some(c => c.nodeType === 3)
            )
              editor = node
            for (const child of node.children ?? []) visit(child)
          }
          for (const shadow of node.shadowRoots ?? []) visit(shadow)
        }
        for (const child of [
          ...(node.children ?? []),
          ...(node.shadowRoots ?? []),
        ])
          find(child)
      }
      find(root)
      assert(editor, 'the actual textarea editor must be available')
      const { object } = await cdp.send('DOM.resolveNode', {
        nodeId: editor.nodeId,
      })
      const response = await cdp.send('Runtime.callFunctionOn', {
        objectId: object.objectId,
        returnByValue: true,
        functionDeclaration: `function() {
        const walker = document.createTreeWalker(this, NodeFilter.SHOW_TEXT), nodes = [];
        let node, offset = 0;
        while (node = walker.nextNode()) { nodes.push({ node, start: offset }); offset += node.length; }
        const text = nodes.map(n => n.node.data).join(''), segments = [], box = this.getBoundingClientRect();
        const point = index => { const n = nodes.find(n => n.start <= index && index <= n.start + n.node.length); return [n.node, index - n.start]; };
        for (const segment of new Intl.Segmenter('und', { granularity: 'grapheme' }).segment(text)) {
          const range = document.createRange(); range.setStart(...point(segment.index)); range.setEnd(...point(segment.index + segment.segment.length));
          segments.push({ from: segment.index, to: segment.index + segment.segment.length, text: segment.segment,
            rects: [...range.getClientRects()].map(r => ({ left: r.left - box.left, right: r.right - box.left, top: r.top - box.top, width: r.width })) });
        }
        return { text, width: box.width, segments };
      }`,
      })
      const reference = response.result.value
      assert.equal(reference.text, request.text)
      references.push(reference)
    }
  } finally {
    await browser.close()
    await server.close()
  }
}
for (const [at, request] of requests.entries()) {
  const reference = references[at]
  const actual = native.readings[at],
    errors = [],
    expectedRows = new Map()
  const byteAt = index => Buffer.byteLength(request.text.slice(0, index))
  for (const segment of reference.segments) {
    // Formatting controls have no visible box; their ordinary caret sides
    // are checked by field-caret. Preserve their raw rectangles here.
    const rect = segment.rects.reduce(
      (a, b) => (!a || b.width > a.width ? b : a),
      null
    )
    assert(rect, 'a grapheme must have a browser range')
    if (!expectedRows.has(rect.top)) expectedRows.set(rect.top, [])
    if (/^[\p{Cf}]*$/u.test(segment.text)) continue
    expectedRows.get(rect.top).push({
      from: byteAt(segment.from),
      to: byteAt(segment.to),
      text: segment.text,
      ...rect,
    })
  }
  const expected = [...expectedRows.values()]
  if (actual.rows.length !== expected.length)
    errors.push(`row count ${actual.rows.length} != ${expected.length}`)
  for (const [rowIndex, row] of actual.rows.entries()) {
    const wanted = expected[rowIndex]
    if (!wanted) continue
    const visible = reference.segments
      .filter(
        s =>
          !/^[\p{Cf}]*$/u.test(s.text) &&
          row.source[0] <= byteAt(s.from) &&
          byteAt(s.from) < row.source[1]
      )
      .map(s => byteAt(s.from))
    if (JSON.stringify(visible) !== JSON.stringify(wanted.map(s => s.from)))
      errors.push(`row ${rowIndex} source membership differs`)
    if (wanted.some(s => /[^\s\p{Cf}]/u.test(s.text)) && !row.glyphs.length)
      errors.push(`row ${rowIndex} has no glyphs`)
    for (const segment of wanted) {
      const first = row.carets.find(
        c => c.index === segment.from - row.source[0]
      )
      const last = row.carets.find(c => c.index === segment.to - row.source[0])
      if (!first || !last) {
        errors.push(`row ${rowIndex} lacks grapheme edges`)
        continue
      }
      const left = Math.min(first.leading, last.trailing),
        right = Math.max(first.leading, last.trailing)
      if (
        Math.abs(left - segment.left) > 0.02 ||
        Math.abs(right - segment.right) > 0.02
      )
        errors.push(
          `row ${rowIndex} edges at ${segment.from}: ${left},${right} != ${segment.left},${segment.right}`
        )
    }
  }
  const reading = { request, native: actual.rows, reference, errors }
  readings.push(reading)
  if (errors.length) failures.push(reading)
}
await writeFile(
  join(out, 'report.json'),
  JSON.stringify(
    { cases: readings.length, referenceSource, failures, readings },
    null,
    2
  ) + '\n'
)
console.log(
  `${readings.length} native/browser wrapped layouts; ${failures.length} failures — ${out}`
)
if (failures.length) {
  console.error(
    failures.map(f => ({ ...f.request, errors: f.errors.slice(0, 5) }))
  )
  process.exitCode = 1
}
