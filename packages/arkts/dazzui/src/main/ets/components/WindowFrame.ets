import { curves, LengthMetrics } from '@kit.ArkUI'
import { DazzTokens, withAlpha } from '../foundation/Tokens'
import { DazzSidebarContext } from '../foundation/SidebarData'
import { DazzShadowLayer } from './ShadowLayer'
import { DazzWindowLight } from './WindowLight'
import {
  DazzWindowPlatform, DazzWindowControls, DazzWindowControlItem,
  DazzWindowContext, DazzWindowChrome, windowChrome, windowMac,
  DazzWindowTitleStyle, windowTitleStyle,
  windowControls, windowControlItems, windowGlyph, windowTitlebarControls, windowHandoff,
} from '../foundation/WindowData'

let windowSurfaceSequence: number = 0

/** Presentation chrome only. These buttons never manipulate the host OS window. */
@Component
struct WindowControl {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop platform: DazzWindowPlatform = 'macos'
  @Prop item: DazzWindowControlItem = { kind: 'close', state: 'enabled' }
  @Prop bandHeight: number = 0
  @Prop inactive: boolean = false
  @Prop groupHovered: boolean = false
  @State hovered: boolean = false
  @State pressed: boolean = false
  onAction: () => void = () => {}
  chrome(): DazzWindowChrome { return windowChrome(this.platform) }
  mac(): boolean { return windowMac(this.platform) }
  ink(): ResourceColor {
    if (this.mac()) return '#8C000000'
    if (this.inactive) return this.tokens.color('content-faint')
    return this.platform === 'windows' && this.item.kind === 'close' && this.hovered ? '#FFFFFF' : this.tokens.color('content')
  }
  surface(): ResourceColor {
    if (this.mac()) return this.item.state === 'disabled' || (this.inactive && !this.groupHovered)
      ? this.tokens.color('surface-sunken') : this.item.kind === 'close' ? '#FF5F57' : this.item.kind === 'minimize' ? '#FEBC2E' : '#28C840'
    if (this.platform === 'windows') return this.hovered ? this.item.kind === 'close' ? '#C42B1C' : this.tokens.color('surface-subtle') : Color.Transparent
    return this.hovered ? withAlpha(this.tokens.color(this.inactive ? 'content-faint' : 'content'), 0.16)
      : this.platform === 'kde' ? Color.Transparent : this.tokens.color('surface-subtle')
  }
  lightSurface(): string { return this.surface() as string }
  glyphSize(): number { return this.mac() || this.platform === 'windows' ? 10 : 16 }
  viewport(): number { return this.mac() ? 12 : this.platform === 'windows' ? 10 : 16 }
  setPressed(pressed: boolean): void {
    if (this.item.state !== 'enabled') return
    if (this.platform === 'macos') this.getUIContext().animateTo({ duration: pressed ? 90 : 420,
      curve: pressed ? Curve.EaseOut : curves.cubicBezierCurve(0.34, 1.9, 0.5, 1) }, () => { this.pressed = pressed })
    else this.pressed = pressed
  }
  @LocalBuilder glyph() {
    Shape() {
      Path().commands(windowGlyph(this.platform, this.item.kind))
        .fill(Color.Transparent).stroke(this.ink())
        .strokeWidth((this.mac() ? 1.3 : this.platform === 'windows' ? 1 : 1.5) + 'px')
        .strokeLineCap(this.platform === 'windows' ? LineCapStyle.Butt : LineCapStyle.Round)
        .strokeLineJoin(LineJoinStyle.Round)
        .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
    // Path commands use physical pixels; keep the SVG coordinate space in px.
    }.viewPort({ x: 0, y: 0, width: this.viewport() + 'px', height: this.viewport() + 'px' })
      .id('dazz-window-control-glyph-' + this.item.kind)
      .width(this.glyphSize()).height(this.glyphSize())
      .opacity(this.mac() && (!this.groupHovered || this.item.state !== 'enabled') ? 0 : 1)
      .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
  }
  build() {
    Stack() {
      Button({ type: ButtonType.Normal }) {
        if (!this.mac()) { this.glyph() }
      }.width(this.chrome().button).height(this.platform === 'windows' ? this.bandHeight : this.chrome().button)
        .padding(0).backgroundColor(this.mac() ? Color.Transparent : this.surface()).borderRadius(this.platform === 'windows' ? 0 : this.chrome().button / 2)
        .shadow({ radius: 0, color: Color.Transparent })
        .id('dazz-window-control-' + this.item.kind)
        .stateEffect(false).enabled(this.item.state === 'enabled').focusable(false).focusOnTouch(false)
        .accessibilityText(this.item.kind === 'close' ? 'Close' : this.item.kind === 'minimize' ? 'Minimize' : 'Maximize')
        .onHover((hovered: boolean) => { this.hovered = hovered; if (!hovered) this.setPressed(false) })
        .onTouch((event: TouchEvent) => { if (event.type === TouchType.Down) this.setPressed(true); else if (event.type === TouchType.Up || event.type === TouchType.Cancel) this.setPressed(false) })
        .onClick(() => { if (this.item.state === 'enabled') this.onAction() })
      if (this.mac()) {
        Stack() {
          DazzWindowLight({ diameter: this.chrome().button, surface: this.lightSurface(), glazed: this.platform === 'macos' })
          this.glyph()
        }.width('100%').height('100%').hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
          .accessibilityLevel('no-hide-descendants')
      }
    }.width(this.chrome().button).height(this.platform === 'windows' ? this.bandHeight : this.chrome().button)
      .scale({ x: this.platform === 'macos' && this.pressed ? 0.8 : 1, y: this.platform === 'macos' && this.pressed ? 0.8 : 1 })
  }
}

