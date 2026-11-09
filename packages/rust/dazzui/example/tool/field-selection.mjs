/** Compare actual browser highlight pixels with real-window selection quads.
 * Build dazzui-example first, then run with --out <dir> [--themes drawn|ids].
 * macOS reads actual Metal pixels by default; --window-capture reads the OS
 * window instead, requiring Screen Recording. --quads reads submitted boxes.
 * The pixel audit excludes paired partial edge pixels at renderer AA curves.
 * Includes wrapped RTL, Arabic and explicit embedding cases, and exits
 * nonzero for any submitted selection geometry that differs from the browser.
 */
import assert from 'node:assert/strict'
import { execFile as execFileCallback } from 'node:child_process'
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
      default: join(tmpdir(), `dazzui-field-selection-${process.pid}`),
    },
    themes: { type: 'string', default: 'drawn' },
    pixels: { type: 'boolean', default: process.platform === 'darwin' },
    quads: { type: 'boolean', default: false },
    'window-capture': { type: 'boolean', default: false },
    cases: { type: 'string' },
    binary: {
      type: 'string',
      default: join(repo, 'packages/rust/dazzui/target/debug/dazzui-example'),
    },
  },
})
const pixels = args.pixels && !args.quads
const out = resolve(args.out),
  example = join(repo, 'packages/js/dazzui/example'),
  execFile = promisify(execFileCallback)
