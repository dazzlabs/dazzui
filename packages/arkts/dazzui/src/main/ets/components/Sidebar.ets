import { ColorMetrics, LengthMetrics, curves } from '@kit.ArkUI'
import { DazzTokens } from '../foundation/Tokens'
import { DazzShadowLayer } from './ShadowLayer'
import { DazzTint, DazzVariant } from '../foundation/TokenTypes'
import {
  DazzSidebarContext, DazzSidebarColumnContext, DazzSidebarRowContext,
  DazzSidebarColumnData, registerSidebarShortcut, unregisterSidebarShortcut,
} from '../foundation/SidebarData'
import { dispatchSidebarKey } from '../foundation/SidebarKeys'
import { DazzIconButton } from './Button'
import { DazzIcon } from './Icon'
import { DazzTooltip, DazzTooltipLabel, tooltipShadow } from './Tooltip'
import { DazzTooltipDefaults } from '../foundation/TooltipData'
import { DazzSectionLabel } from './SectionLabel'

let sidebarSequence: number = 0
let sidebarSurfaceSequence: number = 0

@Component
export struct DazzSidebarProvider {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop @Watch('sync') open: boolean | undefined = undefined
  @Prop defaultOpen: boolean = true
  @State localOpen: boolean = true
  @Provide({ allowOverride: 'dazzui.sidebar' }) sidebar: DazzSidebarContext = new DazzSidebarContext()
  @Provide({ allowOverride: 'dazzui.tooltip-defaults' }) tooltipDefaults: DazzTooltipDefaults = new DazzTooltipDefaults(0)
  @BuilderParam content: () => void = this.empty
  onOpenChange: (open: boolean) => void = () => {}
  private owner: string = 'dazzui-sidebar-provider-' + String(++sidebarSequence)
  private columns: DazzSidebarColumnData[] = []
  private mounted: boolean = false
  @LocalBuilder empty() {}
  aboutToAppear(): void {
    this.mounted = true
    this.localOpen = this.defaultOpen
    this.sync()
    registerSidebarShortcut(this.getUIContext().getWindowName() ?? '', this.owner,
      () => this.setOpen(!this.sidebar.open))
  }
  aboutToDisappear(): void {
    this.mounted = false
    unregisterSidebarShortcut(this.owner)
  }
  setOpen(next: boolean): void {
    if (this.open === undefined) {
      this.localOpen = next
      this.sync()
    }
    this.onOpenChange(next)
  }
  column(id: string, variant: string, collapsible: string): void {
    const old = this.columns.find(column => column.id === id)
    if (old?.variant === variant && old?.collapsible === collapsible) return
    this.columns = this.columns.filter(column => column.id !== id)
    this.columns.push({ id, variant, collapsible })
    this.sync()
  }
  removeColumn(id: string): void {
    this.columns = this.columns.filter(column => column.id !== id)
    this.sync()
  }
  sync(): void {
    if (!this.mounted) return
    const snapshot = new DazzSidebarContext()
    snapshot.open = this.open ?? this.localOpen
    snapshot.inset = this.columns.some(column => column.variant === 'inset')
    snapshot.insetCollapsed = !snapshot.open && this.columns.some(column =>
      column.variant === 'inset' && column.collapsible !== 'none')
    snapshot.toggle = () => this.setOpen(!this.sidebar.open)
    snapshot.column = (id: string, variant: string, collapsible: string) => this.column(id, variant, collapsible)
    snapshot.removeColumn = (id: string) => this.removeColumn(id)
    this.sidebar = snapshot
  }
  build() {
    Row() { this.content() }.width('100%').height('100%')
      .alignItems(VerticalAlign.Top)
      .backgroundColor(this.sidebar.inset ? this.tokens.color('surface-column') : Color.Transparent)
      .onKeyEvent((event: KeyEvent): boolean =>
        dispatchSidebarKey(this.getUIContext().getWindowName() ?? '', event))
  }
}

