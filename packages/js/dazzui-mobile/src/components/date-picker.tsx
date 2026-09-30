import { type KeyboardEvent, useEffect, useMemo, useRef, useState } from 'react'

import { cx } from '../lib/cx'
import { ChevronLeft, ChevronRight, DropDown } from '../lib/glyphs'
import { usePlatform } from '../lib/theme'

/*
 * Dates here are days: a `Date` at local midnight, whatever time of day it
 * arrived with. Every value the picker hands back is one of these.
 */

const toDay = (date: Date) =>
  new Date(date.getFullYear(), date.getMonth(), date.getDate())

const addDays = (date: Date, days: number) =>
  new Date(date.getFullYear(), date.getMonth(), date.getDate() + days)

const daysIn = (year: number, month: number) =>
  new Date(year, month + 1, 0).getDate()

/** The same day of another month, or that month's last if it is shorter. */
function addMonths(date: Date, months: number): Date {
  const first = new Date(date.getFullYear(), date.getMonth() + months, 1)
  const last = daysIn(first.getFullYear(), first.getMonth())
  return new Date(
    first.getFullYear(),
    first.getMonth(),
    Math.min(date.getDate(), last)
  )
}

const sameDay = (a: Date | null | undefined, b: Date | null | undefined) =>
  !!a && !!b && a.getTime() === b.getTime()

const monthIndex = (date: Date) => date.getFullYear() * 12 + date.getMonth()

const keyOf = (date: Date) =>
  `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate()}`

/** The locale's first day of the week, 0 for Sunday; Sunday if it will not say. */
function weekStart(locale: string | undefined): number {
  try {
    const tag = new Intl.DateTimeFormat(locale).resolvedOptions().locale
    const info = new Intl.Locale(tag) as unknown as {
      getWeekInfo?: () => { firstDay: number }
      weekInfo?: { firstDay: number }
    }
    const week = info.getWeekInfo?.() ?? info.weekInfo
    return week ? week.firstDay % 7 : 0
  } catch {
    return 0
  }
}

export interface DatePickerProps {
  /** The chosen day; `null` is controlled with none chosen. */
  value?: Date | null
  defaultValue?: Date | null
  /** Called with the day chosen, at local midnight. */
  onValueChange?: (date: Date) => void
  /** The first day that can be chosen. */
  min?: Date
  /** The last day that can be chosen. */
  max?: Date
  /** Days inside the range that still cannot be chosen — weekends, say. */
  isDateDisabled?: (date: Date) => boolean
  /** A BCP 47 tag for the month and weekday names; the browser's by default. */
  locale?: string
  /** 0 for Sunday through 6 for Saturday; the locale's by default. */
  firstDayOfWeek?: 0 | 1 | 2 | 3 | 4 | 5 | 6
  /**
   * Android draws the picker on the high surface container at the 28dp
   * corner, as it stands outside a dialog. Left off, it is transparent and
   * sits on whatever holds it — a sheet, a card. iOS's picker never has a
   * surface of its own.
   */
  surface?: boolean
  'aria-label'?: string
  className?: string
}

/**
 * UIDatePicker in its inline style, and Material's date picker calendar.
 * iOS draws the month and year in the headline weight with a tint chevron
 * that opens the month and year choice, the two month arrows at the
 * trailing edge, the weekdays in small capitals and the chosen day on a
 * circle of the tint wash. Material draws the month and year in label-large
 * with a drop-down arrow, the arrows as icon buttons, and the chosen day on
 * a 40dp primary circle with today ringed. Where iOS spins its wheels
 * this picker lays the twelve months out in a grid, which both platforms'
 * month menus are standing in for.
 *
 * The grid is a `grid`: one day holds the focus, the arrows move it by day
 * and week, Page Up and Page Down by month, Home and End to the ends of the
 * week, and the month on show follows it.
 */
