import { DazzTokens } from '../foundation/Tokens'

import { DazzIcon } from './Icon'
import { DazzButton } from './Button'

export class DazzDialogContext {
  controller?: CustomDialogController
  close(): void { this.controller?.close() }
}

// ArkUI owns the modal focus scope, outside-click dismissal and Escape.
// The explicit provider carries the caller's theme into the overlay subtree.
@CustomDialog
export struct DazzDialogSurface {
  controller?: CustomDialogController
  @Provide('dazzui.theme') tokens: DazzTokens = new DazzTokens()
  @Provide('dazzui.dialog') dialog: DazzDialogContext = new DazzDialogContext()
  @Prop title: string = ''
  @Prop description: string = ''
  @Prop contentWidth: number | undefined = undefined
  @BuilderParam content: () => void = this.empty
  @BuilderParam footer: () => void = this.empty
  @LocalBuilder dismiss() {
    DazzIcon({
      name: 'Dismiss12Regular',
      iconSize: this.tokens.metric('icon.small'),
    })
  }
  @LocalBuilder empty() {}
  aboutToAppear(): void {
    this.dialog.controller = this.controller
  }
  build() {
    Column({ space: this.tokens.metric('spacing.4') }) {
      if (this.title) {
        Row({ space: this.tokens.metric('spacing.3') }) {
          Text(this.title)
            .fontSize(this.tokens.type('labelStrong').size)
            .fontWeight(this.tokens.type('labelStrong').weight)
            .fontColor(this.tokens.color('content'))
            .layoutWeight(1)
          DazzButton({
            content: this.dismiss,
            square: true,
            variant: 'plain',
            accessibilityLabel: 'Close',
            onAction: (): void => { this.controller?.close() },
          })
        }
          .width('100%')
      }
      if (this.description) {
        Text(this.description)
          .fontSize(this.tokens.type('bodySmall').size)
          .fontColor(this.tokens.color('content-muted'))
      }
      this.content()
      this.footer()
    }
      .alignItems(HorizontalAlign.Start)
      .padding(
        this.tokens.metric('spacing.4') + this.tokens.metric('spacing.0.5')
      )
      .width(this.contentWidth ?? this.tokens.metric('dialog.width'))
      .constraintSize({ maxWidth: '90%' })
      .backgroundColor(this.tokens.color('surface-overlay'))
      .border({
        width: this.tokens.metric('stroke.hairline'),
        color: this.tokens.color('border-strong'),
        radius: this.tokens.metric('frame.popover.radius'),
      })
  }
}

@Component
export struct DazzDialog {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop title: string = ''
  @Prop description: string = ''
  @Prop contentWidth: number | undefined = undefined
  @BuilderParam trigger: () => void = this.defaultTrigger
  @BuilderParam content: () => void = this.empty
  @BuilderParam footer: () => void = this.empty
  private controller?: CustomDialogController
  @LocalBuilder empty() {}
  @LocalBuilder defaultTrigger() {
    DazzButton({ label: 'Open dialog' })
  }
  open(): void {
    this.controller = new CustomDialogController({
      builder: DazzDialogSurface({
        tokens: this.tokens,
        title: this.title,
        description: this.description,
        contentWidth: this.contentWidth,
        content: this.forwardContent,
        footer: this.forwardFooter,
      }),
      customStyle: true,
      autoCancel: true,
      maskColor: this.tokens.scrim(),
      alignment: DialogAlignment.Center,
    })
    this.controller.open()
  }
  aboutToDisappear(): void {
    this.controller?.close()
  }
  build() {
    Row() {
      this.trigger()
    }
      .onClick(() => this.open())
  }

  @LocalBuilder forwardContent() {
    this.content()
  }

  @LocalBuilder forwardFooter() {
    this.footer()
  }
}

@Component
export struct DazzDialogClose {
  @Consume('dazzui.dialog') dialog: DazzDialogContext = new DazzDialogContext()
  @Prop label: string = 'Close'
  build() {
    DazzButton({ label: this.label, onAction: () => this.dialog.close() })
  }
}
