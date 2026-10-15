import { DazzTokens } from '../foundation/Tokens'

@Component
export struct DazzActionBar {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop contentAlign: string = 'end'
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Row({ space: this.tokens.metric('spacing.2') }) {
      this.content()
    }
      .justifyContent(
        this.contentAlign === 'start'
          ? FlexAlign.Start
          : this.contentAlign === 'between'
            ? FlexAlign.SpaceBetween
            : FlexAlign.End
      )
      .width('100%')
  }
}
