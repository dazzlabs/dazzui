import { DazzActivationBoundary } from '../foundation/Activation'
import { DazzTokens } from '../foundation/Tokens'

import { DazzButton } from './Button'
import { DazzPopupSurface, popupShadow } from './PopupSurface'

export class DazzPopoverContext { close: () => void = () => {} }


@Component
export struct DazzPopover {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop title: string = ''
  @Prop description: string = ''
  @Prop contentWidth: number | undefined = undefined
  @Prop side: Placement = Placement.Bottom
  @State open: boolean = false
  @State hasBody: boolean = false
  @Provide({ allowOverride: 'dazzui.popover' }) popup: DazzPopoverContext = new DazzPopoverContext()
  @BuilderParam trigger: () => void = this.defaultTrigger
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  @LocalBuilder defaultTrigger() {
    DazzButton({ label: 'Open popover' })
  }
  @LocalBuilder body() {
    Column() {
      Column({ space: this.tokens.metric('spacing.1') }) {
      if (this.title) {
        Text(this.title)
          .fontSize(this.tokens.fontSize(this.tokens.type('labelLarge').size))
          .fontWeight(this.tokens.type('labelLarge').weight)
          .lineHeight(this.tokens.fontSize(this.tokens.type('labelLarge').leading))
          .fontColor(this.tokens.color('content'))
      }
      if (this.description) {
        Text(this.description)
          .fontSize(this.tokens.fontSize(this.tokens.type('bodySmall').size))
          .lineHeight(this.tokens.fontSize(this.tokens.type('bodySmall').leading))
          .fontColor(this.tokens.color('content-secondary'))
      }
      }.alignItems(HorizontalAlign.Start)
      Column() { this.content() }
        .alignItems(HorizontalAlign.Start).height(LayoutPolicy.wrapContent)
        .onAreaChange((_old: Area, area: Area) => { this.hasBody = Number(area.height) > 0 })
        .margin({ top: this.hasBody && (this.title || this.description) ? this.tokens.metric('spacing.3') : 0 })
    }
      .alignItems(HorizontalAlign.Start)
  }
  @LocalBuilder sheet() { DazzPopupSurface({ contentWidth: this.contentWidth, content: this.body }) }
  aboutToAppear(): void { this.popup.close = () => { this.open = false } }
  @LocalBuilder forwardTrigger() { this.trigger() }
  build() {
    DazzActivationBoundary({ content: this.forwardTrigger, onAction: () => { this.open = true } })
      .bindPopup(this.open, {
        builder: this.sheet,
        placement: this.side,
        popupColor: this.tokens.color('surface-overlay'), backgroundBlurStyle: BlurStyle.NONE,
        enableArrow: false, radius: this.tokens.metric('radius.large'), mask: false,
        targetSpace: this.tokens.metric('spacing.1'),
        shadow: popupShadow(this.tokens, this.getUIContext().vp2px(1)),
        autoCancel: true,
        onStateChange: (event: PopupStateChangeParam) => {
          this.open = event.isVisible
        },
      })
  }
}

@Component
export struct DazzPopoverClose {
  @Consume('dazzui.popover') popup: DazzPopoverContext = new DazzPopoverContext()
  @BuilderParam content: () => void = this.defaultContent
  @LocalBuilder defaultContent() { DazzButton({ label: 'Close' }) }
  @LocalBuilder forwarded() { this.content() }
  build() { DazzActivationBoundary({ content: this.forwarded, onAction: () => this.popup.close() }) }
}
