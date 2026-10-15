import { DazzTokens } from '../foundation/Tokens'
import { DazzSize } from '../foundation/TokenTypes'

@Component
export struct DazzTextField {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop value: string | undefined = undefined
  @Prop defaultValue: string = ''
  @Prop placeholder: string = ''
  @Prop accessibilityLabel: string = ''
  @Prop controlSize: DazzSize = 'medium'
  @Prop disabled: boolean = false
  @Prop readOnly: boolean = false
  @Prop multiline: boolean = false
  @Prop rows: number = 4
  @Prop state: string = 'normal'
  @Prop inputType: InputType = InputType.Normal
  @Prop mono: boolean = false
  @State localValue: string = ''
  @State focused: boolean = false
  onValueChange: (value: string) => void = () => {}
  onCommit: (value: string) => void = () => {}
  aboutToAppear(): void {
    this.localValue = this.defaultValue
  }
  text(): string {
    return this.value ?? this.localValue
  }
  change(value: string): void {
    if (this.disabled || this.readOnly) return
    if (this.value === undefined) this.localValue = value
    this.onValueChange(value)
  }
  fieldBackground(): string {
    return this.disabled
      ? this.tokens.color('surface-sunken')
      : this.state === 'error'
        ? this.tokens.color('danger.50')
        : this.focused
          ? this.tokens.color('surface')
          : this.tokens.color('surface-muted')
  }
  edge(): string {
    return this.state === 'error'
      ? this.tokens.color('danger.600')
      : this.focused
        ? this.tokens.color('primary.600')
        : this.tokens.color('border-strong')
  }

  build() {
    if (this.readOnly) {
      Text(this.text() || this.placeholder)
        .fontSize(this.tokens.label(this.controlSize).size)
        .fontColor(
          this.tokens.color(this.disabled ? 'content-faint' : 'content')
        )
        .copyOption(CopyOptions.LocalDevice)
        .width('100%')
        .height(
          this.multiline
            ? this.tokens.type('bodySmall').leading * this.rows +
                this.tokens.metric('spacing.5')
            : this.tokens.size(this.controlSize)
        )
        .padding({
          left: this.tokens.metric('spacing.3'),
          right: this.tokens.metric('spacing.3'),
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
        .fontSize(this.tokens.label(this.controlSize).size)
        .fontWeight(this.tokens.type('bodySmall').weight)
        .fontFamily(this.mono ? 'monospace' : 'HarmonyOS Sans')
        .fontColor(
          this.tokens.color(this.disabled ? 'content-faint' : 'content')
        )
        .placeholderColor(this.tokens.color('content-faint'))
        .caretColor(this.tokens.color('primary.600'))
        .height(
          this.tokens.type('bodySmall').leading * this.rows +
            this.tokens.metric('spacing.5')
        )
        .padding(this.tokens.metric('spacing.3'))
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
        })
        .onBlur(() => {
          this.focused = false
          this.onCommit(this.text())
        })
        .onChange((value: string) => this.change(value))
    } else {
      TextInput({ text: this.text(), placeholder: this.placeholder })
        .type(this.inputType)
        .fontSize(this.tokens.label(this.controlSize).size)
        .fontWeight(this.tokens.type('bodySmall').weight)
        .fontFamily(this.mono ? 'monospace' : 'HarmonyOS Sans')
        .fontColor(
          this.tokens.color(this.disabled ? 'content-faint' : 'content')
        )
        .placeholderColor(this.tokens.color('content-faint'))
        .caretColor(this.tokens.color('primary.600'))
        .height(this.tokens.size(this.controlSize))
        .padding({
          left: this.tokens.metric('spacing.3'),
          right: this.tokens.metric('spacing.3'),
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
        })
        .onBlur(() => {
          this.focused = false
          this.onCommit(this.text())
        })
        .onChange((value: string) => this.change(value))
    }
  }
}

@Component
export struct DazzSearchField {
  @Prop value: string | undefined = undefined
  @Prop placeholder: string = 'Search'
  @Prop accessibilityLabel: string = 'Search'
  @Prop controlSize: DazzSize = 'medium'
  @Prop disabled: boolean = false
  onValueChange: (value: string) => void = () => {}
  build() {
    DazzTextField({
      value: this.value,
      placeholder: this.placeholder,
      accessibilityLabel: this.accessibilityLabel,
      controlSize: this.controlSize,
      disabled: this.disabled,
      onValueChange: this.onValueChange,
    })
  }
}
