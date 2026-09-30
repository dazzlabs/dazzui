import { useMemo } from 'react'

import type { Loaded, Scene, StoryScenes } from './model'
import { type Check, EMPTY, type Tally, add, checks, tally } from './verdict'

/*
 * Every verdict under one theme, worked out once and looked up by address,
 * scene or story. Recomputed when the theme changes and not otherwise.
 */
export interface Index {
  theme: string
  /** Whether the reference drew anything under this theme at all. */
  collected: boolean
  specimen: (platform: string, address: string) => Check[]
  scene: (platform: string, scene: Scene) => Tally
  story: (platform: string, story: StoryScenes) => Tally
  /** Over every story: the platform's whole standing. */
  total: (platform: string) => Tally
}

export function buildIndex(loaded: Loaded, theme: string): Index {
  const reference = loaded.readings[loaded.reference]!
  const collected = Boolean(reference.themes[theme])
  const bySpecimen = new Map<string, Check[]>()
  const byScene = new Map<string, Tally>()
  const byStory = new Map<string, Tally>()
  const totals = new Map<string, Tally>()

  for (const platform of loaded.implementations) {
    const implementation = loaded.readings[platform]!
    let total = { ...EMPTY }
    for (const story of loaded.scenes.stories) {
      let storyTally = { ...EMPTY }
      for (const scene of story.scenes) {
        let sceneTally = { ...EMPTY }
        for (const specimen of scene.specimens) {
          const list = checks(
            theme,
            specimen.address,
            reference,
            implementation,
            loaded.tolerances
          )
          bySpecimen.set(`${platform} ${specimen.address}`, list)
          sceneTally = add(sceneTally, tally(list))
        }
        byScene.set(`${platform} ${scene.id}`, sceneTally)
        storyTally = add(storyTally, sceneTally)
      }
      byStory.set(`${platform} ${story.id}`, storyTally)
      total = add(total, storyTally)
    }
    totals.set(platform, total)
  }

  return {
    theme,
    collected,
    specimen: (platform, address) =>
      bySpecimen.get(`${platform} ${address}`) ?? [],
    scene: (platform, scene) => byScene.get(`${platform} ${scene.id}`) ?? EMPTY,
    story: (platform, story) => byStory.get(`${platform} ${story.id}`) ?? EMPTY,
    total: platform => totals.get(platform) ?? EMPTY,
  }
}

export function useIndex(loaded: Loaded, theme: string) {
  return useMemo(() => buildIndex(loaded, theme), [loaded, theme])
}

/** The tallies of several platforms, added. */
export function across(
  platforms: readonly string[],
  read: (platform: string) => Tally
): Tally {
  return platforms.reduce((sum, p) => add(sum, read(p)), { ...EMPTY })
}
