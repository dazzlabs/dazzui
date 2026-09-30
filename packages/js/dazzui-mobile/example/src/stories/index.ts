import type { Story } from '../lib/story'

/**
 * Every story, by file. The overview leads; the rest sort by name.
 */
const modules = import.meta.glob<{ default: Story }>('./*.tsx', { eager: true })

export const STORIES: readonly Story[] = Object.values(modules)
  .map(module => module.default)
  .sort((a, b) =>
    a.id === 'overview'
      ? -1
      : b.id === 'overview'
        ? 1
        : a.name.localeCompare(b.name)
  )

export const storyOf = (id: string | undefined) =>
  STORIES.find(story => story.id === id)

export function matching(query = ''): Story[] {
  const needle = query.trim().toLowerCase()
  return STORIES.filter(
    story =>
      story.name.toLowerCase().includes(needle) ||
      story.summary.toLowerCase().includes(needle)
  )
}
