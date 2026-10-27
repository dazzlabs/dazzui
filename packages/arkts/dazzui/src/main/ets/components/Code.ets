import { pasteboard } from '@kit.BasicServicesKit'
import { DazzIconButton } from './Button'
import { DazzIcon } from './Icon'
import { DazzTokens } from '../foundation/Tokens'

@Component
export struct DazzCode {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop code: string = ''
  @Prop block: boolean = false
  build() {
    Text(this.code)
      .fontFamily('monospace')
      .fontSize(this.tokens.fontSize(this.tokens.type(this.block ? 'bodySmall' : 'captionSmall').size))
      .fontColor(this.tokens.color('content'))
      .backgroundColor(this.tokens.color('surface-sunken'))
      .padding({ top: this.tokens.metric(this.block ? 'spacing.4' : 'spacing.0.5'), bottom: this.tokens.metric(this.block ? 'spacing.4' : 'spacing.0.5'), left: this.tokens.metric(this.block ? 'spacing.4' : 'spacing.1'), right: this.tokens.metric(this.block ? 'spacing.4' : 'spacing.1') })
      .borderRadius(this.tokens.metric('radius.tiny'))
      .copyOption(CopyOptions.LocalDevice)
  }
}

@Component
export struct DazzCodeBlock {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop code: string = ''
  @Prop title: string = ''
  @Prop language: string = ''
  @Prop lineNumbers: boolean = false
  @Prop copyable: boolean = false
  @Prop wrap: boolean = false
  @State copied: boolean = false
  private timer: number = -1
  body(): string { return this.code.replace(/\n+$/, '') }
  aboutToDisappear(): void { clearTimeout(this.timer) }
  async copy(): Promise<void> {
    try {
      await pasteboard.getSystemPasteboard().setData(pasteboard.createData(pasteboard.MIMETYPE_TEXT_PLAIN, this.body()))
      this.copied = true
      clearTimeout(this.timer)
      this.timer = setTimeout(() => { this.copied = false }, 1600)
    } catch (_) {}
  }
  @LocalBuilder copyGlyph() { DazzIcon({ name: this.copied ? 'Checkmark16Regular' : 'Copy16Regular' }) }
  @LocalBuilder source() {
    Text(this.body()).fontFamily('monospace')
      .fontSize(this.tokens.fontSize(this.tokens.type('captionSmall').size))
      .lineHeight(this.tokens.fontSize(this.tokens.type('bodySmall').leading))
      .fontColor(this.tokens.color('content')).copyOption(CopyOptions.LocalDevice)
      .width(this.wrap ? '100%' : 'auto')
  }
  build() {
    Column() {
      if (this.title || this.language || this.copyable) {
        Row({ space: this.tokens.metric('spacing.2') }) {
          if (this.title) {
            Text(this.title).fontFamily('monospace').fontSize(this.tokens.fontSize(this.tokens.type('captionSmall').size))
              .fontColor(this.tokens.color('content-secondary')).maxLines(1).textOverflow({ overflow: TextOverflow.Ellipsis })
          }
          if (this.language) {
            Text(this.language.toUpperCase()).fontSize(this.tokens.fontSize(this.tokens.type('labelSmall').size))
              .fontWeight(this.tokens.type('labelMedium').weight).fontColor(this.tokens.color('content-faint'))
          }
          Blank()
          if (this.copyable) { DazzIconButton({ label: this.copied ? 'Copied' : 'Copy', hint: this.copied ? 'Copied' : 'Copy code', content: this.copyGlyph, onAction: () => this.copy() }) }
        }.constraintSize({ minHeight: this.tokens.metric('spacing.8') })
          .padding({ top: this.tokens.metric('spacing.1'), bottom: this.tokens.metric('spacing.1'), left: this.tokens.metric('spacing.3'), right: this.tokens.metric('spacing.3') })
          .width('100%').backgroundColor(this.tokens.color('surface-chrome'))
          .border({ width: { bottom: this.tokens.metric('stroke.hairline') }, color: this.tokens.color('border') })
      }
      Row() {
        if (this.lineNumbers) {
          Text(this.body().split('\n').map((_line: string, index: number): string => String(index + 1)).join('\n'))
            .fontFamily('monospace').fontSize(this.tokens.fontSize(this.tokens.type('captionSmall').size))
            .lineHeight(this.tokens.fontSize(this.tokens.type('bodySmall').leading))
            .fontColor(this.tokens.color('content-faint')).textAlign(TextAlign.End)
            .constraintSize({ minWidth: this.tokens.metric('spacing.10') })
            .padding({ right: this.tokens.metric('spacing.3') }).accessibilityLevel('no')
        }
        if (this.wrap) { Column() { this.source() }.layoutWeight(1) }
        else { Scroll() { this.source() }.scrollable(ScrollDirection.Horizontal).scrollBar(BarState.Auto).layoutWeight(1) }
      }.alignItems(VerticalAlign.Top).width('100%')
        .padding({ top: this.tokens.metric('spacing.2.5'), bottom: this.tokens.metric('spacing.2.5'),
          left: this.lineNumbers ? 0 : this.tokens.metric('spacing.3'), right: this.tokens.metric('spacing.3') })
    }.width('100%').backgroundColor(this.tokens.color('surface-muted'))
      .border({ width: this.tokens.metric('stroke.hairline'), color: this.tokens.color('border'), radius: this.tokens.metric('control.container.radius') })
      .clip(true)
  }
}
