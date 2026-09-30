import { Mail20Regular, Person20Regular } from '@fluentui/react-icons'
import { NavigationBar, Screen, TextField } from '@dazzlabs/dazzui-mobile'

import { Col, Group, Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Demo() {
  return (
    <Screen bar={<NavigationBar title="Text field" />}>
      <Stage>
        <Group title="Outlined">
          <Col>
            <TextField
              label="Name"
              placeholder="Your name"
              leading={<Person20Regular />}
            />
            <TextField
              label="Email"
              defaultValue="ada@example.com"
              leading={<Mail20Regular />}
            />
            <TextField
              label="Handle"
              defaultValue="ada"
              supporting="Letters and numbers only"
            />
            <TextField
              label="Password"
              type="password"
              defaultValue="hunter2"
              error
              supporting="At least eight characters"
            />
            <TextField label="Disabled" defaultValue="Read only" disabled />
          </Col>
        </Group>
        <Group title="Filled (Material)">
          <Col>
            <TextField
              variant="filled"
              label="City"
              placeholder="Where you live"
            />
            <TextField
              variant="filled"
              label="Country"
              defaultValue="Portugal"
            />
          </Col>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'text-field',
  name: 'Text field',
  summary:
    'A rounded field with a clear button; Material’s outlined and filled boxes with a floating label.',
  Demo,
}

export default story
