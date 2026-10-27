import { ColorMetrics, LengthMetrics, UIObserver } from '@kit.ArkUI'
import { DazzTokens } from '../foundation/Tokens'
import { DazzTint } from '../foundation/TokenTypes'
import { DazzPointerContext } from '../foundation/Pointer'
import {
  DazzShortcutEvent,
  formatShortcut,
  isShortcutModifier,
  heldShortcutModifiers,
} from '../foundation/Shortcut'
import { DazzIcon } from './Icon'

let recorderSequence: number = 0

/** Native buttons own activation; only recording consumes key events. */
@Component
export struct DazzShortcutRecorder {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.pointer') pointer: DazzPointerContext = new DazzPointerContext()
  @Prop value: string = ''
  @Prop placeholder: string = 'Record shortcut'
  @Prop recordingLabel: string = 'Press keys…'
  @Prop clearLabel: string = 'Clear shortcut'
  @Prop accessibilityLabel: string = ''
  @Prop state: string = 'default'
  @Prop tint: DazzTint = 'primary'
  @Prop @Watch('disabledChanged') disabled: boolean = false
  onValueChange: (value: string) => void = () => {}
  onClear?: () => void
  @State recording: boolean = false
  @State held: string = ''
  @State rejected: boolean = false
  @State clearHovered: boolean = false
  private recorderID: string = 'dazzui-recorder-' + String(++recorderSequence)
  private rejectTimer: number = -1
  private releaseKey: number = -1
  private modifierKeys: number[] = []
  private observer?: UIObserver
  private watchingClicks: boolean = false
  private outsideClick: (event: ClickEvent) => void = (
    event: ClickEvent
  ): void => {
    if (!this.recording ||
      (event.source !== SourceType.Mouse &&
        event.source !== SourceType.TouchScreen)) return
    this.outsidePress(event.windowX, event.windowY)
  }
  private outsidePress: (x: number, y: number) => void = (
    x: number, y: number
  ): void => {
    if (!this.recording) return
    const ui = this.getUIContext()
    const node = ui.getFrameNodeById(this.recorderID + '-area')
    if (!node) {
      this.stop()
      return
    }
    // Position is vp; measured dimensions are px. Ignore bubbling clicks that
    // still fall inside this recorder, including its clear-button overlay.
    const position = node.getPositionToWindow()
    const size = node.getMeasuredSize()
    if (x < position.x || y < position.y ||
      x > position.x + ui.px2vp(size.width) ||
      y > position.y + ui.px2vp(size.height)) this.stop()
  }

