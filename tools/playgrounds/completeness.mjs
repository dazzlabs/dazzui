export const PLATFORMS = [
  'flutter',
  'swiftui',
  'gpui',
  'winui3',
  'inset',
  'ios',
  'android',
]

/** Completion means every source story is declarative and emitted on every target. */
export function requireComplete(stories, scenes, report) {
  const missingDeclarations = stories
    .filter(story => !scenes.has(story.id))
    .map(s => s.id)
  const gaps = PLATFORMS.map(platform => {
    const generated = new Set(report.platforms[platform]?.generated ?? [])
    const missing = stories.filter(s => !generated.has(s.id)).map(s => s.id)
    return missing.length
      ? `${platform}: ${missing.length} missing (${missing.slice(0, 3).join(', ')})`
      : null
  }).filter(Boolean)
  if (missingDeclarations.length || gaps.length)
    throw new Error(
      `Shared-declaration migration is incomplete.\n${missingDeclarations.length} stories still contain React-only content/interaction.\n${gaps.join('\n')}\nSee tools/playgrounds/coverage.json. Native catalog entries and legacy renderers do not count as generated scenes.`
    )
}
