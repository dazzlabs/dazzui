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
    'packages/arkts/dazzui/example/entry/build/verification/button-focus'
)
const native = JSON.parse(
  fs.readFileSync(path.join(directory, 'native-focus.json'), 'utf8')
)
assert.deepEqual(
  native.map(row => row.theme),
  ['studio-light', 'studio-dark', 'bright-light']
)
const fixture = path.join(example, '.build/harmony-button-focus')
fs.mkdirSync(fixture, { recursive: true })
fs.writeFileSync(
  path.join(fixture, 'index.html'),
  '<!doctype html><html><body><div id="root"></div><script type="module" src="./reference.mjs"></script></body></html>'
)
fs.writeFileSync(
  path.join(fixture, 'reference.mjs'),
  `
import {createElement as h} from 'react'
import {createRoot} from 'react-dom/client'
import {Button} from '../../../src/components/button'
import '../../../src/generated/style.css'
import '../../../src/styles/components.css'
import '../../src/styles/base.css'
createRoot(document.getElementById('root')).render(h('div',null,h('button',{id:'before',style:{position:'absolute',left:600,top:400}},'Before'),h(Button,{id:'target'},'Action')))
`
)
const server = await createServer({
  root: example,
  logLevel: 'warn',
  server: { host: '127.0.0.1', port: 0 },
})
await server.listen()
let browser
const reports = []
try {
  browser = await chromium.launch({
    executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE || undefined,
  })
  for (const row of native) {
    const { density, frame } = row.native.$geometry
    assert.ok(
      Math.abs(frame.x - row.bounds[0]) <= 1 &&
        Math.abs(frame.y - row.bounds[1]) <= 1,
      'Capture uses the maximized window coordinate space'
    )
    const page = await browser.newPage({
      viewport: { width: 1200, height: 800 },
      deviceScaleFactor: density,
      hasTouch: true,
    })
    await page.goto(
      new URL(
        '.build/harmony-button-focus/index.html',
        server.resolvedUrls.local[0]
      ).href
    )
    await page.locator('#target').waitFor()
    await page.evaluate(
      ({ theme, frame, density }) => {
        document.documentElement.dataset.theme = theme
        document.body.style.margin = '0'
        document.body.style.background = 'var(--color-surface)'
        Object.assign(document.getElementById('target').style, {
          position: 'absolute',
          left: frame.x / density + 'px',
          top: frame.y / density + 'px',
          width: frame.width / density + 'px',
          height: frame.height / density + 'px',
        })
      },
      { theme: row.theme, frame, density }
    )
    await page.locator('#before').tap()
    await page.keyboard.press('Tab')
    await page.mouse.move(20, 20)
    await page.waitForTimeout(200)
    const style = await page.locator('#target').evaluate(node => {
      const style = getComputedStyle(node),
        box = node.getBoundingClientRect()
      return {
        visible: node.matches(':focus-visible'),
        width: Number.parseFloat(style.outlineWidth),
        offset: Number.parseFloat(style.outlineOffset),
        color: style.outlineColor,
        frame: { x: box.x, y: box.y, width: box.width, height: box.height },
      }
    })
    assert.equal(style.visible, true)
    const file = path.join(directory, 'react-focus-' + row.theme + '.png')
    await page.screenshot({ path: file })
    const expected = readPNG(fs.readFileSync(file)),
      actual = readPNG(
        fs.readFileSync(
          path.join(directory, 'native-focus-' + row.theme + '.png')
        )
      )
    // Match the native border box explicitly: this diagnostic isolates focus
    // paint, independently of the platform font's different label advances.
    const outset = Math.ceil((style.width + style.offset + 3) * density),
      cx = Math.floor(frame.x + frame.width / 2),
      cy = Math.floor(frame.y + frame.height / 2)
    const points = []
    for (let x = Math.floor(frame.x) - outset; x < Math.ceil(frame.x); x++)
      points.push([x, cy])
    for (
      let x = Math.ceil(frame.x + frame.width);
      x < Math.ceil(frame.x + frame.width) + outset;
      x++
    )
      points.push([x, cy])
    for (let y = Math.floor(frame.y) - outset; y < Math.ceil(frame.y); y++)
      points.push([cx, y])
    for (
      let y = Math.ceil(frame.y + frame.height);
      y < Math.ceil(frame.y + frame.height) + outset;
      y++
    )
      points.push([cx, y])
    let maximum = 0,
      squared = 0
    const samples = points.map(([x, y]) => {
      const a = actual.pixel(x, y).slice(0, 3),
        b = expected.pixel(x, y).slice(0, 3),
        error = Math.max(...a.map((value, index) => Math.abs(value - b[index])))
      maximum = Math.max(maximum, error)
      squared += a.reduce(
        (sum, value, index) => sum + (value - b[index]) ** 2,
        0
      )
      return { x, y, actual: a, expected: b, error }
    })
    const rms = Math.sqrt(squared / (points.length * 3))
    const result = {
      theme: row.theme,
      passed: maximum <= 2 && rms <= 1,
      maximum,
      rms,
      style,
      nativeFrame: frame,
      samples,
    }
    if (row.unfocused) {
      await page.keyboard.press('Shift+Tab')
      await page.waitForTimeout(100)
      const file = path.join(directory, 'react-unfocused-' + row.theme + '.png')
      await page.screenshot({ path: file })
      const nativeBase = readPNG(
        fs.readFileSync(
          path.join(directory, 'native-unfocused-' + row.theme + '.png')
        )
      )
      const sourceBase = readPNG(fs.readFileSync(file))
      result.unfocusedSamples = samples
        .filter(sample => sample.error > 2)
        .map(({ x, y }) => ({
          x,
          y,
          actual: nativeBase.pixel(x, y),
          expected: sourceBase.pixel(x, y),
        }))
      await page.keyboard.press('Tab')
      await page.waitForTimeout(100)
    }
    if (row.theme === 'studio-light') {
      result.sourcePhaseProbe = []
      for (const delta of [0, 0.125, 0.25, 0.375, 0.5, 0.75]) {
        const box = await page.locator('#target').evaluate(
          (node, left) => {
            node.style.left = left + 'px'
            return node.getBoundingClientRect().x
          },
          frame.x / density + delta
        )
        await page.evaluate(
          () =>
            new Promise(resolve =>
              requestAnimationFrame(() => requestAnimationFrame(resolve))
            )
        )
        const file = path.join(directory, 'react-focus-phase-' + delta + '.png')
        await page.screenshot({ path: file })
        const image = readPNG(fs.readFileSync(file)),
          base = image.pixel(0, cy)
        const first = Array.from(
          { length: 3 * outset },
          (_, index) => Math.floor(frame.x) - 2 * outset + index
        ).find(x =>
          image.pixel(x, cy).some((value, index) => value !== base[index])
        )
        result.sourcePhaseProbe.push({
          delta,
          cssLeft: box,
          deviceLeft: box * density,
          firstPaintedPixel: first,
        })
      }
    }
    reports.push(result)
    console.log(
      JSON.stringify({ theme: row.theme, passed: result.passed, maximum, rms })
    )
    await page.close()
  }
  fs.writeFileSync(
    path.join(directory, 'focus-reference-results.json'),
    JSON.stringify(reports, null, 2)
  )
  assert.ok(
    reports.every(row => row.passed),
    'Focus edge profiles differ; see focus-reference-results.json'
  )
} finally {
  await browser?.close()
  await server.close()
}
