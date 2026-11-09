import assert from 'node:assert/strict'
import { readFile } from 'node:fs/promises'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

const { cases, rangeCases, maxLengthCases } = JSON.parse(
  await readFile(
    new URL('text-field-graphemes.cases.json', import.meta.url),
    'utf8'
  )
)

test('text editing keeps emoji intact and deletes ordinary marks individually', async () => {
  const server = await createServer({
    root: fileURLToPath(new URL('..', import.meta.url)),
    cacheDir: 'node_modules/.vite-test-text-field-graphemes',
    logLevel: 'silent',
    server: { host: '127.0.0.1', port: 0 },
    plugins: [
      {
        name: 'text-field-graphemes-fixture',
        resolveId: id =>
          id === '/text-field-graphemes.js'
            ? '\0text-field-graphemes'
            : undefined,
        load: id =>
          id === '\0text-field-graphemes'
            ? `
        import {createElement as h} from 'react';import {createRoot} from 'react-dom/client';import {flushSync} from 'react-dom';import {TextField} from '@dazzlabs/dazzui';
        const host=document.createElement('div');host.id='text-field-graphemes';document.body.append(host);const root=createRoot(host);let key=0;
        export function mount(props) {flushSync(()=>root.render(h(TextField,{key:++key,'aria-label':'Text',...props})));}
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
      await window.conformance.show('text-field/preview', 'studio-light')
      document.getElementById('root').style.display = 'none'
      window.textFieldGraphemes = await import('/text-field-graphemes.js')
    })
    for (const [scene, selector, props] of [
      ['text', 'input', {}],
      ['multiline', 'textarea', { multiline: true }],
      ['password', 'input', { type: 'password' }],
    ]) {
      await page.evaluate(
        props => window.textFieldGraphemes.mount(props),
        props
      )
      const input = page.locator(`#text-field-graphemes ${selector}`)
      for (const fixture of cases) {
        const text = `X${fixture.cluster}Y`
        const select = async offset => {
          await input.fill(text)
          await input.evaluate(
            (node, at) => node.setSelectionRange(at, at),
            offset
          )
        }
        const state = () =>
          input.evaluate(node => ({
            value: node.value,
            start: node.selectionStart,
            end: node.selectionEnd,
            reversed: node.selectionDirection === 'backward',
          }))
        const at = async (key, expected) => {
          await input.press(key)
          assert.deepEqual(
            await state(),
            expected,
            `${scene}/${fixture.id}/${key}`
          )
        }
        await select(0)
        for (const expected of fixture.right) await at('ArrowRight', expected)
        await select(text.length)
        for (const expected of fixture.left) await at('ArrowLeft', expected)
        await select(1 + fixture.cluster.length)
        for (const expected of fixture.backspace)
          await at('Backspace', expected)
        await select(1)
        await at('Delete', fixture.forward)
        await select(1 + fixture.cluster.length)
        await at('Shift+ArrowLeft', fixture.selection)
        await select(1)
        await at('Shift+ArrowRight', fixture.selectionRight)
      }
      for (const fixture of rangeCases) {
        await input.fill(fixture.text)
        await input.evaluate(
          (node, range) => node.setSelectionRange(...range),
          fixture.selection
        )
        await input.press(fixture.key)
        const actual = await input.evaluate(node => ({
          value: node.value,
          start: node.selectionStart,
          end: node.selectionEnd,
          reversed: node.selectionDirection === 'backward',
        }))
        assert.deepEqual(actual, fixture.expected, `${scene}/${fixture.id}`)
      }
      for (const fixture of maxLengthCases) {
        await page.evaluate(
          ({ props, limit }) =>
            window.textFieldGraphemes.mount({ ...props, maxLength: limit }),
          { props, limit: fixture.limit }
        )
        const input = page.locator(`#text-field-graphemes ${selector}`)
        await input.fill('')
        await page.keyboard.insertText(fixture.input)
        const actual = await input.evaluate(node => ({
          value: node.value,
          start: node.selectionStart,
          end: node.selectionEnd,
          reversed: node.selectionDirection === 'backward',
        }))
        assert.deepEqual(
          actual,
          scene === 'password' ? fixture.passwordExpected : fixture.expected,
          `${scene}/maxlength/${fixture.input}`
        )
        await input.press('Backspace')
        const after = await input.evaluate(node => ({
          value: node.value,
          start: node.selectionStart,
          end: node.selectionEnd,
          reversed: node.selectionDirection === 'backward',
        }))
        assert.deepEqual(
          after,
          scene === 'password'
            ? fixture.passwordAfterBackspace
            : fixture.afterBackspace,
          `${scene}/maxlength/backspace/${fixture.input}`
        )
      }
      await page.evaluate(
        props =>
          window.textFieldGraphemes.mount({
            ...props,
            maxLength: 3,
            defaultValue: 'A😀',
          }),
        props
      )
      const replacement = page.locator(`#text-field-graphemes ${selector}`)
      await replacement.focus()
      await replacement.evaluate(node => node.setSelectionRange(1, 3))
      await page.keyboard.insertText('BCDE')
      assert.equal(
        await replacement.inputValue(),
        scene === 'password' ? 'AB' : 'ABC'
      )
    }
    await page.evaluate(() =>
      window.textFieldGraphemes.mount({ type: 'password' })
    )
    const password = page.locator('#text-field-graphemes input')
    await password.evaluate(node =>
      node.style.setProperty('caret-color', '#ff00ff', 'important')
    )
    const caretX = async (value, at) => {
      await password.fill(value)
      await password.evaluate((node, at) => node.setSelectionRange(at, at), at)
      const png = await password.screenshot({ caret: 'initial' })
      return page.evaluate(
        async src => {
          const image = new Image()
          image.src = src
          await image.decode()
          const canvas = document.createElement('canvas')
          canvas.width = image.width
          canvas.height = image.height
          const context = canvas.getContext('2d')
          context.drawImage(image, 0, 0)
          const rgba = context.getImageData(
            0,
            0,
            canvas.width,
            canvas.height
          ).data
          let left = Infinity
          for (let at = 0; at < rgba.length; at += 4) {
            if (rgba[at] > 240 && rgba[at + 1] < 20 && rgba[at + 2] > 240)
              left = Math.min(left, (at / 4) % canvas.width)
          }
          if (!Number.isFinite(left))
            throw new Error('password caret was not painted')
          return left
        },
        `data:image/png;base64,${png.toString('base64')}`
      )
    }
    for (const [cluster, masks] of [
      ['e\u0301', 1],
      ['👨‍👩‍👧‍👦', 1],
      ['👩🏽‍💻', 1],
      ['क्‍षि', 1],
      ['a‍👨', 2],
    ]) {
      const expected = await caretX('x'.repeat(masks + 2), 1 + masks)
      assert.equal(
        await caretX(`X${cluster}Y`, 1 + cluster.length),
        expected,
        cluster
      )
      if (cluster === 'क्‍षि')
        assert.equal(await caretX(`X${cluster}Y`, 4), expected)
    }
    for (const range of [
      [3, 3],
      [1, 2],
    ]) {
      await page.evaluate(() =>
        window.textFieldGraphemes.mount({
          readOnly: true,
          defaultValue: 'Xe\u0301Y',
        })
      )
      const input = page.locator('#text-field-graphemes input')
      await input.evaluate((node, range) => {
        node.focus()
        node.setSelectionRange(...range)
      }, range)
      for (const key of ['Backspace', 'Delete']) {
        await input.press(key)
        assert.equal(await input.inputValue(), 'Xe\u0301Y')
        assert.deepEqual(
          await input.evaluate(node => [
            node.selectionStart,
            node.selectionEnd,
          ]),
          range
        )
      }
    }
  } finally {
    await browser.close()
    await server.close()
  }
})
