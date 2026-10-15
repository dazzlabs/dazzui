import { DazzTokens } from '../foundation/Tokens'
import { DazzSize, DazzItem } from '../foundation/TokenTypes'

@Component
export struct DazzSelect {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop options: DazzItem[] = []
  @Prop value: string | undefined = undefined
  @Prop defaultValue: string = ''
  @Prop placeholder: string = 'Choose'
  @Prop controlSize: DazzSize = 'medium'
  @Prop disabled: boolean = false
  @Prop accessibilityLabel: string = ''
  @State localValue: string = ''
  onValueChange: (value: string) => void = () => {}
  aboutToAppear(): void {
    this.localValue = this.defaultValue
  }
  index(): number {
    return this.options.findIndex(
      (option: DazzItem) => option.value === (this.value ?? this.localValue)
    )
  }
  label(): string {
    return this.index() < 0
      ? this.placeholder
      : this.options[this.index()].label
  }
  entries(): SelectOption[] {
    return this.options.map((option: DazzItem): SelectOption => {
      return { value: option.label }
    })
  }
  build() {
    Select(this.entries())
      .selected(this.index())
      .value(this.label())
      .font({
        size: this.tokens.label(this.controlSize).size,
        weight: this.tokens.type('bodySmall').weight,
      })
      .fontColor(this.tokens.color(this.disabled ? 'content-faint' : 'content'))
      .optionFont({ size: this.tokens.label(this.controlSize).size })
      .optionFontColor(this.tokens.color('content'))
      .optionBgColor(this.tokens.color('surface-overlay'))
      .selectedOptionBgColor(this.tokens.paint('tinted', 'surface'))
      .selectedOptionFontColor(this.tokens.paint('tinted', 'content'))
      .height(this.tokens.size(this.controlSize))
      .width('100%')
      .padding({
        left: this.tokens.metric('spacing.3'),
        right: this.tokens.metric('spacing.3'),
      })
      .backgroundColor(
        this.tokens.color(this.disabled ? 'surface-sunken' : 'surface-muted')
      )
      .border({
        width: this.tokens.metric('stroke.hairline'),
        color: this.tokens.color('border-strong'),
        radius: this.tokens.metric('control.field.radius'),
      })
      .enabled(!this.disabled)
      .accessibilityText(this.accessibilityLabel)
      .onSelect((index: number) => {
        if (index < 0 || index >= this.options.length) return
        const next = this.options[index].value
        if (this.value === undefined) this.localValue = next
        this.onValueChange(next)
      })
  }
}
