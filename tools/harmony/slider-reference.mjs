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
    'packages/arkts/dazzui/example/entry/build/verification/slider'
)
const files = fs
  .readdirSync(directory)
  .filter(name => /^native-slider-.*\.json$/.test(name))
assert.ok(files.length, 'Capture the native Slider first')
const fixture = path.join(example, '.build/harmony-slider')
fs.mkdirSync(fixture, { recursive: true })
fs.writeFileSync(
  path.join(fixture, 'index.html'),
  '<!doctype html><html><body><div id="root"></div><script type="module" src="./reference.mjs"></script></body></html>'
)
fs.writeFileSync(
  path.join(fixture, 'reference.mjs'),
  `
import { createElement as h } from 'react'
import { flushSync } from 'react-dom'
import { createRoot } from 'react-dom/client'
import { Slider } from '../../../src/components/slider'
import '../../../src/generated/style.css'
import '../../../src/styles/components.css'
import '../../src/styles/base.css'
import { applyTheme, parseTheme } from '../../src/lib/themes'
const root = createRoot(document.getElementById('root'))
document.head.insertAdjacentHTML('beforeend','<style>body{margin:0;background:var(--color-surface)}</style>')
const rect = node => { const r=node.getBoundingClientRect(); return {x:r.x,y:r.y,width:r.width,height:r.height} }
const rgba = value => {
  const rgb=/^rgba?\\(([^)]+)\\)$/.exec(value), srgb=/^color\\(srgb ([^)]+)\\)$/.exec(value)
  if (!rgb && !srgb) throw new Error('Unsupported CSS color: '+value)
  const p=(rgb??srgb)[1].split(/[\\s,/]+/).filter(Boolean)
  const c=p.slice(0,3).map(n=>Math.round(parseFloat(n)*(n.endsWith('%')?2.55:srgb?255:1)))
  c.push(p.length===4?Math.round(parseFloat(p[3])*(p[3].endsWith('%')?2.55:255)):255)
  return '#'+[c[3],...c.slice(0,3)].map(n=>n.toString(16).padStart(2,'0')).join('').toUpperCase()
}
window.renderSliders = (theme,cases) => {
  applyTheme(document.documentElement,parseTheme(theme))
  flushSync(()=>root.render(h('div',{},...cases.map(c=>h('div',{key:c.key,'data-case':c.key,style:{position:'absolute',left:c.x,top:c.y,width:c.width}},
    h(Slider,{size:c.size,defaultValue:c.value,disabled:c.key==='disabled','aria-label':c.key}))))))
}
window.measureSliders = () => Array.from(document.querySelectorAll('[data-case]')).map(node=>{
  const track=node.querySelector('.dz-slider__track'), s=getComputedStyle(track)
  return {key:node.dataset.case,control:rect(node.querySelector('.dz-slider__control')),track:{...rect(track),color:rgba(s.backgroundColor)},
    indicator:rect(node.querySelector('.dz-slider__indicator')),thumbs:Array.from(node.querySelectorAll('.dz-slider__thumb')).map(thumb=>{
      const t=getComputedStyle(thumb);return {...rect(thumb),fill:rgba(t.backgroundColor),border:rgba(t.borderTopColor),edge:parseFloat(t.borderTopWidth),inputFocusVisible:thumb.querySelector('input')?.matches(':focus-visible')??false,outline:t.outlineStyle,outlineWidth:parseFloat(t.outlineWidth)}
    })}
})
`
)
function rasterDifference(
  actual,
  expected,
  region,
  mask = () => true,
  dx = 0,
  dy = 0
) {
  let count = 0,
    maximum = 0,
    squared = 0
  for (let y = Math.ceil(region[1]); y < Math.floor(region[3]); y++)
    for (let x = Math.ceil(region[0]); x < Math.floor(region[2]); x++) {
      if (!mask(x + 0.5, y + 0.5)) continue
      if (x < 0 || y < 0 || x >= expected.width || y >= expected.height)
        continue
      if (
        x + dx < 0 ||
        y + dy < 0 ||
        x + dx >= actual.width ||
        y + dy >= actual.height
      )
        continue
      const a = actual.pixel(x + dx, y + dy),
        b = expected.pixel(x, y)
      for (let channel = 0; channel < 3; channel++) {
        const error = Math.abs(a[channel] - b[channel])
        maximum = Math.max(maximum, error)
        squared += error * error
        count++
      }
    }
  assert.ok(count, 'A raster region must contain real samples')
  return {
    samples: count / 3,
    maximum,
    rms: Math.sqrt(squared / count),
    passed: maximum <= 2 && Math.sqrt(squared / count) <= 1,
  }
}
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
  for (const file of files) {
    const native = JSON.parse(
      fs.readFileSync(path.join(directory, file), 'utf8')
    )
    const raster = readPNG(
      fs.readFileSync(path.join(directory, file.replace(/\.json$/, '.png')))
    )
    const page = await browser.newPage({
      viewport: {
        width: Math.ceil(raster.width / native.density),
        height: Math.ceil(raster.height / native.density),
      },
      deviceScaleFactor: native.density,
    })
    try {
      await page.goto(
        new URL(
          '.build/harmony-slider/index.html',
          server.resolvedUrls.local[0]
        ).href
      )
      await page.waitForFunction(() => window.renderSliders)
      const cases = native.sliders.map(slider => {
        const size = ['small', 'large'].includes(slider.key)
          ? slider.key
          : 'medium'
        const input = slider.handles[0].native,
          frame = input.$geometry.frame
        const d = parseFloat(input.$attrs.blockSize?.width ?? 0)
        const inset = input.$attrs.style === 'SliderStyle.OutSet' ? d / 2 : 0
        return {
          key: slider.key,
          size,
          value:
            slider.key === 'min'
              ? 0
              : slider.key === 'max'
                ? 100
                : slider.key === 'range'
                  ? [20, 80]
                  : 40,
          x: frame.x / native.density + inset,
          y: frame.y / native.density,
          width: frame.width / native.density - 2 * inset,
        }
      })
      await page.evaluate(
        ({ theme, cases }) => window.renderSliders(theme, cases),
        { theme: native.theme, cases }
      )
      await page.evaluate(() => document.fonts.ready)
      if (native.state === 'focus') await page.keyboard.press('Tab')
      if (native.state === 'hover')
        await page.locator('[data-case="small"] .dz-slider__thumb').hover()
      await page.waitForTimeout(250)
      const expected = await page.evaluate(() => window.measureSliders())
      const firstThumb = expected[0].thumbs[0]
      if (native.state === 'focus') {
        assert.equal(
          firstThumb.inputFocusVisible,
          true,
          'The real range input receives keyboard focus'
        )
        assert.equal(
          firstThumb.outline,
          'solid',
          'The painted wrapper exposes the focused input'
        )
        assert.ok(
          firstThumb.outlineWidth > 0,
          'Keyboard focus has a visible ring'
        )
      } else
        assert.equal(
          firstThumb.outline,
          'none',
          'Pointer/rest states do not acquire a keyboard ring'
        )
      const referenceFile = path.join(
        directory,
        file.replace(/^native-/, 'react-').replace(/\.json$/, '.png')
      )
      await page.screenshot({ path: referenceFile })
      const reference = readPNG(fs.readFileSync(referenceFile))
      const comparisons = []
      for (const slider of native.sliders) {
        const source = expected.find(item => item.key === slider.key)
        const thumbs = []
        for (const [index, thumb] of source.thumbs.entries()) {
          const d = native.density,
            cx = (thumb.x + thumb.width / 2) * d,
            cy = (thumb.y + thumb.height / 2) * d,
            r = (thumb.width * d) / 2,
            edge = thumb.edge * d
          const region = [
            cx - r - 8 * d,
            cy - r - 8 * d,
            cx + r + 8 * d,
            cy + r + 8 * d,
          ]
          const whole = rasterDifference(raster, reference, region)
          const body = rasterDifference(
            raster,
            reference,
            region,
            (x, y) => Math.hypot(x - cx, y - cy) < r - edge - 2
          )
          const shadow = rasterDifference(
            raster,
            reference,
            region,
            (x, y) =>
              Math.hypot(x - cx, y - cy) > r + 2 &&
              Math.abs(y - cy) > (source.track.height * d) / 2 + 1
          )
          // A translated best fit diagnoses a displaced native thumb. It never
          // substitutes for the unshifted raster gate above.
          let fit
          const fitting = [
            cx - r - 2 * d,
            cy - r - 2 * d,
            cx + r + 2 * d,
            cy + r + 2 * d,
          ]
          for (let dy = -2; dy <= 2; dy++)
            for (let dx = -Math.ceil(6 * d); dx <= Math.ceil(6 * d); dx++) {
              const difference = rasterDifference(
                raster,
                reference,
                fitting,
                undefined,
                dx,
                dy
              )
              if (!fit || difference.rms < fit.rms)
                fit = { dx, dy, rms: difference.rms }
            }
          const paint = slider.paints?.[index]
          let geometry, attributes
          if (paint) {
            const frame = paint.$geometry.frame,
              a = paint.$attrs
            geometry = {
              x: frame.x + frame.width / 2 - cx,
              y: frame.y + frame.height / 2 - cy,
              width: frame.width - thumb.width * d,
              height: frame.height - thumb.height * d,
            }
            geometry.passed = Object.values(geometry).every(
              value => Math.abs(value) <= 1
            )
            attributes = {
              fill: a.backgroundColor === thumb.fill,
              border: a.borderColor === thumb.border,
              edge: Math.abs(parseFloat(a.borderWidth) - thumb.edge) <= 0.05,
            }
            attributes.passed = Object.values(attributes).every(Boolean)
          }
          thumbs.push({
            index,
            expected: thumb,
            geometry,
            attributes,
            body,
            shadow,
            whole,
            fit,
            passed:
              (geometry?.passed ?? true) &&
              (attributes?.passed ?? true) &&
              whole.passed,
          })
        }
        const control = source.control,
          d = native.density
        const trackPaint = rasterDifference(raster, reference, [
          control.x * d,
          control.y * d,
          (control.x + control.width) * d,
          (control.y + control.height) * d,
        ])
        comparisons.push({
          key: slider.key,
          expected: source,
          thumbs,
          trackPaint,
          passed: trackPaint.passed && thumbs.every(thumb => thumb.passed),
        })
      }
      reports.push({
        theme: native.theme,
        state: native.state ?? 'rest',
        file,
        sliders: comparisons,
        passed: comparisons.every(slider => slider.passed),
      })
    } finally {
      await page.close()
    }
  }
} finally {
  await browser?.close()
  await server.close()
}
fs.writeFileSync(
  path.join(directory, 'reference-results.json'),
  JSON.stringify(reports, null, 2)
)
for (const report of reports)
  console.log(
    JSON.stringify({
      theme: report.theme,
      state: report.state,
      passed: report.passed,
      sliders: report.sliders.map(slider => ({
        key: slider.key,
        thumbs: slider.thumbs.map(thumb => ({
          geometry: thumb.geometry,
          attributes: thumb.attributes,
          body: thumb.body,
          shadow: thumb.shadow,
          whole: thumb.whole,
          fit: thumb.fit,
        })),
      })),
    })
  )
if (reports.some(report => !report.passed)) process.exitCode = 1
