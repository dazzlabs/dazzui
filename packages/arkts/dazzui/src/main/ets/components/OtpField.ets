import { DazzTokens } from '../foundation/Tokens'
import { DazzSize, DazzTint } from '../foundation/TokenTypes'
import { DazzFormContext } from '../foundation/FormData'
import { normalizeOtp, replaceOtp, removeOtp } from '../foundation/Otp'
import { FrameCallback } from '@kit.ArkUI'

class OtpEditFrame extends FrameCallback {
  private action: () => void
  constructor(action: () => void) {
    super()
    this.action = action
  }
  onIdle(_timeLeft: number): void { this.action() }
}

let otpSequence: number = 0

/** Native editors own IME, selection, paste menus and code autofill. */
@Component
export struct DazzOtpField {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.form') form: DazzFormContext = new DazzFormContext()
  @Prop @Watch('valueChanged') value: string | undefined = undefined
  @Prop defaultValue: string = ''
  @Prop @Watch('lengthChanged') length: number = 6
  @Prop groupAfter: number = 0
  @Prop controlSize: DazzSize = 'medium'
  @Prop state: string = 'default'
  @Prop tint: DazzTint = 'primary'
  @Prop disabled: boolean = false
  @Prop readOnly: boolean = false
  @Prop required: boolean = false
  @Prop autoSubmit: boolean = false
  @Prop accessibilityLabel: string = 'Code'
  @State @Watch('valueChanged') localValue: string = ''
  @State focused: number = -1
  @State invalid: boolean = false
  onValueChange: (value: string) => void = () => {}
  onValueComplete: (value: string) => void = () => {}
  onValueInvalid: (value: string) => void = () => {}
  private fieldID: string = 'dazzui-otp-' + String(++otpSequence)
  private controllers: TextInputController[] = []
  private pendingValue: string | undefined = undefined
  private pendingTarget: number = 0
  private pendingComplete: boolean = false
  private focusTimer: number = -1
  private logicalIndex: number = -1
  private focusTarget: number = -1
  private focusPending: boolean = false
  private draftValue: string | undefined = undefined
  private draftTarget: number = 0
  private draftEpoch: number = 0
  private draftFramePending: boolean = false
  private draftHistory: string[] = []

