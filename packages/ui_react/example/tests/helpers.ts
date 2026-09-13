import { expect, type Page } from '@playwright/test'
import type { Scene } from '../src/lib/scene'
import sceneData from '../src/generated/scenes.json' with { type: 'json' }
const scenes: Record<string, Scene> = sceneData
const clockPages = new WeakSet<Page>()

export async function show(page: Page, story: string, theme: string) {
  if (scenes[story]?.timers?.length && !clockPages.has(page)) {
    await page.clock.install({ time: 0 })
    await page.clock.pauseAt(1000)
    clockPages.add(page)
  }
  if (await page.evaluate(() => Boolean(window.__PLAYGROUND__))) {
    await page.evaluate(
      ({ story, theme }) => window.__PLAYGROUND__!.show(story, theme),
      { story, theme }
    )
  } else {
    await page.goto(`/index.html?canvas=1&story=${story}&theme=${theme}`)
  }
  if (clockPages.has(page)) {
    const ready = async () => {
      await page.clock.runFor(16)
      return page.evaluate(
        ({ story, theme }) => {
          const preview = window.__PLAYGROUND__
          if (preview?.error) throw new Error(preview.error)
          return (
            preview?.story === story && preview.theme === theme && preview.ready
          )
        },
        { story, theme }
      )
    }
    await expect.poll(ready).toBe(true)
    // Mount again after code and fonts have loaded so checkpoints start from
    // declared state, independently of the warm-up's elapsed virtual time.
    await page.evaluate(
      ({ story, theme }) => window.__PLAYGROUND__!.show(story, theme),
      { story, theme }
    )
    await expect.poll(ready).toBe(true)
  }
  await page.waitForFunction(
    ({ story, theme }) => {
      const preview = window.__PLAYGROUND__
      if (preview?.error) throw new Error(preview.error)
      return (
        preview?.story === story && preview.theme === theme && preview.ready
      )
    },
    { story, theme }
  )
}

export async function checkpoint(
  page: Page,
  point: NonNullable<Scene['checkpoints']>[number]
) {
  for (const step of point.steps) {
    if (step.action === 'advance') {
      await page.clock.runFor(step.milliseconds!)
      continue
    }
    const target = page.locator(`[data-scene-id="${step.target}"]`)
    if (step.action === 'click') await target.click()
    else if (step.action === 'fill') await target.fill(step.value!)
    else throw new Error(`Unknown action ${step.action}`)
  }
  for (const [id, text] of Object.entries(point.expectText ?? {})) {
    if (text === undefined) throw new Error(`Missing expected text for ${id}`)
    await expect(page.locator(`[data-scene-id="${id}"]`)).toHaveText(text)
  }
  for (const [id, disabled] of Object.entries(point.expectDisabled ?? {})) {
    const target = page.locator(`[data-scene-id="${id}"]`)
    if (disabled) await expect(target).toBeDisabled()
    else await expect(target).toBeEnabled()
  }
  if (point.expect)
    await expect
      .poll(() => page.evaluate(() => window.__SCENE_STATE__))
      .toMatchObject(point.expect)
  if (clockPages.has(page)) {
    await page.clock.runFor(40)
    return
  }
  await page.evaluate(
    () =>
      new Promise(resolve =>
        requestAnimationFrame(() => requestAnimationFrame(resolve))
      )
  )
}
