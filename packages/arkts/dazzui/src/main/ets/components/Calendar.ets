import { ColorMetrics, LengthMetrics, FrameCallback } from '@kit.ArkUI'
import { DazzActivationContext } from '../foundation/Activation'
import { DazzTokens } from '../foundation/Tokens'
import { rememberFocus, forgetFocus } from '../foundation/FocusData'
import { DazzTint, DazzState } from '../foundation/TokenTypes'
import { DazzCalendarValue, DazzCalendarMode, DazzCalendarSize, DazzDateRange, DazzCalendarDayState, calendarDisabled, calendarVisible, calendarFocusMonth, calendarTabStop, calendarPick, calendarDayState, calendarKey } from '../foundation/CalendarData'
import { startOfDay, startOfMonth, addDays, addMonths, sameDay, sameMonth, monthGrid } from '../foundation/DateData'
import { DazzIconButton } from './Button'

let calendarSequence: number = 0
class CalendarFocusFrame extends FrameCallback {
  private action: () => void
  constructor(action: () => void) { super(); this.action = action }
  // New month cells must finish mounting before a native focus request.
  onIdle(_timeLeft: number): void { this.action() }
}

/** A six-week grid; native Buttons own activation, focus and accessibility. */
@Component
export struct DazzCalendar {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Provide({ allowOverride: 'dazzui.activation' }) activation: DazzActivationContext = new DazzActivationContext()
  @Prop mode: DazzCalendarMode = 'single'
  @Prop value: DazzCalendarValue | undefined = undefined
  @Prop defaultValue: DazzCalendarValue = null
  @Prop month: Date | undefined = undefined
  @Prop defaultMonth: Date | undefined = undefined
  @Prop today: Date = new Date()
  @Prop min: Date | undefined = undefined
  @Prop max: Date | undefined = undefined
  @Prop months: 1 | 2 = 1
  @Prop weekStartsOn: 0 | 1 | 2 | 3 | 4 | 5 | 6 = 1
  @Prop locale: string | undefined = undefined
  @Prop navigation: boolean = true
  @Prop focusNavigationOnAppear: boolean = false
  @Prop navigationControlID: string = ''
  @Prop controlSize: DazzCalendarSize = 'medium'
  @Prop tint: DazzTint = 'primary'
  @Prop accessibilityLabel: string = ''
  onValueChange: (value: DazzCalendarValue) => void = () => {}
  onMonthChange: (month: Date) => void = () => {}
  @State innerSingle: Date | null = null
  @State innerRange: DazzDateRange = { start: null, end: null }
  @State innerMonth: number = 0
  @State focused: number = 0
  @State hovered: number | undefined = undefined
  @State pressedID: string = ''
  @State hoverID: string = ''
  private owner: string = 'dazz-calendar-' + String(++calendarSequence)
  private alive: boolean = false
  private focusSequence: number = 0
  private consumedKey: number = -1
  private focusedID: string = ''
  private formattedLocale: string | undefined = undefined
  private titleFormat: Intl.DateTimeFormat = new Intl.DateTimeFormat(undefined, { month: 'long', year: 'numeric' })
  private weekdayFormat: Intl.DateTimeFormat = new Intl.DateTimeFormat(undefined, { weekday: 'narrow' })
  private fullFormat: Intl.DateTimeFormat = new Intl.DateTimeFormat(undefined, { dateStyle: 'full' })
  aboutToAppear(): void {
    this.alive = true
    if (this.mode === 'range') this.innerRange = this.defaultValue as DazzDateRange ?? { start: null, end: null }
    else this.innerSingle = this.defaultValue as Date | null
    this.innerMonth = startOfMonth(this.defaultMonth ?? this.range().start ?? this.today).getTime()
    this.focused = (this.range().start ?? startOfDay(this.today)).getTime()
  }
  aboutToDisappear(): void {
    this.alive = false; this.focusSequence++
    forgetFocus(this.getUIContext().getWindowName() ?? '', this.focusedID)
  }
  range(): DazzDateRange {
    if (this.mode === 'range') return this.value as DazzDateRange ?? this.innerRange
    const day = this.value === undefined ? this.innerSingle : this.value as Date | null
    return { start: day, end: day }
  }
  first(): Date { return this.month ? startOfMonth(this.month) : new Date(this.innerMonth) }
  setMonth(day: Date): void {
    const next = startOfMonth(day)
    if (!this.month) this.innerMonth = next.getTime()
    this.onMonthChange(next)
  }
  pick(day: Date): void {
    const next = calendarPick(this.mode, this.range(), day, this.min, this.max)
    if (next === undefined) return
    if (this.value === undefined) {
      if (this.mode === 'range') this.innerRange = next as DazzDateRange
      else this.innerSingle = next as Date
    }
    this.onValueChange(next)
  }
  moveFocus(day: Date): void {
    this.focused = day.getTime()
    if (!calendarVisible(day, this.first(), this.months)) this.setMonth(calendarFocusMonth(day, this.first(), this.months))
    const sequence = ++this.focusSequence
    this.getUIContext().postFrameCallback(new CalendarFocusFrame(() => {
      if (!this.alive || sequence !== this.focusSequence) return
      // Like React's querySelector, use the first matching cell, even when
      // two visible month grids contain the same outside day.
      for (let index = 0; index < this.months; index++) {
        if (monthGrid(addMonths(this.first(), index), this.weekStartsOn).some((item: Date): boolean => sameDay(item, day))) {
          this.getUIContext().getFocusController().requestFocus(this.dayID(day, index))
          return
        }
      }
    }))
  }
  handleKey(day: Date, event: KeyEvent): boolean {
    if (event.type === KeyType.Up && event.keyCode === this.consumedKey) {
      this.consumedKey = -1; event.stopPropagation(); return true
    }
    if (event.type !== KeyType.Down) return false
    const key = event.keyCode === 2014 ? 'ArrowLeft' : event.keyCode === 2015 ? 'ArrowRight'
      : event.keyCode === 2012 ? 'ArrowUp' : event.keyCode === 2013 ? 'ArrowDown'
      : event.keyCode === 2068 ? 'PageUp' : event.keyCode === 2069 ? 'PageDown'
      : event.keyCode === 2081 ? 'Home' : event.keyCode === 2082 ? 'End'
      : event.keyCode === 2054 || event.keyCode === 2119 ? 'Enter' : event.keyCode === 2050 ? ' ' : ''
    const result = calendarKey(day, key, event.getModifierKeyState?.(['Shift']) ?? false, this.weekStartsOn)
    if (!result.handled) return false
    this.getUIContext().getFocusController().activate(true)
    if (result.focus) this.moveFocus(result.focus)
    if (result.pick) this.pick(result.pick)
    this.consumedKey = event.keyCode; event.stopPropagation(); return true
  }
  dayID(day: Date, index: number): string { return this.owner + '-' + String(index) + '-' + String(day.getTime()) }
  cell(): number { return this.tokens.metric('calendar.' + this.controlSize + '.cell-size') }
  monthWidth(): number { return 7 * this.cell() + 6 * this.tokens.metric('calendar.cell-gap') }
  dates(index: number): Date[] { return monthGrid(addMonths(this.first(), index), this.weekStartsOn) }
  dayState(day: Date): DazzCalendarDayState {
    return calendarDayState(this.mode, this.range(), day, new Date(this.focused), this.hovered === undefined ? null : new Date(this.hovered))
  }
  variant(day: Date): string { const state = this.dayState(day); return state.picked ? 'filled' : state.inSpan ? 'tinted' : 'plain' }
  state(day: Date, index: number): DazzState {
    return calendarDisabled(day, this.min, this.max) ? 'disabled' : this.pressedID === this.dayID(day, index) ? 'pressed' : this.hoverID === this.dayID(day, index) ? 'hovered' : 'normal'
  }
  ink(day: Date, index: number): string {
    const state = this.dayState(day)
    if (state.picked || state.inSpan) return this.tokens.paint(this.variant(day) as 'filled' | 'tinted', 'content', this.state(day, index), this.tint)
    if (sameDay(day, this.today)) return this.tokens.color(this.tint + '.600')
    return this.tokens.color(sameMonth(day, addMonths(this.first(), index)) ? 'content' : 'content-faint')
  }
  surface(day: Date, index: number): ResourceColor {
    const variant = this.variant(day)
    const state = this.state(day, index)
    if (variant !== 'plain') return this.tokens.paint(variant as 'filled' | 'tinted', 'surface', state, this.tint)
    return state === 'hovered' || state === 'pressed' ? this.tokens.paint('normal', 'surface', state) : Color.Transparent
  }
  corner(day: Date, start: boolean): number {
    const state = this.dayState(day)
    return !state.inSpan || (start ? state.spanStart : state.spanEnd) ? this.tokens.metric('radius.small') : 0
  }
  stop(day: Date, index: number): boolean {
    return sameDay(day, calendarTabStop(new Date(this.focused), this.today, this.first(), this.months)) &&
      sameMonth(day, addMonths(this.first(), index))
  }
  format(day: Date, kind: string): string {
    if (this.formattedLocale !== this.locale) {
      this.formattedLocale = this.locale
      this.titleFormat = new Intl.DateTimeFormat(this.locale, { month: 'long', year: 'numeric' })
      this.weekdayFormat = new Intl.DateTimeFormat(this.locale, { weekday: 'narrow' })
      this.fullFormat = new Intl.DateTimeFormat(this.locale, { dateStyle: 'full' })
    }
    return (kind === 'title' ? this.titleFormat : kind === 'weekday' ? this.weekdayFormat : this.fullFormat).format(day)
  }
  @LocalBuilder head(index: number) {
    Row({ space: this.tokens.metric('spacing.1') }) {
      if (this.navigation) {
        if (index === 0) { DazzIconButton({ label: 'Previous month', icon: 'ChevronLeft12Regular', focusControlID: this.navigationControlID || this.owner + '-previous', initialFocus: this.focusNavigationOnAppear, onAction: () => this.setMonth(addMonths(this.first(), -1)) }) }
        else { Row() {}.width(this.tokens.size('tiny')).height(this.tokens.size('tiny')) }
      }
      Text(this.format(addMonths(this.first(), index), 'title')).layoutWeight(1).textAlign(TextAlign.Center)
        .fontFeature('"tnum" on')
        .fontSize(this.tokens.fontSize(this.tokens.type('labelMedium').size)).fontWeight(this.tokens.type('labelMedium').weight)
        .lineHeight(this.tokens.fontSize(this.tokens.type('labelMedium').leading)).fontColor(this.tokens.color('content'))
      if (this.navigation) {
        if (index === this.months - 1) { DazzIconButton({ label: 'Next month', icon: 'ChevronRight12Regular', focusControlID: this.owner + '-next', onAction: () => this.setMonth(addMonths(this.first(), 1)) }) }
        else { Row() {}.width(this.tokens.size('tiny')).height(this.tokens.size('tiny')) }
      }
    }.width(this.monthWidth())
  }
  @LocalBuilder day(day: Date, index: number) {
    Button({ type: ButtonType.Normal }) {
      Stack({ alignContent: Alignment.Center }) {
        Text(String(day.getDate())).fontSize(this.tokens.fontSize(this.tokens.type('labelQuiet').size))
          .fontFeature('"tnum" on')
          .fontWeight(this.tokens.type(this.dayState(day).picked ? 'labelMedium' : sameDay(day, this.today) ? 'labelStrong' : 'labelQuiet').weight)
          .lineHeight(this.tokens.fontSize(this.tokens.type('labelQuiet').size)).fontColor(this.ink(day, index))
        if (sameDay(day, this.today)) {
          Circle().width(this.tokens.metric('spacing.0.5')).height(this.tokens.metric('spacing.0.5')).fill(this.ink(day, index))
            .position({ x: (this.cell() - this.tokens.metric('spacing.0.5')) / 2, y: this.cell() - 2 * this.tokens.metric('spacing.0.5') })
        }
        if (this.dayState(day).previewed) {
          Row() {}.width(this.cell()).height(this.cell()).border({ width: this.tokens.metric('stroke.hairline'), color: this.tokens.color(this.tint + '.600'), style: BorderStyle.Dashed,
            radius: { topLeft: this.corner(day, true), bottomLeft: this.corner(day, true), topRight: this.corner(day, false), bottomRight: this.corner(day, false) } }).hitTestBehavior(HitTestMode.None)
        }
      }.width(this.cell()).height(this.cell()).hitTestBehavior(HitTestMode.None)
    }.width(this.cell()).height(this.cell()).padding(0).border({ width: 0 })
      .borderRadius({ topLeft: this.corner(day, true), bottomLeft: this.corner(day, true), topRight: this.corner(day, false), bottomRight: this.corner(day, false) })
      .backgroundColor(this.surface(day, index)).stateEffect(false)
      .focusBox({ margin: LengthMetrics.vp(this.tokens.metric('focus.offset')), strokeWidth: LengthMetrics.vp(this.dayState(day).previewed ? 0 : this.tokens.metric('focus.width')), strokeColor: ColorMetrics.resourceColor(this.tokens.focus()) })
      .id(this.dayID(day, index)).tabIndex(this.stop(day, index) ? 0 : -1).focusOnTouch(true)
      .accessibilityText(this.format(day, 'full')).accessibilitySelected(this.dayState(day).picked || this.dayState(day).inSpan)
      .accessibilityDescription(calendarDisabled(day, this.min, this.max) ? 'Unavailable' : '')
      .onClick(() => { this.focused = day.getTime(); this.pick(day) })
      .onFocus(() => {
        this.focusedID = this.dayID(day, index)
        rememberFocus(this.getUIContext().getWindowName() ?? '', this.focusedID)
        this.focused = day.getTime()
      })
      .onBlur(() => { forgetFocus(this.getUIContext().getWindowName() ?? '', this.dayID(day, index)) })
      .onHover((hover: boolean) => { this.hoverID = hover ? this.dayID(day, index) : ''; if (hover) this.hovered = day.getTime() })
      .onTouch((event: TouchEvent) => { this.pressedID = event.type === TouchType.Down ? this.dayID(day, index) : '' })
      .onKeyEvent((event: KeyEvent): boolean => this.handleKey(day, event))
  }
  @LocalBuilder grid(index: number) {
    Column({ space: this.tokens.metric('calendar.cell-gap') }) {
      ForEach([0, 1, 2, 3, 4, 5], (row: number) => {
        Row({ space: this.tokens.metric('calendar.cell-gap') }) {
          ForEach(this.dates(index).slice(row * 7, row * 7 + 7), (day: Date) => { this.day(day, index) }, (day: Date): string => String(day.getTime()))
        }
      })
    }
  }
  build() {
    Row({ space: this.tokens.metric('spacing.4') }) {
      ForEach(this.months === 2 ? [0, 1] : [0], (index: number) => {
        Column({ space: this.tokens.metric('spacing.1') }) {
          this.head(index)
          Row({ space: this.tokens.metric('calendar.cell-gap') }) {
            ForEach([0, 1, 2, 3, 4, 5, 6], (weekday: number) => {
              Text(this.format(addDays(new Date(2024, 0, 7), this.weekStartsOn + weekday), 'weekday'))
                .fontSize(this.tokens.fontSize(this.tokens.type('captionSmall').size)).fontWeight(this.tokens.type('labelSmall').weight)
                .fontColor(this.tokens.color('content-subtle')).textAlign(TextAlign.Center)
                .width(this.cell()).height(this.tokens.metric('spacing.5')).accessibilityLevel('no')
            })
          }
          this.grid(index)
        }.width(this.monthWidth())
      })
    }.width(this.months * this.monthWidth() + (this.months - 1) * this.tokens.metric('spacing.4')).height(LayoutPolicy.wrapContent)
      .alignItems(VerticalAlign.Top).accessibilityText(this.accessibilityLabel)
      .onHover((hover: boolean) => { if (!hover) { this.hovered = undefined; this.hoverID = ''; this.pressedID = '' } })
  }
}
