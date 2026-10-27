import { FrameCallback, LengthMetrics } from '@kit.ArkUI'
import {
  DazzTheme, DazzTokens, DazzButton, DazzIconButton, DazzIcon, DazzBadge,
  DazzSearchField, DazzSelect, DazzDivider, DazzSizing, DazzSectionLabel,
  DazzSidebarProvider,
} from '@dazzlabs/dazzui'
import { STORIES } from '../generated/Stories'
import { THEME_GROUPS } from '../generated/Themes'
import { Scene, Story, ThemeFamily, ThemeGroup, SceneNavigation } from '../lib/SceneTypes'
import { SceneView } from '../lib/SceneView'
import { GalleryNavRow, GalleryThemeTile } from '../lib/GalleryParts'

class ResizeFrame extends FrameCallback {
  private action: () => void
  constructor(action: () => void) {
    super()
    this.action = action
  }
  onIdle(_timeLeft: number): void { this.action() }
}

@Component
struct GallerySearch {
  @Link query: string
  build() {
    DazzSearchField({ value: this.query, controlSize: 'small', placeholder: 'Search',
      accessibilityLabel: 'Search components', onValueChange: (value: string) => { this.query = value } })
  }
}

@Entry({ useSharedStorage: true })
@Component
struct Index {
  @LocalStorageProp('themeName') theme: string = 'studio-light'
  @LocalStorageProp('sizing') sizing: DazzSizing = 'design'
  @LocalStorageProp('page') selected: string = ''
  @LocalBuilder gallery() {
    Gallery({ theme: this.$theme, sizing: this.$sizing, selected: this.$selected })
  }
  build() {
    Column() {
      DazzTheme({ name: this.theme, sizing: this.sizing, content: this.gallery })
    }.width('100%').height('100%')
  }
}

@Component
struct Gallery {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Link theme: string
  @Link sizing: DazzSizing
  @Link selected: string
  @Provide('dazzui.navigation') pageNavigation: SceneNavigation = new SceneNavigation()
  @State query: string = ''
  @State showThemes: boolean = true
  @State showList: boolean = true
  @State documentWidth: number = 640
  @State shellWidth: number = 1100
  @State shellHeight: number = 700
  @State listWidth: number = 272
  @State themesWidth: number = 336
  private resizing: string = ''
  private resizeEpoch: number = 0
  private documentScroll: Scroller = new Scroller()

