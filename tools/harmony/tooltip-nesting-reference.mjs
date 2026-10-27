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
    'packages/arkts/dazzui/example/entry/build/verification/tooltip-nesting'
)
const fixture = path.join(example, '.build/harmony-tooltip-nesting')
fs.mkdirSync(fixture, { recursive: true })
fs.mkdirSync(directory, { recursive: true })
fs.writeFileSync(
  path.join(fixture, 'index.html'),
  '<!doctype html><html><body><div id="root"></div><script type="module" src="./reference.mjs"></script></body></html>'
)
fs.writeFileSync(
  path.join(fixture, 'reference.mjs'),
  `
import {createElement as h,StrictMode,useState} from 'react'
import {createRoot} from 'react-dom/client'
import {Tooltip} from '../../../src/components/tooltip'
import {IconButton} from '../../../src/components/icon-button'
import {Settings20Regular} from '@fluentui/react-icons'
import '../../../src/generated/style.css'
import '../../../src/styles/components.css'
import '../../src/styles/base.css'
function Fixture(){
 const [actions,setActions]=useState(0),[nested,setNested]=useState(true)
 return h('div',{style:{margin:'96px 128px',display:'flex',flexDirection:'column',alignItems:'start',gap:32}},
  h('button',{id:'before'},'Before icon'),
  h(Tooltip,{label:'Custom settings',side:'bottom'},nested
   ?h(IconButton,{id:'target',label:'Settings',onClick:()=>setActions(n=>n+1)},h(Settings20Regular))
   :h('button',{id:'target',onClick:()=>setActions(n=>n+1)},'Raw control')),
  h('span',{id:'actions'},String(actions)),
  h('button',{id:'replace',onClick:()=>setNested(v=>!v)},'Replace control'))
}
createRoot(document.getElementById('root')).render(h(StrictMode,null,h(Fixture)))
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
    viewport: { width: 1000, height: 800 },
    hasTouch: true,
  })
  await page.goto(
    new URL(
      '.build/harmony-tooltip-nesting/index.html',
      server.resolvedUrls.local[0]
    ).href
  )
  await page.locator('#target').waitFor()
  const hints = () => page.locator('.dz-tooltip').allTextContents()
  const check = async (name, expected, actions) => {
    await page.waitForTimeout(100)
    const actual = await hints()
    assert.deepEqual(actual, expected, name)
    assert.equal(
      await page.locator('#actions').textContent(),
      String(actions),
      name + ' actions'
    )
    results.push({ name, tooltips: actual, actions })
  }
  const focus = async () => {
    await page.locator('#before').tap()
    await page.keyboard.press('Tab')
  }
  await page.locator('#target').hover()
  await page.locator('.dz-tooltip').waitFor()
  await check('hover', ['Settings'], 0)
  assert.equal(
    await page
      .locator('.dz-tooltip')
      .evaluate(node => node.closest('[data-side]').dataset.side),
    'top'
  )
  await page.keyboard.press('Escape')
  await page.waitForTimeout(750)
  await check('hover-escape', [], 0)
  await page.mouse.move(20, 20)
  await focus()
  await check('focus', ['Settings'], 0)
  await page.keyboard.press('Enter')
  await page.waitForTimeout(750)
  await check('enter', [], 1)
  await page.locator('#target').tap()
  await check('touch', [], 2)
  await focus()
  await check('refocus', ['Settings'], 2)
  await page.keyboard.press('Escape')
  await check('focus-escape', [], 2)
  await page.locator('#replace').tap()
  await focus()
  await check('unwrapped', ['Custom settings'], 2)
  assert.equal(
    await page
      .locator('.dz-tooltip')
      .evaluate(node => node.closest('[data-side]').dataset.side),
    'bottom'
  )
  await page.locator('#replace').tap()
  await focus()
  await check('rewrapped', ['Settings'], 2)
  await page.keyboard.press('Space')
  await check('space', [], 3)
} finally {
  await browser?.close()
  await server.close()
}
fs.writeFileSync(
  path.join(directory, 'react-tooltip-nesting.json'),
  JSON.stringify(results, null, 2)
)
console.log(
  'React nested tooltip: ' +
    results.length +
    ' interaction/lifecycle checkpoints passed'
)
