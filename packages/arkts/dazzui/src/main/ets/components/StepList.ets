import { DazzTokens } from '../foundation/Tokens'

import { DazzIcon } from './Icon'

@Component
export struct DazzStepList {
  @Consume('dazzui.theme') tokens: DazzTokens
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Column({ space: this.tokens.metric('spacing.2') }) {
      this.content()
    }
      .alignItems(HorizontalAlign.Start)
      .width('100%')
  }
}

@Component
export struct DazzStep {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop status: string = 'idle'
  @Prop label: string = ''
  @Prop meta: string = ''
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Row({ space: this.tokens.metric('spacing.2.5') }) {
      if (this.status === 'done') {
        DazzIcon({
          name: 'Checkmark12Filled',
          iconSize: this.tokens.metric('icon.small'),
          color: this.tokens.color('success.600'),
        })
      } else if (this.status === 'active') {
        LoadingProgress()
          .color(this.tokens.color('primary.600'))
          .width(this.tokens.metric('icon.medium'))
          .height(this.tokens.metric('icon.medium'))
      } else {
        Circle()
          .stroke(this.tokens.color('border-muted'))
          .strokeWidth(this.tokens.metric('stroke.control'))
          .fill(Color.Transparent)
          .width(this.tokens.metric('icon.medium'))
          .height(this.tokens.metric('icon.medium'))
      }
      Text(this.label)
        .fontSize(this.tokens.type('labelQuiet').size)
        .fontWeight(
          this.tokens.type(
            this.status === 'active' ? 'labelMedium' : 'labelQuiet'
          ).weight
        )
        .fontColor(
          this.tokens.color(
            this.status === 'done'
              ? 'content-muted'
              : this.status === 'idle'
                ? 'content-subtle'
                : 'content'
          )
        )
      this.content()
      Blank()
      if (this.meta) {
        Text(this.meta)
          .fontSize(this.tokens.type('labelSmall').size)
          .fontColor(
            this.status === 'active'
              ? this.tokens.paint('plain', 'content')
              : this.tokens.color('content-faint')
          )
      }
    }
      .width('100%')
  }
}
