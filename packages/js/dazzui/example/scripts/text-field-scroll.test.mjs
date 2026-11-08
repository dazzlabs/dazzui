import assert from 'node:assert/strict'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

test('multiline rows keep a viewport with caret reveal and nested wheel scrolling', async () => {
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
    const measure = () =>
      input.evaluate(n => ({
        height: n.getBoundingClientRect().height,
        top: n.scrollTop,
        max: n.scrollHeight - n.clientHeight,
        selection: [n.selectionStart, n.selectionEnd],
        page: document.getElementById('scroll-parent').scrollTop,
      }))
    const text = Array.from({ length: 20 }, (_, i) => `Line ${i + 1}`).join(
      '\n'
    )
    await input.fill(text)
    const end = await measure()
    assert.equal(end.height, 80)
    assert.equal(end.top, 314)
    assert.equal(end.max, 326)
    await input.press(
      process.platform === 'darwin' ? 'Meta+ArrowUp' : 'Control+Home'
    )
    assert.equal((await measure()).top, 0)
    await input.press(process.platform === 'darwin' ? 'Meta+a' : 'Control+a')
    assert.equal((await measure()).top, 0)
    await input.press('ArrowLeft')
    await input.hover()
    await page.mouse.wheel(0, 60)
    await page.waitForFunction(
      () =>
        document.querySelector('#text-field-scroll textarea').scrollTop === 60
    )
    assert.deepEqual((await measure()).selection, [0, 0])
    assert.equal((await measure()).page, 0)
    await input.press('ArrowRight')
    assert.equal((await measure()).top, 0)
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
