import assert from 'node:assert/strict'
import { readFile } from 'node:fs/promises'
import { fileURLToPath } from 'node:url'
import { test } from 'node:test'
import { chromium } from 'playwright'
import { createServer } from 'vite'

const { cases, platforms, wrappedCase } = JSON.parse(
  await readFile(
    new URL('text-field-word-selection.cases.json', import.meta.url),
    'utf8'
  )
)

const paragraphFixtures = JSON.parse(
  await readFile(
    new URL('text-field-paragraph-selection.cases.json', import.meta.url),
    'utf8'
  )
)

test(
  'word and paragraph pointer selection follows browser gestures',
  {
    skip: platforms.includes(process.platform)
      ? false
      : 'Fixtures recorded on macOS and Linux',
  },
  async () => {
    const server = await createServer({
      root: fileURLToPath(new URL('..', import.meta.url)),
      cacheDir: 'node_modules/.vite-test-text-field-word-selection',
      logLevel: 'silent',
      server: { host: '127.0.0.1', port: 0 },
      plugins: [
        {
          name: 'text-field-word-selection-fixture',
          resolveId: id =>
            id === '/text-field-word-selection.js'
              ? '\0text-field-word-selection'
              : undefined,
          load: id =>
            id === '\0text-field-word-selection'
              ? `
        import {createElement as h} from 'react';import {createRoot} from 'react-dom/client';import {flushSync} from 'react-dom';import {TextField} from '@dazzlabs/dazzui';
        const host=document.createElement('div');host.id='text-field-word-selection';document.body.append(host);const root=createRoot(host);let key=0;
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
        window.textFieldWordSelection =
          await import('/text-field-word-selection.js')
      })
      for (const multiline of [false, true]) {
        await page.evaluate(
          multiline =>
            window.textFieldWordSelection.mount({
              multiline,
              style: {
                font: '16px Arial',
                lineHeight: '28px',
                width: 1100,
                height: 28,
                padding: 0,
                border: 0,
                letterSpacing: 0,
                direction: 'ltr',
              },
            }),
          multiline
        )
        const input = page.locator(
          `#text-field-word-selection ${multiline ? 'textarea' : 'input'}`
        )
        for (const fixture of cases) {
          await input.fill(fixture.text)
          const points = await input.evaluate((node, hits) => {
            node.setSelectionRange(0, 0)
            node.scrollLeft = 0
            const mirror = document.createElement('span'),
              style = getComputedStyle(node)
            Object.assign(mirror.style, {
              position: 'absolute',
              left: '0',
              top: '300px',
              whiteSpace: 'pre',
              font: style.font,
              letterSpacing: style.letterSpacing,
              direction: 'ltr',
              pointerEvents: 'none',
            })
            mirror.textContent = node.value
            document.body.append(mirror)
            const frame = node.getBoundingClientRect(),
              origin = mirror.getBoundingClientRect()
            const points = hits.map(([start, end, fraction]) => {
              const range = document.createRange()
              range.setStart(mirror.firstChild, start)
              range.setEnd(mirror.firstChild, end)
              const rect = range.getBoundingClientRect()
              return {
                x:
                  frame.left +
                  node.clientLeft +
                  parseFloat(style.paddingLeft) +
                  rect.left -
                  origin.left +
                  rect.width * fraction,
                y:
                  frame.top +
                  node.clientTop +
                  parseFloat(style.paddingTop) +
                  parseFloat(style.lineHeight) / 2,
              }
            })
            mirror.remove()
            return points
          }, fixture.hits)
          const visited = new Set()
          for (const point of points) {
            await page.mouse.click(point.x, point.y)
            const at = await input.evaluate(node => node.selectionStart)
            visited.add(at)
            const expected = fixture.checks.find(check => check[0] === at)
            assert.ok(
              expected,
              `${fixture.id}: unrecorded insertion point ${at}`
            )
            await page.mouse.click(point.x, point.y, { clickCount: 2 })
            const selected = await input.evaluate(node => [
              node.selectionStart,
              node.selectionEnd,
            ])
            assert.deepEqual(
              selected,
              expected.slice(1),
              `${fixture.id}/${at}/${multiline}`
            )
          }
          assert.ok(
            visited.size > 1,
            `${fixture.id}: hit points must reach different insertion positions`
          )
        }
        for (const path of [[11.25], [1.25], [1.25, 5.25], [11.25, 5.25]]) {
          await input.fill('one two three four')
          const points = await input.evaluate(node => {
            node.setSelectionRange(0, 0)
            node.scrollLeft = 0
            const mirror = document.createElement('span'),
              style = getComputedStyle(node)
            Object.assign(mirror.style, {
              position: 'absolute',
              left: '0',
              top: '300px',
              whiteSpace: 'pre',
              font: style.font,
              letterSpacing: style.letterSpacing,
              direction: 'ltr',
              pointerEvents: 'none',
            })
            mirror.textContent = node.value
            document.body.append(mirror)
            const frame = node.getBoundingClientRect(),
              origin = mirror.getBoundingClientRect()
            const points = [...node.value].map((_, start) => {
              const range = document.createRange()
              range.setStart(mirror.firstChild, start)
              range.setEnd(mirror.firstChild, start + 1)
              const rect = range.getBoundingClientRect()
              return {
                x:
                  frame.left +
                  node.clientLeft +
                  parseFloat(style.paddingLeft) +
                  rect.left -
                  origin.left +
                  rect.width / 4,
                y:
                  frame.top +
                  node.clientTop +
                  parseFloat(style.paddingTop) +
                  parseFloat(style.lineHeight) / 2,
              }
            })
            mirror.remove()
            return points
          })
          await page.mouse.move(points[5].x, points[5].y)
          await page.mouse.down({ clickCount: 2 })
          for (const column of path) {
            const point = points[Math.floor(column)]
            await page.mouse.move(point.x, point.y)
          }
          const backward = path[0] === 1.25 && process.platform !== 'darwin'
          const range =
            path.length > 1 ? [4, 7] : path[0] > 5 ? [4, 13] : [0, 7]
          const state = () =>
            input.evaluate(node => [
              node.selectionStart,
              node.selectionEnd,
              node.selectionDirection === 'backward',
            ])
          assert.deepEqual(await state(), [...range, backward])
          await page.mouse.up({ clickCount: 2 })
          await input.press('Shift+ArrowRight')
          assert.deepEqual(
            await state(),
            backward
              ? [range[0] + 1, range[1], true]
              : [range[0], range[1] + 1, false]
          )
        }
      }
      await page.evaluate(
        fixture =>
          window.textFieldWordSelection.mount({
            multiline: true,
            style: {
              font: `${fixture.fontSize}px monospace`,
              lineHeight: `${fixture.lineHeight}px`,
              width: fixture.width,
              padding: 0,
              border: 0,
              letterSpacing: 0,
              direction: 'ltr',
            },
          }),
        wrappedCase
      )
      const wrapped = page.locator('#text-field-word-selection textarea')
      await wrapped.fill(wrappedCase.text)
      await wrapped.evaluate(node => {
        node.setSelectionRange(0, 0)
        node.scrollTop = 0
      })
      const frame = await wrapped.boundingBox()
      for (const [x, y, start, end] of wrappedCase.points) {
        await page.mouse.click(frame.x + x, frame.y + y, { clickCount: 2 })
        assert.deepEqual(
          await wrapped.evaluate(node => [
            node.selectionStart,
            node.selectionEnd,
          ]),
          [start, end],
          `wrapped/${x}/${y}`
        )
      }
      for (const fixture of paragraphFixtures.cases) {
        await page.evaluate(
          ({ fixture, layout }) =>
            window.textFieldWordSelection.mount({
              multiline: fixture.multiline,
              readOnly: !!fixture.readOnly,
              ...(fixture.type ? { type: fixture.type } : {}),
              defaultValue: fixture.text,
              style: {
                font: `${layout.size}px monospace`,
                lineHeight: `${layout.lineHeight}px`,
                width: layout.width,
                height: fixture.multiline ? layout.height : layout.inputHeight,
                minHeight: 0,
                padding: 0,
                border: 0,
                letterSpacing: 0,
                direction: 'ltr',
              },
            }),
          { fixture, layout: paragraphFixtures.font }
        )
        const input = page.locator(
          `#text-field-word-selection ${fixture.multiline ? 'textarea' : 'input'}`
        )
        const reset = () =>
          input.evaluate(node => {
            node.focus()
            node.setSelectionRange(0, 0)
            node.scrollLeft = 0
            node.scrollTop = 0
          })
        const state = () =>
          input.evaluate(node => [
            node.selectionStart,
            node.selectionEnd,
            node.selectionDirection === 'backward',
          ])
        const frame = await input.boundingBox()
        for (const check of fixture.clicks) {
          await reset()
          const [x, y] = check.point
          await page.mouse.click(frame.x + x, frame.y + y)
          await page.mouse.click(frame.x + x, frame.y + y, { clickCount: 3 })
          assert.deepEqual(
            await state(),
            check.selected,
            `${fixture.id}/${x}/${y}`
          )
          await page.keyboard.press('Shift+ArrowRight')
          assert.deepEqual(
            await state(),
            check.afterShiftRight,
            `${fixture.id}/${x}/${y}/keyboard`
          )
        }
        for (const check of fixture.drags) {
          await reset()
          const expected = check.expectations[process.platform]
          await page.mouse.move(
            frame.x + check.from[0],
            frame.y + check.from[1]
          )
          await page.mouse.down({ clickCount: 3 })
          assert.deepEqual(
            await state(),
            expected.states[0],
            `${fixture.id}/drag/start`
          )
          for (const [index, [x, y]] of check.to.entries()) {
            await page.mouse.move(frame.x + x, frame.y + y)
            assert.deepEqual(
              await state(),
              expected.states[index + 1],
              `${fixture.id}/drag/${index}`
            )
          }
          await page.mouse.up({ clickCount: 3 })
          await page.keyboard.press('Shift+ArrowRight')
          assert.deepEqual(
            await state(),
            expected.afterShiftRight,
            `${fixture.id}/drag/keyboard`
          )
        }
      }
    } finally {
      await browser.close()
      await server.close()
    }
  }
)
