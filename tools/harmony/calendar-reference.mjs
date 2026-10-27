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
    'packages/arkts/dazzui/example/entry/build/verification/calendar-live'
)
const native = JSON.parse(
  fs.readFileSync(path.join(directory, 'native-calendar-live.json'), 'utf8')
)
const fixture = path.join(example, '.build/harmony-calendar')
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
import {Calendar} from '../../../src/components/calendar'
import '../../../src/generated/style.css'
import '../../../src/styles/components.css'
import '../../src/styles/base.css'
function Fixture(){
 const [month,setMonth]=useState(new Date(2026,9,1)),[selected,setSelected]=useState(new Date(2026,9,20))
 const [range,setRange]=useState({start:new Date(2026,9,20),end:null})
 const [rangeMode,setRangeMode]=useState(false),[two,setTwo]=useState(false),[refuse,setRefuse]=useState(false)
 const [proposal,setProposal]=useState(0),[requests,setRequests]=useState(0)
 window.fixtureState={month:month.getTime(),selected:selected.getTime(),start:range.start?.getTime()??0,end:range.end?.getTime()??0,range:rangeMode,two,refuse,proposal,requests}
 const button=(id,label,action)=>h('button',{id,onClick:action},label)
 return h('div',{style:{margin:40,display:'flex',flexDirection:'column',alignItems:'start',gap:16}},
   h('div',null,
    button('calendar-june','Show June',()=>setMonth(new Date(2027,5,1))),
    button('calendar-october','Show October',()=>setMonth(new Date(2026,9,1))),
    button('calendar-refuse','Refuse month',()=>setRefuse(v=>!v)),
    button('calendar-two','Two months',()=>setTwo(v=>!v)),
    button('calendar-range','Range',()=>setRangeMode(v=>!v))),
   button('calendar-before','Before'),
   h(Calendar,{month,mode:rangeMode?'range':'single',months:two?2:1,value:rangeMode?range:selected,
    today:new Date(2026,9,16),min:new Date(2026,9,12),max:new Date(2026,9,23),locale:'en-US',
    onValueChange:rangeMode?setRange:setSelected,onMonthChange:next=>{setRequests(n=>n+1);setProposal(next.getTime());if(!refuse)setMonth(next)}}),
   button('calendar-after','After'))
}
createRoot(document.getElementById('root')).render(h(Fixture))
window.measureCalendar=()=>{
 const cells=[...document.querySelectorAll('button[data-day]')]
 return {state:window.fixtureState,titles:[...document.querySelectorAll('.dz-calendar__title')].map(n=>n.textContent),
  days:cells.map((node,i)=>({index:Math.floor(i/42),label:node.getAttribute('aria-label'),selected:node.parentElement.getAttribute('aria-selected')==='true',focused:document.activeElement===node,tabIndex:node.tabIndex})),
  tabStops:cells.filter(node=>node.tabIndex===0).map(node=>node.getAttribute('aria-label'))}
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
    viewport: { width: 1200, height: 700 },
    timezoneId: 'Asia/Shanghai',
    hasTouch: true,
  })
  await page.goto(
    new URL('.build/harmony-calendar/index.html', server.resolvedUrls.local[0])
      .href
  )
  await page.waitForSelector('button[data-day]')
  const tap = id => page.locator('#' + id).tap()
  const date = label =>
    page.getByRole('button', { name: label, exact: true }).first()
  const focus = async label =>
    assert.equal(
      await page.locator('button[data-day]:focus').getAttribute('aria-label'),
      label
    )
  const enter = async (count = 3) => {
    await tap('calendar-before')
    for (let i = 0; i < count; i++) await page.keyboard.press('Tab')
  }
  const capture = async name => {
    await page.waitForTimeout(100)
    const expected = await page.evaluate(() => window.measureCalendar())
    const actual = native.find(snapshot => snapshot.name === name)
    assert.ok(actual, 'Native capture: ' + name)
    const actualDays = actual.days.map(day => ({
      index: Number(day.id.match(/-(\d)-\d+$/)[1]),
      label: day.label,
      selected: day.selected,
      focused: day.focused,
      tabIndex: day.tabIndex,
    }))
    const fields = {
      state: isDeepStrictEqual(actual.state, expected.state),
      titles: isDeepStrictEqual(actual.titles, expected.titles),
      days: isDeepStrictEqual(actualDays, expected.days),
      oneTabStop: expected.tabStops.length === 1,
      nativeOneTabStop:
        actual.days.filter(day => day.tabIndex === 0).length === 1,
    }
    reports.push({
      name,
      expected,
      actual,
      fields,
      passed: Object.values(fields).every(Boolean),
    })
    fs.writeFileSync(
      path.join(directory, 'reference-results.json'),
      JSON.stringify(reports, null, 2)
    )
    await page.screenshot({
      path: path.join(directory, 'react-calendar-' + name + '.png'),
    })
  }
  await capture('initial')
  await tap('calendar-june')
  await enter()
  await focus('Tuesday, June 1, 2027')
  await capture('fallback')
  await page.keyboard.press('Tab')
  assert.equal(
    await page
      .locator('#calendar-after')
      .evaluate(node => document.activeElement === node),
    true
  )
  await page.keyboard.press('Shift+Tab')
  await focus('Tuesday, June 1, 2027')
  await tap('calendar-two')
  await enter(2)
  await focus('Tuesday, June 1, 2027')
  await page.keyboard.press('Tab')
  assert.equal(
    await page
      .getByRole('button', { name: 'Next month', exact: true })
      .evaluate(node => document.activeElement === node),
    true
  )
  await page.keyboard.press('Tab')
  assert.equal(
    await page
      .locator('#calendar-after')
      .evaluate(node => document.activeElement === node),
    true
  )
  await capture('two-month-fallback')
  await tap('calendar-two')
  await tap('calendar-october')
  await tap('calendar-refuse')
  await page.getByRole('button', { name: 'Next month', exact: true }).tap()
  await capture('refused-navigation')
  await date('Saturday, October 31, 2026').tap({ force: true })
  await page.keyboard.press('PageDown')
  await capture('refused-keyboard')
  await focus('Saturday, October 31, 2026')
  await tap('calendar-refuse')
  await date('Saturday, October 31, 2026').tap({ force: true })
  await page.keyboard.press('PageDown')
  await focus('Monday, November 30, 2026')
  await capture('accepted-keyboard')
  await tap('calendar-october')
  await tap('calendar-range')
  await date('Thursday, October 22, 2026').hover()
  await capture('range-hover')
  await page.mouse.move(20, 120)
  await date('Tuesday, October 20, 2026').tap()
  await date('Tuesday, October 20, 2026').tap()
  await page.keyboard.press('ArrowLeft')
  await focus('Monday, October 19, 2026')
  await capture('range-keyboard')
  await page.keyboard.press('Enter')
  await capture('range-complete')
  assert.equal(
    reports.length,
    native.length,
    'Every native snapshot has a React reference'
  )
  await page.close()
} finally {
  await browser?.close()
  await server.close()
}
for (const { name, fields, passed } of reports)
  console.log(JSON.stringify({ name, fields, passed }))
if (reports.some(report => !report.passed)) process.exitCode = 1
