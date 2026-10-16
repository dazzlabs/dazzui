import { Popover as Base } from '@base-ui/react/popover'
import {
  CalendarLtr16Regular,
  Checkmark12Filled,
  ChevronDown12Regular,
} from '@fluentui/react-icons'
import { useMemo, useState } from 'react'

import { cx } from '../lib/cx'
import {
  addDays,
  compareDay,
  minutesOfDay,
  sameDay,
  setTime,
  startOfDay,
  startOfMonth,
  withTime,
} from '../lib/date'
import type { Tint } from '../lib/tint'
import { Button } from './button'
import { Calendar, type DateRange } from './calendar'
import { Select } from './text-field'
import type { TextFieldSize, TextFieldState } from './text-field'
import { tintFor } from './text-field'
import { Toggle } from './toggle'

/**
 * What the picker chooses: when something starts, when it ends, and whether
 * the times matter. An all-day span keeps its times — switching all-day off
 * again brings them back — and a consumer reads `allDay` before them.
 */
export interface DateRangeValue {
  start: Date
  end: Date
  allDay: boolean
}

/** A named span the reader can pick in one press, counted from today. */
export interface DateRangePreset {
  label: string
  range: (today: Date) => { start: Date; end: Date }
}

/** A length the end is set to, from the start. */
export interface DateRangeDuration {
  label: string
  minutes: number
}

export interface DateRangePickerLabels {
  /** What the trigger says while nothing is chosen. */
  placeholder: string
  presets: string
  start: string
  end: string
  duration: string
  allDay: string
  cancel: string
  done: string
}

export interface DateRangePickerProps {
  value?: DateRangeValue | null
  defaultValue?: DateRangeValue | null
  onValueChange?: (value: DateRangeValue) => void
  /** The presets column. `DATE_RANGE_PRESETS` unless the product has its own. */
  presets?: readonly DateRangePreset[]
  /** The duration chips. `DATE_RANGE_DURATIONS` unless the product has its own. */
  durations?: readonly DateRangeDuration[]
  /** Minutes between the times the lists offer. */
  timeStep?: number
  /** Months side by side in the sheet. */
  months?: 1 | 2
  /** `0` is Sunday. Monday by default. */
  weekStartsOn?: 0 | 1 | 2 | 3 | 4 | 5 | 6
  /** The BCP 47 tag dates are written in. The page's by default. */
  locale?: string
  /** The strings the sheet is labelled with, for a product in another language. */
  labels?: Partial<DateRangePickerLabels>
  /** What the grid counts as today. */
  today?: Date
  size?: TextFieldSize
  state?: TextFieldState
  tint?: Tint
  disabled?: boolean
  id?: string
  className?: string
  'aria-label'?: string
}

const LABELS: DateRangePickerLabels = {
  placeholder: 'Choose dates',
  presets: 'Quick pick',
  start: 'Start',
  end: 'End',
  duration: 'Duration',
  allDay: 'All day',
  cancel: 'Cancel',
  done: 'Done',
}

const HOUR = 60 * 60 * 1000

/** The week's Saturday, or next week's if today is one. */
function weekend(today: Date) {
  const until = (6 - today.getDay() + 7) % 7 || 7
  return addDays(today, until)
}

/** Next Monday, never today. */
function nextMonday(today: Date) {
  const until = (8 - today.getDay()) % 7 || 7
  return addDays(today, until)
}

/** The spans a calendar reaches for most. */
export const DATE_RANGE_PRESETS: readonly DateRangePreset[] = [
  { label: 'Today', range: t => ({ start: t, end: t }) },
  {
    label: 'Tomorrow',
    range: t => ({ start: addDays(t, 1), end: addDays(t, 1) }),
  },
  {
    label: 'This weekend',
    range: t => ({ start: weekend(t), end: addDays(weekend(t), 1) }),
  },
  {
    label: 'Next Monday',
    range: t => ({ start: nextMonday(t), end: nextMonday(t) }),
  },
  {
    label: 'Next week',
    range: t => ({ start: nextMonday(t), end: addDays(nextMonday(t), 6) }),
  },
  {
    label: 'This month',
    range: t => ({
      start: startOfMonth(t),
      end: new Date(t.getFullYear(), t.getMonth() + 1, 0),
    }),
  },
]

