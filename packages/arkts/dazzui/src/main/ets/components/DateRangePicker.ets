import { LengthMetrics, FrameCallback, FrameNode } from '@kit.ArkUI'
import { DazzTokens } from '../foundation/Tokens'
import { DazzTint, DazzSize, DazzTypography } from '../foundation/TokenTypes'
import { DazzActivationContext } from '../foundation/Activation'
import { rememberFocus, forgetFocus, focusTarget, observeFocus } from '../foundation/FocusData'
import { DazzCalendarValue, DazzDateRange } from '../foundation/CalendarData'
import { startOfDay, sameDay } from '../foundation/DateData'
import { DazzDateRangeValue, DazzDateRangePreset, DazzDateRangeDuration, DazzDateRangePickerLabels,
  DazzDateRangeSummaryPart, DazzDateRangeSession, DATE_RANGE_PRESETS, DATE_RANGE_DURATIONS,
  dateRangeLabels, dateRangeCopy, dateRangeBlank, dateRangeClock, dateRangeTimes, dateRangeDurationOn, dateRangeOneDay, dateRangeSummary } from '../foundation/DateRangeData'
import { DazzCalendar } from './Calendar'
import { DazzButton, DazzToggle } from './Button'
import { DazzSelect } from './Select'
import { DazzMenu, DazzMenuItem } from './Menu'
import { DazzPopupSurface, popupShadow } from './PopupSurface'
import { DazzIcon } from './Icon'

interface DateRangeOptions {
  today: Date; months: 1 | 2; weekStartsOn: 0 | 1 | 2 | 3 | 4 | 5 | 6
  locale?: string; tint: DazzTint; labels: Record<string, string>
  presets: DazzDateRangePreset[]; durations: DazzDateRangeDuration[]; timeStep: number
}
function dateRangeSheetWidth(tokens: DazzTokens, months: number): number {
  const calendar = 7 * tokens.metric('calendar.small.cell-size') + 6 * tokens.metric('calendar.cell-gap')
  return months * calendar + (months - 1) * tokens.metric('spacing.4') +
    2 * tokens.metric('spacing.2') + tokens.metric('date-range-picker.side-width') + 2 * tokens.metric('stroke.hairline')
}
let pickerSequence: number = 0
class PickerFocusFrame extends FrameCallback {
  private action: () => void
  constructor(action: () => void) { super(); this.action = action }
  onIdle(_timeLeft: number): void { this.action() }
}
class DateRangePopupContext {
  owner: string
  session: DazzDateRangeSession = new DazzDateRangeSession()
  options: () => DateRangeOptions
  cancel: () => void
  done: () => void
  focusTrigger: () => void = () => {}
  forwardTab: (event: KeyEvent) => void = () => {}
  escape: () => void = () => {}
  touchOpened: boolean = false
  opening: number = 0
  focusReady: boolean = false
  constructor(owner: string, options: () => DateRangeOptions, cancel: () => void, done: () => void) {
    this.owner = owner; this.options = options; this.cancel = cancel; this.done = done
  }
}

