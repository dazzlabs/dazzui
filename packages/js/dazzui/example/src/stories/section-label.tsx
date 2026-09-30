import { SectionLabel, TINTS } from '@dazzlabs/dazzui'

import { Col, Text } from '../components/stage'
import type { Story } from '../lib/story'

const story: Story = {
  id: 'section-label',
  name: 'SectionLabel',
  summary: 'The small heading over a group of things.',
  Preview: () => (
    <Col gap="1">
      <SectionLabel>Recent</SectionLabel>
      <Text>Quarterly report</Text>
      <Text>Launch plan</Text>
    </Col>
  ),
  examples: [
    {
      title: 'Default and faint',
      Demo: () => (
        <Col gap="3">
          <SectionLabel>Recent</SectionLabel>
          <SectionLabel faint>Archived</SectionLabel>
        </Col>
      ),
    },
    {
      title: 'Tints',
      Demo: () => (
        <Col gap="3">
          {TINTS.map(tint => (
            <SectionLabel key={tint} tint={tint}>
              {tint}
            </SectionLabel>
          ))}
        </Col>
      ),
    },
  ],
}

export default story