export const DATE_RANGE_DURATIONS: readonly DateRangeDuration[] = [
  { label: '30 min', minutes: 30 },
  { label: '1 hour', minutes: 60 },
  { label: '2 hours', minutes: 120 },
]

const pad = (n: number) => String(n).padStart(2, '0')
const parseClock = (time: string): [number, number] => {
  const [h = 0, m = 0] = time.split(':').map(Number)
  return [h, m]
}
const clock = (date: Date) =>
  `${pad(date.getHours())}:${pad(date.getMinutes())}`

/**
 * A span of days and times, chosen in one sheet.
 *
 * The trigger is a field — it wears the select's box — and what it opens
 * is a sheet: the presets a calendar reaches for most down one side, two
 * months to pick the span on, and under them the start and end with their
 * times and a row of durations. Nothing in the sheet reaches the value until Done; an
 * outside press or Escape puts it back. That is the one way this differs
 * from every other popup here, and it is deliberate: a span is several
 * decisions, and a reader who has made two of them should not lose both to
 * a stray click.
 *
 * Base UI owns the popup — its position, its outside press, its focus —
 * and the kit's own `Calendar`, `Select` and `Toggle` own the rest, so the
 * sheet is composition rather than behaviour. The two rules of its own are
 * the ones a calendar app keeps: moving the start carries the end with it,
 * and an end set before its start pulls the start back an hour ahead of it.
 */
