import {
  NavigationBar,
  Progress,
  Screen,
  Spinner,
} from '@dazzlabs/dazzui-mobile'

import { Col, Group, Row, Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Demo() {
  return (
    <Screen bar={<NavigationBar title="Progress" />}>
      <Stage>
        <Group title="Bars">
          <Col>
            <Progress value={0} aria-label="Nothing yet" />
            <Progress value={35} aria-label="A third" />
            <Progress value={100} aria-label="Done" />
            <Progress value={null} aria-label="Working" />
          </Col>
        </Group>
        <Group title="Rings">
          <Row>
            <Progress circular size="small" value={65} aria-label="Small" />
            <Progress circular value={65} aria-label="Medium" />
            <Progress circular size="large" value={65} aria-label="Large" />
            <Progress circular value={null} aria-label="Working" />
          </Row>
        </Group>
        <Group title="Spinner">
          <Row>
            <Spinner />
            <Spinner size="large" />
          </Row>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'progress',
  name: 'Progress',
  summary: 'Bars and rings, determinate and not, and the activity spinner.',
  Demo,
}

export default story
