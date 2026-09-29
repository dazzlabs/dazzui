import { Tag } from '@dazzlabs/dazzui-react'
import { useState } from 'react'
import { Row } from '../components/stage'
import type { Story } from '../lib/story'

function Filters() {
  const [picked, setPicked] = useState<readonly string[]>(['Design'])
  return (
    <Row gap="2">
      {['Design', 'Engineering', 'Research'].map(name => (
        <Tag
          key={name}
          selected={picked.includes(name)}
          onSelectedChange={on =>
            setPicked(on ? [...picked, name] : picked.filter(p => p !== name))
          }
        >
          {name}
        </Tag>
      ))}
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
