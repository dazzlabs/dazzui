import { DazzTokens, DazzIcon, DazzBadge } from '@dazzlabs/dazzui'

/** The same navigation row used by React's example sidebar. */
@Component
export struct GalleryNavRow {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop label: string = ''
  @Prop selected: boolean = false
  @Prop icon: string = ''
  @Prop count: number = -1
  @State hovered: boolean = false
  onOpen: () => void = () => {}
  build() {
    Button({ type: ButtonType.Normal }) {
      Row({ space: this.tokens.metric('spacing.2') }) {
        if (this.icon) { DazzIcon({ name: this.icon, iconSize: this.tokens.metric('icon.medium') }) }
        Text(this.label)
          .fontSize(this.tokens.fontSize(this.tokens.type('labelSmall').size))
          .fontWeight(this.tokens.type(this.selected ? 'labelStrong' : 'labelQuiet').weight)
          .fontColor(this.tokens.color(this.selected ? 'content' : 'content-nav'))
          .maxLines(1).textOverflow({ overflow: TextOverflow.Ellipsis }).layoutWeight(1)
        if (this.count >= 0) { DazzBadge({ label: String(this.count), tint: 'neutral', controlSize: 'small' }) }
      }.width('100%')
    }
      .height(this.tokens.size('small')).width('100%')
      .padding({ left: this.tokens.metric('spacing.2'), right: this.tokens.metric('spacing.2') })
      .backgroundColor(this.selected ? this.tokens.color('surface-sunken') :
        this.hovered ? this.tokens.paint('normal', 'surface', 'hovered', 'neutral') : Color.Transparent)
      .borderRadius(this.tokens.metric('control.container.radius'))
      .stateEffect(false).accessibilityText(this.label)
      .onHover((value: boolean) => { this.hovered = value })
      .onClick(() => this.onOpen())
  }
}

/** A palette preview reads its own tokens; picking it does not resize it. */
@Component
export struct GalleryThemeTile {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop themeName: string = 'studio-light'
  @Prop title: string = ''
  @Prop selected: boolean = false
  onChoose: () => void = () => {}
  preview(): DazzTokens { return new DazzTokens(this.themeName) }
  build() {
    Button({ type: ButtonType.Normal }) {
      Column() {
        Row() {
          Rect().fill(this.preview().color('surface-column')).width('25%').height('100%')
          Column({ space: 4 }) {
            Rect().fill(this.preview().color('content')).width('72%').height(2)
            Rect().fill(this.preview().color('content')).width('48%').height(2).opacity(0.5)
            Rect().fill(this.preview().color('primary.500')).width('40%').height(6).borderRadius(2)
          }.alignItems(HorizontalAlign.Start).justifyContent(FlexAlign.Center)
            .padding(6).layoutWeight(1).height('100%').backgroundColor(this.preview().color('surface'))
        }.height(40).width('100%')
          .border({ width: { bottom: this.tokens.metric('stroke.hairline') }, color: this.preview().color('border-strong') })
        Text(this.title).fontSize('11vp').fontWeight(FontWeight.Medium)
          .fontColor(this.tokens.color('content')).height(24).width('100%')
          .padding({ left: 6, right: 6 }).textAlign(TextAlign.Center)
          .maxLines(1).textOverflow({ overflow: TextOverflow.Ellipsis })
      }.width('100%')
    }.width('100%').padding(0).stateEffect(false)
      .backgroundColor(this.tokens.color('surface'))
      .border({ width: this.tokens.metric('stroke.hairline'),
        color: this.selected ? this.tokens.color('primary.600') : this.tokens.color('border'),
        radius: this.tokens.metric('control.container.radius') })
      .clip(true).accessibilityText(this.title)
      .accessibilityChecked(this.selected).onClick(() => this.onChoose())
  }
}
