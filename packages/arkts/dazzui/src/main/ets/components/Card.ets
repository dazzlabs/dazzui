import { DazzTokens, withAlpha } from '../foundation/Tokens'
import { DazzTint } from '../foundation/TokenTypes'

@Component
export struct DazzCard {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop variant: string = 'raised'
  @Prop controlSize: string = 'medium'
  @Prop tint: DazzTint = 'primary'
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Column({
      space: this.tokens.metric(
        this.controlSize === 'small'
          ? 'spacing.2'
          : this.controlSize === 'large'
            ? 'spacing.4'
            : 'spacing.3'
      ),
    }) {
      this.content()
    }
      .alignItems(HorizontalAlign.Start)
      .padding({
        top: this.tokens.metric(
          this.controlSize === 'large' ? 'spacing.4' : 'spacing.3'
        ),
        bottom: this.tokens.metric(
          this.controlSize === 'large' ? 'spacing.4' : 'spacing.3'
        ),
        left: this.tokens.metric(
          this.controlSize === 'small'
            ? 'spacing.3'
            : this.controlSize === 'large'
              ? 'spacing.4'
              : 'spacing.3.5'
        ),
        right: this.tokens.metric(
          this.controlSize === 'small'
            ? 'spacing.3'
            : this.controlSize === 'large'
              ? 'spacing.4'
              : 'spacing.3.5'
        ),
      })
      .backgroundColor(
        this.variant === 'tinted'
          ? withAlpha(
              this.tokens.color(this.tint + '.600'),
              this.tokens.metric('wash.surface')
            )
          : this.variant === 'outlined'
            ? Color.Transparent
            : this.tokens.color(
                this.variant === 'sunken' ? 'surface-muted' : 'surface'
              )
      )
      .border({
        width: this.tokens.metric('stroke.hairline'),
        color:
          this.variant === 'tinted'
            ? withAlpha(
                this.tokens.color(this.tint + '.600'),
                this.tokens.metric('wash.edge')
              )
            : this.tokens.color('border'),
        radius: this.tokens.metric('radius.large'),
      })
  }
}
