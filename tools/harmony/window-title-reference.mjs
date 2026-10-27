import assert from 'node:assert/strict'
import fs from 'node:fs'
import path from 'node:path'
import { createRequire } from 'node:module'
import { fileURLToPath, pathToFileURL } from 'node:url'
import { readPNG } from './paint.mjs'
import { checkTextInk } from './text-ink.mjs'

const repo = fileURLToPath(new URL('../../', import.meta.url))
const example = path.join(repo, 'packages/js/dazzui/example')
const require = createRequire(path.join(example, 'package.json'))
const { createServer } = await import(
  pathToFileURL(require.resolve('vite')).href
)
const { chromium } = require('playwright')
const directory = path.resolve(
  process.argv[2] ??
    'packages/arkts/dazzui/example/entry/build/verification/window-titles'
)
const files = fs
  .readdirSync(directory)
  .filter(name => /^native-window-titles-.*\.json$/.test(name))
assert.ok(files.length, 'Measure native window titles first')
const fixture = path.join(example, '.build/harmony-window-titles')
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
import '@dazzlabs/dazzui/style.css'
import '@dazzlabs/dazzui/components.css'
import story from '../../src/stories/react/window-frame'
import { applyTheme, parseTheme } from '../../src/lib/themes'
const root = createRoot(document.getElementById('root'))
const css = document.createElement('style')
css.textContent = 'body{margin:0;background:var(--color-surface)}#root{position:absolute;left:40px;top:40px;width:760px}'
document.head.append(css)
const rgba = value => {
  // Compare submitted font colors, not a premultiplied canvas round trip.
  // Readback changes low-alpha RGB channels even when the CSS ink is exact.
  const rgb = /^rgba?\\(([^)]+)\\)$/.exec(value)
  const srgb = /^color\\(srgb ([^)]+)\\)$/.exec(value)
  if (!rgb && !srgb) throw new Error('Unsupported computed font color: ' + value)
  const parts = (rgb ?? srgb)[1].split(/[\\s,/]+/).filter(Boolean)
  const c = parts.slice(0,3).map(n => Math.round(parseFloat(n) * (srgb || n.endsWith('%') ? n.endsWith('%') ? 2.55 : 255 : 1)))
  c.push(parts.length === 4 ? Math.round(parseFloat(parts[3]) * (parts[3].endsWith('%') ? 2.55 : 255)) : 255)
  return '#' + [c[3], ...c.slice(0,3)].map(n => n.toString(16).padStart(2,'0')).join('').toUpperCase()
}
const text = node => {
  const style = getComputedStyle(node), bounds = node.getBoundingClientRect()
  return { text: node.textContent, size: parseFloat(style.fontSize), leading: parseFloat(style.lineHeight),
    weight: Number(style.fontWeight), color: rgba(style.color), cssColor: style.color, height: bounds.height,
    whitespace: style.whiteSpace, overflow: style.textOverflow }
}
window.measureWindowTitles = async (theme, section) => {
  applyTheme(document.documentElement, parseTheme(theme))
  flushSync(() => root.render(h(story.examples[section].Demo)))
  await document.fonts.ready
  await new Promise(resolve => requestAnimationFrame(resolve))
  return Array.from(document.querySelectorAll('.dz-window-frame')).map((frame, index) => {
    const heading = frame.querySelector('.dz-window-frame__title-text')
    if (!heading) return { index, platform: frame.dataset.platform }
    const leaf = Array.from(heading.querySelectorAll('span')).find(node => node.childElementCount === 0) ?? heading
    const subtitle = frame.querySelector('.dz-window-frame__subtitle')
    const icon = heading.querySelector('.dz-icon')
    const body = frame.querySelector('.dz-window-frame__content span')
    return { index, platform: frame.dataset.platform, title: text(leaf), subtitle: subtitle ? text(subtitle) : undefined,
      background: rgba(getComputedStyle(frame).backgroundColor), body: body ? text(body) : undefined,
      icon: icon ? { color: rgba(getComputedStyle(icon).color), width: parseFloat(getComputedStyle(icon).width), height: parseFloat(getComputedStyle(icon).height) } : undefined }
  })
}
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
  for (const file of files) {
    const native = JSON.parse(
      fs.readFileSync(path.join(directory, file), 'utf8')
    )
    const page = await browser.newPage({
      viewport: { width: 1000, height: 1000 },
    })
    try {
      await page.goto(
        new URL(
          '.build/harmony-window-titles/index.html',
          server.resolvedUrls.local[0]
        ).href
      )
      await page.waitForFunction(() => window.measureWindowTitles)
      const comparisons = []
      for (const section of new Set(
        native.frames.map(frame => frame.section)
      )) {
        const references = await page.evaluate(
          ({ theme, section }) => window.measureWindowTitles(theme, section),
          { theme: native.theme, section }
        )
        for (const frame of native.frames.filter(
          frame => frame.section === section
        )) {
          const expected = references[frame.index]
          assert.equal(frame.platform, expected.platform)
          const compare = (actual, expected) => {
            const attrs = actual.native.$attrs
            const measured = {
              size: parseFloat(attrs.fontSize),
              leading: parseFloat(attrs.lineHeight),
              weight: Number(attrs.fontWeight),
              color: attrs.fontColor,
            }
            const passed =
              actual.text === expected.text &&
              Math.abs(measured.size - expected.size) <= 0.05 &&
              Math.abs(measured.leading - expected.leading) <= 0.05 &&
              measured.weight === expected.weight &&
              measured.color === expected.color
            return { expected, measured, passed }
          }
          let inkPaint
          if (section === 4 && frame.platform === 'windows')
            assert.ok(
              frame.inkPaint,
              'Capture the real title/body ink with the current window-title check'
            )
          if (frame.inkPaint) {
            assert.equal(frame.inkPaint.title.text, expected.title.text)
            assert.equal(frame.inkPaint.body.text, expected.body.text)
            const raster = readPNG(
              fs.readFileSync(path.join(directory, frame.inkPaint.file))
            )
            const title = checkTextInk(
              raster,
              frame.inkPaint.title.bounds,
              expected.title.color,
              expected.background
            )
            const body = checkTextInk(
              raster,
              frame.inkPaint.body.bounds,
              expected.body.color,
              expected.background
            )
            inkPaint = { title, body, passed: title.passed && body.passed }
          }
          comparisons.push({
            section,
            index: frame.index,
            platform: frame.platform,
            title: compare(frame.title, expected.title),
            inkPaint,
            subtitle: frame.subtitle
              ? compare(frame.subtitle, expected.subtitle)
              : undefined,
            icon: frame.icon
              ? (() => {
                  const measured = {
                    color: frame.icon.$attrs.fillColor,
                    width: parseFloat(frame.icon.$attrs.width),
                    height: parseFloat(frame.icon.$attrs.height),
                  }
                  return {
                    expected: expected.icon,
                    measured,
                    passed:
                      measured.color === expected.icon.color &&
                      Math.abs(measured.width - expected.icon.width) <= 0.05 &&
                      Math.abs(measured.height - expected.icon.height) <= 0.05,
                  }
                })()
              : undefined,
          })
        }
        await page.screenshot({
          path: path.join(
            directory,
            'react-window-titles-' + native.theme + '-' + section + '.png'
          ),
          fullPage: true,
        })
      }
      const report = {
        theme: native.theme,
        frames: comparisons,
        passed: comparisons.every(
          frame =>
            frame.title.passed &&
            (frame.subtitle?.passed ?? true) &&
            (frame.icon?.passed ?? true) &&
            (frame.inkPaint?.passed ?? true)
        ),
      }
      reports.push(report)
      console.log(JSON.stringify(report))
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
if (reports.some(report => !report.passed)) process.exitCode = 1
