import { Card, SectionLabel } from '@dazzlabs/dazzui'

import { plural } from '../lib/format'
import { type Index, across } from '../lib/index'
import type { Lens } from '../lib/lens'
import type { Loaded, StoryScenes } from '../lib/model'
import { Chip } from './chip'
import { Renderings } from './renderings'
import { Specimens } from './specimens'

interface Props {
  loaded: Loaded
  index: Index
  lens: Lens
  story: StoryScenes
}

/** A story's page: its scenes in order, each drawn and then measured. */
export function Document({ loaded, index, lens, story }: Props) {
  const specimens = story.scenes.reduce((n, s) => n + s.specimens.length, 0)
  return (
    <div className="gk-page">
      <p className="gk-lead">
        {story.summary}{' '}
        <span className="g-quiet">
          {plural(story.scenes.length, 'scene')},{' '}
          {plural(specimens, 'specimen')}, held to{' '}
          {loaded.readings[loaded.reference]!.title} in {index.theme}.
        </span>
      </p>
      {!index.collected && (
        <p className="g-quiet">
          The reference was not collected in this theme. Choose another in the
          lens, or rerun the collection.
        </p>
      )}
      {story.scenes.map(scene => (
        <section key={scene.id} className="gk-section">
          <div className="g-section-head">
            <SectionLabel>{scene.title}</SectionLabel>
            <span className="g-quiet">
              {plural(scene.specimens.length, 'specimen')}
            </span>
            <span className="gk-space" />
            <Chip tally={across(lens.platforms, p => index.scene(p, scene))} />
          </div>
          <Card variant="outlined" size="large" className="g-scene">
            <Renderings
              loaded={loaded}
              index={index}
              scene={scene}
              platforms={lens.platforms}
            />
            <Specimens
              loaded={loaded}
              index={index}
              scene={scene}
              platforms={lens.platforms}
              show={lens.show}
            />
          </Card>
        </section>
      ))}
    </div>
  )
}
