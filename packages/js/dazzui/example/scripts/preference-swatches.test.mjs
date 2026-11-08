import assert from 'node:assert/strict'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

test('preference theme swatches retain button semantics, palettes and wrapping', async () => {
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    cacheDir: 'node_modules/.vite-test-preference-swatches',
    logLevel: 'silent',
    server: { host: '127.0.0.1', port: 0 },
    plugins: [
      {
        name: 'preference-swatches-fixture',
        resolveId: id =>
          id === '/swatches-fixture.js' ? '\0swatches-fixture' : undefined,
        load: id =>
          id === '\0swatches-fixture'
            ? `
        import {createElement as h} from 'react'
        import {createRoot} from 'react-dom/client'
        import {flushSync} from 'react-dom'
        import {PreferenceRow, PreferenceThemeSwatches, Button} from '@dazzlabs/dazzui'
        const host=document.createElement('div')
        host.id='swatches-fixture'
        host.style.cssText='position:fixed;left:24px;top:64px;width:320px'
        document.body.append(host)
        const root=createRoot(host)
        let key=0
        export const changes=[]
        const options=['bright','studio','frost','graphite','ember','nocturne'].map(name=>({value:name+'-light',label:name,...(name==='bright'?{theme:''}:{})}))
        export function mount({value, defaultValue, width=320, row=false, special=false, reset=false}={}) {
          if(reset)key++
          host.style.width=width+'px'
          const choices=special?[
            {value:'root',label:'Root',theme:''},
            {value:'bright-light',label:'Inherited Bright name'},
            {value:'unknown-theme',label:'Inherited unknown'},
            {value:'alias',label:'Alias',theme:'studio-light'},
          ]:options
          const swatches=h(PreferenceThemeSwatches,{key,options:choices,value,defaultValue,onValueChange(value){changes.push(value)}})
          flushSync(()=>root.render(h('div',null,row?h(PreferenceRow,{title:'Theme'},swatches):swatches,h(Button,null,'Next'))))
        }
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
      await window.conformance.show('preferences/3', 'studio-dark')
      document.getElementById('root').style.visibility = 'hidden'
      window.swatchesFixture = await import('/swatches-fixture.js')
    })
    const fixture = page.locator('#swatches-fixture')
    const radios = fixture.getByRole('radio')
    const mount = options =>
      page.evaluate(options => window.swatchesFixture.mount(options), options)
    const checked = () =>
      radios.evaluateAll(buttons =>
        buttons
          .filter(button => button.getAttribute('aria-checked') === 'true')
          .map(button => button.getAttribute('aria-label'))
      )
    await mount({ reset: true })
    assert.deepEqual(await checked(), [])
    await radios.nth(0).focus()
    await page.keyboard.press('ArrowRight')
    assert.equal(
      await radios.nth(0).evaluate(button => button === document.activeElement),
      true
    )
    await page.keyboard.down('Enter')
    assert.deepEqual(await checked(), ['bright'])
    await page.keyboard.up('Enter')
    await page.keyboard.press('Enter')
    assert.deepEqual(
      await page.evaluate(() => window.swatchesFixture.changes),
      ['bright-light', 'bright-light']
    )
    await page.keyboard.press('Tab')
    await page.keyboard.down('Space')
    assert.deepEqual(await checked(), ['bright'])
    await page.keyboard.up('Space')
    assert.deepEqual(await checked(), ['studio'])
    for (const name of ['frost', 'graphite', 'ember', 'nocturne']) {
      await page.keyboard.press('Tab')
      assert.equal(
        await fixture
          .getByRole('radio', { name, exact: true })
          .evaluate(button => button === document.activeElement),
        true
      )
    }
    await page.keyboard.press('Tab')
    assert.equal(
      await fixture
        .getByRole('button', { name: 'Next' })
        .evaluate(button => button === document.activeElement),
      true
    )
    await mount({ reset: true, value: 'bright-light' })
    await radios.nth(1).click()
    assert.deepEqual(await checked(), ['bright'])
    await mount({})
    assert.deepEqual(await checked(), ['studio'])
    const preview = () =>
      fixture.locator('.dz-preference-themes__window').evaluateAll(nodes =>
        nodes.map(node => ({
          background: getComputedStyle(node).backgroundColor,
          height: node.getBoundingClientRect().height,
          accent: getComputedStyle(
            node.querySelector('.dz-preference-themes__accent')
          ).backgroundColor,
        }))
      )
    const originalPreviews = await preview()
    const bright = originalPreviews[0].background
    for (const host of ['bright-light', 'studio-dark', 'macos27-dark']) {
      await page.evaluate(
        host => window.conformance.show('preferences/3', host),
        host
      )
      await mount({ reset: true })
      const previews = await preview()
      assert.equal(previews[0].background, bright)
      assert.deepEqual(
        previews.slice(1).map(({ height: _height, ...colours }) => colours),
        originalPreviews
          .slice(1)
          .map(({ height: _height, ...colours }) => colours)
      )
      assert.ok(
        previews.every(preview => preview.height === previews[0].height)
      )
      assert.notEqual(previews[1].accent, previews[2].accent)
    }
    await mount({ reset: true, special: true })
    const special = await preview()
    assert.equal(special[0].background, bright)
    assert.equal(special[0].height, special[1].height)
    assert.equal(special[1].background, special[2].background)
    assert.notEqual(special[0].background, special[1].background)
    for (const width of [320, 200, 140]) {
      await mount({ reset: true, row: true, width })
      const actual = await fixture.evaluate(host => {
        const row = host
          .querySelector('.dz-preference-row')
          .getBoundingClientRect()
        const title = host
          .querySelector('.dz-preference-row__text')
          .getBoundingClientRect()
        const boxes = [
          ...host.querySelectorAll('.dz-preference-themes__window'),
        ].map(node => {
          const box = node.getBoundingClientRect()
          return { left: box.left, right: box.right, top: box.top }
        })
        return { right: row.right, titleWidth: title.width, boxes }
      })
      assert.ok(actual.boxes.every(box => box.right <= actual.right + 0.5))
      assert.ok(actual.titleWidth > 20)
      if (width < 300) assert.ok(actual.boxes.at(-1).top > actual.boxes[0].top)
    }
  } finally {
    await browser.close()
    await server.close()
  }
})
