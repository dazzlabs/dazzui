import { DazzTokens } from '../foundation/Tokens'
import { DazzTint } from '../foundation/TokenTypes'

import { DazzIcon } from './Icon'

@Component
export struct DazzTag {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop label: string = ''
  @Prop selected: boolean = false
  @Prop tint: DazzTint = 'primary'
  @Prop disabled: boolean = false
  @Prop removable: boolean = false
  @Prop selectable: boolean = false
  onSelectedChange: (value: boolean) => void = () => {}
  onRemove: () => void = () => {}
  build() {
    Row({ space: this.tokens.metric('spacing.1') }) {
      if (this.selectable) {
        Button(this.label, { type: ButtonType.Normal })
          .fontSize(this.tokens.type('labelSmall').size)
          .fontWeight(this.tokens.type('labelMedium').weight)
          .fontColor(
            this.tokens.paint(
              'tinted',
              'content',
              this.disabled ? 'disabled' : 'normal',
              this.tint
            )
          )
          .backgroundColor(Color.Transparent)
          .padding(0)
          .enabled(!this.disabled)
          .onClick(() => this.onSelectedChange(!this.selected))
      } else {
        Text(this.label)
          .fontSize(this.tokens.type('labelSmall').size)
          .fontWeight(this.tokens.type('labelMedium').weight)
          .fontColor(
            this.tokens.paint(
              'tinted',
              'content',
              this.disabled ? 'disabled' : 'normal',
              this.tint
            )
          )
      }
      if (this.removable) {
        Button({ type: ButtonType.Normal }) {
          DazzIcon({
            name: 'Dismiss12Regular',
            iconSize: this.tokens.metric('icon.small'),
            color: this.tokens.paint(
              'tinted',
              'content',
              this.disabled ? 'disabled' : 'normal',
              this.tint
            ),
          })
        }
          .padding(0)
          .backgroundColor(Color.Transparent)
          .enabled(!this.disabled)
          .accessibilityText('Remove ' + this.label)
          .onClick(this.onRemove)
      }
    }
      .padding({
        left: this.tokens.metric('spacing.2'),
        right: this.tokens.metric('spacing.2'),
        top: this.tokens.metric('spacing.0.5'),
        bottom: this.tokens.metric('spacing.0.5'),
      })
      .backgroundColor(
        this.tokens.paint(
          'tinted',
          'surface',
          this.disabled ? 'disabled' : 'normal',
          this.tint
        )
      )
      .border({
        width: this.tokens.metric('stroke.hairline'),
        color: this.selected
          ? this.tokens.paint('outlined', 'border', 'normal', this.tint)
          : Color.Transparent,
        radius: this.tokens.metric('radius.tiny'),
      })
  }
}
