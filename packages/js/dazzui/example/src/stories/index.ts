import type { Story, StoryExtension } from '../lib/story'

type Modules = Record<string, { default: Story }>

const sorted = (modules: Modules) =>
  Object.values(modules)
    .map(module => module.default)
    .sort((a, b) => a.name.localeCompare(b.name))

const isStory = (entry: Story | StoryExtension): entry is Story =>
  'Preview' in entry

/**
 * The stories codegen compiles into the other platforms' examples: each a
 * file beside this one, named as the kit names the component. These are
 * also the scenes conformance reads.
 */
export const COMPILED: readonly Story[] = sorted(
  import.meta.glob<{ default: Story }>('./*.tsx', { eager: true })
)

const REACT = Object.values(
  import.meta.glob<{ default: Story | StoryExtension }>('./react/*.tsx', {
    eager: true,
  })
).map(module => module.default)

/**
 * Every component the example shows, by name: the compiled stories and the
 * ones in `react/`, for components only the React package has. Being in
 * either directory is all it takes to be listed. A file in `react/` that
 * carries only examples, under a compiled story's id, adds them to that
 * story's page instead.
 */
export const STORIES: readonly Story[] = [
  ...COMPILED.map(story => {
    const extra = REACT.filter(
      entry => !isStory(entry) && entry.id === story.id
    )
    return extra.length
      ? {
          ...story,
          examples: [...story.examples, ...extra.flatMap(e => e.examples)],
        }
      : story
  }),
  ...REACT.filter(isStory),
].sort((a, b) => a.name.localeCompare(b.name))

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
