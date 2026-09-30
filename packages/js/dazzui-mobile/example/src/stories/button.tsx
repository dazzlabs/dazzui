import { Add20Regular } from '@fluentui/react-icons'
import { Button, NavigationBar, Screen } from '@dazzlabs/dazzui-mobile'

import { Col, Group, Row, Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Demo() {
  return (
    <Screen bar={<NavigationBar title="Button" />}>
      <Stage>
        <Group title="Styles">
          <Row>
            <Button variant="filled">Filled</Button>
            <Button variant="tonal">Tonal</Button>
            <Button variant="outlined">Outlined</Button>
            <Button variant="plain">Plain</Button>
            <Button variant="glass">Glass</Button>
          </Row>
        </Group>
        <Group title="Sizes">
          <Row>
            <Button size="small" variant="filled">
              Small
            </Button>
            <Button size="medium" variant="filled">
              Medium
            </Button>
          </Row>
          <Button size="large" variant="filled" fullWidth>
            Continue
          </Button>
        </Group>
        <Group title="With a glyph">
          <Row>
            <Button variant="filled" icon={<Add20Regular />}>
              New note
            </Button>
            <Button variant="tonal" icon={<Add20Regular />}>
              Add
            </Button>
          </Row>
        </Group>
        <Group title="Destructive">
          <Row>
            <Button variant="filled" tone="destructive">
              Delete
            </Button>
            <Button variant="tonal" tone="destructive">
              Remove
            </Button>
            <Button variant="plain" tone="destructive">
              Discard
            </Button>
          </Row>
        </Group>
        <Group title="Square shape (Material Expressive)">
          <Row>
            <Button variant="filled" shape="square">
              Filled
            </Button>
            <Button variant="tonal" shape="square">
              Tonal
            </Button>
          </Row>
        </Group>
        <Group title="Disabled">
          <Col>
            <Row>
              <Button variant="filled" disabled>
                Filled
              </Button>
              <Button variant="tonal" disabled>
                Tonal
              </Button>
              <Button variant="outlined" disabled>
                Outlined
              </Button>
              <Button variant="plain" disabled>
                Plain
              </Button>
            </Row>
          </Col>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'button',
  name: 'Button',
  summary:
    'Glass and prominent capsules on iOS; filled, tonal, outlined, text and elevated on Android.',
  Demo,
}

export default story