const { createServer } = await import(
  pathToFileURL(join(example, 'node_modules/vite/dist/node/index.js'))
)
const { chromium } = await import(
  pathToFileURL(join(example, 'node_modules/playwright/index.mjs'))
)
// Ranges name grapheme units, matching the native arrow-key replay below.
const cases = [
  { id: 'ascii', text: 'one two three', from: 2, to: 7 },
  { id: 'bidi-character', text: 'abc אבג def', from: 4, to: 5 },
  { id: 'bidi-disjoint', text: 'abc אבג def', from: 2, to: 5 },
  { id: 'bidi-all', text: 'abc אבג def', from: 0, to: 11 },
  ...['AV', 'VA', 'To', 'Yo', 'Ta', 'WA', 'LT', 'FA'].flatMap(pair =>
    [0, 1].map(index => ({
      id: `rtl-kerning-${pair}-${index}`,
      text: `abc \u202e${pair}\u202c xyz`,
      from: 5 + index,
      to: 6 + index,
    }))
  ),
  { id: 'cjk-bidi', text: '中文 abc אבג xyz', from: 1, to: 8 },
  { id: 'newline', text: 'one\ntwo\nthree', from: 2, to: 5, multiline: true },
  {
    id: 'paragraphs',
    text: 'one\ntwo\nthree',
    from: 0,
    to: 13,
    multiline: true,
  },
  { id: 'empty-line', text: 'one\n\nthree', from: 0, to: 10, multiline: true },
  { id: 'trailing-newline', text: 'one\n', from: 0, to: 4, multiline: true },
  {
    id: 'rtl-newline',
    text: 'אבג דהו\nזחט יכל',
    from: 1,
    to: 12,
    multiline: true,
  },
  {
    id: 'rtl-paragraphs',
    text: 'אבג דהו\nזחט יכל',
    from: 0,
    to: 15,
    multiline: true,
  },
  {
    id: 'rtl-trailing-newline',
    text: 'אבג דהו\n',
    from: 0,
    to: 8,
    multiline: true,
  },
  {
    id: 'bidi-paragraphs',
    text: 'abc אבג def\none דהו xyz',
    from: 2,
    to: 18,
    multiline: true,
  },
  {
    id: 'wrapped-ltr',
    text: 'one two three four five six seven eight nine ten eleven twelve '.repeat(
      2
    ),
    from: 2,
    to: 90,
    multiline: true,
  },
  {
    id: 'soft-end-before',
    text: 'one two three four five six seven eight nine ten eleven twelve '.repeat(
      2
    ),
    from: 2,
    to: 48,
    multiline: true,
  },
  {
    id: 'soft-end-at',
    text: 'one two three four five six seven eight nine ten eleven twelve '.repeat(
      2
    ),
    from: 2,
    to: 49,
    multiline: true,
  },
  {
    id: 'soft-end-after',
    text: 'one two three four five six seven eight nine ten eleven twelve '.repeat(
      2
    ),
    from: 2,
    to: 50,
    multiline: true,
  },
  {
    id: 'soft-start-at',
    text: 'one two three four five six seven eight nine ten eleven twelve '.repeat(
      2
    ),
    from: 49,
    to: 90,
    multiline: true,
  },
  {
    id: 'wrapped-three-rows',
    text: 'one two three four five six seven eight nine ten eleven twelve '.repeat(
      2
    ),
    from: 2,
    to: 124,
    multiline: true,
  },
  {
    id: 'soft-last-newline',
    text:
      'one two three four five six seven eight nine ten eleven twelve '.repeat(
        2
      ) + '\nend',
    from: 2,
    to: 128,
    multiline: true,
  },
  {
    id: 'wrapped-bidi',
    text: 'one אבג three דהו four זחט five יכל six מנע seven. '.repeat(2),
    from: 2,
    to: 80,
    multiline: true,
  },
  {
    id: 'wrapped-rtl',
    text: 'אבג דהו זחט יכל abc מנע פצק רשת more אבג דהו end '.repeat(2),
    from: 2,
    to: 80,
    multiline: true,
  },
  {
    id: 'wrapped-pure-rtl',
    text: 'בראשית ברא אלוהים את השמים ואת הארץ והארץ היתה תוהו ובוהו '.repeat(
      2
    ),
    from: 2,
    to: 80,
    multiline: true,
  },
  {
    id: 'wrapped-rtl-run',
    text: 'abc ' + 'אבגדהוזחט '.repeat(10) + 'xyz',
    from: 2,
    to: 80,
    multiline: true,
  },
  {
    id: 'wrapped-rtl-run-only',
    text: 'abc ' + 'אבגדהוזחט '.repeat(10) + 'xyz',
    from: 6,
    to: 80,
    multiline: true,
  },
  {
    id: 'rtl-soft-space',
    text: 'abc ' + 'אבגדהוזחט '.repeat(10) + 'xyz',
    from: 42,
    to: 46,
    multiline: true,
  },
  {
    id: 'wrapped-arabic',
    text: 'مرحبا بالعالم هذا نص عربي طويل لاختبار التفاف الكلمات والاختيار '.repeat(
      2
    ),
    from: 2,
    to: 80,
    multiline: true,
  },
  {
    id: 'wrapped-bidi-embedding',
    text: 'abc \u202b' + 'אבג 123 דהו xyz '.repeat(7) + '\u202c end',
    from: 2,
    to: 80,
    multiline: true,
  },
  {
    id: 'wrapped-arabic-word',
    text: 'لاختبارالتفافالكلمات'.repeat(5),
    from: 2,
    to: 80,
    multiline: true,
  },
  {
    id: 'rtl-hanging-control',
    text: 'abc \u202b' + 'אבגדהוזחט '.repeat(4) + ' '.repeat(10) + '\u202c',
    from: 2,
    to: 56,
    multiline: true,
  },

  { id: 'bidi-control-only', text: 'abc \u202bאבג\u202c xyz', from: 4, to: 5 },
  { id: 'bidi-pop-only', text: 'abc \u202bאבג\u202c xyz', from: 8, to: 9 },
  ...[
    ['embedding', '\u202b', '\u202c'],
    ['isolate', '\u2067', '\u2069'],
  ].flatMap(([kind, open, close]) =>
    [3, 4, 5, 6, 7, 8, 9].map(from => ({
      id: `bidi-${kind}-char-${from}`,
      text: `abc ${open}אבג${close} xyz`,
      from,
      to: from + 1,
    }))
  ),
  ...[
    ['hebrew-marks', 'abc \u202bאבג\u05b0\u202c xyz'],
    ['arabic-marks', 'abc \u202bمَرْحَبًا\u202c xyz'],
    ['arabic-ligature', 'abc \u2067لا\u2069 xyz'],
    ['override', 'abc \u202eAV\u202c xyz'],
  ].map(([id, text]) => {
    const closing = text.search(/[\u202c\u2069]/)
    const segments = [
      ...new Intl.Segmenter('und', { granularity: 'grapheme' }).segment(
        text.slice(0, closing)
      ),
    ]
    return {
      id: `bidi-terminal-${id}`,
      text,
      from: segments.length - 1,
      to: segments.length,
    }
  }),
].filter(c => !args.cases || args.cases.split(',').includes(c.id))
assert.ok(cases.length, 'no matching selection cases')
assert.ok(
  !pixels || process.platform === 'darwin',
  'native pixel capture currently requires macOS'
)
await mkdir(out, { recursive: true })
const server = await createServer({
  root: example,
  cacheDir: 'node_modules/.vite-native-field-selection',
  logLevel: 'silent',
  server: { host: '127.0.0.1', port: 0 },
})
await server.listen()
const browser = await chromium.launch({
  executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE || undefined,
})
const readings = [],
  failures = []
