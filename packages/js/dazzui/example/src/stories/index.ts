import type { Story } from '../lib/story'

type Modules = Record<string, { default: Story }>

const sorted = (modules: Modules) =>
  Object.values(modules)
    .map(module => module.default)
    .sort((a, b) => a.name.localeCompare(b.name))

/**
 * The stories codegen compiles into the other platforms' examples: each a
 * file beside this one, named as the kit names the component. These are
 * also the scenes conformance reads.
 */
export const COMPILED: readonly Story[] = sorted(
  import.meta.glob<{ default: Story }>('./*.tsx', { eager: true })
)

/**
 * Every component the example shows, by name: the compiled stories and the
 * ones in `react/`, for components only the React package has. Being in
 * either directory is all it takes to be listed.
 */
export const STORIES: readonly Story[] = sorted({
  ...import.meta.glob<{ default: Story }>('./*.tsx', { eager: true }),
  ...import.meta.glob<{ default: Story }>('./react/*.tsx', { eager: true }),
})

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
