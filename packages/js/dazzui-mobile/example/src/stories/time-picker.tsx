import {
  Card,
  List,
  ListItem,
  ListSection,
  NavigationBar,
  Screen,
  TimePicker,
  type TimePickerProps,
  usePlatform,
} from '@dazzlabs/dazzui-mobile'
import { useState } from 'react'

import { Group, Stage } from '../components/stage'
import type { Story } from '../lib/story'

/** iOS sets an inline picker in an inset group; Android on its own surface. */
function Inline(props: TimePickerProps) {
  const platform = usePlatform()
  return platform === 'ios' ? (
    <Card>
      <TimePicker {...props} />
    </Card>
  ) : (
    <TimePicker surface {...props} />
  )
}

function Demo() {
  const [alarm, setAlarm] = useState('09:30')
  const [starts, setStarts] = useState('10:00')
  const [ends, setEnds] = useState('11:00')
  return (
    <Screen bar={<NavigationBar title="Time Picker" />}>
      <Stage>
        <Group title="Inline">
          <Inline value={alarm} onValueChange={setAlarm} aria-label="Alarm" />
        </Group>
      </Stage>
      <List>
        <ListSection header="In a row">
          <ListItem
            title="Starts"
            trailing={
              <TimePicker
                variant="compact"
                value={starts}
                onValueChange={setStarts}
                aria-label="Starts"
              />
            }
          />
          <ListItem
            title="Ends"
            trailing={
              <TimePicker
                variant="compact"
                value={ends}
                min={starts}
                onValueChange={setEnds}
                aria-label="Ends"
              />
            }
          />
        </ListSection>
      </List>
      <Stage>
        <Group title="24-hour, 15-minute steps (08:00–18:00)">
          <Inline
            defaultValue="14:15"
            min="08:00"
            max="18:00"
            minuteStep={15}
            hourCycle={24}
            aria-label="Meeting time"
          />
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'time-picker',
  name: 'Time Picker',
  summary:
    'UIDatePicker’s time wheels and Material’s time input, inline or as a compact row pill.',
  Demo,
}

export default story
