import { rememberFocus, forgetFocus } from '../foundation/FocusData'
import { DazzFormContext, fieldError } from '../foundation/FormData'
import { DazzEditingContext } from '../foundation/Editing'
import { DazzTokens } from '../foundation/Tokens'
import { DazzSize, DazzTint } from '../foundation/TokenTypes'
import { DazzIcon } from './Icon'
import { ColorMetrics, LengthMetrics } from '@kit.ArkUI'

let fieldSequence: number = 0

@Component
export struct DazzTextField {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.editing') editing: DazzEditingContext = new DazzEditingContext()
  @Consume('dazzui.form') form: DazzFormContext = new DazzFormContext()
  @Prop value: string | undefined = undefined
  @Prop defaultValue: string = ''
  @Prop placeholder: string = ''
  @Prop accessibilityLabel: string = ''
  @Prop controlSize: DazzSize = 'medium'
  @Prop disabled: boolean = false
  @Prop readOnly: boolean = false
  @Prop required: boolean = false
  @Prop multiline: boolean = false
  @Prop rows: number = 4
  @Prop state: string = 'normal'
  @Prop inputType: InputType = InputType.Normal
  @Prop mono: boolean = false
  @State localValue: string = ''
  @State focused: boolean = false
  @State validationError: string = ''
  private fieldID: string = 'dazzui-field-' + String(++fieldSequence)
  onValueChange: (value: string) => void = () => {}
  onCommit: (value: string) => void = () => {}
  aboutToAppear(): void {
    this.localValue = this.defaultValue
    if (this.form.active) this.form.register(this.fieldID, {
      validate: (focus: boolean): boolean => this.validate(focus),
      reset: () => { this.localValue = this.defaultValue; this.validationError = ''; this.onValueChange(this.defaultValue) }
    })
  }
  aboutToDisappear(): void { this.form.unregister(this.fieldID); this.editing.clear(this.fieldID); forgetFocus(this.getUIContext().getWindowName() ?? '', this.fieldID) }
  validate(focus: boolean): boolean {
    this.validationError = this.disabled || this.readOnly ? '' : fieldError(this.text(), this.required, this.inputType === InputType.Email)
    if (this.validationError && focus) this.getUIContext().getFocusController().requestFocus(this.fieldID)
    return !this.validationError
  }
  text(): string {
    return this.value ?? this.localValue
  }
  change(value: string): void {
    if (this.disabled || this.readOnly) return
    if (this.value === undefined) this.localValue = value
    this.validationError = ''
    this.onValueChange(value)
  }
  fieldBackground(): string {
    return this.disabled
      ? this.tokens.color('surface-sunken')
      : (this.state === 'error' || !!this.validationError)
        ? this.tokens.color('danger.50')
        : this.focused
          ? this.tokens.color('surface')
          : this.tokens.color('surface-muted')
  }
  edge(): string {
    return (this.state === 'error' || !!this.validationError)
      ? this.tokens.color('danger.600')
      : this.focused
        ? this.tokens.color('primary.600')
        : this.tokens.color('border-strong')
  }