async function selectionMask(src) {
  const image = new Image()
  image.src = src
  await image.decode()
  const canvas = document.createElement('canvas')
  canvas.width = image.width
  canvas.height = image.height
  const context = canvas.getContext('2d')
  context.drawImage(image, 0, 0)
  const data = context.getImageData(0, 0, canvas.width, canvas.height).data,
    rows = [],
    partial = []
  for (let y = 0; y < canvas.height; y++) {
    const spans = []
    let start
    for (let x = 0; x <= canvas.width; x++) {
      const i = (y * canvas.width + x) * 4,
        yes =
          x < canvas.width &&
          data[i] > 240 &&
          data[i + 1] < 20 &&
          data[i + 2] > 240
      if (
        x < canvas.width &&
        data[i] > data[i + 1] + 80 &&
        data[i + 2] > data[i + 1] + 80 &&
        !(data[i] === 255 && data[i + 1] === 0 && data[i + 2] === 255)
      ) {
        partial.push(y * canvas.width + x)
      }
      if (yes && start === undefined) start = x
      if (!yes && start !== undefined) {
        spans.push([start, x])
        start = undefined
      }
    }
    rows.push(spans)
  }
  return { width: canvas.width, height: canvas.height, rows, partial }
}
let page, cdp, scale
try {
  for (const [index, c] of cases.entries()) {
    const segments = [
      ...new Intl.Segmenter('en', { granularity: 'grapheme' }).segment(c.text),
    ].map(s => s.segment)
    const range = [
      segments.slice(0, c.from).join('').length,
      segments.slice(0, c.to).join('').length,
    ]
    const scene = c.multiline ? 'text-field/2' : 'text-field/preview',
      nativeDir = join(out, `native-${index}`)
    const keys = [
      process.platform === 'darwin' ? 'cmd-up' : 'ctrl-home',
      ...Array(c.from).fill('right'),
      ...Array(c.to - c.from).fill('shift-right'),
    ]
    console.log(`native selection ${index + 1}/${cases.length}: ${c.id}`)
    const result = await execFile(
      resolve(args.binary),
      [
        '--conformance',
        '--out',
        nativeDir,
        '--contract',
        join(repo, 'tools/conformance/scenes.json'),
        '--themes',
        args.themes,
        '--scenes',
        scene,
        ...(pixels ? ['--caret-selection-probe'] : ['--no-images']),
        ...(args['window-capture'] ? ['--window-capture'] : []),
        '--caret-focus',
        '0',
        '--caret-input',
        c.text,
        '--caret-batch-keys',
        '--caret-keys',
        keys.join(','),
      ],
      { maxBuffer: 1024 * 1024 }
    )
    await writeFile(
      join(out, `native-${index}.log`),
      result.stdout + result.stderr
    )
    const native = JSON.parse(
      await readFile(join(nativeDir, 'carets.json'), 'utf8')
    )
    if (page) assert.equal(scale, native.scaleFactor, 'native scale changed')
    else {
      scale = native.scaleFactor
      page = await browser.newPage({ deviceScaleFactor: scale })
      cdp = await page.context().newCDPSession(page)
      await page.goto(
        new URL('conformance.html', server.resolvedUrls.local[0]).href
      )
      await page.waitForFunction(() => window.conformance)
      await page.addStyleTag({
        content:
          '#root input::selection,#root textarea::selection{background:#ff00ff!important;color:transparent!important;text-shadow:none!important}',
      })
    }
    for (const [theme, traces] of Object.entries(native.themes)) {
      await page.evaluate(
        ({ scene, theme }) => window.conformance.show(scene, theme),
        { scene, theme }
      )
      const trace = traces[`${scene}/0`],
        errors = []
      assert.ok(
        trace?.surface,
        `${c.id}/${theme}: missing native selection trace`
      )
      const input = page.locator('#root .dz-text-field').first()
      await input.focus()
      const font = await input.evaluate((node, frame) => {
        Object.assign(node.style, {
          position: 'fixed',
          left: frame.left + 'px',
          top: frame.top + 'px',
          width: frame.width + 'px',
          height: frame.height + 'px',
          margin: '0',
          color: 'transparent',
          caretColor: 'transparent',
        })
        node.setAttribute('data-selection-probe', '')
        const style = getComputedStyle(node)
        return {
          family: style.fontFamily,
          size: style.fontSize,
          lineHeight: style.lineHeight,
          frame: node.getBoundingClientRect().toJSON(),
        }
      }, trace.surface)
      await input.fill(c.text)
      await input.evaluate(
        (node, range) => node.setSelectionRange(...range),
        range
      )
      const actualRange = await input.evaluate(node => [
        node.selectionStart,
        node.selectionEnd,
      ])
      const png = await input.screenshot({ caret: 'hide' })
      const imageDir = join(out, 'browser', theme)
      await mkdir(imageDir, { recursive: true })
      await writeFile(join(imageDir, c.id + '.png'), png)
      const reference = await page.evaluate(
        selectionMask,
        'data:image/png;base64,' + png.toString('base64')
      )
      // Resolve the real UA editor through DevTools. These are the native
      // control's layout rectangles, not a separately styled text mirror.
      const { root } = await cdp.send('DOM.getDocument', {
        depth: -1,
        pierce: true,
      })
      let editor
      const find = node => {
        if ((node.attributes || []).includes('data-selection-probe')) {
          const editors = []
          const visit = n => {
            if (
              n.nodeName === 'DIV' &&
              !JSON.stringify(n.attributes || []).includes('placeholder') &&
              n.children?.some(n => n.nodeType === 3 || n.nodeName === 'BR')
            )
              editors.push(n)
            for (const child of n.children || []) visit(child)
          }
          for (const shadow of node.shadowRoots || []) visit(shadow)
          editor = editors.at(-1)
        }
        for (const child of [
          ...(node.children || []),
          ...(node.shadowRoots || []),
        ])
          find(child)
      }
      find(root)
      let browserLayout
      assert.ok(editor, `${c.id}/${theme}: missing UA editor`)
      if (editor) {
        const { object } = await cdp.send('DOM.resolveNode', {
          nodeId: editor.nodeId,
        })
        const response = await cdp.send('Runtime.callFunctionOn', {
          objectId: object.objectId,
          returnByValue: true,
          arguments: range.map(value => ({ value })),
          functionDeclaration: `function(start,end){
      const leaves=[];const visit=n=>{if(n.nodeType===3||n.nodeName==='BR')leaves.push(n);else for(const child of n.childNodes)visit(child)};visit(this);
      const position=offset=>{let base=0;for(const node of leaves){const length=node.nodeType===3?node.length:node.id==='textarea-placeholder-break'?0:1;if(offset<=base+length){if(node.nodeType===3)return[node,Math.max(0,offset-base)];const index=[...node.parentNode.childNodes].indexOf(node);return[node.parentNode,index+(offset>base?1:0)]}base+=length}return[this,this.childNodes.length]};
      const a=position(start),b=position(end),range=document.createRange();range.setStart(...a);range.setEnd(...b);return{value:leaves.map(node=>node.nodeType===3?node.data:node.id==='textarea-placeholder-break'?'':'\\n').join(''),editor:this.getBoundingClientRect().toJSON(),rects:[...range.getClientRects()].map(r=>r.toJSON())};}
    `,
        })
        assert.ok(
          !response.exceptionDetails,
          JSON.stringify(response.exceptionDetails)
        )
        browserLayout = response.result.value
        assert.equal(browserLayout.value, c.text, 'UA editor text differs')
        assert.ok(browserLayout.editor.width > 0, 'UA editor is not visible')
        if (c.id.startsWith('wrapped'))
          assert.ok(
            new Set(browserLayout.rects.map(r => r.y)).size > 1,
            'fixture did not span a soft wrap'
          )
      }
      const nativeRects = (trace.selections || [])
        .map(q => {
          const l = Math.max(q.bounds.left, q.clip.left),
            t = Math.max(q.bounds.top, q.clip.top),
            r = Math.min(
              q.bounds.left + q.bounds.width,
              q.clip.left + q.clip.width
            ),
            b = Math.min(
              q.bounds.top + q.bounds.height,
              q.clip.top + q.clip.height
            )
          return {
            left: (l - Math.floor(trace.surface.left)) * scale,
            top: (t - Math.floor(trace.surface.top)) * scale,
            right: (r - Math.floor(trace.surface.left)) * scale,
            bottom: (b - Math.floor(trace.surface.top)) * scale,
          }
        })
        .filter(r => r.right > r.left && r.bottom > r.top)
      const rendered = pixels
        ? await page.evaluate(
            selectionMask,
            'data:image/png;base64,' +
              (
                await readFile(
                  join(
                    nativeDir,
                    'images',
                    theme,
                    scene.replaceAll('/', '-') + '-0.png'
                  )
                )
              ).toString('base64')
          )
        : null
      if (rendered) {
        assert.equal(
          rendered.width,
          reference.width,
          'native screenshot width differs'
        )
        assert.equal(
          rendered.height,
          reference.height,
          'native screenshot height differs'
        )
      }
      const nativePartial = new Set(rendered?.partial),
        referencePartial = new Set(reference.partial)
      const selectedPixel = (mask, x, y) =>
        x >= 0 &&
        x < mask.width &&
        y >= 0 &&
        y < mask.height &&
        mask.rows[y].some(([left, right]) => left <= x && x < right)
      const edgePixel = (mask, partial, x, y) =>
        partial.has(y * mask.width + x) &&
        [
          [x - 1, y],
          [x + 1, y],
          [x, y - 1],
          [x, y + 1],
        ].some(
          ([a, b]) =>
            selectedPixel(mask, a, b) && !partial.has(b * mask.width + a)
        )
      let antialiasDifferences = 0
      let mismatched = 0
      const differentRows = []
      for (let y = 0; y < reference.height; y++) {
        let count = 0
        for (let x = 0; x < reference.width; x++) {
          const expected = reference.rows[y].some(([l, r]) => l <= x && x < r),
            actual = rendered
              ? rendered.rows[y].some(([l, r]) => l <= x && x < r)
              : nativeRects.some(
                  r => r.left <= x && x < r.right && r.top <= y && y < r.bottom
                )
          if (expected !== actual) {
            // A threshold can classify the same partially covered curve pixel
            // differently in two rasterizers. Exclude only paired AA pixels
            // touching opaque selection in both captures; flat snapped edges
            // and opaque-versus-empty mismatches remain strict.
            if (
              rendered &&
              edgePixel(rendered, nativePartial, x, y) &&
              edgePixel(reference, referencePartial, x, y)
            )
              antialiasDifferences++
            else count++
          }
        }
        mismatched += count
        if (count)
          differentRows.push({ y, pixels: count, expected: reference.rows[y] })
      }
      if (mismatched) errors.push(`${mismatched} selection pixels differ`)
      if (JSON.stringify(trace.input?.selection) !== JSON.stringify(range))
        errors.push('native source selection differs')
      if (JSON.stringify(actualRange) !== JSON.stringify(range))
        errors.push('browser source selection differs')
      if (trace.input?.value !== c.text) errors.push('native value differs')
      const reading = {
        case: c.id,
        theme,
        range,
        font,
        input: trace.input,
        native: trace.selections,
        nativeRects,
        rendered,
        browserLayout,
        reference,
        differentRows,
        antialiasDifferences,
        errors,
      }
      readings.push(reading)
      if (errors.length) failures.push(reading)
    }
  }
} finally {
  await browser.close()
  await server.close()
}
await writeFile(
  join(out, 'report.json'),
  JSON.stringify(
    {
      scale,
      measurement: pixels ? 'native-pixels' : 'submitted-quads',
      capture: pixels ? (args['window-capture'] ? 'window' : 'renderer') : null,
      cases: readings.length,
      failures,
      readings,
    },
    null,
    2
  ) + '\n'
)
console.log(
  `${readings.length} native/browser selection comparisons; ${failures.length} failures — ${out}`
)
if (failures.length) {
  console.error(
    JSON.stringify(
      failures.map(x => ({ case: x.case, theme: x.theme, errors: x.errors })),
      null,
      2
    )
  )
  process.exitCode = 1
}
