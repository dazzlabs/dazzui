import {
  DateRangePicker,
  type DateRangeValue,
  FormField,
} from '@dazzlabs/dazzui'
import { useState } from 'react'

import { Col } from '../components/stage'
import type { Story } from '../lib/story'

/* The sheet counts a fixed day as today: 16 October 2026, a Friday. */
const TODAY = new Date(2026, 9, 16)

const at = (day: number, hours: number, minutes = 0, month = 9) =>
  new Date(2026, month, day, hours, minutes)

const MEETING: DateRangeValue = {
  start: at(16, 14, 30),
  end: at(16, 15, 30),
  allDay: false,
}

function Meeting() {
  const [value, setValue] = useState<DateRangeValue | null>(MEETING)
  return (
    <DateRangePicker
      value={value}
      onValueChange={setValue}
      today={TODAY}
      aria-label="When"
    />
  )
}

const story: Story = {
  id: 'date-range-picker',
  name: 'DateRangePicker',
  summary: 'When something starts and ends, chosen in one sheet.',
  Preview: Meeting,
  examples: [
    {
      title: 'A meeting',
      note: 'Nothing reaches the value until Done; an outside press or Escape puts the sheet back. Moving the start carries the end with it.',
      Demo: Meeting,
    },
    {
      title: 'Several days, and all day',
      note: 'Two months, for spans that mostly cross one.',
      Demo: () => (
        <Col width="24rem">
          <DateRangePicker
            months={2}
            defaultValue={{ start: at(20, 9), end: at(23, 17), allDay: false }}
            today={TODAY}
            aria-label="Offsite"
          />
          <DateRangePicker
            defaultValue={{ start: at(31, 0), end: at(31, 0), allDay: true }}
            today={TODAY}
            aria-label="Holiday"
          />
        </Col>
      ),
    },
    {
      title: 'Empty, and in a form',
      Demo: () => (
        <Col width="24rem">
          <FormField
            label="When"
            hint="Start, end, and whether the times matter."
          >
            <DateRangePicker today={TODAY} />
          </FormField>
          <DateRangePicker today={TODAY} disabled aria-label="Disabled" />
        </Col>
      ),
    },
    {
      title: 'The product’s own presets and durations',
      note: 'A calendar for sprints: two-week spans counted from today, and a working day as the one duration.',
      Demo: () => (
        <DateRangePicker
          presets={[
            { label: 'Today', days: [0, 0] },
            { label: 'This sprint', days: [0, 13] },
            { label: 'Next sprint', days: [14, 27] },
          ]}
          durations={[{ label: 'Working day', minutes: 8 * 60 }]}
          timeStep={30}
          defaultValue={{ start: at(19, 9), end: at(30, 17), allDay: false }}
          today={TODAY}
          aria-label="Sprint"
        />
      ),
    },
    {
      title: 'Sizes and states',
      Demo: () => (
        <Col width="24rem">
          <DateRangePicker
            size="small"
            defaultValue={MEETING}
            today={TODAY}
            aria-label="Small"
          />
          <DateRangePicker
            size="large"
            defaultValue={MEETING}
            today={TODAY}
            aria-label="Large"
          />
          <DateRangePicker
            state="error"
            defaultValue={MEETING}
            today={TODAY}
            aria-label="Error"
          />
        </Col>
      ),
    },
  ],
}

export default story
