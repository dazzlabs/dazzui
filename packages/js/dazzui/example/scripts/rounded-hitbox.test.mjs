import assert from 'node:assert/strict'
import { readFile, writeFile } from 'node:fs/promises'
import { test } from 'node:test'
import { chromium } from 'playwright'

const cases = JSON.parse(
  await readFile(
    new URL('./rounded-hitbox.cases.json', import.meta.url),
    'utf8'
  )
)
test('native CSS border and overflow curves determine click and wheel targets', async () => {
  const browser = await chromium.launch({
    headless: true,
    executablePath: process.env.DAZZUI_CHROMIUM_EXECUTABLE || undefined,
  })
  try {
    const observations = []
    for (const deviceScaleFactor of [1, 2]) {
      const page = await browser.newPage({
        viewport: { width: 160, height: 160 },
        deviceScaleFactor,
      })
      for (const fixture of cases) {
        const [top, right, bottom, left] = fixture.border
        const overflow =
          fixture.overflow === 'clip-x'
            ? 'overflow-x:clip;overflow-y:visible'
            : `overflow:${fixture.overflow}`
        const child = `<div id="child" class="target" style="transform:scale(${fixture.childFactor ?? 1});transform-origin:0 0;background:#00ff00;${fixture.overflow === 'clip-x' ? 'left:-24px;top:-24px;width:128px;height:128px' : 'inset:0'}"></div>`
        await page.setContent(
          `<style>html,body{margin:0;width:160px;height:160px}.target{position:absolute;box-sizing:border-box}</style><div style="position:absolute;left:0;top:0;width:160px;height:160px;transform:scale(${fixture.factor ?? 1});transform-origin:0 0"><div id="behind" class="target" style="inset:0;background:blue"></div><div id="host" class="target" style="left:${fixture.x ?? 24}px;top:${fixture.y ?? 24}px;width:${fixture.width}px;height:${fixture.height ?? fixture.width}px;background:white;border-style:solid;border-color:transparent;border-width:${top}px ${right}px ${bottom}px ${left}px;border-radius:${fixture.radius}px;${overflow}">${fixture.nested ? `<div id="inner" class="target" style="left:8px;top:8px;width:64px;height:64px;border-radius:20px;overflow:hidden">${child}</div>` : child}</div></div>`
        )
        await page.evaluate(() => {
          window.events = { click: [], wheel: [], mousemove: [] }
          for (const node of document.querySelectorAll('.target')) {
            for (const type of ['click', 'wheel', 'mousemove'])
              node.addEventListener(
                type,
                event => {
                  window.events[type].push(node.id)
                  event.preventDefault()
                  event.stopPropagation()
                },
                { passive: false }
              )
          }
        })
        for (const [
          x,
          y,
          expected,
          wheelExpected = expected,
          queryExpected = expected,
        ] of fixture.samples) {
          await page.mouse.move(159, 159)
          await page.evaluate(() => {
            window.events = { click: [], wheel: [], mousemove: [] }
          })
          await page.mouse.move(x, y)
          const hover = await page.evaluate(() => window.events.mousemove)
          const hit = await page.evaluate(
            ([x, y]) => document.elementFromPoint(x, y)?.id,
            [x, y]
          )
          await page.evaluate(() => {
            window.events = { click: [], wheel: [], mousemove: [] }
          })
          await page.mouse.click(x, y)
          const click = await page.evaluate(() => window.events.click)
          await page.evaluate(() => {
            window.events = { click: [], wheel: [], mousemove: [] }
          })
          await page.mouse.move(x, y)
          await page.mouse.wheel(0, 20)
          await page.waitForFunction(() => window.events.wheel.length > 0)
          const wheel = await page.evaluate(() => window.events.wheel)
          observations.push({
            deviceScaleFactor,
            fixture: fixture.name,
            x,
            y,
            expected,
            wheelExpected,
            queryExpected,
            hit,
            hover,
            click,
            wheel,
          })
        }
      }
      await page.close()
    }
    if (process.env.DAZZUI_ROUNDED_HIT_OUT) {
      await writeFile(
        process.env.DAZZUI_ROUNDED_HIT_OUT,
        JSON.stringify({ browser: browser.version(), observations }, null, 2) +
          '\n'
      )
    }
    console.log(
      `${observations.length} point queries across 1x/2x; hover, click and wheel targets recorded`
    )

    for (const p of observations) {
      assert.deepEqual(
        p.hover,
        [p.expected],
        `${p.fixture}: hover (${p.x},${p.y})`
      )
      assert.equal(p.hit, p.queryExpected, `${p.fixture}: hit (${p.x},${p.y})`)
      assert.deepEqual(
        p.click,
        [p.expected],
        `${p.fixture}: click (${p.x},${p.y})`
      )
      assert.deepEqual(
        p.wheel,
        [p.wheelExpected],
        `${p.fixture}: wheel (${p.x},${p.y})`
      )
    }
  } finally {
    await browser.close()
  }
})
