import { Speaker120Regular, Speaker220Regular } from '@fluentui/react-icons'
import { NavigationBar, Screen, Slider } from '@dazzlabs/dazzui-mobile'

import { Col, Group, Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Demo() {
  return (
    <Screen bar={<NavigationBar title="Slider" />}>
      <Stage>
        <Group title="Volume">
          <Slider
            defaultValue={60}
            aria-label="Volume"
            leading={<Speaker120Regular />}
            trailing={<Speaker220Regular />}
          />
        </Group>
        <Group title="Values">
          <Col>
            <Slider defaultValue={0} aria-label="Empty" />
            <Slider defaultValue={35} aria-label="A third" />
            <Slider defaultValue={100} aria-label="Full" />
          </Col>
        </Group>
        <Group title="Stepped and disabled">
          <Col>
            <Slider defaultValue={50} step={10} aria-label="Stepped" />
            <Slider defaultValue={40} disabled aria-label="Disabled" />
          </Col>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'slider',
  name: 'Slider',
  summary:
    'UISlider’s round thumb on a thin groove; Material Expressive’s thick track and handle bar.',
  Demo,
}

export default story
