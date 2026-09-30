import {
  NavigationBar,
  Screen,
  Text,
  type TextRole,
} from '@dazzlabs/dazzui-mobile'

import { Col, Group, Stage } from '../components/stage'
import type { Story } from '../lib/story'

const ROLES: readonly TextRole[] = [
  'largeTitle',
  'title',
  'title2',
  'title3',
  'headline',
  'body',
  'callout',
  'subheadline',
  'footnote',
  'caption',
]

function Demo() {
  return (
    <Screen bar={<NavigationBar title="Text" />}>
      <Stage>
        <Group title="Roles">
          <Col>
            {ROLES.map(role => (
              <Text key={role} role={role}>
                {role}
              </Text>
            ))}
          </Col>
        </Group>
        <Group title="Tones">
          <Col>
            <Text>Primary label</Text>
            <Text tone="secondary">Secondary label</Text>
            <Text tone="tertiary">Tertiary label</Text>
            <Text tone="tint">Tinted</Text>
          </Col>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'text',
  name: 'Text',
  summary: 'Dynamic Type’s roles, and the Material type scale each maps to.',
  Demo,
}

export default story