@Component
export struct DazzSidebar {
  @Consume('dazzui.theme') @Watch('sync') tokens: DazzTokens
  @Consume('dazzui.sidebar') @Watch('sync') sidebar: DazzSidebarContext = new DazzSidebarContext()
  @Provide({ allowOverride: 'dazzui.sidebar.column' }) column: DazzSidebarColumnContext = new DazzSidebarColumnContext()
  @Prop @Watch('sync') side: string = 'left'
  @Prop @Watch('sync') variant: string = 'sidebar'
  @Prop @Watch('sync') collapsible: string = 'offcanvas'
  @BuilderParam content: () => void = this.empty
  private columnID: string = 'dazzui-sidebar-column-' + String(++sidebarSequence)
  private surfaceID: string = 'dazzui-sidebar-surface-' + String(++sidebarSurfaceSequence)
  @State private paintWidth: number = 0
  @State private paintHeight: number = 0
  @LocalBuilder empty() {}
  aboutToAppear(): void { this.sync() }
  aboutToDisappear(): void { this.sidebar.removeColumn(this.columnID) }
  baseWidth(): number {
    return this.tokens.metric(this.side === 'right' ? 'frame.aside.width'
      : this.collapsible === 'none' ? 'frame.rail.width' : 'frame.sidebar.width')
  }
  sync(): void {
    const snapshot = new DazzSidebarColumnContext()
    snapshot.icon = !this.sidebar.open && this.collapsible === 'icon'
    snapshot.offcanvas = !this.sidebar.open && this.collapsible === 'offcanvas'
    snapshot.expandedWidth = this.baseWidth()
    this.column = snapshot
    this.sidebar.column(this.columnID, this.variant, this.collapsible)
  }
  inset(): boolean { return this.variant !== 'sidebar' && this.collapsible !== 'none' }
  rootWidth(): number {
    const iconWidth = this.tokens.systemStyle
      ? Math.max(this.tokens.metric('frame.sidebar.icon-width'),
        this.tokens.size('medium') + 2 * this.tokens.metric('spacing.2.5') +
          this.tokens.metric('stroke.hairline'))
      : this.tokens.metric('frame.sidebar.icon-width')
    return this.column.offcanvas ? 0 : this.column.icon
      ? iconWidth + (this.inset() ? this.tokens.metric('spacing.4') : 0)
      : this.baseWidth()
  }
  containerWidth(): number {
    return this.column.icon ? this.rootWidth() + (this.inset() ? 2 * this.tokens.metric('stroke.hairline') : 0)
      : this.baseWidth()
  }
  motion(): AnimateParam {
    return { duration: this.tokens.metric('motion.duration'),
      curve: curves.cubicBezierCurve(this.tokens.metric('motion.easing.x1'), this.tokens.metric('motion.easing.y1'),
        this.tokens.metric('motion.easing.x2'), this.tokens.metric('motion.easing.y2')) }
  }
  floating(): boolean { return this.variant === 'floating' && this.collapsible !== 'none' }
  build() {
    Stack({ alignContent: this.side === 'left' ? Alignment.TopStart : Alignment.TopEnd }) {
      Column() {
        Stack({ alignContent: Alignment.TopStart }) {
          if (this.floating() && this.paintWidth > 0 && this.paintHeight > 0) {
            DazzShadowLayer({ paintWidth: this.paintWidth, paintHeight: this.paintHeight,
              corner: this.tokens.metric('radius.large'), shadowLayers: this.tokens.shadow('shadow.sm') })
              .position({ x: 0, y: 0 }).hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
          }
          Column() { this.content() }.width('100%').height('100%')
            .alignItems(HorizontalAlign.Start)
            .backgroundColor(this.tokens.color(this.collapsible === 'none' && this.side === 'left' ? 'surface' : 'surface-column'))
            .border({ width: this.floating() ? this.tokens.metric('stroke.hairline') : 0,
              color: this.tokens.color('border'), radius: this.floating() ? this.tokens.metric('radius.large') : 0 })
            .clip(this.column.icon).id(this.surfaceID)
            .onAreaChange((_old: Area, area: Area) => {
              this.paintWidth = Number(area.width); this.paintHeight = Number(area.height)
            })
        }.width('100%').height('100%').clip(false)
      }.width(this.containerWidth()).height('100%')
        .padding(this.inset() ? this.tokens.metric('spacing.2') : 0)
        .border({ width: {
          left: (this.collapsible === 'none' || this.variant === 'sidebar') && this.side === 'right' ? this.tokens.metric('stroke.hairline') : 0,
          right: (this.collapsible === 'none' || this.variant === 'sidebar') && this.side === 'left' ? this.tokens.metric('stroke.hairline') : 0,
        }, color: this.tokens.color('border') })
        .translate({ x: this.column.offcanvas ? (this.side === 'left' ? -this.baseWidth() : this.baseWidth()) : 0 })
        .animation(this.motion())
    }.width(this.rootWidth()).height('100%').clip(this.variant !== 'floating' || this.column.offcanvas)
      // React's positioned column paints above its following content pane.
      .zIndex(this.collapsible === 'none' ? 0 : 1)
      .animation(this.motion())
      .accessibilityRole(AccessibilityRoleType.NAVIGATION)
  }
}

