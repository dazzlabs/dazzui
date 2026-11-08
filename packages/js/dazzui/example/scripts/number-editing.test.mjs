import assert from 'node:assert/strict'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

test('number editing distinguishes typed text, partial values and selection-aware pastes', async () => {
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    cacheDir: 'node_modules/.vite-test-number-editing',
    logLevel: 'silent',
    server: { host: '127.0.0.1', port: 0 },
    plugins: [
      {
        name: 'number-editing-fixture',
        resolveId: id =>
          id === '/number-editing.js' ? '\0number-editing' : undefined,
        load: id =>
          id === '\0number-editing'
            ? `
      import {createElement as h} from 'react';import {createRoot} from 'react-dom/client';import {flushSync} from 'react-dom';import {Button,Form,NumberField} from '@dazzlabs/dazzui';
      const host=document.createElement('div');host.id='number-editing';document.body.append(host);const root=createRoot(host);let key=0;export const changes=[],commits=[],events=[];export let submits=0;
      export function mount({form=false,controlled=false,...props}={}){changes.length=0;commits.length=0;events.length=0;submits=0;const field=h(NumberField,{key:++key,defaultValue:12,value:controlled?12:undefined,'aria-label':'Number',onValueChange:(value,details)=>{changes.push(value);events.push(['change',value,details.reason])},onValueCommitted:(value,details)=>{commits.push([value,details.reason]);events.push(['commit',value,details.reason])},...props});flushSync(()=>root.render(form?h(Form,{onSubmit:event=>{event.preventDefault();submits++}},field,h(Button,{type:'submit'},'Submit')):field));}
      export function unmount(){flushSync(()=>root.render(null));}
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
      window.numberEditing = await import('/number-editing.js')
    })
    const input = page.locator('#number-editing input').first()
    const mount = async props => {
      await page.evaluate(props => window.numberEditing.mount(props), props)
      await input.focus()
    }
    const changes = () => page.evaluate(() => window.numberEditing.changes)
    for (const text of ['abc', '12e3', '12%', '12 3', '∞']) {
      await mount()
      await input.fill(text)
      assert.equal(await input.inputValue(), '12', text)
      assert.deepEqual(await changes(), [], text)
    }
    for (const [text, value] of [
      ['+', undefined],
      ['-', undefined],
      ['.', undefined],
      ['12.', 12],
      ['1.2.3', 12.3],
      ['1,2,3', 123],
      ['12-', -12],
      ['--12', undefined],
      ['+-12', -12],
      ['12+3', 12],
      ['１２．３', 12.3],
      ['١٢٫٣', 12.3],
      ['۱۲٫۳', 12.3],
      ['一二．三', 12.3],
      ['\u200e12\u200f', 12],
    ]) {
      await mount()
      await input.fill(text)
      assert.equal(await input.inputValue(), text, text)
      assert.deepEqual(
        await changes(),
        value === undefined ? [] : [value],
        text
      )
    }
    const paste = async text =>
      input.evaluate((node, text) => {
        const data = new DataTransfer()
        data.setData('text/plain', text)
        node.dispatchEvent(
          new ClipboardEvent('paste', {
            bubbles: true,
            cancelable: true,
            clipboardData: data,
          })
        )
      }, text)
    for (const [text, value] of [
      ['12e3', 12000],
      ['12%', 0.12],
      ['12‰', 0.012],
      ['12foo', 12],
      ['12 3', 12],
      ['１２．３', 12.3],
      ['١٢٫٣', 12.3],
    ]) {
      await mount()
      await input.selectText()
      await paste(text)
      assert.equal(await input.inputValue(), text, text)
      assert.deepEqual(await changes(), [value], text)
      assert.equal(
        await input.evaluate(node => node.selectionStart),
        text.length
      )
    }
    await mount()
    await input.selectText()
    await paste('abc')
    assert.equal(await input.inputValue(), '12')
    assert.deepEqual(await changes(), [])
    await input.evaluate(node => node.setSelectionRange(1, 1))
    await paste('5')
    assert.equal(await input.inputValue(), '152')
    assert.deepEqual(await changes(), [152])
    assert.equal(await input.evaluate(node => node.selectionStart), 2)
    await input.evaluate(node => node.setSelectionRange(1, 2))
    await paste('９')
    assert.equal(await input.inputValue(), '1９2')
    assert.equal((await changes()).at(-1), 192)
    assert.equal(await input.evaluate(node => node.selectionStart), 2)
    await mount()
    const cdp = await page.context().newCDPSession(page)
    await cdp.send('Input.imeSetComposition', {
      text: 'yi',
      selectionStart: 2,
      selectionEnd: 2,
      replacementStart: 0,
      replacementEnd: 2,
    })
    assert.equal(await input.inputValue(), '12')
    assert.deepEqual(await changes(), [])
    await cdp.send('Input.insertText', { text: '一' })
    assert.equal(await input.inputValue(), '12一')
    assert.deepEqual(await changes(), [121])
    await mount()
    await input.fill('1.2')
    await page.keyboard.press('.')
    await page.keyboard.press('e')
    await page.keyboard.press('Space')
    assert.equal(await input.inputValue(), '1.2')
    await page.keyboard.press('ArrowLeft')
    await page.keyboard.press('ArrowRight')
    await page.keyboard.press('Backspace')
    assert.equal(await input.inputValue(), '1.')
    await input.selectText()
    await page.keyboard.press('-')
    await page.keyboard.press('-')
    assert.equal(await input.inputValue(), '-')
    await page.keyboard.press('1')
    assert.equal(await input.inputValue(), '-1')
    await mount({ min: 0 })
    await input.selectText()
    await page.keyboard.press('-')
    assert.equal(await input.inputValue(), '12')
    for (const [text, blurred, values] of [
      ['-', '-', []],
      ['+', '+', []],
      ['.', '.', []],
      ['12.', '12', [12]],
      ['1.2.3', '12.3', [12.3]],
    ]) {
      await mount()
      await input.fill(text)
      await page.keyboard.press('Enter')
      assert.equal(
        await input.inputValue(),
        text,
        'Enter preserves editing text'
      )
      await input.blur()
      assert.equal(await input.inputValue(), blurred)
      assert.deepEqual(await changes(), values)
    }
    await mount({ form: true })
    await page.keyboard.press('Enter')
    assert.equal(await page.evaluate(() => window.numberEditing.submits), 1)
    const commits = () => page.evaluate(() => window.numberEditing.commits)
    const events = () => page.evaluate(() => window.numberEditing.events)
    await mount()
    await input.blur()
    assert.deepEqual(
      await commits(),
      [],
      'untouched focus/blur does not commit'
    )
    await mount({ value: null })
    await input.blur()
    assert.deepEqual(
      await commits(),
      [],
      'an untouched empty field does not commit'
    )
    for (const allowOutOfRange of [false, true]) {
      await mount({ min: 0, max: 10, allowOutOfRange })
      await input.fill('20')
      assert.deepEqual(await changes(), [allowOutOfRange ? 20 : 10])
      assert.deepEqual(await commits(), [])
      await page.keyboard.press('Enter')
      assert.deepEqual(await commits(), [])
      await input.blur()
      assert.deepEqual(await commits(), [
        [allowOutOfRange ? 20 : 10, 'input-blur'],
      ])
      assert.deepEqual(await changes(), allowOutOfRange ? [20] : [10, 10])
      await input.focus()
      await page.keyboard.press('ArrowDown')
      assert.equal((await changes()).at(-1), allowOutOfRange ? 10 : 9)
      assert.equal((await commits()).at(-1)[1], 'keyboard')
    }
    await mount({ min: 0, allowOutOfRange: true })
    await input.selectText()
    await page.keyboard.press('-')
    assert.equal(await input.inputValue(), '-')
    await input.blur()
    assert.deepEqual(await commits(), [])
    await mount()
    await input.fill('')
    await input.blur()
    assert.deepEqual(await commits(), [[null, 'input-clear']])
    await mount({ controlled: true, allowOutOfRange: true })
    await input.fill('20')
    await input.blur()
    assert.deepEqual(await commits(), [[20, 'input-blur']])
    assert.equal(await input.inputValue(), '12')
    await mount({ step: 5 })
    await page.keyboard.press('ArrowUp')
    assert.deepEqual(await events(), [
      ['change', 17, 'keyboard'],
      ['commit', 17, 'keyboard'],
    ])
    await input.blur()
    assert.equal((await commits()).length, 1)
    await mount({ step: 5, allowWheelScrub: true })
    await input.evaluate(node =>
      node.dispatchEvent(
        new WheelEvent('wheel', {
          deltaY: -80,
          bubbles: true,
          cancelable: true,
        })
      )
    )
    assert.deepEqual(await events(), [
      ['change', 17, 'wheel'],
      ['commit', 17, 'wheel'],
    ])
    await input.blur()
    assert.equal((await commits()).length, 1)
    const clockStart = new Date('2025-02-01T00:00:00Z')
    await page.clock.install({ time: clockStart })
    await page.clock.pauseAt(clockStart)
    const press = async (decrement = false) => {
      const box = await page
        .locator('#number-editing .dz-number-field__step')
        .nth(decrement ? 1 : 0)
        .boundingBox()
      await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2)
      await page.mouse.down()
    }
    await mount({ step: 5, max: 22 })
    await press()
    assert.deepEqual(await commits(), [])
    await page.clock.runFor(460)
    assert.deepEqual(await changes(), [17, 22])
    assert.deepEqual(await commits(), [])
    await page.mouse.move(900, 600)
    await page.mouse.up()
    assert.deepEqual(await commits(), [[22, 'increment-press']])
    await input.blur()
    assert.equal((await commits()).length, 1)
    await mount({ step: 0 })
    await press()
    await page.mouse.up()
    assert.deepEqual(
      await commits(),
      [[12, 'increment-press']],
      'a no-op pointer hold still commits on release'
    )
    await mount({ controlled: true, step: 5 })
    await press()
    await page.clock.runFor(460)
    await page.mouse.up()
    assert.deepEqual(await changes(), [17, 17])
    assert.deepEqual(await commits(), [[17, 'increment-press']])
    await mount({ allowOutOfRange: true, min: 0, max: 10, step: 1 })
    await input.fill('20')
    await press(true)
    assert.deepEqual(
      await changes(),
      [20, 10, 10],
      'pointer normalization and its first tick share the same render value'
    )
    await page.mouse.up()
    assert.deepEqual(await commits(), [[10, 'decrement-press']])
    await mount({ step: 5 })
    await press()
    await page.evaluate(() => window.numberEditing.unmount())
    await page.mouse.up()
    assert.deepEqual(
      await commits(),
      [],
      'unmounting a held field does not commit'
    )
    await page.clock.resume()
    await page.evaluate(async () => {
      document.getElementById('number-editing').style.display = 'none'
      document.getElementById('root').style.display = ''
      await window.conformance.show('number-field/5', 'studio-light')
    })
    const example = page.locator('#root .dz-number-field__input')
    assert.equal(await example.count(), 2)
    await example.first().focus()
    await example.first().fill('105')
    assert.equal(await example.first().inputValue(), '105')
    assert.equal(await example.nth(1).inputValue(), '12')
    await page.keyboard.press('Enter')
    assert.equal(await example.nth(1).inputValue(), '12')
    await example.nth(1).focus()
    assert.equal(await example.nth(1).inputValue(), '105')
    await example.first().focus()
    await page.keyboard.press('ArrowUp')
    assert.equal(await example.first().inputValue(), '100')
    assert.equal(await example.nth(1).inputValue(), '100')
    await example.first().fill('')
    await example.nth(1).focus()
    assert.equal(await example.nth(1).inputValue(), '')
  } finally {
    await browser.close()
    await server.close()
  }
})
