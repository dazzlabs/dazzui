import assert from 'node:assert/strict'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

test('sidebar surfaces and independent collapse preserve both edge layouts', async () => {
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
        import {SidebarProvider, Sidebar, SidebarInset} from '@dazzlabs/dazzui'
        const host = document.createElement('div')
        host.id = 'sidebar-fixture'
        host.style.cssText = 'position:fixed;left:24px;top:24px;width:420px;height:180px;display:flex'
        document.body.append(host)
        const root = createRoot(host)
        export function mount({open, ...props}) {
          flushSync(() => root.render(h(SidebarProvider, {open},
            h(Sidebar, {...props, width:'172px'}, h('div', null, 'Column')),
            h(SidebarInset, null, 'Content'))))
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
  } finally {
    await browser.close()
    await server.close()
  }
})
