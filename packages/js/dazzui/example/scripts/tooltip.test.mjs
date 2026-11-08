import assert from 'node:assert/strict'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

test('tooltips follow activation, icon hints and nested ownership', async () => {
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
        import {Button, IconButton, Toggle, Switch, Tooltip, TooltipProvider, SidebarProvider} from '@dazzlabs/dazzui'
        const host = document.createElement('div')
        host.id = 'tooltip-fixture'
        host.style.cssText = 'position:fixed;left:24px;top:64px;display:flex;gap:24px'
        document.body.append(host)
        const root = createRoot(host)
        export const changes = []
        let groupKey = 0
        export function mountGroup({delay, closeDelay, ownDelay, ownClose, timeout, nested, sidebar, disabled, side, first = true, present = true, reset = false} = {}) {
          if(reset) groupKey++
          const hint = name => h(Tooltip, {key:name, label:name + ' hint', delay:ownDelay, closeDelay:ownClose, disabled, side}, h(Button, null, name))
          const third = nested ? h(TooltipProvider, {key:'nested'}, hint('Third')) : hint('Third')
          const row = present ? h('div', {style:{display:'flex', gap:60}}, first && hint('First'), hint('Second'), third) : null
          flushSync(() => root.render(sidebar
            ? h(SidebarProvider, {key:groupKey}, row)
            : h(TooltipProvider, {key:groupKey, delay, closeDelay, timeout}, row)))
        }
        export function mount({kind, preventDown, preventUp, noHandler, nested, innerDisabled, richLabel, disabled}) {
          const change = noHandler ? undefined : () => changes.push('activate')
          let control = kind === 'icon'
            ? h(IconButton, {label:'Target', disabled, onClick:change}, 'X')
            : kind === 'switch'
            ? h(Switch, {'aria-label':'Target', onCheckedChange:change})
            : kind === 'toggle'
              ? h(Toggle, {onPressedChange:change}, 'Target')
              : h(Button, {onClick:change}, 'Target')
          if(nested) control = h(Tooltip, {label:'Inner', delay:0, disabled:innerDisabled}, control)
          const label = richLabel ? h('span', null, 'Hint', h(Tooltip, {label:'Label hint'}, h('span', null, ' detail'))) : 'Hint'
          flushSync(() => root.render(h('div', {
            style:{display:'flex',gap:24},
            onKeyDownCapture(event) { if(preventDown && ['Enter',' '].includes(event.key)) event.preventDefault() },
            onKeyUpCapture(event) { if(preventUp && event.key === ' ') event.preventDefault() },
          }, h(Tooltip, {label, delay:0}, control), h(Button, null, 'Next'))))
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
    const mount = async options => {
      await page.evaluate(options => window.tooltipFixture.mount(options), {
        kind: 'button',
        ...options,
      })
      await page.evaluate(
        () =>
          new Promise(resolve =>
            requestAnimationFrame(() => requestAnimationFrame(resolve))
          )
      )
    }
    const focus = async () => {
      await fixture.getByRole('button', { name: 'Next' }).focus()
      await page.keyboard.press('Shift+Tab')
    }
    for (const [options, label] of [
      [{ nested: true }, 'Inner'],
      [{ nested: true, innerDisabled: true }, null],
      [{}, 'Hint'],
      [{ richLabel: true }, 'Hint detail'],
      [{ kind: 'icon' }, 'Target'],
      [{ kind: 'icon', nested: true }, 'Target'],
    ]) {
      await mount(options)
      await focus()
      if (label) {
        await page.waitForFunction(label => {
          const nodes = [...document.querySelectorAll('.dz-tooltip')]
          return nodes.length === 1 && nodes[0].textContent === label
        }, label)
        assert.deepEqual(await hint.allTextContents(), [label])
      } else await hint.waitFor({ state: 'detached' })
    }
    await mount({ kind: 'icon' })
    await fixture.getByRole('button', { name: 'Next' }).focus()
    await page.mouse.move(0, 0)
    const icon = fixture.getByRole('button', { name: 'Target' })
    const before = await icon.boundingBox()
    await icon.hover()
    await hint.waitFor()
    assert.deepEqual(await hint.allTextContents(), ['Target'])
    const popup = await hint.boundingBox()
    assert.equal(Math.round(before.y - popup.y - popup.height), 4)
    assert.ok(
      Math.abs(popup.x + popup.width / 2 - before.x - before.width / 2) <= 0.5
    )
    await page.mouse.move(0, 0)
    await hint.waitFor({ state: 'detached' })
    await focus()
    await hint.waitFor()
    await page.keyboard.down('Enter')
    await hint.waitFor({ state: 'detached' })
    await page.keyboard.up('Enter')
    await page.mouse.move(0, 0)
    await mount({ kind: 'icon', disabled: true })
    await icon.hover()
    await page.waitForTimeout(650)
    assert.equal(await hint.count(), 0)

    const mountGroup = async options => {
      await page.mouse.move(0, 0)
      await page.evaluate(
        options => window.tooltipFixture.mountGroup(options),
        {
          reset: true,
          ...options,
        }
      )
      await page.evaluate(
        () =>
          new Promise(resolve =>
            requestAnimationFrame(() => requestAnimationFrame(resolve))
          )
      )
    }
    const first = fixture.getByRole('button', { name: 'First', exact: true })
    const second = fixture.getByRole('button', { name: 'Second', exact: true })
    const third = fixture.getByRole('button', { name: 'Third', exact: true })
    const visibleHint = () => hint.filter({ visible: true })
    await mountGroup({})
    await first.hover()
    await page.waitForTimeout(150)
    assert.equal(await visibleHint().count(), 0)
    await visibleHint().waitFor()
    assert.equal(await visibleHint().textContent(), 'First hint')
    await second.hover()
    await page.waitForTimeout(50)
    assert.equal(await visibleHint().textContent(), 'Second hint')
    await page.mouse.move(0, 0)
    await visibleHint().waitFor({ state: 'hidden' })
    await page.waitForTimeout(100)
    await third.hover()
    await page.waitForTimeout(50)
    assert.equal(await visibleHint().textContent(), 'Third hint')
    await page.mouse.move(0, 0)
    await visibleHint().waitFor({ state: 'hidden' })
    await page.waitForTimeout(450)
    await first.hover()
    await page.waitForTimeout(150)
    assert.equal(await visibleHint().count(), 0)

    await mountGroup({ closeDelay: 300 })
    await first.focus()
    await visibleHint().waitFor()
    await second.hover()
    await page.waitForTimeout(50)
    assert.equal(await visibleHint().textContent(), 'Second hint')
    await page.mouse.move(0, 0)
    await page.waitForTimeout(100)
    assert.equal(await visibleHint().textContent(), 'Second hint')
    await visibleHint().waitFor({ state: 'hidden' })

    await mountGroup({ nested: true })
    await first.focus()
    await visibleHint().waitFor()
    await third.hover()
    await page.waitForTimeout(150)
    assert.equal(
      await hint.filter({ hasText: 'Third hint', visible: true }).count(),
      0
    )
    await page.mouse.move(0, 0)
    // An open member unmounts without warming its siblings.
    await page.evaluate(() =>
      window.tooltipFixture.mountGroup({ first: false, nested: true })
    )
    await second.hover()
    await page.waitForTimeout(150)
    assert.equal(await visibleHint().count(), 0)

    await mountGroup({
      delay: 300,
      ownDelay: 100,
      closeDelay: 300,
      ownClose: 100,
      timeout: 0,
    })
    await first.hover()
    await page.waitForTimeout(150)
    assert.equal(await visibleHint().textContent(), 'First hint')
    await page.mouse.move(0, 0)
    await page.waitForTimeout(150)
    assert.equal(await visibleHint().count(), 0)
    await second.hover()
    // The explicit child delay applies again after timeout=0 resets the group.
    assert.equal(await visibleHint().count(), 0)
    await visibleHint().waitFor()
    await mountGroup({ delay: 0, ownDelay: 900 })
    await first.hover()
    await page.waitForTimeout(50)
    assert.equal(await visibleHint().textContent(), 'First hint')
    await mountGroup({ sidebar: true, ownDelay: 900 })
    await first.hover()
    await page.waitForTimeout(50)
    assert.equal(await visibleHint().textContent(), 'First hint')

    await mountGroup({ delay: 300, timeout: 0 })
    const restTarget = await first.boundingBox()
    const restAt = {
      x: restTarget.x + restTarget.width / 2,
      y: restTarget.y + restTarget.height / 2,
    }
    await page.mouse.move(restAt.x, restAt.y)
    await page.waitForTimeout(150)
    await page.mouse.move(restAt.x + 8, restAt.y)
    await page.waitForTimeout(180)
    assert.equal(
      await visibleHint().count(),
      0,
      'movement restarts the rest delay'
    )
    await page.mouse.move(restAt.x + 9, restAt.y)
    await page.waitForTimeout(150)
    assert.equal(
      await visibleHint().textContent(),
      'First hint',
      'one-pixel jitter preserves the timer'
    )
    await page.mouse.move(restAt.x - 8, restAt.y)
    assert.equal(
      await visibleHint().textContent(),
      'First hint',
      'movement keeps an open hint'
    )
    await page.keyboard.press('Escape')
    await visibleHint().waitFor({ state: 'hidden' })
    await page.mouse.move(restAt.x, restAt.y)
    await page.waitForTimeout(350)
    assert.equal(
      await visibleHint().count(),
      0,
      'dismissal blocks movement until re-entry'
    )
    await page.mouse.move(0, 0)
    await first.hover()
    await visibleHint().waitFor()

    await mountGroup({ delay: 300, timeout: 0, present: false })
    await page.mouse.move(restAt.x, restAt.y)
    await page.evaluate(() =>
      window.tooltipFixture.mountGroup({ delay: 300, timeout: 0 })
    )
    await page.waitForTimeout(450)
    assert.equal(
      await visibleHint().count(),
      0,
      'mounting under a resting pointer does not start the timer'
    )
    await page.mouse.move(restAt.x + 1, restAt.y)
    await visibleHint().waitFor()

    await mountGroup({ delay: 300, timeout: 0 })
    await first.hover()
    await page.waitForTimeout(100)
    await page.mouse.down()
    await page.mouse.up()
    await page.mouse.move(restAt.x + 8, restAt.y)
    await page.waitForTimeout(350)
    assert.equal(
      await visibleHint().count(),
      0,
      'activation cancels the pending rest timer'
    )

    await mountGroup({ delay: 300, timeout: 0, disabled: true })
    await first.hover()
    await page.waitForTimeout(350)
    assert.equal(await visibleHint().count(), 0)
    await page.evaluate(() =>
      window.tooltipFixture.mountGroup({ delay: 300, timeout: 0 })
    )
    await page.waitForTimeout(350)
    assert.equal(await visibleHint().count(), 0)
    await page.mouse.move(restAt.x + 8, restAt.y)
    await page.waitForTimeout(350)
    assert.equal(
      await visibleHint().count(),
      0,
      'enabling without re-entry preserves the blocked state'
    )
    await page.mouse.move(0, 0)
    await first.hover()
    await visibleHint().waitFor()
    await page.evaluate(() =>
      window.tooltipFixture.mountGroup({
        delay: 300,
        timeout: 0,
        disabled: true,
      })
    )
    await visibleHint().waitFor({ state: 'hidden' })
    await page.evaluate(() =>
      window.tooltipFixture.mountGroup({ delay: 300, timeout: 0 })
    )
    await page.mouse.move(restAt.x - 8, restAt.y)
    await page.waitForTimeout(350)
    assert.equal(
      await visibleHint().count(),
      0,
      'disabling an open hint keeps its dismissal blocked'
    )

    for (const side of ['top', 'bottom', 'left', 'right']) {
      await mountGroup({ delay: 0, side })
      await second.hover()
      await visibleHint().waitFor()
      const from = await second.boundingBox()
      const to = await visibleHint().boundingBox()
      const placed = await visibleHint().locator('..').getAttribute('data-side')
      assert.equal(placed, side)
      const gap =
        placed === 'top'
          ? { x: from.x + from.width / 2, y: (from.y + to.y + to.height) / 2 }
          : placed === 'bottom'
            ? {
                x: from.x + from.width / 2,
                y: (from.y + from.height + to.y) / 2,
              }
            : placed === 'left'
              ? {
                  x: (from.x + to.x + to.width) / 2,
                  y: from.y + from.height / 2,
                }
              : {
                  x: (from.x + from.width + to.x) / 2,
                  y: from.y + from.height / 2,
                }
      await page.mouse.move(gap.x, gap.y)
      await page.waitForTimeout(80)
      assert.equal(
        await visibleHint().count(),
        1,
        side + ' gap remains traversable'
      )
      await page.mouse.move(to.x + to.width / 2, to.y + to.height / 2)
      await page.waitForTimeout(80)
      assert.equal(
        await visibleHint().count(),
        1,
        side + ' popup stays hoverable'
      )
      await page.mouse.move(0, 0)
      await visibleHint().waitFor({ state: 'hidden' })
    }

    await mountGroup({ delay: 0, closeDelay: 150 })
    await first.hover()
    await visibleHint().waitFor()
    const delayedPopup = await visibleHint().boundingBox()
    await page.mouse.move(
      delayedPopup.x + delayedPopup.width / 2,
      delayedPopup.y + delayedPopup.height / 2
    )
    await page.mouse.move(0, 0)
    await page.waitForTimeout(50)
    assert.equal(await visibleHint().count(), 1)
    await page.mouse.move(
      delayedPopup.x + delayedPopup.width / 2,
      delayedPopup.y + delayedPopup.height / 2
    )
    await page.waitForTimeout(180)
    assert.equal(
      await visibleHint().count(),
      1,
      're-entering the popup cancels its pending close'
    )
    await page.mouse.move(0, 0)
    await visibleHint().waitFor({ state: 'hidden' })

    await mountGroup({ delay: 0, side: 'top' })
    await second.hover()
    await visibleHint().waitFor()
    const intentTarget = await second.boundingBox()
    await page.mouse.move(
      intentTarget.x + intentTarget.width + 1,
      intentTarget.y + 1
    )
    assert.equal(
      await visibleHint().count(),
      1,
      'diagonal travel initially preserves pointer intent'
    )
    await page.waitForTimeout(80)
    assert.equal(
      await visibleHint().count(),
      0,
      'resting outside the corridor expires pointer intent'
    )
  } finally {
    await browser.close()
    await server.close()
  }
})
