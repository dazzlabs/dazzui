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
const pointer = process.argv.includes('--pointer')
const mode = pointer ? 'pointer' : 'keyboard'
const directory = path.resolve(
  process.argv.slice(2).find(value => !value.startsWith('--')) ??
    'packages/arkts/dazzui/example/entry/build/verification/' +
      mode +
      '-actions'
)
const fixture = path.join(example, '.build/harmony-keyboard-actions')
fs.mkdirSync(fixture, { recursive: true })
fs.mkdirSync(directory, { recursive: true })
fs.writeFileSync(
  path.join(fixture, 'index.html'),
  '<!doctype html><html><body><div id="root"></div><script type="module" src="./reference.mjs"></script></body></html>'
)
fs.writeFileSync(
  path.join(fixture, 'reference.mjs'),
  `
import {createElement as h,useState} from 'react'
import {createRoot} from 'react-dom/client'
import {Button} from '../../../src/components/button'
import {IconButton} from '../../../src/components/icon-button'
import {Tabs} from '../../../src/components/tabs'
import {Settings20Regular,Document20Regular} from '@fluentui/react-icons'
import '../../../src/generated/style.css'
import '../../../src/styles/components.css'
import '../../src/styles/base.css'
function Fixture(){
 const [button,setButton]=useState(0),[icon,setIcon]=useState(0),[closed,setClosed]=useState(0)
 window.actions={button,icon,closed}
 return h('div',{style:{margin:64,display:'flex',flexDirection:'column',alignItems:'start',gap:24}},
  h('div',{style:{display:'flex',gap:12}},h(Button,{id:'before'},'Before'),h(Button,{id:'button',onClick:()=>setButton(n=>n+1)},'Action'),h(IconButton,{id:'icon',label:'Settings',onClick:()=>setIcon(n=>n+1)},h(Settings20Regular)),h(Button,{id:'after'},'After')),
  h('div',{style:{width:200,height:36,display:'flex',background:'var(--color-surface-chrome)'}},h(Tabs,{items:['a','b','c','d'].map((value,index)=>({value,label:'Document '+(index+1),icon:Document20Regular,closable:true})),value:'a',onValueChange:()=>{},onClose:()=>setClosed(n=>n+1)})))
}
createRoot(document.getElementById('root')).render(h(Fixture))
`
)
const server = await createServer({
  root: example,
  logLevel: 'warn',
  server: { host: '127.0.0.1', port: 0 },
})
await server.listen()
let browser
const results = []
try {
  browser = await chromium.launch({
    executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE || undefined,
  })
  const page = await browser.newPage({
    viewport: { width: 1200, height: 800 },
    hasTouch: true,
  })
  const pageErrors = []
  page.on('pageerror', error => pageErrors.push(error.message))
  await page.goto(
    new URL(
      '.build/harmony-keyboard-actions/index.html',
      server.resolvedUrls.local[0]
    ).href
  )
  await page.locator('#button').waitFor()
  const targets = {
    button: '#button',
    icon: '#icon',
    close: '[data-tab="a"]+.dz-tabs__close',
    more: '.dz-tabs__more',
  }
  const capture = async (name, kind) => {
    await page.waitForTimeout(180)
    await page.locator(targets[kind]).evaluate(async node => {
      await new Promise(resolve =>
        requestAnimationFrame(() => requestAnimationFrame(resolve))
      )
      await Promise.all(
        node
          .getAnimations()
          .filter(
            animation => animation.effect?.getTiming().iterations !== Infinity
          )
          .map(animation => animation.finished.catch(() => {}))
      )
    })
    const result = await page.evaluate(
      ({ name, selector }) => {
        const button = document.querySelector(selector),
          style = getComputedStyle(button),
          focus = document.activeElement
        return {
          name,
          state: window.actions,
          focused:
            focus?.tagName === 'BUTTON'
              ? [focus.getAttribute('aria-label') ?? focus.textContent.trim()]
              : [],
          menu: !!document.querySelector('[role="menu"]'),
          background: (() => {
            // Read straight sRGB components. Painting translucent ink onto
            // an 8-bit canvas loses its original channels when unpremultiplied.
            const rgb = /^rgba?\(([^)]+)\)$/.exec(style.backgroundColor)
            const srgb = /^color\(srgb\s+([^)]+)\)$/.exec(style.backgroundColor)
            if (!rgb && !srgb)
              throw new Error(
                'Unexpected settled color: ' + style.backgroundColor
              )
            const values = (rgb ?? srgb)[1]
              .split(/[\s,/]+/)
              .filter(Boolean)
              .map(Number)
            const channels = values
              .slice(0, 3)
              .map(value => Math.round(value * (srgb ? 255 : 1)))
            return (
              '#' +
              [Math.round((values[3] ?? 1) * 255), ...channels]
                .map(value => value.toString(16).padStart(2, '0'))
                .join('')
                .toUpperCase()
            )
          })(),
          active: button.matches(':active'),
        }
      },
      { name, selector: targets[kind] }
    )
    if (pointer) {
      const box = await page.locator(targets[kind]).boundingBox()
      const file = path.join(directory, 'react-pointer-' + name + '.png')
      await page.screenshot({ path: file })
      result.surfacePixel = readPNG(fs.readFileSync(file)).pixel(
        Math.round(box.x + 3),
        Math.floor(box.y + box.height / 2)
      )
    }
    results.push(result)
    console.log(JSON.stringify(result))
  }
  const focus = async kind => {
    if (kind === 'close' || kind === 'more') {
      await page.locator('[data-tab="a"]').tap()
      await page.keyboard.press('Tab')
      if (kind === 'more') await page.keyboard.press('Tab')
    } else {
      await page.locator('#before').tap()
      await page.keyboard.press('Tab')
      if (kind === 'icon') await page.keyboard.press('Tab')
    }
    await page.mouse.move(20, 20)
    assert.equal(
      await page
        .locator(targets[kind])
        .evaluate(node => node === document.activeElement),
      true,
      kind + ' focus'
    )
  }
  if (pointer) {
    for (const kind of ['button', 'icon', 'close']) {
      const b = await page.locator(targets[kind]).boundingBox()
      const x = b.x + b.width / 2,
        y = b.y + b.height / 2
      await page.mouse.move(x, y)
      await capture(kind + '-hover', kind)
      await page.mouse.down()
      await capture(kind + '-down', kind)
      await page.mouse.move(x + 2, y)
      await capture(kind + '-move-inside', kind)
      await page.mouse.move(x, y + b.height + 12)
      await capture(kind + '-outside', kind)
      await page.mouse.move(x, y)
      await capture(kind + '-reentered', kind)
      await page.mouse.up()
      await capture(kind + '-up', kind)
      await page.mouse.down()
      await page.mouse.move(x, y + b.height + 12)
      await capture(kind + '-cancel-outside', kind)
      await page.mouse.up()
      await capture(kind + '-cancelled', kind)
      await page.mouse.move(x, y)
      await capture(kind + '-hover-after-cancel', kind)
    }
  } else {
    for (const kind of ['button', 'icon', 'close']) {
      await focus(kind)
      await capture(kind + '-rest', kind)
      await page.keyboard.down('Space')
      await capture(kind + '-space-down', kind)
      await page.keyboard.down('Space')
      await capture(kind + '-space-repeat', kind)
      await page.keyboard.up('Space')
      await capture(kind + '-space-up', kind)
      await page.keyboard.down('Space')
      await page.keyboard.press('Tab')
      await capture(kind + '-space-blur', kind)
      await page.keyboard.up('Space')
      await capture(kind + '-space-cancelled', kind)
      await focus(kind)
      await page.keyboard.down('Enter')
      await capture(kind + '-enter-down', kind)
      await page.keyboard.down('Enter')
      await capture(kind + '-enter-repeat', kind)
      await page.keyboard.up('Enter')
      await capture(kind + '-enter-up', kind)
    }
    await focus('more')
    await capture('more-rest', 'more')
    await page.keyboard.down('Space')
    await capture('more-space-down', 'more')
    await page.keyboard.up('Space')
    await capture('more-space-up', 'more')
    await page.keyboard.press('Escape')
    await capture('more-escape', 'more')
  }
  assert.deepEqual(pageErrors, [], 'The source fixture runs without errors')
  fs.writeFileSync(
    path.join(directory, 'react-' + mode + '-actions.json'),
    JSON.stringify(results, null, 2)
  )
  const file = path.join(directory, 'native-' + mode + '-actions.json')
  if (!process.argv.includes('--reference-only')) {
    const native = JSON.parse(fs.readFileSync(file, 'utf8'))
    assert.equal(
      native.length,
      results.length,
      'Every held-key state was captured'
    )
    assert.equal(new Set(native.map(row => row.name)).size, results.length)
    assert.deepEqual(
      results.at(-1).state,
      pointer
        ? { button: 1, icon: 1, closed: 1 }
        : { button: 3, icon: 3, closed: 3 }
    )
    const reports = results.map(reference => {
      const actual = native.find(row => row.name === reference.name)
      const expected = {
        state: reference.state,
        focused: reference.focused,
        menu: reference.menu,
        background: reference.background,
      }
      const observed = actual && {
        state: actual.state,
        focused: actual.focused,
        menu: actual.menu,
        background: actual.background,
      }
      let error = null
      try {
        assert.deepEqual(observed, expected)
        if (pointer) {
          assert.equal(
            actual.surfacePixel?.length,
            4,
            'Native surface pixel is captured'
          )
          const differences = actual.surfacePixel
            .slice(0, 3)
            .map((value, index) =>
              Math.abs(value - reference.surfacePixel[index])
            )
          assert.ok(
            Math.max(...differences) <= 2,
            'Rendered surface maximum error'
          )
          assert.ok(
            Math.sqrt(
              differences.reduce((sum, value) => sum + value * value, 0) / 3
            ) <= 1,
            'Rendered surface RMS error'
          )
        }
      } catch (failure) {
        error = failure.message
      }
      return {
        name: reference.name,
        passed: !error,
        surfacePixel: pointer
          ? { actual: actual?.surfacePixel, expected: reference.surfacePixel }
          : undefined,
        expected,
        actual: observed,
        error,
      }
    })
    fs.writeFileSync(
      path.join(directory, mode + '-reference-results.json'),
      JSON.stringify(reports, null, 2)
    )
    console.log(
      'Native ' +
        mode +
        ' alignment: ' +
        reports.filter(row => row.passed).length +
        '/' +
        reports.length
    )
    assert.ok(
      reports.every(row => row.passed),
      'Native ' +
        mode +
        ' actions differ; see ' +
        mode +
        '-reference-results.json'
    )
  }
} finally {
  await browser?.close()
  await server.close()
}
