import assert from 'node:assert/strict'
import fs from 'node:fs'
import path from 'node:path'
import { createRequire } from 'node:module'
import { fileURLToPath, pathToFileURL } from 'node:url'
import { isDeepStrictEqual } from 'node:util'
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
    'packages/arkts/dazzui/example/entry/build/verification/tabs-actions'
)
const native = JSON.parse(
  fs.readFileSync(path.join(directory, 'native-tabs-actions.json'), 'utf8')
)
const fixture = path.join(example, '.build/harmony-tabs-actions')
fs.mkdirSync(fixture, { recursive: true })
fs.writeFileSync(
  path.join(fixture, 'index.html'),
  '<!doctype html><html><body><div id="root"></div><script type="module" src="./reference.mjs"></script></body></html>'
)
fs.writeFileSync(
  path.join(fixture, 'reference.mjs'),
  `
import {createElement as h,useState} from 'react'
import {createRoot} from 'react-dom/client'
import {Tabs} from '../../../src/components/tabs'
import {Button} from '../../../src/components/button'
import {Document20Regular} from '@fluentui/react-icons'
import '../../../src/generated/style.css'
import '../../../src/styles/components.css'
import '../../src/styles/base.css'
function Fixture(){
 const [values,setValues]=useState(['a','b','c','d']),[selected,setSelected]=useState('a'),[width,setWidth]=useState(700),[closed,setClosed]=useState(0),[created,setCreated]=useState(0),[refuse,setRefuse]=useState(false),[closable,setClosable]=useState(true)
 window.tabsState={values,selected,closed,created,width,refuse,closable}
 return h('div',{style:{margin:64,display:'flex',flexDirection:'column',alignItems:'start',gap:24}},
  h('div',{style:{display:'flex',gap:12}},h(Button,{id:'reset',onClick:()=>{setValues(['a','b','c','d']);setSelected('a');setWidth(700);setClosed(0);setCreated(0);setRefuse(false);setClosable(true)}},'Reset'),h(Button,{id:'narrow',onClick:()=>setWidth(260)},'Narrow'),h(Button,{id:'refuse',onClick:()=>setRefuse(v=>!v)},'Refuse close'),h(Button,{id:'toggle-close',onClick:()=>{setClosable(v=>!v);setWidth(200)}},'Toggle close')),
  h('div',{id:'header',style:{width,height:36,display:'flex',background:'var(--color-surface-chrome)'}},
   h(Tabs,{items:values.map(value=>({value,label:'Document '+(value.charCodeAt(0)-96),icon:Document20Regular,closable})),value:selected,onValueChange:setSelected,onClose:value=>{if(!refuse)setValues(items=>items.filter(item=>item!==value));setClosed(n=>n+1)},trailing:h(Button,{id:'new',size:'tiny',onClick:()=>setCreated(n=>n+1)},'New')})))
}
createRoot(document.getElementById('root')).render(h(Fixture))
const rgba=value=>{const m=/^rgba?\\(([^)]+)\\)$/.exec(value);if(!m)throw new Error('Unsupported color: '+value);const parts=m[1].split(/[\\s,/]+/).filter(Boolean);return '#'+[parts.length===4?Math.round(Number(parts[3])*255):255,...parts.slice(0,3).map(Number)].map(n=>n.toString(16).padStart(2,'0')).join('').toUpperCase()}
window.measureTabsActions = kind => {
 const target=kind?document.querySelector(kind==='more'?'.dz-tabs__more':'[data-tab="a"]+.dz-tabs__close'):null
 const style=target&&getComputedStyle(target),glyph=target?.querySelector('svg'),b=target?.getBoundingClientRect(),g=glyph?.getBoundingClientRect(),focused=document.activeElement
 return {state:window.tabsState,selected:[...document.querySelectorAll('[data-tab][aria-selected="true"]')].map(node=>node.dataset.tab),
  tooltips:[...document.querySelectorAll('.dz-tooltip')].map(node=>node.textContent),
  focused:focused?.tagName==='BUTTON'?[focused.getAttribute('aria-label')??focused.textContent.trim()]:[],
  menu:!!document.querySelector('[role="menu"]'),
  action:target?{label:target.getAttribute('aria-label'),background:rgba(style.backgroundColor),ink:rgba(style.color),width:b.width,height:b.height,glyphWidth:g.width,glyphHeight:g.height,backdrop:rgba(getComputedStyle(document.getElementById('header')).backgroundColor)}:null}
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
const composite = (foreground, background) => {
  const argb = value =>
    [1, 3, 5, 7].map(index => parseInt(value.slice(index, index + 2), 16))
  const [alpha, ...ink] = argb(foreground),
    [backgroundAlpha, ...base] = argb(background)
  assert.equal(backgroundAlpha, 255)
  return (
    '#FF' +
    ink
      .map((channel, index) =>
        Math.round(base[index] + ((channel - base[index]) * alpha) / 255)
          .toString(16)
          .padStart(2, '0')
      )
      .join('')
      .toUpperCase()
  )
}
try {
  browser = await chromium.launch({
    executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE || undefined,
  })
  const page = await browser.newPage({
    viewport: { width: 1200, height: 800 },
    hasTouch: true,
  })
  await page.goto(
    new URL(
      '.build/harmony-tabs-actions/index.html',
      server.resolvedUrls.local[0]
    ).href
  )
  await page.locator('[data-tab="a"]').waitFor()
  const close = () =>
    page.getByRole('button', { name: 'Close Document 1', exact: true })
  const more = () => page.locator('.dz-tabs__more')
  const capture = async (name, kind) => {
    await page.waitForTimeout(180)
    // Native inspection happens after several device round trips. Compare the
    // browser's settled colours too, rather than a compatibility-mouse fade
    // that can begin after a touch opens the menu.
    await page.evaluate(
      () =>
        new Promise(resolve =>
          requestAnimationFrame(() => requestAnimationFrame(resolve))
        )
    )
    await page.evaluate(() =>
      Promise.all(
        [...document.querySelectorAll('.dz-tabs button')]
          .flatMap(node => node.getAnimations())
          .filter(
            animation =>
              animation.effect?.getComputedTiming().iterations !== Infinity
          )
          .map(animation => animation.finished.catch(() => {}))
      )
    )
    const expected = await page.evaluate(
        kind => window.measureTabsActions(kind),
        kind
      ),
      actual = native.find(n => n.name === name)
    assert.ok(actual, 'Native tabs action checkpoint: ' + name)
    const fields = Object.fromEntries(
      ['state', 'selected', 'tooltips', 'focused', 'menu'].map(key => [
        key,
        isDeepStrictEqual(actual[key], expected[key]),
      ])
    )
    fields.action =
      (actual.action === null && expected.action === null) ||
      (!!actual.action &&
        !!expected.action &&
        ['label', 'background', 'ink'].every(
          key => actual.action[key] === expected.action[key]
        ) &&
        ['width', 'height', 'glyphWidth', 'glyphHeight'].every(
          key => Math.abs(actual.action[key] - expected.action[key]) <= 0.6
        ))
    let paint = null
    if (actual.action && expected.action) {
      assert.equal(
        actual.action.backdrop,
        expected.action.backdrop,
        'Source and native frame grounds agree'
      )
      paint = checkTextInk(
        readPNG(
          fs.readFileSync(
            path.join(directory, 'native-tabs-actions-' + name + '.png')
          )
        ),
        actual.action.glyphBounds,
        expected.action.ink,
        composite(expected.action.background, expected.action.backdrop)
      )
      fields.glyphPaint = paint.passed
    }
    reports.push({
      name,
      expected,
      actual,
      fields,
      paint,
      passed: Object.values(fields).every(Boolean),
    })
    fs.writeFileSync(
      path.join(directory, 'actions-reference-results.json'),
      JSON.stringify(reports, null, 2)
    )
    await page.screenshot({
      path: path.join(directory, 'react-tabs-actions-' + name + '.png'),
    })
  }
  await capture('rest', 'close')
  await close().hover()
  await page.locator('.dz-tooltip').waitFor()
  await capture('hover-close', 'close')
  await page.keyboard.press('Escape')
  await capture('escape-close', 'close')
  await page.mouse.move(20, 20)
  await page.locator('[data-tab="a"]').tap()
  await page.keyboard.press('Tab')
  await capture('focus-close', 'close')
  await page.keyboard.press('Enter')
  await capture('closed')
  await page.locator('#reset').tap()
  await capture('reset')
  await page.locator('#narrow').tap()
  await more().waitFor()
  await capture('narrow', 'more')
  await more().hover()
  await page.locator('.dz-tooltip').waitFor()
  await capture('hover-more', 'more')
  await page.keyboard.press('Escape')
  await capture('escape-more', 'more')
  await page.mouse.move(20, 20)
  await page.locator('[data-tab="a"]').tap()
  await page.keyboard.press('Tab')
  await page.keyboard.press('Tab')
  await capture('focus-more', 'more')
  await page.keyboard.press('Enter')
  await page.getByRole('menu').waitFor()
  await capture('opened-menu', 'more')
  await page.keyboard.press('Escape')
  await page.getByRole('menu').waitFor({ state: 'detached' })
  await capture('menu-escape', 'more')
  await more().tap()
  await page.getByRole('menuitem', { name: 'Document 4', exact: true }).tap()
  await page.getByRole('menu').waitFor({ state: 'detached' })
  await capture('selected-hidden', 'more')
  await page.locator('#new').tap()
  await capture('trailing', 'more')
  await page.locator('#reset').tap()
  await page.locator('#refuse').tap()
  await page.locator('[data-tab="a"]').tap()
  await page.keyboard.press('Tab')
  await page.keyboard.press('Enter')
  await capture('refused-close', 'close')
  await page.locator('#refuse').tap()
  await page.locator('#toggle-close').tap()
  await page.locator('[data-tab="a"]').tap()
  await page.keyboard.press('Tab')
  await capture('no-close-more', 'more')
  await page.keyboard.press('Shift+Tab')
  await capture('no-close-reverse', 'more')
  await page.keyboard.press('Shift+Tab')
  await capture('no-close-before', 'more')
  await page.keyboard.press('Tab')
  await capture('no-close-reentry', 'more')
  await page.keyboard.press('Tab')
  await page.keyboard.press('Tab')
  await capture('no-close-tail', 'more')
  await page.locator('#reset').tap()
  await page.locator('#narrow').tap()
  await more().tap()
  await page.getByRole('menu').waitFor()
  await capture('touch-menu', 'more')
  await page.getByRole('menuitem', { name: 'Document 4', exact: true }).tap()
  await page.getByRole('menu').waitFor({ state: 'detached' })
  await capture('touch-selected', 'more')
  assert.equal(
    native.length,
    reports.length,
    'Every native state has a React reference'
  )
} finally {
  await browser?.close()
  await server.close()
}
for (const { name, fields, passed } of reports)
  console.log(JSON.stringify({ name, fields, passed }))
if (reports.some(report => !report.passed)) process.exitCode = 1
