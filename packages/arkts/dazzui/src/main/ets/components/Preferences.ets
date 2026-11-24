import { ColorMetrics, LengthMetrics } from '@kit.ArkUI'
import { DazzIcon } from './Icon'
import { DazzTooltip } from './Tooltip'
import { DazzTokens } from '../foundation/Tokens'
import {
  DazzPreferenceThemeOption,
  DazzSwatchPalette,
  resolveSwatchPalette,
  resolveSwatchThemeName,
} from '../foundation/PreferenceSwatchesData'

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

@Component
struct PreferenceThemeSwatch {
  @Consume('dazzui.theme') @Watch('syncPalette') tokens: DazzTokens
  @Prop label: string = ''
  @Prop @Watch('syncPalette') themeName: string = 'bright-light'
  @Prop selected: boolean = false
  @Prop disabled: boolean = false
  onSelect: () => void = () => {}
  @State private palette: DazzSwatchPalette = resolveSwatchPalette(
    { value: 'bright-light', label: '', theme: 'bright-light' },
    new DazzTokens('bright-light')
  )
  aboutToAppear(): void {
    this.syncPalette()
  }
  syncPalette(): void {
    this.palette = resolveSwatchPalette(
      { value: this.themeName, label: this.label, theme: this.themeName },
      this.tokens
    )
  }
  @LocalBuilder swatchButton() {
    Button({ type: ButtonType.Normal }) {
      Stack({ alignContent: Alignment.Center }) {
        Row() {
          Row() {}
            .width('30%')
            .height('100%')
            .backgroundColor(this.palette.column)
            .border({
              width: { right: this.palette.hairline },
              color: this.palette.border,
            })
          Column() {
            Row() {}
              .width('100%')
              .height(this.palette.inkHeight)
              .borderRadius(this.palette.barRadius)
              .backgroundColor(this.palette.ink)
            Row() {}
              .width('60%')
              .height(this.palette.accentHeight)
              .borderRadius(this.palette.barRadius)
              .backgroundColor(this.palette.accent)
          }
            .layoutWeight(1)
            .height('100%')
            .padding(this.palette.paperPadding)
            .alignItems(HorizontalAlign.Start)
            .justifyContent(FlexAlign.SpaceBetween)
        }
          .width(this.palette.swatchWidth)
          .height(this.palette.swatchHeight)
          .backgroundColor(this.palette.surface)
          .borderRadius(this.palette.radius)
          .border({
            width: this.palette.hairline,
            color: this.palette.border,
            radius: this.palette.radius,
          })
          .clip(true)
        if (this.selected) {
          Row() {}
            .width(this.palette.swatchWidth + this.palette.ringGap * 2)
            .height(this.palette.swatchHeight + this.palette.ringGap * 2)
            .border({
              width: this.palette.ringGap,
              color: this.palette.ringGapColor,
              radius: this.palette.radius + this.palette.ringGap,
            })
            .hitTestBehavior(HitTestMode.None)
            .touchable(false)
            .focusable(false)
          Row() {}
            .width(
              this.palette.swatchWidth +
                (this.palette.ringGap + this.palette.ringStroke) * 2
            )
            .height(
              this.palette.swatchHeight +
                (this.palette.ringGap + this.palette.ringStroke) * 2
            )
            .border({
              width: this.palette.ringStroke,
              color: this.palette.ringColor,
              radius:
                this.palette.radius +
                this.palette.ringGap +
                this.palette.ringStroke,
            })
            .hitTestBehavior(HitTestMode.None)
            .touchable(false)
            .focusable(false)
        }
      }
        .width(this.palette.swatchWidth)
        .height(this.palette.swatchHeight)
        .clip(false)
    }
      .width(this.palette.swatchWidth)
      .height(this.palette.swatchHeight)
      .padding(0)
      .backgroundColor(Color.Transparent)
      .borderRadius(this.palette.radius)
      .stateEffect(false)
      .clip(false)
      .enabled(!this.disabled)
      .opacity(this.disabled ? 0.5 : 1)
      .accessibilityRole(AccessibilityRoleType.BUTTON)
      .accessibilityText(this.label)
      .accessibilitySelected(this.selected)
      .focusBox({
        margin: LengthMetrics.vp(this.tokens.metric('focus.offset')),
        strokeWidth: LengthMetrics.vp(this.tokens.metric('focus.width')),
        strokeColor: ColorMetrics.resourceColor(this.tokens.focus()),
      })
      .onClick(() => {
        if (!this.disabled) this.onSelect()
      })
  }
  build() {
    DazzTooltip({
      label: this.label,
      disabled: !this.label,
      content: this.swatchButton,
    })
  }
}

@Component
export struct DazzPreferenceThemeSwatches {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop options: DazzPreferenceThemeOption[] = []
  @Prop value: string | undefined = undefined
  @Prop defaultValue: string = ''
  @Prop disabled: boolean = false
  @Prop accessibilityLabel: string = ''
  @State localValue: string = ''
  onValueChange: (value: string) => void = () => {}
  aboutToAppear(): void {
    this.localValue = this.defaultValue
  }
  choose(option: DazzPreferenceThemeOption): void {
    if (this.disabled) return
    if (this.value === undefined) this.localValue = option.value
    this.onValueChange(option.value)
  }
  build() {
    Flex({
      wrap: FlexWrap.Wrap,
      justifyContent: FlexAlign.End,
      space: {
        main: LengthMetrics.vp(this.tokens.metric('spacing.1.5')),
        cross: LengthMetrics.vp(this.tokens.metric('spacing.1.5')),
      },
    }) {
      ForEach(
        this.options,
        (option: DazzPreferenceThemeOption) => {
          PreferenceThemeSwatch({
            label: option.label,
            themeName: resolveSwatchThemeName(option, this.tokens.name),
            selected: (this.value ?? this.localValue) === option.value,
            disabled: this.disabled,
            onSelect: () => this.choose(option),
          })
        },
        (option: DazzPreferenceThemeOption) => option.value
      )
    }
      .width(LayoutPolicy.wrapContent)
      .height(LayoutPolicy.wrapContent)
      .flexShrink(1)
      .clip(false)
      .accessibilityText(this.accessibilityLabel)
  }
}
