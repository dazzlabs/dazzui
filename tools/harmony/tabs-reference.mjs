import assert from 'node:assert/strict'
import fs from 'node:fs'
import path from 'node:path'
import { createRequire } from 'node:module'
import { fileURLToPath, pathToFileURL } from 'node:url'
import { createHash } from 'node:crypto'

const repo = fileURLToPath(new URL('../../', import.meta.url))
const example = path.join(repo, 'packages/js/dazzui/example')
const require = createRequire(path.join(example, 'package.json'))
const { createServer } = await import(
  pathToFileURL(require.resolve('vite')).href
)
const { chromium } = require('playwright')
const directory = path.resolve(
  process.argv[2] ??
    'packages/arkts/dazzui/example/entry/build/verification/tabs'
)
const snapshots = JSON.parse(
  fs.readFileSync(path.join(directory, 'native-tabs-live.json'), 'utf8')
)
const fontOption = process.argv.indexOf('--font-file')
const fontPath =
  fontOption < 0 ? null : path.resolve(process.argv[fontOption + 1])
const fontBytes = fontPath ? fs.readFileSync(fontPath) : null
const referenceFont = fontBytes
  ? {
      file: path.basename(fontPath),
      sha256: createHash('sha256').update(fontBytes).digest('hex'),
    }
  : null
