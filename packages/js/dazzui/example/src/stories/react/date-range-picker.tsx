import {
  DATE_RANGE_PRESETS,
  DateRangePicker,
  type DateRangeValue,
  FormField,
} from '@dazzlabs/dazzui'
import { useState } from 'react'

import { Col, Text } from '../../components/stage'
import type { Story } from '../../lib/story'

/*
 * React only, for now: the sheet composes Calendar, Select and Toggle, and
 * no other platform draws the first of those yet. Moving this story up is
 * the claim that every platform maps the whole sheet.
 */

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
    <Col gap="2" align="start">
      <DateRangePicker
        value={value}
        onValueChange={setValue}
        today={TODAY}
        aria-label="When"
      />
      <Text tone="secondary">
        {value
          ? `${value.start.toLocaleString()} → ${value.end.toLocaleString()}${
              value.allDay ? ' (all day)' : ''
            }`
          : 'Nothing chosen'}
      </Text>
    </Col>
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
      Demo: () => (
        <Col width="24rem">
          <DateRangePicker
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
      title: 'One month, the product’s own presets and durations',
      note: 'A calendar for sprints: two-week spans, and a working day as the one duration.',
      Demo: () => (
        <DateRangePicker
          months={1}
          presets={[
            ...DATE_RANGE_PRESETS.slice(0, 1),
            {
              label: 'This sprint',
              range: t => ({
                start: t,
                end: new Date(t.getTime() + 13 * 864e5),
              }),
            },
            {
              label: 'Next sprint',
              range: t => ({
                start: new Date(t.getTime() + 14 * 864e5),
                end: new Date(t.getTime() + 27 * 864e5),
              }),
            },
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