  aboutToAppear(): void {
    this.pageNavigation.open = (href: string) => {
      const id = href.replace(/^#\//, '')
      if (STORIES.some((story: Story) => story.id === id)) this.open(id)
    }
  }
  matches(): Story[] {
    const query = this.query.trim().toLowerCase()
    return STORIES.filter((story: Story) => !query ||
      (story.name + ' ' + story.summary).toLowerCase().includes(query))
  }
  selectedStory(): Story | undefined { return STORIES.find((story: Story) => story.id === this.selected) }
  open(id: string = ''): void { this.selected = id; this.documentScroll.scrollTo({ xOffset: 0, yOffset: 0 }) }
  family(): string { return this.theme.replace(/-(light|dark)$/, '') }
  dark(): boolean { return this.theme.endsWith('-dark') }
  beginResize(x: number): void {
    this.resizeEpoch++
    const hit = this.tokens.metric('spacing.2')
    this.resizing = this.showList && Math.abs(x - this.listWidth) <= hit ? 'list' :
      this.showThemes && Math.abs(x - (this.shellWidth - this.themesWidth)) <= hit ? 'themes' : ''
  }
  endResize(): void {
    const side = this.resizing
    if (!side) return
    this.resizing = ''
    const epoch = ++this.resizeEpoch
    // The final native split layout may arrive after pointer release. Save its
    // measured width after rendering rather than freezing an intermediate move.
    this.getUIContext().postFrameCallback(new ResizeFrame(() => {
      if (epoch !== this.resizeEpoch) return
      const ui = this.getUIContext()
      const node = ui.getFrameNodeById(
        side === 'list' ? 'component-list' : 'theme-panel'
      )
      if (!node) return
      const width = ui.px2vp(node.getMeasuredSize().width)
      if (side === 'list') this.listWidth = Math.max(200, Math.min(420, width))
      else this.themesWidth = Math.max(280, Math.min(560, width))
    }))
  }
  aboutToDisappear(): void { this.resizeEpoch++ }
  centerWidth(): number { return Math.max(280, this.shellWidth - (this.showList ? this.listWidth : 0) - (this.showThemes ? this.themesWidth : 0) - (this.showList ? 1 : 0) - (this.showThemes ? 1 : 0)) }
  tileWidth(): number {
    const available = Math.max(0, this.documentWidth - this.tokens.metric('spacing.6') * 2)
    const gap = this.tokens.metric('spacing.3')
    const columns = Math.max(1, Math.floor((available + gap) / (256 + gap)))
    return Math.max(0, (available - gap * (columns - 1)) / columns)
  }
  @LocalBuilder sidebarGlyph() { DazzIcon({ name: 'PanelLeft20Regular' }) }
  @LocalBuilder panelGlyph() { DazzIcon({ name: 'PanelRight20Regular' }) }
  @LocalBuilder appearanceGlyph() { DazzIcon({ name: this.dark() ? 'WeatherMoon20Regular' : 'WeatherSunny20Regular' }) }
  @LocalBuilder navigation() {
    Column() {
      Button({ type: ButtonType.Normal }) {
        Row({ space: this.tokens.metric('spacing.2') }) {
          Rect().fill(this.tokens.color('primary.600')).width(this.tokens.metric('spacing.3'))
            .height(this.tokens.metric('spacing.3')).borderRadius(this.tokens.metric('radius.tiny'))
          Text('DazzUI').fontSize(this.tokens.fontSize(this.tokens.type('labelStrong').size))
            .fontWeight(this.tokens.type('labelStrong').weight).fontColor(this.tokens.color('content'))
          DazzBadge({ label: 'ArkUI', controlSize: 'small' })
        }.width('100%')
      }.height(this.tokens.metric('frame.titlebar.size')).width('100%')
        .padding({ left: this.tokens.metric('spacing.4'), right: this.tokens.metric('spacing.4') })
        .backgroundColor(Color.Transparent).stateEffect(false).onClick(() => this.open())
      Column({ space: this.tokens.metric('spacing.2') }) {
        GallerySearch({ query: $query }).id('component-search')
        GalleryNavRow({ label: 'Overview', icon: 'Home20Regular', count: STORIES.length,
          selected: !this.selectedStory(), onOpen: () => this.open() }).id('nav-overview')
        DazzSectionLabel({ label: 'Components' }).margin({ top: this.tokens.metric('spacing.2') })
        Scroll() {
          Column({ space: this.tokens.metric('frame.nav-gap') }) {
            ForEach(this.matches(), (story: Story) => {
              GalleryNavRow({ label: story.name, selected: this.selected === story.id,
                onOpen: () => this.open(story.id) }).id('nav-' + story.id)
            }, (story: Story) => story.id)
          }.width('100%')
        }.layoutWeight(1).width('100%')
      }.padding({ left: this.tokens.metric('spacing.2'), right: this.tokens.metric('spacing.2') })
        .layoutWeight(1).width('100%')
    }.width(this.listWidth).onAreaChange((_old: Area, area: Area) => { if (this.resizing === 'list') this.listWidth = Math.max(200, Math.min(420, Number(area.width))) }).constraintSize({ minWidth: 200, maxWidth: 420 })
      .height('100%').backgroundColor(this.tokens.color('surface-column')).id('component-list')
  }
  @LocalBuilder stage(scene: Scene, minimum: number) {
    Column() { SceneView({ scene }) }
      .justifyContent(FlexAlign.Center).alignItems(HorizontalAlign.Center)
      .constraintSize({ minHeight: minimum }).width('100%')
  }
  @LocalBuilder overview() {
    Flex({ wrap: FlexWrap.Wrap, space: { main: LengthMetrics.vp(this.tokens.metric('spacing.3')), cross: LengthMetrics.vp(this.tokens.metric('spacing.3')) } }) {
      ForEach(this.matches(), (story: Story) => {
        Column({ space: this.tokens.metric('spacing.2') }) {
          this.stage(story.preview, 128)
          DazzButton({ label: story.name, variant: 'plain', fullWidth: true,
            onAction: () => this.open(story.id) })
        }.padding(this.tokens.metric('spacing.3')).width(this.tileWidth())
          .border({ width: this.tokens.metric('stroke.hairline'), color: this.tokens.color('border'), radius: this.tokens.metric('radius.large') })
          .id('overview-' + story.id)
      }, (story: Story) => story.id)
    }.width('100%')
  }
  @LocalBuilder document(story: Story) {
    Column({ space: this.tokens.metric('spacing.6') }) {
      Text(story.summary).fontSize(this.tokens.fontSize(this.tokens.type('bodySmall').size))
        .fontColor(this.tokens.color('content-secondary')).constraintSize({ maxWidth: 640 })
      ForEach(story.examples, (scene: Scene) => {
        Column({ space: this.tokens.metric('spacing.2') }) {
          DazzSectionLabel({ label: scene.title })
          Column() { this.stage(scene, 0) }
            .justifyContent(FlexAlign.Center)
            .constraintSize({ minHeight: 160 })
            .padding(this.tokens.metric('spacing.4')).width('100%')
            .border({ width: this.tokens.metric('stroke.hairline'), color: this.tokens.color('border'), radius: this.tokens.metric('radius.large') })
            .id('stage-' + scene.id)
          if (scene.note) {
            Text(scene.note).fontSize(this.tokens.fontSize(this.tokens.type('captionLarge').size))
              .fontColor(this.tokens.color('content-secondary'))
          }
        }.alignItems(HorizontalAlign.Start).width('100%').id('example-' + scene.id)
      }, (scene: Scene) => scene.id)
    }.alignItems(HorizontalAlign.Start).width('100%')
  }
  @LocalBuilder themePanel() {
    Column() {
      Row() {
        Text('Theme').fontSize(this.tokens.fontSize(this.tokens.type('labelStrong').size))
          .fontWeight(this.tokens.type('labelStrong').weight).fontColor(this.tokens.color('content')).layoutWeight(1)
        DazzIconButton({ label: this.dark() ? 'Switch to light' : 'Switch to dark',
          content: this.appearanceGlyph, onAction: () => { this.theme = this.family() + (this.dark() ? '-light' : '-dark') } }).id('appearance-toggle')
      }.height(this.tokens.metric('frame.titlebar.size')).padding({ left: this.tokens.metric('spacing.4'), right: this.tokens.metric('spacing.4') }).width('100%')
      DazzDivider()
      Scroll() {
        Column({ space: this.tokens.metric('spacing.2') }) {
          Column({ space: this.tokens.metric('spacing.2') }) {
            DazzSectionLabel({ label: 'Sizing' })
            DazzSelect({ options: [{ value: 'design', label: 'Design sizes' }, { value: 'system', label: 'System controls' }],
              value: this.sizing, onValueChange: (value: string) => { this.sizing = value === 'system' ? 'system' : 'design' } }).id('sizing-select')
          }.width('100%')
          ForEach(THEME_GROUPS, (group: ThemeGroup) => {
            Column({ space: this.tokens.metric('spacing.2') }) {
              DazzSectionLabel({ label: group.title })
              Grid() {
                ForEach(group.families, (family: ThemeFamily) => {
                  GridItem() {
                    GalleryThemeTile({ title: family.title, themeName: family.value + (this.dark() ? '-dark' : '-light'),
                      selected: this.family() === family.value,
                      onChoose: () => { this.theme = family.value + (this.dark() ? '-dark' : '-light') } }).id('theme-' + family.value)
                  }
                }, (family: ThemeFamily) => family.value)
              }.columnsTemplate('1fr 1fr 1fr').columnsGap(this.tokens.metric('spacing.2'))
                .rowsGap(this.tokens.metric('spacing.2')).height(Math.ceil(group.families.length / 3) * 64 + Math.max(0, Math.ceil(group.families.length / 3) - 1) * this.tokens.metric('spacing.2'))
                .width('100%').scrollBar(BarState.Off)
            }.alignItems(HorizontalAlign.Start).width('100%').padding({ top: this.tokens.metric('spacing.2'), bottom: this.tokens.metric('spacing.2') })
          }, (group: ThemeGroup) => group.title)
        }.alignItems(HorizontalAlign.Start).padding(this.tokens.metric('spacing.4')).width('100%')
      }.layoutWeight(1).width('100%')
    }.width(this.themesWidth).onAreaChange((_old: Area, area: Area) => { if (this.resizing === 'themes') this.themesWidth = Math.max(280, Math.min(560, Number(area.width))) }).constraintSize({ minWidth: 280, maxWidth: 560 }).height('100%')
      .backgroundColor(this.tokens.color('surface-column')).id('theme-panel')
  }
  @LocalBuilder documentColumn() {
    Column() {
      Row({ space: this.tokens.metric('spacing.2') }) {
        DazzIconButton({ label: 'Toggle sidebar', content: this.sidebarGlyph,
          onAction: () => { this.showList = !this.showList } }).id('toggle-sidebar')
        Text(this.selectedStory()?.name ?? 'Overview')
          .fontSize(this.tokens.fontSize(this.tokens.type('labelStrong').size))
          .fontWeight(this.tokens.type('labelStrong').weight).fontColor(this.tokens.color('content'))
        Text(this.selectedStory() ? String(this.selectedStory()?.examples.length) + ' examples' : String(STORIES.length) + ' components')
          .fontSize(this.tokens.fontSize(this.tokens.type('captionLarge').size)).fontColor(this.tokens.color('content-secondary'))
        Blank()
        DazzIconButton({ label: 'Toggle theme panel', content: this.panelGlyph,
          onAction: () => { this.showThemes = !this.showThemes } }).id('toggle-theme-panel')
      }.padding({ left: this.tokens.metric('spacing.4'), right: this.tokens.metric('spacing.4') })
        .height(this.tokens.metric('frame.titlebar.size')).width('100%')
      DazzDivider()
      Scroll(this.documentScroll) {
        Column() {
          if (this.selectedStory()) {
            ForEach(STORIES.filter((story: Story) => story.id === this.selected), (story: Story) => { this.document(story) }, (story: Story) => story.id)
          } else { this.overview() }
        }.padding({ top: this.tokens.metric('spacing.5'), left: this.tokens.metric('spacing.6'), right: this.tokens.metric('spacing.6'), bottom: this.tokens.metric('spacing.10') }).width('100%')
      }.layoutWeight(1).width('100%').align(Alignment.TopStart)
    }.width(this.centerWidth()).constraintSize({ minWidth: 280 }).height('100%')
      .onAreaChange((_old: Area, area: Area) => { this.documentWidth = Number(area.width) })
      .backgroundColor(this.tokens.color('surface')).id('document-column')
  }
  @LocalBuilder shell() {
    RowSplit() {
      if (this.showList) { this.navigation() }
      this.documentColumn()
      if (this.showThemes) { this.themePanel() }
    }.resizeable(true).width(this.shellWidth).height(this.shellHeight)
      .onTouch((event: TouchEvent) => {
        if (event.type === TouchType.Down && event.touches.length) this.beginResize(event.touches[0].x)
        if (event.type === TouchType.Up || event.type === TouchType.Cancel) this.endResize()
      })
      .onMouse((event: MouseEvent) => {
        if (event.action === MouseAction.Press) this.beginResize(event.x)
        if (event.action === MouseAction.Release) this.endResize()
      })
      .backgroundColor(this.tokens.color('surface'))
  }
  build() {
    Column() {
      DazzSidebarProvider({ open: this.showList, content: this.shell,
        onOpenChange: (open: boolean) => { this.showList = open } })
    }.width('100%').height('100%')
      .onAreaChange((_old: Area, area: Area) => {
        // Explicit split dimensions invalidate ArkUI's cached drag geometry
        // when the host resizes, without unmounting story controls or IME.
        this.shellWidth = Number(area.width)
        this.shellHeight = Number(area.height)
      })
  }
}
