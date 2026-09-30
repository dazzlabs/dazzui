import { Tag } from '@dazzlabs/dazzui'
import { useState } from 'react'
import { Row } from '../components/stage'
import type { Story } from '../lib/story'

/** A tag that remembers whether it was picked. */
function Filter({ name, picked = false }: { name: string; picked?: boolean }) {
  const [selected, setSelected] = useState(picked)
  return (
    <Tag selected={selected} onSelectedChange={setSelected}>
      {name}
    </Tag>
  )
}

function Filters() {
  return (
    <Row gap="2">
      <Filter name="Design" picked />
      <Filter name="Engineering" />
      <Filter name="Research" />
    </Row>
  )
}

const story: Story = {
  id: 'tag',
  name: 'Tag',
  summary: 'A label a person can pick, or take away.',
  Preview: () => <Filters />,
  examples: [
    { title: 'Selectable', Demo: Filters },
    {
      title: 'Removable',
      Demo: () => (
        <Row gap="2">
          <Tag onRemove={() => {}}>alice@example.com</Tag>
          <Tag onRemove={() => {}} tint="info">
            bob@example.com
          </Tag>
        </Row>
      ),
    },
  ],
}

export default story
