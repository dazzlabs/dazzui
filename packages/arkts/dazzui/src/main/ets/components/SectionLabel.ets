import { DazzTokens } from '../foundation/Tokens'
import { DazzTint } from '../foundation/TokenTypes'

@Component
export struct DazzSectionLabel {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop label: string = ''
  @Prop faint: boolean = false
  @Prop tint: DazzTint = 'neutral'
  build() {
    Text(this.label)
      .fontSize(this.tokens.fontSize(this.tokens.type('labelSmall').size))
      .fontWeight(this.tokens.type('labelSmall').weight)
      .lineHeight(this.tokens.fontSize(this.tokens.type('labelSmall').leading))
      .fontColor(
        this.faint
          ? this.tokens.color('content-faint')
          : this.tint === 'neutral'
            ? this.tokens.color('content-subtle')
            : this.tokens.paint('plain', 'content', 'normal', this.tint)
      )
  }
}