@Component
struct WindowControls {
  @Prop platform: DazzWindowPlatform = 'macos'
  @Prop controls: DazzWindowControls | false = false
  @Prop bandHeight: number = 0
  @Prop inactive: boolean = false
  @State hovered: boolean = false
  onClose: () => void = () => {}
  onMinimize: () => void = () => {}
  onMaximize: () => void = () => {}
  build() {
    Row({ space: windowChrome(this.platform).gap }) {
      ForEach(windowControlItems(this.platform, this.controls), (item: DazzWindowControlItem) => {
        WindowControl({ platform: this.platform, item, bandHeight: this.bandHeight,
          inactive: this.inactive, groupHovered: this.hovered,
          onAction: () => { if (item.kind === 'close') this.onClose(); else if (item.kind === 'minimize') this.onMinimize(); else this.onMaximize() } })
      }, (item: DazzWindowControlItem) => item.kind)
    }.height(windowMac(this.platform) ? windowChrome(this.platform).button : this.bandHeight)
      .id('dazz-window-controls-group-' + String(this.getUniqueId()))
      .onHover((hovered: boolean) => { this.hovered = hovered })
  }
}

/** A heading-local provider keeps forwarded LocalBuilder slots live. Passing
 * a style snapshot as a builder argument would freeze its initial palette. */
@Component
struct WindowHeading {
  @Consume('dazzui.theme') @Watch('sync') tokens: DazzTokens
  @Consume('dazzui.window') @Watch('sync') window: DazzWindowContext = new DazzWindowContext()
  @Provide({ allowOverride: 'dazzui.window-title' }) style: DazzWindowTitleStyle = new DazzWindowTitleStyle()
  @Prop title: string = ''
  @Prop subtitle: string = ''
  @Prop titleHidden: boolean = false
  @Prop inPane: boolean = false
  @Prop contentInset: number = 0
  @BuilderParam titleContent?: () => void
  aboutToAppear(): void { this.sync() }
  sync(): void { this.style = windowTitleStyle(this.tokens, this.window.platform, this.window.inactive) }
  build() {
    Flex({ direction: FlexDirection.Row, alignItems: ItemAlign.Baseline,
      justifyContent: this.inPane || this.window.platform === 'windows' ? FlexAlign.Start : FlexAlign.Center,
      space: { main: LengthMetrics.vp(this.tokens.metric('spacing.2')) } }) {
      if (this.titleContent !== undefined) {
        Row() { this.titleContent() }.width(LayoutPolicy.wrapContent).constraintSize({ maxWidth: '100%' }).flexShrink(1)
      } else {
        Text(this.title).fontSize(this.tokens.fontSize(this.style.size)).lineHeight(this.tokens.fontSize(this.style.leading))
          .id('dazz-window-title-text-' + String(this.getUniqueId()))
          .fontWeight(this.style.weight).fontColor(this.style.color)
          .maxLines(1).textOverflow({ overflow: TextOverflow.Ellipsis }).flexShrink(1)
      }
      if (this.subtitle) {
        Text(this.subtitle).fontSize(this.tokens.fontSize(this.tokens.type('captionSmall').size))
          .id('dazz-window-subtitle-' + String(this.getUniqueId()))
          .fontWeight(this.tokens.type('captionSmall').weight).fontColor(this.tokens.color('content-subtle'))
          .lineHeight(this.tokens.fontSize(this.style.leading))
          .maxLines(1).textOverflow({ overflow: TextOverflow.Ellipsis }).flexShrink(1)
      }
    }.width(LayoutPolicy.wrapContent).constraintSize({ maxWidth: '100%' })
      .padding({ left: this.contentInset, right: this.contentInset })
      .layoutWeight(this.window.platform === 'windows' && !this.inPane ? 0 : 1)
      .visibility(this.titleHidden ? Visibility.Hidden : Visibility.Visible)
  }
}

