import { Meter } from '@dazzlabs/dazzui-react'

import { Col } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'meter',
  name: 'Meter',
  summary: 'How much of something there is, against how much there could be.',
  Preview: () => (
    <Col width="15rem">
      <Meter value={62} label="Storage" showValue />
    </Col>
  ),
  examples: [
    {
      title: 'With its label and value',
      Demo: () => (
        <Col width="22rem">
          <Meter value={62} label="Storage" showValue />
        </Col>
      ),
    },
    {
      title: 'Sizes',
      Demo: () => (
        <Col width="22rem" gap="4">
          {(['small', 'medium', 'large'] as const).map(size => (
            <Meter key={size} size={size} value={48} aria-label={size} />
          ))}
        </Col>
      ),
    },
    {
      title: 'Tints',
      Demo: () => (
        <Col width="22rem" gap="4">
          <Meter value={30} tint="success" label="Memory" showValue />
          <Meter value={74} tint="warning" label="Disk" showValue />
          <Meter value={96} tint="danger" label="Quota" showValue />
        </Col>
      ),
    },
  ],
}

export default story
