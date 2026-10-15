import { DazzTokens } from '../foundation/Tokens'

import { DazzIcon } from './Icon'
import { DazzButton } from './Button'

@Component
export struct DazzCollapsible {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop title: string = ''
  @Prop meta: string = ''
  @Prop defaultOpen: boolean = false
  @State open: boolean = false
  @BuilderParam content: () => void = this.empty
  @LocalBuilder arrow() {
    DazzIcon({
      name: 'ChevronRight12Regular',
      iconSize: this.tokens.metric('icon.small'),
    }).rotate({ angle: this.open ? 90 : 0 })
  }
  @LocalBuilder empty() {}
  aboutToAppear(): void {
    this.open = this.defaultOpen
  }
  build() {
    Column({ space: this.tokens.metric('spacing.3') }) {
      DazzButton({
        label: this.title,
        content: this.arrow,
        shortcut: this.meta,
        variant: 'plain',
        tint: 'neutral',
        fullWidth: true,
        onAction: () => {
          this.open = !this.open
        },
      })
      if (this.open) {
        this.content()
      }
    }
      .alignItems(HorizontalAlign.Start)
      .width('100%')
  }
}