  aboutToAppear(): void {
    this.controllers = this.indices().map(() => new TextInputController())
    this.localValue = normalizeOtp(this.defaultValue, this.length)
    if (this.form.active) this.form.register(this.fieldID, {
      validate: (focus: boolean): boolean => {
        this.invalid = !this.disabled && !this.readOnly &&
          (this.required || this.code().length > 0) &&
          this.code().length !== this.length
        if (this.invalid && focus) this.focus(this.code().length)
        return !this.invalid
      },
      reset: () => {
        this.localValue = normalizeOtp(this.defaultValue, this.length)
        this.invalid = false
        this.onValueChange(this.localValue)
      },
    })
  }
  aboutToDisappear(): void {
    clearTimeout(this.focusTimer)
    this.clearDraft()
    this.form.unregister(this.fieldID)
  }
  lengthChanged(): void {
    clearTimeout(this.focusTimer)
    this.clearDraft()
    this.pendingValue = undefined
    this.controllers = this.indices().map(() => new TextInputController())
    this.focused = -1
    this.logicalIndex = -1
    this.focusTarget = -1
    this.focusPending = false
  }
  indices(): number[] {
    const result: number[] = []
    if (Number.isFinite(this.length)) {
      for (let index = 0; index < Math.floor(this.length); index++) {
        result.push(index)
      }
    }
    return result
  }
  code(): string {
    return normalizeOtp(this.value ?? this.localValue, this.length)
  }
  editCode(): string { return this.draftValue ?? this.code() }
  clearDraft(): void {
    this.draftEpoch++
    this.draftValue = undefined
    this.draftHistory = []
    this.draftFramePending = false
  }
  queueDraft(value: string, target: number): void {
    if (this.value === undefined) return
    this.draftValue = value
    this.draftTarget = Math.max(0,
      Math.min(target, value.length, this.length - 1))
    this.draftHistory.push(value)
    if (this.draftFramePending) return
    this.draftFramePending = true
    const epoch = ++this.draftEpoch
    // ArkUI can deliver several input events before forwarding a parent's
    // @Prop update. Chain those edits for this frame, but always render code().
    this.getUIContext().postFrameCallback(new OtpEditFrame(() => {
      if (epoch === this.draftEpoch) this.clearDraft()
    }))
  }
  cellID(index: number): string { return this.fieldID + '-' + String(index) }
  activeIndex(): number {
    return this.focused < 0
      ? Math.min(this.code().length, this.length - 1) : this.focused
  }
  pointFocus(index: number): void {
    this.logicalIndex = Math.max(0,
      Math.min(index, this.editCode().length, this.length - 1))
    this.focusTarget = this.logicalIndex
    this.focusPending = false
    if (this.draftValue !== undefined) {
      this.draftTarget = this.logicalIndex
      this.pendingTarget = this.logicalIndex
    }
  }
  move(index: number): void {
    if (this.draftValue === undefined) this.focus(index)
    else {
      this.draftTarget = Math.max(0,
        Math.min(index, this.editCode().length, this.length - 1))
      this.pendingTarget = this.draftTarget
    }
  }
  focus(index: number): void {
    if (this.disabled || this.length < 1) return
    const target = Math.max(0,
      Math.min(index, this.code().length, this.length - 1))
    this.logicalIndex = target
    this.focusTarget = target
    this.focusPending = this.focused !== target
    this.getUIContext().getFocusController().requestFocus(this.cellID(target))
    this.controllers[target]?.setTextSelection(
      0, this.code().charAt(target).length, { menuPolicy: MenuPolicy.HIDE }
    )
  }
  valueChanged(): void {
    if (this.pendingValue === undefined) return
    if (this.code() !== this.pendingValue) {
      if (this.draftHistory.includes(this.code())) return
      this.pendingValue = undefined
      this.clearDraft()
      return
    }
    const complete = this.pendingComplete
    const target = this.pendingTarget
    const expected = this.pendingValue
    this.pendingValue = undefined
    this.clearDraft()
    clearTimeout(this.focusTimer)
    // Move focus in the accepted edit's turn. Deferring it can deliver the
    // next keyboard character to the previous native editor.
    this.focus(target)
    this.focusTimer = setTimeout(() => {
      if (this.code() !== expected || this.disabled) return
      if (complete) {
        this.onValueComplete(expected)
        if (this.autoSubmit) this.form.submit()
      }
    }, 0)
  }
  update(value: string, target: number, complete: boolean = false): void {
    if (value === this.editCode()) {
      if (this.draftValue !== undefined) {
        this.pendingComplete = this.pendingComplete || complete
        return
      }
      if (complete) {
        this.onValueComplete(value)
        if (this.autoSubmit) this.form.submit()
      }
      return
    }
    this.pendingValue = value
    this.pendingTarget = target
    this.pendingComplete = complete
    this.invalid = false
    this.queueDraft(value, target)
    if (this.value === undefined) this.localValue = value
    this.onValueChange(value)
  }
  insert(sourceIndex: number, raw: string, paste: boolean = false): void {
    if (this.disabled || this.readOnly) return
    // IME events can already be queued for the previous editor when native
    // focus moves. Apply them at the accepted edit's logical cursor.
    const index = this.draftValue !== undefined ? this.draftTarget
      : this.logicalIndex < 0 ? sourceIndex : this.logicalIndex
    const digits = normalizeOtp(raw, this.length)
    if (digits !== raw.replace(/\s/g, '')) this.onValueInvalid(raw)
    if (!digits) return
    const before = this.editCode()
    const next = replaceOtp(before, index, digits, this.length)
    if (!paste && digits.length === 1 && next === before) {
      this.move(index + 1)
      return
    }
    this.update(next, index + digits.length,
      next.length === this.length && (before.length !== this.length || paste))
  }
  erase(sourceIndex: number, backward: boolean): void {
    if (this.disabled || this.readOnly) return
    const index = this.draftValue !== undefined ? this.draftTarget
      : this.logicalIndex < 0 ? sourceIndex : this.logicalIndex
    const target = backward ? Math.max(0, index - 1) : index
    const remove = backward && !this.editCode().charAt(index) ? target : index
    const next = removeOtp(this.editCode(), remove)
    if (next === this.editCode()) this.move(target)
    else this.update(next, target)
  }
  handleKey(sourceIndex: number, event: KeyEvent): boolean {
    if (event.type !== KeyType.Down || this.disabled) return false
    const index = this.draftValue !== undefined ? this.draftTarget
      : this.logicalIndex < 0 ? sourceIndex : this.logicalIndex
    const boundary = ((event.getModifierKeyState?.(['Ctrl']) ?? false) ||
      event.metaKey !== 0) && !(event.getModifierKeyState?.(['Alt']) ?? false)
    let target = -1
    if (event.keyCode === 2014) target = boundary ? 0 : Math.max(0, index - 1)
    if (event.keyCode === 2015) {
      target = boundary ? this.editCode().length : index + 1
    }
    if (event.keyCode === 2081 || event.keyCode === 2012) target = 0
    if (event.keyCode === 2082 || event.keyCode === 2013) {
      target = this.editCode().length
    }
    if (target >= 0) {
      this.getUIContext().getFocusController().activate(true)
      this.move(target)
    } else if (!this.readOnly && event.keyCode === 2055) {
      if (boundary) this.update('', 0)
      else this.erase(index, true)
    } else if (!this.readOnly && event.keyCode === 2071) {
      this.erase(index, false)
    } else return false
    event.stopPropagation()
    return true
  }
  ink(): string {
    return this.disabled
      ? this.tokens.paint('normal', 'content', 'disabled')
      : this.tokens.color(this.invalid ? this.tintName() + '.800' : 'content')
  }
  tintName(): DazzTint { return this.state === 'error' ? 'danger' : this.tint }
  cellBackground(index: number): string {
    return this.disabled
      ? this.tokens.paint('normal', 'surface', 'disabled')
      : this.invalid ? this.tokens.color(this.tintName() + '.50')
      : this.focused === index || !!this.code().charAt(index)
        ? this.tokens.color('surface') : this.tokens.color('surface-muted')
  }
  edge(index: number): string {
    return this.tokens.color(
      this.invalid || (this.focused === index && !this.code().charAt(index))
        ? this.tintName() + '.600' : 'border-strong'
    )
  }
  build() {
    Row({ space: this.tokens.metric('spacing.2') }) {
      ForEach(this.indices(), (index: number) => {
        Stack() {
          TextInput({
            text: this.code().charAt(index),
            controller: this.controllers[index],
          })
            .id(this.cellID(index))
            .type(InputType.ONE_TIME_CODE).enableAutoFill(index === 0)
            .enablePreviewText(false).maxLength(this.length)
            .width('100%').height('100%').padding(0).margin(0)
            .fontFamily('monospace').fontWeight(this.tokens.fieldWeight())
            .fontSize(this.tokens.fontSize(
              this.tokens.fieldLabel(this.controlSize).size
            ))
            .textAlign(TextAlign.Center).fontColor(this.ink())
            .caretColor(this.tokens.color(this.tintName() + '.600'))
            .backgroundColor(Color.Transparent).borderWidth(0)
            .borderRadius(this.tokens.metric('control.field.radius'))
            .enabled(!this.disabled)
            .tabIndex(index === this.activeIndex() ? 0 : -1)
            .accessibilityText(this.accessibilityLabel + ', ' +
              String(index + 1) + ' of ' + String(this.length))
            .onFocus(() => {
              if (this.focusPending && this.focusTarget !== index) {
                this.focus(this.focusTarget)
                return
              }
              if (index > this.code().length) {
                this.focus(this.code().length)
                return
              }
              this.focused = index
              this.logicalIndex = index
              this.focusTarget = index
              this.focusPending = false
              this.controllers[index]?.setTextSelection(
                0, this.code().charAt(index).length,
                { menuPolicy: MenuPolicy.HIDE }
              )
            })
            .onTouch((event: TouchEvent) => {
              if (event.type === TouchType.Down) this.pointFocus(index)
            })
            .onMouse((event: MouseEvent) => {
              if (event.action === MouseAction.Press) this.pointFocus(index)
            })
            .onBlur(() => { if (this.focused === index) this.focused = -1 })
            .onWillInsert((value: InsertValue): boolean => {
              this.insert(index, value.insertValue)
              return false
            })
            .onWillDelete((value: DeleteValue): boolean => {
              this.erase(
                index, value.direction === TextDeleteDirection.BACKWARD
              )
              return false
            })
            .onPaste((value: string, event: PasteEvent) => {
              event.preventDefault?.()
              this.insert(index, value, true)
            })
            .onKeyEvent((event: KeyEvent): boolean =>
              this.handleKey(index, event))
            .onSubmit(() => { if (this.form.active) this.form.submit() })
        }.id(this.cellID(index) + '-box')
          .width(this.tokens.fieldSize(this.controlSize))
          .height(this.tokens.fieldSize(this.controlSize))
          .padding(0).margin({
            left: this.groupAfter > 0 && index > 0 &&
              index % this.groupAfter === 0 && index !== this.length - 1
              ? this.tokens.metric('spacing.3') : 0,
          })
          .backgroundColor(this.cellBackground(index))
          .border({
            width: this.tokens.metric('stroke.hairline'),
            color: this.edge(index),
            radius: this.tokens.metric('control.field.radius'),
          })
          .outline({
            width: this.focused === index
              ? this.tokens.metric('focus.width') : 0,
            color: this.tokens.focusGlow(this.tintName()),
            radius: this.tokens.metric('control.field.radius'),
          })
          .enabled(!this.disabled).accessibilityLevel('no')
          .onClick(() => this.focus(index))
      }, (index: number) => String(index))
    }.width(LayoutPolicy.wrapContent)
  }
}
