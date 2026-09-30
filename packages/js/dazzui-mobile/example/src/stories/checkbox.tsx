import { Checkbox, NavigationBar, Screen } from '@dazzlabs/dazzui-mobile'

import { Col, Group, Row, Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Demo() {
  return (
    <Screen bar={<NavigationBar title="Checkbox" />}>
      <Stage>
        <Group title="With a label">
          <Col>
            <Checkbox defaultChecked>Remember this device</Checkbox>
            <Checkbox>Send me the newsletter</Checkbox>
            <Checkbox indeterminate>Some of the above</Checkbox>
            <Checkbox disabled>Not available</Checkbox>
            <Checkbox disabled defaultChecked>
              Required
            </Checkbox>
          </Col>
        </Group>
        <Group title="Alone">
          <Row>
            <Checkbox aria-label="Off" />
            <Checkbox aria-label="On" defaultChecked />
            <Checkbox aria-label="Mixed" indeterminate />
          </Row>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'checkbox',
  name: 'Checkbox',
  summary: 'The circled check iOS selects a row with; Material’s square.',
  Demo,
}

export default story
