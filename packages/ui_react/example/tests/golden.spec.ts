import { expect, test } from '@playwright/test'
import catalog from '../src/generated/catalog.json' with { type: 'json' }
import sceneData from '../src/generated/scenes.json' with { type: 'json' }
import { THEMES } from '../src/lib/themes'
import type { Scene } from '../src/lib/scene'
import { checkpoint, show } from './helpers'

const scenes: Record<string, Scene> = sceneData
for (const story of catalog.stories) {
  for (const theme of THEMES) {
    for (const point of scenes[story.id]?.checkpoints ?? [
      { id: 'initial', steps: [] },
    ]) {
      test(`${story.id} / ${point.id} / ${theme.value}`, async ({ page }) => {
        await page.goto('/index.html?canvas=1')
        await show(page, story.id, theme.value)
        await checkpoint(page, point)
        await expect(page).toHaveScreenshot(
          [theme.value, story.id, `${point.id}-960x640.png`],
          {
            animations: 'disabled',
            fullPage: story.layout !== 'centered',
            scale: 'device',
            caret: 'hide',
            maxDiffPixels: 0,
          }
        )
      })
    }
  }
}
