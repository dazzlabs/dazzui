import { KeyCode } from '@kit.InputKit'
import { LengthMetrics } from '@kit.ArkUI'
import { DazzTokens } from '../foundation/Tokens'
import { DazzTint } from '../foundation/TokenTypes'
import { DazzSliderValue, DazzSliderSize, sliderValues, sliderSnap, sliderChange, sliderKey, sliderNearest } from '../foundation/SliderData'
import { DazzSliderPaint } from './SliderPaint'

let sliderSequence: number = 0

@Component
export struct DazzSlider {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop value: DazzSliderValue | undefined = undefined
  @Prop defaultValue: DazzSliderValue | undefined = undefined
  @Prop min: number = 0
  @Prop max: number = 100
  @Prop step: number = 1
  @Prop controlSize: DazzSliderSize = 'medium'
  @Prop tint: DazzTint = 'primary'
  @Prop disabled: boolean = false
  @Prop showValue: boolean = false
  @Prop accessibilityLabel: string = ''
  @State localValue: DazzSliderValue = 0
  @State pointerIndex: number = 0
  @State nativeRevision: number = 0
  @State hoverIndex: number = -1
  @State focusIndex: number = -1
  @State private pointerActive: boolean = false
  @State private controlWidth: number = 0
  onValueChange: (value: DazzSliderValue) => void = () => {}
  private sliderID: string = 'dazzui-slider-' + String(++sliderSequence)

