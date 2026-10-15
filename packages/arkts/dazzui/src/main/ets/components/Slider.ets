import { DazzTokens } from '../foundation/Tokens'
import { DazzTint } from '../foundation/TokenTypes'

@Component
export struct DazzSlider {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop value: number | undefined = undefined
  @Prop defaultValue: number = 0
  @Prop min: number = 0
  @Prop max: number = 100
  @Prop step: number = 1
  @Prop controlSize: string = 'medium'
  @Prop tint: DazzTint = 'primary'
  @Prop disabled: boolean = false
  @Prop showValue: boolean = false
  @Prop accessibilityLabel: string = ''
  @State localValue: number = 0
  onValueChange: (value: number) => void = () => {}
  aboutToAppear(): void {
    this.localValue = this.defaultValue
  }
  build() {
    Row({ space: this.tokens.metric('spacing.3') }) {
      Slider({
        value: this.value ?? this.localValue,
        min: this.min,
        max: this.max,
        step: this.step,
        style: SliderStyle.OutSet,
      })
        .trackThickness(this.tokens.metric('slider.' + this.controlSize + '.track'))
        .blockSize({
          width: this.tokens.metric('slider.' + this.controlSize + '.thumb'),
          height: this.tokens.metric('slider.' + this.controlSize + '.thumb'),
        })
        .selectedColor(
          this.tokens.paint(
            'filled',
            'surface',
            this.disabled ? 'disabled' : 'normal',
            this.tint
          )
        )
        .trackColor(this.tokens.color('surface-sunken'))
        .blockColor(this.tokens.color('surface-raised'))
        .layoutWeight(1)
        .enabled(!this.disabled)
        .accessibilityText(this.accessibilityLabel)
        .onChange((value: number) => {
          if (this.value === undefined) this.localValue = value
          this.onValueChange(value)
        })
      if (this.showValue) {
        Text(String(this.value ?? this.localValue))
          .fontSize(this.tokens.type('labelSmall').size)
          .fontColor(this.tokens.color('content-muted'))
      }
    }
      .width('100%')
  }
}
