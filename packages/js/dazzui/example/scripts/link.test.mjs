import assert from 'node:assert/strict'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

test('links activate Enter on down, leave Space to the page and honor canceled down', async () => {
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    cacheDir: 'node_modules/.vite-test-link',
    logLevel: 'silent',
    server: { host: '127.0.0.1', port: 0 },
    plugins: [
      {
        name: 'link-fixture',
        resolveId: id =>
          id === '/link-fixture.js' ? '\0link-fixture' : undefined,
        load: id =>
          id === '\0link-fixture'
            ? `
        import {createElement as h} from 'react'
        import {createRoot} from 'react-dom/client'
        import {flushSync} from 'react-dom'
        import {Link,Button} from '@dazzlabs/dazzui'
        const host=document.createElement('div');host.id='link-fixture';document.body.append(host)
        const root=createRoot(host);export const events=[];let phase=''
        for(const type of ['keydown','keyup'])document.addEventListener(type,event=>phase=type+':'+event.key,true)
        export function mount({preventDown=false,preventUp=false,href='#/read'}={}) {
          events.length=0
          flushSync(()=>root.render(h('div',{
            onKeyDownCapture:event=>{if(preventDown&&event.key==='Enter')event.preventDefault()},
            onKeyUpCapture:event=>{if(preventUp&&event.key==='Enter')event.preventDefault()}
          },h(Link,{href,onClick:event=>{event.preventDefault();events.push({target:'link',phase,detail:event.detail})}},'Read'),
          h(Button,{onClick:()=>events.push({target:'button',phase})},'Next'))))
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
      await window.conformance.show('link/preview', 'studio-light')
      document.getElementById('root').style.display = 'none'
      window.linkFixture = await import('/link-fixture.js')
    })
    const link = page.locator('#link-fixture a'),
      next = page.locator('#link-fixture button')
    const mount = async props => {
      await page.evaluate(props => window.linkFixture.mount(props), props)
      await link.focus()
    }
    const events = () => page.evaluate(() => window.linkFixture.events)
    await mount()
    for (const modifier of [null, 'Shift', 'Control', 'Alt', 'Meta']) {
      const before = (await events()).length
      if (modifier) await page.keyboard.down(modifier)
      await page.keyboard.down('Enter')
      assert.equal(
        (await events()).length,
        before + 1,
        `${modifier || ''} Enter activates on down`
      )
      await page.keyboard.up('Enter')
      if (modifier) await page.keyboard.up(modifier)
      assert.equal((await events()).length, before + 1)
    }
    await page.keyboard.down('Enter')
    await page.keyboard.down('Enter')
    await page.keyboard.up('Enter')
    assert.equal(
      (await events()).length,
      6,
      'held Enter does not repeat link activation'
    )
    assert(
      (await events()).every(e => e.phase === 'keydown:Enter' && e.detail === 0)
    )
    await mount()
    await page.keyboard.up('Space')
    await page.keyboard.down('Space')
    await page.keyboard.down('Space')
    await page.keyboard.up('Space')
    assert.equal((await events()).length, 0)
    await page.keyboard.down('Space')
    await page.keyboard.press('Tab')
    await page.keyboard.up('Space')
    assert.equal((await events()).length, 0)
    assert(await next.evaluate(node => node === document.activeElement))
    await page.keyboard.press('Enter')
    assert.equal((await events())[0].target, 'button')
    await mount({ preventDown: true })
    await page.keyboard.press('Enter')
    assert.equal((await events()).length, 0)
    await mount({ preventUp: true })
    await page.keyboard.down('Enter')
    assert.equal((await events()).length, 1)
    await page.keyboard.up('Enter')
    assert.equal((await events()).length, 1)
    await link.click()
    assert.equal((await events()).length, 2, 'pointer click remains available')
    await mount({ href: null })
    assert.equal(
      await link.evaluate(node => node === document.activeElement),
      false
    )
    await next.focus()
    await page.keyboard.press('Shift+Tab')
    assert.equal(
      await link.evaluate(node => node === document.activeElement),
      false
    )
    await link.click()
    assert.equal(
      (await events()).length,
      1,
      'an anchor without href still receives pointer clicks'
    )
  } finally {
    await browser.close()
    await server.close()
  }
})
