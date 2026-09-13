import fs from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { sanitize } from './catalog.mjs'

const directory = fileURLToPath(
  new URL('../../packages/ui_gpui/example/src/stories/', import.meta.url)
)
const q = JSON.stringify

/** Native implementations supply render functions; React owns matching IDs/names. */
export function gpuiCatalog(stories, generated) {
  const pages = new Map()
  for (const file of fs.readdirSync(directory, { recursive: true }).sort()) {
    if (!file.endsWith('.rs') || file.endsWith('mod.rs')) continue
    const source = fs.readFileSync(path.join(directory, file), 'utf8')
    const name =
      /pub const PAGE: StoryPage =\s*StoryPage::new\(\s*"([^"]+)"/.exec(
        source
      )?.[1]
    if (!name) continue
    const group = file.startsWith('tokens/') ? 'Tokens' : 'Components'
    const title = `${group}/${name}`
    const symbol = `crate::stories::${file.slice(0, -3).replaceAll('/', '::')}::PAGE`
    const nativeStories = [
      ...source.matchAll(/Story::(?:new|page|fullscreen)\(\s*"([^"]+)"/g),
    ]
    pages.set(
      title,
      nativeStories.map((match, index) => {
        const id = `${sanitize(title)}--${sanitize(match[1])}`
        const canonical = stories.find(
          s =>
            s.id === id ||
            (s.title === title &&
              s.name.toLowerCase() === match[1].toLowerCase())
        )
        return {
          id: canonical?.id ?? id,
          name: canonical?.name ?? match[1],
          render: `${symbol}.stories[${index}].render`,
          layout: `${symbol}.stories[${index}].layout`,
        }
      })
    )
  }
  for (const story of generated) {
    const entries = pages.get(story.title) ?? []
    const item = {
      id: story.id,
      name: story.name,
      render: `|state, _, cx| build(${q(story.id)}, state, cx).expect("generated scene")`,
      layout: `StoryLayout::${story.layout === 'fullscreen' ? 'Fullscreen' : story.layout === 'padded' ? 'Page' : 'Centered'}`,
    }
    const index = entries.findIndex(entry => entry.id === story.id)
    if (index < 0) entries.push(item)
    else entries[index] = item
    pages.set(story.title, entries)
  }
  const order = [...new Set(stories.map(s => s.title))]
  const titles = [...pages.keys()].sort((a, b) => {
    const rank = x => (order.includes(x) ? order.indexOf(x) : order.length)
    return rank(a) - rank(b) || a.localeCompare(b)
  })
  return `use crate::storybook::{Story, StoryGroup, StoryPage, StoryLayout};
${titles
  .map(
    (title, i) => `const PAGE_${i}_STORIES: &[Story] = &[
${pages
  .get(title)
  .map(
    s =>
      `Story { source_id: Some(${q(s.id)}), name: ${q(s.name)}, layout: ${s.layout}, render: ${s.render} },`
  )
  .join('\n')}
];`
  )
  .join('\n')}
pub const GROUPS: &[StoryGroup] = &[
${['Tokens', 'Components']
  .map(
    group => `StoryGroup::new(${q(group)}, &[
${titles.map((title, i) => (title.startsWith(group + '/') ? `StoryPage::new(${q(title.split('/')[1])}, PAGE_${i}_STORIES),` : '')).join('\n')}
]),`
  )
  .join('\n')}
];`
}