  aboutToDisappear(): void {
    this.stop()
    clearTimeout(this.rejectTimer)
  }
  disabledChanged(): void {
    if (this.disabled) {
      this.stop()
      this.releaseKey = -1
    }
  }
  stop(): void {
    this.recording = false
    this.held = ''
    this.modifierKeys = []
    if (this.watchingClicks) this.observer?.off('willClick', this.outsideClick)
    this.pointer.off(this.outsidePress)
    this.watchingClicks = false
  }
  begin(): void {
    if (this.disabled || this.recording) return
    this.recording = true
    this.pointer.on(this.outsidePress)
    this.observer = this.getUIContext().getUIObserver()
    this.observer.on('willClick', this.outsideClick)
    this.watchingClicks = true
    try {
      this.getUIContext().getFocusController().requestFocus(this.recorderID)
    } catch (_) {}
  }
  clear(): void {
    if (this.disabled) return
    this.stop()
    if (this.onClear) this.onClear()
    else this.onValueChange('')
  }
  clearable(): boolean {
    return this.value !== '' && !this.recording && !this.disabled
  }
  glyphs(): boolean {
    return this.recording ? this.held !== '' : this.value !== ''
  }
  label(): string {
    return this.recording
      ? this.held || this.recordingLabel
      : this.value || this.placeholder
  }
  ink(): string {
    return this.tokens.color(
      this.disabled ? 'content-faint' : this.state === 'error'
        ? 'danger.800' : this.glyphs() ? 'content' : 'content-faint'
    )
  }
  edge(): string {
    return this.tokens.color(
      this.state === 'error' || this.rejected ? 'danger.600'
        : this.recording ? this.tint + '.600' : 'border-strong'
    )
  }
  eventValue(event: KeyEvent): DazzShortcutEvent {
    return {
      keyCode: event.keyCode,
      ctrl: event.getModifierKeyState?.(['Ctrl']) ?? false,
      alt: event.getModifierKeyState?.(['Alt']) ?? false,
      shift: event.getModifierKeyState?.(['Shift']) ?? false,
      meta: event.metaKey !== 0,
    }
  }
  handleKey(event: KeyEvent): boolean {
    if (this.disabled) return false
    if (this.releaseKey === event.keyCode) {
      if (event.type === KeyType.Up) this.releaseKey = -1
      event.stopPropagation()
      return true
    }
    if (!this.recording) {
      const enter = event.keyCode === 2054 || event.keyCode === 2119
      const space = event.keyCode === 2050
      if (!enter && !space) return false
      this.getUIContext().getFocusController().activate(true)
      if ((enter && event.type === KeyType.Down) ||
        (space && event.type === KeyType.Up)) {
        this.begin()
        if (event.type === KeyType.Down) this.releaseKey = event.keyCode
      }
      event.stopPropagation()
      return true
    }
    // Returning true also prevents native Space activation and Tab traversal.
    event.stopPropagation()
    const chord = this.eventValue(event)
    if (isShortcutModifier(event.keyCode)) {
      this.modifierKeys = this.modifierKeys.filter(
        (code: number): boolean => code !== event.keyCode
      )
      if (event.type === KeyType.Down) this.modifierKeys.push(event.keyCode)
      this.held = heldShortcutModifiers(
        chord, this.modifierKeys, event.type === KeyType.Up
      )
    } else if (event.type === KeyType.Down) {
      if (event.keyCode === 2070) this.stop()
      else if (!chord.ctrl && !chord.alt && !chord.meta && !chord.shift &&
        (event.keyCode === 2055 || event.keyCode === 2071)) this.clear()
      else {
        const value = formatShortcut(chord)
        if (value === null) {
          if (!this.rejected) {
            this.rejected = true
            this.rejectTimer = setTimeout(() => {
              this.rejected = false
            }, this.tokens.metric('shortcut-recorder.reject-duration'))
          }
          return true
        }
        this.stop()
        this.onValueChange(value)
      }
      this.releaseKey = event.keyCode
    }
    return true
  }
  build() {
    Stack({ alignContent: Alignment.End }) {
      Button({ type: ButtonType.Normal }) {
        Text(this.label())
          .fontSize(this.tokens.fontSize(this.tokens.type('labelStrong').size))
          .fontWeight(
            this.tokens.type(
              this.glyphs() ? 'labelStrong' : 'labelQuiet'
            ).weight
          )
          .lineHeight(
            this.tokens.fontSize(this.tokens.type('labelStrong').leading)
          )
          .fontColor(this.ink()).maxLines(1)
      }.id(this.recorderID).height(this.tokens.fieldSize('medium'))
        .constraintSize({
          minWidth: this.tokens.metric('shortcut-recorder.width'),
        })
        .padding({
          top: 0,
          bottom: 0,
          left: this.tokens.metric(
            this.clearable() ? 'spacing.8' : 'spacing.3'
          ),
          right: this.tokens.metric(
            this.clearable() ? 'spacing.8' : 'spacing.3'
          ),
        })
        .backgroundColor(this.tokens.color(
          this.state === 'error' ? 'danger.50'
            : this.recording ? 'surface' : 'surface-muted'
        ))
        .border({
          width: this.tokens.metric('stroke.hairline'),
          radius: this.tokens.metric('control.field.radius'),
          color: this.edge(),
        })
        .outline({
          width: this.rejected || this.recording
            ? this.tokens.metric('focus.width') : 0,
          color: this.tokens.color(this.rejected ? 'danger.50' : 'focus.glow'),
          radius: this.tokens.metric('control.field.radius'),
        })
        .focusBox({
          margin: LengthMetrics.vp(this.tokens.metric('focus.offset')),
          strokeWidth: LengthMetrics.vp(this.tokens.metric('focus.width')),
          strokeColor: ColorMetrics.resourceColor(this.tokens.focus()),
        })
        .stateEffect(false).enabled(!this.disabled).focusOnTouch(true)
        .accessibilityText(this.accessibilityLabel || this.placeholder)
        .accessibilityDescription(this.label())
        .accessibilitySelected(this.recording)
        .onClick(() => this.begin())
        .onBlur(() => {
          this.stop()
          this.releaseKey = -1
        })
        // Capture a recording chord before window shortcuts dispatch. Normal
        // button activation still follows the native key-event path below.
        .onKeyPreIme((event: KeyEvent): boolean => this.recording ? this.handleKey(event) : false)
        .onKeyEvent((event: KeyEvent): boolean => this.handleKey(event))
      if (this.clearable()) {
        Button({ type: ButtonType.Normal }) {
          DazzIcon({
            name: 'DismissCircle12Filled',
            iconSize: this.tokens.metric('icon.small'),
            color: this.tokens.color(
              this.clearHovered ? 'content-subtle' : 'content-faint'
            ),
          })
        }.id(this.recorderID + '-clear')
          .width(this.tokens.metric('icon.small'))
          .height(this.tokens.metric('icon.small'))
          .padding(0).margin({ right: this.tokens.metric('spacing.2') })
          .backgroundColor(Color.Transparent).stateEffect(false)
          .focusBox({
            margin: LengthMetrics.vp(this.tokens.metric('focus.offset')),
            strokeWidth: LengthMetrics.vp(this.tokens.metric('focus.width')),
            strokeColor: ColorMetrics.resourceColor(this.tokens.focus()),
          })
          .accessibilityText(this.clearLabel)
          .onHover((hover: boolean) => { this.clearHovered = hover })
          .onClick(() => this.clear())
      }
    }.id(this.recorderID + '-area')
      .width(LayoutPolicy.wrapContent)
      .height(this.tokens.fieldSize('medium'))
  }
}
