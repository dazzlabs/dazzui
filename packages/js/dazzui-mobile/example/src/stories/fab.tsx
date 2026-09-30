import {
  Add24Regular,
  Compose24Regular,
  Navigation24Regular,
} from '@fluentui/react-icons'
import { Fab, NavigationBar, Screen, Text } from '@dazzlabs/dazzui-mobile'

import { Group, Row, Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Demo() {
  return (
    <Screen bar={<NavigationBar title="FAB" />}>
      <Stage>
        <Group title="Sizes">
          <Row>
            <Fab icon={<Add24Regular />} label="Add" size="small" />
            <Fab icon={<Add24Regular />} label="Add" />
            <Fab icon={<Add24Regular />} label="Add" size="large" />
          </Row>
        </Group>
        <Group title="Extended">
          <Fab icon={<Compose24Regular />}>Compose</Fab>
        </Group>
        <Group title="Tones (Material)">
          <Row>
            <Fab icon={<Navigation24Regular />} label="Primary" />
            <Fab
              icon={<Navigation24Regular />}
              label="Secondary"
              tone="secondary"
            />
            <Fab
              icon={<Navigation24Regular />}
              label="Tertiary"
              tone="tertiary"
            />
            <Fab
              icon={<Navigation24Regular />}
              label="Surface"
              tone="surface"
            />
          </Row>
        </Group>
        <Text role="footnote" tone="secondary">
          iOS has no floating action button; it draws the glass circle its own
          apps float.
        </Text>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'fab',
  name: 'FAB',
  summary: 'Material’s floating action button; a floating glass circle on iOS.',
  Demo,
}

export default story
