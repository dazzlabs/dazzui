import assert from 'node:assert/strict'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

test('multiline rows keep a viewport with caret reveal and nested wheel scrolling', async t => {
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    cacheDir: 'node_modules/.vite-test-text-field-scroll',
    logLevel: 'silent',
    server: { host: '127.0.0.1', port: 0 },
    plugins: [
      {
        name: 'text-field-scroll-fixture',
        resolveId: id =>
          id === '/text-field-scroll.js' ? '\0text-field-scroll' : undefined,
        load: id =>
          id === '\0text-field-scroll'
            ? `
        import {createElement as h} from 'react';import {createRoot} from 'react-dom/client';import {flushSync} from 'react-dom';import {TextField} from '@dazzlabs/dazzui';
        const host=document.createElement('div');host.id='text-field-scroll';document.body.append(host);const root=createRoot(host);let key=0;
        export function mount(props={}) {flushSync(()=>root.render(h('div',{id:'scroll-parent',style:{width:320,height:160,overflowY:'auto'}},h(TextField,{key:++key,multiline:true,rows:3,'aria-label':'Notes',...props}),h('div',{style:{height:800}}))));}
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
      await window.conformance.show('text-field/preview', 'studio-light')
      document.getElementById('root').style.display = 'none'
      window.textFieldScroll = await import('/text-field-scroll.js')
      window.textFieldScroll.mount()
    })
    const input = page.locator('#text-field-scroll textarea')
    const measure = (target = input) =>
      target.evaluate(n => ({
        height: n.getBoundingClientRect().height,
        top: n.scrollTop,
        max: n.scrollHeight - n.clientHeight,
        selection: [n.selectionStart, n.selectionEnd],
        page: document.getElementById('scroll-parent').scrollTop,
      }))
    const text = Array.from({ length: 20 }, (_, i) => `Line ${i + 1}`).join(
      '\n'
    )
    // Native caret metrics depend on the host font. Keep the exact browser
    // behavior as the oracle, independently of the component's event wiring.
    await input.evaluate(n => {
      const native = document.createElement('textarea')
      native.id = 'native-scroll-oracle'
      const style = getComputedStyle(n)
      for (const property of style) {
        if (!property.startsWith('--'))
          native.style.setProperty(property, style.getPropertyValue(property))
      }
      Object.assign(native.style, {
        position: 'fixed',
        top: '16px',
        right: '16px',
      })
      document.body.append(native)
    })
    const native = page.locator('#native-scroll-oracle')
    await native.fill(text)
    const nativeEnd = await measure(native)
    await input.fill(text)
    const end = await measure()
    assert.equal(end.height, 80)
    assert.deepEqual(
      end,
      nativeEnd,
      'caret reveal matches the same styled native textarea'
    )
    assert.equal(end.max, 326)
    t.diagnostic(
      `native caret scroll=${nativeEnd.top}, viewport=${end.height}, max=${end.max}`
    )
    const press = async key => {
      const before = await measure()
      const replay = async target => {
        await target.focus()
        await target.evaluate((n, before) => {
          n.setSelectionRange(...before.selection)
          n.scrollTop = before.top
        }, before)
        await page.keyboard.press(key)
        return measure(target)
      }
      const expected = await replay(native)
      assert.deepEqual(
        await replay(input),
        expected,
        `${key} matches native caret reveal`
      )
      return expected
    }
    const start = await press(
      process.platform === 'darwin' ? 'Meta+ArrowUp' : 'Control+Home'
    )
    t.diagnostic(`native document-start scroll=${start.top}`)
    assert.deepEqual(start.selection, [0, 0])
    assert.equal(
      (await press(process.platform === 'darwin' ? 'Meta+a' : 'Control+a')).top,
      start.top
    )
    await press('ArrowLeft')
    await input.hover()
    await page.mouse.wheel(0, 60)
    await page.waitForFunction(
      top =>
        document.querySelector('#text-field-scroll textarea').scrollTop === top,
      start.top + 60
    )
    assert.deepEqual((await measure()).selection, [0, 0])
    assert.equal((await measure()).page, 0)
    assert.equal((await press('ArrowRight')).top, start.top)
    await page.mouse.wheel(0, 7)
    await page.waitForFunction(
      top =>
        document.querySelector('#text-field-scroll textarea').scrollTop === top,
      start.top + 7
    )
    const nearStart = await press(
      process.platform === 'darwin' ? 'Meta+ArrowUp' : 'Control+Home'
    )
    t.diagnostic(`native near-start navigation scroll=${nearStart.top}`)
    await page.mouse.wheel(0, 10000)
    await page.waitForFunction(
      () =>
        document.querySelector('#text-field-scroll textarea').scrollTop === 326
    )
    assert.equal((await measure()).page, 0)
    await page.mouse.wheel(0, 80)
    await page.waitForFunction(
      () => document.getElementById('scroll-parent').scrollTop > 0
    )
    await input.evaluate(n => {
      document.getElementById('scroll-parent').scrollTop = 0
      n.focus()
      n.setSelectionRange(0, 0)
      n.scrollTop = 40
    })
    await native.evaluate(n => {
      n.setSelectionRange(0, 0)
      n.scrollTop = 40
    })
    for (const target of [native, input]) {
      const box = await target.boundingBox()
      await page.mouse.move(box.x + box.width - 3, box.y + box.height - 3)
      await page.mouse.down()
      await page.mouse.move(box.x + box.width - 3, box.y + box.height + 41)
      await page.mouse.up()
      const resized = await measure(target)
      assert.equal(resized.height, 124)
      assert.equal(resized.top, 40, 'dragging the grip retains manual scroll')
    }
    const resizedStart = await press('ArrowRight')
    assert.equal(resizedStart.top, start.top)
    t.diagnostic(`native post-resize caret scroll=${resizedStart.top}`)
    for (const props of [{ readOnly: true }, { disabled: true }]) {
      await page.evaluate(
        ({ props, text }) =>
          window.textFieldScroll.mount({ ...props, defaultValue: text }),
        { props, text }
      )
      await input.hover()
      await page.mouse.wheel(0, 60)
      await page.waitForFunction(
        () =>
          document.querySelector('#text-field-scroll textarea').scrollTop === 60
      )
      assert.equal(await input.inputValue(), text)
    }
  } finally {
    await browser.close()
    await server.close()
  }
})