  aboutToAppear(): void {
    const initial = this.defaultValue ?? this.min
    this.localValue = Array.isArray(initial) ? initial.slice() : initial
  }
  current(): number[] {
    // Also invalidate the native builder when a controlled owner refuses the
    // proposal, so the native Slider receives the authoritative value again.
    if (this.nativeRevision < 0) return []
    return sliderValues(this.value ?? this.localValue, this.min, this.max)
  }
  indices(): number[] { return this.current().map((_value: number, index: number): number => index) }
  range(): boolean { return Array.isArray(this.value ?? this.localValue) }
  thumbID(index: number): string { return this.sliderID + '-' + String(index) }
  diameter(): number { return this.tokens.metric('slider.' + this.controlSize + '.thumb') }
  percent(value: number): number { return 100 * (value - this.min) / (this.max - this.min) }
  start(): number { return this.current().length > 1 ? this.percent(this.current()[0]) : 0 }
  end(): number { const values = this.current(); return this.percent(values[values.length - 1] ?? this.min) }
  commit(next: number[]): void {
    const previous = this.current()
    if (this.disabled || next.every((value: number, index: number): boolean => value === previous[index])) return
    const value: DazzSliderValue = this.range() ? next : next[0]
    if (this.value === undefined) this.localValue = value
    this.onValueChange(Array.isArray(value) ? value.slice() : value)
  }
  change(index: number, value: number, mode: SliderChangeMode): void {
    if (this.disabled) return
    const push = this.pointerActive || mode === SliderChangeMode.Begin || mode === SliderChangeMode.Moving || mode === SliderChangeMode.Click
    const candidate = sliderSnap(value, this.min, this.max, this.step)
    this.commit(sliderChange(this.current(), index, candidate, this.min, this.max, push))
    if (this.value !== undefined) this.nativeRevision++
    if (mode === SliderChangeMode.End) this.pointerActive = false
  }
  handleKey(index: number, event: KeyEvent): boolean {
    if (this.disabled) return false
    let key = ''
    if (event.keyCode === KeyCode.KEYCODE_DPAD_UP) key = 'ArrowUp'
    else if (event.keyCode === KeyCode.KEYCODE_DPAD_DOWN) key = 'ArrowDown'
    else if (event.keyCode === KeyCode.KEYCODE_DPAD_LEFT) key = 'ArrowLeft'
    else if (event.keyCode === KeyCode.KEYCODE_DPAD_RIGHT) key = 'ArrowRight'
    else if (event.keyCode === KeyCode.KEYCODE_PAGE_UP) key = 'PageUp'
    else if (event.keyCode === KeyCode.KEYCODE_PAGE_DOWN) key = 'PageDown'
    else if (event.keyCode === KeyCode.KEYCODE_MOVE_HOME) key = 'Home'
    else if (event.keyCode === KeyCode.KEYCODE_MOVE_END) key = 'End'
    else return false
    if (event.type === KeyType.Down) {
      this.getUIContext().getFocusController().activate(true)
      this.focusIndex = index
      const shift = event.getModifierKeyState?.(['Shift']) ?? false
      const next = sliderKey(this.current(), index, key, shift, this.min, this.max, this.step, 10)
      if (next) this.commit(next)
    }
    event.stopPropagation()
    return true
  }
  target(info: TouchTestInfo[]): TouchResult {
    const child = info.find((item: TouchTestInfo): boolean => item.id === this.thumbID(0))
    if (this.disabled || !child || child.rect.width <= 0) return { strategy: TouchTestStrategy.DEFAULT }
    // NONE has no system thumb/shadow inset. Its native input range is exactly
    // the ds track, including both endpoints, regardless of the system theme.
    const fraction = child.x / child.rect.width
    const value = this.min + fraction * (this.max - this.min)
    const index = sliderNearest(this.current(), value, this.max)
    if (index >= 0) this.pointerIndex = index
    return index < 0 ? { strategy: TouchTestStrategy.DEFAULT } : { strategy: TouchTestStrategy.FORWARD, id: this.thumbID(index) }
  }
  @LocalBuilder thumb(index: number) {
    Slider({ value: this.current()[index], min: this.min, max: this.max, step: this.step, style: SliderStyle.NONE })
      .id(this.thumbID(index))
      .focusOnTouch(true)
      .zIndex(index === this.pointerIndex ? 1 : 0)
      .width('100%')
      .height(this.tokens.size(this.controlSize))
      .responseRegion({ x: -this.diameter() / 2, y: 0, width: this.controlWidth + this.diameter(), height: this.tokens.size(this.controlSize) })
      .mouseResponseRegion({ x: -this.diameter() / 2, y: 0, width: this.controlWidth + this.diameter(), height: this.tokens.size(this.controlSize) })
      .trackThickness(this.tokens.metric('slider.' + this.controlSize + '.track'))
      .selectedColor(Color.Transparent)
      .trackColor(Color.Transparent)
      .focusBox({ strokeWidth: LengthMetrics.vp(0) })
      .enabled(!this.disabled)
      .accessibilityText(this.accessibilityLabel)
      .onFocus(() => { this.focusIndex = this.getUIContext().getFocusController().isActive() ? index : -1 })
      .onBlur(() => { if (this.focusIndex === index) this.focusIndex = -1 })
      .onTouch((event: TouchEvent) => {
        this.pointerActive = event.type === TouchType.Down || event.type === TouchType.Move
        if (this.pointerActive) { this.pointerIndex = index; this.focusIndex = -1 }
      })
      .onMouse((event: MouseEvent) => {
        if (event.action === MouseAction.Press) { this.pointerActive = true; this.pointerIndex = index; this.focusIndex = -1 }
        else if (event.action === MouseAction.Release) this.pointerActive = false
      })
      .onKeyPreIme((event: KeyEvent): boolean => this.handleKey(index, event))
      .onChange((value: number, mode: SliderChangeMode) => this.change(index, value, mode))
  }
  build() {
    Row({ space: this.tokens.metric('spacing.3') }) {
      Stack({ alignContent: Alignment.Center }) {
        ForEach(this.indices(), (index: number) => { this.thumb(index) }, (index: number): string => String(index))
        DazzSliderPaint({ contentWidth: this.controlWidth, contentHeight: this.tokens.size(this.controlSize), anchorID: this.thumbID(0),
          fractions: this.current().map((value: number): number => this.percent(value) / 100),
          controlSize: this.controlSize, tint: this.tint, disabled: this.disabled,
          hotIndex: this.pointerActive ? this.pointerIndex : this.hoverIndex, focusIndex: this.focusIndex })
          .position({ x: 0, y: 0 }).zIndex(2).hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
      }
        .layoutWeight(1).height(this.tokens.size(this.controlSize))
        .onAreaChange((_old: Area, area: Area) => { this.controlWidth = Number(area.width) })
        .onMouse((event: MouseEvent) => {
          if (event.action === MouseAction.Move && !this.pointerActive && this.controlWidth > 0) {
            this.pointerIndex = sliderNearest(this.current(), this.min + event.x / this.controlWidth * (this.max - this.min), this.max)
            const center = this.controlWidth * this.percent(this.current()[this.pointerIndex]) / 100
            this.hoverIndex = !this.disabled && Math.abs(event.x - center) <= this.diameter() / 2 && Math.abs(event.y - this.tokens.size(this.controlSize) / 2) <= this.diameter() / 2 ? this.pointerIndex : -1
          }
        })
        .onHover((hover: boolean) => { if (!hover) this.hoverIndex = -1 })
        .onChildTouchTest((info: TouchTestInfo[]): TouchResult => this.target(info))
      if (this.showValue) {
        Text(this.current().map((value: number): string => value.toLocaleString()).join(' – '))
          .fontSize(this.tokens.fontSize(this.tokens.type('labelSmall').size))
          .fontFamily('monospace').lineHeight(this.tokens.fontSize(this.tokens.type('labelSmall').size))
          .fontColor(this.tokens.color('content-muted'))
          .constraintSize({ minWidth: this.tokens.metric('spacing.10') })
          .maxLines(1).flexShrink(0)
          .textAlign(TextAlign.End)
      }
    }.width('100%')
  }
}
