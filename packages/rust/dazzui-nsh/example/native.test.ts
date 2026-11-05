import { expect, open, test } from 'nsh/test'

// Run with `nsh test native.test.ts --target macos` (or linux / windows).
test('native desktop gallery', async () => {
  const app = await open('.', { width: 1280, height: 860 })
  await app.waitFor('One design. Native controls.')
  await app.press('label:Open Button')
  await app.waitFor('Variants')
  await app.press('label:Studio theme')
  await app.press('label:Dark appearance')
  await app.screenshot('../target/native-button-dark.png')
  await app.press('label:Open Checkbox')
  await app.waitFor('Unchecked')
  await app.press('Unchecked')
  await app.press('label:Open Menu')
  await app.waitFor('From a button')
  await app.press('Actions')
  await app.waitFor('Rename')
  await app.keyboard.press('escape')
  await app.press('File')
  await app.keyboard.press('right')
  await app.waitFor('Theme panel')
  await app.keyboard.press('escape')
  await app.press('label:Open PillTabs')
  const all = 'id:pill-tabs/0/0 >> text:All'
  const unread = 'id:pill-tabs/0/0 >> text:Unread'
  await app.press(all)
  const selectedInk = app.spanStyle(all).color
  await app.keyboard.press('right')
  expect(app.spanStyle(unread).color).toBe(selectedInk)
  await app.keyboard.press('left')
  expect(app.spanStyle(all).color).toBe(selectedInk)
  await app.press('label:Open FormField')
  await app.waitFor('In error')
  await app.type('id:form-field/1/1', 'hello@example.com')
  await app.press('label:Open Preferences')
  await app.waitFor('A section')
  await app.press('label:Open Switch')
  await app.waitFor('Sizes')
  await app.press('label:Open Dialog')
  await app.press('Rename…')
  await app.waitFor('Rename project')
  await app.type('label:Name', ' revised')
  await app.keyboard.press('escape')
  await app.press('label:Open DateRangePicker')
  await app.press('label:When >> nth=0')
  await app.waitFor('Quick dates')
  await app.press('Tomorrow')
  await app.press('role:checkbox[All day]')
  await app.screenshot('../target/native-date-range-dark.png')
  await app.press('Done')
  await app.press('label:Open Toast')
  await app.press('Save')
  await app.waitFor('Saved to Downloads')
  await app.press('label:Dismiss notification')
  for (const page of [
    'Accordion',
    'NumberField',
    'OtpField',
    'Calendar',
    'Table',
    'Tree',
    'Sidebar',
    'Splitter',
    'Skeleton',
  ]) {
    await app.press(`label:Open ${page}`)
  }
  await app.press('label:Open Overview')
  expect(app.texts()).toContain('One design. Native controls.')
}, 120000)
