import {
  Button,
  Card,
  DatePicker,
  type DatePickerProps,
  List,
  ListItem,
  ListSection,
  NavigationBar,
  Screen,
  Sheet,
  SheetClose,
  usePlatform,
} from '@dazzlabs/dazzui-mobile'
import { useState } from 'react'

import { Group, Stage } from '../components/stage'
import type { Story } from '../lib/story'

const medium = new Intl.DateTimeFormat(undefined, { dateStyle: 'medium' })

/** iOS sets an inline picker in an inset group; Android on its own surface. */
function Inline(props: DatePickerProps) {
  const platform = usePlatform()
  return platform === 'ios' ? (
    <Card>
      <DatePicker {...props} />
    </Card>
  ) : (
    <DatePicker surface {...props} />
  )
}

const weekend = (date: Date) => date.getDay() === 0 || date.getDay() === 6

function Demo() {
  const [date, setDate] = useState(() => new Date(2026, 9, 1))
  const [due, setDue] = useState(() => new Date(2026, 9, 1))
  const [draft, setDraft] = useState(due)
  return (
    <Screen bar={<NavigationBar title="Date Picker" />}>
      <Stage>
        <Group title="Inline">
          <Inline value={date} onValueChange={setDate} aria-label="Date" />
        </Group>
      </Stage>
      <List>
        <ListSection>
          <ListItem title="Chosen" value={medium.format(date)} />
        </ListSection>
      </List>
      <Stage>
        <Group title="October to November, weekdays only">
          <Inline
            defaultValue={new Date(2026, 9, 1)}
            min={new Date(2026, 9, 1)}
            max={new Date(2026, 10, 30)}
            isDateDisabled={weekend}
            aria-label="Delivery"
          />
        </Group>
        <Group title="In a sheet">
          <Sheet
            title="Due date"
            onOpenChange={open => {
              if (open) setDraft(due)
            }}
            trigger={<Button variant="tonal">{medium.format(due)}</Button>}
          >
            <Stage>
              <DatePicker
                value={draft}
                onValueChange={setDraft}
                aria-label="Due date"
              />
              <SheetClose
                render={
                  <Button
                    variant="filled"
                    size="large"
                    fullWidth
                    onClick={() => setDue(draft)}
                  />
                }
              >
                Done
              </SheetClose>
            </Stage>
          </Sheet>
        </Group>
      </Stage>
    </Screen>
  )
}

const story: Story = {
  id: 'date-picker',
  name: 'Date Picker',
  summary:
    'UIDatePicker’s inline calendar and Material’s date picker: a month of days, the chosen one on a circle.',
  Demo,
}

export default story
