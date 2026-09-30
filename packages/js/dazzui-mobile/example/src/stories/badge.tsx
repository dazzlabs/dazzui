import { Badge, NavigationBar, Screen, Text } from '@dazzlabs/dazzui-mobile'

import { Group, Row, Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Demo() {
  return (
    <Screen bar={<NavigationBar title="Badge" />}>
      <Stage>
        <Group title="Counts">
          <Row>
            <Badge count={1} />
            <Badge count={12} />
            <Badge count={120} />
            <Badge count={1200} max={999} />
            <Badge dot />
          </Row>
        </Group>
        <Group title="Tones">
          <Row>
            <Badge count={3} />
            <Badge count={3} tone="tint" />
            <Badge>New</Badge>
          </Row>
        </Group>
        <Group title="Beside a label">
          <Row between>
            <Text>Notifications</Text>
            <Badge count={9} />
          </Row>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'badge',
  name: 'Badge',
  summary: 'The red count a tab carries, on both platforms.',
  Demo,
}

export default story
