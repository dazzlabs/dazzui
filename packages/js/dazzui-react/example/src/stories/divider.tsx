import { Divider } from '@dazzlabs/dazzui-react'

import { Col, Row, Text } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'divider',
  name: 'Divider',
  summary: 'A rule between two things that are not the same thing.',
  Preview: () => (
    <Col width="14rem">
      <Text>Account</Text>
      <Divider />
      <Text>Billing</Text>
    </Col>
  ),
  examples: [
    {
      title: 'Tones',
      Demo: () => (
        <Col width="24rem" gap="5">
          <Divider />
          <Divider tone="muted" />
        </Col>
      ),
    },
    {
      title: 'Vertical',
      Demo: () => (
        <Row align="stretch">
          <Text>Edit</Text>
          <Divider orientation="vertical" />
          <Text>View</Text>
          <Divider orientation="vertical" />
          <Text>Help</Text>
        </Row>
      ),
    },
  ],
}

export default story
