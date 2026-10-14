import assert from 'node:assert/strict'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

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
