/** The key a support list uses for a component that takes children. */
export const CHILDREN = '#children'

/**
 * Every component and prop the stories set that a platform's list does not
 * name — each one something that platform would silently leave out.
 */
export function gaps(stories, support, platform) {
  const found = new Set()
  const visit = (value, where) => {
    if (Array.isArray(value)) return value.forEach(item => visit(item, where))
    if (!value || typeof value !== 'object') return
    if ('$el' in value) {
      const known = support[value.$el]
      if (!known)
        found.add(`${where}: <${value.$el}> is not drawn by ${platform}`)
      else {
        for (const prop of Object.keys(value.props ?? {}))
          if (!known.includes(prop))
            found.add(
              `${where}: <${value.$el} ${prop}> is not drawn by ${platform}`
            )
        if (value.children && !known.includes(CHILDREN))
          found.add(`${where}: <${value.$el}> takes no children on ${platform}`)
      }
    }
    for (const item of Object.values(value)) visit(item, where)
  }
  for (const story of stories) {
    visit(story.preview, `${story.id} preview`)
    for (const example of story.examples)
      visit(example.scene, `${story.id} / ${example.title}`)
  }
  return [...found]
}
