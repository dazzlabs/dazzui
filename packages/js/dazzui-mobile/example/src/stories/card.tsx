import {
  Button,
  Card,
  NavigationBar,
  Screen,
  Text,
} from '@dazzlabs/dazzui-mobile'

import { Col, Group, Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Body({ title }: { title: string }) {
  return (
    <Col>
      <Text role="headline">{title}</Text>
      <Text tone="secondary" role="subheadline">
        A card holds one thing: a note, a place, a result. The corner and the
        surface are the platform's.
      </Text>
      <div>
        <Button variant="tonal" size="small">
          Open
        </Button>
      </div>
    </Col>
  )
}

function Demo() {
  return (
    <Screen bar={<NavigationBar title="Card" />}>
      <Stage>
        <Group title="Variants">
          <Col>
            <Card variant="elevated">
              <Body title="Elevated" />
            </Card>
            <Card variant="filled">
              <Body title="Filled" />
            </Card>
            <Card variant="outlined">
              <Body title="Outlined" />
            </Card>
          </Col>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'card',
  name: 'Card',
  summary:
    'The grouped surface at the big corner; Material’s elevated, filled and outlined cards.',
  Demo,
}

export default story
