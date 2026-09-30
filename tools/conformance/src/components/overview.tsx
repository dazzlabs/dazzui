import { Button, Card, Meter } from '@dazzlabs/dazzui'

import { imageOf } from '../lib/data'
import { percent } from '../lib/format'
import type { Index } from '../lib/index'
import type { Loaded, StoryScenes } from '../lib/model'
import { score } from '../lib/verdict'

interface Props {
  loaded: Loaded
  index: Index
  stories: readonly StoryScenes[]
  platforms: readonly string[]
  open: (page: string) => void
}

/**
 * Every story as a tile: the reference's rendering of its preview, and under
 * it a meter per implementation. The grid is the example's.
 */
export function Overview({ loaded, index, stories, platforms, open }: Props) {
  const reference = loaded.readings[loaded.reference]!
  return (
    <div className="gk-page">
      <div className="gk-grid">
        {stories.map(story => {
          const preview = story.scenes[0]
          const src = preview
            ? imageOf(
                loaded.reference,
                reference.scenes[index.theme]?.[preview.id]
              )
            : undefined
          return (
            <Card key={story.id} variant="outlined" className="gk-tile">
              <div className="gk-stage g-tile__stage">
                {src ? (
                  <img src={src} alt="" />
                ) : (
                  <span className="g-quiet">not drawn</span>
                )}
              </div>
              <div className="g-tile__meters">
                {platforms.map(platform => {
                  const value = score(index.story(platform, story))
                  return (
                    <Meter
                      key={platform}
                      size="small"
                      value={Math.round((value ?? 0) * 100)}
                      tint={tintFor(value)}
                      label={
                        <span className="g-tile__meter">
                          <span>{loaded.readings[platform]!.title}</span>
                          <span className="g-quiet">{percent(value)}</span>
                        </span>
                      }
                    />
                  )
                })}
              </div>
              <Button variant="plain" fullWidth onClick={() => open(story.id)}>
                {story.name}
              </Button>
            </Card>
          )
        })}
      </div>
    </div>
  )
}

export function tintFor(value: number | null) {
  if (value === null) return 'neutral' as const
  if (value >= 0.98) return 'success' as const
  if (value >= 0.9) return 'warning' as const
  return 'danger' as const
}