@Component
export struct DazzSidebarInset {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.sidebar') sidebar: DazzSidebarContext = new DazzSidebarContext()
  @BuilderParam content: () => void = this.empty
  private surfaceID: string = 'dazzui-sidebar-inset-surface-' + String(++sidebarSurfaceSequence)
  @State private paintWidth: number = 0
  @State private paintHeight: number = 0
  @LocalBuilder empty() {}
  build() {
    Column() {
      Stack({ alignContent: Alignment.TopStart }) {
        if (this.sidebar.inset && this.paintWidth > 0 && this.paintHeight > 0) {
          DazzShadowLayer({ paintWidth: this.paintWidth, paintHeight: this.paintHeight,
            corner: this.tokens.metric('radius.large'), shadowLayers: this.tokens.shadow('shadow.sm') })
            .position({ x: 0, y: 0 }).hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
        }
        Column() { this.content() }.width('100%').height('100%')
          .alignItems(HorizontalAlign.Start).backgroundColor(this.tokens.color('surface'))
          .borderRadius(this.sidebar.inset ? this.tokens.metric('radius.large') : 0).id(this.surfaceID)
          .onAreaChange((_old: Area, area: Area) => {
            this.paintWidth = Number(area.width); this.paintHeight = Number(area.height)
          })
      }.width('100%').height('100%').clip(false)
    }.layoutWeight(1).width('100%').height('100%')
      .padding({ top: this.sidebar.inset ? this.tokens.metric('spacing.2') : 0,
        bottom: this.sidebar.inset ? this.tokens.metric('spacing.2') : 0,
        right: this.sidebar.inset ? this.tokens.metric('spacing.2') : 0,
        left: this.sidebar.insetCollapsed ? this.tokens.metric('spacing.2') : 0 })
  }
}

@Component
export struct DazzSidebarHeader {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.sidebar.column') column: DazzSidebarColumnContext = new DazzSidebarColumnContext()
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Column({ space: this.tokens.metric('spacing.2') }) { this.content() }
      .alignItems(HorizontalAlign.Start).justifyContent(FlexAlign.Center)
      .width(this.column.icon ? this.column.expandedWidth : '100%')
      .constraintSize({ minHeight: this.tokens.metric('frame.titlebar.size') })
      .padding({ top: this.tokens.metric('spacing.2'), bottom: this.tokens.metric('spacing.2'),
        left: this.tokens.metric('spacing.2.5'), right: this.tokens.metric('spacing.2.5') })
  }
}

@Component
export struct DazzSidebarContent {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.sidebar.column') column: DazzSidebarColumnContext = new DazzSidebarColumnContext()
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Scroll() {
      Column() { this.content() }.width('100%').alignItems(HorizontalAlign.Start)
    }.layoutWeight(1).width('100%').height('100%').align(Alignment.TopStart)
      .padding({ top: this.tokens.metric('spacing.2'), bottom: this.tokens.metric('spacing.2') })
      .scrollable(this.column.icon ? ScrollDirection.None : ScrollDirection.Vertical)
      .scrollBar(BarState.Auto)
  }
}

