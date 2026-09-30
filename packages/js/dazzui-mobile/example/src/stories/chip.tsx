import { Location20Regular, Star20Regular } from '@fluentui/react-icons'
import { Chip, NavigationBar, Screen } from '@dazzlabs/dazzui-mobile'
import { useState } from 'react'

import { Group, Row, Stage } from '../components/stage'
import type { Story } from '../lib/story'

const FILTERS = ['Open now', 'Top rated', 'Nearby', 'Cheap']

function Demo() {
  const [chosen, setChosen] = useState<string[]>(['Open now'])
  const [tags, setTags] = useState(['Design', 'Swift', 'Kotlin'])
  return (
    <Screen bar={<NavigationBar title="Chip" />}>
      <Stage>
        <Group title="Filters">
          <Row>
            {FILTERS.map(filter => (
              <Chip
                key={filter}
                selected={chosen.includes(filter)}
                onSelectedChange={on =>
                  setChosen(list =>
                    on ? [...list, filter] : list.filter(f => f !== filter)
                  )
                }
              >
                {filter}
              </Chip>
            ))}
          </Row>
        </Group>
        <Group title="Assist">
          <Row>
            <Chip icon={<Location20Regular />}>Directions</Chip>
            <Chip icon={<Star20Regular />} elevated>
              Save
            </Chip>
            <Chip disabled>Unavailable</Chip>
          </Row>
        </Group>
        <Group title="Input">
          <Row>
            {tags.map(tag => (
              <Chip
                key={tag}
                onRemove={() => setTags(list => list.filter(t => t !== tag))}
              >
                {tag}
              </Chip>
            ))}
          </Row>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'chip',
  name: 'Chip',
  summary: 'Glass filter capsules; Material’s assist, filter and input chips.',
  Demo,
}

export default story
