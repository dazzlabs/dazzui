import { Calendar, type DateRange } from '@dazzlabs/dazzui'
import { useState } from 'react'

import { Row } from '../components/stage'
import type { Story } from '../lib/story'

/*
 * The grid counts a fixed day as today, so every platform draws the same
 * month and the scenes compare: 16 October 2026, a Friday.
 */
const TODAY = new Date(2026, 9, 16)

function Single() {
  const [day, setDay] = useState<Date | null>(new Date(2026, 9, 20))
  return <Calendar value={day} onValueChange={setDay} today={TODAY} />
}

function Range() {
  const [range, setRange] = useState<DateRange>({
    start: new Date(2026, 9, 20),
    end: new Date(2026, 9, 24),
  })
  return (
    <Calendar
      mode="range"
      value={range}
      onValueChange={setRange}
      today={TODAY}
    />
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
          <Calendar defaultValue={TODAY} weekStartsOn={0} today={TODAY} />
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
