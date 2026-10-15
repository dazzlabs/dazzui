import { DazzTokens } from '../foundation/Tokens'
import { DazzTint, DazzSize, DazzVariant } from '../foundation/TokenTypes'

@Component
export struct DazzBadge {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop label: string = ''
  @Prop variant: string = 'tinted'
  @Prop tint: DazzTint = 'primary'
  @Prop controlSize: DazzSize = 'medium'
  build() {
    Text(this.label)
      .fontSize(
        this.tokens.type(this.controlSize === 'large' ? 'labelMedium' : 'labelSmall')
          .size
      )
      .fontWeight(this.tokens.type('labelMedium').weight)
      .fontColor(
        this.variant === 'raised'
          ? this.tokens.color('content-muted')
          : this.tokens.paint(
              this.variant as DazzVariant,
              'content',
              'normal',
              this.tint
            )
      )
      .backgroundColor(
        this.variant === 'raised'
          ? this.tokens.color('surface')
          : this.tokens.paint(
              this.variant as DazzVariant,
              'surface',
              'normal',
              this.tint
            )
      )
      .padding({
        left: this.tokens.metric(
          this.controlSize === 'small' ? 'spacing.1.5' : 'spacing.2'
        ),
        right: this.tokens.metric(
          this.controlSize === 'small' ? 'spacing.1.5' : 'spacing.2'
        ),
        top: this.tokens.metric(
          this.controlSize === 'large' ? 'spacing.1' : 'spacing.0.5'
        ),
        bottom: this.tokens.metric(
          this.controlSize === 'large' ? 'spacing.1' : 'spacing.0.5'
        ),
      })
      .border({
        width: this.tokens.metric('stroke.hairline'),
        color:
          this.variant === 'raised'
            ? Color.Transparent
            : this.tokens.paint(
                this.variant as DazzVariant,
                'border',
                'normal',
                this.tint
              ),
        radius: this.tokens.metric('radius.tiny'),
      })
  }
}
