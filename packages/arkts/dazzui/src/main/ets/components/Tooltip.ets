import { ComponentContent } from '@kit.ArkUI'
import { DazzTokens } from '../foundation/Tokens'
import { shadowOptions } from '../foundation/Shadow'
import { DazzTooltipDefaults, DazzTooltipInteraction, DazzTooltipScope } from '../foundation/TooltipData'
import { DazzPointerContext } from '../foundation/Pointer'
import { DazzShadowLayer } from './ShadowLayer'

export function tooltipShadow(tokens: DazzTokens, density: number): ShadowOptions {
  return shadowOptions(tokens.shadow('shadow.sm')[0], density)
}

@Component
export struct DazzTooltipLabel {
  @Consume('dazzui.theme') tokens: DazzTokens
  @Prop @Watch('resetLines') label: string = ''
  @Prop maximumWidth: number | undefined = undefined
  @State lines: number = 0
  private text: TextController = new TextController()
  resetLines(): void { this.lines = 0 }
  blockPadding(): number { return this.tokens.metric('spacing.1') + this.tokens.metric('spacing.0.5') }
  build() {
    Text(this.label, { controller: this.text })
      .id('dazzui-tooltip-surface-' + String(this.getUniqueId()))
      .height(this.lines > 0 ? this.lines * this.tokens.type('labelSmall').leading + 2 * (this.blockPadding() + this.tokens.metric('stroke.hairline')) : undefined)
      .fontSize(this.tokens.fontSize(this.tokens.type('labelSmall').size))
      .fontWeight(this.tokens.type('labelSmall').weight)
      .lineHeight(this.tokens.fontSize(this.tokens.type('labelSmall').leading))
      .fontColor(this.tokens.color('content'))
      .padding({ top: this.blockPadding() + this.tokens.metric('stroke.hairline'),
        bottom: this.blockPadding() + this.tokens.metric('stroke.hairline'),
        left: this.tokens.metric('spacing.2') + this.tokens.metric('stroke.hairline'), right: this.tokens.metric('spacing.2') + this.tokens.metric('stroke.hairline') })
      .constraintSize({ maxWidth: this.maximumWidth ?? this.tokens.metric('toast.max-width') })
      .backgroundColor(this.tokens.color('surface-raised'))
      .border({ width: this.tokens.metric('stroke.hairline'), color: this.tokens.color('border'),
        radius: this.tokens.metric('radius.small') })
      .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
      .onAreaChange(() => { const count = this.text.getLayoutManager().getLineCount(); if (count > 0 && this.lines !== count) this.lines = count })
  }
}

@Observed
class TooltipLayerData {
  tokens: DazzTokens = new DazzTokens()
  label: string = ''
  id: string = ''
  x: number = 0
  y: number = 0
  width: number = 0
  height: number = 0
  maximumWidth: number = 0
  measured: (width: number, height: number) => void = () => {}
}

/** A passive overlay: native popup hosts intercept presses inside their sheet. */
@Component
struct TooltipLayer {
  @ObjectLink @Watch('syncTheme') data: TooltipLayerData
  @Provide({ allowOverride: 'dazzui.theme' }) tokens: DazzTokens = new DazzTokens()
  aboutToAppear(): void { this.syncTheme() }
  syncTheme(): void { this.tokens = this.data.tokens }
  build() {
    Column() {
      Stack({ alignContent: Alignment.TopStart }) {
        DazzTooltipLabel({ label: this.data.label, maximumWidth: this.data.maximumWidth })
          .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
          .onAreaChange((_old: Area, area: Area) => this.data.measured(Number(area.width), Number(area.height)))
        DazzShadowLayer({ paintWidth: this.data.width, paintHeight: this.data.height,
          corner: this.tokens.metric('radius.small'), shadowLayers: this.tokens.shadow('shadow.sm') })
          .position({ x: 0, y: 0 }).hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
          .accessibilityLevel('no-hide-descendants')
      }.position({ x: this.data.x, y: this.data.y })
        .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
    }.width('100%').height('100%').id(this.data.id)
      .hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
  }
}

