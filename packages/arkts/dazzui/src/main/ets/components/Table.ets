import { DazzTokens } from '../foundation/Tokens'

@Component
export struct DazzTable {
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Column() {
      this.content()
    }
      .width('100%')
  }
}

@Component
export struct DazzTableRow {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop head: boolean = false
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Row() {
      this.content()
    }
      .padding({
        left: this.tokens.metric('spacing.5'),
        right: this.tokens.metric('spacing.5'),
        top: this.head ? 0 : this.tokens.metric('spacing.3'),
        bottom: this.head ? 0 : this.tokens.metric('spacing.3'),
      })
      .backgroundColor(
        this.head ? this.tokens.color('surface-chrome') : Color.Transparent
      )
      .border({
        width: { bottom: this.tokens.metric('stroke.hairline') },
        color: this.tokens.color('border'),
      })
      .width('100%')
  }
}

@Component
export struct DazzTableCell {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop head: boolean = false
  @Prop contentAlign: string = 'start'
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Column() {
      this.content()
    }
      .alignItems(
        this.contentAlign === 'end' ? HorizontalAlign.End : HorizontalAlign.Start
      )
      .padding({
        top: this.head ? this.tokens.metric('spacing.2') : 0,
        bottom: this.head ? this.tokens.metric('spacing.2') : 0,
      })
      .layoutWeight(1)
  }
}
