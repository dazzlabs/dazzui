import assert from 'node:assert/strict'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

test('window frames retain platform chrome, control policy and native button phases', async () => {
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    cacheDir: 'node_modules/.vite-test-window-frame',
    logLevel: 'silent',
    server: { host: '127.0.0.1', port: 0 },
    plugins: [
      {
        name: 'window-frame-fixture',
        resolveId: id =>
          id === '/window-frame-fixture.js'
            ? '\0window-frame-fixture'
            : undefined,
        load: id =>
          id === '\0window-frame-fixture'
            ? `
      import {createElement as h} from 'react'
      import {createRoot} from 'react-dom/client'
      import {flushSync} from 'react-dom'
      import {WindowFrame,WindowBody,WindowMain,WindowContent,WindowTitlebar,WindowFooter,SidebarProvider,Sidebar,Button} from '@dazzlabs/dazzui'
      const host=document.createElement('div');host.id='window-frame-fixture';host.style.cssText='position:fixed;left:24px;top:24px;width:800px';document.body.append(host)
      const root=createRoot(host);export const events=[]
      export function mount({platform='macos',controls,workspace=false,collapsed=false,icon=false,...props}={}) {
        const frame=h(WindowFrame,{platform,title:'Document',width:320,onClose:()=>events.push('close'),onMinimize:()=>events.push('minimize'),onMaximize:()=>events.push('maximize'),controls,...props},workspace?
          h(WindowBody,null,h(SidebarProvider,{open:!collapsed},h(Sidebar,{collapsible:icon?'icon':'offcanvas'}),h(WindowMain,null,h(WindowTitlebar,{title:'Main',leading:h(Button,null,'Leading')}),h(WindowContent,null,'Content'),h(WindowFooter,null,'Footer')))):
          h('div',{id:'frame-content',style:{height:80}},'Content'))
        flushSync(()=>root.render(h('div',null,frame,h(Button,null,'Next'))))
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
      await window.conformance.show('window-frame/preview', 'studio-light')
      document.getElementById('root').style.display = 'none'
      window.frameFixture = await import('/window-frame-fixture.js')
    })
    const fixture = page.locator('#window-frame-fixture')
    const mount = options =>
      page.evaluate(options => window.frameFixture.mount(options), options)
    for (const [platform, height, width] of [
      ['macos', 32, 14],
      ['macos15', 28, 12],
      ['windows', 32, 46],
      ['gnome', 47, 24],
      ['ubuntu', 47, 24],
      ['kde', 30, 18],
      ['omarchy', 0, 0],
    ]) {
      await mount({ platform })
      const actual = await fixture.evaluate(host => {
        const frame = host.querySelector('.dz-window-frame'),
          box = frame.getBoundingClientRect(),
          content = host
            .querySelector('#frame-content')
            .getBoundingClientRect(),
          close = host.querySelector('[aria-label="Close"]')
        return {
          contentTop: content.top - box.top,
          closeWidth: close?.getBoundingClientRect().width,
          controls: host.querySelectorAll('[data-control-kind]').length,
        }
      })
      assert.equal(
        actual.contentTop,
        height + (platform === 'omarchy' ? 22 : 0)
      )
      if (platform === 'omarchy') assert.equal(actual.controls, 0)
      else {
        assert.equal(actual.closeWidth, width)
        assert.equal(actual.controls, 3)
      }
    }
    for (const platform of ['macos', 'windows', 'gnome']) {
      await mount({ platform, controls: { close: true } })
      assert.equal(
        await fixture.locator('[data-control-kind]').count(),
        platform === 'macos' ? 3 : 1
      )
      if (platform === 'macos')
        assert.equal(await fixture.locator('button:disabled').count(), 2)
      await mount({
        platform,
        controls: { close: 'hidden', minimize: 'hidden', maximize: 'hidden' },
      })
      assert.equal(await fixture.locator('[data-control-kind]').count(), 0)
    }
    await mount({ platform: 'windows' })
    await fixture.getByRole('button', { name: 'Next', exact: true }).focus()
    await fixture.getByRole('button', { name: 'Close', exact: true }).click()
    await page.keyboard.down('Enter')
    await page.keyboard.up('Enter')
    await page.keyboard.press('Enter')
    await page.keyboard.down('Space')
    assert.deepEqual(await page.evaluate(() => window.frameFixture.events), [
      'close',
      'close',
      'close',
    ])
    await page.keyboard.up('Space')
    assert.deepEqual(await page.evaluate(() => window.frameFixture.events), [
      'close',
      'close',
      'close',
      'close',
    ])
    await page.keyboard.press('Tab')
    assert.equal(
      await fixture
        .getByRole('button', { name: 'Next', exact: true })
        .evaluate(node => node === document.activeElement),
      true
    )
    for (const platform of ['macos', 'windows', 'ubuntu', 'kde']) {
      await mount({
        platform,
        workspace: true,
        titlebar: false,
        fullSizeContent: true,
        width: 700,
        height: 220,
      })
      const frame = fixture.locator('.dz-window-frame')
      assert.equal(
        await frame
          .locator('.dz-window-frame__main [data-control-kind]')
          .count(),
        platform === 'macos' ? 0 : 3
      )
      assert.equal(
        await frame.locator('[aria-label="Close"]:visible').count(),
        1
      )
      if (platform === 'macos')
        for (const icon of [false, true]) {
          await mount({
            platform,
            workspace: true,
            titlebar: false,
            fullSizeContent: true,
            width: 700,
            height: 220,
            collapsed: true,
            icon,
          })
          assert.equal(
            await fixture
              .getByRole('button', { name: 'Leading', exact: true })
              .evaluate(node => node.getBoundingClientRect().left),
            104
          )
        }
    }
    await mount({
      platform: 'windows',
      workspace: true,
      titlebar: false,
      fullSizeContent: true,
      width: 700,
      height: 220,
      controls: { close: 'hidden', minimize: 'hidden', maximize: 'hidden' },
    })
    assert.equal(await fixture.locator('[data-control-kind]').count(), 0)
    // Remove only the collector's transition reset, then sample the actual
    // component's CSS transitions through the browser animation timeline.
    await page.evaluate(() => {
      for (const style of document.querySelectorAll('style[data-vite-dev-id]'))
        if (
          style.dataset.viteDevId.endsWith('/src/conformance/conformance.css')
        )
          style.remove()
    })
    await mount({ platform: 'macos' })
    const light = fixture.getByRole('button', { name: 'Close', exact: true })
    await light.focus()
    await page.keyboard.down('Space')
    const sample = () =>
      light.evaluate(async node => {
        await new Promise(requestAnimationFrame)
        const transition = node
          .getAnimations()
          .find(animation => animation.transitionProperty === 'transform')
        if (!transition) return null
        transition.pause()
        const duration = transition.effect.getTiming().duration
        transition.currentTime = duration === 90 ? 90 : 210
        return {
          duration,
          scale: new DOMMatrixReadOnly(getComputedStyle(node).transform).a,
        }
      })
    const down = await sample()
    assert.equal(down.duration, 90)
    assert.ok(Math.abs(down.scale - 0.8) < 0.001)
    await page.keyboard.up('Space')
    const up = await sample()
    assert.equal(up.duration, 420)
    assert.ok(up.scale > 1)
  } finally {
    await browser.close()
    await server.close()
  }
})
