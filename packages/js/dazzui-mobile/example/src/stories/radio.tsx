import {
  NavigationBar,
  Radio,
  RadioGroup,
  Screen,
} from '@dazzlabs/dazzui-mobile'

import { Group, Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Demo() {
  return (
    <Screen bar={<NavigationBar title="Radio" />}>
      <Stage>
        <Group title="Appearance">
          <RadioGroup defaultValue="system" aria-label="Appearance">
            <Radio value="system">Automatic</Radio>
            <Radio value="light">Light</Radio>
            <Radio value="dark">Dark</Radio>
            <Radio value="oled" disabled>
              True black
            </Radio>
          </RadioGroup>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'radio',
  name: 'Radio',
  summary:
    'One of several: a check at the row’s end on iOS, a radio button on Android.',
  Demo,
}

export default story
