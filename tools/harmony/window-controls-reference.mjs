import assert from 'node:assert/strict'
import fs from 'node:fs'
import path from 'node:path'
import { createRequire } from 'node:module'
import { fileURLToPath, pathToFileURL } from 'node:url'
import { readPNG } from './paint.mjs'

const repo = fileURLToPath(new URL('../../', import.meta.url))
const example = path.join(repo, 'packages/js/dazzui/example')
const require = createRequire(path.join(example, 'package.json'))
const { createServer } = await import(
  pathToFileURL(require.resolve('vite')).href
)
const { chromium } = require('playwright')
const directory = path.resolve(
  process.argv[2] ??
    'packages/arkts/dazzui/example/entry/build/verification/window-controls'
)
const files = fs
  .readdirSync(directory)
  .filter(name => /^native-window-controls-.*\.json$/.test(name))
assert.ok(files.length, 'Capture native window controls first')
const fixture = path.join(example, '.build/harmony-window-controls')
fs.mkdirSync(fixture, { recursive: true })
fs.writeFileSync(
  path.join(fixture, 'index.html'),
  `<!doctype html><html><head><meta charset="utf-8"></head>
<body><div id="root"></div><script type="module" src="./reference.mjs"></script></body></html>`
)
fs.writeFileSync(
  path.join(fixture, 'reference.mjs'),
  `
import { createElement as h } from 'react'
import { flushSync } from 'react-dom'
import { createRoot } from 'react-dom/client'
import { WindowFrame } from '@dazzlabs/dazzui'
import '@dazzlabs/dazzui/style.css'
import '@dazzlabs/dazzui/components.css'
import { applyTheme, parseTheme } from '../../src/lib/themes'
const style = document.createElement('style')
style.textContent = 'body{margin:0;background:var(--color-surface)}#root{position:absolute;left:80px;top:80px}*,*::before,*::after{transition:none!important}'
document.head.append(style)
const root = createRoot(document.getElementById('root'))
const frame = () => new Promise(resolve => requestAnimationFrame(resolve))
window.showWindowControls = async data => {
  applyTheme(document.documentElement, parseTheme(data.theme))
  const state = data.state === 'hidden' ? 'hidden' : data.state === 'disabled' ? 'disabled' : 'enabled'
  flushSync(() => root.render(h(WindowFrame, { key: data.theme + data.platform + data.state,
    title: 'Window chrome', width: 300, height: 120, platform: data.platform,
    inactive: data.state === 'inactive', controls: { close: state, minimize: state, maximize: state } }, 'Window controls')))
  await document.fonts.ready
  await frame(); await frame()
  const fraction = value => ((value % 1) + 1) % 1
  const controls = data.controls.map(native => {
    const button = document.querySelector('[data-control-kind="' + native.kind + '"]')
    const initial = button.getBoundingClientRect()
    // Preserve each toolkit's actual bounds while aligning subpixel phase.
    // Sampling the same raster phase avoids confusing AA with a paint recipe.
    button.style.translate = ((fraction(native.bounds[0]) - fraction(initial.x * data.density)) / data.density) + 'px ' +
      ((fraction(native.bounds[1]) - fraction(initial.y * data.density)) / data.density) + 'px'
    const r = button.getBoundingClientRect()
    return { kind: native.kind, bounds: [r.x * data.density, r.y * data.density,
      (r.x + r.width) * data.density, (r.y + r.height) * data.density],
      shadow: getComputedStyle(button).boxShadow }
  })
  await frame()
  return controls
}
window.removeWindowInset = () => document.querySelector('.dz-window-frame').style.setProperty('--dz-traffic-edge', 'inset 0 0 0 .5px rgb(0 0 0 / .12)')
`
)

