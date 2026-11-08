import assert from 'node:assert/strict'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

test('multiline navigation retains its column and respects visual wrapped rows', async () => {
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    cacheDir: 'node_modules/.vite-test-text-field-navigation',
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
    await page.evaluate(() =>
      window.conformance.show('text-field/2', 'studio-light')
    )
    const input = page.locator('textarea').first()
    await input.evaluate(node => {
      node.style.width = '100px'
      node.style.fontFamily = 'Menlo, monospace'
    })
    const select = async offset =>
      input.evaluate(
        (node, offset) => node.setSelectionRange(offset, offset),
        offset
      )
    const range = () =>
      input.evaluate(node => [
        node.selectionStart,
        node.selectionEnd,
        node.selectionDirection,
      ])
    const at = async (key, expected) => {
      await input.press(key)
      assert.deepEqual((await range()).slice(0, 2), expected, key)
    }
    await input.fill('abcdefghij\nx\nabcdefghij')
    await select(8)
    for (const [key, index] of [
      ['ArrowDown', 12],
      ['ArrowDown', 21],
      ['ArrowUp', 12],
      ['ArrowUp', 8],
    ])
      await at(key, [index, index])
    await at('Shift+ArrowDown', [8, 12])
    await at('Shift+ArrowDown', [8, 21])
    await at('Shift+ArrowUp', [8, 12])
    await at('Shift+ArrowUp', [8, 8])
    await at('ArrowDown', [12, 12])
    await at('ArrowDown', [21, 21])
    await at('ArrowLeft', [20, 20])
    await at('ArrowUp', [12, 12])
    await at('ArrowUp', [7, 7])
    for (const [direction, key, target] of [
      ['forward', 'ArrowUp', 0],
      ['forward', 'ArrowDown', 23],
      ['backward', 'ArrowUp', 0],
      ['backward', 'ArrowDown', 23],
    ]) {
      await input.evaluate(
        (node, direction) => node.setSelectionRange(8, 21, direction),
        direction
      )
      await at(key, [target, target])
    }

    await input.fill('abcdefghij'.repeat(4))
    const home = process.platform === 'darwin' ? 'Meta+ArrowLeft' : 'Home'
    const end = process.platform === 'darwin' ? 'Meta+ArrowRight' : 'End'
    await select(15)
    await at(home, [10, 10])
    await at(end, [20, 20])
    await at(home, [10, 10])
    await select(9)
    await at('ArrowRight', [10, 10])
    await at(home, [10, 10])
    await select(11)
    await at('ArrowLeft', [10, 10])
    await at(end, [20, 20])
    await select(15)
    await at(`Shift+${home}`, [10, 15])
    assert.equal((await range())[2], 'backward')
    await select(15)
    await at(`Shift+${end}`, [15, 20])
    await input.fill('abc def ghi jkl mno pqr')
    await select(4)
    await at(end, [8, 8])
    await at(home, [0, 0])
    await select(12)
    await at(home, [8, 8])
    await at(end, [16, 16])
    if (process.platform === 'darwin') {
      await input.fill('abcdefghij'.repeat(4))
      await select(15)
      await input.press('Meta+Backspace')
      assert.equal(
        await input.inputValue(),
        'abcdefghijfghijabcdefghijabcdefghij'
      )
    }
  } finally {
    await browser.close()
    await server.close()
  }
})
