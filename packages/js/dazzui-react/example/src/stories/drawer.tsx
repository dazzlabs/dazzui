import { Button, Drawer, DrawerClose, Switch } from '@dazzlabs/dazzui-react'

import { Col, Row, Text } from '../components/stage'
import type { Story } from '../lib/story'

function Filters({ side }: { side?: 'left' | 'right' | 'top' | 'bottom' }) {
  return (
    <Drawer
      side={side}
      title="Filters"
      description="What the list leaves out."
      trigger={<Button>{side ?? 'Open filters'}</Button>}
      footer={<DrawerClose render={<Button variant="filled">Done</Button>} />}
    >
      <Col gap="4">
        <Row justify="between">
          <Text>Archived projects</Text>
          <Switch aria-label="Archived projects" />
        </Row>
        <Row justify="between">
          <Text>Shared with me</Text>
          <Switch defaultChecked aria-label="Shared with me" />
        </Row>
      </Col>
    </Drawer>
  )
}

const story: Story = {
  id: 'drawer',
  name: 'Drawer',
  summary: 'A panel that comes in from an edge and leaves by it.',
  Preview: () => <Filters />,
  examples: [
    { title: 'From the right', Demo: () => <Filters /> },
    {
      title: 'Sides',
      Demo: () => (
        <Row>
          {(['left', 'right', 'top', 'bottom'] as const).map(side => (
            <Filters key={side} side={side} />
          ))}
        </Row>
      ),
    },
  ],
}

export default story
