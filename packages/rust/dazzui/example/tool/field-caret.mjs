/** Compare browser caret pixels with quads submitted by a real GPUI window.
 * Build dazzui-example first, then run with --out <dir> [--themes drawn|all|ids].
 * This deliberately does not use TestAppContext's NoopTextSystem.
 */
import assert from 'node:assert/strict'
import { execFile as execFileCallback } from 'node:child_process'
import { mkdir, readFile, writeFile } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { join, resolve } from 'node:path'
import { fileURLToPath, pathToFileURL } from 'node:url'
import { promisify, parseArgs } from 'node:util'

const repo = fileURLToPath(new URL('../../../../../', import.meta.url))
const { values: args } = parseArgs({
  options: {
    out: {
      type: 'string',
      default: join(tmpdir(), `dazzui-field-caret-${process.pid}`),
    },
    themes: { type: 'string', default: 'drawn' },
    cases: { type: 'string' },
    binary: {
      type: 'string',
      default: join(repo, 'packages/rust/dazzui/target/debug/dazzui-example'),
    },
  },
})
const out = resolve(args.out)
const example = join(repo, 'packages/js/dazzui/example')
const { createServer } = await import(
  pathToFileURL(join(example, 'node_modules/vite/dist/node/index.js'))
)
const { chromium } = await import(
  pathToFileURL(join(example, 'node_modules/playwright/index.mjs'))
)
const execFile = promisify(execFileCallback)
const field = (id, scene, at, input, root, index = 0) => ({
  id,
  scene,
  address: `${scene}/${at}`,
  input,
  root,
  index,
})
const mac = process.platform === 'darwin'
const rowKeys = [
  {
    native: mac ? 'cmd-up' : 'ctrl-home',
    browser: mac ? 'Meta+ArrowUp' : 'Control+Home',
  },
  { native: 'down', browser: 'ArrowDown' },
  {
    native: mac ? 'cmd-right' : 'end',
    browser: mac ? 'Meta+ArrowRight' : 'End',
  },
]
const batches = [
  {
    focus: 0,
    cases: [
      field(
        'text-field/default',
        'text-field/preview',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
      field(
        'search-field/default',
        'search-field/preview',
        0,
        '.dz-search-field__input',
        '.dz-search-field'
      ),
      field(
        'number-field/default',
        'number-field/preview',
        0,
        '.dz-number-field__input',
        '.dz-number-field__group'
      ),
      field(
        'combobox/default',
        'combobox/preview',
        0,
        '.dz-combobox__input',
        '.dz-combobox__group'
      ),
      field(
        'autocomplete/default',
        'combobox/2',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
      field(
        'otp-field/default',
        'otp-field/preview',
        0,
        '.dz-otp-field__cell',
        '.dz-otp-field__cell'
      ),
      field(
        'text-field/small',
        'text-field/0',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
      field(
        'text-field/multiline-empty',
        'text-field/2',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
      field(
        'otp-field/small',
        'otp-field/2',
        0,
        '.dz-otp-field__cell',
        '.dz-otp-field__cell'
      ),
    ],
  },
  {
    focus: 1,
    cases: [
      field(
        'text-field/medium',
        'text-field/0',
        1,
        '.dz-text-field',
        '.dz-text-field',
        1
      ),
      field(
        'text-field/multiline-filled',
        'text-field/2',
        1,
        '.dz-text-field',
        '.dz-text-field',
        1
      ),
    ],
  },
  {
    focus: 2,
    cases: [
      field(
        'text-field/large',
        'text-field/0',
        2,
        '.dz-text-field',
        '.dz-text-field',
        2
      ),
      // The disabled input before the mono field is absent from native Tab order.
      field(
        'text-field/mono',
        'text-field/1',
        3,
        '.dz-text-field',
        '.dz-text-field',
        3
      ),
    ],
  },
  {
    focus: 4,
    cases: [
      field(
        'otp-field/large',
        'otp-field/2',
        1,
        '.dz-otp-field__cell',
        '.dz-otp-field__cell',
        4
      ),
    ],
  },
  {
    focus: 8,
    cases: [
      field(
        'otp-field/error',
        'otp-field/2',
        2,
        '.dz-otp-field__cell',
        '.dz-otp-field__cell',
        8
      ),
    ],
  },
  {
    focus: 0,
    inputText: Array.from({ length: 20 }, (_, i) => `Line ${i + 1}`).join('\n'),
    cases: [
      field(
        'text-field/multiline-overflow',
        'text-field/2',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  },
  {
    focus: 0,
    inputText: 'Line 20',
    cases: [
      field(
        'text-field/filled-end',
        'text-field/preview',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  },
  {
    focus: 0,
    inputText: 'DazzUI 世界 end',
    cases: [
      field(
        'text-field/cjk-end',
        'text-field/preview',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  },
  {
    focus: 0,
    inputText: 'क्षि कर्मि',
    cases: [
      field(
        'text-field/indic-end',
        'text-field/preview',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  },
  {
    focus: 0,
    inputText: 'DazzUI 世界 end '.repeat(12),
    keys: rowKeys,
    cases: [
      field(
        'text-field/cjk-wrapped-end',
        'text-field/2',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  },
  {
    focus: 0,
    inputText: 'Hello 👨‍👩‍👧‍👦 end',
    cases: [
      field(
        'text-field/emoji-end',
        'text-field/preview',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  },
  {
    focus: 2,
    inputText: 'Hello 👨‍👩‍👧‍👦 end',
    cases: [
      field(
        'text-field/mono-emoji-end',
        'text-field/1',
        3,
        '.dz-text-field',
        '.dz-text-field',
        3
      ),
    ],
  },
  ...[false, true].map(home => ({
    focus: 0,
    inputText: 'abc def ghi jkl mno pqr '.repeat(8),
    keys: [
      ...rowKeys,
      ...(home
        ? [
            {
              native: mac ? 'cmd-left' : 'home',
              browser: mac ? 'Meta+ArrowLeft' : 'Home',
            },
          ]
        : []),
    ],
    cases: [
      field(
        `text-field/wrapped-${home ? 'home' : 'end'}`,
        'text-field/2',
        0,
        '.dz-text-field',
        '.dz-text-field'
      ),
    ],
  })),
]
  .map(batch => ({
    ...batch,
    cases: batch.cases.filter(
      item => !args.cases || args.cases.split(',').includes(item.id)
    ),
  }))
  .filter(batch => batch.cases.length)
assert.ok(batches.length, 'no matching caret cases')
await mkdir(out, { recursive: true })
const server = await createServer({
  root: example,
  cacheDir: 'node_modules/.vite-native-field-caret',
  logLevel: 'silent',
  server: { host: '127.0.0.1', port: 0 },
})
await server.listen()
const browser = await chromium.launch({
  executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE || undefined,
})
let page, scale
const readings = [],
  failures = []
try {
  for (const [batchIndex, batch] of batches.entries()) {
    const nativeDir = join(out, `native-${batchIndex}`)
    console.log(
      `native caret batch ${batchIndex + 1}/${batches.length}: focus ${batch.focus}`
    )
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
        [...new Set(batch.cases.map(item => item.scene))].join(','),
        '--no-images',
        '--caret-focus',
        String(batch.focus),
        ...(batch.inputText ? ['--caret-input', batch.inputText] : []),
        ...(batch.keys
          ? ['--caret-keys', batch.keys.map(key => key.native).join(',')]
          : []),
      ],
      { maxBuffer: 1024 * 1024 }
    )
    await writeFile(
      join(out, `native-${batchIndex}.log`),
      result.stdout + result.stderr
    )
    const native = JSON.parse(
      await readFile(join(nativeDir, 'carets.json'), 'utf8')
    )
    if (page)
      assert.equal(
        native.scaleFactor,
        scale,
        'native display scale changed between batches'
      )
    else {
      scale = native.scaleFactor
      page = await browser.newPage({ deviceScaleFactor: scale })
      await page.goto(
        new URL('conformance.html', server.resolvedUrls.local[0]).href
      )
      await page.waitForFunction(() => window.conformance)
    }
    for (const [theme, traces] of Object.entries(native.themes))
      for (const item of batch.cases) {
        await page.evaluate(
          ({ scene, theme }) => window.conformance.show(scene, theme),
          { scene: item.scene, theme }
        )
        const trace = traces[item.address]
        const input = page.locator(item.input).nth(item.index)
        await input.focus()
        const font = await input.evaluate(
          (node, { root, frame }) => {
            document
              .querySelectorAll('[data-caret-crop]')
              .forEach(node => node.removeAttribute('data-caret-crop'))
            const crop = node.closest(root)
            crop.setAttribute('data-caret-crop', '')
            // Compare the caret in the same outer box. The ordinary collector
            // owns box conformance; fractional layout differences can otherwise
            // cross a pixel boundary and change the caret's raster position.
            if (frame) {
              Object.assign(crop.style, {
                position: 'fixed',
                left: `${frame.left}px`,
                top: `${frame.top}px`,
                width: `${frame.width}px`,
                height: `${frame.height}px`,
                margin: '0',
              })
            }
            node.setSelectionRange(0, 0)
            node.style.setProperty('caret-color', '#ff00ff', 'important')
            const style = getComputedStyle(node)
            return {
              family: style.fontFamily,
              size: style.fontSize,
              weight: style.fontWeight,
              lineHeight: style.lineHeight,
              frame: crop.getBoundingClientRect().toJSON(),
            }
          },
          { root: item.root, frame: trace?.surface }
        )
        if (batch.inputText) await input.fill(batch.inputText)
        for (const key of batch.keys || []) await input.press(key.browser)
        font.selection = await input.evaluate(node => [
          node.selectionStart,
          node.selectionEnd,
        ])
        const png = await page
          .locator('[data-caret-crop]')
          .screenshot({ caret: 'initial' })
        const imageDir = join(out, 'browser', theme)
        await mkdir(imageDir, { recursive: true })
        await writeFile(
          join(imageDir, item.id.replaceAll('/', '-') + '.png'),
          png
        )
        const reference = await page.evaluate(
          async ({ data, scale }) => {
            const image = new Image()
            image.src = data
            await image.decode()
            const canvas = document.createElement('canvas')
            canvas.width = image.width
            canvas.height = image.height
            const context = canvas.getContext('2d')
            context.drawImage(image, 0, 0)
            const pixels = context.getImageData(
              0,
              0,
              canvas.width,
              canvas.height
            ).data
            let left = Infinity,
              top = Infinity,
              right = -1,
              bottom = -1
            for (let y = 0; y < canvas.height; y++)
              for (let x = 0; x < canvas.width; x++) {
                const at = (y * canvas.width + x) * 4
                if (
                  pixels[at] > 240 &&
                  pixels[at + 1] < 20 &&
                  pixels[at + 2] > 240
                ) {
                  left = Math.min(left, x)
                  top = Math.min(top, y)
                  right = Math.max(right, x)
                  bottom = Math.max(bottom, y)
                }
              }
            return right < 0
              ? null
              : {
                  left: left / scale,
                  top: top / scale,
                  width: (right - left + 1) / scale,
                  height: (bottom - top + 1) / scale,
                }
          },
          { data: 'data:image/png;base64,' + png.toString('base64'), scale }
        )
        let actual = null
        const errors = []
        if (!reference) errors.push('browser did not paint a caret')
        if (trace?.quads.length !== 1)
          errors.push('native must submit exactly one caret quad')
        else {
          const { bounds, clip } = trace.quads[0],
            surface = trace.surface
          actual = {
            // Playwright encloses the crop in whole CSS pixels.
            left: bounds.left - Math.floor(surface.left),
            top: bounds.top - Math.floor(surface.top),
            width: bounds.width,
            height: bounds.height,
          }
          if (
            ['left', 'top', 'width', 'height'].some(
              key => Math.abs(font.frame[key] - surface[key]) > 0.01
            )
          )
            errors.push('browser did not adopt the native outer box')
          if (
            bounds.left < clip.left ||
            bounds.top < clip.top ||
            bounds.left + bounds.width > clip.left + clip.width ||
            bounds.top + bounds.height > clip.top + clip.height
          )
            errors.push('native caret is clipped')
          if (
            reference &&
            Object.keys(reference).some(
              key => Math.abs(reference[key] - actual[key]) > 0.01
            )
          )
            errors.push('caret geometry differs')
        }
        const reading = {
          theme,
          case: item.id,
          scene: item.scene,
          address: item.address,
          font,
          reference,
          actual,
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
    { scale, cases: readings.length, failures, readings },
    null,
    2
  ) + '\n'
)
console.log(
  `${readings.length} native/browser caret comparisons; ${failures.length} failures — ${out}`
)
if (failures.length) {
  console.error(JSON.stringify(failures.slice(0, 8), null, 2))
  process.exitCode = 1
}
