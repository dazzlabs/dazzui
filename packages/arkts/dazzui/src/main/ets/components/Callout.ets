import { DazzTokens } from '../foundation/Tokens'
import { DazzTint } from '../foundation/TokenTypes'

@Component
export struct DazzCallout {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop title: string = ''
  @Prop tint: DazzTint = 'info'
  @BuilderParam content: () => void = this.empty
  @BuilderParam action: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Column({ space: this.tokens.metric('spacing.2') }) {
      if (this.title) {
        Text(this.title)
          .fontSize(this.tokens.type('labelSmall').size)
          .fontWeight(this.tokens.type('labelStrong').weight)
          .fontColor(
            this.tokens.paint('tinted', 'content', 'normal', this.tint)
          )
      }
      this.content()
      this.action()
    }
      .alignItems(HorizontalAlign.Start)
      .padding(this.tokens.metric('spacing.4'))
      .backgroundColor(
        this.tokens.paint('tinted', 'surface', 'normal', this.tint)
      )
      .border({
        width: this.tokens.metric('stroke.hairline'),
        color: this.tokens.color('border'),
        radius: this.tokens.metric('control.container.radius'),
      })
  }
}
