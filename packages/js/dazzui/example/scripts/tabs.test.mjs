import assert from 'node:assert/strict'
import fs from 'node:fs'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

test('navigation tabs measure, promote, close and select through their real controls', async () => {
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    cacheDir: 'node_modules/.vite-test-tabs',
    logLevel: 'silent',
    server: { host: '127.0.0.1', port: 0 },
    plugins: [
      {
        name: 'tabs-fixture',
        resolveId: id =>
          id === '/tabs-fixture.js' ? '\0tabs-fixture' : undefined,
        load: id =>
          id === '\0tabs-fixture'
            ? `
        import {createElement as h, useState} from 'react'
        import {createRoot} from 'react-dom/client'
        import {flushSync} from 'react-dom'
        import {Search16Regular, MoreHorizontalRegular} from '@fluentui/react-icons'
        import {Tabs, IconButton, Icon} from '@dazzlabs/dazzui'
        const host = document.createElement('div')
        host.id = 'tabs-fixture'
        host.style.cssText = 'position:fixed;left:24px;top:24px;display:flex;height:35px'
        document.body.append(host)
        const root = createRoot(host)
        let sequence = 0
        export const changes = []
        export const closes = []
        function Fixture({options}) {
          const [value, setValue] = useState(options.value ?? '0')
          const items = ['Pull Requests', 'Issues', 'Actions', 'Releases', 'History'].map((label, index) => ({
            value: String(index), label, icon: Search16Regular,
            disabled: index === 2, closable: options.documents,
            count: options.metadata ? [2,4,undefined,12,0][index] : undefined,
            detail: options.metadata && index === 3 ? 'Mac Studio' : undefined,
            status: options.documents ? 'success' : undefined, busy: options.busy,
          }))
          return h(Tabs, {items, value,
            onValueChange(next) {changes.push(next); if (options.accept !== false) setValue(next)},
            onClose: options.close === false ? undefined : next => closes.push(next),
            trailing: options.documents ? h(IconButton, {label:'New', size:'tiny', tint:'neutral', variant:'plain'},
              h(Icon, {icon:MoreHorizontalRegular, size:14})) : undefined,
          })
        }
        export function mount(options) {
          changes.length = 0
          closes.length = 0
          host.style.width = options.width + 'px'
          flushSync(() => root.render(h(Fixture, {key:++sequence, options})))
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
    const page = await browser.newPage({
      reducedMotion: 'reduce',
      deviceScaleFactor: 2,
    })
    await page.goto(
      new URL('conformance.html', server.resolvedUrls.local[0]).href
    )
    await page.waitForFunction(() => window.conformance)
    const declared = await page.evaluate(async () => {
      const preview = await window.conformance.show(
        'tabs/preview',
        'studio-light'
      )
      const documents = await window.conformance.show('tabs/3', 'studio-light')
      return [preview, documents].map(scene =>
        scene.specimens.map(specimen => [
          specimen.component,
          Boolean(specimen.node ?? specimen.reading),
        ])
      )
    })
    assert.deepEqual(declared, [
      [['Tabs', true]],
      [
        ['Tabs', true],
        ['IconButton', true],
        ['Icon', true],
      ],
    ])
    await page.evaluate(async () => {
      window.tabsFixture = await import('/tabs-fixture.js')
    })
    const mount = async options => {
      await page.evaluate(async options => {
        window.tabsFixture.mount(options)
        await document.fonts.ready
        await new Promise(requestAnimationFrame)
        await new Promise(requestAnimationFrame)
      }, options)
    }
    const fixture = page.locator('#tabs-fixture')
    const read = () =>
      fixture.evaluate(host => {
        const root = host.querySelector('.dz-tabs')
        const box = root.getBoundingClientRect()
        const rect = el => {
          const r = el.getBoundingClientRect()
          return {
            x: r.x - box.x,
            y: r.y - box.y,
            width: r.width,
            height: r.height,
          }
        }
        return {
          width: box.width,
          height: box.height,
          basis: parseFloat(root.style.flexBasis),
          floor: parseFloat(root.style.minWidth),
          whole: [...root.querySelectorAll('[data-ruler="whole"]')].map(
            el => el.offsetWidth
          ),
          compact: [...root.querySelectorAll('[data-ruler="compact"]')].map(
            el => el.offsetWidth
          ),
          buttons: [...root.querySelectorAll('[role="tab"]')].map(el => ({
            value: el.dataset.tab,
            ...rect(el),
            weight: Number(getComputedStyle(el).fontWeight),
            ink: getComputedStyle(el).color,
            label: el.querySelector('.dz-tabs__label')?.textContent ?? null,
            selected: el.getAttribute('aria-selected') === 'true',
            indicator: getComputedStyle(el.parentElement, '::after')
              .backgroundColor,
          })),
          close: [...root.querySelectorAll('.dz-tabs__close')].map(el => ({
            ...rect(el),
            visibility: getComputedStyle(el).visibility,
          })),
          more: root.querySelector('.dz-tabs__more')
            ? rect(root.querySelector('.dz-tabs__more'))
            : null,
          trailing: root.querySelector('.dz-tabs__trailing')
            ? rect(root.querySelector('.dz-tabs__trailing'))
            : null,
        }
      })
    const readings = []
    for (const theme of await page.evaluate(() => window.conformance.drawn)) {
      await page.evaluate(
        theme => window.conformance.show('checkbox/preview', theme),
        theme
      )
      for (const options of [
        { width: 700 },
        { width: 340 },
        { width: 180 },
        { width: 180, value: '4' },
        { width: 900, documents: true },
        { width: 260, documents: true, value: '4' },
        { width: 700, metadata: true },
        { width: 220, metadata: true, value: '3' },
      ]) {
        await mount(options)
        const actual = await read()
        assert.equal(actual.buttons.filter(button => button.selected).length, 1)
        const active = actual.buttons.find(button => button.selected)
        assert.notEqual(active.label, null)
        assert.notEqual(
          active.indicator,
          'rgba(0, 0, 0, 0)',
          'Tabs binds its own primary recipe'
        )
        assert.ok(actual.width >= actual.floor)
        if (options.width === 700) assert.equal(actual.buttons.length, 5)
        if (options.width === 180) {
          assert.ok(actual.more)
          assert.ok(actual.buttons.length < 5)
          if (options.value === '4')
            assert.equal(actual.buttons.at(-1).value, '4')
        }
        readings.push({ theme, options, ...actual })
      }
    }
    await mount({ width: 700 })
    await fixture.getByRole('tab').first().focus()
    for (const key of ['ArrowRight', 'ArrowRight', 'End', 'Home'])
      await page.keyboard.press(key)
    assert.deepEqual(await page.evaluate(() => window.tabsFixture.changes), [
      '1',
      '3',
      '4',
      '0',
    ])
    await mount({ width: 700, accept: false })
    await fixture.getByRole('tab').first().focus()
    await page.keyboard.press('ArrowRight')
    await page.keyboard.press('ArrowRight')
    assert.deepEqual(await page.evaluate(() => window.tabsFixture.changes), [
      '1',
      '1',
    ])
    await mount({ width: 900, documents: true })
    const close = fixture.getByRole('button', { name: 'Close Pull Requests' })
    await close.focus()
    await page.keyboard.down('Space')
    await page.keyboard.down('Space')
    assert.deepEqual(await page.evaluate(() => window.tabsFixture.closes), [])
    await page.keyboard.up('Space')
    await page.keyboard.down('Enter')
    await page.keyboard.down('Enter')
    await page.keyboard.up('Enter')
    assert.deepEqual(await page.evaluate(() => window.tabsFixture.closes), [
      '0',
      '0',
      '0',
    ])
    assert.deepEqual(await page.evaluate(() => window.tabsFixture.changes), [])
    await mount({ width: 180 })
    await fixture.getByRole('button', { name: / more$/ }).click()
    await page.getByRole('menuitem', { name: 'History', exact: true }).click()
    assert.equal(
      await fixture
        .getByRole('tab', { selected: true })
        .getAttribute('data-tab'),
      '4'
    )
    await page.emulateMedia({ reducedMotion: 'no-preference' })
    await mount({ width: 900, documents: true, busy: true })
    const pulse = []
    for (const time of [0, 350, 700, 1050, 1400]) {
      pulse.push(
        await fixture
          .locator('.dz-tabs__status')
          .first()
          .evaluate((dot, time) => {
            for (const animation of dot.getAnimations()) {
              animation.pause()
              animation.currentTime = time
            }
            return Number(getComputedStyle(dot).opacity)
          }, time)
      )
    }
    assert.ok(pulse[1] < 1 && pulse[1] > 0.35)
    assert.equal(pulse[2], 0.35)
    assert.ok(pulse[3] > 0.35 && pulse[3] < 1)
    assert.equal(pulse[4], 1)
    if (process.env.DAZZUI_TABS_REFERENCE)
      fs.writeFileSync(
        process.env.DAZZUI_TABS_REFERENCE,
        JSON.stringify({ readings, pulse }, null, 2)
      )
  } finally {
    await browser.close()
    await server.close()
  }
})