@Component
struct DateRangeSummary {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.tabular') tabular: boolean = false
  @Prop value: DazzDateRangeValue
  @Prop locale: string | undefined = undefined
  @Prop controlSize: DazzSize | undefined = undefined
  @Prop ink: string = ''
  private formattedLocale: string | undefined = undefined
  private formatter: Intl.DateTimeFormat = new Intl.DateTimeFormat(undefined, { month: 'short', day: 'numeric', weekday: 'short' })
  type(): DazzTypography { return this.controlSize ? this.tokens.fieldLabel(this.controlSize) : this.tokens.type('labelSmall') }
  parts(): DazzDateRangeSummaryPart[] {
    if (this.locale !== this.formattedLocale) {
      this.formattedLocale = this.locale
      this.formatter = new Intl.DateTimeFormat(this.locale, { month: 'short', day: 'numeric', weekday: 'short' })
    }
    return dateRangeSummary(this.value, (date: Date): string => this.formatter.format(date))
  }
  gapTracking(): number {
    const width = this.getUIContext().getMeasureUtils().measureTextSize({ textContent: ' ', fontSize: this.tokens.fontSize(this.type().size),
      fontWeight: this.controlSize ? this.tokens.fieldWeight() : this.type().weight }).width
    return this.tokens.metric('spacing.1') - this.getUIContext().px2vp(Number(width))
  }
  build() {
    Text() {
      ForEach(this.parts(), (part: DazzDateRangeSummaryPart) => {
        if (part.separator) {
          Span(' ').letterSpacing(this.gapTracking())
          Span(part.text).fontColor(this.tokens.color('content-faint'))
          Span(' ').letterSpacing(this.gapTracking())
        } else { Span(part.text) }
      }, (part: DazzDateRangeSummaryPart, index: number): string => String(index) + ':' + part.text)
    }.fontSize(this.tokens.fontSize(this.type().size)).fontWeight(this.controlSize ? this.tokens.fieldWeight() : this.type().weight)
      .fontFeature(this.tabular ? '"tnum" on' : 'normal')
      .lineHeight(this.tokens.fontSize(this.type().leading)).fontColor(this.ink || this.tokens.color('content-secondary'))
      .maxLines(1).textOverflow({ overflow: TextOverflow.Ellipsis })
  }
}

