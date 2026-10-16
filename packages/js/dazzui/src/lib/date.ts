/**
 * The handful of calendar arithmetic a month grid and a range picker need,
 * on local-time `Date`s. A day is a `Date` at local midnight; a time of day
 * rides on the same object. Nothing here reaches for a library: the system
 * draws a calendar, it does not do calendars, and a consumer with a real
 * date library hands the kit plain `Date`s either way.
 */

export const DAY = 24 * 60 * 60 * 1000

export function startOfDay(date: Date): Date {
  return new Date(date.getFullYear(), date.getMonth(), date.getDate())
}

export function startOfMonth(date: Date): Date {
  return new Date(date.getFullYear(), date.getMonth(), 1)
}

export function addDays(date: Date, days: number): Date {
  const next = new Date(date)
  next.setDate(next.getDate() + days)
  return next
}

/** The first of the month `months` away, so the day never overflows. */
export function addMonths(date: Date, months: number): Date {
  return new Date(date.getFullYear(), date.getMonth() + months, 1)
}

export function sameDay(
  a: Date | null | undefined,
  b: Date | null | undefined
) {
  return (
    !!a &&
    !!b &&
    a.getFullYear() === b.getFullYear() &&
    a.getMonth() === b.getMonth() &&
    a.getDate() === b.getDate()
  )
}

export function sameMonth(a: Date, b: Date) {
  return a.getFullYear() === b.getFullYear() && a.getMonth() === b.getMonth()
}

/** Day order, ignoring the time of day: negative when `a` is earlier. */
export function compareDay(a: Date, b: Date) {
  return startOfDay(a).getTime() - startOfDay(b).getTime()
}

/** `day`'s date carrying `time`'s hours and minutes. */
export function withTime(day: Date, time: Date): Date {
  return new Date(
    day.getFullYear(),
    day.getMonth(),
    day.getDate(),
    time.getHours(),
    time.getMinutes()
  )
}

export function setTime(date: Date, hours: number, minutes: number): Date {
  return new Date(
    date.getFullYear(),
    date.getMonth(),
    date.getDate(),
    hours,
    minutes
  )
}

export function minutesOfDay(date: Date) {
  return date.getHours() * 60 + date.getMinutes()
}

/**
 * The six weeks a month grid draws, starting on the week day `weekStartsOn`
 * — always forty-two days, so the grid never changes height between months.
 */
export function monthGrid(month: Date, weekStartsOn: number): Date[] {
  const first = startOfMonth(month)
  const offset = (first.getDay() - weekStartsOn + 7) % 7
  const start = addDays(first, -offset)
  return Array.from({ length: 42 }, (_, i) => addDays(start, i))
}
