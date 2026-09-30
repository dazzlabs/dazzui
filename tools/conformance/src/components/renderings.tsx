import { Badge } from '@dazzlabs/dazzui-react'

import { imageOf } from '../lib/data'
import type { Index } from '../lib/index'
import type { Loaded, Scene } from '../lib/model'
import { Chip } from './chip'

interface Props {
  loaded: Loaded
  index: Index
  scene: Scene
  platforms: readonly string[]
}

/**
 * A scene as every side drew it, one column each, at one scale, in the
 * theme the lens holds. The reference is first and is never judged.
 */
export function Renderings({ loaded, index, scene, platforms }: Props) {
  const columns = [loaded.reference, ...platforms]
  return (
    <div
      className="g-renderings"
      style={{ '--columns': columns.length } as React.CSSProperties}
    >
      {columns.map(platform => {
        const readings = loaded.readings[platform]!
        const src = imageOf(platform, readings.scenes[index.theme]?.[scene.id])
        const reference = platform === loaded.reference
        return (
          <figure key={platform} className="g-rendering">
            <figcaption className="g-rendering__head">
              <span className="g-rendering__title">{readings.title}</span>
              {reference ? (
                <Badge size="small" variant="outlined" tint="neutral">
                  reference
                </Badge>
              ) : (
                <Chip tally={index.scene(platform, scene)} />
              )}
            </figcaption>
            <div className="g-rendering__stage">
              {src ? (
                <img src={src} alt={`${scene.title} on ${readings.title}`} />
              ) : (
                <span className="g-quiet">not drawn</span>
              )}
            </div>
          </figure>
        )
      })}
    </div>
  )
}
