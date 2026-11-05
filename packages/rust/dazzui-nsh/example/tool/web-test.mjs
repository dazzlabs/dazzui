import assert from 'node:assert/strict'
import { createRequire } from 'node:module'
import { readFileSync, mkdirSync } from 'node:fs'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

// Reuse the browser harness already installed for the React example.
const require = createRequire(
  new URL('../../../../js/dazzui/example/package.json', import.meta.url)
)
const { chromium } = require('playwright')
const example = resolve(dirname(fileURLToPath(import.meta.url)), '..')
const url = process.argv[2] ?? 'http://127.0.0.1:8127'
const coverage = JSON.parse(
  readFileSync(resolve(example, 'coverage.json'), 'utf8')
)
const ids = [...new Set(coverage.map(scene => scene.id.split('/')[0]))]
const stories = ids.map(id =>
  JSON.parse(
    readFileSync(resolve(example, 'src/stories', id + '.json'), 'utf8')
  )
)
const browser = await chromium.launch({ headless: true })
try {
  const page = await browser.newPage({ viewport: { width: 1280, height: 860 } })
  page.setDefaultTimeout(15000)
  const errors = []
  page.on('pageerror', error => {
    errors.push(error.message)
    console.error(error.message)
  })
  page.on('console', message => {
    if (message.type() === 'error') errors.push(message.text())
  })
  const response = await page.goto(url)
  assert.equal(response.status(), 200)
  await page
    .getByText('One design. Native controls.', { exact: true })
    .waitFor({ timeout: 60000 })
  const open = async name =>
    page
      .getByLabel('Open ' + name, { exact: true })
      .first()
      .click()
  for (const story of stories) {
    console.log('Checking ' + story.name)
    await open(story.name)
    await page.getByText(story.summary, { exact: true }).waitFor()
  }
  await open('OptionCard')
  const choice = page.getByTestId('option-card/0/0')
  const singleLine = await choice.boundingBox()
  await page.setViewportSize({ width: 720, height: 860 })
  await page.waitForFunction(
    height =>
      document
        .querySelector('[data-testid="option-card/0/0"]')
        .getBoundingClientRect().height > height,
    singleLine.height
  )
  await page.setViewportSize({ width: 1280, height: 860 })
  await page.waitForFunction(
    height =>
      document
        .querySelector('[data-testid="option-card/0/0"]')
        .getBoundingClientRect().height === height,
    singleLine.height
  )
  await open('PillTabs')
  const tabs = page.getByRole('tab')
  await tabs.first().click()
  await page.keyboard.press('ArrowRight')
  await page.waitForFunction(() =>
    document
      .querySelector('[role="tab"][aria-selected="true"]')
      ?.textContent.includes('Unread')
  )
  await page.keyboard.press('End')
  await page.waitForFunction(() =>
    document
      .querySelector('[role="tab"][aria-selected="true"]')
      ?.textContent.includes('Archived')
  )
  await open('SegmentedControl')
  const segments = page.getByTestId('segmented-control/0/0').getByRole('tab')
  await segments.first().click()
  await page.keyboard.press('ArrowRight')
  await page.waitForFunction(() =>
    document
      .querySelector(
        '[data-testid="segmented-control/0/0"] [role="tab"][aria-selected="true"]'
      )
      ?.textContent.includes('Board')
  )
  await open('Calendar')
  const calendar = page.getByTestId('calendar/0/0')
  await calendar.getByLabel('2026-10-16', { exact: true }).click()
  await page.keyboard.press('ArrowRight')
  await page.keyboard.press('Enter')
  assert.equal(
    await calendar
      .getByLabel('2026-10-17', { exact: true })
      .getAttribute('aria-selected'),
    'true'
  )
  await page.keyboard.press('PageDown')
  await page.keyboard.press('Enter')
  await calendar.getByLabel('2026-11-17', { exact: true }).waitFor()
  assert.equal(
    await calendar
      .getByLabel('2026-11-17', { exact: true })
      .getAttribute('aria-selected'),
    'true'
  )
  await open('Toggle')
  const marks = page.getByTestId('toggle/1/0').locator('[aria-pressed]')
  assert.equal(await marks.nth(0).getAttribute('aria-pressed'), 'true')
  await marks.nth(1).click()
  assert.equal(await marks.nth(0).getAttribute('aria-pressed'), 'true')
  assert.equal(await marks.nth(1).getAttribute('aria-pressed'), 'true')
  await open('Menu')
  await page.getByText('Actions', { exact: true }).click()
  await page.getByText('Rename', { exact: true }).waitFor()
  await page.keyboard.press('Escape')
  await open('Dialog')
  await page.getByText('Rename…', { exact: true }).click()
  await page.getByText('Rename project', { exact: true }).waitFor()
  await page.keyboard.press('Escape')
  await open('Toast')
  await page.getByText('Save', { exact: true }).click()
  await page.getByText('Saved to Downloads', { exact: true }).waitFor()
  await page.getByLabel('Dismiss notification', { exact: true }).click()
  await open('Button')
  await page.getByLabel('Studio theme', { exact: true }).click()
  await page.getByLabel('Dark appearance', { exact: true }).click()
  await page.getByText('Variants', { exact: true }).waitFor()
  mkdirSync(resolve(example, '../target'), { recursive: true })
  await page.screenshot({ path: resolve(example, '../target/web-smoke.png') })
  assert.deepEqual(errors, [], 'browser errors')
  console.log(
    `Web: ${stories.length} pages, responsive wrapping, keyboard tabs and calendar navigation, multiple toggles, menu, dialog, toast and theme switching passed at ${url}`
  )
} finally {
  await browser.close()
}
