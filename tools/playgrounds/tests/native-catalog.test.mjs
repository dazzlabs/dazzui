import test from 'node:test'
import assert from 'node:assert/strict'
import fs from 'node:fs'
import { gpuiCatalog } from '../native-catalog.mjs'

const manifest = JSON.parse(
  fs.readFileSync(
    new URL(
      '../../../packages/ui_react/example/src/generated/catalog.json',
      import.meta.url
    )
  )
).stories

test('GPUI catalog keeps every native page, including multiline declarations', () => {
  const output = gpuiCatalog(manifest, [])
  for (const page of ['Collapsible', 'Skeleton', 'Tag', 'Tree'])
    assert.ok(output.includes(`StoryPage::new("${page}",`), page)
  const ids = [...output.matchAll(/source_id: Some\("([^"]+)"\)/g)].map(
    m => m[1]
  )
  assert.equal(new Set(ids).size, ids.length)
})

test('new portable GPUI pages need no native registry placeholders', () => {
  const item = {
    id: 'components-newwidget--stable-export',
    title: 'Components/NewWidget',
    name: 'Display name',
    layout: 'centered',
  }
  const output = gpuiCatalog([...manifest, item], [item])
  assert.ok(output.includes('StoryPage::new("NewWidget",'))
  assert.ok(
    output.includes('build("components-newwidget--stable-export", state, cx)')
  )
  assert.ok(
    output.includes(
      'source_id: Some("components-newwidget--stable-export"), name: "Display name"'
    )
  )
})

test('a fully migrated page is supplied entirely by shared declarations', () => {
  const buttons = manifest.filter(s => s.title === 'Components/Button')
  const output = gpuiCatalog(manifest, buttons)
  assert.ok(output.includes('StoryPage::new("Button",'))
  assert.ok(!output.includes('crate::stories::button::PAGE'))
  for (const story of buttons)
    assert.ok(output.includes(`build("${story.id}", state, cx)`))
})
