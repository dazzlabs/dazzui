import { Calendar, type DateRange } from '@dazzlabs/dazzui'
import { useState } from 'react'

import { Col, Row, Text } from '../../components/stage'
import type { Story } from '../../lib/story'

/*
 * Outside the shared portable pipeline: not every toolkit draws this grid.
 * HarmonyOS reads this source through its own date-aware story compiler.
 * Moving it up is the claim that every platform maps `Calendar`.
 */

const TODAY = new Date(2026, 9, 16)

const describe = (range: DateRange) =>
  range.start
    ? `${range.start.toLocaleDateString()} → ${
        range.end ? range.end.toLocaleDateString() : '…'
      }`
    : 'Pick a start'

function Single() {
  const [day, setDay] = useState<Date | null>(new Date(2026, 9, 20))
  return (
    <Col gap="2" align="start">
      <Calendar value={day} onValueChange={setDay} today={TODAY} />
      <Text tone="secondary">
        {day ? day.toLocaleDateString() : 'Nothing picked'}
      </Text>
    </Col>
  )
}

function Range() {
  const [range, setRange] = useState<DateRange>({
    start: new Date(2026, 9, 20),
    end: new Date(2026, 9, 24),
  })
  return (
    <Col gap="2" align="start">
      <Calendar
        mode="range"
        value={range}
        onValueChange={setRange}
        today={TODAY}
      />
      <Text tone="secondary">{describe(range)}</Text>
    </Col>
  )
}

const story: Story = {
  id: 'calendar',
  name: 'Calendar',
  summary: 'A month grid: one day, or a span of them.',
  Preview: Single,
  examples: [
    {
      title: 'One day',
      note: 'Arrow keys move by a day and a week, Page Up and Page Down by a month; the grid is one tab stop.',
      Demo: Single,
    },
    {
      title: 'A span',
      note: 'The first press sets the start and the second the end, in either order; the day under the pointer previews the span until then.',
      Demo: Range,
    },
    {
      title: 'Two months, small',
      Demo: () => (
        <Calendar
          mode="range"
          size="small"
          months={2}
          defaultValue={{
            start: new Date(2026, 9, 28),
            end: new Date(2026, 10, 3),
          }}
          today={TODAY}
        />
      ),
    },
    {
      title: 'Bounds and week start',
      note: 'Days outside min and max are drawn but cannot be picked; the week may start on Sunday.',
      Demo: () => (
        <Row align="start">
          <Calendar
            defaultValue={TODAY}
            min={new Date(2026, 9, 12)}
            max={new Date(2026, 9, 23)}
            today={TODAY}
          />
          <Calendar
            defaultValue={TODAY}
            weekStartsOn={0}
            locale="en-US"
            today={TODAY}
          />
        </Row>
      ),
    },
    {
      title: 'Tints',
      Demo: () => (
        <Row align="start">
          {(['primary', 'success', 'danger'] as const).map(tint => (
            <Calendar
              key={tint}
              tint={tint}
              size="small"
              mode="range"
              defaultValue={{
                start: new Date(2026, 9, 14),
                end: new Date(2026, 9, 18),
              }}
              today={TODAY}
            />
          ))}
        </Row>
      ),
    },
  ],
}

export default story
