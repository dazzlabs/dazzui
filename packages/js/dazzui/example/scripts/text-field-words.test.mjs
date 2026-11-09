import assert from 'node:assert/strict'
import { readFile } from 'node:fs/promises'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

const { cases, platforms } = JSON.parse(
  await readFile(
    new URL('text-field-words.cases.json', import.meta.url),
    'utf8'
  )
)

test(
  'word navigation, selection and deletion follow browser Unicode boundaries',
  {
    skip: platforms.includes(process.platform)
      ? false
      : 'Fixtures recorded on macOS and Linux',
  },
  async () => {
    const server = await createServer({
      root: fileURLToPath(new URL('..', import.meta.url)),
      cacheDir: 'node_modules/.vite-test-text-field-words',
      logLevel: 'silent',
      server: { host: '127.0.0.1', port: 0 },
      plugins: [
        {
          name: 'text-field-words-fixture',
          resolveId: id =>
            id === '/text-field-words.js' ? '\0text-field-words' : undefined,
          load: id =>
            id === '\0text-field-words'
              ? `
        import {createElement as h} from 'react';import {createRoot} from 'react-dom/client';import {flushSync} from 'react-dom';import {TextField} from '@dazzlabs/dazzui';
        const host=document.createElement('div');host.id='text-field-words';document.body.append(host);const root=createRoot(host);let key=0;
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
        window.textFieldWords = await import('/text-field-words.js')
      })
      const modifier = process.platform === 'darwin' ? 'Alt' : 'Control'
      const keys = [
        'ArrowLeft',
        'ArrowRight',
        'Shift+ArrowLeft',
        'Shift+ArrowRight',
        'Backspace',
        'Delete',
      ]
      for (const multiline of [false, true]) {
        await page.evaluate(
          multiline => window.textFieldWords.mount({ multiline }),
          multiline
        )
        const input = page.locator(
          `#text-field-words ${multiline ? 'textarea' : 'input'}`
        )
        for (const fixture of cases) {
          if (fixture.multiline && !multiline) continue
          for (const [
            at,
            left,
            right,
            extendLeft,
            extendRight,
            deleteLeft,
            deleteRight,
          ] of fixture.checks) {
            const caret = (value, offset) => [value, offset, offset, false]
            const selection = offset => [
              fixture.text,
              Math.min(at, offset),
              Math.max(at, offset),
              offset < at,
            ]
            const expected = [
              caret(fixture.text, left),
              caret(fixture.text, right),
              selection(extendLeft),
              selection(extendRight),
              caret(...deleteLeft),
              caret(...deleteRight),
            ]
            for (const [index, key] of keys.entries()) {
              await input.fill(fixture.text)
              await input.evaluate(
                (node, at) => node.setSelectionRange(at, at),
                at
              )
              await input.press(`${modifier}+${key}`)
              const actual = await input.evaluate(node => [
                node.value,
                node.selectionStart,
                node.selectionEnd,
                node.selectionDirection === 'backward',
              ])
              assert.deepEqual(
                actual,
                expected[index],
                `${fixture.id}/${at}/${key}/${multiline}`
              )
            }
          }
        }
      }
    } finally {
      await browser.close()
      await server.close()
    }
  }
)