@Builder
function tooltipLayer(data: TooltipLayerData) {
  TooltipLayer({ data }).hitTestBehavior(HitTestMode.None).touchable(false).focusable(false)
}

@Component
export struct DazzTooltip {
  @Consume('dazzui.theme') @Watch('refresh') tokens: DazzTokens
  @Consume('dazzui.tooltip-defaults') defaults: DazzTooltipDefaults = new DazzTooltipDefaults()
  @Consume('dazzui.pointer') pointer: DazzPointerContext = new DazzPointerContext()
  @Consume('dazzui.tooltip-scope') parentScope: DazzTooltipScope = new DazzTooltipScope()
  @Provide({ allowOverride: 'dazzui.tooltip-scope' }) scope: DazzTooltipScope = new DazzTooltipScope()
  @Prop @Watch('refresh') label: string = ''
  @Prop @Watch('refresh') side: Placement = Placement.Top
  @Prop @Watch('configure') delay: number | undefined = undefined
  @Prop @Watch('configure') closeDelay: number | undefined = undefined
  @Prop @Watch('configure') disabled: boolean = false
  @State visible: boolean = false
  private interaction: DazzTooltipInteraction = new DazzTooltipInteraction()
  private mounted: boolean = false
  private children: Set<number> = new Set()
  private data: TooltipLayerData = new TooltipLayerData()
  private layer?: ComponentContent<TooltipLayerData>
  private geometry: string = ''
  private layoutChanged: () => void = () => this.refresh()
  private press: (x: number, y: number) => void = () => { this.interaction.dismiss() }
  @BuilderParam content: () => void = this.empty
  @LocalBuilder empty() {}
  aboutToAppear(): void {
    this.mounted = true
    this.interaction.current = () => this.visible
    this.interaction.change = (visible: boolean) => { this.visible = visible; this.refresh() }
    this.interaction.disabled = () => this.disabled || this.children.size > 0
    this.scope.register = (id: number) => { this.children.add(id); this.interaction.configure() }
    this.scope.unregister = (id: number) => { this.children.delete(id) }
    this.parentScope.register(this.getUniqueId())
    this.interaction.openDelay = () => this.delay ?? this.defaults.delay ?? this.tokens.metric('tooltip.open-delay')
    this.interaction.closeDelay = () => this.closeDelay ?? this.defaults.closeDelay ?? this.tokens.metric('tooltip.close-delay')
    this.pointer.on(this.press)
    this.data.id = 'dazzui-tooltip-layer-' + String(this.getUniqueId())
    this.data.measured = (width: number, height: number) => {
      const density = this.getUIContext().vp2px(1)
      this.data.width = Math.round(width * density) / density
      this.data.height = Math.round(height * density) / density
      this.refresh()
    }
    this.getUIContext().getUIObserver().on('didLayout', this.layoutChanged)
  }
  configure(): void { this.interaction.configure() }
  aboutToDisappear(): void {
    this.mounted = false
    this.pointer.off(this.press)
    this.getUIContext().getUIObserver().off('didLayout', this.layoutChanged)
    this.interaction.dispose()
    this.removeLayer()
    this.parentScope.unregister(this.getUniqueId())
  }
  removeLayer(): void {
    if (!this.layer) return
    this.getUIContext().getOverlayManager().removeComponentContent(this.layer)
    this.layer.dispose(); this.layer = undefined; this.geometry = ''
  }
  refresh(): void {
    if (!this.mounted) return
    if (!this.visible) { this.removeLayer(); return }
    const ui = this.getUIContext()
    this.data.tokens = this.tokens; this.data.label = this.label
    if (!this.layer) {
      this.data.maximumWidth = this.tokens.metric('toast.max-width')
      this.layer = new ComponentContent(ui, wrapBuilder<[TooltipLayerData]>(tooltipLayer), this.data)
      ui.getOverlayManager().addComponentContent(this.layer)
    }
    const target = ui.getAttachedFrameNodeById('dazzui-tooltip-target-' + String(this.getUniqueId()))
    const root = ui.getAttachedFrameNodeById(this.data.id)
    if (!target || !root) return
    const origin = root.getPositionToScreen(), anchor = target.getPositionToScreen()
    const size = target.getMeasuredSize(), viewport = root.getMeasuredSize()
    const w = ui.px2vp(viewport.width), h = ui.px2vp(viewport.height)
    const left = anchor.x - origin.x, top = anchor.y - origin.y
    const tw = ui.px2vp(size.width), th = ui.px2vp(size.height)
    const gap = this.tokens.metric('spacing.1'), pw = this.data.width, ph = this.data.height
    this.data.maximumWidth = Math.max(0, Math.min(this.tokens.metric('toast.max-width'), w - 2 * gap))
    const start = [Placement.TopLeft, Placement.BottomLeft, Placement.LeftTop, Placement.RightTop].includes(this.side)
    const end = [Placement.TopRight, Placement.BottomRight, Placement.LeftBottom, Placement.RightBottom].includes(this.side)
    let side = [Placement.Top, Placement.TopLeft, Placement.TopRight].includes(this.side) ? Placement.Top :
      [Placement.Bottom, Placement.BottomLeft, Placement.BottomRight].includes(this.side) ? Placement.Bottom :
      [Placement.Left, Placement.LeftTop, Placement.LeftBottom].includes(this.side) ? Placement.Left : Placement.Right
    if (side === Placement.Top && top - gap < ph && h - top - th > top) side = Placement.Bottom
    else if (side === Placement.Bottom && h - top - th - gap < ph && top > h - top - th) side = Placement.Top
    else if (side === Placement.Left && left - gap < pw && w - left - tw > left) side = Placement.Right
    else if (side === Placement.Right && w - left - tw - gap < pw && left > w - left - tw) side = Placement.Left
    let x = start ? left : end ? left + tw - pw : left + (tw - pw) / 2
    let y = start ? top : end ? top + th - ph : top + (th - ph) / 2
    if (side === Placement.Top) y = top - gap - ph
    else if (side === Placement.Bottom) y = top + th + gap
    else if (side === Placement.Left) x = left - gap - pw
    else if (side === Placement.Right) x = left + tw + gap
    const density = ui.vp2px(1)
    x = Math.round(Math.max(gap, Math.min(w - gap - pw, x)) * density) / density
    y = Math.round(Math.max(gap, Math.min(h - gap - ph, y)) * density) / density
    const key = [x, y, pw, ph, this.data.maximumWidth, this.tokens.name, this.label].join(':')
    if (this.geometry === key) return
    this.geometry = key; this.data.x = x; this.data.y = y
    this.layer.update(this.data)
  }
  handleKey(event: KeyEvent): boolean {
    if (event.type !== KeyType.Down) return false
    if (event.keyCode === 2070 && this.visible) { this.interaction.dismiss(); event.stopPropagation(); return true }
    if (event.keyCode === 2054 || event.keyCode === 2050) this.interaction.dismiss()
    return false
  }
  build() {
    Row() {
      this.content()
    }
      .id('dazzui-tooltip-target-' + String(this.getUniqueId()))
      // Hover works before any keyboard focus is active in this window.
      .keyboardShortcut(this.visible ? FunctionKey.ESC : '', [], () => this.interaction.dismiss())
      .onHover((hover: boolean, event: HoverEvent) => { if (event.source !== SourceType.TouchScreen) this.interaction.hover(hover) })
      .onMouse((event: MouseEvent) => {
        if (event.action === MouseAction.Move) this.interaction.move(event.x, event.y)
        else if (event.action === MouseAction.Press) this.interaction.dismiss()
      })
      .onTouch((event: TouchEvent) => { if (event.type === TouchType.Down) this.interaction.dismiss() })
      .onFocus(() => {
        this.interaction.focus(this.getUIContext().getFocusController().isActive())
      })
      .onBlur(() => {
        this.interaction.focus(false)
      })
      // Observe activation before native Buttons consume it, without taking
      // over dispatch to the focused child.
      .onKeyPreIme((event: KeyEvent): boolean => this.handleKey(event))
      .onKeyEvent((event: KeyEvent): boolean => this.handleKey(event))
  }
}