@Component
export struct DazzSidebarGroup {
  @Consume('dazzui.theme') tokens: DazzTokens
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Column() { this.content() }.width('100%').alignItems(HorizontalAlign.Start)
      .padding({ top: this.tokens.metric('spacing.1.5'), bottom: this.tokens.metric('spacing.1.5'),
        left: this.tokens.metric('spacing.2.5'), right: this.tokens.metric('spacing.2.5') })
  }
}

@Component
export struct DazzSidebarGroupLabel {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.sidebar.column') column: DazzSidebarColumnContext = new DazzSidebarColumnContext()
  @Prop label: string = ''
  build() {
    if (!this.column.icon) {
      Row() { DazzSectionLabel({ label: this.label, faint: true }) }
        .width('100%')
        .padding({ top: this.tokens.metric('spacing.1'), bottom: this.tokens.metric('spacing.1'),
          left: this.tokens.metric('spacing.2.5'), right: this.tokens.metric('spacing.2.5') })
    }
  }
}

@Component
export struct DazzSidebarGroupContent {
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() { Column() { this.content() }.width('100%').alignItems(HorizontalAlign.Start) }
}

@Component
export struct DazzSidebarMenu {
  @Consume('dazzui.theme') tokens: DazzTokens
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  build() {
    Column({ space: this.tokens.metric('frame.nav-gap') }) { this.content() }
      .width('100%').alignItems(HorizontalAlign.Start)
      .accessibilityRole(AccessibilityRoleType.LIST)
  }
}

@Component
export struct DazzSidebarMenuItem {
  @Prop @Watch('sync') active: boolean = false
  @Prop @Watch('sync') disabled: boolean = false
  @Prop @Watch('sync') variant: DazzVariant = 'filled'
  @Prop @Watch('sync') tint: DazzTint = 'primary'
  @Prop @Watch('sync') hasBadge: boolean = false
  @Provide({ allowOverride: 'dazzui.sidebar.row' }) row: DazzSidebarRowContext = new DazzSidebarRowContext()
  @State hovered: boolean = false
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  aboutToAppear(): void { this.sync() }
  sync(): void {
    const snapshot = new DazzSidebarRowContext()
    snapshot.owned = true
    snapshot.active = this.active
    snapshot.disabled = this.disabled
    snapshot.variant = this.variant
    snapshot.tint = this.tint
    snapshot.badge = this.hasBadge
    snapshot.hovered = this.hovered
    this.row = snapshot
  }
  build() {
    Stack({ alignContent: Alignment.Start }) { this.content() }
      .width('100%').height(LayoutPolicy.wrapContent)
      .onHover((hovered: boolean) => { this.hovered = hovered; this.sync() })
      .accessibilityRole(AccessibilityRoleType.LIST_ITEM)
  }
}

function rowInk(tokens: DazzTokens, variant: DazzVariant, tint: DazzTint,
  active: boolean, hovered: boolean, disabled: boolean): string {
  if (disabled) return tokens.paint('normal', 'content', 'disabled')
  if (variant === 'plain') return tokens.paint('plain', 'content', hovered ? 'hovered' : 'normal', tint)
  if (active) return tokens.paint(variant, 'content', 'normal', tint)
  return tokens.color(hovered && variant !== 'normal' ? 'content' : 'content-nav')
}

