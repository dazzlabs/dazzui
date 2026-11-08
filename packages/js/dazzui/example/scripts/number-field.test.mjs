import assert from 'node:assert/strict'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

test('number fields preserve authoritative precision, modifier steps and dirty controlled input', async () => {
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    cacheDir: 'node_modules/.vite-test-number-field',
    logLevel: 'silent',
    server: { host: '127.0.0.1', port: 0 },
    plugins: [
      {
        name: 'number-fixture',
        resolveId: id =>
          id === '/number-fixture.js' ? '\0number-fixture' : undefined,
        load: id =>
          id === '\0number-fixture'
            ? `
 import {createElement as h} from 'react';import {createRoot} from 'react-dom/client';import {flushSync} from 'react-dom';import {NumberField} from '@dazzlabs/dazzui'
 const viewport=document.createElement('div');viewport.id='number-viewport';viewport.style.cssText='position:fixed;left:24px;top:64px;width:240px;height:120px;overflow:auto';const host=document.createElement('div');host.id='number-fixture';const filler=document.createElement('div');filler.style.height='800px';viewport.append(host,filler);document.body.append(viewport);const root=createRoot(host);let key=0;export const changes=[],wheelPrevented=[];viewport.addEventListener('wheel',event=>wheelPrevented.push(event.defaultPrevented))
 export function mount({initial=12,controlled=false,...props}={}){changes.length=0;flushSync(()=>root.render(h(NumberField,{key:++key,defaultValue:initial,value:controlled?initial:undefined,step:5,'aria-label':'Number',onValueChange:value=>changes.push(value),...props})))}
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
      window.numberFixture = await import('/number-fixture.js')
    })
    const input = page.locator('#number-fixture .dz-number-field__input')
    const mount = async options => {
      await page.evaluate(
        options => window.numberFixture.mount(options),
        options
      )
      await input.focus()
    }
    const changes = () => page.evaluate(() => window.numberFixture.changes)
    await mount({ min: -2, max: 30 })
    for (const [key, want] of [
      ['ArrowUp', 17],
      ['ArrowDown', 12],
      ['Shift+ArrowUp', 22],
      ['Alt+ArrowDown', 21.9],
      ['Home', -2],
      ['End', 30],
      ['Alt+Shift+ArrowDown', 29.9],
    ]) {
      await page.keyboard.press(key)
      assert.equal((await changes()).at(-1), want, key)
    }
    for (const [options, keys, want] of [
      [
        { snapOnStep: true, min: 0 },
        ['ArrowUp', 'ArrowDown', 'Shift+ArrowUp', 'Alt+ArrowDown'],
        19.9,
      ],
      [{ initial: null, min: -10, max: -2 }, ['ArrowUp'], -2],
      [{ initial: null, min: -10, max: 10 }, ['ArrowDown'], 0],
      [{ step: 0 }, ['ArrowUp', 'ArrowDown'], undefined],
    ]) {
      await mount(options)
      for (const key of keys) await page.keyboard.press(key)
      assert.equal((await changes()).at(-1), want)
    }
    await mount({ controlled: true })
    await page.keyboard.press('ArrowUp')
    await page.keyboard.press('ArrowUp')
    assert.deepEqual(await changes(), [17, 17])
    await input.fill('12.2')
    assert.equal((await changes()).at(-1), 12.2)
    await page.evaluate(() => (window.numberFixture.changes.length = 0))
    await page.keyboard.press('ArrowUp')
    assert.deepEqual(await changes(), [17.2])
    assert.equal(await input.inputValue(), '12')
    await mount({ initial: 0.123456789, step: 0.00000001 })
    assert.equal(await input.inputValue(), '0.123')
    await page.keyboard.press('ArrowUp')
    assert.equal((await changes()).at(-1), 0.123456799)
    assert.equal(await input.inputValue(), '0.123')
    await input.fill('')
    await page.keyboard.press('ArrowDown')
    assert.equal(await input.inputValue(), '0')
    await mount({ initial: 1234.56789 })
    assert.equal(await input.inputValue(), '1,234.568')
    await mount({ readOnly: true, min: 0, max: 30 })
    for (const key of [
      'ArrowUp',
      'Shift+ArrowDown',
      'Alt+ArrowUp',
      'Home',
      'End',
    ])
      await page.keyboard.press(key)
    assert.deepEqual(await changes(), [])
    assert.equal(await input.inputValue(), '12')

    // Dispatch real DOM WheelEvents through the actual input. Inspect both
    // native cancellation and ancestor bubbling, separately from page motion.
    const wheel = (deltaY, extra = {}, target = input) =>
      target.evaluate(
        (node, options) => {
          const event = new WheelEvent('wheel', {
            bubbles: true,
            cancelable: true,
            ...options,
          })
          node.dispatchEvent(event)
          return event.defaultPrevented
        },
        { deltaY, ...extra }
      )
    for (const [props, blur, extra] of [
      [{}, false, {}],
      [{ allowWheelScrub: true }, true, {}],
      [{ allowWheelScrub: true, readOnly: true }, false, {}],
      [{ allowWheelScrub: true, disabled: true }, false, {}],
      [{ allowWheelScrub: true }, false, { ctrlKey: true }],
    ]) {
      await mount(props)
      if (blur) await input.evaluate(node => node.blur())
      assert.equal(await wheel(80, extra), false)
      assert.deepEqual(await changes(), [])
    }
    await mount({ allowWheelScrub: true })
    for (const [deltaY, extra, expected] of [
      [-0.1, {}, 17],
      [800, {}, 12],
      [-4, { deltaMode: 1, shiftKey: true }, 22],
      [1, { deltaMode: 1, altKey: true }, 21.9],
      [0, { deltaX: 90 }, 26.9],
    ]) {
      assert.equal(await wheel(deltaY, extra), true)
      assert.equal((await changes()).at(-1), expected)
      assert.equal(
        await page.evaluate(() => window.numberFixture.wheelPrevented.at(-1)),
        true,
        'prevented events still bubble to ancestor observers'
      )
    }
    assert.equal(
      await wheel(
        80,
        {},
        page.locator('#number-fixture .dz-number-field__step').first()
      ),
      false
    )
    assert.equal((await changes()).length, 5)
    await mount({ allowWheelScrub: true, controlled: true })
    await input.fill('12.2')
    await page.evaluate(() => (window.numberFixture.changes.length = 0))
    await wheel(-1)
    await wheel(-1)
    assert.deepEqual(
      await changes(),
      [17, 17],
      'wheel steps from the authoritative controlled number'
    )
    assert.equal(await input.inputValue(), '12')
    await mount({ allowWheelScrub: true, initial: 30, max: 30 })
    assert.equal(
      await wheel(-80),
      true,
      'boundary no-ops still prevent scrolling'
    )
    assert.deepEqual(await changes(), [])
    // Actual browser wheel input must prevent page motion only while the
    // opted-in editor is focused; a blurred field lets the page scroll.
    await mount({ allowWheelScrub: true })
    await input.hover()
    const scrollBefore = await page
      .locator('#number-viewport')
      .evaluate(node => node.scrollTop)
    await page.mouse.wheel(0, 80)
    await page.waitForFunction(() => window.numberFixture.changes.at(-1) === 7)
    assert.equal(
      await page.locator('#number-viewport').evaluate(node => node.scrollTop),
      scrollBefore
    )
    await input.evaluate(node => node.blur())
    await page.mouse.wheel(0, 80)
    await page.waitForFunction(
      before => document.getElementById('number-viewport').scrollTop > before,
      scrollBefore
    )
    await page
      .locator('#number-viewport')
      .evaluate(node => (node.scrollTop = 0))

    const clockStart = new Date('2025-01-01T00:00:00Z')
    await page.clock.install({ time: clockStart })
    await page.clock.pauseAt(clockStart)
    const stepper = direction =>
      page
        .locator(`#number-fixture .dz-number-field__step`)
        .nth(direction === 'up' ? 0 : 1)
    const press = async (direction, modifiers = []) => {
      const box = await stepper(direction).boundingBox()
      await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2)
      for (const modifier of modifiers) await page.keyboard.down(modifier)
      await page.mouse.down()
      for (const modifier of modifiers) await page.keyboard.up(modifier)
      return box
    }
    await mount({})
    await press('up')
    assert.deepEqual(await changes(), [17], 'pointer down changes immediately')
    await page.clock.runFor(459)
    assert.deepEqual(await changes(), [17])
    await page.clock.runFor(1)
    assert.deepEqual(await changes(), [17, 22])
    await page.clock.runFor(60)
    assert.deepEqual(await changes(), [17, 22, 27])
    await page.mouse.up()
    await page.clock.runFor(1000)
    assert.deepEqual(await changes(), [17, 22, 27], 'release never adds a step')

    await mount({})
    const box = await press('down')
    assert.deepEqual(await changes(), [7])
    await page.mouse.move(500, 400)
    await page.clock.runFor(1000)
    assert.deepEqual(await changes(), [7], 'leaving pauses the hold')
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2)
    assert.deepEqual(await changes(), [7, 2], 'reentry steps immediately')
    await page.clock.runFor(460)
    assert.deepEqual(await changes(), [7, 2, -3])
    await page.mouse.move(500, 400)
    await page.mouse.up()
    await page.clock.runFor(1000)
    assert.deepEqual(
      await changes(),
      [7, 2, -3],
      'release outside stops the hold'
    )

    await mount({ max: 30 })
    await press('up', ['Shift'])
    assert.deepEqual(await changes(), [22])
    await page.clock.runFor(460)
    assert.deepEqual(
      await changes(),
      [22, 30],
      'repeat retains the press modifier'
    )
    await page.clock.runFor(1000)
    assert.deepEqual(
      await changes(),
      [22, 30],
      'boundary stops repeated changes'
    )
    await page.mouse.up()

    await mount({ controlled: true })
    await press('up')
    await page.clock.runFor(460)
    assert.deepEqual(
      await changes(),
      [17, 17],
      'controlled refusal retains its numeric base'
    )
    await page.mouse.up()
    await mount({ controlled: true })
    await input.fill('12.2')
    await page.evaluate(() => (window.numberFixture.changes.length = 0))
    await press('up')
    assert.deepEqual(
      await changes(),
      [12.2, 17.2],
      'pointer down commits dirty text before stepping'
    )
    assert.equal(await input.inputValue(), '12')
    await page.mouse.up()
    assert.deepEqual(await changes(), [12.2, 17.2])
    for (const props of [{ readOnly: true }, { disabled: true }]) {
      await mount(props)
      await press('up')
      await page.clock.runFor(1000)
      await page.mouse.up()
      assert.deepEqual(await changes(), [])
    }
  } finally {
    await browser.close()
    await server.close()
  }
})
