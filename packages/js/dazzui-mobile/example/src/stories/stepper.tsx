import {
  List,
  ListItem,
  ListSection,
  NavigationBar,
  Screen,
  Stepper,
} from '@dazzlabs/dazzui-mobile'
import { useState } from 'react'

import { Group, Row, Stage } from '../components/stage'
import type { Story } from '../lib/story'

function Demo() {
  const [guests, setGuests] = useState<number | null>(2)
  return (
    <Screen bar={<NavigationBar title="Stepper" />}>
      <List>
        <ListSection header="In a row">
          <ListItem
            title="Guests"
            value={guests ?? 0}
            trailing={
              <Stepper
                aria-label="Guests"
                value={guests}
                onValueChange={setGuests}
                min={1}
                max={8}
              />
            }
          />
        </ListSection>
      </List>
      <Stage>
        <Group title="Showing its value">
          <Row>
            <Stepper
              aria-label="Quantity"
              defaultValue={1}
              min={0}
              max={10}
              showValue
            />
            <Stepper
              aria-label="Disabled"
              defaultValue={3}
              disabled
              showValue
            />
          </Row>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'stepper',
  name: 'Stepper',
  summary:
    'UIStepper; and, where Material has none, a connected pair of buttons.',
  Demo,
}

export default story
