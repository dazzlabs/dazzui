import { Popover as Base } from '@base-ui/react/popover'
import {
  type ChangeEvent,
  type KeyboardEvent,
  type WheelEvent,
  useMemo,
  useRef,
  useState,
} from 'react'

import { cx } from '../lib/cx'
import { usePlatform, useThemeContainer } from '../lib/theme'

const DEFAULT_MINUTES = 9 * 60
const DAY_MINUTES = 24 * 60

/** Parse a 24-hour `HH:MM` string into minutes since midnight. */
function parseTime(value: string | undefined): number | null {
  if (!value) return null
  const match = /^(\d{1,2}):(\d{2})$/.exec(value.trim())
  if (!match) return null
  const hours = Number(match[1])
  const minutes = Number(match[2])
  if (hours < 0 || hours > 23 || minutes < 0 || minutes > 59) return null
  return hours * 60 + minutes
}

/** Format minutes since midnight as a zero-padded 24-hour `HH:MM` string. */
function formatTime(totalMinutes: number): string {
  const normalized = ((totalMinutes % DAY_MINUTES) + DAY_MINUTES) % DAY_MINUTES
  const hours = Math.floor(normalized / 60)
  const minutes = normalized % 60
  return `${String(hours).padStart(2, '0')}:${String(minutes).padStart(2, '0')}`
}

function clampMinutes(
  minutes: number,
  low: number | undefined,
  high: number | undefined
): number {
  if (low !== undefined && minutes < low) return low
  if (high !== undefined && minutes > high) return high
  return minutes
}

function resolveHourCycle(
  locale: string | undefined,
  explicit?: 12 | 24
): 12 | 24 {
  if (explicit === 12 || explicit === 24) return explicit
  try {
    const cycle = new Intl.DateTimeFormat(locale, {
      hour: 'numeric',
    }).resolvedOptions().hourCycle
    return cycle === 'h11' || cycle === 'h12' ? 12 : 24
  } catch {
    return 24
  }
}

function resolveDayPeriods(locale: string | undefined): {
  am: string
  pm: string
} {
  try {
    const formatter = new Intl.DateTimeFormat(locale, {
      hour: 'numeric',
      hour12: true,
    })
    const am =
      formatter
        .formatToParts(new Date(2026, 0, 1, 9, 0))
        .find(part => part.type === 'dayPeriod')?.value ?? 'AM'
    const pm =
      formatter
        .formatToParts(new Date(2026, 0, 1, 21, 0))
        .find(part => part.type === 'dayPeriod')?.value ?? 'PM'
    return { am, pm }
  } catch {
    return { am: 'AM', pm: 'PM' }
  }
}

function toDisplayHour(hour24: number, cycle: 12 | 24): number {
  if (cycle === 24) return hour24
  const h = hour24 % 12
  return h === 0 ? 12 : h
}

function fromDisplayHour(
  displayHour: number,
  cycle: 12 | 24,
  isPm: boolean
): number {
  if (cycle === 24) return Math.min(23, Math.max(0, displayHour))
  const clamped = Math.min(12, Math.max(1, displayHour))
  const base = clamped === 12 ? 0 : clamped
  return isPm ? base + 12 : base
}

export interface TimePickerProps {
  /** The chosen time in 24-hour `HH:MM` format (e.g. `'09:30'`). */
  value?: string
  /** Initial time in 24-hour `HH:MM` format when uncontrolled. */
  defaultValue?: string
  /** Called with the chosen time in 24-hour `HH:MM` format. */
  onValueChange?: (value: string) => void
  /** The earliest time that can be chosen, in `HH:MM` format. */
  min?: string
  /** The latest time that can be chosen, in `HH:MM` format. */
  max?: string
  /** Minute increment when stepping (default `1`). */
  minuteStep?: number
  /** Force 12-hour or 24-hour display; inferred from `locale` by default. */
  hourCycle?: 12 | 24
  /** A BCP 47 tag for digit formatting, period names and default hour cycle. */
  locale?: string
  /**
   * `'inline'` draws the picker in place (iOS wheels, Material time input);
   * `'compact'` draws a compact pill that opens the picker in a popover,
   * fitting inside a `ListItem` trailing slot.
   */
  variant?: 'inline' | 'compact'
  /**
   * Android draws an inline picker on the high surface container at the 28dp
   * corner when standing outside a dialog or sheet. Left off, it is
   * transparent. iOS's inline picker never has a surface of its own.
   */
  surface?: boolean
  disabled?: boolean
  hourLabel?: string
  minuteLabel?: string
  periodLabel?: string
  'aria-label'?: string
  className?: string
}

