import assert from 'node:assert/strict'
import { readFile, writeFile } from 'node:fs/promises'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

const fixture = JSON.parse(
  await readFile(
    new URL('text-field-shift-selection.cases.json', import.meta.url),
    'utf8'
  )
)
const platform =
  process.env.DAZZUI_HISTORY_PLATFORM ||
  (process.platform === 'darwin' ? 'macos' : process.platform)

test(
  'Shift pointer selection preserves word and paragraph granularity',
  { skip: !['macos', 'linux'].includes(platform) },
  async () => {
    const server = await createServer({
      root: fileURLToPath(new URL('..', import.meta.url)),
      cacheDir: 'node_modules/.vite-test-text-field-shift-selection',
      logLevel: 'silent',
      server: {
        host: '127.0.0.1',
        port: Number(process.env.DAZZUI_HISTORY_PORT || 0),
      },
      plugins: [
        {
          name: 'shift-selection-fixture',
          resolveId: id =>
            id === '/shift-selection.js' ? '\0shift-selection' : undefined,
          load: id =>
            id === '\0shift-selection'
              ? `
   import {createElement as h} from 'react';import{createRoot}from'react-dom/client';import{flushSync}from'react-dom';import{TextField}from'@dazzlabs/dazzui';
   const host=document.createElement('div');host.id='shift-selection';document.body.append(host);const root=createRoot(host);let key=0;
   export function mount(c,font){flushSync(()=>root.render(h(TextField,{key:++key,'aria-label':'Text',defaultValue:c.text,multiline:!!c.multiline,readOnly:!!c.readOnly,type:c.type,style:{font:font.size+'px '+font.family,lineHeight:font.lineHeight+'px',width:c.width||font.width,height:c.multiline?font.height:font.inputHeight,minHeight:0,padding:0,border:0,letterSpacing:0,direction:'ltr'}})));const input=host.querySelector('input,textarea');input.focus();input.setSelectionRange(0,0);input.scrollLeft=0;input.scrollTop=0;}
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
      const page = await browser.newPage()
      await page.goto(
        new URL('conformance.html', server.resolvedUrls.local[0]).href
      )
      await page.waitForFunction(() => window.conformance)
      await page.evaluate(async () => {
        await window.conformance.show('text-field/preview', 'studio-light')
        document.getElementById('root').style.display = 'none'
        window.shiftSelection = await import('/shift-selection.js')
      })
      const results = [],
        failures = []
      for (const c of fixture.cases) {
        await page.evaluate(
          ({ c, font }) => window.shiftSelection.mount(c, font),
          { c, font: fixture.font }
        )
        const input = page.locator(
            '#shift-selection input,#shift-selection textarea'
          ),
          frame = await input.boundingBox(),
          states = []
        let shifted = false
        for (const [i, op] of c.ops.entries()) {
          const [kind, x, y, count = 1, shift = false] = op
          if (kind === 'click' || kind === 'down') {
            if (shift) {
              await page.keyboard.down('Shift')
              shifted = true
            }
            await page.mouse.move(frame.x + x, frame.y + y)
            if (kind === 'click') {
              await page.mouse.click(frame.x + x, frame.y + y, {
                clickCount: count,
              })
              if (shifted) {
                await page.keyboard.up('Shift')
                shifted = false
              }
            } else await page.mouse.down({ clickCount: count })
          } else if (kind === 'move')
            await page.mouse.move(frame.x + x, frame.y + y)
          else if (kind === 'up') {
            await page.mouse.up({ clickCount: count })
            if (shifted) {
              await page.keyboard.up('Shift')
              shifted = false
            }
          } else if (kind === 'key')
            await page.keyboard.press(
              {
                all: platform === 'macos' ? 'Meta+a' : 'Control+a',
                right: 'ArrowRight',
                left: 'ArrowLeft',
                'shift-right': 'Shift+ArrowRight',
              }[x]
            )
          else throw Error('Unknown operation ' + op)
          const actual = await input.evaluate(n => [
            n.selectionStart,
            n.selectionEnd,
            n.selectionDirection === 'backward',
          ])
          states.push(actual)
          const expected = fixture.platforms[platform]?.results.find(
            t => t.id === c.id
          )?.states[i]
          if (!expected && !process.env.DAZZUI_SHIFT_RECORD)
            failures.push(`${c.id}/${i}: missing checkpoint`)
          if (expected && JSON.stringify(actual) !== JSON.stringify(expected))
            failures.push(
              `${c.id}/${i}: ${JSON.stringify(actual)} != ${JSON.stringify(expected)}`
            )
        }
        results.push({ id: c.id, states })
      }
      if (process.env.DAZZUI_SHIFT_RECORD)
        await writeFile(
          process.env.DAZZUI_SHIFT_RECORD,
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