export function DatePicker({
  value,
  defaultValue,
  onValueChange,
  min,
  max,
  isDateDisabled,
  locale,
  firstDayOfWeek,
  surface = false,
  className,
  ...props
}: DatePickerProps) {
  const platform = usePlatform()
  const controlled = value !== undefined
  const [inner, setInner] = useState<Date | null>(() =>
    defaultValue ? toDay(defaultValue) : null
  )
  const selected = controlled ? (value ? toDay(value) : null) : inner
  const low = min ? toDay(min) : undefined
  const high = max ? toDay(max) : undefined
  const today = toDay(new Date())

  const clamp = (date: Date) =>
    low && date < low ? low : high && date > high ? high : date
  const isDisabled = (date: Date) =>
    (!!low && date < low) ||
    (!!high && date > high) ||
    isDateDisabled?.(date) === true

  // The focused day is the one the grid's tab stop sits on, and the month
  // on show is its month.
  const [focused, setFocused] = useState<Date>(() => clamp(selected ?? today))
  const [direction, setDirection] = useState<'next' | 'previous'>()
  const [mode, setMode] = useState<'days' | 'months'>('days')
  const pendingFocus = useRef(false)
  const grid = useRef<HTMLDivElement>(null)

  const go = (target: Date, focus = false) => {
    const next = clamp(target)
    const step = monthIndex(next) - monthIndex(focused)
    if (step !== 0) setDirection(step > 0 ? 'next' : 'previous')
    if (!sameDay(next, focused)) setFocused(next)
    pendingFocus.current = focus
  }

  // A value that changes from outside brings its month into view.
  const selectedKey = selected ? keyOf(selected) : ''
  const [seen, setSeen] = useState(selectedKey)
  if (seen !== selectedKey) {
    setSeen(selectedKey)
    if (selected && !sameDay(selected, focused)) {
      const step = monthIndex(selected) - monthIndex(focused)
      if (step !== 0) setDirection(step > 0 ? 'next' : 'previous')
      setFocused(selected)
    }
  }

  useEffect(() => {
    if (!pendingFocus.current) return
    pendingFocus.current = false
    grid.current
      ?.querySelector<HTMLElement>(`[data-date='${keyOf(focused)}']`)
      ?.focus()
  })

  const format = useMemo(() => {
    const at = (options: Intl.DateTimeFormatOptions) =>
      new Intl.DateTimeFormat(locale, options)
    return {
      title: at({ month: 'long', year: 'numeric' }),
      year: at({ year: 'numeric' }),
      month: at({ month: 'short' }),
      full: at({ dateStyle: 'full' }),
      short: at({ weekday: 'short' }),
      narrow: at({ weekday: 'narrow' }),
      long: at({ weekday: 'long' }),
      number: new Intl.NumberFormat(locale, { useGrouping: false }),
      weekStart: weekStart(locale),
    }
  }, [locale])

  const first = firstDayOfWeek ?? format.weekStart
  const year = focused.getFullYear()
  const month = focused.getMonth()
  const column = (date: Date) => (date.getDay() - first + 7) % 7

  const choose = (date: Date) => {
    if (isDisabled(date)) return
    if (!controlled) setInner(date)
    go(date)
    if (!sameDay(date, selected)) onValueChange?.(date)
  }

  // Weeks of the month on show; the days either side are left blank.
  const weeks: (Date | null)[][] = []
  const lead = column(new Date(year, month, 1))
  const cells: (Date | null)[] = Array.from({ length: lead }, () => null)
  for (let day = 1; day <= daysIn(year, month); day++) {
    cells.push(new Date(year, month, day))
  }
  while (cells.length % 7) cells.push(null)
  for (let i = 0; i < cells.length; i += 7) weeks.push(cells.slice(i, i + 7))

  const weekdays = Array.from({ length: 7 }, (_, i) => {
    // 1 January 2023 was a Sunday.
    const date = new Date(2023, 0, 1 + ((first + i) % 7))
    return {
      label: format.long.format(date),
      text: (platform === 'ios' ? format.short : format.narrow).format(date),
    }
  })

  const onKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    const rtl = getComputedStyle(event.currentTarget).direction === 'rtl'
    const ahead = rtl ? -1 : 1
    let next: Date
    switch (event.key) {
      case 'ArrowLeft':
        next = addDays(focused, -ahead)
        break
      case 'ArrowRight':
        next = addDays(focused, ahead)
        break
      case 'ArrowUp':
        next = addDays(focused, -7)
        break
      case 'ArrowDown':
        next = addDays(focused, 7)
        break
      case 'PageUp':
        next = addMonths(focused, event.shiftKey ? -12 : -1)
        break
      case 'PageDown':
        next = addMonths(focused, event.shiftKey ? 12 : 1)
        break
      case 'Home':
        next = addDays(focused, -column(focused))
        break
      case 'End':
        next = addDays(focused, 6 - column(focused))
        break
      default:
        return
    }
    event.preventDefault()
    go(next, true)
  }

  // The arrows step a month, or a year while the months are showing.
  const span = mode === 'days' ? 1 : 12
  const before = addMonths(new Date(year, month, 1), -span)
  const after = addMonths(new Date(year, month, 1), span)
  const beforeEnd =
    mode === 'days'
      ? new Date(before.getFullYear(), before.getMonth() + 1, 0)
      : new Date(before.getFullYear(), 11, 31)
  const afterStart =
    mode === 'days' ? after : new Date(after.getFullYear(), 0, 1)
  const stepLabel = (date: Date) =>
    (mode === 'days' ? format.title : format.year).format(date)

  const title = format.title.format(focused)

  return (
    <div
      role="group"
      aria-label={props['aria-label']}
      data-platform={platform}
      data-mode={mode}
      data-surface={surface || undefined}
      className={cx('dzm-date-picker', className)}
    >
      <div className="dzm-date-picker__header">
        <button
          type="button"
          aria-expanded={mode === 'months'}
          className="dzm-date-picker__title dzm-m3-layer"
          onClick={() => setMode(mode === 'days' ? 'months' : 'days')}
        >
          <span aria-live="polite">{title}</span>
          <span className="dzm-date-picker__disclosure">
            {platform === 'ios' ? <ChevronRight /> : <DropDown />}
          </span>
        </button>
        <div className="dzm-date-picker__steps">
          <button
            type="button"
            aria-label={stepLabel(before)}
            disabled={!!low && beforeEnd < low}
            className="dzm-date-picker__step dzm-m3-layer"
            onClick={() => go(addMonths(focused, -span))}
          >
            <ChevronLeft />
          </button>
          <button
            type="button"
            aria-label={stepLabel(after)}
            disabled={!!high && afterStart > high}
            className="dzm-date-picker__step dzm-m3-layer"
            onClick={() => go(addMonths(focused, span))}
          >
            <ChevronRight />
          </button>
        </div>
      </div>
      <div className="dzm-date-picker__body">
        {mode === 'days' ? (
          <div
            ref={grid}
            role="grid"
            aria-label={title}
            className="dzm-date-picker__grid"
            onKeyDown={onKeyDown}
          >
            <div role="row" className="dzm-date-picker__row">
              {weekdays.map(weekday => (
                <div
                  key={weekday.label}
                  role="columnheader"
                  aria-label={weekday.label}
                  className="dzm-date-picker__weekday"
                >
                  <span aria-hidden>{weekday.text}</span>
                </div>
              ))}
            </div>
            <div
              key={`${year}-${month}`}
              role="rowgroup"
              data-direction={direction}
              className="dzm-date-picker__weeks"
            >
              {weeks.map((week, row) => (
                <div key={row} role="row" className="dzm-date-picker__row">
                  {week.map((date, cell) =>
                    date ? (
                      <div
                        key={cell}
                        role="gridcell"
                        aria-selected={sameDay(date, selected)}
                        className="dzm-date-picker__cell"
                      >
                        <button
                          type="button"
                          tabIndex={sameDay(date, focused) ? 0 : -1}
                          aria-label={format.full.format(date)}
                          aria-current={
                            sameDay(date, today) ? 'date' : undefined
                          }
                          aria-disabled={isDisabled(date) || undefined}
                          data-date={keyOf(date)}
                          data-today={sameDay(date, today) || undefined}
                          data-selected={sameDay(date, selected) || undefined}
                          data-disabled={isDisabled(date) || undefined}
                          className="dzm-date-picker__day dzm-m3-layer"
                          onClick={() => choose(date)}
                          onFocus={() => {
                            if (!sameDay(date, focused)) setFocused(date)
                          }}
                        >
                          {format.number.format(date.getDate())}
                        </button>
                      </div>
                    ) : (
                      <div
                        key={cell}
                        role="gridcell"
                        className="dzm-date-picker__cell"
                      />
                    )
                  )}
                </div>
              ))}
            </div>
          </div>
        ) : (
          <div
            role="group"
            aria-label={format.year.format(focused)}
            className="dzm-date-picker__months"
          >
            {Array.from({ length: 12 }, (_, index) => {
              const start = new Date(year, index, 1)
              const end = new Date(year, index + 1, 0)
              const out = (!!low && end < low) || (!!high && start > high)
              return (
                <button
                  key={index}
                  type="button"
                  aria-pressed={index === month}
                  aria-label={format.title.format(start)}
                  disabled={out}
                  data-current={
                    (year === today.getFullYear() &&
                      index === today.getMonth()) ||
                    undefined
                  }
                  data-selected={index === month || undefined}
                  className="dzm-date-picker__month dzm-m3-layer"
                  onClick={() => {
                    go(
                      new Date(
                        year,
                        index,
                        Math.min(focused.getDate(), daysIn(year, index))
                      ),
                      true
                    )
                    setMode('days')
                  }}
                >
                  {format.month.format(start)}
                </button>
              )
            })}
          </div>
        )}
      </div>
    </div>
  )
}
