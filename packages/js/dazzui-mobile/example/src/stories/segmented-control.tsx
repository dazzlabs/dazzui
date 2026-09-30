import {
  NavigationBar,
  Screen,
  SegmentedControl,
  Text,
} from '@dazzlabs/dazzui-mobile'
import { useState } from 'react'

import { Col, Group, Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Demo() {
  const [range, setRange] = useState('week')
  return (
    <Screen bar={<NavigationBar title="Segmented control" />}>
      <Stage>
        <Group title="Two">
          <SegmentedControl
            aria-label="Layout"
            segments={[
              { value: 'list', label: 'List' },
              { value: 'grid', label: 'Grid' },
            ]}
          />
        </Group>
        <Group title="Controlled">
          <Col>
            <SegmentedControl
              aria-label="Range"
              value={range}
              onValueChange={setRange}
              segments={[
                { value: 'day', label: 'Day' },
                { value: 'week', label: 'Week' },
                { value: 'month', label: 'Month' },
                { value: 'year', label: 'Year' },
              ]}
            />
            <Text role="footnote" tone="secondary">
              Showing the {range}.
            </Text>
          </Col>
        </Group>
        <Group title="Disabled segment">
          <SegmentedControl
            aria-label="Plan"
            segments={[
              { value: 'free', label: 'Free' },
              { value: 'pro', label: 'Pro' },
              { value: 'team', label: 'Team', disabled: true },
            ]}
          />
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'segmented-control',
  name: 'Segmented control',
  summary:
    'A glass lens sliding along a capsule; Material’s connected button group.',
  Demo,
}

export default story
