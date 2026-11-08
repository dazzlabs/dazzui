import assert from 'node:assert/strict'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

test('single-line field carets use actual font height and explicit input insets', async () => {
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    cacheDir: 'node_modules/.vite-test-field-caret',
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
    for (const [scene, selector, rootSelector, left, height] of [
      ['text-field/preview', '.dz-text-field', '.dz-text-field', 13, 15],
      [
        'search-field/preview',
        '.dz-search-field__input',
        '.dz-search-field',
        39,
        15,
      ],
      [
        'number-field/preview',
        '.dz-number-field__input',
        '.dz-number-field__group',
        13,
        15,
      ],
      [
        'combobox/preview',
        '.dz-combobox__input',
        '.dz-combobox__group',
        13,
        15,
      ],
      ['combobox/2', '.dz-text-field', '.dz-text-field', 13, 15],
      [
        'otp-field/preview',
        '.dz-otp-field__cell',
        '.dz-otp-field__cell',
        14,
        14,
      ],
    ]) {
      await page.evaluate(
        scene => window.conformance.show(scene, 'studio-light'),
        scene
      )
      const input = page.locator(selector).first()
      await input.evaluate(node => {
        node.focus()
        node.setSelectionRange(0, 0)
        node.style.setProperty('caret-color', '#ff00ff', 'important')
      })
      // Read the actual browser caret, not a line-height-derived proxy. The
      // collector normally hides it; only its colour is overridden here.
      const png = await page
        .locator(rootSelector)
        .first()
        .screenshot({ caret: 'initial' })
      const actual = await page.evaluate(
        async src => {
          const image = new Image()
          image.src = src
          await image.decode()
          const canvas = document.createElement('canvas')
          canvas.width = image.width
          canvas.height = image.height
          const context = canvas.getContext('2d')
          context.drawImage(image, 0, 0)
          const pixels = context.getImageData(
            0,
            0,
            canvas.width,
            canvas.height
          ).data
          let left = Infinity,
            top = Infinity,
            right = -1,
            bottom = -1
          for (let y = 0; y < canvas.height; y++)
            for (let x = 0; x < canvas.width; x++) {
              const at = (y * canvas.width + x) * 4
              if (
                pixels[at] > 240 &&
                pixels[at + 1] < 20 &&
                pixels[at + 2] > 240
              ) {
                left = Math.min(left, x)
                top = Math.min(top, y)
                right = Math.max(right, x)
                bottom = Math.max(bottom, y)
              }
            }
          return {
            left: left / 2,
            top: top / 2,
            width: (right - left + 1) / 2,
            height: (bottom - top + 1) / 2,
          }
        },
        'data:image/png;base64,' + png.toString('base64')
      )
      assert.deepEqual(actual, { left, top: 7, width: 1, height }, scene)
    }
  } finally {
    await browser.close()
    await server.close()
  }
})
