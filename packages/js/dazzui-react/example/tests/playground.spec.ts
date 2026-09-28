import { expect, test } from '@playwright/test'
import catalog from '../src/generated/catalog.json' with { type: 'json' }
import scenes from '../src/generated/scenes.json' with { type: 'json' }
import { THEMES } from '../src/lib/themes'
import { checkpoint, show } from './helpers'

test('shared grids resize columns and preserve the last row width', async ({
  page,
}) => {
  await page.goto('/index.html?canvas=1')
  await show(page, 'components-card--overview', 'studio-light')
  const grid = page.locator('.dt-grid').first()
  for (const [width, columns] of [
    [960, 3],
    [720, 2],
    [400, 1],
  ]) {
    await page.setViewportSize({ width, height: 640 })
    await expect
      .poll(() =>
        grid.evaluate(
          element =>
            getComputedStyle(element).gridTemplateColumns.split(' ').length
        )
      )
      .toBe(columns)
    const widths = await grid
      .locator(':scope > .dz-card')
      .evaluateAll(elements =>
        elements.map(element => element.getBoundingClientRect().width)
      )
    expect(widths).toHaveLength(4)
    for (const value of widths)
      expect(Math.abs(value - widths[0])).toBeLessThan(0.1)
  }
})

test('timed scenes render updates and reset when revisited', async ({
  page,
}) => {
  await page.goto('/index.html?canvas=1')
  await show(page, 'components-progress--live', 'studio-light')
  await expect(page.getByRole('progressbar')).toHaveAttribute(
    'aria-valuenow',
    '8'
  )
  await page.clock.runFor(400)
  await expect(page.getByRole('progressbar')).toHaveAttribute(
    'aria-valuenow',
    '12'
  )
  await show(page, 'components-progress--default', 'studio-light')
  await page.clock.runFor(1200)
  await expect(page.getByRole('progressbar')).toHaveAttribute(
    'aria-valuenow',
    '43'
  )
  expect(await page.evaluate(() => window.__SCENE_STATE__)).toEqual({})
  await show(page, 'components-progress--live', 'studio-light')
  await expect(page.getByRole('progressbar')).toHaveAttribute(
    'aria-valuenow',
    '8'
  )
})

for (const theme of THEMES) {
  test(`every story renders in ${theme.value}`, async ({ page }) => {
    test.setTimeout(240_000)
    const errors: string[] = []
    page.on('pageerror', error => errors.push(error.message))
    await page.goto('/index.html?canvas=1')
    for (const story of catalog.stories) {
      await test.step(story.id, async () => {
        await show(page, story.id, theme.value)
        await expect(page.locator('#playground-root')).toBeAttached()
      })
    }
    expect(errors).toEqual([])
  })
}
for (const [id, scene] of Object.entries(scenes)) {
  if (!('checkpoints' in scene)) continue
  test(`shared checkpoints: ${id}`, async ({ page }) => {
    await page.goto('/index.html?canvas=1')
    for (const point of scene.checkpoints) {
      await show(page, id, 'studio-light')
      await checkpoint(page, point)
    }
  })
}
test('navigation, deep links, theme, reset and browser history', async ({
  page,
}) => {
  await page.goto('/?story=components-button--filled&theme=studio-dark')
  await expect(
    page.getByRole('link', { name: 'Filled', exact: true })
  ).toHaveAttribute('aria-current', 'page')
  await page
    .getByRole('searchbox', { name: 'Find a story' })
    .fill('TextField Default')
  await page.getByRole('link', { name: 'Default', exact: true }).click()
  await expect(page).toHaveURL(/story=components-textfield--default/)
  await page.getByLabel('Theme', { exact: true }).selectOption('studio-light')
  await expect(page.locator('html')).not.toHaveAttribute('data-theme')
  const canvas = page.frameLocator('iframe')
  await canvas.locator('[data-scene-id="field"]').fill('Changed')
  await page.getByRole('button', { name: 'Reset', exact: true }).click()
  await expect(canvas.locator('[data-scene-id="field"]')).toHaveValue(
    'Jianying Li'
  )
  await page.goBack()
  await expect(page.locator('html')).toHaveAttribute(
    'data-theme',
    'studio-dark'
  )
})
