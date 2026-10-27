import { DazzIcon } from './Icon'
import { DazzTokens } from '../foundation/Tokens'

@Component
export struct DazzPreferences {
  @Consume('dazzui.theme') tokens: DazzTokens
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Column({ space: this.tokens.metric('preferences.group-gap') }) {
      this.content()
    }
      .alignItems(HorizontalAlign.Start)
      .width('100%')
      .constraintSize({ maxWidth: this.tokens.metric('preferences.width') })
  }
}

@Component
export struct DazzPreferenceGroup {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop title: string = ''
  @Prop description: string = ''
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Column({ space: this.tokens.metric('preferences.title-gap') }) {
      Column({ space: this.tokens.metric('preferences.heading-gap') }) {
        Text(this.title)
          .fontSize(this.tokens.fontSize(this.tokens.type('labelStrong').size))
          .fontWeight(this.tokens.type('labelStrong').weight)
          .fontColor(this.tokens.color('content'))
        if (this.description) {
          Text(this.description)
            .fontSize(this.tokens.fontSize(this.tokens.type('bodySmall').size))
            .fontColor(this.tokens.color('content-muted'))
        }
      }
        .alignItems(HorizontalAlign.Start)
      this.content()
    }
      .alignItems(HorizontalAlign.Start)
      .width('100%')
  }
}

@Component
export struct DazzPreferenceSection {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop label: string = ''
  @Prop footer: string = ''
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Column({ space: this.tokens.metric('preferences.heading-gap') }) {
      if (this.label) {
        Text(this.label)
          .fontSize(this.tokens.fontSize(this.tokens.type('captionSmall').size))
          .fontColor(this.tokens.color('content-muted'))
      }
      Column({ space: this.tokens.metric('preferences.row-gap') }) {
        this.content()
      }
        .padding(
          this.tokens.metric('spacing.4') + this.tokens.metric('spacing.0.5')
        )
        .backgroundColor(this.tokens.color('surface-muted'))
        .borderRadius(this.tokens.metric('control.container.radius'))
        .width('100%')
      if (this.footer) {
        Text(this.footer)
          .fontSize(this.tokens.fontSize(this.tokens.type('captionSmall').size))
          .fontColor(this.tokens.color('content-muted'))
      }
    }
      .alignItems(HorizontalAlign.Start)
      .width('100%')
  }
}

@Component
export struct DazzPreferenceRow {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop title: string = ''
  @Prop subtitle: string = ''
  onOpen: (() => void) | undefined = undefined
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Row({ space: this.tokens.metric('spacing.4') }) {
      Column({ space: this.tokens.metric('preferences.row-gap') }) {
        Text(this.title)
          .fontSize(this.tokens.fontSize(this.tokens.type('labelSmall').size))
          .fontWeight(this.tokens.type('labelSmall').weight)
          .fontColor(this.tokens.color('content'))
        if (this.subtitle) {
          Text(this.subtitle)
            .fontSize(this.tokens.fontSize(this.tokens.type('captionSmall').size))
            .fontColor(this.tokens.color('content-muted'))
        }
      }
        .alignItems(HorizontalAlign.Start)
        .layoutWeight(1)
      this.content()
      if (this.onOpen) { DazzIcon({ name: 'ChevronRight12Regular', iconSize: this.tokens.metric('icon.small'), color: this.tokens.color('content-faint') }) }
    }
      .padding({
        top: this.tokens.metric('preferences.row-padding'),
        bottom: this.tokens.metric('preferences.row-padding'),
      })
      .width('100%').focusable(this.onOpen !== undefined)
      .accessibilityRole(this.onOpen ? AccessibilityRoleType.BUTTON : AccessibilityRoleType.TEXT)
      .onClick(() => this.onOpen?.())
      .onKeyEvent((event: KeyEvent) => {
        if (this.onOpen && event.type === KeyType.Down && (event.keyCode === 2050 || event.keyCode === 2054)) { this.onOpen(); event.stopPropagation() }
      })
  }
}
