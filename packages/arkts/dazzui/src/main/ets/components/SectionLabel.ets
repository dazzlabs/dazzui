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
      .fontSize(this.tokens.type('captionSmall').size)
      .fontWeight(this.tokens.type('labelSmall').weight)
      .fontColor(
        this.faint
          ? this.tokens.color('content-faint')
          : this.tint === 'neutral'
            ? this.tokens.color('content-muted')
            : this.tokens.paint('plain', 'content', 'normal', this.tint)
      )
  }
}
