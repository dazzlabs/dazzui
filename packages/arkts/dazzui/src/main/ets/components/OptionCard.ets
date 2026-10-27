import { DazzTokens } from '../foundation/Tokens'
import { DazzTint } from '../foundation/TokenTypes'

@Component
export struct DazzOptionCard {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop title: string = ''
  @Prop description: string = ''
  @Prop selected: boolean = false
  @Prop disabled: boolean = false
  @Prop tint: DazzTint = 'primary'
  onAction: () => void = () => {}
  build() {
    Button({ type: ButtonType.Normal }) {
      Column({ space: this.tokens.metric('spacing.2') }) {
        Text(this.title)
          .fontSize(this.tokens.fontSize(this.tokens.type('labelStrong').size))
          .fontWeight(
            this.tokens.type(this.selected ? 'labelStrong' : 'labelSmall')
              .weight
          )
          .fontColor(
            this.tokens.color(this.disabled ? 'content-faint' : 'content')
          )
        Text(this.description)
          .fontSize(this.tokens.fontSize(this.tokens.type('bodySmall').size))
          .fontColor(this.tokens.color('content-muted'))
      }
        .alignItems(HorizontalAlign.Start)
    }
      .padding(this.tokens.metric('spacing.4'))
      .borderRadius(this.tokens.metric('control.container.radius'))
      .backgroundColor(
        this.tokens.color(
          this.disabled
            ? 'surface-sunken'
            : this.selected
              ? 'surface'
              : 'surface-muted'
        )
      )
      .border({
        width: this.tokens.metric('stroke.control'),
        color: this.selected
          ? this.tokens.paint('filled', 'surface', 'normal', this.tint)
          : this.tokens.color('border'),
      })
      .enabled(!this.disabled)
      .onClick(this.onAction)
  }
}