  build() {
    Column() {
    if (this.readOnly) {
      Text(this.text() || this.placeholder)
        .fontFamily(this.mono ? 'monospace' : undefined)
        .fontWeight(this.tokens.fieldWeight())
        .fontSize(this.tokens.fontSize(this.tokens.fieldLabel(this.controlSize).size))
        .fontColor(
          this.tokens.color(this.disabled ? 'content-faint' : 'content')
        )
        .copyOption(CopyOptions.LocalDevice)
        .width('100%')
        .height(
          this.multiline
            ? this.tokens.fieldLineHeight() * this.rows +
                this.tokens.metric('spacing.5')
            : this.tokens.fieldSize(this.controlSize)
        )
        .padding({
          left: this.tokens.fieldPadding(),
          right: this.tokens.fieldPadding(),
        })
        .backgroundColor(this.fieldBackground())
        .border({
          width: this.tokens.metric('stroke.hairline'),
          color: this.edge(),
          radius: this.tokens.metric('control.field.radius'),
        })
        .accessibilityText(this.accessibilityLabel)
    } else if (this.multiline) {
      TextArea({ text: this.text(), placeholder: this.placeholder })
        .id(this.fieldID).accessibilityDescription(this.required ? 'Required' : '')
        .fontSize(this.tokens.fontSize(this.tokens.fieldLabel(this.controlSize).size))
        .fontWeight(this.tokens.fieldWeight())
        .fontFamily(this.mono ? 'monospace' : undefined)
        .fontColor(
          this.tokens.color(this.disabled ? 'content-faint' : 'content')
        )
        .placeholderColor(this.tokens.color('content-faint'))
        .placeholderFont({
          size: this.tokens.fontSize(this.tokens.fieldLabel(this.controlSize).size),
          weight: this.tokens.fieldWeight(),
          family: this.mono ? 'monospace' : undefined,
        })
        .caretColor(this.tokens.color('primary.600'))
        .height(
          this.tokens.fieldLineHeight() * this.rows +
            this.tokens.metric('spacing.5')
        )
        .padding(this.tokens.fieldPadding())
        .backgroundColor(this.fieldBackground())
        .border({
          width: this.tokens.metric('stroke.hairline'),
          color: this.edge(),
          radius: this.tokens.metric('radius.large'),
        })
        .enabled(!this.disabled)
        .accessibilityText(this.accessibilityLabel)
        .onFocus(() => {
          this.focused = true
          rememberFocus(this.getUIContext().getWindowName() ?? '', this.fieldID)
        })
        .onBlur(() => {
          this.focused = false
          forgetFocus(this.getUIContext().getWindowName() ?? '', this.fieldID)
          this.editing.clear(this.fieldID)
          this.onCommit(this.text())
          if (this.form.attempted) this.validate(false)
        })
        .onChange((value: string, preview?: PreviewText) => { this.editing.preview(this.fieldID, preview?.value ?? ''); this.change(value) })
    } else {
      TextInput({ text: this.text(), placeholder: this.placeholder })
        .id(this.fieldID).accessibilityDescription(this.required ? 'Required' : '')
        .type(this.inputType)
        .fontSize(this.tokens.fontSize(this.tokens.fieldLabel(this.controlSize).size))
        .fontWeight(this.tokens.fieldWeight())
        .fontFamily(this.mono ? 'monospace' : undefined)
        .fontColor(
          this.tokens.color(this.disabled ? 'content-faint' : 'content')
        )
        .placeholderColor(this.tokens.color('content-faint'))
        .placeholderFont({
          size: this.tokens.fontSize(this.tokens.fieldLabel(this.controlSize).size),
          weight: this.tokens.fieldWeight(),
          family: this.mono ? 'monospace' : undefined,
        })
        .caretColor(this.tokens.color('primary.600'))
        .height(this.tokens.fieldSize(this.controlSize))
        .padding({
          left: this.tokens.fieldPadding(),
          right: this.tokens.fieldPadding(),
          top: 0,
          bottom: 0,
        })
        .backgroundColor(this.fieldBackground())
        .border({
          width: this.tokens.metric('stroke.hairline'),
          color: this.edge(),
          radius: this.tokens.metric('control.field.radius'),
        })
        .enabled(!this.disabled)
        .accessibilityText(this.accessibilityLabel)
        .onFocus(() => {
          this.focused = true
          rememberFocus(this.getUIContext().getWindowName() ?? '', this.fieldID)
        })
        .onBlur(() => {
          this.focused = false
          forgetFocus(this.getUIContext().getWindowName() ?? '', this.fieldID)
          this.editing.clear(this.fieldID)
          this.onCommit(this.text())
          if (this.form.attempted) this.validate(false)
        })
        .onSubmit(() => { this.onCommit(this.text()); if (this.form.active) this.form.submit() })
        .onChange((value: string, preview?: PreviewText) => { this.editing.preview(this.fieldID, preview?.value ?? ''); this.change(value) })
    }
      if (this.validationError) {
        Text(this.validationError).fontSize(this.tokens.fontSize(this.tokens.type('captionSmall').size))
          .fontColor(this.tokens.color('danger.700')).margin({ top: this.tokens.metric('spacing.1') })
      }
    }.width('100%').alignItems(HorizontalAlign.Start)
  }
}