/**
 * UIDatePicker in its time style, and Material's time input.
 * iOS draws three wheel columns — hours, minutes and the day period when the
 * locale uses a 12-hour clock — behind a translucent selection lens; Android
 * draws Material 3's 96 by 72dp hour and minute input fields with a stacked
 * AM/PM period toggle. With `variant="compact"`, either platform draws a
 * compact time pill that opens the picker in a popover, fitting inside a
 * `ListItem` row.
 */
export function TimePicker({
  value,
  defaultValue,
  onValueChange,
  min,
  max,
  minuteStep = 1,
  hourCycle,
  locale,
  variant = 'inline',
  surface = false,
  disabled = false,
  hourLabel = 'Hour',
  minuteLabel = 'Minute',
  periodLabel = 'Period',
  className,
  ...props
}: TimePickerProps) {
  const platform = usePlatform()
  const container = useThemeContainer()
  const controlled = value !== undefined
  const low = parseTime(min) ?? undefined
  const high = parseTime(max) ?? undefined
  const step = Math.min(60, Math.max(1, Math.round(minuteStep)))

  const [inner, setInner] = useState<number>(() =>
    clampMinutes(parseTime(defaultValue) ?? DEFAULT_MINUTES, low, high)
  )
  const rawMinutes = controlled ? (parseTime(value) ?? DEFAULT_MINUTES) : inner
  const current = clampMinutes(rawMinutes, low, high)
  const hours = Math.floor(current / 60)
  const minutes = current % 60
  const isPm = hours >= 12

  const cycle = useMemo(
    () => resolveHourCycle(locale, hourCycle),
    [locale, hourCycle]
  )
  const periods = useMemo(() => resolveDayPeriods(locale), [locale])
  const format = useMemo(
    () => ({
      padded: new Intl.NumberFormat(locale, {
        minimumIntegerDigits: 2,
        useGrouping: false,
      }),
      plain: new Intl.NumberFormat(locale, { useGrouping: false }),
      display: new Intl.DateTimeFormat(locale, {
        hour: cycle === 12 ? 'numeric' : '2-digit',
        minute: '2-digit',
        hour12: cycle === 12,
      }),
    }),
    [locale, cycle]
  )

  const commit = (candidate: number) => {
    if (disabled) return
    const next = clampMinutes(candidate, low, high)
    if (!controlled) setInner(next)
    const formatted = formatTime(next)
    if (formatted !== formatTime(rawMinutes)) onValueChange?.(formatted)
  }

  const stepHourBy = (delta: number) => {
    const wrapped =
      cycle === 12
        ? (isPm ? 12 : 0) + (((((hours % 12) + delta) % 12) + 12) % 12)
        : (((hours + delta) % 24) + 24) % 24
    const bounded =
      low !== undefined || high !== undefined
        ? Math.min(23, Math.max(0, hours + delta))
        : wrapped
    commit(bounded * 60 + minutes)
  }

  const stepMinuteBy = (deltaSteps: number) => {
    let nextMin: number
    if (deltaSteps > 0) {
      const snapped = Math.floor(minutes / step) * step
      nextMin = snapped + deltaSteps * step
    } else {
      const remainder = minutes % step
      const snapped =
        remainder === 0 ? minutes : Math.floor(minutes / step) * step
      nextMin =
        remainder === 0
          ? snapped + deltaSteps * step
          : snapped + (deltaSteps + 1) * step
    }
    if (low !== undefined || high !== undefined) {
      const total = hours * 60 + nextMin
      commit(Math.min(DAY_MINUTES - 1, Math.max(0, total)))
    } else {
      const wrappedMin = ((nextMin % 60) + 60) % 60
      commit(hours * 60 + wrappedMin)
    }
  }

  const setPeriod = (nextPm: boolean) => {
    const h12 = hours % 12
    commit((nextPm ? h12 + 12 : h12) * 60 + minutes)
  }

  const amDisabled = low !== undefined && low >= 12 * 60
  const pmDisabled = high !== undefined && high < 12 * 60

  const formatHour = (hour24: number) => {
    const display = toDisplayHour(hour24, cycle)
    return cycle === 24
      ? format.padded.format(display)
      : format.plain.format(display)
  }
  const formatMinute = (minValue: number) => format.padded.format(minValue)

  const displayHour = toDisplayHour(hours, cycle)
  const displayText = format.display.format(
    new Date(2026, 0, 1, hours, minutes)
  )

  const picker =
    platform === 'ios' ? (
      <IosTimeWheels
        hours={hours}
        minutes={minutes}
        isPm={isPm}
        cycle={cycle}
        step={step}
        low={low}
        high={high}
        disabled={disabled}
        periods={periods}
        amDisabled={amDisabled}
        pmDisabled={pmDisabled}
        hourLabel={hourLabel}
        minuteLabel={minuteLabel}
        periodLabel={periodLabel}
        ariaLabel={props['aria-label']}
        formatHour={formatHour}
        formatMinute={formatMinute}
        onStepHour={stepHourBy}
        onStepMinute={stepMinuteBy}
        onSetPeriod={setPeriod}
        onCommit={commit}
        className={variant === 'inline' ? className : undefined}
      />
    ) : (
      <AndroidTimeInput
        hours={hours}
        minutes={minutes}
        displayHour={displayHour}
        isPm={isPm}
        cycle={cycle}
        low={low}
        high={high}
        surface={variant === 'inline' ? surface : false}
        disabled={disabled}
        periods={periods}
        amDisabled={amDisabled}
        pmDisabled={pmDisabled}
        hourLabel={hourLabel}
        minuteLabel={minuteLabel}
        periodLabel={periodLabel}
        ariaLabel={props['aria-label']}
        formatHour={formatHour}
        formatMinute={formatMinute}
        onStepHour={stepHourBy}
        onStepMinute={stepMinuteBy}
        onSetPeriod={setPeriod}
        onCommit={commit}
        className={variant === 'inline' ? className : undefined}
      />
    )

  if (variant === 'inline') return picker

  return (
    <Base.Root>
      <Base.Trigger
        disabled={disabled}
        aria-label={props['aria-label']}
        data-platform={platform}
        className={cx('dzm-time-picker-trigger', 'dzm-m3-layer', className)}
      >
        {displayText}
      </Base.Trigger>
      <Base.Portal container={container}>
        <Base.Positioner
          side="bottom"
          align="end"
          sideOffset={platform === 'ios' ? 8 : 4}
          data-platform={platform}
          className="dzm-popover-positioner"
        >
          <Base.Popup
            data-platform={platform}
            className={cx(
              'dzm-time-picker-popup',
              platform === 'ios' && 'dzm-glass dzm-glass--thick'
            )}
          >
            {picker}
          </Base.Popup>
        </Base.Positioner>
      </Base.Portal>
    </Base.Root>
  )
}

