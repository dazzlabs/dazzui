import assert from 'node:assert/strict'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

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
      `
      document.body.append(fixture)
      const read = selector =>
        measure(fixture.querySelector(selector), new Set())
      return [
        read('.dz-number-field'),
        read('.dz-shortcut-recorder'),
        read('.collector-placeholder'),
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
  } finally {
    await browser.close()
    await server.close()
  }
})
