import { DazzTokens } from '../foundation/Tokens'

@Component
export struct DazzEmptyState {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop title: string = ''
  @BuilderParam content: () => void = this.empty
  @BuilderParam action: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Column({ space: this.tokens.metric('spacing.4') }) {
      Text(this.title)
        .fontSize(this.tokens.type('labelStrong').size)
        .fontWeight(this.tokens.type('labelStrong').weight)
        .fontColor(this.tokens.color('content'))
      this.content()
      this.action()
    }
      .padding(this.tokens.metric('spacing.8'))
      .width('100%')
  }
}
