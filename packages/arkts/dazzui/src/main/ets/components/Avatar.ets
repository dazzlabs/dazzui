import { DazzTokens } from '../foundation/Tokens'
import { DazzTint } from '../foundation/TokenTypes'

@Component
export struct DazzAvatar {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop name: string = ''
  @Prop fallback: string = ''
  @Prop controlSize: string = 'medium'
  @Prop shape: string = 'circle'
  @Prop tint: DazzTint = 'primary'
  initials(): string {
    return (
      this.fallback ||
      this.name
        .trim()
        .split(/\s+/)
        .map((word: string) => word.slice(0, 1))
        .slice(0, 2)
        .join('')
        .toUpperCase()
    )
  }
  length(): number {
    return this.tokens.metric(
      this.controlSize === 'small'
        ? 'spacing.5'
        : this.controlSize === 'large'
          ? 'spacing.9'
          : 'spacing.7'
    )
  }
  build() {
    Text(this.initials())
      .fontSize(
        this.tokens.type(
          this.controlSize === 'small'
            ? 'captionSmall'
            : this.controlSize === 'large'
              ? 'labelLarge'
              : 'labelMedium'
        ).size
      )
      .fontWeight(this.tokens.type('labelStrong').weight)
      .fontColor(this.tokens.paint('tinted', 'content', 'normal', this.tint))
      .backgroundColor(
        this.tokens.paint('tinted', 'surface', 'normal', this.tint)
      )
      .textAlign(TextAlign.Center)
      .width(this.length())
      .height(this.length())
      .borderRadius(
        this.tokens.metric(
          this.shape === 'square' ? 'radius.medium' : 'radius.full'
        )
      )
      .accessibilityText(this.name)
  }
}