@Component
export struct DazzSidebarMenuButton {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.sidebar.column') column: DazzSidebarColumnContext = new DazzSidebarColumnContext()
  @Consume('dazzui.sidebar.row') row: DazzSidebarRowContext = new DazzSidebarRowContext()
  @Prop label: string = ''
  @Prop icon: string = ''
  @Prop iconID: string = ''
  @Prop iconSize: number | undefined = undefined
  @Prop active: boolean | undefined = undefined
  @Prop disabled: boolean | undefined = undefined
  @Prop variant: DazzVariant | undefined = undefined
  @Prop tint: DazzTint | undefined = undefined
  @Prop controlSize: string = 'medium'
  @Prop tooltip: string = ''
  @State hovered: boolean = false
  @State focused: boolean = false
  onAction: () => void = () => {}
  current(): boolean { return this.active ?? this.row.active }
  inert(): boolean { return this.disabled ?? this.row.disabled }
  recipe(): DazzVariant { return this.variant ?? this.row.variant }
  ramp(): DazzTint { return this.tint ?? this.row.tint }
  hover(): boolean { return this.row.owned ? this.row.hovered : this.hovered }
  ink(): string { return rowInk(this.tokens, this.recipe(), this.ramp(), this.current(), this.hover(), this.inert()) }
  surface(): ResourceColor {
    if (this.inert()) return Color.Transparent
    const variant = this.recipe()
    if (variant === 'plain') return this.hover() ? this.tokens.paint('plain', 'surface', 'hovered', this.ramp()) : Color.Transparent
    if (this.current()) return variant === 'outlined' ? Color.Transparent
      : this.tokens.paint(variant, 'surface', variant === 'normal' || variant === 'recessed' ? 'hovered' : 'normal', this.ramp())
    if (variant === 'normal') return this.tokens.paint('normal', 'surface')
    return this.hover() ? this.tokens.paint(variant === 'filled' ? 'plain' : 'recessed', 'surface', variant === 'filled' ? 'hovered' : 'normal', this.ramp()) : Color.Transparent
  }
  edge(): ResourceColor {
    if (this.inert()) return Color.Transparent
    if (this.recipe() === 'outlined' && this.current()) return this.tokens.paint('outlined', 'border', 'normal', this.ramp())
    if (this.recipe() !== 'normal') return Color.Transparent
    return this.current() ? this.tokens.paint('normal', 'border') : this.hover()
      ? this.tokens.paint('outlined', 'border', 'hovered', this.ramp()) : this.tokens.color('border')
  }
  rowHeight(): number {
    if (this.column.icon) return this.tokens.size('medium')
    const height = this.controlSize === 'large'
      ? this.tokens.metric('spacing.12') : this.tokens.type('labelQuiet').size +
        2 * this.tokens.metric(this.controlSize === 'small' ? 'menu.item-padding' : 'spacing.2')
    return this.tokens.systemStyle
      ? Math.max(height, this.tokens.size(this.controlSize === 'small' ? 'small' : 'medium')) : height
  }
  @LocalBuilder tooltipSurface() { DazzTooltipLabel({ label: this.tooltip }) }
  build() {
    Button({ type: ButtonType.Normal }) {
      Row({ space: this.tokens.metric('spacing.1.5') }) {
        if (this.icon) {
          Stack() {
            DazzIcon({ name: this.icon, iconSize: this.iconSize ?? this.tokens.metric('icon.medium'), color: this.ink() })
              .id(this.iconID)
          }
            .width(this.tokens.metric('icon.medium')).height(this.tokens.type('labelQuiet').size).flexShrink(0)
        }
        if (!this.column.icon) {
          Text(this.label).fontSize(this.tokens.fontSize(this.tokens.type('labelQuiet').size))
            .fontWeight(this.tokens.type(this.recipe() === 'plain' ? 'labelMedium' : 'labelQuiet').weight)
            .lineHeight(this.tokens.fontSize(this.tokens.type('labelQuiet').size))
            .fontColor(this.ink()).maxLines(this.controlSize === 'large' ? 2 : 1)
            .textOverflow({ overflow: TextOverflow.Ellipsis }).textAlign(TextAlign.Start)
            // A Row otherwise measures Text at its intrinsic width, beyond
            // the trailing space reserved for the sibling count.
            .layoutWeight(1)
        }
      }.width('100%').justifyContent(FlexAlign.Start)
    }.width(this.column.icon ? this.tokens.size('medium') : '100%')
      .height(this.rowHeight()).flexShrink(0).stateEffect(false).focusOnTouch(true)
      .padding({ top: 0, bottom: 0,
        left: this.column.icon ? (this.tokens.size('medium') - this.tokens.metric('icon.medium')) / 2 : this.tokens.metric('spacing.2.5'),
        right: this.column.icon ? (this.tokens.size('medium') - this.tokens.metric('icon.medium')) / 2
          : this.row.badge ? this.tokens.metric('spacing.8') : this.tokens.metric('spacing.2.5') })
      .borderRadius(this.tokens.metric(this.controlSize === 'large' && !this.column.icon ? 'radius.large' : 'radius.small'))
      .backgroundColor(this.surface())
      // React uses a spread shadow for these row edges, which consumes no
      // label inset. A native border would move the glyph and label inward.
      .outline({ width: !this.inert() && (this.recipe() === 'normal' ||
        (this.recipe() === 'outlined' && this.current())) ? this.tokens.metric('stroke.hairline') : 0,
        color: this.edge(), radius: this.tokens.metric(this.controlSize === 'large' && !this.column.icon ? 'radius.large' : 'radius.small') })
      .enabled(!this.inert()).accessibilityText(this.label).accessibilitySelected(this.current())
      .focusBox({ margin: LengthMetrics.vp(this.tokens.metric('focus.offset')),
        strokeWidth: LengthMetrics.vp(this.tokens.metric('focus.width')),
        strokeColor: ColorMetrics.resourceColor(this.tokens.focus()) })
      .onHover((hovered: boolean) => { this.hovered = hovered })
      .onFocus(() => { this.focused = this.getUIContext().getFocusController().isActive() })
      .onBlur(() => { this.focused = false })
      .onClick(() => { this.focused = false; if (!this.inert()) this.onAction() })
      .bindPopup(this.column.icon && !!this.tooltip && (this.hover() || this.focused), {
        builder: this.tooltipSurface, placement: Placement.Right, autoCancel: true,
        mask: false, focusable: false, enableArrow: false, radius: this.tokens.metric('radius.small'),
        targetSpace: this.tokens.metric('spacing.1'), popupColor: Color.Transparent,
        backgroundBlurStyle: BlurStyle.NONE, shadow: tooltipShadow(this.tokens, this.getUIContext().vp2px(1)),
      })
  }
}