export function DateRangePicker({
  value: valueProp,
  defaultValue = null,
  onValueChange,
  presets = DATE_RANGE_PRESETS,
  durations = DATE_RANGE_DURATIONS,
  timeStep = 15,
  months = 2,
  weekStartsOn = 1,
  locale,
  labels: labelsProp,
  today: todayProp,
  size = 'medium',
  state = 'default',
  tint = 'primary',
  disabled,
  id,
  className,
  'aria-label': ariaLabel,
}: DateRangePickerProps) {
  const labels = { ...LABELS, ...labelsProp }
  const today = useMemo(() => startOfDay(todayProp ?? new Date()), [todayProp])

  const [inner, setInner] = useState<DateRangeValue | null>(defaultValue)
  const value = valueProp === undefined ? inner : valueProp

  const [open, setOpen] = useState(false)
  const [draft, setDraft] = useState<DateRangeValue>(() => blank(today))
  // The days as the calendar holds them, apart from the draft: between the
  // first press and the second the calendar's end is unset, and the draft
  // reads that as a one-day span until the reader finishes.
  const [days, setDaysState] = useState<DateRange>({ start: null, end: null })
  const [month, setMonth] = useState(() => startOfMonth(today))

  const formats = useMemo(() => {
    const day = new Intl.DateTimeFormat(locale, {
      month: 'short',
      day: 'numeric',
      weekday: 'short',
    })
    return { day }
  }, [locale])

  const times = useMemo(() => {
    const options = []
    for (let m = 0; m < 24 * 60; m += timeStep) {
      const label = `${pad(Math.floor(m / 60))}:${pad(m % 60)}`
      options.push({ value: label, label })
    }
    return options
  }, [timeStep])

  const commit = (next: DateRangeValue) => {
    if (valueProp === undefined) setInner(next)
    onValueChange?.(next)
  }

  const onOpenChange = (next: boolean) => {
    if (next) {
      const initial = value ?? blank(today)
      setDraft(initial)
      setDaysState({ start: initial.start, end: initial.end })
      setMonth(startOfMonth(initial.start))
    }
    setOpen(next)
  }

  const setDays = (range: DateRange) => {
    setDaysState(range)
    const start = range.start ?? draft.start
    const end = range.end ?? start
    setDraft(current => {
      let nextStart = withTime(start, current.start)
      let nextEnd = withTime(end, current.end)
      if (nextEnd < nextStart) nextEnd = new Date(nextStart.getTime() + HOUR)
      return { ...current, start: nextStart, end: nextEnd }
    })
  }

  const setStartTime = (time: string) => {
    const [h, m] = parseClock(time)
    setDraft(current => {
      const length = current.end.getTime() - current.start.getTime()
      const start = setTime(current.start, h, m)
      return { ...current, start, end: new Date(start.getTime() + length) }
    })
  }

  const setEndTime = (time: string) => {
    const [h, m] = parseClock(time)
    setDraft(current => {
      const end = setTime(current.end, h, m)
      const start =
        end < current.start ? new Date(end.getTime() - HOUR) : current.start
      return { ...current, start, end }
    })
  }

  const setDuration = (minutes: number) => {
    setDraft(current => ({
      ...current,
      allDay: false,
      end: new Date(current.start.getTime() + minutes * 60 * 1000),
    }))
  }

  const applyPreset = (preset: DateRangePreset) => {
    const { start, end } = preset.range(today)
    setMonth(startOfMonth(start))
    setDays({ start, end })
  }

  const presetOn = (preset: DateRangePreset) => {
    const { start, end } = preset.range(today)
    return sameDay(start, draft.start) && sameDay(end, draft.end)
  }

  const length = draft.end.getTime() - draft.start.getTime()
  const durationOn = (duration: DateRangeDuration) =>
    !draft.allDay && length === duration.minutes * 60 * 1000
  // A duration is a length within a day; a span across days has none.
  const oneDay = compareDay(draft.start, draft.end) === 0

  // A time list that starts on the step but still shows a value off it —
  // a span handed in at 14:07 keeps 14:07 until the reader moves it.
  const timesWith = (date: Date) => {
    const current = clock(date)
    return minutesOfDay(date) % timeStep === 0
      ? times
      : [{ value: current, label: current }, ...times]
  }

  const summary = (range: DateRangeValue) => (
    <Summary range={range} format={formats.day} />
  )

  return (
    <Base.Root open={open} onOpenChange={onOpenChange}>
      <Base.Trigger
        id={id}
        disabled={disabled}
        aria-label={ariaLabel}
        aria-invalid={state === 'error' || undefined}
        data-size={size}
        data-tint={tintFor(state, tint)}
        className={cx('dz-select', 'dz-date-range-picker__trigger', className)}
      >
        <span className="dz-date-range-picker__glyph">
          <CalendarLtr16Regular />
        </span>
        <span
          className="dz-select__value"
          data-placeholder={value ? undefined : ''}
        >
          {value ? summary(value) : labels.placeholder}
        </span>
        <span className="dz-select__chevron">
          <ChevronDown12Regular />
        </span>
      </Base.Trigger>
      <Base.Portal>
        <Base.Positioner
          side="bottom"
          align="start"
          className="dz-popup-positioner"
        >
          <Base.Popup
            data-tint={tint}
            data-months={months}
            className="dz-popup dz-date-range-picker"
          >
            <div className="dz-date-range-picker__presets">
              <div className="dz-popup__group-label">{labels.presets}</div>
              {presets.map(preset => (
                <button
                  key={preset.label}
                  type="button"
                  className="dz-popup__item"
                  data-selected={presetOn(preset) || undefined}
                  onClick={() => applyPreset(preset)}
                >
                  <span aria-hidden className="dz-popup__check">
                    <Checkmark12Filled />
                  </span>
                  <span className="dz-popup__label">{preset.label}</span>
                </button>
              ))}
            </div>
            <div className="dz-date-range-picker__months">
              <Calendar
                mode="range"
                size="small"
                months={months}
                month={month}
                onMonthChange={setMonth}
                value={days}
                onValueChange={setDays}
                weekStartsOn={weekStartsOn}
                locale={locale}
                today={today}
                tint={tint}
              />
            </div>
            <div className="dz-date-range-picker__ends">
              <div className="dz-date-range-picker__end">
                <span className="dz-date-range-picker__key">
                  {labels.start}
                </span>
                <span className="dz-date-range-picker__date">
                  {formats.day.format(draft.start)}
                </span>
                <Select
                  size="small"
                  options={timesWith(draft.start)}
                  value={clock(draft.start)}
                  onValueChange={setStartTime}
                  disabled={draft.allDay}
                  aria-label={`${labels.start} time`}
                  className="dz-date-range-picker__time"
                />
              </div>
              <div className="dz-date-range-picker__end">
                <span className="dz-date-range-picker__key">{labels.end}</span>
                <span className="dz-date-range-picker__date">
                  {formats.day.format(draft.end)}
                </span>
                <Select
                  size="small"
                  options={timesWith(draft.end)}
                  value={clock(draft.end)}
                  onValueChange={setEndTime}
                  disabled={draft.allDay}
                  aria-label={`${labels.end} time`}
                  className="dz-date-range-picker__time"
                />
              </div>
              <div className="dz-date-range-picker__durations">
                <span className="dz-date-range-picker__key">
                  {labels.duration}
                </span>
                <div className="dz-date-range-picker__chips">
                  {durations.map(duration => (
                    <Toggle
                      key={duration.minutes}
                      size="tiny"
                      variant="outlined"
                      tint={tint}
                      pressed={durationOn(duration)}
                      disabled={!oneDay}
                      onPressedChange={() => setDuration(duration.minutes)}
                    >
                      {duration.label}
                    </Toggle>
                  ))}
                  <Toggle
                    size="tiny"
                    variant="outlined"
                    tint={tint}
                    pressed={draft.allDay}
                    onPressedChange={allDay =>
                      setDraft(current => ({ ...current, allDay }))
                    }
                  >
                    {labels.allDay}
                  </Toggle>
                </div>
              </div>
            </div>
            <div className="dz-date-range-picker__foot">
              <span className="dz-date-range-picker__summary">
                {summary(draft)}
              </span>
              <span className="dz-date-range-picker__actions">
                <Button variant="normal" onClick={() => setOpen(false)}>
                  {labels.cancel}
                </Button>
                <Button
                  variant="filled"
                  tint={tint}
                  onClick={() => {
                    commit(draft)
                    setOpen(false)
                  }}
                >
                  {labels.done}
                </Button>
              </span>
            </div>
          </Base.Popup>
        </Base.Positioner>
      </Base.Portal>
    </Base.Root>
  )
}

