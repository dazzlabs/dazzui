import { DazzTokens } from '../foundation/Tokens'
import { DazzTint } from '../foundation/TokenTypes'

@Component
export struct DazzAvatar {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop name: string = ''
  @Prop fallback: string = ''
  @Prop controlSize: string = 'medium'
  @Prop shape: string = 'circle'
  @Prop tint: DazzTint = 'primary'
  initials(): string {
    return (
      this.fallback ||
      this.name
        .trim()
        .split(/\s+/)
        .map((word: string) => word.slice(0, 1))
        .slice(0, 2)
        .join('')
        .toUpperCase()
    )
  }
  length(): number {
    return this.tokens.metric(
      this.controlSize === 'small'
        ? 'spacing.5'
        : this.controlSize === 'large'
          ? 'spacing.9'
          : 'spacing.7'
    )
  }
  build() {
    Text(this.initials())
      .fontSize(this.tokens.fontSize(
        this.tokens.type(
          this.controlSize === 'small'
            ? 'captionSmall'
            : this.controlSize === 'large'
              ? 'labelLarge'
              : 'labelMedium'
        ).size
      ))
      .fontWeight(this.tokens.type('labelStrong').weight)
      .fontColor(this.tokens.paint('tinted', 'content', 'normal', this.tint))
      .backgroundColor(
        this.tokens.paint('tinted', 'surface', 'normal', this.tint)
      )
      .textAlign(TextAlign.Center)
      .width(this.length())
      .height(this.length())
      .borderRadius(
        this.tokens.metric(
          this.shape === 'square' ? 'radius.medium' : 'radius.full'
        )
      )
      .accessibilityText(this.name)
  }
}

export interface DazzAvatarPerson {
  name: string
  fallback?: string
  tint?: DazzTint
}

@Component
export struct DazzAvatarGroup {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop people: DazzAvatarPerson[] = []
  @Prop max: number = Infinity
  @Prop controlSize: string = 'medium'
  visible(): DazzAvatarPerson[] { return this.people.slice(0, Math.max(0, Math.floor(this.max))) }
  overlap(): number { return this.tokens.metric(this.controlSize === 'small' ? 'spacing.1.5' : this.controlSize === 'large' ? 'spacing.3' : 'spacing.2') }
  length(): number { return this.tokens.metric(this.controlSize === 'small' ? 'spacing.5' : this.controlSize === 'large' ? 'spacing.9' : 'spacing.7') }
  build() {
    Row() {
      ForEach(this.visible(), (person: DazzAvatarPerson, index: number) => {
        Stack() {
          DazzAvatar({ name: person.name, fallback: person.fallback ?? '', tint: person.tint ?? 'primary', controlSize: this.controlSize })
        }.backgroundColor(this.tokens.color('surface')).borderRadius(this.tokens.metric('radius.full'))
          .outline({ width: this.tokens.metric('stroke.control'), color: this.tokens.color('surface'), radius: this.tokens.metric('radius.full') })
          .margin({ left: index ? -this.overlap() : 0 })
      }, (person: DazzAvatarPerson, index: number) => String(index) + person.name)
      if (this.people.length > this.visible().length) {
        Text('+' + String(this.people.length - this.visible().length))
          .fontSize(this.tokens.fontSize(this.tokens.type(this.controlSize === 'large' ? 'labelMedium' : 'captionSmall').size))
          .fontWeight(this.tokens.type('labelStrong').weight).fontColor(this.tokens.color('content-muted'))
          .backgroundColor(this.tokens.color('surface-sunken')).textAlign(TextAlign.Center)
          .width(this.length()).height(this.length()).borderRadius(this.tokens.metric('radius.full'))
          .outline({ width: this.tokens.metric('stroke.control'), color: this.tokens.color('surface'), radius: this.tokens.metric('radius.full') })
          .margin({ left: this.visible().length ? -this.overlap() : 0 })
          .accessibilityText(String(this.people.length - this.visible().length) + ' more people')
      }
    }
  }
}