@Component
export struct DazzSidebarMenuBadge {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.sidebar.column') column: DazzSidebarColumnContext = new DazzSidebarColumnContext()
  @Consume('dazzui.sidebar.row') row: DazzSidebarRowContext = new DazzSidebarRowContext()
  @Prop label: string = ''
  @Prop badgeID: string = ''
  @Prop tint: DazzTint = 'primary'
  ink(): string {
    if (this.row.active) {
      if (this.row.variant === 'filled' || this.row.variant === 'tinted' || this.row.variant === 'outlined')
        return this.tokens.paint(this.row.variant, 'content', 'normal', this.tint)
      if (this.row.variant === 'recessed' || this.row.variant === 'normal') return this.tokens.color('content')
    }
    return this.tokens.color(this.row.hovered && !this.row.disabled ? 'content' : 'content-nav')
  }
  build() {
    if (!this.column.icon) {
      Row() {
        Text(this.label).height(this.tokens.metric('spacing.5'))
          .constraintSize({ minWidth: this.tokens.metric('spacing.5') })
          .padding({ left: this.tokens.metric('spacing.1'), right: this.tokens.metric('spacing.1') })
          .fontSize(this.tokens.fontSize(this.tokens.type('captionSmall').size))
          .fontWeight(this.tokens.type('labelMedium').weight).textAlign(TextAlign.Center)
          .fontColor(this.ink()).fontFeature('"tnum" 1')
          .margin({ right: this.tokens.metric('spacing.1') }).id(this.badgeID)
          .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
      }.width('100%').height(LayoutPolicy.wrapContent).justifyContent(FlexAlign.End)
        .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
    }
  }
}

@Component
export struct DazzSidebarTrigger {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Consume('dazzui.sidebar') sidebar: DazzSidebarContext = new DazzSidebarContext()
  @Prop label: string = 'Toggle sidebar'
  @LocalBuilder glyph() {
    DazzIcon({ name: 'PanelLeft20Regular', iconSize: this.tokens.size('tiny', 'icon-size'),
      color: this.tokens.paint('plain', 'content', 'normal', 'neutral') })
  }
  @LocalBuilder button() {
    DazzIconButton({ label: this.label, hint: this.sidebar.open ? 'Expanded' : 'Collapsed',
      content: this.glyph, onAction: () => this.sidebar.toggle() })
  }
  build() { this.button() }
}
