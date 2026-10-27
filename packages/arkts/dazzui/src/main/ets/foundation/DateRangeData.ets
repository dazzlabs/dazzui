import { DazzDateRange } from './CalendarData'
import { addDays, compareDay, minutesOfDay, sameDay, setTime, startOfMonth, withTime } from './DateData'

export interface DazzDateRangeValue { start: Date; end: Date; allDay: boolean }
export interface DazzDateRangeSpan { start: Date; end: Date }
export interface DazzDateRangePreset { label: string; range: (today: Date) => DazzDateRangeSpan }
export interface DazzDateRangeDuration { label: string; minutes: number }
export interface DazzDateRangePickerLabels {
  placeholder?: string; presets?: string; start?: string; end?: string
  duration?: string; allDay?: string; cancel?: string; done?: string
}
export interface DazzDateRangeSummaryPart { text: string; separator?: boolean }
export interface DazzClockOption { value: string; label: string }

export const DATE_RANGE_LABELS: Record<string, string> = {
  'placeholder': 'Choose dates', 'presets': 'Quick pick', 'start': 'Start', 'end': 'End',
  'duration': 'Duration', 'allDay': 'All day', 'cancel': 'Cancel', 'done': 'Done',
}
export function dateRangeLabels(value: DazzDateRangePickerLabels): Record<string, string> {
  return { 'placeholder': value.placeholder ?? DATE_RANGE_LABELS['placeholder'], 'presets': value.presets ?? DATE_RANGE_LABELS['presets'],
    'start': value.start ?? DATE_RANGE_LABELS['start'], 'end': value.end ?? DATE_RANGE_LABELS['end'], 'duration': value.duration ?? DATE_RANGE_LABELS['duration'],
    'allDay': value.allDay ?? DATE_RANGE_LABELS['allDay'], 'cancel': value.cancel ?? DATE_RANGE_LABELS['cancel'], 'done': value.done ?? DATE_RANGE_LABELS['done'] }
}
export const DATE_RANGE_DURATIONS: DazzDateRangeDuration[] = [
  { label: '30 min', minutes: 30 }, { label: '1 hour', minutes: 60 }, { label: '2 hours', minutes: 120 },
]
export const DATE_RANGE_PRESETS: DazzDateRangePreset[] = [
  { label: 'Today', range: (today: Date): DazzDateRangeSpan => ({ start: today, end: today }) },
  { label: 'Tomorrow', range: (today: Date): DazzDateRangeSpan => ({ start: addDays(today, 1), end: addDays(today, 1) }) },
  { label: 'This weekend', range: (today: Date): DazzDateRangeSpan => {
    const start = addDays(today, (6 - today.getDay() + 7) % 7 || 7)
    return { start, end: addDays(start, 1) }
  } },
  { label: 'Next Monday', range: (today: Date): DazzDateRangeSpan => {
    const start = addDays(today, (8 - today.getDay()) % 7 || 7)
    return { start, end: start }
  } },
  { label: 'Next week', range: (today: Date): DazzDateRangeSpan => {
    const start = addDays(today, (8 - today.getDay()) % 7 || 7)
    return { start, end: addDays(start, 6) }
  } },
  { label: 'This month', range: (today: Date): DazzDateRangeSpan => ({ start: startOfMonth(today), end: new Date(today.getFullYear(), today.getMonth() + 1, 0) }) },
]

