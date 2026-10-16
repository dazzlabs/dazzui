import {
  ChevronLeft12Regular,
  ChevronRight12Regular,
} from '@fluentui/react-icons'
import { type KeyboardEvent, useEffect, useMemo, useRef, useState } from 'react'

import { cx } from '../lib/cx'
import {
  addDays,
  addMonths,
  compareDay,
  monthGrid,
  sameDay,
  sameMonth,
  startOfDay,
  startOfMonth,
} from '../lib/date'
import type { Tint } from '../lib/tint'
import { IconButton } from './icon-button'

export type CalendarSize = 'small' | 'medium'

/**
 * A span of days. `end` is `null` while the first day has been picked and the
 * second has not — a range mid-pick is still a value, because the picker
 * around the calendar wants to show the start while the reader chooses the
 * end.
 */
export interface DateRange {
  start: Date | null
  end: Date | null
}

interface CalendarBase {
  /** The regular control height per day, or the small step for a panel that holds two months. */
  size?: CalendarSize
  /** How many months stand side by side. */
  months?: 1 | 2
  /** The month shown — the first of two. Any day in it will do. */
  month?: Date
  defaultMonth?: Date
  onMonthChange?: (month: Date) => void
  /** Days before `min` and after `max` are drawn but cannot be picked. */
  min?: Date
  max?: Date
  /** `0` is Sunday. Monday by default, as ISO counts the week. */
  weekStartsOn?: 0 | 1 | 2 | 3 | 4 | 5 | 6
  /** The BCP 47 tag the month and weekday names are written in. The page's by default. */
  locale?: string
  /** What the grid counts as today. The clock's, unless a test says otherwise. */
  today?: Date
  /**
   * Whether the head carries the chevrons that move the month. Off for a
   * calendar whose host moves it — a picker with the chevrons in its own
   * bar — which then shows the month's name alone.
   */
  navigation?: boolean
  tint?: Tint
  className?: string
  'aria-label'?: string
}

export type CalendarProps = CalendarBase &
  (
    | {
        mode?: 'single'
        value?: Date | null
        defaultValue?: Date | null
        onValueChange?: (value: Date) => void
      }
    | {
        mode: 'range'
        value?: DateRange
        defaultValue?: DateRange
        onValueChange?: (value: DateRange) => void
      }
  )

const EMPTY_RANGE: DateRange = { start: null, end: null }

/** The two days of a range in order, or the one of them that is set. */
function ordered(a: Date | null, b: Date | null): [Date | null, Date | null] {
  if (a && b) return compareDay(a, b) <= 0 ? [a, b] : [b, a]
  return [a ?? b, null]
}

/**
 * A month grid, in one of two modes: one day, or a span of them.
 *
 * Base UI has no calendar, so this is one of the few components that writes
 * its behaviour out, and the reason is written here: a grid's keyboard is not
 * a list's. The arrow keys move by a day and by a week, Page Up and Page Down
 * by a month, Home and End to the ends of the week, and the whole grid is one
 * tab stop that lands on the day last reached — none of which falls out of a
 * toggle group. Picking is a press or Enter; in range mode the first press
 * sets the start and the second the end, in either order, and while the end
 * is pending the day under the pointer or the keyboard previews the span.
 *
 * Every day is a control, so it wears the shared state machine with the
 * recipe its state calls for: `tinted` inside a range — the system's one
 * drawing for a selection — and `filled` for the day or the two days
 * actually picked. At rest it is drawn as a row, in ink with the neutral
 * hover wash, which calendar.css says over the `plain` recipe.
 */
