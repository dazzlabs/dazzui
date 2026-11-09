import assert from 'node:assert/strict'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

test('textarea corner resizes vertically without editing, focusing or revealing its caret', async () => {
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    cacheDir: 'node_modules/.vite-test-text-field-resize',
    logLevel: 'silent',
    server: { host: '127.0.0.1', port: 0 },
    plugins: [
      {
        name: 'text-field-resize-fixture',
        resolveId: id =>
          id === '/text-field-resize.js' ? '\0text-field-resize' : undefined,
        load: id =>
          id === '\0text-field-resize'
            ? `
        import {createElement as h} from 'react';import {createRoot} from 'react-dom/client';import {flushSync} from 'react-dom';import {TextField} from '@dazzlabs/dazzui';
        const host=document.createElement('div');host.id='text-field-resize';Object.assign(host.style,{position:'fixed',left:'20px',top:'20px',width:'320px'});document.body.append(host);const root=createRoot(host);let props={},key=0;window.resizeChanges=0;
        export function mount(next={},reset=true) {props=reset?next:{...props,...next};if(reset)key++;flushSync(()=>root.render(h('div',null,h('input',{id:'resize-anchor'}),h(TextField,{key,multiline:true,rows:3,'aria-label':'Notes',onChange:()=>window.resizeChanges++,...props}))));}
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
      window.textFieldResize = await import('/text-field-resize.js')
    })
    const input = page.locator('#text-field-resize .dz-text-field')
    const measure = () =>
      input.evaluate(n => ({
        height: n.getBoundingClientRect().height,
        width: n.getBoundingClientRect().width,
        focused: document.activeElement === n,
        selection: [n.selectionStart, n.selectionEnd],
        scroll: n.scrollTop,
        changes: window.resizeChanges,
        value: n.value,
      }))
    const mount = props =>
      page.evaluate(props => window.textFieldResize.mount(props), props)
    const update = props =>
      page.evaluate(props => window.textFieldResize.mount(props, false), props)
    const corner = async () => {
      const b = await input.boundingBox()
      return { x: b.x + b.width - 3, y: b.y + b.height - 3 }
    }
    const drag = async (dx, dy) => {
      const from = await corner()
      await page.mouse.move(from.x, from.y)
      await page.mouse.down()
      await page.mouse.move(from.x + dx, from.y + dy)
      const held = await measure()
      await page.mouse.up()
      return held
    }
    const text = Array.from({ length: 20 }, (_, i) => `Line ${i + 1}`).join(
      '\n'
    )
    for (const props of [{}, { readOnly: true }, { disabled: true }]) {
      await mount({ ...props, defaultValue: text })
      await input.evaluate(n => {
        document.getElementById('resize-anchor').focus()
        n.setSelectionRange(5, 9)
      })
      const before = await measure()
      assert.equal(before.height, 80)
      assert.deepEqual(await drag(70, 44), { ...before, height: 124 })
      await page.mouse.move(600, 300)
      assert.equal(
        (await measure()).height,
        124,
        'release outside ends resizing'
      )
      assert.deepEqual(
        await drag(-60, -500),
        before,
        'ds minimum still applies'
      )
    }
    await mount()
    await drag(70, 0)
    await update({ rows: 8 })
    assert(
      (await measure()).height > 80,
      'horizontal movement retains intrinsic sizing'
    )
    const initial = await measure()
    const grown = await drag(70, 44)
    assert.equal(grown.height, Math.round(initial.height) + 44)
    assert.equal(grown.width, initial.width)
    await update({ rows: 12, size: 'large' })
    assert.equal(
      (await measure()).height,
      grown.height,
      'user height overrides intrinsic rows'
    )
    await update({ multiline: false })
    await update({ multiline: true })
    assert(
      (await measure()).height > grown.height,
      'a different element starts with intrinsic rows'
    )

    await mount()
    const from = await corner()
    await page.mouse.move(from.x, from.y)
    await page.mouse.down()
    await page.mouse.move(from.x, from.y + 44)
    await page.mouse.move(from.x, from.y)
    assert.equal(
      (await measure()).height,
      80,
      'moving back restores the starting size'
    )
    await page.mouse.up()

    await mount({ defaultValue: text })
    await input.evaluate(n => {
      n.focus()
      n.setSelectionRange(n.value.length, n.value.length)
      n.scrollTop = 40
    })
    const scrolled = await measure()
    assert.equal(scrolled.scroll, 40)
    assert.deepEqual(await drag(70, 44), { ...scrolled, height: 124 })
  } finally {
    await browser.close()
    await server.close()
  }
})
