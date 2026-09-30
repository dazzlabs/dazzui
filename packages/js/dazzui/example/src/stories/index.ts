import type { Story } from '../lib/story'

/**
 * Every component the example shows, by name. Each is a file beside this
 * one, named as the kit names the component, and being here is all it takes
 * to be listed.
 */
const modules = import.meta.glob<{ default: Story }>('./*.tsx', { eager: true })

export const STORIES: readonly Story[] = Object.values(modules)
  .map(module => module.default)
  .sort((a, b) => a.name.localeCompare(b.name))

export const storyOf = (id: string | undefined) =>
  STORIES.find(story => story.id === id)

/** The components a query leaves. */
export function matching(query = ''): Story[] {
  const needle = query.trim().toLowerCase()
  return STORIES.filter(
    story =>
      story.name.toLowerCase().includes(needle) ||
      story.summary.toLowerCase().includes(needle)
  )
}
