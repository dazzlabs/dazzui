import assert from 'node:assert/strict'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

test('ordinary hints close on their child control activation phases', async () => {
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    cacheDir: 'node_modules/.vite-test-tooltip',
    logLevel: 'silent',
    server: { host: '127.0.0.1', port: 0 },
    plugins: [
      {
        name: 'tooltip-fixture',
        resolveId: id =>
          id === '/tooltip-fixture.js' ? '\0tooltip-fixture' : undefined,
        load: id =>
          id === '\0tooltip-fixture'
            ? `
        import {createElement as h} from 'react'
        import {createRoot} from 'react-dom/client'
        import {flushSync} from 'react-dom'
        import {Button, Toggle, Switch, Tooltip} from '@dazzlabs/dazzui'
        const host = document.createElement('div')
        host.id = 'tooltip-fixture'
        host.style.cssText = 'position:fixed;left:24px;top:64px;display:flex;gap:24px'
        document.body.append(host)
        const root = createRoot(host)
        export const changes = []
        export function mount({kind, preventDown, preventUp, noHandler}) {
          const change = noHandler ? undefined : () => changes.push('activate')
          const control = kind === 'switch'
            ? h(Switch, {'aria-label':'Target', onCheckedChange:change})
            : kind === 'toggle'
              ? h(Toggle, {onPressedChange:change}, 'Target')
              : h(Button, {onClick:change}, 'Target')
          flushSync(() => root.render(h('div', {
            style:{display:'flex',gap:24},
            onKeyDownCapture(event) { if(preventDown && ['Enter',' '].includes(event.key)) event.preventDefault() },
            onKeyUpCapture(event) { if(preventUp && event.key === ' ') event.preventDefault() },
          }, h(Tooltip, {label:'Hint', delay:0}, control), h(Button, null, 'Next'))))
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
      await window.conformance.show('tooltip/1', 'studio-light')
      document.getElementById('root').style.visibility = 'hidden'
      window.tooltipFixture = await import('/tooltip-fixture.js')
    })
    const fixture = page.locator('#tooltip-fixture')
    const hint = page.locator('.dz-tooltip')
    for (const kind of ['button', 'toggle', 'switch']) {
      const mount = options =>
        page.evaluate(options => window.tooltipFixture.mount(options), {
          kind,
          ...options,
        })
      await mount({})
      await page.evaluate(() => {
        window.tooltipFixture.changes.length = 0
      })
      const control = fixture.getByRole(
        kind === 'switch' ? 'switch' : 'button',
        { name: 'Target' }
      )
      const focus = async () => {
        await fixture.getByRole('button', { name: 'Next' }).focus()
        await page.keyboard.press('Shift+Tab')
        await hint.waitFor()
      }
      const count = async expected =>
        assert.equal(
          await page.evaluate(() => window.tooltipFixture.changes.length),
          expected,
          kind
        )
      await focus()
      await page.keyboard.up('Enter')
      await count(0)
      assert.equal(await hint.count(), 1, 'a stray Enter release is inert')
      await page.keyboard.up('Space')
      const stray = kind === 'switch' ? 1 : 0
      await count(stray)
      if (stray) {
        await hint.waitFor({ state: 'detached' })
        await focus()
      } else assert.equal(await hint.count(), 1)
      await page.keyboard.down('Enter')
      await count(stray + 1)
      await hint.waitFor({ state: 'detached' })
      await page.keyboard.up('Enter')
      await count(stray + 1)
      await focus()
      await page.keyboard.down('Space')
      await count(stray + 1)
      assert.equal(await hint.count(), 1)
      await page.keyboard.up('Space')
      await count(stray + 2)
      await hint.waitFor({ state: 'detached' })
      await focus()
      await mount({ preventDown: true })
      await page.keyboard.press('Enter')
      await count(stray + 2)
      assert.equal(await hint.count(), 1)
      await page.keyboard.press('Space')
      const base = stray + 2 + (kind === 'switch' ? 1 : 0)
      await count(base)
      if (kind === 'switch') {
        await hint.waitFor({ state: 'detached' })
        await focus()
      } else assert.equal(await hint.count(), 1)
      await mount({ preventUp: true })
      await page.keyboard.press('Space')
      await count(base)
      assert.equal(await hint.count(), 1)
      await mount({})
      await page.keyboard.up('Space')
      await count(base + 1)
      await hint.waitFor({ state: 'detached' })
      await mount({ noHandler: true })
      await focus()
      await page.keyboard.down('Enter')
      await hint.waitFor({ state: 'detached' })
      await count(base + 1)
      await page.keyboard.up('Enter')
      assert.ok(await control.evaluate(el => el === document.activeElement))
    }
  } finally {
    await browser.close()
    await server.close()
  }
})