export function dateRangeBlank(today: Date, now: Date = new Date()): DazzDateRangeValue {
  const start = sameDay(now, today) ? setTime(today, Math.min(23, now.getHours() + 1), 0) : setTime(today, 9, 0)
  return { start, end: new Date(start.getTime() + 60 * 60 * 1000), allDay: false }
}
export function dateRangeCopy(value: DazzDateRangeValue): DazzDateRangeValue {
  return { start: new Date(value.start.getTime()), end: new Date(value.end.getTime()), allDay: value.allDay }
}
export function dateRangeDays(current: DazzDateRangeValue, range: DazzDateRange): DazzDateRangeValue {
  const start = range.start ?? current.start
  const nextStart = withTime(start, current.start)
  let nextEnd = withTime(range.end ?? start, current.end)
  if (nextEnd < nextStart) nextEnd = new Date(nextStart.getTime() + 60 * 60 * 1000)
  return { start: nextStart, end: nextEnd, allDay: current.allDay }
}
function clockParts(time: string): number[] { const parts = time.split(':').map((part: string): number => Number(part)); return [parts[0] ?? 0, parts[1] ?? 0] }
export function dateRangeStartTime(current: DazzDateRangeValue, time: string): DazzDateRangeValue {
  const parts = clockParts(time)
  const length = current.end.getTime() - current.start.getTime()
  const start = setTime(current.start, parts[0], parts[1])
  return { start, end: new Date(start.getTime() + length), allDay: current.allDay }
}
export function dateRangeEndTime(current: DazzDateRangeValue, time: string): DazzDateRangeValue {
  const parts = clockParts(time)
  const end = setTime(current.end, parts[0], parts[1])
  const start = end < current.start ? new Date(end.getTime() - 60 * 60 * 1000) : current.start
  return { start, end, allDay: current.allDay }
}
export function dateRangeDuration(current: DazzDateRangeValue, minutes: number): DazzDateRangeValue {
  return { start: current.start, end: new Date(current.start.getTime() + minutes * 60 * 1000), allDay: false }
}
export function dateRangeDurationOn(current: DazzDateRangeValue, minutes: number): boolean {
  return !current.allDay && current.end.getTime() - current.start.getTime() === minutes * 60 * 1000
}
export function dateRangeOneDay(current: DazzDateRangeValue): boolean { return compareDay(current.start, current.end) === 0 }
function pad(value: number): string { return String(value).padStart(2, '0') }
export function dateRangeClock(date: Date): string { return pad(date.getHours()) + ':' + pad(date.getMinutes()) }
export function dateRangeTimes(step: number, current: Date): DazzClockOption[] {
  if (!Number.isFinite(step) || step <= 0) throw new Error('Time step must be positive and finite')
  const options: DazzClockOption[] = []
  if (minutesOfDay(current) % step !== 0) { const value = dateRangeClock(current); options.push({ value, label: value }) }
  for (let minutes = 0; minutes < 24 * 60; minutes += step) {
    const value = pad(Math.floor(minutes / 60)) + ':' + pad(minutes % 60)
    options.push({ value, label: value })
  }
  return options
}
export function dateRangeSummary(value: DazzDateRangeValue, format: (date: Date) => string): DazzDateRangeSummaryPart[] {
  const start: DazzDateRangeSummaryPart = { text: format(value.start) }
  const end: DazzDateRangeSummaryPart = { text: format(value.end) }
  const arrow: DazzDateRangeSummaryPart = { text: '→', separator: true }
  const dot: DazzDateRangeSummaryPart = { text: '·', separator: true }
  if (value.allDay) return dateRangeOneDay(value) ? [start, dot, { text: 'All day' }] : [start, arrow, end]
  if (dateRangeOneDay(value)) return [start, dot, { text: dateRangeClock(value.start) }, arrow, { text: dateRangeClock(value.end) }]
  return [{ text: start.text + ' ' + dateRangeClock(value.start) }, arrow, { text: end.text + ' ' + dateRangeClock(value.end) }]
}

/** Editing lifetime is independent of whether the native popup caches its tree. */
export class DazzDateRangeSession {
  editing: boolean = false
  draft: DazzDateRangeValue = dateRangeBlank(new Date())
  days: DazzDateRange = { start: null, end: null }
  month: Date = startOfMonth(new Date())
  private observers: Set<() => void> = new Set()
  observe(callback: () => void): void { this.observers.add(callback) }
  unobserve(callback: () => void): void { this.observers.delete(callback) }
  private changed(): void { this.observers.forEach((callback: () => void) => callback()) }
  invalidate(): void { this.changed() }
  start(value: DazzDateRangeValue | null, today: Date, now: Date = new Date()): void {
    this.draft = dateRangeCopy(value ?? dateRangeBlank(today, now))
    this.days = { start: this.draft.start, end: this.draft.end }
    this.month = startOfMonth(this.draft.start)
    this.editing = true
    this.changed()
  }
  close(): void { this.editing = false; this.changed() }
  dispose(): void { this.editing = false; this.observers.clear() }
  confirm(commit: (value: DazzDateRangeValue) => void): void {
    if (!this.editing) return
    commit(dateRangeCopy(this.draft))
    this.close()
  }
  setDays(range: DazzDateRange): void { this.days = range; this.draft = dateRangeDays(this.draft, range); this.changed() }
  setMonth(month: Date): void { this.month = startOfMonth(month); this.changed() }
  setStartTime(time: string): void { this.draft = dateRangeStartTime(this.draft, time); this.changed() }
  setEndTime(time: string): void { this.draft = dateRangeEndTime(this.draft, time); this.changed() }
  setDuration(minutes: number): void { this.draft = dateRangeDuration(this.draft, minutes); this.changed() }
  setAllDay(allDay: boolean): void { this.draft = { start: this.draft.start, end: this.draft.end, allDay }; this.changed() }
  preset(preset: DazzDateRangePreset, today: Date): void {
    const range = preset.range(today)
    this.month = startOfMonth(range.start)
    this.setDays({ start: range.start, end: range.end })
  }
}
