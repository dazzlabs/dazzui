import assert from 'node:assert/strict'
import fs from 'node:fs'
import path from 'node:path'
import { createRequire } from 'node:module'
import { fileURLToPath, pathToFileURL } from 'node:url'
import { isDeepStrictEqual } from 'node:util'

const repo = fileURLToPath(new URL('../../', import.meta.url))
const example = path.join(repo, 'packages/js/dazzui/example')
const require = createRequire(path.join(example, 'package.json'))
const { createServer } = await import(
  pathToFileURL(require.resolve('vite')).href
)
const { chromium } = require('playwright')
const directory = path.resolve(
  process.argv[2] ??
    'packages/arkts/dazzui/example/entry/build/verification/date-range-live'
)
const native = JSON.parse(
  fs.readFileSync(path.join(directory, 'native-date-range-live.json'), 'utf8')
)
const fixture = path.join(example, '.build/harmony-date-range')
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
import {DateRangePicker} from '../../../src/components/date-range-picker'
import '../../../src/generated/style.css'
import '../../../src/styles/components.css'
import '../../src/styles/base.css'
function Fixture(){
 const [value,setValue]=useState({start:new Date(2026,9,16,14,30),end:new Date(2026,9,16,15,30),allDay:false})
 const [refuse,setRefuse]=useState(false),[commits,setCommits]=useState(0),[proposal,setProposal]=useState(null),[outside,setOutside]=useState(0)
 window.fixtureState={start:value.start.getTime(),end:value.end.getTime(),allDay:value.allDay,refuse,commits,proposedStart:proposal?.start.getTime()??0,proposedEnd:proposal?.end.getTime()??0,proposedAllDay:proposal?.allDay??false,outside}
 const button=(id,label,action)=>h('button',{id,onClick:action},label)
 return h('div',{style:{margin:40,display:'flex',flexDirection:'column',alignItems:'start',gap:16}},
   h('div',null,
    button('picker-live-refuse','Refuse value',()=>setRefuse(v=>!v)),
    button('picker-live-replace','Replace value',()=>setValue({start:new Date(2026,9,20,8),end:new Date(2026,9,20,9),allDay:false}))),
   button('picker-live-before','Before'),
   h('div',{style:{display:'flex',gap:16}},
    h(DateRangePicker,{id:'picker-live-trigger','aria-label':'When',value,today:new Date(2026,9,16),locale:'en-US',
      onValueChange:next=>{setCommits(n=>n+1);setProposal(next);if(!refuse)setValue(next)}}),
    button('picker-live-outside','Outside action',()=>setOutside(n=>n+1))))
}
createRoot(document.getElementById('root')).render(h(Fixture))
window.measurePicker=()=>{
 const panel=document.querySelector('.dz-date-range-picker'),trigger=document.getElementById('picker-live-trigger'),clone=trigger.querySelector('.dz-select__value').cloneNode(true)
 for(const node of clone.querySelectorAll('.dz-date-range-picker__sep'))node.textContent=' '+node.textContent+' '
 const active=document.activeElement
 const label=active?.tagName==='BUTTON' ? active.getAttribute('aria-label')??active.textContent.trim() : null
 const time=start=>panel?.querySelector('[aria-label="'+(start?'Start':'End')+' time"]')?.textContent.trim()??null
 return {state:window.fixtureState,open:trigger.getAttribute('aria-expanded')==='true',focused:label?[label]:[],
   focusOwner:active===trigger?'trigger':active===panel?'popup':label?'control':null,
   tooltips:[...document.querySelectorAll('.dz-tooltip')].map(node=>node.textContent),startTime:time(true),endTime:time(false),
   selectedDays:panel?[...panel.querySelectorAll('[role="gridcell"][aria-selected="true"] button[data-day]')].map(node=>Number(node.dataset.day)):[],
   trigger:clone.textContent+(trigger.getAttribute('aria-expanded')==='true'?', expanded':', collapsed')}
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
  const page = await browser.newPage({
    viewport: { width: 1200, height: 800 },
    timezoneId: 'Asia/Shanghai',
    hasTouch: true,
  })
  await page.goto(
    new URL(
      '.build/harmony-date-range/index.html',
      server.resolvedUrls.local[0]
    ).href
  )
  await page.waitForSelector('#picker-live-trigger')
  const tap = id => page.locator('#' + id).tap()
  const keys = key => page.keyboard.press(key)
  const isOpen = () =>
    page
      .locator('#picker-live-trigger')
      .getAttribute('aria-expanded')
      .then(v => v === 'true')
  const open = async (keyboard, mouse = false) => {
    if (keyboard) {
      await tap('picker-live-before')
      await keys('Tab')
      await keys('Enter')
    } else if (mouse) await page.locator('#picker-live-trigger').click()
    else await tap('picker-live-trigger')
    await page.locator('.dz-date-range-picker').waitFor()
    await page.waitForTimeout(200)
  }
  const closed = async () => {
    await page.locator('.dz-date-range-picker').waitFor({ state: 'detached' })
    await page.waitForTimeout(100)
  }
  const press = label =>
    page
      .locator('.dz-date-range-picker')
      .getByRole('button', { name: label, exact: true })
      .tap()
  const capture = async name => {
    await page.waitForTimeout(100)
    const expected = await page.evaluate(() => window.measurePicker()),
      actual = native.find(s => s.name === name)
    assert.ok(actual, 'Native picker capture: ' + name)
    const fields = {
      state: isDeepStrictEqual(actual.state, expected.state),
      open: actual.open === expected.open,
      focused: isDeepStrictEqual(
        actual.focused.map(n => n.label),
        expected.focused
      ),
      focusOwner: actual.focusOwner === expected.focusOwner,
      tooltips: isDeepStrictEqual(actual.tooltips, expected.tooltips),
      startTime: actual.startTime === expected.startTime,
      endTime: actual.endTime === expected.endTime,
      selectedDays: isDeepStrictEqual(
        actual.selectedDays,
        expected.selectedDays
      ),
      trigger: actual.trigger === expected.trigger,
    }
    reports.push({
      name,
      actual,
      expected,
      fields,
      passed: Object.values(fields).every(Boolean),
    })
    fs.writeFileSync(
      path.join(directory, 'reference-results.json'),
      JSON.stringify(reports, null, 2)
    )
    await page.screenshot({
      path: path.join(directory, 'react-picker-' + name + '.png'),
    })
  }
  await capture('initial')
  await open(true)
  await capture('opened')
  await press('2 hours')
  await capture('draft')
  await press('Cancel')
  await closed()
  await capture('cancelled')
  await open(false)
  await press('2 hours')
  await press('Done')
  await closed()
  await capture('accepted')
  await tap('picker-live-refuse')
  await open(false)
  await press('30 min')
  await press('Done')
  await closed()
  await capture('refused')
  await open(false)
  await capture('reopened-refused')
  await keys('Escape')
  await closed()
  await capture('escape')
  await open(false)
  await keys('Tab')
  await capture('touch-tab-entry')
  await keys('Escape')
  assert.equal(
    await isOpen(),
    true,
    'Escape dismisses the icon hint before its parent picker'
  )
  await capture('hint-escape')
  await keys('Escape')
  await closed()
  await open(false)
  await press('All day')
  await tap('picker-live-outside')
  await closed()
  await capture('outside')
  await open(false, true)
  await capture('mouse-opened')
  await keys('Escape')
  await closed()
  await page.mouse.move(20, 120)
  await open(false)
  await page.locator('.dz-date-range-picker [aria-label="Start time"]').tap()
  await page.getByRole('listbox').waitFor()
  await keys('Escape')
  await page.getByRole('listbox').waitFor({ state: 'detached' })
  assert.equal(await isOpen(), true, 'A nested Escape retains the picker')
  await capture('nested-escape')
  await keys('Escape')
  await closed()
  await capture('nested-popup-escape')
  await open(true)
  for (let i = 0; i < 12; i++) await keys('Tab')
  await page.waitForTimeout(300)
  await capture('tab-out')
  if (await isOpen()) {
    await keys('Escape')
    await closed()
  }
  await open(true)
  await keys('Shift+Tab')
  await page.waitForTimeout(300)
  await capture('reverse-tab-out')
  await keys('Escape')
  await closed()
  await capture('trigger-escape')
  await open(true)
  await keys('Shift+Tab')
  await keys('Tab')
  await page.waitForTimeout(300)
  await capture('reverse-reentry')
  await keys('Shift+Tab')
  await keys('Shift+Tab')
  await page.waitForTimeout(300)
  await capture('reverse-tab-before')
  if (await isOpen()) {
    await keys('Escape')
    await closed()
  }
  assert.equal(
    reports.length,
    native.length,
    'Every native state has a React reference'
  )
  await page.close()
} finally {
  await browser?.close()
  await server.close()
}
for (const { name, fields, passed } of reports)
  console.log(JSON.stringify({ name, fields, passed }))
if (reports.some(report => !report.passed)) process.exitCode = 1