@Component
export struct DazzWindowTitlebar {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.window') @Watch('sync') window: DazzWindowContext = new DazzWindowContext()
  @Consume('dazzui.sidebar') sidebar: DazzSidebarContext = new DazzSidebarContext()
  @Prop title: string = ''
  @Prop subtitle: string = ''
  @Prop titleHidden: boolean = false
  @Prop @Watch('sync') inPane: boolean = false
  @Prop @Watch('sync') controls: DazzWindowControls | false | undefined = undefined
  @BuilderParam titleContent?: () => void
  @BuilderParam leading?: () => void
  @BuilderParam toolbarContent?: () => void
  @BuilderParam trailing?: () => void
  @BuilderParam accessory?: () => void
  onClose?: () => void
  onMinimize?: () => void
  onMaximize?: () => void
  chrome(): DazzWindowChrome { return windowChrome(this.window.platform) }
  mac(): boolean { return windowMac(this.window.platform) }
  carrying(): DazzWindowControls | false {
    return windowTitlebarControls(this.window, this.controls)
  }
  bandHeight(): number {
    return !this.window.owned || this.inPane || this.toolbarContent !== undefined || this.accessory !== undefined || this.window.fullSizeContent
      ? this.tokens.metric('frame.titlebar.size') : this.chrome().height
  }
  startInset(): number {
    if (this.inPane) return this.mac() && !this.window.rootBand && !this.sidebar.open
      ? Math.max(this.tokens.metric('spacing.5'), this.chrome().titleInset - this.tokens.metric('frame.sidebar.icon-width')) : this.tokens.metric('spacing.5')
    return this.window.platform === 'windows' ? this.tokens.metric('spacing.4') : this.chrome().inset
  }
  titleInset(): number { return this.inPane ? 0 : this.chrome().titleInset }
  sync(): void { if (this.inPane) this.window.pane(this.carrying() !== false && !this.mac()) }
  aboutToAppear(): void { this.sync() }
  aboutToDisappear(): void { if (this.inPane && !this.mac()) this.window.pane(false) }
  @LocalBuilder buttons() {
    WindowControls({ platform: this.window.platform, controls: this.carrying(), bandHeight: this.bandHeight(), inactive: this.window.inactive,
      onClose: this.onClose ?? (() => { if (windowHandoff(this.window, this.controls)) this.window.onClose() }),
      onMinimize: this.onMinimize ?? (() => { if (windowHandoff(this.window, this.controls)) this.window.onMinimize() }),
      onMaximize: this.onMaximize ?? (() => { if (windowHandoff(this.window, this.controls)) this.window.onMaximize() }) })
  }
  @LocalBuilder heading() {
    WindowHeading({ title: this.title, subtitle: this.subtitle, titleHidden: this.titleHidden,
      inPane: this.inPane, contentInset: this.titleInset(), titleContent: this.titleContent })
  }
  build() {
    Stack({ alignContent: Alignment.Start }) {
      Row({ space: this.tokens.metric('spacing.2') }) {
        if (this.leading !== undefined) {
          Row({ space: this.tokens.metric('spacing.2') }) { this.leading() }
            .margin({ left: !this.inPane && this.mac() ? this.chrome().titleInset : 0 })
            .flexShrink(0)
        }
        this.heading()
        if (this.toolbarContent !== undefined) {
          Row({ space: this.tokens.metric('spacing.3') }) { this.toolbarContent() }
            .layoutWeight(this.window.platform === 'windows' && !this.inPane ? 1 : 0).flexShrink(1)
        }
        if (this.trailing !== undefined) {
          Row({ space: this.tokens.metric('spacing.2') }) { this.trailing() }
            .margin({ right: this.inPane ? 0 : this.chrome().trailingInset }).flexShrink(0)
        }
        if (this.carrying() !== false && (this.window.platform === 'windows' || (this.inPane && !this.mac()))) { this.buttons() }
      }.width('100%').height(this.bandHeight())
        .padding({ left: this.startInset(), right: this.inPane && this.carrying() === false ? this.tokens.metric('spacing.5') : this.inPane ? this.chrome().inset : 0 })
      if (this.accessory !== undefined) {
        Row() { this.accessory() }.position({ x: '50%', y: 0 }).markAnchor({ x: '50%', y: 0 }).height(this.bandHeight())
      }
      if (this.carrying() !== false && (this.mac() || (!this.inPane && this.window.platform !== 'windows'))) {
        Row() { this.buttons() }.position({ x: this.mac() ? (this.bandHeight() - this.chrome().button) / 2 : 0,
          y: this.mac() ? (this.bandHeight() - this.chrome().button) / 2 : 0 })
          .width(this.mac() ? LayoutPolicy.wrapContent : '100%')
          .justifyContent(this.mac() ? FlexAlign.Start : FlexAlign.End)
          .padding({ right: this.mac() ? 0 : this.chrome().inset }).hitTestBehavior(HitTestMode.Transparent)
      }
    }.width('100%').height(this.bandHeight()).flexShrink(0)
      .backgroundColor(this.window.platform === 'kde' ? this.tokens.color('surface-chrome') : Color.Transparent)
      .border({ width: { bottom: this.window.platform === 'kde' ? this.tokens.metric('stroke.hairline') : 0 }, color: this.tokens.color('border') })
  }
}

