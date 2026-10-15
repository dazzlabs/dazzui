import { DazzTokens } from '../foundation/Tokens'
import { DazzTint } from '../foundation/TokenTypes'

@Component
export struct DazzProgress {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop value: number = 0
  @Prop max: number = 100
  @Prop controlSize: string = 'medium'
  @Prop tint: DazzTint = 'primary'
  @Prop accessibilityLabel: string = ''
  build() {
    Progress({
      value: Math.max(0, Math.min(this.max, this.value)),
      total: this.max,
      type: ProgressType.Linear,
    })
      .color(this.tokens.paint('filled', 'surface', 'normal', this.tint))
      .backgroundColor(this.tokens.color('surface-sunken'))
      .style({
        strokeWidth: this.tokens.metric(
          this.controlSize === 'small'
            ? 'spacing.1'
            : this.controlSize === 'large'
              ? 'spacing.2'
              : 'spacing.1.5'
        ),
      })
      .width('100%')
      .accessibilityText(this.accessibilityLabel)
  }
}

@Component
export struct DazzSpinner {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop controlSize: string = 'medium'
  @Prop tint: DazzTint = 'primary'
  @Prop onAccent: boolean = false
  build() {
    LoadingProgress()
      .width(this.tokens.metric('icon.' + this.controlSize))
      .height(this.tokens.metric('icon.' + this.controlSize))
      .color(
        this.onAccent
          ? this.tokens.color('on-accent')
          : this.tokens.paint('filled', 'surface', 'normal', this.tint)
      )
      .accessibilityText('Loading')
  }
}

@Component
export struct DazzMeter {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop value: number = 0
  @Prop min: number = 0
  @Prop max: number = 100
  @Prop label: string = ''
  @Prop showValue: boolean = false
  @Prop controlSize: string = 'medium'
  @Prop tint: DazzTint = 'primary'
  build() {
    Column({ space: this.tokens.metric('spacing.2') }) {
      Row() {
        Text(this.label)
          .fontSize(this.tokens.type('labelSmall').size)
          .fontColor(this.tokens.color('content-muted'))
        Blank()
        if (this.showValue) {
          Text(String(this.value))
            .fontSize(this.tokens.type('labelSmall').size)
            .fontColor(this.tokens.color('content-muted'))
        }
      }
        .width('100%')
      DazzProgress({
        value: this.value - this.min,
        max: this.max - this.min,
        controlSize: this.controlSize,
        tint: this.tint,
        accessibilityLabel: this.label,
      })
    }
      .width('100%')
  }
}