/** Own state below the forwarded popup builder; its session survives cached trees. */
@Component
struct DateRangeSheet {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Provide({ allowOverride: 'dazzui.activation' }) activation: DazzActivationContext = new DazzActivationContext()
  @Provide({ allowOverride: 'dazzui.tabular' }) tabular: boolean = true
  popup: DateRangePopupContext = new DateRangePopupContext('', () => { throw new Error('DateRangeSheet requires a picker context') }, () => {}, () => {})
  @State draft: DazzDateRangeValue = dateRangeBlank(new Date())
  @State days: DazzDateRange = { start: null, end: null }
  @State month: Date = new Date()
  @State settings: DateRangeOptions = { today: new Date(), months: 1, weekStartsOn: 1,
    tint: 'primary', labels: dateRangeLabels({}), presets: DATE_RANGE_PRESETS,
    durations: DATE_RANGE_DURATIONS, timeStep: 15 }
  private changed: () => void = () => this.sync()
  @State selfFocus: boolean = false
  private opening: number = -1
  private stopFocus: () => void = () => {}
  aboutToAppear(): void {
    this.sync(); this.popup.session.observe(this.changed)
    this.stopFocus = observeFocus(this.getUIContext().getWindowName() ?? '', (id: string) => {
      if (this.popup.focusReady && id && id !== this.popup.owner + '-popup-focus') this.selfFocus = false
    })
  }
  aboutToDisappear(): void { this.popup.session.unobserve(this.changed); this.stopFocus() }
  sync(): void {
    if (this.opening !== this.popup.opening) {
      this.opening = this.popup.opening; this.selfFocus = this.popup.touchOpened
    }
    this.draft = dateRangeCopy(this.popup.session.draft)
    this.days = { start: this.popup.session.days.start, end: this.popup.session.days.end }
    this.month = this.popup.session.month
    this.settings = this.popup.options()
  }
  model(): DazzDateRangeSession { return this.popup.session }
  format(date: Date): string { return new Intl.DateTimeFormat(this.settings.locale, { month: 'short', day: 'numeric', weekday: 'short' }).format(date) }
  calendarWidth(): number { return 7 * this.tokens.metric('calendar.small.cell-size') + 6 * this.tokens.metric('calendar.cell-gap') }
  monthsWidth(): number { return this.settings.months * this.calendarWidth() + (this.settings.months - 1) * this.tokens.metric('spacing.4') + 2 * this.tokens.metric('spacing.2') }
  menuItems(): DazzMenuItem[] {
    return this.settings.presets.map((preset: DazzDateRangePreset): DazzMenuItem => {
      const span = preset.range(this.settings.today)
      return { label: preset.label, checked: sameDay(span.start, this.draft.start) && sameDay(span.end, this.draft.end),
        onSelect: () => this.model().preset(preset, this.settings.today) }
    })
  }
  timeWidth(): number {
    const label = this.tokens.fieldLabel('small')
    const width = this.getUIContext().getMeasureUtils().measureTextSize({ textContent: '00:00', fontSize: this.tokens.fontSize(label.size), fontWeight: this.tokens.fieldWeight() }).width
    return Math.max(this.tokens.metric('date-range-picker.time-min-width'), this.getUIContext().px2vp(Number(width)) + this.tokens.metric('icon.small') + this.tokens.metric('spacing.2') + 2 * this.tokens.fieldPadding() + 2 * this.tokens.metric('stroke.hairline'))
  }
  handleKey(event: KeyEvent): boolean {
    if (event.keyCode === 2070) {
      if (event.type === KeyType.Down) this.popup.escape()
      event.stopPropagation(); return true
    }
    if (event.keyCode !== 2049 || event.type !== KeyType.Down) return false
    const focused = focusTarget(this.getUIContext().getWindowName() ?? '')
    const reverse = event.getModifierKeyState?.(['Shift']) ?? false
    if (focused === this.popup.owner + '-popup-focus') {
      this.selfFocus = false
      if (reverse) this.popup.focusTrigger()
      else this.getUIContext().getFocusController().requestFocus(this.popup.owner + '-navigation')
      event.stopPropagation(); return true
    }
    if (reverse && focused === this.popup.owner + '-navigation') {
      this.popup.focusTrigger(); event.stopPropagation(); return true
    }
    if (!reverse && focused === this.popup.owner + '-done') {
      this.popup.forwardTab(event); event.stopPropagation(); return true
    }
    return false
  }
  @LocalBuilder keyLabel(label: string) {
    Text(label.toLocaleUpperCase()).fontSize(this.tokens.fontSize(this.tokens.type('captionSmall').size))
      .fontFeature('"tnum" on')
      .fontWeight(this.tokens.type('labelSmall').weight).lineHeight(this.tokens.fontSize(this.tokens.type('captionSmall').size))
      .letterSpacing(this.tokens.type('captionSmall').size * this.tokens.metric('date-range-picker.key-tracking'))
      .fontColor(this.tokens.color('content-subtle'))
  }
  @LocalBuilder quick() {
    DazzButton({ label: this.settings.labels['presets'], controlSize: 'tiny', variant: 'plain', tint: 'neutral', trailingIcon: 'ChevronDown12Regular' })
  }
  @LocalBuilder endpoint(start: boolean) {
    Column({ space: this.tokens.metric('spacing.1') }) {
      this.keyLabel(this.settings.labels[start ? 'start' : 'end'])
      Row({ space: this.tokens.metric('spacing.2') }) {
        Text(this.format(start ? this.draft.start : this.draft.end))
          .fontFeature('"tnum" on')
          .fontSize(this.tokens.fontSize(this.tokens.type('labelQuiet').size)).fontWeight(this.tokens.type('labelQuiet').weight)
          .lineHeight(this.tokens.fontSize(this.tokens.type('labelQuiet').size)).fontColor(this.tokens.color('content')).layoutWeight(1)
        DazzSelect({ controlSize: 'small', value: dateRangeClock(start ? this.draft.start : this.draft.end),
          options: dateRangeTimes(this.settings.timeStep, start ? this.draft.start : this.draft.end), disabled: this.draft.allDay,
          accessibilityLabel: this.settings.labels[start ? 'start' : 'end'] + ' time',
          onValueChange: (time: string) => { if (start) this.model().setStartTime(time); else this.model().setEndTime(time) } }).width(this.timeWidth()).id(this.popup.owner + (start ? '-start-time' : '-end-time'))
      }.width('100%')
    }.width('100%').alignItems(HorizontalAlign.Start)
  }
  @LocalBuilder body() {
    Column() {
      Flex({ direction: FlexDirection.Row, alignItems: ItemAlign.Stretch }) {
        Column() {
          DazzCalendar({ mode: 'range', controlSize: 'small', months: this.settings.months,
            focusNavigationOnAppear: !this.popup.touchOpened,
            navigationControlID: this.popup.owner + '-navigation',
            month: this.month, value: this.days, today: this.settings.today,
            weekStartsOn: this.settings.weekStartsOn, locale: this.settings.locale, tint: this.settings.tint,
            onMonthChange: (month: Date) => this.model().setMonth(month),
            onValueChange: (value: DazzCalendarValue) => this.model().setDays(value as DazzDateRange) })
        }.width(this.monthsWidth()).padding({ top: this.tokens.metric('spacing.1'), left: this.tokens.metric('spacing.2'), right: this.tokens.metric('spacing.2'), bottom: this.tokens.metric('spacing.2') })
        Column({ space: this.tokens.metric('spacing.3') }) {
          Row() { DazzMenu({ items: this.menuItems(), trigger: this.quick }) }
            .width('100%').margin({ left: -this.tokens.size('tiny', 'padding.inline') })
          this.endpoint(true)
          this.endpoint(false)
          Column({ space: this.tokens.metric('spacing.1.5') }) {
            this.keyLabel(this.settings.labels['duration'])
            Flex({ wrap: FlexWrap.Wrap, space: { main: LengthMetrics.vp(this.tokens.metric('spacing.1')), cross: LengthMetrics.vp(this.tokens.metric('spacing.1')) } }) {
              ForEach(this.settings.durations, (duration: DazzDateRangeDuration) => {
                DazzToggle({ text: duration.label, controlSize: 'tiny', variant: 'outlined', tint: this.settings.tint,
                  pressed: dateRangeDurationOn(this.draft, duration.minutes), disabled: !dateRangeOneDay(this.draft),
                  onPressedChange: () => this.model().setDuration(duration.minutes) })
              }, (duration: DazzDateRangeDuration): string => String(duration.minutes) + duration.label)
              DazzToggle({ text: this.settings.labels['allDay'], controlSize: 'tiny', variant: 'outlined', tint: this.settings.tint,
                pressed: this.draft.allDay, onPressedChange: (allDay: boolean) => this.model().setAllDay(allDay) })
            }.width('100%').height(LayoutPolicy.wrapContent)
          }.alignItems(HorizontalAlign.Start).width('100%')
        }.width(this.tokens.metric('date-range-picker.side-width')).alignItems(HorizontalAlign.Start)
          .padding({ top: this.tokens.metric('spacing.2'), left: this.tokens.metric('spacing.3'), right: this.tokens.metric('spacing.3'), bottom: this.tokens.metric('spacing.3') })
          .border({ width: { left: this.tokens.metric('stroke.hairline') }, color: this.tokens.color('border') })
      }.width('100%').height(LayoutPolicy.wrapContent)
      Row({ space: this.tokens.metric('spacing.3') }) {
        DateRangeSummary({ value: this.draft, locale: this.settings.locale }).layoutWeight(1)
        Row({ space: this.tokens.metric('spacing.2') }) {
          DazzButton({ label: this.settings.labels['cancel'], variant: 'normal', onAction: () => this.popup.cancel() })
          DazzButton({ label: this.settings.labels['done'], focusControlID: this.popup.owner + '-done', variant: 'filled', tint: this.settings.tint, onAction: () => this.popup.done() })
        }
      }.width('100%').padding({ top: this.tokens.metric('spacing.2'), bottom: this.tokens.metric('spacing.2'), left: this.tokens.metric('spacing.3'), right: this.tokens.metric('spacing.2') })
        .border({ width: { top: this.tokens.metric('stroke.hairline') }, color: this.tokens.color('border') })
    }.width('100%').id(this.popup.owner + '-popup-focus').focusable(true).tabStop(this.selfFocus)
      .accessibilityRole(AccessibilityRoleType.DIALOG).focusBox({ strokeWidth: LengthMetrics.vp(0) })
      .onBlur(() => forgetFocus(this.getUIContext().getWindowName() ?? '', this.popup.owner + '-popup-focus'))
  }
  // Stateful controls are built under their state owner, inside the popup skin.
  build() {
    Column() { this.body() }.width('100%').height(LayoutPolicy.wrapContent)
      .onKeyEvent((event: KeyEvent): boolean => this.handleKey(event))
  }
}

