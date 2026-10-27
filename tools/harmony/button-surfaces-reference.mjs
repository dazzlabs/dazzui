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
    'packages/arkts/dazzui/example/entry/build/verification/button-surfaces'
)
const native = JSON.parse(
  fs.readFileSync(path.join(directory, 'native-button-surfaces.json'), 'utf8')
)
assert.equal(
  native.length,
  126,
  'Capture all six variants, four sizes and three palettes'
)
assert.equal(
  new Set(
    native.map(row => [row.theme, row.size, row.variant, row.state].join('/'))
  ).size,
  126
)
const fixture = path.join(example, '.build/harmony-button-surfaces')
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
const variants=['normal','recessed','filled','tinted','outlined','plain'],sizes=['tiny','small','medium','large']
createRoot(document.getElementById('root')).render(h('div',null,...sizes.flatMap(size=>variants.map(variant=>h(Button,{id:'surface-'+size+'-'+variant,variant,size,key:size+variant},variant)))))
`
)
const server = await createServer({
  root: example,
  logLevel: 'warn',
  server: { host: '127.0.0.1', port: 0 },
})
await server.listen()
const reports = []
let browser
try {
  browser = await chromium.launch({
    executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE || undefined,
  })
  for (const theme of ['studio-light', 'studio-dark', 'bright-light']) {
    const rows = native.filter(row => row.theme === theme),
      density = rows[0].native.$geometry.density
    assert.equal(rows.length, 42)
    const page = await browser.newPage({
      viewport: { width: 1200, height: 800 },
      deviceScaleFactor: density,
    })
    const errors = []
    page.on('pageerror', error => errors.push(error.message))
    await page.goto(
      new URL(
        '.build/harmony-button-surfaces/index.html',
        server.resolvedUrls.local[0]
      ).href
    )
    await page.locator('#surface-small-normal').waitFor()
    await page.evaluate(
      ({ theme, rows, density }) => {
        document.documentElement.dataset.theme = theme
        document.body.style.margin = '0'
        document.body.style.background = 'var(--color-surface)'
        for (const row of rows.filter(row => row.state === 'normal')) {
          const f = row.native.$geometry.frame,
            node = document.getElementById(
              'surface-' + row.size + '-' + row.variant
            )
          Object.assign(node.style, {
            position: 'absolute',
            left: f.x / density + 'px',
            top: f.y / density + 'px',
            width: f.width / density + 'px',
            height: f.height / density + 'px',
          })
        }
      },
      { theme, rows, density }
    )
    let imageName = '',
      actualImage
    for (const row of rows) {
      const id = 'surface-' + row.size + '-' + row.variant,
        target = page.locator('#' + id),
        f = row.native.$geometry.frame
      await target.evaluate(
        (node, { frame, density }) => {
          Object.assign(node.style, {
            left: frame.x / density + 'px',
            top: frame.y / density + 'px',
            width: frame.width / density + 'px',
            height: frame.height / density + 'px',
          })
        },
        { frame: f, density }
      )
      if (row.state === 'disabled')
        await page
          .locator('button')
          .evaluateAll(nodes => nodes.forEach(node => (node.disabled = true)))
      if (row.state === 'normal' || row.state === 'disabled')
        await page.mouse.move(20, 20)
      else await target.hover()
      if (row.state === 'pressed') await page.mouse.down()
      await page.waitForTimeout(220)
      await target.evaluate(async node => {
        await Promise.all(
          node
            .getAnimations()
            .filter(
              animation => animation.effect?.getTiming().iterations !== Infinity
            )
            .map(animation => animation.finished.catch(() => {}))
        )
      })
      const name = [row.theme, row.size, row.variant, row.state].join('-'),
        file = path.join(directory, 'react-surface-' + name + '.png')
      await page.screenshot({ path: file })
      const expectedImage = readPNG(fs.readFileSync(file))
      if (imageName !== row.image) {
        imageName = row.image
        actualImage = readPNG(fs.readFileSync(path.join(directory, row.image)))
      }
      const style = await target.evaluate(node => {
        const s = getComputedStyle(node)
        return {
          background: s.backgroundColor,
          border: s.borderColor,
          edgeWidth: s.borderWidth,
          radius: s.borderRadius,
          shadow: s.boxShadow,
          active: node.matches(':active'),
          disabled: node.disabled,
          focusVisible: node.matches(':focus-visible'),
        }
      })
      assert.equal(style.disabled, row.state === 'disabled')
      assert.equal(style.active, row.state === 'pressed')
      assert.equal(
        style.focusVisible,
        false,
        'Pointer surface specimen has no keyboard outline'
      )
      const outside = Math.ceil(8 * density),
        inside = Math.ceil(3 * density),
        cx = Math.floor(f.x + f.width / 2),
        cy = Math.floor(f.y + f.height / 2),
        points = []
      for (let x = Math.floor(f.x) - outside; x < Math.ceil(f.x) + inside; x++)
        points.push([x, cy])
      for (
        let x = Math.floor(f.x + f.width) - inside;
        x < Math.ceil(f.x + f.width) + outside;
        x++
      )
        points.push([x, cy])
      for (let y = Math.floor(f.y) - outside; y < Math.ceil(f.y) + inside; y++)
        points.push([cx, y])
      for (
        let y = Math.floor(f.y + f.height) - inside;
        y < Math.ceil(f.y + f.height) + outside;
        y++
      )
        points.push([cx, y])
      let maximum = 0,
        squared = 0
      const samples = points.map(([x, y]) => {
        const a = actualImage.pixel(x, y).slice(0, 3),
          b = expectedImage.pixel(x, y).slice(0, 3),
          error = Math.max(...a.map((n, i) => Math.abs(n - b[i])))
        maximum = Math.max(maximum, error)
        squared += a.reduce((s, n, i) => s + (n - b[i]) ** 2, 0)
        return { x, y, actual: a, expected: b, error }
      })
      const rms = Math.sqrt(squared / (points.length * 3)),
        passed = maximum <= 2 && rms <= 1
      reports.push({
        name,
        passed,
        maximum,
        rms,
        style,
        paint: row.native.$paint,
        frame: f,
        samples,
      })
      console.log(JSON.stringify({ name, passed, maximum, rms }))
      if (row.state === 'pressed') await page.mouse.up()
    }
    assert.deepEqual(errors, [])
    await page.close()
  }
  fs.writeFileSync(
    path.join(directory, 'surface-reference-results.json'),
    JSON.stringify(reports, null, 2)
  )
  console.log(
    'Native button surface alignment: ' +
      reports.filter(row => row.passed).length +
      '/' +
      reports.length
  )
  assert.ok(
    reports.every(row => row.passed),
    'Button surface edge profiles differ; see surface-reference-results.json'
  )
} finally {
  await browser?.close()
  await server.close()
}