/** The next whole hour, for an hour — what a new event defaults to. */
function blank(today: Date): DateRangeValue {
  const now = new Date()
  const start = sameDay(now, today)
    ? setTime(today, Math.min(23, now.getHours() + 1), 0)
    : setTime(today, 9, 0)
  return { start, end: new Date(start.getTime() + HOUR), allDay: false }
}

/**
 * A span in one line: the day once when both ends share it, the times
 * unless it is all day, and the joins in the faint ink so the dates and
 * times read as the values and the arrow as punctuation.
 */
function Summary({
  range,
  format,
}: {
  range: DateRangeValue
  format: Intl.DateTimeFormat
}) {
  const oneDay = compareDay(range.start, range.end) === 0
  const sep = (glyph: string) => (
    <span className="dz-date-range-picker__sep">{glyph}</span>
  )
  if (range.allDay) {
    return oneDay ? (
      <>
        {format.format(range.start)}
        {sep('·')}
        {'All day'}
      </>
    ) : (
      <>
        {format.format(range.start)}
        {sep('→')}
        {format.format(range.end)}
      </>
    )
  }
  return oneDay ? (
    <>
      {format.format(range.start)}
      {sep('·')}
      {clock(range.start)}
      {sep('→')}
      {clock(range.end)}
    </>
  ) : (
    <>
      {format.format(range.start)} {clock(range.start)}
      {sep('→')}
      {format.format(range.end)} {clock(range.end)}
    </>
  )
}