const server = await createServer({
  root: example,
  logLevel: 'warn',
  server: { host: '127.0.0.1', port: 0 },
})
await server.listen()
const browser = await chromium.launch({
  executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE || undefined,
})
const reports = []
try {
  for (const name of files) {
    const native = JSON.parse(
      fs.readFileSync(path.join(directory, name), 'utf8')
    )
    const page = await browser.newPage({
      viewport: { width: 500, height: 320 },
      deviceScaleFactor: native.density,
    })
    try {
      await page.goto(
        new URL(
          '.build/harmony-window-controls/index.html',
          server.resolvedUrls.local[0]
        ).href
      )
      await page.waitForFunction(() => window.showWindowControls)
      const controls = await page.evaluate(
        data => window.showWindowControls(data),
        {
          ...native,
          controls: native.controls.map(control => ({
            ...control,
            bounds: control.paintBounds ?? control.bounds,
          })),
        }
      )
      assert.equal(controls.length, native.controls.length)
      if (native.hovered && controls.length) {
        const b = controls[0].bounds
        await page.mouse.move(
          (b[0] + b[2]) / (2 * native.density),
          (b[1] + b[3]) / (2 * native.density)
        )
      }
      const referenceFile = path.join(
        directory,
        name.replace('native-', 'react-').replace('.json', '.png')
      )
      await page.screenshot({ path: referenceFile })
      const reference = readPNG(fs.readFileSync(referenceFile))
      const captured = readPNG(
        fs.readFileSync(path.join(directory, name.replace('.json', '.png')))
      )
      let material
      if (native.hovered && ['enabled', 'inactive'].includes(native.state)) {
        await page.addStyleTag({
          content: '.dz-window-frame__traffic svg{visibility:hidden!important}',
        })
        material = readPNG(await page.screenshot())
      }
      let bare
      if (native.platform === 'macos' && native.controls.length) {
        await page.evaluate(() => window.removeWindowInset())
        bare = readPNG(await page.screenshot())
      }
      const results = native.controls.map((control, index) => {
        const expected = controls[index]
        const [l, t, r, b] = control.paintBounds ?? control.bounds
        if (control.paintBounds)
          assert.ok(
            control.paintBounds.every(
              (value, index) => Math.abs(value - control.bounds[index]) <= 1
            ),
            'Paint remains aligned with its native input control'
          )
        assert.ok(
          Math.abs(r - l - (expected.bounds[2] - expected.bounds[0])) <= 0.1,
          'Control width matches React'
        )
        assert.ok(
          Math.abs(b - t - (expected.bounds[3] - expected.bounds[1])) <= 0.1,
          'Control height matches React'
        )
        const dx = Math.round(expected.bounds[0] - l),
          dy = Math.round(expected.bounds[1] - t)
        assert.ok(
          Math.abs(expected.bounds[0] - l - dx) < 0.04 &&
            Math.abs(expected.bounds[1] - t - dy) < 0.04,
          'Control raster phases match'
        )
        const cx = (l + r) / 2,
          cy = (t + b) / 2,
          radius = (r - l) / 2
        const errors = []
        let signal = 0,
          worst
        for (
          let y = Math.floor(t - 2 * native.density);
          y < b + 2 * native.density;
          y++
        )
          for (
            let x = Math.floor(l - 2 * native.density);
            x < r + 2 * native.density;
            x++
          ) {
            const distance = Math.hypot(x + 0.5 - cx, y + 0.5 - cy)
            // The boundary AA band remains a separate shape-conformance check.
            if (Math.abs(distance - radius) < 0.75) continue
            // The annulus measures the material under real hover, leaving
            // native versus SVG glyph antialiasing to its own comparison.
            if (
              native.hovered &&
              native.state !== 'disabled' &&
              distance < radius - 1.75 * native.density
            )
              continue
            const actual = captured.pixel(x, y),
              want = reference.pixel(x + dx, y + dy)
            const noInset = bare?.pixel(x + dx, y + dy)
            for (let channel = 0; channel < 3; channel++) {
              const error = Math.abs(actual[channel] - want[channel])
              errors.push(error)
              if (!worst || error > worst.error)
                worst = {
                  x,
                  y,
                  channel,
                  actual: actual[channel],
                  expected: want[channel],
                  error,
                }
              if (noInset)
                signal = Math.max(
                  signal,
                  Math.abs(want[channel] - noInset[channel])
                )
            }
          }
        assert.ok(
          errors.length > 300,
          'Enough real control pixels were sampled'
        )
        const maximum = Math.max(...errors)
        const rms = Math.sqrt(
          errors.reduce((sum, n) => sum + n * n, 0) / errors.length
        )
        let glyph
        if (material) {
          // Compare ink centres separately from material pixels. Both native
          // and browser allocation may snap by one physical pixel; a vp/px
          // viewport mismatch instead moves the glyph by several pixels.
          const centre = actual => {
            let mass = 0,
              mx = 0,
              my = 0
            for (let y = Math.floor(t); y < b; y++)
              for (let x = Math.floor(l); x < r; x++) {
                if (Math.hypot(x + 0.5 - cx, y + 0.5 - cy) > radius - 1)
                  continue
                const ground = material.pixel(x + dx, y + dy)
                const ink = actual
                  ? captured.pixel(x, y)
                  : reference.pixel(x + dx, y + dy)
                const weight =
                  ground.reduce(
                    (sum, channel, i) => sum + Math.max(0, channel - ink[i]),
                    0
                  ) / 3
                if (weight <= 8) continue
                mass += weight
                mx += weight * (x + 0.5)
                my += weight * (y + 0.5)
              }
            assert.ok(mass > 0, 'The hovered glyph paints visible ink')
            return { x: mx / mass, y: my / mass, mass }
          }
          const actual = centre(true),
            expected = centre(false)
          glyph = {
            actual,
            expected,
            maxCentreError: Math.max(
              Math.abs(actual.x - expected.x),
              Math.abs(actual.y - expected.y)
            ),
          }
          glyph.passed = glyph.maxCentreError <= 1
        }
        if (
          native.platform === 'macos' &&
          (native.state === 'enabled' ||
            (native.hovered && native.state === 'inactive'))
        )
          assert.ok(
            signal >= 20,
            'The reference distinguishes the white inset from a flat light'
          )
        return {
          kind: control.kind,
          referenceBounds: expected.bounds,
          samples: errors.length,
          maxChannelError: maximum,
          rmsChannelError: rms,
          insetSignal: signal,
          glyph,
          worst,
          passed: maximum <= 2 && rms <= 1 && (glyph?.passed ?? true),
        }
      })
      const report = {
        file: name,
        theme: native.theme,
        platform: native.platform,
        state: native.state,
        hovered: native.hovered ?? false,
        controls: results,
        passed: results.every(result => result.passed),
      }
      reports.push(report)
      console.log(JSON.stringify(report))
    } finally {
      await page.close()
    }
  }
} finally {
  await browser.close()
  await server.close()
}
fs.writeFileSync(
  path.join(directory, 'reference-results.json'),
  JSON.stringify(reports, null, 2)
)
if (reports.some(report => !report.passed)) process.exitCode = 1
