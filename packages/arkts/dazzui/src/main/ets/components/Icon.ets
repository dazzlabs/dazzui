import { DazzTokens } from '../foundation/Tokens'

@Component
export struct DazzIcon {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop name: string = 'Checkmark12Filled'
  @Prop iconSize: number | undefined = undefined
  @Prop color: string = ''
  build() {
    Image($rawfile('dazzui/icons/' + this.name + '.svg'))
      .width(this.iconSize ?? this.tokens.metric('icon.medium'))
      .height(this.iconSize ?? this.tokens.metric('icon.medium'))
      .fillColor(this.color || this.tokens.color('content'))
      .objectFit(ImageFit.Contain)
      .accessibilityLevel('no')
  }
}