/** A field with a native popup; only Done publishes its editing session. */
@Component
export struct DazzDateRangePicker {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop value: DazzDateRangeValue | null | undefined = undefined
  @Prop defaultValue: DazzDateRangeValue | null = null
  @Prop @Watch('refreshSheet') today: Date = new Date()
  @Prop @Watch('refreshSheet') months: 1 | 2 = 1
  @Prop @Watch('refreshSheet') weekStartsOn: 0 | 1 | 2 | 3 | 4 | 5 | 6 = 1
  @Prop @Watch('refreshSheet') locale: string | undefined = undefined
  @Prop @Watch('refreshSheet') presets: DazzDateRangePreset[] = DATE_RANGE_PRESETS
  @Prop @Watch('refreshSheet') durations: DazzDateRangeDuration[] = DATE_RANGE_DURATIONS
  @Prop @Watch('refreshSheet') timeStep: number = 15
  @Prop @Watch('refreshSheet') labels: DazzDateRangePickerLabels = {}
  @Prop @Watch('refreshSheet') tint: DazzTint = 'primary'
  @Prop controlSize: 'small' | 'medium' | 'large' = 'medium'
  @Prop state: 'default' | 'error' = 'default'
  @Prop disabled: boolean = false
  @Prop fullWidth: boolean = false
  @Prop accessibilityLabel: string = ''
  onValueChange: (value: DazzDateRangeValue) => void = () => {}
  @State inner: DazzDateRangeValue | null = null
  @State open: boolean = false
  @State focused: boolean = false
  private owner: string = 'dazz-date-range-' + String(++pickerSequence)
  private alive: boolean = false
  private focusSequence: number = 0
  private consumedKey: number = -1
  private popup: DateRangePopupContext = new DateRangePopupContext(this.owner, () => this.options(), () => this.cancel(), () => this.done())
  aboutToAppear(): void {
    this.alive = true; this.inner = this.defaultValue ? dateRangeCopy(this.defaultValue) : null
    this.popup.focusTrigger = () => this.getUIContext().getFocusController().requestFocus(this.owner + '-trigger')
    this.popup.forwardTab = (event: KeyEvent) => this.closeAndTab(event)
    this.popup.escape = () => { this.consumedKey = 2070; this.cancel() }
  }
  aboutToDisappear(): void {
    this.alive = false; this.focusSequence++; this.popup.session.dispose()
    forgetFocus(this.getUIContext().getWindowName() ?? '', this.owner + '-trigger')
    forgetFocus(this.getUIContext().getWindowName() ?? '', this.owner + '-popup-focus')
  }
  refreshSheet(): void { this.popup.session.invalidate() }
  current(): DazzDateRangeValue | null { return this.value === undefined ? this.inner : this.value }
  options(): DateRangeOptions {
    return { today: startOfDay(this.today), months: this.months, weekStartsOn: this.weekStartsOn, locale: this.locale,
      tint: this.tint, labels: dateRangeLabels(this.labels), presets: this.presets, durations: this.durations, timeStep: this.timeStep }
  }
  cancel(): void {
    if (!this.open) return
    this.focusSequence++; this.popup.session.close(); this.open = false
  }
  closeAndTab(event: KeyEvent): void {
    const ui = this.getUIContext()
    let root: FrameNode | null = ui.getAttachedFrameNodeById(this.owner + '-trigger')
    if (!root) return
    let parent = root.getParent()
    while (parent) { root = parent; parent = root.getParent() }
    this.cancel()
    this.popup.focusTrigger()
    // Dispatch through the page scope so native traversal can continue beyond
    // the trigger. A leaf dispatch only invokes that button's event handler.
    ui.dispatchKeyEvent(root.getUniqueId(), event)
  }
  focusOnOpen(): void {
    const sequence = ++this.focusSequence
    this.getUIContext().postFrameCallback(new PickerFocusFrame(() => {
      if (!this.alive || !this.open || this.focusSequence !== sequence) return
      const target = this.owner + (this.popup.touchOpened ? '-popup-focus' : '-navigation')
      this.getUIContext().getFocusController().requestFocus(target)
      if (this.popup.touchOpened) rememberFocus(this.getUIContext().getWindowName() ?? '', target)
      this.popup.focusReady = true
    }))
  }
  done(): void {
    this.popup.session.confirm((value: DazzDateRangeValue) => { if (this.value === undefined) this.inner = value; this.onValueChange(value) })
    this.open = false
  }
  openPicker(): void {
    if (this.disabled) return
    this.popup.focusReady = false; this.popup.opening++
    this.popup.session.start(this.current(), startOfDay(this.today)); this.open = true
  }
  effectiveTint(): DazzTint { return this.state === 'error' ? 'danger' : this.tint }
  surface(): string { return this.disabled ? this.tokens.paint('normal', 'surface', 'disabled', this.effectiveTint()) : this.tokens.color(this.state === 'error' ? this.effectiveTint() + '.50' : this.open || this.focused ? 'surface' : 'surface-muted') }
  edge(): string { return this.tokens.color(this.state === 'error' || this.open || this.focused ? this.effectiveTint() + '.600' : 'border-strong') }
  ink(): string { return this.disabled ? this.tokens.paint('normal', 'content', 'disabled', this.effectiveTint()) : this.tokens.color(this.state === 'error' ? this.effectiveTint() + '.800' : 'content') }
  summaryText(): string {
    const value = this.current()
    const format = new Intl.DateTimeFormat(this.locale, { month: 'short', day: 'numeric', weekday: 'short' })
    return value ? dateRangeSummary(value, (date: Date): string => format.format(date)).map((part: DazzDateRangeSummaryPart): string => part.separator ? ' ' + part.text + ' ' : part.text).join('') : dateRangeLabels(this.labels)['placeholder']
  }
  triggerWidth(): number {
    const type = this.tokens.fieldLabel(this.controlSize)
    const width = (label: string): number => this.getUIContext().px2vp(Number(this.getUIContext().getMeasureUtils().measureTextSize({ textContent: label, fontSize: this.tokens.fontSize(type.size), fontWeight: this.tokens.fieldWeight() }).width))
    const current = this.current()
    const format = new Intl.DateTimeFormat(this.locale, { month: 'short', day: 'numeric', weekday: 'short' })
    const label = current ? dateRangeSummary(current, (date: Date): string => format.format(date)).reduce((sum: number, part: DazzDateRangeSummaryPart): number => sum + width(part.text) + (part.separator ? 2 * this.tokens.metric('spacing.1') : 0), 0) : width(dateRangeLabels(this.labels)['placeholder'])
    return Math.max(this.tokens.metric('date-range-picker.trigger-min-width'), label + 2 * this.tokens.metric('icon.small') + 2 * this.tokens.metric('spacing.2') + 2 * this.tokens.fieldPadding() + 2 * this.tokens.metric('stroke.hairline'))
  }
  @LocalBuilder sheetContent() { DateRangeSheet({ popup: this.popup }) }
  @LocalBuilder sheet() {
    DazzPopupSurface({ contentWidth: dateRangeSheetWidth(this.tokens, this.months), contentPadding: 0, content: this.sheetContent }).id(this.owner + '-sheet')
  }
  build() {
    Button({ type: ButtonType.Normal }) {
      Row({ space: this.tokens.metric('spacing.2') }) {
        DazzIcon({ name: 'CalendarLtr16Regular', iconSize: this.tokens.metric('icon.small'), color: this.tokens.color('content-subtle') })
        if (this.current()) { DateRangeSummary({ value: this.current()!, locale: this.locale, controlSize: this.controlSize, ink: this.ink() }).layoutWeight(1) }
        else { Text(dateRangeLabels(this.labels)['placeholder']).fontSize(this.tokens.fontSize(this.tokens.fieldLabel(this.controlSize).size)).fontWeight(this.tokens.fieldWeight()).fontColor(this.tokens.color('content-faint')).maxLines(1).textOverflow({ overflow: TextOverflow.Ellipsis }).layoutWeight(1) }
        DazzIcon({ name: 'ChevronDown12Regular', iconSize: this.tokens.metric('icon.small'), color: this.tokens.color('content-subtle') })
      }.width('100%')
    }.id(this.owner + '-trigger').height(this.tokens.fieldSize(this.controlSize)).width(this.fullWidth ? '100%' : this.triggerWidth())
      .constraintSize({ minWidth: this.tokens.metric('date-range-picker.trigger-min-width'), maxWidth: '100%' })
      .padding({ left: this.tokens.fieldPadding(), right: this.tokens.fieldPadding(), top: 0, bottom: 0 })
      .backgroundColor(this.surface()).border({ width: this.tokens.metric('stroke.hairline'), color: this.edge(), radius: this.tokens.metric('control.field.radius') })
      .outline({ width: this.focused || this.open ? this.tokens.metric('focus.width') : 0, color: this.tokens.focusGlow(this.effectiveTint()), radius: this.tokens.metric('control.field.radius') + this.tokens.metric('focus.width') })
      .enabled(!this.disabled).focusOnTouch(true).stateEffect(false).accessibilityText(this.accessibilityLabel || this.summaryText())
      .accessibilityDescription(this.summaryText() + (this.open ? ', expanded' : ', collapsed'))
      .onFocus(() => {
        rememberFocus(this.getUIContext().getWindowName() ?? '', this.owner + '-trigger')
        this.focused = this.getUIContext().getFocusController().isActive()
      }).onBlur(() => {
        forgetFocus(this.getUIContext().getWindowName() ?? '', this.owner + '-trigger')
        this.focused = false
      })
      .onClick((event: ClickEvent) => {
        if (this.open) this.cancel()
        else { this.popup.touchOpened = event.source === SourceType.TouchScreen; this.openPicker() }
      })
      .onKeyEvent((event: KeyEvent): boolean => {
        if (event.type === KeyType.Up && event.keyCode === this.consumedKey) {
          this.consumedKey = -1; event.stopPropagation(); return true
        }
        if (event.type !== KeyType.Down) return false
        this.consumedKey = -1
        this.getUIContext().getFocusController().activate(true)
        if (this.open && event.keyCode === 2070) {
          this.consumedKey = 2070; this.cancel(); event.stopPropagation(); return true
        }
        if (!this.open || event.keyCode !== 2049) return false
        if (event.getModifierKeyState?.(['Shift']) ?? false) {
          this.closeAndTab(event); event.stopPropagation(); return true
        }
        this.getUIContext().getFocusController().requestFocus(this.owner + '-navigation')
        event.stopPropagation(); return true
      })
      .bindPopup(this.open, { builder: this.sheet, placement: Placement.BottomLeft, targetSpace: this.tokens.metric('spacing.1'),
        popupColor: this.tokens.color('surface-overlay'), backgroundBlurStyle: BlurStyle.NONE, enableArrow: false, radius: this.tokens.metric('radius.large'), mask: false,
        shadow: popupShadow(this.tokens, this.getUIContext().vp2px(1)), autoCancel: true, focusable: true, transition: TransitionEffect.IDENTITY,
        onStateChange: (event: PopupStateChangeParam) => { if (event.isVisible) this.focusOnOpen(); else this.cancel() } })
  }
}
