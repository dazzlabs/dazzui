import { DazzTokens } from '../foundation/Tokens'
import { DazzTint, DazzSize, DazzVariant } from '../foundation/TokenTypes'

@Component
export struct DazzBadge {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop label: string = ''
  @Prop variant: string = 'tinted'
  @Prop tint: DazzTint = 'primary'
  @Prop controlSize: DazzSize = 'medium'
  labelSize(): number { return this.tokens.type(this.controlSize === 'large' ? 'labelMedium' : 'labelSmall').size }
  blockPadding(): number { return this.tokens.metric(this.controlSize === 'large' ? 'spacing.1' : 'spacing.0.5') }
  edge(): number { return this.tokens.metric('spacing.px') }
  build() {
    Text(this.label)
      .height(this.labelSize() * 1.4 + 2 * (this.blockPadding() + this.edge()))
      .fontSize(this.tokens.fontSize(
        this.tokens.type(this.controlSize === 'large' ? 'labelMedium' : 'labelSmall')
          .size
      ))
      .fontWeight(this.tokens.type('labelMedium').weight)
      .lineHeight(this.tokens.fontSize(this.tokens.type(
        this.controlSize === 'large' ? 'labelMedium' : 'labelSmall').size * 1.4))
      .maxLines(1)
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
        left: this.edge() + this.tokens.metric(
          this.controlSize === 'small' ? 'spacing.1.5' : 'spacing.2'
        ),
        right: this.edge() + this.tokens.metric(
          this.controlSize === 'small' ? 'spacing.1.5' : 'spacing.2'
        ),
        top: this.blockPadding() + this.edge(),
        bottom: this.blockPadding() + this.edge(),
      })
      .border({
        width: this.edge(),
        color:
          this.variant === 'outlined'
            ? this.tokens.paint('outlined', 'border', 'normal', this.tint)
            : Color.Transparent,
        radius: this.tokens.metric('radius.tiny'),
      })
  }
}
