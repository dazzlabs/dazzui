import { DazzTokens } from '../foundation/Tokens'

@Component
export struct DazzFormField {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop label: string = ''
  @Prop hint: string = ''
  @Prop state: string = 'normal'
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Column({ space: this.tokens.metric('spacing.2') }) {
      if (this.label) {
        Text(this.label)
          .fontSize(this.tokens.fontSize(this.tokens.type('labelSmall').size))
          .fontWeight(this.tokens.type('labelSmall').weight)
          .fontColor(this.tokens.color('content'))
      }
      this.content()
      if (this.hint) {
        Text(this.hint)
          .fontSize(this.tokens.fontSize(this.tokens.type('captionMedium').size))
          .fontColor(
            this.tokens.color(
              this.state === 'error' ? 'danger.700' : 'content-muted'
            )
          )
      }
    }
      .alignItems(HorizontalAlign.Start)
      .width('100%')
  }
}

@Component
export struct DazzFieldset {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop legend: string = ''
  @Prop disabled: boolean = false
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Column({ space: this.tokens.metric('spacing.4') }) {
      if (this.legend) {
        Text(this.legend)
          .fontSize(this.tokens.fontSize(this.tokens.type('labelStrong').size))
          .fontWeight(this.tokens.type('labelStrong').weight)
          .fontColor(this.tokens.color('content'))
      }
      this.content()
    }
      .alignItems(HorizontalAlign.Start)
      .enabled(!this.disabled)
  }
}
