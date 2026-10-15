import { DazzTokens } from '../foundation/Tokens'

@Component
export struct DazzKeyCap {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop label: string = ''
  @Prop controlSize: string = 'medium'
  @Prop variant: string = 'hint'
  build() {
    Text(this.label)
      .fontSize(
        this.tokens.type(this.controlSize === 'large' ? 'labelMedium' : 'labelSmall')
          .size
      )
      .fontWeight(
        this.tokens.type(this.variant === 'key' ? 'labelStrong' : 'labelMedium')
          .weight
      )
      .fontColor(
        this.tokens.color(
          this.variant === 'key'
            ? 'content'
            : this.variant === 'strong'
              ? 'content-subtle'
              : 'content-faint'
        )
      )
      .backgroundColor(
        this.variant === 'key'
          ? this.tokens.color('surface-inset')
          : Color.Transparent
      )
      .padding({
        left: this.variant === 'key' ? this.tokens.metric('spacing.2.5') : 0,
        right: this.variant === 'key' ? this.tokens.metric('spacing.2.5') : 0,
      })
      .height(
        this.variant === 'key'
          ? this.tokens.metric('spacing.5') + this.tokens.metric('spacing.0.5')
          : 'auto'
      )
      .borderRadius(this.tokens.metric('radius.tiny'))
  }
}
