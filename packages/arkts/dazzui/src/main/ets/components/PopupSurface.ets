import { DazzTokens } from '../foundation/Tokens'
import { DazzActivationContext } from '../foundation/Activation'
import { shadowOptions } from '../foundation/Shadow'

let popupSurfaceSequence: number = 0

/** The native popup casts its shadow outside the content clipping boundary. */
export function popupShadow(tokens: DazzTokens, density: number): ShadowOptions {
  return shadowOptions(tokens.shadow('shadow.lg')[0], density)
}

/** The shared prose popup: geometry and decoration come from ds. */
@Component
export struct DazzPopupSurface {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Provide({ allowOverride: 'dazzui.prose' }) prose: string = 'bodySmall'
  @Provide({ allowOverride: 'dazzui.activation' }) activation: DazzActivationContext = new DazzActivationContext()
  @Prop contentWidth: number | undefined = undefined
  @Prop maximumWidth: number | undefined = undefined
  @Prop contentPadding: number | undefined = undefined
  @BuilderParam content: () => void = this.empty
  private surfaceID: string = 'dazzui-popup-surface-' + String(++popupSurfaceSequence)
  @LocalBuilder empty() {}
  build() {
    Column() { this.content() }
      .alignItems(HorizontalAlign.Start)
      .width(this.contentWidth ?? LayoutPolicy.wrapContent).height(LayoutPolicy.wrapContent)
      .constraintSize({ maxWidth: this.maximumWidth })
      .padding((this.contentPadding ?? this.tokens.metric('spacing.4') + this.tokens.metric('spacing.0.5')) + this.tokens.metric('stroke.hairline'))
      .backgroundColor(this.tokens.color('surface-overlay'))
      .border({ width: this.tokens.metric('stroke.hairline'),
        color: this.tokens.color('border-strong'), radius: this.tokens.metric('radius.large') })
      .id(this.surfaceID)
  }
}
