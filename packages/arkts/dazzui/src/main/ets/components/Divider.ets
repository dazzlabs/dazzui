import { DazzTokens } from '../foundation/Tokens'

@Component
export struct DazzDivider {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop orientation: string = 'horizontal'
  @Prop tone: string = 'normal'
  build() {
    Divider()
      .vertical(this.orientation === 'vertical')
      .strokeWidth(this.tokens.metric('stroke.hairline'))
      .color(
        this.tokens.color(this.tone === 'strong' ? 'border-strong' : 'border')
      )
  }
}
