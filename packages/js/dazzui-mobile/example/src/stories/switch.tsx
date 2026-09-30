import {
  List,
  ListItem,
  ListSection,
  NavigationBar,
  Screen,
  Switch,
} from '@dazzlabs/dazzui-mobile'

import { Group, Row, Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Demo() {
  return (
    <Screen bar={<NavigationBar title="Switch" />}>
      <List>
        <ListSection header="In a list">
          <ListItem
            title="Wi-Fi"
            trailing={<Switch defaultChecked aria-label="Wi-Fi" />}
          />
          <ListItem
            title="Bluetooth"
            trailing={<Switch aria-label="Bluetooth" />}
          />
          <ListItem
            title="With icon"
            subtitle="Material draws a check in the thumb"
            trailing={<Switch icon defaultChecked aria-label="With icon" />}
          />
        </ListSection>
      </List>
      <Stage>
        <Group title="States">
          <Row>
            <Switch aria-label="Off" />
            <Switch defaultChecked aria-label="On" />
            <Switch disabled aria-label="Off, disabled" />
            <Switch disabled defaultChecked aria-label="On, disabled" />
          </Row>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'switch',
  name: 'Switch',
  summary: 'UISwitch and Material’s switch, in a row and on their own.',
  Demo,
}

export default story