export function Calendar(props: CalendarProps) {
  const {
    size = 'medium',
    months = 1,
    month,
    defaultMonth,
    onMonthChange,
    min,
    max,
    weekStartsOn = 1,
    locale,
    today: todayProp,
    navigation = true,
    tint = 'primary',
    className,
    'aria-label': ariaLabel,
  } = props
  const mode = props.mode ?? 'single'

  const today = useMemo(() => startOfDay(todayProp ?? new Date()), [todayProp])

  // The value, controlled or not, read as a range either way: a single day
  // is a range whose two ends are the same day.
  const [innerSingle, setInnerSingle] = useState<Date | null>(
    props.mode === 'range' ? null : (props.defaultValue ?? null)
  )
  const [innerRange, setInnerRange] = useState<DateRange>(
    props.mode === 'range' ? (props.defaultValue ?? EMPTY_RANGE) : EMPTY_RANGE
  )
  const range: DateRange =
    props.mode === 'range'
      ? (props.value ?? innerRange)
      : (() => {
          const day = props.value === undefined ? innerSingle : props.value
          return { start: day, end: day }
        })()
  const pending = mode === 'range' && !!range.start && !range.end

  const [innerMonth, setInnerMonth] = useState(() =>
    startOfMonth(defaultMonth ?? range.start ?? today)
  )
  const firstMonth = month ? startOfMonth(month) : innerMonth
  const setMonth = (next: Date) => {
    const first = startOfMonth(next)
    if (!month) setInnerMonth(first)
    onMonthChange?.(first)
  }

  // Keyboard: the day the grid's one tab stop lands on, and the day under
  // the pointer, which previews a pending range the way the focused day does.
  const [focused, setFocused] = useState<Date>(() => range.start ?? today)
  const [hovered, setHovered] = useState<Date | null>(null)
  const rootRef = useRef<HTMLDivElement>(null)
  const focusPending = useRef(false)

  useEffect(() => {
    if (!focusPending.current) return
    focusPending.current = false
    const key = startOfDay(focused).getTime()
    rootRef.current
      ?.querySelector<HTMLButtonElement>(`[data-day='${key}']`)
      ?.focus()
  })

  const visible = (day: Date) =>
    Array.from({ length: months }, (_, i) => addMonths(firstMonth, i)).some(m =>
      sameMonth(m, day)
    )

  const disabled = (day: Date) =>
    (!!min && compareDay(day, min) < 0) || (!!max && compareDay(day, max) > 0)

  const pick = (day: Date) => {
    if (disabled(day)) return
    if (props.mode === 'range') {
      const next: DateRange =
        !range.start || range.end
          ? { start: day, end: null }
          : compareDay(day, range.start) < 0
            ? { start: day, end: range.start }
            : { start: range.start, end: day }
      if (props.value === undefined) setInnerRange(next)
      props.onValueChange?.(next)
    } else {
      if (props.value === undefined) setInnerSingle(day)
      props.onValueChange?.(day)
    }
  }

  const moveFocus = (day: Date) => {
    setFocused(day)
    focusPending.current = true
    if (!visible(day)) {
      // Reached past the shown months: slide them so the day is in the
      // first one going back, or the last one going forward.
      const forward = compareDay(day, firstMonth) > 0
      setMonth(forward ? addMonths(day, 1 - months) : day)
    }
  }

  const onKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    const target = event.target as HTMLElement
    const key = target.dataset.day
    if (!key) return
    const day = new Date(Number(key))
    let next: Date | null = null
    switch (event.key) {
      case 'ArrowLeft':
        next = addDays(day, -1)
        break
      case 'ArrowRight':
        next = addDays(day, 1)
        break
      case 'ArrowUp':
        next = addDays(day, -7)
        break
      case 'ArrowDown':
        next = addDays(day, 7)
        break
      case 'PageUp':
        next = addMonths(day, event.shiftKey ? -12 : -1)
        next.setDate(Math.min(day.getDate(), daysIn(next)))
        break
      case 'PageDown':
        next = addMonths(day, event.shiftKey ? 12 : 1)
        next.setDate(Math.min(day.getDate(), daysIn(next)))
        break
      case 'Home':
        next = addDays(day, -((day.getDay() - weekStartsOn + 7) % 7))
        break
      case 'End':
        next = addDays(day, 6 - ((day.getDay() - weekStartsOn + 7) % 7))
        break
      case 'Enter':
      case ' ':
        event.preventDefault()
        pick(day)
        return
      default:
        return
    }
    event.preventDefault()
    moveFocus(next)
  }

  const names = useMemo(() => {
    const weekday = new Intl.DateTimeFormat(locale, { weekday: 'narrow' })
    const title = new Intl.DateTimeFormat(locale, {
      month: 'long',
      year: 'numeric',
    })
    const full = new Intl.DateTimeFormat(locale, { dateStyle: 'full' })
    return { weekday, title, full }
  }, [locale])

  // The week's names, from the day it starts on — read off a real week so
  // the locale decides what to call Sunday.
  const weekdays = useMemo(() => {
    const anchor = addDays(new Date(2024, 0, 7), weekStartsOn) // a Sunday
    return Array.from({ length: 7 }, (_, i) =>
      names.weekday.format(addDays(anchor, i))
    )
  }, [names, weekStartsOn])

  // What the span to draw is: the two picked ends, or the one picked end
  // and the day being previewed against it.
  const preview = pending ? (hovered ?? focused) : null
  const [lo, hi] =
    mode === 'range'
      ? pending
        ? ordered(range.start, preview)
        : ordered(range.start, range.end)
      : [null, null]

  const dayState = (day: Date) => {
    const isStart = sameDay(day, range.start)
    const isEnd = sameDay(day, range.end)
    const picked = mode === 'range' ? isStart || isEnd : isStart
    const inSpan =
      mode === 'range' &&
      !!lo &&
      !!hi &&
      compareDay(day, lo) >= 0 &&
      compareDay(day, hi) <= 0
    return {
      picked,
      inSpan,
      spanStart: inSpan && sameDay(day, lo),
      spanEnd: inSpan && sameDay(day, hi),
      previewed: pending && !!preview && sameDay(day, preview) && !isStart,
    }
  }

  return (
    <div
      ref={rootRef}
      role="group"
      aria-label={ariaLabel}
      data-size={size}
      data-tint={tint}
      data-months={months}
      className={cx('dz-calendar', className)}
      onKeyDown={onKeyDown}
      onPointerLeave={() => setHovered(null)}
    >
      {Array.from({ length: months }, (_, index) => {
        const current = addMonths(firstMonth, index)
        const days = monthGrid(current, weekStartsOn)
        const rows = Array.from({ length: 6 }, (_, r) =>
          days.slice(r * 7, r * 7 + 7)
        )
        return (
          <div key={index} className="dz-calendar__month">
            <div className="dz-calendar__head">
              {!navigation ? null : index === 0 ? (
                <IconButton
                  label="Previous month"
                  onClick={() => setMonth(addMonths(firstMonth, -1))}
                >
                  <ChevronLeft12Regular />
                </IconButton>
              ) : (
                <span className="dz-calendar__spacer" />
              )}
              <span className="dz-calendar__title" aria-live="polite">
                {names.title.format(current)}
              </span>
              {!navigation ? null : index === months - 1 ? (
                <IconButton
                  label="Next month"
                  onClick={() => setMonth(addMonths(firstMonth, 1))}
                >
                  <ChevronRight12Regular />
                </IconButton>
              ) : (
                <span className="dz-calendar__spacer" />
              )}
            </div>
            <div className="dz-calendar__weekdays" aria-hidden>
              {weekdays.map((name, i) => (
                <span key={i}>{name}</span>
              ))}
            </div>
            <div role="grid" className="dz-calendar__grid">
              {rows.map((row, r) => (
                <div key={r} role="row" className="dz-calendar__row">
                  {row.map(day => {
                    const state = dayState(day)
                    const outside = !sameMonth(day, current)
                    const off = disabled(day)
                    // The one tab stop is the focused day, if it is shown;
                    // otherwise today, or failing that the first day shown.
                    const stop = visible(focused)
                      ? sameDay(day, focused)
                      : visible(today)
                        ? sameDay(day, today)
                        : index === 0 && r === 0 && day === row[0]
                    return (
                      <div
                        key={day.getTime()}
                        role="gridcell"
                        aria-selected={state.picked || state.inSpan}
                      >
                        <button
                          type="button"
                          tabIndex={stop && !outside ? 0 : -1}
                          aria-label={names.full.format(day)}
                          aria-disabled={off || undefined}
                          data-day={day.getTime()}
                          data-control=""
                          data-variant={
                            state.picked
                              ? 'filled'
                              : state.inSpan
                                ? 'tinted'
                                : 'plain'
                          }
                          data-outside={outside || undefined}
                          data-today={sameDay(day, today) || undefined}
                          data-selected={state.picked || undefined}
                          data-range={state.inSpan || undefined}
                          data-range-start={state.spanStart || undefined}
                          data-range-end={state.spanEnd || undefined}
                          data-preview={state.previewed || undefined}
                          className="dz-calendar__day"
                          onClick={() => {
                            setFocused(day)
                            pick(day)
                          }}
                          onPointerEnter={() => setHovered(day)}
                          onFocus={() => setFocused(day)}
                        >
                          {day.getDate()}
                        </button>
                      </div>
                    )
                  })}
                </div>
              ))}
            </div>
          </div>
        )
      })}
    </div>
  )
}

function daysIn(month: Date) {
  return new Date(month.getFullYear(), month.getMonth() + 1, 0).getDate()
}