interface TimeViewProps {
  hours: number
  minutes: number
  isPm: boolean
  cycle: 12 | 24
  low: number | undefined
  high: number | undefined
  disabled: boolean
  periods: { am: string; pm: string }
  amDisabled: boolean
  pmDisabled: boolean
  hourLabel: string
  minuteLabel: string
  periodLabel: string
  ariaLabel: string | undefined
  formatHour: (hour24: number) => string
  formatMinute: (minValue: number) => string
  onStepHour: (delta: number) => void
  onStepMinute: (deltaSteps: number) => void
  onSetPeriod: (isPm: boolean) => void
  onCommit: (minutes: number) => void
  className: string | undefined
}

function IosTimeWheels({
  hours,
  minutes,
  isPm,
  cycle,
  step,
  low,
  high,
  disabled,
  periods,
  amDisabled,
  pmDisabled,
  hourLabel,
  minuteLabel,
  periodLabel,
  ariaLabel,
  formatHour,
  formatMinute,
  onStepHour,
  onStepMinute,
  onSetPeriod,
  onCommit,
  className,
}: TimeViewProps & { step: number }) {
  const hourBuffer = useRef('')
  const hourTimer = useRef<ReturnType<typeof setTimeout> | undefined>(undefined)
  const minBuffer = useRef('')
  const minTimer = useRef<ReturnType<typeof setTimeout> | undefined>(undefined)

  const displayHour = toDisplayHour(hours, cycle)
  const bounded = low !== undefined || high !== undefined

  const prevHour =
    cycle === 12
      ? (isPm ? 12 : 0) + (((hours % 12) - 1 + 12) % 12)
      : (hours - 1 + 24) % 24
  const nextHour =
    cycle === 12
      ? (isPm ? 12 : 0) + (((hours % 12) + 1) % 12)
      : (hours + 1) % 24
  const prevHourCandidate = bounded ? hours - 1 : prevHour
  const nextHourCandidate = bounded ? hours + 1 : nextHour
  const prevHourDisabled =
    disabled ||
    (bounded &&
      (prevHourCandidate < 0 ||
        (low !== undefined && prevHourCandidate * 60 + 59 < low)))
  const nextHourDisabled =
    disabled ||
    (bounded &&
      (nextHourCandidate > 23 ||
        (high !== undefined && nextHourCandidate * 60 > high)))

  const prevMinRaw =
    minutes % step === 0 ? minutes - step : Math.floor(minutes / step) * step
  const nextMinRaw = Math.floor(minutes / step) * step + step
  const prevMinDisplay = ((prevMinRaw % 60) + 60) % 60
  const nextMinDisplay = ((nextMinRaw % 60) + 60) % 60
  const prevMinDisabled =
    disabled ||
    (bounded &&
      (hours * 60 + prevMinRaw < 0 ||
        (low !== undefined && hours * 60 + prevMinRaw < low)))
  const nextMinDisabled =
    disabled ||
    (bounded &&
      (hours * 60 + nextMinRaw >= DAY_MINUTES ||
        (high !== undefined && hours * 60 + nextMinRaw > high)))

  const handleHourKey = (event: KeyboardEvent<HTMLDivElement>) => {
    if (disabled) return
    if (event.key >= '0' && event.key <= '9') {
      event.preventDefault()
      clearTimeout(hourTimer.current)
      const combined = (hourBuffer.current + event.key).slice(-2)
      hourBuffer.current = combined
      const parsed = Number(combined)
      const maxH = cycle === 12 ? 12 : 23
      if (parsed <= maxH && (cycle === 24 || parsed >= 1)) {
        onCommit(fromDisplayHour(parsed, cycle, isPm) * 60 + minutes)
      } else {
        const single = Number(event.key)
        hourBuffer.current = event.key
        if (cycle === 24 || single >= 1) {
          onCommit(fromDisplayHour(single, cycle, isPm) * 60 + minutes)
        }
      }
      hourTimer.current = setTimeout(() => {
        hourBuffer.current = ''
      }, 800)
      return
    }
    switch (event.key) {
      case 'ArrowUp':
        event.preventDefault()
        onStepHour(1)
        break
      case 'ArrowDown':
        event.preventDefault()
        onStepHour(-1)
        break
      case 'PageUp':
        event.preventDefault()
        onStepHour(3)
        break
      case 'PageDown':
        event.preventDefault()
        onStepHour(-3)
        break
      case 'Home':
        event.preventDefault()
        onCommit(
          low !== undefined
            ? low
            : (cycle === 12 && isPm ? 12 : 0) * 60 + minutes
        )
        break
      case 'End':
        event.preventDefault()
        onCommit(
          high !== undefined
            ? high
            : (cycle === 12 ? (isPm ? 23 : 11) : 23) * 60 + minutes
        )
        break
    }
  }

  const handleMinuteKey = (event: KeyboardEvent<HTMLDivElement>) => {
    if (disabled) return
    if (event.key >= '0' && event.key <= '9') {
      event.preventDefault()
      clearTimeout(minTimer.current)
      const combined = (minBuffer.current + event.key).slice(-2)
      minBuffer.current = combined
      const parsed = Number(combined)
      if (parsed <= 59) {
        onCommit(hours * 60 + parsed)
      } else {
        minBuffer.current = event.key
        onCommit(hours * 60 + Number(event.key))
      }
      minTimer.current = setTimeout(() => {
        minBuffer.current = ''
      }, 800)
      return
    }
    switch (event.key) {
      case 'ArrowUp':
        event.preventDefault()
        onStepMinute(1)
        break
      case 'ArrowDown':
        event.preventDefault()
        onStepMinute(-1)
        break
      case 'PageUp':
        event.preventDefault()
        onStepMinute(Math.max(1, Math.round(10 / step)))
        break
      case 'PageDown':
        event.preventDefault()
        onStepMinute(-Math.max(1, Math.round(10 / step)))
        break
      case 'Home':
        event.preventDefault()
        onCommit(hours * 60)
        break
      case 'End':
        event.preventDefault()
        onCommit(hours * 60 + Math.floor(59 / step) * step)
        break
    }
  }

  const handlePeriodKey = (event: KeyboardEvent<HTMLDivElement>) => {
    if (disabled) return
    if (
      event.key === 'ArrowUp' ||
      event.key === 'ArrowDown' ||
      event.key === ' ' ||
      event.key === 'Enter'
    ) {
      event.preventDefault()
      const nextPm = !isPm
      if ((nextPm && !pmDisabled) || (!nextPm && !amDisabled)) {
        onSetPeriod(nextPm)
      }
    }
  }

  const handleWheel =
    (stepFn: (delta: number) => void) => (event: WheelEvent) => {
      if (disabled || event.deltaY === 0) return
      event.preventDefault()
      stepFn(event.deltaY > 0 ? 1 : -1)
    }

  return (
    <div
      role="group"
      aria-label={ariaLabel}
      data-platform="ios"
      data-disabled={disabled || undefined}
      className={cx('dzm-time-picker', className)}
    >
      <div className="dzm-time-picker__wheels">
        <span className="dzm-time-picker__lens" aria-hidden />
        <div
          className="dzm-time-picker__wheel"
          onWheel={handleWheel(onStepHour)}
        >
          <button
            type="button"
            tabIndex={-1}
            disabled={prevHourDisabled}
            aria-hidden
            className="dzm-time-picker__neighbor"
            onClick={() => onStepHour(-1)}
          >
            {prevHourDisabled && bounded
              ? ''
              : formatHour(Math.max(0, prevHourCandidate))}
          </button>
          <div
            role="spinbutton"
            tabIndex={disabled ? -1 : 0}
            aria-label={hourLabel}
            aria-valuenow={displayHour}
            aria-valuemin={cycle === 12 ? 1 : 0}
            aria-valuemax={cycle === 12 ? 12 : 23}
            aria-valuetext={formatHour(hours)}
            aria-disabled={disabled || undefined}
            className="dzm-time-picker__current"
            onKeyDown={handleHourKey}
          >
            {formatHour(hours)}
          </div>
          <button
            type="button"
            tabIndex={-1}
            disabled={nextHourDisabled}
            aria-hidden
            className="dzm-time-picker__neighbor"
            onClick={() => onStepHour(1)}
          >
            {nextHourDisabled && bounded
              ? ''
              : formatHour(Math.min(23, nextHourCandidate))}
          </button>
        </div>

        <span className="dzm-time-picker__colon" aria-hidden>
          :
        </span>

        <div
          className="dzm-time-picker__wheel"
          onWheel={handleWheel(onStepMinute)}
        >
          <button
            type="button"
            tabIndex={-1}
            disabled={prevMinDisabled}
            aria-hidden
            className="dzm-time-picker__neighbor"
            onClick={() => onStepMinute(-1)}
          >
            {prevMinDisabled && bounded ? '' : formatMinute(prevMinDisplay)}
          </button>
          <div
            role="spinbutton"
            tabIndex={disabled ? -1 : 0}
            aria-label={minuteLabel}
            aria-valuenow={minutes}
            aria-valuemin={0}
            aria-valuemax={59}
            aria-valuetext={formatMinute(minutes)}
            aria-disabled={disabled || undefined}
            className="dzm-time-picker__current"
            onKeyDown={handleMinuteKey}
          >
            {formatMinute(minutes)}
          </div>
          <button
            type="button"
            tabIndex={-1}
            disabled={nextMinDisabled}
            aria-hidden
            className="dzm-time-picker__neighbor"
            onClick={() => onStepMinute(1)}
          >
            {nextMinDisabled && bounded ? '' : formatMinute(nextMinDisplay)}
          </button>
        </div>

        {cycle === 12 ? (
          <div
            className="dzm-time-picker__wheel dzm-time-picker__wheel--period"
            onWheel={handleWheel(delta => {
              const nextPm = delta > 0
              if ((nextPm && !pmDisabled) || (!nextPm && !amDisabled)) {
                onSetPeriod(nextPm)
              }
            })}
          >
            {isPm ? (
              <button
                type="button"
                tabIndex={-1}
                disabled={disabled || amDisabled}
                aria-hidden
                className="dzm-time-picker__neighbor"
                onClick={() => onSetPeriod(false)}
              >
                {periods.am}
              </button>
            ) : (
              <span className="dzm-time-picker__spacer" aria-hidden />
            )}
            <div
              role="spinbutton"
              tabIndex={disabled ? -1 : 0}
              aria-label={periodLabel}
              aria-valuenow={isPm ? 1 : 0}
              aria-valuemin={0}
              aria-valuemax={1}
              aria-valuetext={isPm ? periods.pm : periods.am}
              aria-disabled={disabled || undefined}
              className="dzm-time-picker__current"
              onKeyDown={handlePeriodKey}
            >
              {isPm ? periods.pm : periods.am}
            </div>
            {!isPm ? (
              <button
                type="button"
                tabIndex={-1}
                disabled={disabled || pmDisabled}
                aria-hidden
                className="dzm-time-picker__neighbor"
                onClick={() => onSetPeriod(true)}
              >
                {periods.pm}
              </button>
            ) : (
              <span className="dzm-time-picker__spacer" aria-hidden />
            )}
          </div>
        ) : null}
      </div>
    </div>
  )
}

