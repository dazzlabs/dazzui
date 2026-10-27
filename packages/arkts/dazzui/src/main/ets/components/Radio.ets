import { DazzTokens } from '../foundation/Tokens'
import { DazzTint, DazzItem } from '../foundation/TokenTypes'

let radioSequence: number = 0
@Component
export struct DazzRadioGroup {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop options: DazzItem[] = []
  @Prop value: string | undefined = undefined
  @Prop defaultValue: string = ''
  @Prop controlSize: string = 'medium'
  @Prop tint: DazzTint = 'primary'
  @Prop disabled: boolean = false
  @Prop accessibilityLabel: string = ''
  @State localValue: string = ''
  private group: string = 'dazz-radio-' + ++radioSequence
  onValueChange: (value: string) => void = () => {}
  aboutToAppear(): void {
    this.localValue = this.defaultValue
  }
  choose(item: DazzItem): void {
    if (this.disabled || item.disabled) return
    if (this.value === undefined) this.localValue = item.value
    this.onValueChange(item.value)
  }
  build() {
    Column({ space: this.tokens.metric('spacing.2.5') }) {
      ForEach(
        this.options,
        (item: DazzItem) => {
          Row({ space: this.tokens.metric('spacing.2.5') }) {
            Radio({ value: item.value, group: this.group })
              .checked((this.value ?? this.localValue) === item.value)
              .width(this.tokens.metric('radio.' + this.controlSize + '.box'))
              .height(this.tokens.metric('radio.' + this.controlSize + '.box'))
              .padding(0)
              .margin(0)
              .radioStyle({
                checkedBackgroundColor: this.tokens.paint(
                  'filled',
                  'surface',
                  'normal',
                  this.tint
                ),
                uncheckedBorderColor: this.tokens.color('border-muted'),
                indicatorColor: this.tokens.color('on-accent'),
              })
              .enabled(!this.disabled && !item.disabled)
              .accessibilityText(item.label)
              .onChange((checked: boolean) => {
                if (checked) this.choose(item)
              })
            Text(item.label)
              .fontSize(this.tokens.fontSize(this.tokens.type('labelSmall').size))
              .fontWeight(
                this.tokens.type(
                  (this.value ?? this.localValue) === item.value
                    ? 'labelSmall'
                    : 'labelQuiet'
                ).weight
              )
              .fontColor(this.tokens.color('content'))
              .onClick(() => this.choose(item))
          }
        },
        (item: DazzItem) => item.value
      )
    }
      .alignItems(HorizontalAlign.Start)
      .accessibilityText(this.accessibilityLabel)
  }
}
