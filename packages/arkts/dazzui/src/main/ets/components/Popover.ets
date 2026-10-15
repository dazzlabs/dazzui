import { DazzTokens } from '../foundation/Tokens'

import { DazzButton } from './Button'

@Component
export struct DazzPopover {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop title: string = ''
  @Prop description: string = ''
  @Prop contentWidth: number | undefined = undefined
  @Prop side: Placement = Placement.Bottom
  @State open: boolean = false
  @BuilderParam trigger: () => void = this.defaultTrigger
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  @LocalBuilder defaultTrigger() {
    DazzButton({ label: 'Open popover' })
  }
  @LocalBuilder sheet() {
    Column({ space: this.tokens.metric('spacing.3') }) {
      if (this.title) {
        Text(this.title)
          .fontSize(this.tokens.type('labelStrong').size)
          .fontWeight(this.tokens.type('labelStrong').weight)
          .fontColor(this.tokens.color('content'))
      }
      if (this.description) {
        Text(this.description)
          .fontSize(this.tokens.type('bodySmall').size)
          .fontColor(this.tokens.color('content-muted'))
      }
      this.content()
    }
      .alignItems(HorizontalAlign.Start)
      .padding(this.tokens.metric('spacing.4'))
      .width(this.contentWidth ?? this.tokens.metric('preview-card.width'))
      .backgroundColor(this.tokens.color('surface-overlay'))
      .borderRadius(this.tokens.metric('frame.popover.radius'))
  }
  build() {
    Row() {
      this.trigger()
    }
      .onClick(() => {
        this.open = !this.open
      })
      .bindPopup(this.open, {
        builder: this.sheet,
        placement: this.side,
        popupColor: this.tokens.color('surface-overlay'),
        autoCancel: true,
        onStateChange: (event: PopupStateChangeParam) => {
          this.open = event.isVisible
        },
      })
  }
}

@Component
export struct DazzTooltip {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop label: string = ''
  @Prop side: Placement = Placement.Top
  @State visible: boolean = false
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Row() {
      this.content()
    }
      .onHover((hover: boolean) => {
        this.visible = hover
      })
      .onFocus(() => {
        this.visible = true
      })
      .onBlur(() => {
        this.visible = false
      })
      .bindPopup(this.visible, {
        message: this.label,
        placement: this.side,
        popupColor: this.tokens.color('surface-overlay'),
        messageOptions: {
          textColor: this.tokens.color('content'),
          font: { size: this.tokens.type('captionSmall').size },
        },
        autoCancel: true,
      })
  }
}