function AndroidTimeInput({
  hours,
  minutes,
  displayHour,
  isPm,
  cycle,
  low,
  high,
  surface,
  disabled,
  periods,
  amDisabled,
  pmDisabled,
  hourLabel,
  minuteLabel,
  periodLabel,
  ariaLabel,
  formatHour,
  formatMinute,
  onStepHour,
  onStepMinute,
  onSetPeriod,
  onCommit,
  className,
}: TimeViewProps & { displayHour: number; surface: boolean }) {
  const [hourDraft, setHourDraft] = useState<string | null>(null)
  const [minuteDraft, setMinuteDraft] = useState<string | null>(null)

  const commitHourDraft = (text: string) => {
    const digits = text.replace(/\D/g, '')
    if (!digits) return
    const parsed = Number(digits)
    const minH = cycle === 12 ? 1 : 0
    const maxH = cycle === 12 ? 12 : 23
    const clamped = Math.min(maxH, Math.max(minH, parsed))
    onCommit(fromDisplayHour(clamped, cycle, isPm) * 60 + minutes)
  }

  const commitMinuteDraft = (text: string) => {
    const digits = text.replace(/\D/g, '')
    if (!digits) return
    const parsed = Math.min(59, Math.max(0, Number(digits)))
    onCommit(hours * 60 + parsed)
  }

  const onHourChange = (event: ChangeEvent<HTMLInputElement>) => {
    const digits = event.target.value.replace(/\D/g, '').slice(-2)
    setHourDraft(digits)
    if (digits.length > 0) {
      const parsed = Number(digits)
      const maxH = cycle === 12 ? 12 : 23
      if (parsed <= maxH && (cycle === 24 || parsed >= 1)) {
        onCommit(fromDisplayHour(parsed, cycle, isPm) * 60 + minutes)
      }
    }
  }

  const onMinuteChange = (event: ChangeEvent<HTMLInputElement>) => {
    const digits = event.target.value.replace(/\D/g, '').slice(-2)
    setMinuteDraft(digits)
    if (digits.length > 0) {
      const parsed = Number(digits)
      if (parsed <= 59) onCommit(hours * 60 + parsed)
    }
  }

  const onHourKeyDown = (event: KeyboardEvent<HTMLInputElement>) => {
    switch (event.key) {
      case 'ArrowUp':
        event.preventDefault()
        setHourDraft(null)
        onStepHour(1)
        break
      case 'ArrowDown':
        event.preventDefault()
        setHourDraft(null)
        onStepHour(-1)
        break
      case 'Home':
        event.preventDefault()
        setHourDraft(null)
        onCommit(
          low !== undefined
            ? low
            : (cycle === 12 && isPm ? 12 : 0) * 60 + minutes
        )
        break
      case 'End':
        event.preventDefault()
        setHourDraft(null)
        onCommit(
          high !== undefined
            ? high
            : (cycle === 12 ? (isPm ? 23 : 11) : 23) * 60 + minutes
        )
        break
    }
  }

  const onMinuteKeyDown = (event: KeyboardEvent<HTMLInputElement>) => {
    switch (event.key) {
      case 'ArrowUp':
        event.preventDefault()
        setMinuteDraft(null)
        onStepMinute(1)
        break
      case 'ArrowDown':
        event.preventDefault()
        setMinuteDraft(null)
        onStepMinute(-1)
        break
      case 'Home':
        event.preventDefault()
        setMinuteDraft(null)
        onCommit(hours * 60)
        break
      case 'End':
        event.preventDefault()
        setMinuteDraft(null)
        onCommit(hours * 60 + 59)
        break
    }
  }

  return (
    <div
      role="group"
      aria-label={ariaLabel}
      data-platform="android"
      data-surface={surface || undefined}
      data-disabled={disabled || undefined}
      className={cx('dzm-time-picker', className)}
    >
      <div className="dzm-time-picker__fields">
        <input
          type="text"
          inputMode="numeric"
          role="spinbutton"
          aria-label={hourLabel}
          aria-valuenow={displayHour}
          aria-valuemin={cycle === 12 ? 1 : 0}
          aria-valuemax={cycle === 12 ? 12 : 23}
          aria-valuetext={formatHour(hours)}
          disabled={disabled}
          value={hourDraft ?? formatHour(hours)}
          className="dzm-time-picker__field"
          onFocus={event => {
            setHourDraft(formatHour(hours))
            event.currentTarget.select()
          }}
          onBlur={() => {
            if (hourDraft !== null) commitHourDraft(hourDraft)
            setHourDraft(null)
          }}
          onChange={onHourChange}
          onKeyDown={onHourKeyDown}
          onWheel={event => {
            if (disabled || event.deltaY === 0) return
            event.preventDefault()
            setHourDraft(null)
            onStepHour(event.deltaY > 0 ? 1 : -1)
          }}
        />
        <span className="dzm-time-picker__colon" aria-hidden>
          :
        </span>
        <input
          type="text"
          inputMode="numeric"
          role="spinbutton"
          aria-label={minuteLabel}
          aria-valuenow={minutes}
          aria-valuemin={0}
          aria-valuemax={59}
          aria-valuetext={formatMinute(minutes)}
          disabled={disabled}
          value={minuteDraft ?? formatMinute(minutes)}
          className="dzm-time-picker__field"
          onFocus={event => {
            setMinuteDraft(formatMinute(minutes))
            event.currentTarget.select()
          }}
          onBlur={() => {
            if (minuteDraft !== null) commitMinuteDraft(minuteDraft)
            setMinuteDraft(null)
          }}
          onChange={onMinuteChange}
          onKeyDown={onMinuteKeyDown}
          onWheel={event => {
            if (disabled || event.deltaY === 0) return
            event.preventDefault()
            setMinuteDraft(null)
            onStepMinute(event.deltaY > 0 ? 1 : -1)
          }}
        />
        {cycle === 12 ? (
          <div
            role="radiogroup"
            aria-label={periodLabel}
            className="dzm-time-picker__period"
          >
            <button
              type="button"
              role="radio"
              aria-checked={!isPm}
              disabled={disabled || amDisabled}
              data-selected={!isPm || undefined}
              className="dzm-time-picker__period-option dzm-m3-layer"
              onClick={() => onSetPeriod(false)}
            >
              {periods.am}
            </button>
            <button
              type="button"
              role="radio"
              aria-checked={isPm}
              disabled={disabled || pmDisabled}
              data-selected={isPm || undefined}
              className="dzm-time-picker__period-option dzm-m3-layer"
              onClick={() => onSetPeriod(true)}
            >
              {periods.pm}
            </button>
          </div>
        ) : null}
      </div>
    </div>
  )
}