@Component
export struct DazzWindowFrame {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Provide({ allowOverride: 'dazzui.window' }) window: DazzWindowContext = new DazzWindowContext()
  @Prop @Watch('sync') platform: DazzWindowPlatform = 'macos'
  @Prop title: string = ''
  @Prop subtitle: string = ''
  @Prop titleHidden: boolean = false
  @Prop @Watch('sync') showTitlebar: boolean = true
  @Prop @Watch('sync') controls: DazzWindowControls | false | undefined = undefined
  @Prop @Watch('sync') inactive: boolean = false
  @Prop @Watch('sync') fullSizeContent: boolean = false
  @Prop applicationBody: boolean = false
  @Prop contentWidth: number | string | undefined = undefined
  @Prop contentHeight: number | string | undefined = undefined
  @Prop minimumWidth: number | string | undefined = undefined
  @Prop maximumWidth: number | string = '100%'
  @Prop minimumHeight: number | string | undefined = undefined
  @Prop maximumHeight: number | string | undefined = undefined
  @BuilderParam content: () => void = this.empty
  @BuilderParam titleContent?: () => void
  @BuilderParam leading?: () => void
  @BuilderParam toolbarContent?: () => void
  @BuilderParam trailing?: () => void
  @BuilderParam accessory?: () => void
  onClose: () => void = () => {}
  onMinimize: () => void = () => {}
  onMaximize: () => void = () => {}
  @State paneCarries: boolean = false
  @State paintWidth: number = 0
  @State paintHeight: number = 0
  private surfaceID: string = 'dazzui-window-surface-' + String(++windowSurfaceSequence)
  @LocalBuilder empty() {}
  band(): boolean { return this.showTitlebar && this.platform !== 'omarchy' }
  aboutToAppear(): void { this.sync() }
  sync(): void {
    const next = new DazzWindowContext()
    next.owned = true; next.platform = this.platform; next.controls = this.platform === 'omarchy' ? false : windowControls(this.platform, this.controls)
    next.rootBand = this.band(); next.inactive = this.inactive; next.fullSizeContent = this.fullSizeContent
    next.onClose = () => this.onClose(); next.onMinimize = () => this.onMinimize(); next.onMaximize = () => this.onMaximize()
    next.pane = (mounted: boolean) => { this.paneCarries = mounted }
    this.window = next
  }
  chrome(): DazzWindowChrome { return windowChrome(this.platform) }
  floatingHeight(): number { return this.fullSizeContent ? this.tokens.metric('frame.titlebar.size') : this.chrome().height }
  @LocalBuilder titlebar() {
    DazzWindowTitlebar({ title: this.title, subtitle: this.subtitle, titleHidden: this.titleHidden,
      controls: this.window.controls, onClose: this.onClose, onMinimize: this.onMinimize, onMaximize: this.onMaximize,
      titleContent: this.titleContent, leading: this.leading, toolbarContent: this.toolbarContent, trailing: this.trailing, accessory: this.accessory })
  }
  build() {
    Stack({ alignContent: Alignment.TopStart }) {
      if (this.platform !== 'omarchy' && this.paintWidth > 0 && this.paintHeight > 0) {
        DazzShadowLayer({ paintWidth: this.paintWidth, paintHeight: this.paintHeight, corner: this.chrome().radius,
          shadowLayers: this.tokens.shadow(this.inactive ? 'shadow.md' : 'shadow.xl') }).position({ x: 0, y: 0 })
          .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
      }
      Stack({ alignContent: Alignment.TopStart }) {
        Column() {
          if (this.band() && !this.fullSizeContent) { this.titlebar() }
          Column() { this.content() }.width('100%').alignItems(HorizontalAlign.Start)
            .layoutWeight(this.contentHeight !== undefined ? 1 : 0)
            .height(this.contentHeight === undefined ? LayoutPolicy.wrapContent : undefined)
            .padding({ top: this.platform === 'omarchy' && !this.applicationBody ? 20 : 0 })
            .opacity(this.platform === 'macos' && this.inactive ? 0.72 : 1)
        }.width('100%').height(this.contentHeight === undefined ? LayoutPolicy.wrapContent : '100%').alignItems(HorizontalAlign.Start)
        if (this.band() && this.fullSizeContent) {
          Row() { this.titlebar() }.width('100%').position({ x: 0, y: 0 })
        }
        if (!this.band() && this.window.controls !== false && (!this.paneCarries || windowMac(this.platform))) {
          Row() {
            WindowControls({ platform: this.platform, controls: this.window.controls, bandHeight: this.floatingHeight(), inactive: this.inactive,
              onClose: this.onClose, onMinimize: this.onMinimize, onMaximize: this.onMaximize })
          }.position({ x: windowMac(this.platform) ? (this.floatingHeight() - this.chrome().button) / 2 : 0,
            y: windowMac(this.platform) ? (this.floatingHeight() - this.chrome().button) / 2 : 0 })
            .width(windowMac(this.platform) ? LayoutPolicy.wrapContent : '100%')
            .justifyContent(windowMac(this.platform) ? FlexAlign.Start : FlexAlign.End)
            .padding({ right: windowMac(this.platform) ? 0 : this.chrome().inset }).hitTestBehavior(HitTestMode.Transparent)
        }
      }.id(this.surfaceID).width('100%').height(this.contentHeight === undefined ? LayoutPolicy.wrapContent : '100%')
      .borderRadius(this.chrome().radius).clip(true).backgroundColor(this.tokens.color('surface'))
      .border({ width: this.platform === 'omarchy' ? 2 : 0, color: this.tokens.color(this.inactive ? 'border-muted' : 'primary.500') })
      .outline({ width: this.platform === 'omarchy' ? 0 : this.tokens.metric('stroke.hairline'), color: this.tokens.color('border-strong'), radius: this.chrome().radius })
      .onAreaChange((_old: Area, area: Area) => { this.paintWidth = Number(area.width); this.paintHeight = Number(area.height) })
    }.width(this.contentWidth ?? this.tokens.metric('frame.window.width')).height(this.contentHeight ?? LayoutPolicy.wrapContent)
      .constraintSize({ minWidth: this.minimumWidth, maxWidth: this.maximumWidth, minHeight: this.minimumHeight, maxHeight: this.maximumHeight })
      .accessibilityText(this.title)
  }
}

@Component
export struct DazzWindowBody {
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() { Row() { this.content() }.width('100%').height('100%').clip(true) }
}
@Component
export struct DazzWindowMain {
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() { Column() { this.content() }.width('100%').height('100%').layoutWeight(1).alignItems(HorizontalAlign.Start).clip(true) }
}
@Component
export struct DazzWindowContent {
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() { Row() { this.content() }.width('100%').height('100%').layoutWeight(1).alignItems(VerticalAlign.Top).clip(true) }
}
@Component
export struct DazzWindowFooter {
  @Consume('dazzui.theme') tokens: DazzTokens
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Row({ space: this.tokens.metric('spacing.4') }) { this.content() }.width('100%').flexShrink(0)
      .padding({ top: this.tokens.metric('spacing.3'), bottom: this.tokens.metric('spacing.3'), left: this.tokens.metric('spacing.5'), right: this.tokens.metric('spacing.5') })
      .backgroundColor(this.tokens.color('surface-chrome'))
      .border({ width: { top: this.tokens.metric('stroke.hairline') }, color: this.tokens.color('border') })
  }
}