@Component
export struct DazzSearchField {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.editing') editing: DazzEditingContext = new DazzEditingContext()
  @Consume('dazzui.form') form: DazzFormContext = new DazzFormContext()
  @Prop value: string | undefined = undefined
  @Prop placeholder: string = 'Search'
  @Prop accessibilityLabel: string = 'Search'
  @Prop controlSize: DazzSize = 'medium'
  @Prop disabled: boolean = false
  @Prop stretch: boolean = false
  @Prop tint: DazzTint = 'primary'
  @Prop clearLabel: string = 'Clear search'
  @State localValue: string = ''
  @State focused: boolean = false
  @State clearHovered: boolean = false
  private inputID: string = 'dazzui-search-' + String(++fieldSequence)
  onValueChange: (value: string) => void = () => {}
  onDismiss: () => void = () => {}
  @BuilderParam hint: () => void = this.empty
  @LocalBuilder empty() {}
  aboutToDisappear(): void { this.editing.clear(this.inputID); forgetFocus(this.getUIContext().getWindowName() ?? '', this.inputID); forgetFocus(this.getUIContext().getWindowName() ?? '', this.inputID + '-clear') }
  text(): string { return this.value ?? this.localValue }
  change(value: string): void { if (this.disabled) return; if (this.value === undefined) this.localValue = value; this.onValueChange(value) }
  naturalWidth(): number {
    return this.tokens.metric('search-field.input-width') + this.tokens.size(this.controlSize, 'icon-size') + this.tokens.size(this.controlSize, 'gap') +
      2 * this.tokens.fieldPadding() + 2 * this.tokens.metric('stroke.hairline') + (this.text() ? this.tokens.metric('icon.small') + this.tokens.size(this.controlSize, 'gap') : 0)
  }
  build() {
    Row({ space: this.tokens.size(this.controlSize, 'gap') }) {
      DazzIcon({ name: 'Search16Regular', iconSize: this.tokens.size(this.controlSize, 'icon-size'), color: this.tokens.color('content-faint') })
      TextInput({ text: this.text(), placeholder: this.placeholder })
        .id(this.inputID).layoutWeight(1).height('100%').padding(0).backgroundColor(Color.Transparent)
        .fontSize(this.tokens.fontSize(this.tokens.fieldLabel(this.controlSize).size)).fontWeight(this.tokens.fieldWeight())
        .fontColor(this.tokens.color('content')).placeholderColor(this.tokens.color('content-faint'))
        .placeholderFont({ size: this.tokens.fontSize(this.tokens.fieldLabel(this.controlSize).size), weight: this.tokens.fieldWeight() })
        .caretColor(this.tokens.color(this.tint + '.600')).enabled(!this.disabled).accessibilityText(this.accessibilityLabel)
        .onFocus(() => { this.focused = true; rememberFocus(this.getUIContext().getWindowName() ?? '', this.inputID) }).onBlur(() => { this.focused = false; this.editing.clear(this.inputID); forgetFocus(this.getUIContext().getWindowName() ?? '', this.inputID) })
        .onChange((value: string, preview?: PreviewText) => { this.editing.preview(this.inputID, preview?.value ?? ''); this.change(value) })
        .onSubmit(() => { if (this.form.active) this.form.submit() })
        .onKeyEvent((event: KeyEvent): boolean => {
          if (event.type !== KeyType.Down || event.keyCode !== 2070) return false
          if (this.text()) this.change(''); else this.onDismiss()
          event.stopPropagation(); return true
        })
      if (this.text()) {
        Button({ type: ButtonType.Normal }) {
          DazzIcon({ name: 'Dismiss12Regular', iconSize: this.tokens.metric('icon.small'), color: this.tokens.color(this.clearHovered ? 'content' : 'content-faint') })
        }.width(this.tokens.metric('icon.small')).height(this.tokens.metric('icon.small')).padding(0)
          .id(this.inputID + '-clear').backgroundColor(Color.Transparent).stateEffect(false).enabled(!this.disabled)
          .accessibilityText(this.clearLabel).onHover((hovered: boolean) => { this.clearHovered = hovered })
          .focusBox({ margin: LengthMetrics.vp(this.tokens.metric('focus.offset')), strokeWidth: LengthMetrics.vp(this.tokens.metric('focus.width')), strokeColor: ColorMetrics.resourceColor(this.tokens.focus()) })
          .onFocus(() => { this.focused = true; rememberFocus(this.getUIContext().getWindowName() ?? '', this.inputID + '-clear') }).onBlur(() => { this.focused = false; forgetFocus(this.getUIContext().getWindowName() ?? '', this.inputID + '-clear') })
          .onClick(() => this.change(''))
      } else { this.hint() }
    }.width(this.stretch ? '100%' : this.naturalWidth()).constraintSize({ maxWidth: '100%' })
      .height(this.tokens.fieldSize(this.controlSize))
      .padding({ left: this.tokens.fieldPadding(), right: this.tokens.fieldPadding() })
      .backgroundColor(this.tokens.color(this.focused ? 'surface' : 'surface-muted'))
      .border({ width: this.tokens.metric('stroke.hairline'), color: this.tokens.color(this.focused ? this.tint + '.600' : 'border-strong'), radius: this.tokens.metric('control.field.radius') })
      .outline({ width: this.focused ? this.tokens.metric('focus.width') : 0, color: this.tokens.focusGlow(this.tint), radius: this.tokens.metric('control.field.radius') })
  }
}
