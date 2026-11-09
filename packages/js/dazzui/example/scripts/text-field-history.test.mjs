import assert from 'node:assert/strict'
import { readFile, writeFile } from 'node:fs/promises'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

const fixture = JSON.parse(
  await readFile(
    new URL('text-field-history.cases.json', import.meta.url),
    'utf8'
  )
)
const platform =
  process.env.DAZZUI_HISTORY_PLATFORM ||
  (process.platform === 'darwin' ? 'macos' : process.platform)

test(
  'TextField edit history follows native browser commands across fields',
  { skip: !['macos', 'linux'].includes(platform) },
  async () => {
    const server = await createServer({
      root: fileURLToPath(new URL('..', import.meta.url)),
      cacheDir: 'node_modules/.vite-test-text-field-history',
      logLevel: 'silent',
      server: {
        host: '127.0.0.1',
        port: Number(process.env.DAZZUI_HISTORY_PORT || 0),
      },
      plugins: [
        {
          name: 'text-field-history-fixture',
          resolveId: id =>
            id === '/text-field-history.js'
              ? '\0text-field-history'
              : undefined,
          load: id =>
            id === '\0text-field-history'
              ? `
        import {createElement as h,useState} from 'react';import {createRoot} from 'react-dom/client';import {flushSync} from 'react-dom';import {TextField} from '@dazzlabs/dazzui';
        const host=document.createElement('div');host.id='text-field-history';document.body.append(host);const root=createRoot(host);let key=0;
        function Field({c,mode,value,i}) {const [text,setText]=useState(value);return h(TextField,{id:'f'+i,'aria-label':'Text '+i,multiline:mode==='multi',type:mode==='password'?'password':undefined,maxLength:c.limit,...(c.controlled?{value:text,onValueChange:setText}:{defaultValue:value})})}
        export function mount(c,mode) {flushSync(()=>root.render(h('div',{key:++key},...(c.fields||[c.initial||'']).map((value,i)=>h(Field,{key:i,c,mode,value,i})))));const n=host.querySelector('input,textarea');n.focus();n.setSelectionRange(n.value.length,n.value.length);}
      `
              : undefined,
        },
      ],
    })
    await server.listen()
    const browser = process.env.DAZZUI_CHROMIUM_CDP
      ? await chromium.connectOverCDP(process.env.DAZZUI_CHROMIUM_CDP)
      : await chromium.launch({
          executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE || undefined,
        })
    try {
      const context = await browser.newContext({
        permissions: ['clipboard-read', 'clipboard-write'],
      })
      const page = await context.newPage()
      await page.goto(
        new URL('conformance.html', server.resolvedUrls.local[0]).href
      )
      await page.waitForFunction(() => window.conformance)
      await page.evaluate(async () => {
        await window.conformance.show('text-field/preview', 'studio-light')
        document.getElementById('root').style.display = 'none'
        window.textFieldHistory = await import('/text-field-history.js')
      })
      const cdp = await context.newCDPSession(page)
      const primary = platform === 'macos' ? 'Meta' : 'Control'
      const keys = {
        undo: `${primary}+z`,
        redo: `${primary}+Shift+z`,
        redoAlt: 'Control+y',
        paste: `${primary}+v`,
        cut: `${primary}+x`,
        start: platform === 'macos' ? 'Meta+ArrowUp' : 'Control+Home',
        left: 'ArrowLeft',
        right: 'ArrowRight',
        sleft: 'Shift+ArrowLeft',
        sright: 'Shift+ArrowRight',
        backspace: 'Backspace',
        delete: 'Delete',
        enter: 'Enter',
        wordBackspace:
          platform === 'macos' ? 'Alt+Backspace' : 'Control+Backspace',
      }
      const results = [],
        failures = []
      for (const mode of ['text', 'multi', 'password'])
        for (const c of fixture.cases) {
          if (
            (c.multiline && mode !== 'multi') ||
            (c.platforms && !c.platforms.includes(platform))
          )
            continue
          await page.evaluate(
            ({ c, mode }) => window.textFieldHistory.mount(c, mode),
            { c, mode }
          )
          const states = []
          for (const [i, op] of c.ops.entries()) {
            switch (op[0]) {
              case 'type':
                await page.keyboard.type(op[1])
                break
              case 'insert':
              case 'commit':
                await page.keyboard.insertText(op[1])
                break
              case 'key':
                await page.keyboard.press(keys[op[1]])
                break
              case 'paste':
                await page.evaluate(
                  text => navigator.clipboard.writeText(text),
                  op[1]
                )
                await page.keyboard.press(keys.paste)
                break
              case 'value':
                await page.evaluate(text => {
                  document.activeElement.value = text
                }, op[1])
                break
              case 'readonly':
                await page.evaluate(value => {
                  document.activeElement.readOnly = value
                }, op[1])
                break
              case 'maxLength':
                await page.evaluate(value => {
                  document.activeElement.maxLength = value
                }, op[1])
                break
              case 'blur':
                await page.evaluate(() => {
                  const n = document.activeElement
                  n.blur()
                  n.focus()
                })
                break
              case 'focus':
                await page.evaluate(i => {
                  const n = document.getElementById('f' + i)
                  n.focus()
                  n.setSelectionRange(n.value.length, n.value.length)
                }, op[1])
                break
              case 'ime':
                await cdp.send('Input.imeSetComposition', {
                  text: op[1],
                  selectionStart: op[2],
                  selectionEnd: op[3],
                })
                break
              case 'wait':
                await new Promise(resolve => setTimeout(resolve, op[1]))
                break
              default:
                throw Error('Unknown operation ' + op)
            }
            // Match a native rendered frame before the next command; in
            // particular, readonly changes must reach the editor computed style.
            await page.evaluate(() =>
              document.activeElement.getBoundingClientRect()
            )
            const actual = await page.evaluate(() => {
              const n = document.activeElement
              return {
                values: [
                  ...document.querySelectorAll(
                    '#text-field-history input,#text-field-history textarea'
                  ),
                ].map(n => n.value),
                focus: Number(n.id.slice(1)),
                range: [
                  n.selectionStart,
                  n.selectionEnd,
                  n.selectionDirection === 'backward',
                ],
              }
            })
            states.push(actual)
            const expected = fixture.platforms[platform].results.find(
              trace => trace.id === c.id && trace.mode === mode
            )?.states[i]
            if (!expected && !process.env.DAZZUI_HISTORY_RECORD)
              failures.push(`${mode}/${c.id}/${i}: missing recorded checkpoint`)
            if (
              expected &&
              JSON.stringify(actual) !==
                JSON.stringify({
                  values: expected.values,
                  focus: expected.focus,
                  range: expected.range,
                })
            )
              failures.push(
                `${mode}/${c.id}/${i}: ${JSON.stringify(actual)} != ${JSON.stringify(expected)}`
              )
          }
          results.push({ id: c.id, mode, states })
        }
      if (process.env.DAZZUI_HISTORY_RECORD)
        await writeFile(
          process.env.DAZZUI_HISTORY_RECORD,
          JSON.stringify(
            {
              browser: await (await browser.newBrowserCDPSession()).send(
                'Browser.getVersion'
              ),
              results,
            },
            null,
            2
          ) + '\n'
        )
      assert.deepEqual(failures, [])
    } finally {
      await browser.close()
      await server.close()
    }
  }
)
