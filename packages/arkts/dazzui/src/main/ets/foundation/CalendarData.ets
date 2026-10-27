import { addDays, addMonths, compareDay, sameDay, sameMonth, startOfMonth } from './DateData'

export interface DazzDateRange { start: Date | null; end: Date | null }
export type DazzCalendarMode = 'single' | 'range'
export type DazzCalendarSize = 'small' | 'medium'
export type DazzCalendarValue = Date | DazzDateRange | null
export interface DazzCalendarDayState {
  picked: boolean
  inSpan: boolean
  spanStart: boolean
  spanEnd: boolean
  previewed: boolean
}
export interface DazzCalendarKeyResult { handled: boolean; focus?: Date; pick?: Date }

export function calendarDisabled(day: Date, min?: Date, max?: Date): boolean {
  return (!!min && compareDay(day, min) < 0) || (!!max && compareDay(day, max) > 0)
}
export function calendarVisible(day: Date, first: Date, months: number): boolean {
  for (let index = 0; index < months; index++) if (sameMonth(addMonths(first, index), day)) return true
  return false
}
export function calendarFocusMonth(day: Date, first: Date, months: number): Date {
  return calendarVisible(day, first, months) ? first
    : startOfMonth(compareDay(day, first) > 0 ? addMonths(day, 1 - months) : day)
}
export function calendarTabStop(focused: Date, today: Date, first: Date, months: number): Date {
  return calendarVisible(focused, first, months) ? focused
    : calendarVisible(today, first, months) ? today : startOfMonth(first)
}
export function calendarPick(mode: string, range: DazzDateRange, day: Date, min?: Date, max?: Date): DazzCalendarValue | undefined {
  if (calendarDisabled(day, min, max)) return undefined
  if (mode !== 'range') return day
  if (!range.start || range.end) return { start: day, end: null }
  return compareDay(day, range.start) < 0 ? { start: day, end: range.start } : { start: range.start, end: day }
}
export function calendarDayState(mode: string, range: DazzDateRange, day: Date, focused: Date, hovered: Date | null): DazzCalendarDayState {
  const pending = mode === 'range' && !!range.start && !range.end
  const preview = pending ? hovered ?? focused : null
  let lo = mode === 'range' ? range.start : null
  let hi = mode === 'range' ? pending ? preview : range.end : null
  if (lo && hi && compareDay(lo, hi) > 0) { const swap = lo; lo = hi; hi = swap }
  if (!lo) { lo = hi; hi = null }
  const isStart = sameDay(day, range.start)
  const isEnd = sameDay(day, range.end)
  const inSpan = mode === 'range' && !!lo && !!hi && compareDay(day, lo) >= 0 && compareDay(day, hi) <= 0
  return {
    picked: mode === 'range' ? isStart || isEnd : isStart,
    inSpan, spanStart: inSpan && sameDay(day, lo), spanEnd: inSpan && sameDay(day, hi),
    previewed: pending && !!preview && sameDay(day, preview) && !isStart,
  }
}
export function calendarKey(day: Date, key: string, shift: boolean, weekStartsOn: number): DazzCalendarKeyResult {
  if (key === 'Enter' || key === ' ') return { handled: true, pick: day }
  let next: Date
  if (key === 'ArrowLeft') next = addDays(day, -1)
  else if (key === 'ArrowRight') next = addDays(day, 1)
  else if (key === 'ArrowUp') next = addDays(day, -7)
  else if (key === 'ArrowDown') next = addDays(day, 7)
  else if (key === 'Home') next = addDays(day, -((day.getDay() - weekStartsOn + 7) % 7))
  else if (key === 'End') next = addDays(day, 6 - ((day.getDay() - weekStartsOn + 7) % 7))
  else if (key === 'PageUp' || key === 'PageDown') {
    next = addMonths(day, (key === 'PageUp' ? -1 : 1) * (shift ? 12 : 1))
    next.setDate(Math.min(day.getDate(), new Date(next.getFullYear(), next.getMonth() + 1, 0).getDate()))
  } else return { handled: false }
  return { handled: true, focus: next }
}
