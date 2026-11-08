import assert from 'node:assert/strict'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

test('checkbox groups restore mixed selections and preserve disabled children', async () => {
  const fixture = '/checkbox-group-fixture.js'
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    logLevel: 'silent',
    server: { host: '127.0.0.1', port: 0 },
    plugins: [
      {
        name: 'checkbox-group-fixture',
        resolveId: id =>
          id === fixture ? '\0checkbox-group-fixture' : undefined,
        load: id =>
          id === '\0checkbox-group-fixture'
            ? `
        import {createElement as h, useState} from 'react'
        import {createRoot} from 'react-dom/client'
        import {flushSync} from 'react-dom'
        import {Checkbox, CheckboxGroup} from '@dazzlabs/dazzui'
        const host = document.createElement('div')
        host.id = 'checkbox-fixture'
        host.style.cssText = 'position:fixed;top:24px;left:24px;width:240px'
        document.body.append(host)
        const root = createRoot(host)
        let sequence = 0
        export const changes = []
        function Fixture({options}) {
          const [value, setValue] = useState(options.initial)
          return h(CheckboxGroup, {
            allValues: options.plain ? undefined : ['a', 'b', 'c'],
            defaultValue: options.initial,
            value: options.controlled ? value : undefined,
            disabled: options.groupDisabled,
            onValueChange(next) {
              changes.push(next)
              if (options.accept !== false) setValue(next)
            },
          },
          h(Checkbox, {parent: true}, 'All'),
          ...['a', 'b', 'c'].map(value => h(Checkbox, {
            key: value, value,
            disabled: options.disabled?.includes(value),
            readOnly: options.readOnly?.includes(value),
          }, value)))
        }
        export function mount(options) {
          changes.length = 0
          flushSync(() => root.render(h(Fixture, {key: ++sequence, options})))
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
    await page.evaluate(() =>
      window.conformance.show('checkbox/preview', 'studio-light')
    )
    await page.evaluate(async () => {
      window.checkboxFixture = await import('/checkbox-group-fixture.js')
    })
    const group = page.locator('#checkbox-fixture')
    const controls = group.getByRole('checkbox')
    const checked = () =>
      controls.evaluateAll(nodes =>
        nodes.map(node => node.getAttribute('aria-checked'))
      )
    for (const theme of await page.evaluate(() => window.conformance.drawn)) {
      await page.evaluate(
        theme => window.conformance.show('checkbox/preview', theme),
        theme
      )
      for (const [initial, disabled, proposals, states] of [
        [
          ['a'],
          ['b'],
          [['a', 'c'], [], ['a']],
          ['mixed', 'mixed', 'false', 'mixed'],
        ],
        [
          ['a', 'b'],
          ['b'],
          [['a', 'b', 'c'], ['b'], ['a', 'b']],
          ['mixed', 'true', 'mixed', 'mixed'],
        ],
      ]) {
        await page.evaluate(options => window.checkboxFixture.mount(options), {
          initial,
          disabled,
        })
        assert.equal((await checked())[0], states[0])
        for (let i = 0; i < 3; i++) {
          await controls.first().click()
          assert.equal((await checked())[0], states[i + 1])
        }
        assert.deepEqual(
          await page.evaluate(() => window.checkboxFixture.changes),
          proposals
        )
        const geometry = await controls.evaluateAll(nodes =>
          nodes.map(node => ({
            box: node.getBoundingClientRect().toJSON(),
            mark: node
              .querySelector('.dz-checkbox__box')
              .getBoundingClientRect()
              .toJSON(),
          }))
        )
        assert.equal(geometry[1].mark.x - geometry[0].mark.x, 26)
        assert.equal(geometry[1].box.y - geometry[0].box.y, 26)
      }
    }
    await page.evaluate(() =>
      window.checkboxFixture.mount({
        initial: ['a'],
        controlled: true,
        accept: false,
      })
    )
    await controls.nth(2).click()
    assert.deepEqual(await checked(), ['mixed', 'true', 'false', 'false'])
    assert.deepEqual(
      await page.evaluate(() => window.checkboxFixture.changes),
      [['a', 'b']]
    )
    await page.evaluate(() =>
      window.checkboxFixture.mount({
        initial: [],
        plain: true,
        readOnly: ['a'],
      })
    )
    await controls.nth(1).click()
    await controls.nth(2).click()
    assert.deepEqual(await checked(), ['false', 'false', 'true', 'false'])
    assert.deepEqual(
      await page.evaluate(() => window.checkboxFixture.changes),
      [['b']]
    )
    await controls.nth(2).focus()
    await page.keyboard.press('Enter')
    assert.deepEqual(
      await page.evaluate(() => window.checkboxFixture.changes),
      [['b']]
    )
  } finally {
    await browser.close()
    await server.close()
  }
})

test('standalone toggles, grouped toggles and switches retain their distinct key phases', async () => {
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    logLevel: 'silent',
    server: { host: '127.0.0.1', port: 0 },
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
    for (const [scene, name, property, phases] of [
      [
        'toggle/0',
        'Wrap lines',
        'aria-pressed',
        [
          ['Space', false, false, false],
          ['Enter', true, true, false],
          ['Enter', false, true, false],
          ['Space', true, true, true],
          ['Space', true, true, true],
          ['Space', false, false, false],
        ],
      ],
      [
        'toggle/1',
        'Italic',
        'aria-pressed',
        [
          ['Space', true, true, false],
          ['Space', true, false, false],
          ['Space', false, false, false],
          ['Enter', true, true, false],
          ['Enter', false, true, false],
        ],
      ],
      [
        'switch/preview',
        'Sync over cellular',
        'aria-checked',
        [
          ['Space', false, true, false],
          ['Enter', true, false, false],
          ['Enter', false, false, false],
          ['Space', true, false, false],
          ['Space', true, false, false],
          ['Space', false, true, false],
        ],
      ],
    ]) {
      await page.evaluate(
        scene => window.conformance.show(scene, 'studio-light'),
        scene
      )
      const control = page.getByRole(
        property === 'aria-checked' ? 'switch' : 'button',
        { name, exact: true }
      )
      await control.focus()
      assert.equal(await control.getAttribute(property), 'false')
      for (const [key, down, checked, active] of phases) {
        await page.keyboard[down ? 'down' : 'up'](key)
        assert.equal(
          await control.getAttribute(property),
          String(checked),
          `${scene} ${key} ${down ? 'down' : 'up'}`
        )
        assert.equal(
          await control.evaluate(el => el.matches(':active')),
          active
        )
      }
    }
  } finally {
    await browser.close()
    await server.close()
  }
})

test('switch transitions change backgrounds while its thumb position switches immediately', async () => {
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    logLevel: 'silent',
    server: { host: '127.0.0.1', port: 0 },
  })
  await server.listen()
  const browser = await chromium.launch({
    executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE || undefined,
  })
  try {
    const page = await browser.newPage({ reducedMotion: 'no-preference' })
    await page.goto(
      new URL('conformance.html', server.resolvedUrls.local[0]).href
    )
    await page.waitForFunction(() => window.conformance)
    await page.evaluate(() => {
      for (const sheet of document.styleSheets)
        for (const rule of sheet.cssRules)
          if (
            rule instanceof CSSStyleRule &&
            rule.style.caretColor === 'transparent'
          )
            rule.style.removeProperty('transition')
    })
    for (const theme of ['bright-light', 'studio-dark']) {
      await page.evaluate(
        theme => window.conformance.show('switch/preview', theme),
        theme
      )
      const control = page.getByRole('switch', { name: 'Sync over cellular' })
      const read = time =>
        control.evaluate((el, time) => {
          const animations = el.getAnimations({ subtree: true })
          if (time !== null)
            for (const animation of animations) {
              animation.pause()
              animation.currentTime = time
            }
          const thumb = el.querySelector('.dz-switch__thumb')
          const trackBox = el.getBoundingClientRect()
          const thumbBox = thumb.getBoundingClientRect()
          return {
            track: getComputedStyle(el).backgroundColor,
            thumb: getComputedStyle(thumb).backgroundColor,
            offset: thumbBox.x - trackBox.x,
            end:
              trackBox.width -
              parseFloat(getComputedStyle(el).paddingRight) -
              thumbBox.width,
            properties: animations.map(a => a.transitionProperty),
          }
        }, time)
      const off = await read(null)
      await control.click()
      const first = await read(0)
      const middle = await read(75)
      const end = await read(150)
      assert.deepEqual(first.properties, [
        'background-color',
        'background-color',
      ])
      assert.equal(first.offset, first.end)
      assert.equal(first.offset, middle.offset)
      assert.equal(first.offset, end.offset)
      assert.deepEqual([first.track, first.thumb], [off.track, off.thumb])
      for (const property of ['track', 'thumb']) {
        assert.notEqual(middle[property], off[property])
        assert.notEqual(middle[property], end[property])
      }
    }
  } finally {
    await browser.close()
    await server.close()
  }
})

test('painted text baselines follow the browser inline baseline through scaling', async () => {
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    logLevel: 'silent',
    server: { host: '127.0.0.1', port: 0 },
  })
  await server.listen()
  const browser = await chromium.launch({
    executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE || undefined,
  })
  try {
    const page = await browser.newPage({ deviceScaleFactor: 2 })
    await page.goto(
      new URL('conformance.html', server.resolvedUrls.local[0]).href
    )
    await page.waitForFunction(() => window.conformance)
    const results = await page.evaluate(async () => {
      const { measure } = await import('/src/conformance/measure.ts')
      const fixture = document.createElement('div')
      fixture.style.cssText = 'position:fixed;top:30px;left:20px'
      fixture.innerHTML = `
        <div class="outer"><div class="inner" style="font:600 12px/16.8px system-ui">
          <span class="run">Actual baseline</span><i class="marker" style="display:inline-block;width:0;height:0;vertical-align:baseline"></i>
        </div></div>
        <input value="no DOM text rectangle">
      `
      document.body.append(fixture)
      const outer = fixture.querySelector('.outer')
      const inner = fixture.querySelector('.inner')
      const run = fixture.querySelector('.run')
      const marker = fixture.querySelector('.marker')
      const result = []
      for (const [transform, nested, zoom] of [
        ['none', 'none', '1'],
        ['translate(7.5px,13.25px) scale(.93)', 'scale(.91)', '1'],
        ['translate(3px,8px) scale(.95)', 'none', '1.25'],
      ]) {
        outer.style.transform = transform
        inner.style.transform = nested
        outer.style.zoom = zoom
        result.push({
          actual: measure(run, new Set()).values.baseline,
          expected: marker.getBoundingClientRect().top,
        })
      }
      outer.style.zoom = '1'
      outer.style.transform = 'rotate(3deg)'
      result.push({ unread: measure(run, new Set()).values.baseline })
      outer.style.transform = 'scale(1,.9)'
      result.push({ unread: measure(run, new Set()).values.baseline })
      outer.style.transform = 'none'
      outer.style.scale = '.9'
      result.push({ unread: measure(run, new Set()).values.baseline })
      result.push({
        unread: measure(fixture.querySelector('input'), new Set()).values
          .baseline,
      })
      fixture.remove()
      return result
    })
    for (const result of results) {
      if ('unread' in result) assert.equal(result.unread, null)
      else {
        assert.equal(typeof result.actual, 'number', JSON.stringify(results))
        assert.ok(
          Math.abs(result.actual - result.expected) < 0.02,
          `painted baseline ${result.actual} differs from inline alignment ${result.expected}`
        )
      }
    }
  } finally {
    await browser.close()
    await server.close()
  }
})

test('modal exit retains content and focus until its transitions finish', async () => {
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    logLevel: 'silent',
    server: { host: '127.0.0.1', port: 0 },
  })
  await server.listen()
  const browser = await chromium.launch({
    executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE || undefined,
  })
  try {
    const page = await browser.newPage({ reducedMotion: 'no-preference' })
    await page.goto(
      new URL('conformance.html', server.resolvedUrls.local[0]).href
    )
    await page.waitForFunction(() => window.conformance)
    await page.addStyleTag({
      content:
        '.dz-dialog,.dz-dialog-scrim,.dz-drawer,.dz-drawer-scrim {transition:opacity 150ms cubic-bezier(.4,0,.2,1),transform 150ms cubic-bezier(.4,0,.2,1)!important}',
    })
    for (const [scene, selector] of [
      ['dialog/0', '.dz-dialog'],
      ['drawer/0', '.dz-drawer'],
    ]) {
      await page.evaluate(
        scene => window.conformance.show(scene, 'studio-light'),
        scene
      )
      const opener = page.locator('[aria-haspopup="dialog"]').first()
      await opener.click()
      await page.locator(selector).waitFor()
      await page.waitForFunction(
        selector =>
          document
            .getAnimations()
            .some(
              a =>
                a instanceof CSSTransition && a.effect.target.matches(selector)
            ),
        selector
      )
      await page.evaluate(() => {
        for (const a of document.getAnimations())
          if (
            a instanceof CSSTransition &&
            a.effect.target.matches(
              '.dz-dialog,.dz-dialog-scrim,.dz-drawer,.dz-drawer-scrim'
            )
          )
            a.finish()
      })
      if (scene === 'drawer/0') {
        await page.waitForFunction(
          selector =>
            document.activeElement === document.querySelector(selector),
          selector
        )
        assert.ok(
          await page
            .locator(selector)
            .evaluate(el => document.activeElement === el),
          'Drawer initially focuses the panel'
        )
      }
      await page.keyboard.press('Escape')
      await page.waitForFunction(
        selector =>
          document
            .getAnimations()
            .some(
              a =>
                a instanceof CSSTransition &&
                a.playState !== 'finished' &&
                a.effect.target.matches(selector)
            ),
        selector
      )
      await page.evaluate(() => {
        for (const a of document.getAnimations()) {
          if (
            a instanceof CSSTransition &&
            a.effect.target.matches(
              '.dz-dialog,.dz-dialog-scrim,.dz-drawer,.dz-drawer-scrim'
            )
          ) {
            a.pause()
            a.currentTime = 75
          }
        }
      })
      assert.equal(await page.locator(selector).count(), 1)
      assert.ok(
        await page
          .locator(selector)
          .evaluate(el => el.contains(document.activeElement)),
        'the closing popup retains focus'
      )
      const live = await page.evaluate(() => window.conformance.read())
      assert.ok(
        live.specimens.some(s => s.reading),
        'interaction capture can remeasure mounted content'
      )
      await page.evaluate(() => {
        for (const a of document.getAnimations())
          if (
            a instanceof CSSTransition &&
            a.effect.target.matches(
              '.dz-dialog,.dz-dialog-scrim,.dz-drawer,.dz-drawer-scrim'
            )
          )
            a.finish()
      })
      await page.waitForFunction(
        selector => !document.querySelector(selector),
        selector
      )
      assert.ok(await opener.evaluate(el => document.activeElement === el))
    }
  } finally {
    await browser.close()
    await server.close()
  }
})

test('loading motion uses native state attributes and retains reduced-motion exceptions', async () => {
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    logLevel: 'silent',
    server: { host: '127.0.0.1', port: 0 },
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
    for (const reduced of [false, true]) {
      await page.emulateMedia({
        reducedMotion: reduced ? 'reduce' : 'no-preference',
      })
      await page.evaluate(() =>
        window.conformance.show('progress/2', 'studio-light')
      )
      const progress = await page
        .locator('.dz-progress')
        .first()
        .evaluate(root => {
          const fill = root.querySelector('.dz-progress__fill')
          const style = getComputedStyle(fill)
          return {
            attribute: root.getAttribute('data-indeterminate'),
            fraction:
              parseFloat(style.width) /
              parseFloat(getComputedStyle(root).width),
            name: style.animationName,
            duration: style.animationDuration,
            iterations: style.animationIterationCount,
            easing: style.animationTimingFunction,
          }
        })
      assert.equal(progress.attribute, '', 'Base UI uses a presence attribute')
      assert.equal(progress.fraction, 0.4)
      assert.equal(progress.name, 'dz-progress-sweep')
      assert.equal(progress.duration, reduced ? '3s' : '1.2s')
      assert.equal(progress.iterations, 'infinite')
      assert.equal(progress.easing, 'cubic-bezier(0.42, 0, 0.58, 1)')
      await page.evaluate(() =>
        window.conformance.show('spinner/0', 'studio-light')
      )
      const spinner = await page
        .locator('.dz-spinner')
        .first()
        .evaluate(el => {
          const style = getComputedStyle(el)
          return [style.animationDuration, style.animationIterationCount]
        })
      assert.deepEqual(spinner, [reduced ? '2.4s' : '1s', 'infinite'])
      await page.evaluate(() =>
        window.conformance.show('skeleton/0', 'studio-light')
      )
      const skeleton = await page
        .locator('.dz-skeleton')
        .first()
        .evaluate(el => {
          for (const animation of el.getAnimations())
            animation.currentTime = 800
          const style = getComputedStyle(el)
          return [
            style.animationDuration,
            style.animationIterationCount,
            style.backgroundPosition,
          ]
        })
      assert.equal(skeleton[0], reduced ? '1e-05s' : '1.6s')
      assert.equal(skeleton[1], reduced ? '1' : 'infinite')
      assert.equal(
        parseFloat(skeleton[2]),
        0,
        'full half-cycle and reduced rest both use base position'
      )
    }
  } finally {
    await browser.close()
    await server.close()
  }
})

test('collector reads compound surfaces and editable text', async () => {
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    logLevel: 'silent',
    server: { host: '127.0.0.1', port: 0 },
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
    const washes = await page.evaluate(async () => {
      const { hex } = await import('/src/conformance/measure.ts')
      return [
        hex('rgba(31, 157, 94, 0.12)'),
        hex('color(srgb 0.121568627451 0.61568627451 0.36862745098 / 12%)'),
        hex('#1f9d5e1f'),
        hex('color(display-p3 1 1 1 / 12%)'),
        hex('transparent'),
        hex('rgb(31, 157, 94)'),
      ]
    })
    assert.deepEqual(washes, [
      '#1f9d5e1f',
      '#1f9d5e1f',
      '#1f9d5e1f',
      '#ffffff1f',
      '#00000000',
      '#1f9d5e',
    ])
    const tree = await page.evaluate(async () => {
      const scene = await window.conformance.show(
        'tree/preview',
        'studio-light'
      )
      return scene.specimens.map(specimen => specimen.reading)
    })
    assert.equal(
      tree[6],
      null,
      'the closed styles folder does not draw base.css'
    )
    assert.ok(
      tree[9],
      'the closed branch must not take the README icon address'
    )
    for (const scene of ['toast/preview', 'toast/0', 'toast/1', 'toast/2']) {
      const specimens = await page.evaluate(
        async scene =>
          (await window.conformance.show(scene, 'studio-light')).specimens,
        scene
      )
      assert.equal(specimens[0].component, 'ToastProvider')
      assert.equal(
        specimens[0].reading,
        null,
        'a context provider must not claim the layout helper its child draws'
      )
      const buttons = specimens.filter(s => s.component === 'Button')
      assert.equal(buttons.length, scene === 'toast/0' ? 6 : 1)
      assert.ok(buttons.every(s => s.reading))
    }
    const readings = await page.evaluate(async () => {
      const { measure } = await import('/src/conformance/measure.ts')
      const fixture = document.createElement('div')
      fixture.innerHTML = `
        <div class="dz-number-field" style="width:200px">
          <div class="dz-number-field__group" style="border:1px solid black;border-radius:13px;background:rgb(10,20,30)">
            <input type="hidden" value="hidden">
            <input value="42" style="font-size:17px;font-weight:500;color:rgb(40,50,60)">
            <span style="font-size:9px;color:red">kg</span>
          </div>
        </div>
        <div class="dz-shortcut-recorder">
          <button class="dz-shortcut-recorder__button" style="border-radius:9px">Record</button>
        </div>
        <style>.collector-placeholder::placeholder { color:rgb(70,80,90) }</style>
        <input class="collector-placeholder" placeholder="Email" style="font-size:15px;color:red">
        <svg class="collector-icon" style="color:rgb(20,40,60)" width="16" height="16"><path d="M0 0h16v16H0z" fill="currentColor" /></svg>
        <button class="collector-button" style="color:rgb(80,90,100)"><svg class="collector-nested-icon" style="color:red" width="16" height="16"></svg>Save</button>
        <div class="collector-sentinel"><span style="clip-path:inset(50%);position:fixed;width:1px;height:1px">x</span></div>
        <input class="collector-empty" value="">
        <svg class="collector-stroke" width="12" height="12" style="fill:none;stroke:rgba(0,0,0,0.55);color:red"><path d="M2 2l8 8" /></svg>
        <div class="collector-hidden"><span style="visibility:hidden;color:red">Hidden title</span><span style="color:rgb(10,20,30)">Visible body</span></div>
      `
      document.body.append(fixture)
      const read = selector =>
        measure(fixture.querySelector(selector), new Set())
      return [
        read('.dz-number-field'),
        read('.dz-shortcut-recorder'),
        read('.collector-placeholder'),
        read('.collector-icon'),
        measure(
          fixture.querySelector('.collector-button'),
          new Set([fixture.querySelector('.collector-nested-icon')])
        ),
        read('.collector-sentinel'),
        read('.collector-empty'),
        read('.collector-stroke'),
        read('.collector-hidden'),
      ]
    })
    assert.equal(readings[0].values.corner, 13)
    assert.equal(readings[0].values.fill, '#0a141e')
    assert.equal(readings[0].values.fontSize, 17)
    assert.equal(readings[0].values.fontWeight, 500)
    assert.equal(readings[0].values.ink, '#28323c')
    assert.equal(readings[1].values.corner, 9)
    assert.equal(readings[2].values.fontSize, 15)
    assert.equal(readings[2].values.ink, '#46505a')
    assert.equal(readings[3].values.ink, '#14283c')
    assert.equal(readings[3].values.fontSize, null)
    assert.equal(readings[4].values.ink, '#505a64')
    assert.equal(readings[5].values.ink, null)
    assert.equal(readings[5].values.fontSize, null)
    assert.equal(readings[6].values.ink, null)
    assert.equal(readings[6].values.fontSize, null)
    assert.equal(readings[7].values.ink, '#0000008c')
    assert.equal(readings[8].values.ink, '#0a141e')
  } finally {
    await browser.close()
    await server.close()
  }
})
