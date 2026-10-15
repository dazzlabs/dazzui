import { DazzTokens } from '../foundation/Tokens'

@Component
export struct DazzCode {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop code: string = ''
  @Prop block: boolean = false
  build() {
    Text(this.code)
      .fontFamily('monospace')
      .fontSize(this.tokens.type('bodySmall').size)
      .fontColor(this.tokens.color('content'))
      .backgroundColor(this.tokens.color('surface-inset'))
      .padding(this.tokens.metric(this.block ? 'spacing.4' : 'spacing.1'))
      .borderRadius(this.tokens.metric('radius.small'))
      .copyOption(CopyOptions.LocalDevice)
  }
}