const fixture = path.join(example, '.build/harmony-tabs')
fs.mkdirSync(fixture, { recursive: true })
fs.writeFileSync(
  path.join(fixture, 'index.html'),
  '<!doctype html><html><body><div id="root"></div><script type="module" src="./reference.mjs"></script></body></html>'
)
fs.writeFileSync(
  path.join(fixture, 'reference.mjs'),
  `
import {createElement as h} from 'react'
import {flushSync} from 'react-dom'
import {createRoot} from 'react-dom/client'
import {Tabs} from '../../../src/components/tabs'
import {BranchRegular,CircleRegular,PlayRegular,TagRegular,HistoryRegular} from '@fluentui/react-icons'
import '../../../src/generated/style.css'
import '../../../src/styles/components.css'
import '../../src/styles/base.css'
const root=createRoot(document.getElementById('root'))
window.renderTabs = c => {
  const items=[
    {value:'alpha',label:c.renamed?'Alpha with a longer name':'Alpha',icon:BranchRegular,count:2,title:'Alpha details'},
    {value:'beta',label:'Beta',icon:CircleRegular,count:c.wideCount?400000:4},
    {value:'gamma',label:'Gamma',icon:PlayRegular,status:'primary',busy:c.busy},
    {value:'delta',label:'Delta',icon:TagRegular,detail:'Preview',count:12,title:''},
    {value:'epsilon',label:'Epsilon',icon:HistoryRegular,disabled:true,title:'Unavailable tab'},
  ].filter(item=>c.showGamma||item.value!=='gamma')
  flushSync(()=>root.render(h('div',{id:'header',style:{display:'flex',width:c.width,height:36,background:'var(--color-surface-chrome)',margin:40}},
    h(Tabs,{items,value:c.value,onValueChange:()=>{},trailing:h('button',{id:'tail',style:{width:c.wideTail?180:24,height:24,padding:0}},c.wideTail?'Extra trailing action':'New')}))))
}
const rect=node=>{const r=node.getBoundingClientRect();return[r.left,r.top,r.right,r.bottom]}
window.measureTabs = () => {
  const tabs=document.querySelector('.dz-tabs'), tail=document.getElementById('tail'), list=tabs.querySelector('.dz-tabs__list')
  return {control:rect(tabs),height:tabs.offsetHeight,tail:rect(tail),tailGap:tail.getBoundingClientRect().left-list.lastElementChild.getBoundingClientRect().right,
    tailMargin:parseFloat(getComputedStyle(tabs.querySelector('.dz-tabs__trailing')).marginInlineStart),
    basis:parseFloat(tabs.style.flexBasis),required:[...tabs.querySelectorAll('[data-ruler="whole"]')].reduce((n,node)=>n+node.offsetWidth,0)+tabs.querySelector('.dz-tabs__trailing').offsetWidth+parseFloat(getComputedStyle(tabs.querySelector('.dz-tabs__trailing')).marginInlineStart),
    tabFont:getComputedStyle(tabs.querySelector('button[data-tab]')).fontFamily,
    rulerFonts:[...tabs.querySelectorAll('[data-ruler] .dz-tabs__tab')].map(node=>getComputedStyle(node).fontFamily),
    activeRulerWeight:tabs.querySelector('[data-ruler="whole"][data-active] .dz-tabs__label')?getComputedStyle(tabs.querySelector('[data-ruler="whole"][data-active] .dz-tabs__label')).fontWeight:null,
    activeWeight:tabs.querySelector('button[aria-selected="true"] .dz-tabs__label')?getComputedStyle(tabs.querySelector('button[aria-selected="true"] .dz-tabs__label')).fontWeight:null,
    more:tabs.querySelector('.dz-tabs__more')?.getAttribute('aria-label')??null,
    tabs:[...tabs.querySelectorAll('button[data-tab]')].map(tab=>({value:tab.dataset.tab,label:itemsLabel(tab),text:[...tab.querySelectorAll('.dz-tabs__label,.dz-tabs__detail,.dz-tabs__count')].map(n=>n.textContent),selected:tab.getAttribute('aria-selected')==='true',disabled:tab.disabled,title:tab.getAttribute('title'),bounds:rect(tab)}))}
}
const itemsLabel=tab=>tab.getAttribute('aria-label')??tab.querySelector('.dz-tabs__label')?.textContent
window.pulseTrace = () => {
 const dot=document.querySelector('button[data-tab="gamma"] .dz-tabs__status'),animation=dot?.getAnimations()[0]
 if(!animation)return null
 animation.pause()
 const trace=[0,175,350,525,700,875,1050,1225,1399,1400,1575].map(time=>{animation.currentTime=time;return {time,opacity:Number(getComputedStyle(dot).opacity)}})
 return {duration:animation.effect.getTiming().duration,easing:getComputedStyle(dot).animationTimingFunction,frames:animation.effect.getKeyframes().map(f=>({offset:f.offset,opacity:f.opacity,easing:f.easing})),trace}
}
`
)
const legacyInputs = name => ({
  width: ['narrow', 'promoted'].includes(name) ? 200 : 560,
  value: ['wide', 'narrow'].includes(name) ? 'alpha' : 'gamma',
  renamed: ['updated', 'wide-trailing', 'removed', 'restored'].includes(name),
  wideCount: ['updated', 'wide-trailing', 'removed', 'restored'].includes(name),
  wideTail: ['wide-trailing', 'removed', 'restored'].includes(name),
  showGamma: name !== 'removed',
  busy: true,
})
const server = await createServer({
  root: example,
  logLevel: 'warn',
  server: { host: '127.0.0.1', port: 0 },
})
await server.listen()
let browser
const reports = []
let pulse
try {
  browser = await chromium.launch({
    executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE || undefined,
  })
  const page = await browser.newPage({ viewport: { width: 1200, height: 700 } })
  await page.goto(
    new URL('.build/harmony-tabs/index.html', server.resolvedUrls.local[0]).href
  )
  await page.waitForFunction(() => window.renderTabs)
  if (fontBytes) {
    await page.evaluate(async source => {
      const face = new FontFace(
        'DazzNativeFontReference',
        'url(data:font/ttf;base64,' + source + ')',
        { weight: '1 1000' }
      )
      document.fonts.add(await face.load())
      const style = document.createElement('style')
      style.textContent =
        '.dz-tabs__tab{font-family:DazzNativeFontReference!important}'
      document.head.append(style)
    }, fontBytes.toString('base64'))
  }
  for (const native of snapshots) {
    const inputs = native.inputs ?? legacyInputs(native.name)
    await page.evaluate(c => window.renderTabs(c), inputs)
    await page.evaluate(() => document.fonts.ready)
    await page.waitForTimeout(100)
    const expected = await page.evaluate(() => window.measureTabs())
    if (native.name === 'wide')
      pulse = await page.evaluate(() => window.pulseTrace())
    const measured = native.tabs.map(tab => ({
      value: tab.id.replace(/^dazz-navigation-tabs-\d+-/, ''),
      label: tab.native.$attrs.accessibilityText,
      text: tab.text,
      selected: tab.selected === 'true',
      disabled: !tab.native.$attrs.enabled,
      bounds: tab.bounds,
      fullWidth: tab.native.$geometry.frame.width,
    }))
    const density = native.tabs[0]?.native.$geometry.density
    assert.ok(Number.isFinite(density) && density > 0)
    const fields = {
      referenceFontApplied:
        !referenceFont ||
        (expected.tabFont === 'DazzNativeFontReference' &&
          expected.rulerFonts.every(
            family => family === 'DazzNativeFontReference'
          )),
      inputsCaptured: !!native.inputs,
      headerWidth:
        Math.abs(
          native.header[2] - native.header[0] - inputs.width * density
        ) <= 1,
      shown:
        JSON.stringify(measured.map(tab => tab.value)) ===
        JSON.stringify(expected.tabs.map(tab => tab.value)),
      labels: measured.every(tab =>
        expected.tabs.some(
          ref => ref.value === tab.value && ref.label === tab.label
        )
      ),
      text: measured.every(
        tab =>
          Array.isArray(tab.text) &&
          expected.tabs.some(
            ref =>
              ref.value === tab.value &&
              JSON.stringify(ref.text) === JSON.stringify(tab.text)
          )
      ),
      selection:
        native.tabs.every(tab => ['true', 'false'].includes(tab.selected)) &&
        JSON.stringify(
          measured.filter(tab => tab.selected).map(tab => tab.value)
        ) ===
          JSON.stringify(
            expected.tabs.filter(tab => tab.selected).map(tab => tab.value)
          ),
      disabled: measured.every(tab =>
        expected.tabs.some(
          ref => ref.value === tab.value && ref.disabled === tab.disabled
        )
      ),
      more:
        (native.more?.native.$attrs.accessibilityText ?? null) ===
        expected.more,
      height:
        Math.abs(
          (native.control[3] - native.control[1]) / density - expected.height
        ) <= 1,
      completeTabs: measured.every(
        tab => Math.abs(tab.bounds[2] - tab.bounds[0] - tab.fullWidth) <= 1
      ),
      completeTrailing:
        Math.abs(
          (native.trailing[2] - native.trailing[0]) / density -
            (inputs.wideTail ? 180 : 24)
        ) <=
        1 / density,
      tailWithinRoot: native.trailing[2] <= native.control[2] + 1,
      basisIncludesMargin: Math.abs(expected.basis - expected.required) <= 1,
      activeRulerWeight: expected.activeRulerWeight === expected.activeWeight,
      trailingGap: expected.tailGap >= -0.5,
      nativeTrailingGap:
        native.trailing[0] >=
        (native.more?.bounds ?? measured.at(-1).bounds)[2] - 1,
    }
    reports.push({
      name: native.name,
      inputs,
      expected,
      measured,
      fields,
      passed: Object.values(fields).every(Boolean),
    })
    await page.screenshot({
      path: path.join(
        directory,
        'react-tabs-' +
          (referenceFont ? 'same-font-' : '') +
          native.name +
          '.png'
      ),
    })
  }
  await page.close()
} finally {
  await browser?.close()
  await server.close()
}
fs.writeFileSync(
  path.join(
    directory,
    referenceFont
      ? 'same-font-reference-results.json'
      : 'reference-results.json'
  ),
  JSON.stringify({ referenceFont, pulse, reports }, null, 2)
)
for (const r of reports)
  console.log(
    JSON.stringify({ name: r.name, fields: r.fields, passed: r.passed })
  )
if (reports.some(r => !r.passed)) process.exitCode = 1
