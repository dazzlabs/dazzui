import { DazzActivationBoundary, DazzActivationContext } from '../foundation/Activation'
import { DazzTokens, withAlpha } from '../foundation/Tokens'
import { DazzShadowLayer } from './ShadowLayer'
import { dispatchSidebarKey } from '../foundation/SidebarKeys'
import { focusID, rememberFocusScope, forgetFocusScope } from '../foundation/FocusData'

import { DazzButton } from './Button'

export class DazzDialogContext {
  controller?: CustomDialogController
  close(): void { this.controller?.close() }
}

// ArkUI owns the modal focus scope, outside-click dismissal and Escape.
// The native overlay inherits the caller's live theme scope.
@CustomDialog
export struct DazzDialogSurface {
  controller?: CustomDialogController
  @Provide({ allowOverride: 'dazzui.activation' }) activation: DazzActivationContext = new DazzActivationContext()
  @Consume('dazzui.theme') tokens: DazzTokens
  @Provide({ allowOverride: 'dazzui.dialog' }) dialog: DazzDialogContext = new DazzDialogContext()
  @Prop title: string = ''
  @Prop accessibilityLabel: string = ''
  @Prop tone: string = 'default'
  @Prop description: string = ''
  @Prop contentWidth: number | undefined = undefined
  @BuilderParam content: () => void = this.empty
  @BuilderParam footer: () => void = this.empty
  private scopeID: string = focusID('dialog')
  @State private paintWidth: number = 0
  @State private paintHeight: number = 0
  aboutToDisappear(): void { forgetFocusScope(this.getUIContext().getWindowName() ?? '', this.scopeID) }
  @LocalBuilder empty() {}
  aboutToAppear(): void {
    this.dialog.controller = this.controller
  }
  maximumSize(): string { return 'calc(100% - ' + String(2 * this.tokens.metric('spacing.6')) + 'vp)' }
  build() {
    Stack({ alignContent: Alignment.TopStart }) {
      if (this.paintWidth > 0 && this.paintHeight > 0) {
        DazzShadowLayer({ paintWidth: this.paintWidth, paintHeight: this.paintHeight,
          corner: this.tokens.metric('frame.window.radius'), shadowLayers: this.tokens.shadow('shadow.lg') })
          .position({ x: 0, y: 0 }).hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
      }
      Flex({ direction: FlexDirection.Column, alignItems: ItemAlign.Start }) {
        if (this.title) { DazzDialogHeader({ title: this.title, subtitle: this.description }) }
        this.content()
        this.footer()
      }
        .id(this.scopeID)
        .onFocus(() => rememberFocusScope(this.getUIContext().getWindowName() ?? '', this.scopeID))
        .onBlur(() => forgetFocusScope(this.getUIContext().getWindowName() ?? '', this.scopeID))
        .width('100%').height(LayoutPolicy.wrapContent)
        .constraintSize({ maxHeight: '100%' })
        .backgroundColor(this.tokens.color('surface-overlay'))
        .border({ width: this.tokens.metric('stroke.hairline'),
          color: this.tone === 'danger' ? withAlpha(this.tokens.color('danger.600'), this.tokens.metric('dialog.danger-border-alpha')) : this.tokens.color('border-strong'),
          radius: this.tokens.metric('frame.window.radius') })
        .borderRadius(this.tokens.metric('frame.window.radius')).clip(true)
        .onAreaChange((_old: Area, area: Area) => {
          this.paintWidth = Number(area.width); this.paintHeight = Number(area.height)
        })
        .accessibilityRole(this.tone === 'danger' ? AccessibilityRoleType.ALERT_DIALOG : AccessibilityRoleType.DIALOG)
        .accessibilityText(this.accessibilityLabel || this.title).accessibilityDescription(this.description)
        .onKeyEvent((event: KeyEvent): boolean =>
          dispatchSidebarKey(this.getUIContext().getWindowName() ?? '', event))
    }.width(this.contentWidth ?? this.tokens.metric('dialog.width'))
      .height(LayoutPolicy.wrapContent)
      .constraintSize({ maxWidth: this.maximumSize(), maxHeight: this.maximumSize() }).clip(false)
  }
}

@Component
export struct DazzDialog {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop title: string = ''
  @Prop accessibilityLabel: string = ''
  @Prop tone: string = 'default'
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
        title: this.title,
        accessibilityLabel: this.accessibilityLabel,
        tone: this.tone,
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
    DazzActivationBoundary({ content: this.forwardTrigger, onAction: () => this.open() })
  }

  @LocalBuilder forwardTrigger() { this.trigger() }
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


@Component
export struct DazzDialogHeader {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop title: string = ''
  @Prop subtitle: string = ''
  build() {
    Column({ space: this.tokens.metric('spacing.1') }) {
      if (this.title) {
        Text(this.title).fontSize(this.tokens.fontSize(this.tokens.type('titleSmall').size))
          .fontWeight(this.tokens.type('labelStrong').weight).lineHeight(this.tokens.fontSize(this.tokens.type('titleSmall').size))
          .fontColor(this.tokens.color('content'))
      }
      if (this.subtitle) {
        Text(this.subtitle).fontSize(this.tokens.fontSize(this.tokens.type('captionSmall').size))
          .lineHeight(this.tokens.fontSize(this.tokens.type('captionSmall').leading)).fontColor(this.tokens.color('content-subtle'))
      }
    }.alignItems(HorizontalAlign.Start).constraintSize({ minHeight: this.tokens.metric('spacing.11') }).flexShrink(0)
      .padding({ top: this.tokens.metric('spacing.3'), bottom: this.tokens.metric('spacing.3') + this.tokens.metric('stroke.hairline'), left: this.tokens.metric('spacing.5'), right: this.tokens.metric('spacing.5') })
      .border({ width: { bottom: this.tokens.metric('stroke.hairline') }, color: this.tokens.color('border') }).width('100%')
  }
}

@Component
export struct DazzDialogBody {
  @Consume('dazzui.theme') tokens: DazzTokens
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Scroll() {
      Column({ space: this.tokens.metric('spacing.4') }) { this.content() }
        .alignItems(HorizontalAlign.Start).width('100%')
        .padding({ top: this.tokens.metric('spacing.4') + this.tokens.metric('spacing.0.5'), bottom: this.tokens.metric('spacing.4') + this.tokens.metric('spacing.0.5'), left: this.tokens.metric('spacing.5'), right: this.tokens.metric('spacing.5') })
    }.width('100%').height(LayoutPolicy.wrapContent).flexShrink(1)
      .scrollable(ScrollDirection.Vertical).scrollBar(BarState.Auto).edgeEffect(EdgeEffect.None)
  }
}

@Component
export struct DazzDialogFooter {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop contentAlign: FlexAlign = FlexAlign.End
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Row({ space: this.tokens.metric('spacing.2') }) { this.content() }
      .justifyContent(this.contentAlign).width('100%').flexShrink(0).backgroundColor(this.tokens.color('surface-chrome'))
      .padding({ top: this.tokens.metric('spacing.3') + this.tokens.metric('stroke.hairline'), bottom: this.tokens.metric('spacing.3'), left: this.tokens.metric('spacing.5'), right: this.tokens.metric('spacing.5') })
      .border({ width: { top: this.tokens.metric('stroke.hairline') }, color: this.tokens.color('border') })
  }
}
