import assert from 'node:assert/strict'
import fs from 'node:fs'
import path from 'node:path'
import { createRequire } from 'node:module'
import { fileURLToPath, pathToFileURL } from 'node:url'

const repo = fileURLToPath(new URL('../../', import.meta.url))
const example = path.join(repo, 'packages/js/dazzui/example')
const require = createRequire(path.join(example, 'package.json'))
const { createServer } = await import(
  pathToFileURL(require.resolve('vite')).href
)
const { chromium } = require('playwright')
const directory = path.resolve(
  process.argv[2] ??
    'packages/arkts/dazzui/example/entry/build/verification/tooltip'
)
const files = fs
  .readdirSync(directory)
  .filter(name => /^native-tooltip-.*\.json$/.test(name))
assert.ok(files.length, 'Collect native Tooltip measurements first')
const fixture = path.join(example, '.build/harmony-tooltip')
fs.mkdirSync(fixture, { recursive: true })
fs.writeFileSync(
  path.join(fixture, 'index.html'),
  '<!doctype html><html><body><div id="root"></div><script type="module" src="./reference.mjs"></script></body></html>'
)
fs.writeFileSync(
  path.join(fixture, 'reference.mjs'),
  `
import { createElement as h, Fragment } from 'react'
import { flushSync } from 'react-dom'
import { createRoot } from 'react-dom/client'
import { Tooltip } from '../../../src/components/tooltip'
import '../../../src/generated/style.css'
import '../../../src/styles/components.css'
import { applyTheme, parseTheme } from '../../src/lib/themes'
const root = createRoot(document.getElementById('root'))
document.head.insertAdjacentHTML('beforeend', '<style>body{margin:0;background:var(--color-surface)}#root{position:absolute;left:400px;top:400px}#target{display:block;width:28px;height:28px;padding:0;border:0}</style>')
const rgba = value => {
  const rgb = /^rgba?\\(([^)]+)\\)$/.exec(value), srgb = /^color\\(srgb ([^)]+)\\)$/.exec(value)
  if (!rgb && !srgb) throw new Error('Unsupported CSS color: ' + value)
  const parts = (rgb ?? srgb)[1].split(/[\\s,/]+/).filter(Boolean)
  const c = parts.slice(0,3).map(n => Math.round(parseFloat(n) * (n.endsWith('%') ? 2.55 : srgb ? 255 : 1)))
  c.push(parts.length === 4 ? Math.round(parseFloat(parts[3]) * (parts[3].endsWith('%') ? 2.55 : 255)) : 255)
  return '#' + [c[3], ...c.slice(0,3)].map(n => n.toString(16).padStart(2,'0')).join('').toUpperCase()
}
window.renderTooltip = (theme, side, label) => {
  applyTheme(document.documentElement, parseTheme(theme))
  const lines = label.split('\\n')
  const content = h(Fragment, {}, ...lines.flatMap((line,index)=> index ? [h('br',{key:index}), line] : [line]))
  flushSync(() => root.render(h(Tooltip, {label:content,side}, h('button', {id:'target','aria-label':'Tooltip trigger'}))))
}
window.measureTooltip = () => {
  const node = document.querySelector('.dz-tooltip'), s = getComputedStyle(node), b = node.getBoundingClientRect()
  const t = document.getElementById('target').getBoundingClientRect(), side = node.closest('[data-side]').dataset.side
  return {side, size:parseFloat(s.fontSize), leading:parseFloat(s.lineHeight), weight:Number(s.fontWeight), color:rgba(s.color), background:rgba(s.backgroundColor), borderColor:rgba(s.borderTopColor), border:parseFloat(s.borderTopWidth), radius:parseFloat(s.borderTopLeftRadius), padding:[s.paddingTop,s.paddingRight,s.paddingBottom,s.paddingLeft].map(parseFloat), height:b.height, gap:side==='top'?t.top-b.bottom:side==='bottom'?b.top-t.bottom:side==='left'?t.left-b.right:b.left-t.right, pointerEvents:s.pointerEvents}
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
      viewport: { width: 1000, height: 900 },
    })
    try {
      await page.goto(
        new URL(
          '.build/harmony-tooltip/index.html',
          server.resolvedUrls.local[0]
        ).href
      )
      await page.waitForFunction(() => window.renderTooltip)
      await page.evaluate(
        ({ theme, side, label }) =>
          window.renderTooltip(theme, side, label ?? 'Helpful label'),
        native
      )
      await page.locator('#target').focus()
      await page.locator('.dz-tooltip').waitFor({ state: 'visible' })
      await page.evaluate(() => document.fonts.ready)
      const expected = await page.evaluate(() => window.measureTooltip())
      const attrs = native.native.$attrs
      const padding =
        typeof attrs.padding === 'string'
          ? JSON.parse(attrs.padding)
          : attrs.padding
      // ArkUI's padding includes the border inset; CSS reports it separately.
      const measured = {
        size: parseFloat(attrs.fontSize),
        leading: parseFloat(attrs.lineHeight),
        weight: Number(attrs.fontWeight),
        color: attrs.fontColor,
        background: attrs.backgroundColor,
        borderColor: attrs.borderColor,
        border: parseFloat(attrs.borderWidth),
        radius: parseFloat(attrs.borderRadius),
        padding: ['top', 'right', 'bottom', 'left'].map(
          side => parseFloat(padding[side]) - parseFloat(attrs.borderWidth)
        ),
        height: (native.bounds[3] - native.bounds[1]) / native.density,
      }
      const fields = Object.fromEntries(
        [
          'size',
          'leading',
          'weight',
          'color',
          'background',
          'borderColor',
          'border',
          'radius',
        ].map(key => [
          key,
          typeof expected[key] === 'number'
            ? Math.abs(measured[key] - expected[key]) <= 0.05
            : measured[key] === expected[key],
        ])
      )
      fields.padding = measured.padding.every(
        (value, index) => Math.abs(value - expected.padding[index]) <= 0.05
      )
      fields.height =
        Math.abs(measured.height - expected.height) <= 1 / native.density
      fields.pointerEvents =
        expected.pointerEvents === 'none' &&
        attrs.touchable === false &&
        attrs.hitTestBehavior === 'HitTestMode.None'
      fields.gap =
        Math.abs(expected.gap - 4) <= 0.05 &&
        Math.abs(native.gap - expected.gap) <= 1 / native.density
      fields.side = expected.side === native.side
      reports.push({
        theme: native.theme,
        side: native.side,
        label: native.label,
        expected,
        measured,
        fields,
        passed: Object.values(fields).every(Boolean),
      })
      await page.screenshot({
        path: path.join(
          directory,
          file.replace(/^native-/, 'react-').replace(/\.json$/, '.png')
        ),
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
for (const report of reports) console.log(JSON.stringify(report))
if (reports.some(report => !report.passed)) process.exitCode = 1
