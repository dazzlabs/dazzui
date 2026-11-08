import assert from 'node:assert/strict'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

test('sidebar geometry, hints and keyboard activation follow React', async () => {
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    cacheDir: 'node_modules/.vite-test-sidebar',
    logLevel: 'silent',
    server: { host: '127.0.0.1', port: 0 },
    plugins: [
      {
        name: 'sidebar-fixture',
        resolveId: id =>
          id === '/sidebar-fixture.js' ? '\0sidebar-fixture' : undefined,
        load: id =>
          id === '\0sidebar-fixture'
            ? `
        import {createElement as h} from 'react'
        import {createRoot} from 'react-dom/client'
        import {flushSync} from 'react-dom'
        import {SidebarProvider, Sidebar, SidebarInset, SidebarMenu, SidebarMenuItem, SidebarMenuButton, Button} from '@dazzlabs/dazzui'
        const host = document.createElement('div')
        host.id = 'sidebar-fixture'
        host.style.cssText = 'position:fixed;left:24px;top:24px;width:420px;height:180px;display:flex'
        document.body.append(host)
        const root = createRoot(host)
        export const changes = []
        export const nextChanges = []
        export function mount({open, rowDisabled = false, hint = true, preventDown = false, preventUp = false, ...props}) {
          flushSync(() => root.render(h(SidebarProvider, {open,
            onKeyDownCapture(event) { if (preventDown && ['Enter', ' '].includes(event.key)) event.preventDefault() },
            onKeyUpCapture(event) { if (preventUp && event.key === ' ') event.preventDefault() },
          },
            h(Sidebar, {...props, width:'172px'},
              h(SidebarMenu, null, h(SidebarMenuItem, null,
                h(SidebarMenuButton, {tooltip:hint ? 'Documents' : undefined, disabled:rowDisabled, onClick() { changes.push('click') }}, 'Documents')))),
            h(SidebarInset, null, h(Button, {onClick() { nextChanges.push('click') }}, 'Next')))))
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
      window.sidebarFixture = await import('/sidebar-fixture.js')
      document.getElementById('root').style.visibility = 'hidden'
    })
    for (const theme of ['bright-light', 'studio-dark']) {
      await page.evaluate(
        theme => window.conformance.show('sidebar/2', theme),
        theme
      )
      for (const variant of ['sidebar', 'floating', 'inset'])
        for (const collapsible of ['none', 'icon', 'offcanvas'])
          for (const side of ['left', 'right'])
            for (const open of [true, false])
              for (const collapsed of [null, true, false]) {
                const options = { variant, collapsible, side, open, collapsed }
                if (collapsed === null) delete options.collapsed
                const actual = await page.evaluate(options => {
                  window.sidebarFixture.mount(options)
                  const host = document.querySelector('#sidebar-fixture')
                  const provider = host.querySelector('.dz-sidebar-wrapper')
                  const sidebar = host.querySelector('.dz-sidebar')
                  const container = host.querySelector('.dz-sidebar__container')
                  const inset = host.querySelector('.dz-sidebar-inset')
                  const box = node => {
                    const { x, y, width, height } = node.getBoundingClientRect()
                    return [x, y, width, height]
                  }
                  const column = getComputedStyle(provider)
                    .getPropertyValue('--color-surface-column')
                    .trim()
                  const swatch = document.createElement('div')
                  swatch.style.backgroundColor = column
                  provider.append(swatch)
                  const ground = getComputedStyle(swatch).backgroundColor
                  swatch.remove()
                  return {
                    sidebar: box(sidebar),
                    container: container && box(container),
                    inset: box(inset),
                    ground,
                    background: getComputedStyle(provider).backgroundColor,
                    rootPadding: getComputedStyle(sidebar).padding,
                    containerPadding:
                      container && getComputedStyle(container).padding,
                    corners: getComputedStyle(inset).borderTopLeftRadius,
                    shadow: getComputedStyle(inset).boxShadow,
                  }
                }, options)
                const away = collapsible !== 'none' && (collapsed ?? !open)
                const width = !away
                  ? 172
                  : collapsible === 'offcanvas'
                    ? 0
                    : variant === 'sidebar'
                      ? 48
                      : 64
                const raised = variant === 'inset'
                const lead = raised && away ? 8 : 0
                const label = JSON.stringify({ theme, ...options })
                assert.deepEqual(actual.sidebar, [24, 24, width, 180], label)
                assert.equal(actual.rootPadding, '0px', label)
                assert.deepEqual(
                  actual.inset,
                  [
                    24 + width + lead,
                    raised ? 32 : 24,
                    420 - width - lead - (raised ? 8 : 0),
                    raised ? 164 : 180,
                  ],
                  label
                )
                assert.equal(
                  actual.background,
                  raised ? actual.ground : 'rgba(0, 0, 0, 0)',
                  label
                )
                assert.equal(actual.corners, raised ? '12px' : '0px', label)
                assert.equal(actual.shadow === 'none', !raised, label)
                if (
                  (collapsible !== 'none' && !away) ||
                  collapsible === 'icon'
                ) {
                  const containerWidth =
                    width + (away && variant !== 'sidebar' ? 2 : 0)
                  assert.deepEqual(
                    actual.container,
                    [
                      24 - (side === 'right' ? containerWidth - width : 0),
                      24,
                      containerWidth,
                      180,
                    ],
                    label
                  )
                  assert.equal(
                    actual.containerPadding,
                    variant === 'sidebar' ? '0px' : '8px',
                    label
                  )
                }
              }
    }
    const row = page
      .locator('#sidebar-fixture')
      .getByRole('button', { name: 'Documents' })
    const hint = page.locator('.dz-tooltip')
    const mount = options =>
      page.evaluate(
        options =>
          window.sidebarFixture.mount({
            side: 'left',
            variant: 'sidebar',
            collapsible: 'icon',
            ...options,
          }),
        options
      )
    for (const [open, collapsed, enabled] of [
      [false, undefined, true],
      [true, undefined, false],
      [true, true, false],
      [false, false, true],
    ]) {
      await page.mouse.move(0, 0)
      await mount({ open, collapsed })
      const before = await row.boundingBox()
      await row.hover()
      if (!enabled) {
        assert.equal(await hint.count(), 0)
        continue
      }
      await hint.waitFor({ timeout: 2000 })
      const popup = await hint.boundingBox()
      assert.deepEqual(
        await row.boundingBox(),
        before,
        'a hint does not change its trigger box'
      )
      assert.equal(popup.x - before.x - before.width, 4)
      assert.ok(
        Math.abs(popup.y + popup.height / 2 - before.y - before.height / 2) <=
          0.5,
        JSON.stringify({ before, popup })
      )
      await page.mouse.move(0, 0)
      await hint.waitFor({ state: 'detached' })
    }
    await mount({ open: false })
    await row.focus()
    await hint.waitFor()
    await page.keyboard.press('Escape')
    await hint.waitFor({ state: 'detached' })
    assert.ok(await row.evaluate(el => el === document.activeElement))
    await page.keyboard.press('Tab')
    await page.keyboard.press('Shift+Tab')
    await hint.waitFor()
    await row.click()
    await hint.waitFor({ state: 'detached' })
    await page.mouse.move(0, 0)
    await row.hover()
    await hint.waitFor()
    await mount({ open: false, preventUp: true })
    await page
      .locator('#sidebar-fixture')
      .getByRole('button', { name: 'Next' })
      .focus()
    await page.keyboard.down('Space')
    await page.keyboard.up('Space')
    const blockedNext = await page
      .locator('#sidebar-fixture')
      .getByRole('button', { name: 'Next' })
      .evaluate(el => ({
        count: window.sidebarFixture.nextChanges.length,
        active: el.matches(':active'),
      }))
    await mount({ open: false })
    await page.keyboard.up('Space')
    assert.deepEqual(blockedNext, { count: 0, active: true })
    assert.equal(
      await page.evaluate(() => window.sidebarFixture.nextChanges.length),
      1
    )
    for (const enabledHint of [true, false]) {
      await page.mouse.move(0, 0)
      await mount({ open: false, hint: enabledHint })
      await page.evaluate(() => {
        window.sidebarFixture.changes.length = 0
      })
      const next = page
        .locator('#sidebar-fixture')
        .getByRole('button', { name: 'Next' })
      const focus = async () => {
        await next.focus()
        await page.keyboard.press('Shift+Tab')
        if (enabledHint) await hint.waitFor()
      }
      const count = async expected =>
        assert.equal(
          await page.evaluate(() => window.sidebarFixture.changes.length),
          expected
        )
      await focus()
      await page.keyboard.up('Enter')
      await page.keyboard.up('Space')
      await count(0)
      await page.keyboard.down('Enter')
      await count(1)
      if (enabledHint) await hint.waitFor({ state: 'detached' })
      await page.keyboard.down('Enter')
      await count(2)
      await page.keyboard.up('Enter')
      await count(2)
      await focus()
      await page.keyboard.down('Space')
      await page.keyboard.down('Space')
      await count(2)
      if (enabledHint) assert.equal(await hint.count(), 1)
      await page.keyboard.up('Space')
      await count(3)
      if (enabledHint) await hint.waitFor({ state: 'detached' })
      await page.keyboard.up('Space')
      await count(3)
      await focus()
      await mount({ open: false, hint: enabledHint, preventDown: true })
      await page.keyboard.press('Enter')
      await page.keyboard.press('Space')
      await count(3)
      if (enabledHint) assert.equal(await hint.count(), 1)
      await mount({ open: false, hint: enabledHint, preventUp: true })
      await page.keyboard.press('Space')
      await count(3)
      if (enabledHint) assert.equal(await hint.count(), 1)
      await mount({ open: false, hint: enabledHint })
      await page.keyboard.up('Space')
      await count(4)
      if (enabledHint) await hint.waitFor({ state: 'detached' })
      await page.keyboard.down('Space')
      await page.keyboard.press('Tab')
      await page.keyboard.up('Space')
      await focus()
      await page.keyboard.up('Space')
      await count(4)
      await page.keyboard.down('Space')
      await mount({ open: false, hint: enabledHint, rowDisabled: true })
      await page.keyboard.up('Space')
      await mount({ open: false, hint: enabledHint })
      await focus()
      await page.keyboard.up('Space')
      await count(4)
      await row.click()
      await count(5)
    }
    // Disabling cancels the hint; enabling alone does not reopen it.
    await mount({ open: true })
    await hint.waitFor({ state: 'detached' })
    await mount({ open: false })
    assert.equal(await hint.count(), 0)
    await page.mouse.move(0, 0)
    await row.hover()
    await hint.waitFor()
  } finally {
    await browser.close()
    await server.close()
  }
})
