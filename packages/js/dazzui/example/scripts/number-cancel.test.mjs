import assert from 'node:assert/strict'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

test('canceled number changes retain drafts, numeric authority and valid commit history', async () => {
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    cacheDir: 'node_modules/.vite-test-number-cancel',
    logLevel: 'silent',
    server: { host: '127.0.0.1', port: 0 },
    plugins: [
      {
        name: 'number-cancel-fixture',
        resolveId: id =>
          id === '/number-cancel.js' ? '\0number-cancel' : undefined,
        load: id =>
          id === '\0number-cancel'
            ? `
      import {createElement as h} from 'react';import {createRoot} from 'react-dom/client';import {flushSync} from 'react-dom';import {NumberField} from '@dazzlabs/dazzui';
      const host=document.createElement('div');host.id='number-cancel';document.body.append(host);const root=createRoot(host);let key=0,rule=null;export const changes=[],commits=[];
      export function setCancel(next){rule=next}
      export function mount({initial=12,controlled=false,cancel=null,...props}={}){changes.length=0;commits.length=0;rule=cancel;flushSync(()=>root.render(h(NumberField,{key:++key,defaultValue:initial,value:controlled?initial:undefined,step:5,'aria-label':'Number',onValueChange:(value,details)=>{if(rule==='all'||rule===details.reason||rule==='normalization'&&details.reason.endsWith('-press')&&details.direction===undefined)details.cancel();changes.push([value,details.reason,details.direction??null,details.isCanceled])},onValueCommitted:(value,details)=>commits.push([value,details.reason]),...props})));}
    `
            : undefined,
      },
    ],
  })
  await server.listen()
  const browser = await chromium.launch({
    executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE || undefined,
  })
  try {
    const page = await browser.newPage()
    await page.goto(
      new URL('conformance.html', server.resolvedUrls.local[0]).href
    )
    await page.waitForFunction(() => window.conformance)
    await page.evaluate(async () => {
      await window.conformance.show('number-field/preview', 'studio-light')
      document.getElementById('root').style.display = 'none'
      window.numberCancel = await import('/number-cancel.js')
    })
    const input = page.locator('#number-cancel .dz-number-field__input')
    const mount = async props => {
      await page.evaluate(props => window.numberCancel.mount(props), props)
      await input.focus()
    }
    const cancel = rule =>
      page.evaluate(rule => window.numberCancel.setCancel(rule), rule)
    const changes = () => page.evaluate(() => window.numberCancel.changes)
    const commits = () => page.evaluate(() => window.numberCancel.commits)
    await mount({ cancel: 'keyboard', min: 0, max: 30 })
    for (const key of ['ArrowUp', 'ArrowDown', 'Home', 'End'])
      await page.keyboard.press(key)
    assert.deepEqual(await changes(), [
      [17, 'keyboard', 1, true],
      [7, 'keyboard', -1, true],
      [0, 'keyboard', null, true],
      [30, 'keyboard', null, true],
    ])
    assert.equal(await input.inputValue(), '12')
    assert.deepEqual(await commits(), [])
    await cancel(null)
    await page.keyboard.press('ArrowUp')
    assert.deepEqual(await commits(), [[17, 'keyboard']])
    await mount({ cancel: 'wheel', allowWheelScrub: true })
    const wheel = () =>
      input.evaluate(node =>
        node.dispatchEvent(
          new WheelEvent('wheel', {
            deltaY: -80,
            bubbles: true,
            cancelable: true,
          })
        )
      )
    await wheel()
    assert.deepEqual(await changes(), [[17, 'wheel', 1, true]])
    assert.equal(await input.inputValue(), '12')
    assert.deepEqual(await commits(), [])
    await cancel(null)
    await wheel()
    assert.deepEqual(await commits(), [[17, 'wheel']])
    await mount({ cancel: 'input-change' })
    await input.fill('20')
    assert.equal(await input.inputValue(), '20')
    await cancel('keyboard')
    await page.keyboard.press('ArrowUp')
    assert.equal(await input.inputValue(), '20')
    await cancel(null)
    await page.keyboard.press('ArrowUp')
    assert.deepEqual(await changes(), [
      [20, 'input-change', null, true],
      [25, 'keyboard', 1, true],
      [17, 'keyboard', 1, false],
    ])
    assert.equal(await input.inputValue(), '17')
    assert.deepEqual(await commits(), [[17, 'keyboard']])
    await mount({ cancel: 'all' })
    await input.fill('20')
    await input.blur()
    assert.equal(await input.inputValue(), '20')
    assert.deepEqual(await changes(), [
      [20, 'input-change', null, true],
      [20, 'input-blur', null, true],
    ])
    assert.deepEqual(await commits(), [])
    await input.focus()
    await cancel(null)
    await page.keyboard.press('ArrowUp')
    assert.equal(await input.inputValue(), '17')
    await mount({ cancel: 'input-clear' })
    await input.fill('')
    await input.blur()
    assert.equal(await input.inputValue(), '')
    assert.deepEqual(await commits(), [])
    await input.focus()
    await cancel(null)
    await page.keyboard.press('ArrowUp')
    assert.equal(await input.inputValue(), '17')
    await mount({ cancel: 'input-paste' })
    await input.selectText()
    await input.evaluate(node => {
      const data = new DataTransfer()
      data.setData('text/plain', '12.2')
      node.dispatchEvent(
        new ClipboardEvent('paste', {
          bubbles: true,
          cancelable: true,
          clipboardData: data,
        })
      )
    })
    assert.equal(await input.inputValue(), '12.2')
    assert.deepEqual(await changes(), [[12.2, 'input-paste', null, true]])
    await page.keyboard.press('ArrowUp')
    assert.deepEqual(await commits(), [[17.2, 'keyboard']])
    await mount({ controlled: true, cancel: 'keyboard' })
    await input.fill('20')
    await page.keyboard.press('ArrowUp')
    await input.blur()
    assert.deepEqual(
      await commits(),
      [[12, 'input-blur']],
      'a canceled key cannot commit its stale draft or proposal'
    )
    await mount({ initial: null, cancel: 'keyboard' })
    await page.keyboard.press('ArrowUp')
    assert.deepEqual(await changes(), [[0, 'keyboard', null, true]])
    await cancel(null)
    await page.keyboard.press('ArrowUp')
    assert.deepEqual(await commits(), [[0, 'keyboard']])
    const start = new Date('2025-03-01T00:00:00Z')
    await page.clock.install({ time: start })
    await page.clock.pauseAt(start)
    const press = async () => {
      const box = await page
        .locator('#number-cancel .dz-number-field__step')
        .first()
        .boundingBox()
      await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2)
      await page.mouse.down()
    }
    await mount({ cancel: 'all' })
    await press()
    await page.clock.runFor(1000)
    assert.deepEqual(await changes(), [[17, 'increment-press', 1, true]])
    await page.mouse.up()
    assert.deepEqual(
      await commits(),
      [[12, 'increment-press']],
      'release commits the current value when the first tick was canceled'
    )
    await mount({ controlled: true, cancel: 'normalization' })
    await input.fill('12.2')
    await press()
    await page.mouse.up()
    assert.deepEqual(await changes(), [
      [12.2, 'input-change', null, false],
      [12.2, 'increment-press', null, true],
      [17, 'increment-press', 1, false],
    ])
    assert.deepEqual(await commits(), [[17, 'increment-press']])
    await mount()
    await press()
    await cancel('increment-press')
    await page.clock.runFor(1000)
    await page.mouse.up()
    assert.deepEqual(await changes(), [
      [17, 'increment-press', 1, false],
      [22, 'increment-press', 1, true],
    ])
    assert.deepEqual(
      await commits(),
      [[17, 'increment-press']],
      'a canceled repeat cannot replace the last applied value'
    )
  } finally {
    await browser.close()
    await server.close()
  }
})
